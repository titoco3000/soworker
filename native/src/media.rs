//! Calls on the official LiveKit Rust SDK: join with the token from gaia's `cluster` event,
//! audio through the SDK's platform audio (mic capture + playout + echo cancellation),
//! remote video decoded by libwebrtc and delivered as small RGBA frames.
use crate::calls::{MediaCmd, MediaEvent};
use futures_util::StreamExt;
use livekit::options::{TrackPublishOptions, VideoCodec, VideoEncoding};
use livekit::prelude::*;
use livekit::webrtc::video_frame::native::VideoFrameBufferExt;
use livekit::webrtc::native::yuv_helper;
use livekit::webrtc::video_frame::{I420Buffer, VideoBuffer, VideoFormatType, VideoFrame, VideoRotation};
use livekit::webrtc::video_source::native::NativeVideoSource;
use livekit::webrtc::video_source::{RtcVideoSource, VideoResolution};
use livekit::webrtc::desktop_capturer::{DesktopCaptureSourceType, DesktopCapturer, DesktopCapturerOptions};
use std::sync::atomic::{AtomicBool, Ordering};
use livekit::webrtc::video_stream::native::NativeVideoStream;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

type Emit = Arc<dyn Fn(MediaEvent) + Send + Sync>;

/// A running local capture (camera or screen): the thread feeding it and the published track.
struct Capture {
    stop: Arc<AtomicBool>,
    sid: TrackSid,
}

struct Session {
    room: Room,
    events: UnboundedReceiver<RoomEvent>,
    audio: Option<PlatformAudio>,
    mic: Option<LocalAudioTrack>,
    cam: Option<Capture>,
    share: Option<Capture>,
    video_tasks: HashMap<String, tokio::task::JoinHandle<()>>,
    room_id: String,
}

pub fn spawn(emit: impl Fn(MediaEvent) + Send + Sync + 'static) -> UnboundedSender<MediaCmd> {
    let (tx, rx) = unbounded_channel();
    let emit: Emit = Arc::new(emit);
    std::thread::Builder::new()
        .name("media".into())
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(2).enable_all().build().expect("tokio runtime");
            rt.block_on(run(rx, emit));
        })
        .expect("media thread");
    tx
}

async fn run(mut rx: UnboundedReceiver<MediaCmd>, emit: Emit) {
    let mut session: Option<Session> = None;
    loop {
        tokio::select! {
            cmd = rx.recv() => {
                let Some(cmd) = cmd else { break };
                match cmd {
                    MediaCmd::Join { url, token, room_id } => {
                        if session.as_ref().is_some_and(|s| s.room_id == room_id) { continue; }
                        if let Some(s) = session.take() { close(s, &emit).await; }
                        match join(&url, &token, room_id, &emit).await {
                            Ok(s) => session = Some(s),
                            Err(e) => { emit(MediaEvent::Error(format!("call failed: {e}"))); emit(MediaEvent::Left); }
                        }
                    }
                    MediaCmd::Leave => if let Some(s) = session.take() { close(s, &emit).await; },
                    MediaCmd::Mic(on) => if let Some(s) = session.as_mut() { set_mic(s, on, &emit).await; },
                    MediaCmd::Camera(on) => if let Some(s) = session.as_mut() { set_camera(s, on, &emit).await; },
                    MediaCmd::Share(on) => if let Some(s) = session.as_mut() { set_share(s, on, &emit).await; },
                }
            }
            ev = async { match session.as_mut() { Some(s) => s.events.recv().await, None => std::future::pending().await } } => {
                match ev {
                    Some(ev) => {
                        if let Some(s) = session.as_mut() {
                            if on_room_event(s, ev, &emit) { if let Some(s) = session.take() { close(s, &emit).await; } }
                        }
                    }
                    None => if let Some(s) = session.take() { close(s, &emit).await; },
                }
            }
        }
    }
}

async fn join(url: &str, token: &str, room_id: String, emit: &Emit) -> Result<Session, String> {
    let mut opts = RoomOptions::default();
    opts.auto_subscribe = true;
    opts.adaptive_stream = true;
    opts.dynacast = true;
    let (room, events) = Room::connect(url, token, opts).await.map_err(|e| e.to_string())?;
    // platform audio: plays everything we subscribe to; the mic stays closed until the user unmutes
    let audio = match PlatformAudio::new() {
        Ok(a) => Some(a),
        Err(e) => {
            emit(MediaEvent::Error(format!("audio device: {e}")));
            None
        }
    };
    emit(MediaEvent::Joined(room_id.clone()));
    Ok(Session { room, events, audio, mic: None, cam: None, share: None, video_tasks: HashMap::new(), room_id })
}

async fn close(mut s: Session, emit: &Emit) {
    for c in [s.cam.take(), s.share.take()].into_iter().flatten() {
        c.stop.store(true, Ordering::Relaxed);
    }
    emit(MediaEvent::CameraState(false));
    emit(MediaEvent::ShareState(false));
    emit(MediaEvent::VideoGone("me".into()));
    for (key, t) in s.video_tasks.drain() {
        t.abort();
        emit(MediaEvent::VideoGone(key));
    }
    let _ = s.room.close().await;
    drop(s.mic.take());
    drop(s.audio.take()); // last reference releases the audio device
    emit(MediaEvent::MicState(false));
    emit(MediaEvent::Left);
}

async fn set_mic(s: &mut Session, on: bool, emit: &Emit) {
    if on && s.mic.is_none() {
        let Some(audio) = &s.audio else { return emit(MediaEvent::Error("no audio device".into())) };
        let track = LocalAudioTrack::create_audio_track("microphone", audio.rtc_source());
        let mut opts = TrackPublishOptions::default();
        opts.source = TrackSource::Microphone;
        if let Err(e) = s.room.local_participant().publish_track(LocalTrack::Audio(track.clone()), opts).await {
            return emit(MediaEvent::Error(format!("microphone: {e}")));
        }
        s.mic = Some(track);
    } else if let Some(t) = &s.mic {
        if on { t.unmute() } else { t.mute() }
    }
    emit(MediaEvent::MicState(on && s.mic.is_some()));
}

/// Returns true when the session is over.
fn on_room_event(s: &mut Session, ev: RoomEvent, emit: &Emit) -> bool {
    match ev {
        RoomEvent::ActiveSpeakersChanged { speakers } => {
            emit(MediaEvent::Speakers(speakers.iter().map(|p| p.identity().to_string()).collect()));
        }
        RoomEvent::TrackSubscribed { track: RemoteTrack::Video(v), publication, participant } => {
            let screen = publication.source() == TrackSource::Screenshare;
            let key = format!("{}{}", participant.identity(), if screen { ":screen" } else { "" });
            let name = participant.name();
            let (max_w, emit2) = (if screen { 480u32 } else { 240 }, emit.clone());
            let k = key.clone();
            let task = tokio::spawn(async move {
                let mut stream = NativeVideoStream::new(v.rtc_track());
                let mut last = Instant::now() - Duration::from_secs(1);
                while let Some(frame) = stream.next().await {
                    if last.elapsed() < Duration::from_millis(100) {
                        continue; // ~10 fps is plenty for a thumbnail; skip the conversion work
                    }
                    last = Instant::now();
                    let (sw, sh) = (frame.buffer.width().max(1), frame.buffer.height().max(1));
                    let w = max_w.min(sw) & !1;
                    let h = ((w as u64 * sh as u64 / sw as u64) as u32 & !1).max(2);
                    let mut i420 = frame.buffer.to_i420();
                    let mut scaled = i420.scale(w as i32, h as i32);
                    let mut rgba = vec![0u8; (w * h * 4) as usize];
                    scaled.to_argb(VideoFormatType::ABGR, &mut rgba, w * 4, w as i32, h as i32);
                    emit2(MediaEvent::Video { key: k.clone(), name: name.clone(), w, h, rgba });
                }
            });
            if let Some(old) = s.video_tasks.insert(key, task) {
                old.abort();
            }
        }
        RoomEvent::TrackUnsubscribed { track: RemoteTrack::Video(_), publication, participant } => {
            let screen = publication.source() == TrackSource::Screenshare;
            let key = format!("{}{}", participant.identity(), if screen { ":screen" } else { "" });
            if let Some(t) = s.video_tasks.remove(&key) {
                t.abort();
            }
            emit(MediaEvent::VideoGone(key));
        }
        RoomEvent::ParticipantDisconnected(p) => {
            for key in [p.identity().to_string(), format!("{}:screen", p.identity())] {
                if let Some(t) = s.video_tasks.remove(&key) {
                    t.abort();
                }
                emit(MediaEvent::VideoGone(key));
            }
        }
        RoomEvent::Disconnected { reason } => {
            emit(MediaEvent::Error(format!("call ended: {reason:?}")));
            return true;
        }
        _ => {}
    }
    false
}


// ---- camera: V4L2 (YUYV) -> I420 -> NativeVideoSource, published as the camera track

fn now_us() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_micros() as i64)
}

/// Packed YUYV 4:2:2 to planar I420 (chroma averaged over row pairs).
fn yuyv_to_i420(src: &[u8], w: usize, h: usize, dst: &mut I420Buffer) {
    let (sy, su, sv) = dst.strides();
    let (dy, du, dv) = dst.data_mut();
    for row in 0..h {
        let line = &src[row * w * 2..(row + 1) * w * 2];
        for x in 0..w {
            dy[row * sy as usize + x] = line[x * 2];
        }
        if row % 2 == 0 && row + 1 < h {
            let next = &src[(row + 1) * w * 2..(row + 2) * w * 2];
            for x in 0..w / 2 {
                let (u0, u1) = (line[x * 4 + 1] as u16, next[x * 4 + 1] as u16);
                let (v0, v1) = (line[x * 4 + 3] as u16, next[x * 4 + 3] as u16);
                du[(row / 2) * su as usize + x] = ((u0 + u1) / 2) as u8;
                dv[(row / 2) * sv as usize + x] = ((v0 + v1) / 2) as u8;
            }
        }
    }
}

fn thumbnail(buf: &mut I420Buffer, max_w: u32) -> (u32, u32, Vec<u8>) {
    let (sw, sh) = (buf.width().max(1), buf.height().max(1));
    let w = max_w.min(sw) & !1;
    let h = ((w as u64 * sh as u64 / sw as u64) as u32 & !1).max(2);
    let mut scaled = buf.scale(w as i32, h as i32);
    let mut rgba = vec![0u8; (w * h * 4) as usize];
    scaled.to_argb(VideoFormatType::ABGR, &mut rgba, w * 4, w as i32, h as i32);
    (w, h, rgba)
}

fn camera_thread(source: NativeVideoSource, stop: Arc<AtomicBool>, emit: Emit, ready: tokio::sync::oneshot::Sender<Result<(), String>>) {
    use v4l::io::traits::CaptureStream;
    use v4l::prelude::*;
    use v4l::video::Capture;
    let open = || -> Result<(Device, usize, usize), String> {
        let dev = Device::new(0).map_err(|e| format!("no camera (/dev/video0): {e}"))?;
        let mut fmt = dev.format().map_err(|e| e.to_string())?;
        (fmt.width, fmt.height, fmt.fourcc) = (640, 480, v4l::FourCC::new(b"YUYV"));
        let fmt = dev.set_format(&fmt).map_err(|e| e.to_string())?;
        if &fmt.fourcc.repr != b"YUYV" {
            return Err(format!("camera does not offer YUYV (got {})", fmt.fourcc));
        }
        Ok((dev, fmt.width as usize, fmt.height as usize))
    };
    let (dev, w, h) = match open() {
        Ok(v) => v,
        Err(e) => return drop(ready.send(Err(e))),
    };
    let mut stream = match v4l::io::mmap::Stream::with_buffers(&dev, v4l::buffer::Type::VideoCapture, 4) {
        Ok(s) => s,
        Err(e) => return drop(ready.send(Err(format!("camera stream: {e}")))),
    };
    let _ = ready.send(Ok(()));
    let (mut n, mut last_preview) = (0u64, Instant::now() - Duration::from_secs(1));
    while !stop.load(Ordering::Relaxed) {
        let Ok((data, _)) = stream.next() else { break };
        n += 1;
        if n % 2 == 1 || data.len() < w * h * 2 {
            continue; // 15 fps is enough and halves the encode cost
        }
        let mut i420 = I420Buffer::new(w as u32, h as u32);
        yuyv_to_i420(data, w, h, &mut i420);
        if last_preview.elapsed() >= Duration::from_millis(100) {
            last_preview = Instant::now();
            let (pw, ph, rgba) = thumbnail(&mut i420, 160);
            emit(MediaEvent::Video { key: "me".into(), name: "you".into(), w: pw, h: ph, rgba });
        }
        source.capture_frame(&VideoFrame { rotation: VideoRotation::VideoRotation0, timestamp_us: now_us(), frame_metadata: None, buffer: i420 });
    }
}

async fn set_camera(s: &mut Session, on: bool, emit: &Emit) {
    if on && s.cam.is_none() {
        let source = NativeVideoSource::new(VideoResolution { width: 640, height: 480 }, false);
        let (stop, (tx, rx)) = (Arc::new(AtomicBool::new(false)), tokio::sync::oneshot::channel());
        let (src2, stop2, emit2) = (source.clone(), stop.clone(), emit.clone());
        std::thread::spawn(move || camera_thread(src2, stop2, emit2, tx));
        match rx.await {
            Ok(Ok(())) => {}
            other => {
                stop.store(true, Ordering::Relaxed);
                emit(MediaEvent::Error(match other {
                    Ok(Err(e)) => e,
                    _ => "camera thread ended".into(),
                }));
                return emit(MediaEvent::CameraState(false));
            }
        }
        let track = LocalVideoTrack::create_video_track("camera", RtcVideoSource::Native(source));
        let mut opts = TrackPublishOptions::default();
        opts.source = TrackSource::Camera;
        opts.video_codec = VideoCodec::VP8;
        match s.room.local_participant().publish_track(LocalTrack::Video(track), opts).await {
            Ok(p) => s.cam = Some(Capture { stop, sid: p.sid() }),
            Err(e) => {
                stop.store(true, Ordering::Relaxed);
                emit(MediaEvent::Error(format!("camera publish: {e}")));
            }
        }
    } else if !on {
        if let Some(c) = s.cam.take() {
            c.stop.store(true, Ordering::Relaxed);
            let _ = s.room.local_participant().unpublish_track(&c.sid).await;
            emit(MediaEvent::VideoGone("me".into()));
        }
    }
    emit(MediaEvent::CameraState(s.cam.is_some()));
}

// ---- screen share: libwebrtc desktop capturer (X11 or the Wayland portal picker) -> screencast source

#[derive(serde::Serialize)]
struct Meta {
    #[serde(rename = "raisedHandAt")]
    raised: Option<()>,
    #[serde(rename = "audioOnly")]
    audio_only: bool,
    #[serde(rename = "allowsScreenDrawing")]
    drawing: bool,
    desktop: bool,
    screenshare: Option<serde_json::Value>,
}

/// SoWork's own clients expect this on a share: participant metadata plus a reliable `screenshare` message.
async fn announce_share(s: &Session, start: bool, sid: Option<&TrackSid>) {
    let lp = s.room.local_participant();
    let meta = Meta { raised: None, audio_only: false, drawing: false, desktop: false, screenshare: start.then(|| serde_json::json!({"type": "screen"})) };
    let _ = lp.set_metadata(serde_json::to_string(&meta).unwrap_or_default()).await;
    let mut pairs = vec![("userId".into(), lp.identity().to_string().into()), ("start".into(), start.into())];
    if let Some(sid) = sid {
        pairs.push(("trackSid".into(), sid.to_string().into()));
    }
    let mut payload = Vec::new();
    let _ = rmpv::encode::write_value(&mut payload, &rmpv::Value::Map(pairs));
    let _ = lp.publish_data(DataPacket { payload, topic: Some("screenshare".into()), reliable: true, ..Default::default() }).await;
}

fn screen_thread(source: NativeVideoSource, stop: Arc<AtomicBool>, ready: tokio::sync::oneshot::Sender<Result<(), String>>) {
    let Some(mut cap) = DesktopCapturer::new(DesktopCapturerOptions::new(DesktopCaptureSourceType::Generic)) else {
        return drop(ready.send(Err("screen capture is not available here".into())));
    };
    let (ready, dead) = (std::sync::Mutex::new(Some(ready)), Arc::new(AtomicBool::new(false)));
    let dead2 = dead.clone();
    cap.start_capture(None, move |res| match res {
        Ok(frame) => {
            if let Some(r) = ready.lock().ok().and_then(|mut g| g.take()) {
                let _ = r.send(Ok(())); // the user accepted the picker: frames flow
            }
            let (w, h) = (frame.width().max(2) & !1, frame.height().max(2) & !1);
            let mut i420 = I420Buffer::new(w as u32, h as u32);
            let (sy, su, sv) = i420.strides();
            let (dy, du, dv) = i420.data_mut();
            yuv_helper::argb_to_i420(frame.data(), frame.stride(), dy, sy, du, su, dv, sv, w, h);
            source.capture_frame(&VideoFrame { rotation: VideoRotation::VideoRotation0, timestamp_us: now_us(), frame_metadata: None, buffer: i420 });
        }
        Err(livekit::webrtc::desktop_capturer::CaptureError::Permanent) => {
            if let Some(r) = ready.lock().ok().and_then(|mut g| g.take()) {
                let _ = r.send(Err("screen share cancelled or not allowed".into()));
            }
            dead2.store(true, Ordering::Relaxed);
        }
        Err(_) => {}
    });
    while !stop.load(Ordering::Relaxed) && !dead.load(Ordering::Relaxed) {
        cap.capture_frame();
        std::thread::sleep(Duration::from_millis(66)); // ~15 fps
    }
}

async fn set_share(s: &mut Session, on: bool, emit: &Emit) {
    if on && s.share.is_none() {
        let source = NativeVideoSource::new(VideoResolution { width: 1920, height: 1080 }, true);
        let (stop, (tx, rx)) = (Arc::new(AtomicBool::new(false)), tokio::sync::oneshot::channel());
        let (src2, stop2) = (source.clone(), stop.clone());
        std::thread::spawn(move || screen_thread(src2, stop2, tx));
        // on Wayland this waits for the user to choose a screen or window in the portal dialog
        let first = tokio::time::timeout(Duration::from_secs(90), rx).await;
        match first {
            Ok(Ok(Ok(()))) => {}
            Ok(Ok(Err(e))) => {
                stop.store(true, Ordering::Relaxed);
                emit(MediaEvent::Error(e));
                return emit(MediaEvent::ShareState(false));
            }
            _ => {
                stop.store(true, Ordering::Relaxed);
                emit(MediaEvent::Error("screen share: no source was chosen".into()));
                return emit(MediaEvent::ShareState(false));
            }
        }
        let track = LocalVideoTrack::create_video_track("screen", RtcVideoSource::Native(source));
        let mut opts = TrackPublishOptions::default();
        opts.source = TrackSource::Screenshare;
        opts.video_codec = VideoCodec::VP8;
        opts.video_encoding = Some(VideoEncoding { max_bitrate: 4_000_000, max_framerate: 15.0 });
        match s.room.local_participant().publish_track(LocalTrack::Video(track), opts).await {
            Ok(p) => {
                let sid = p.sid();
                s.share = Some(Capture { stop, sid: sid.clone() });
                announce_share(s, true, Some(&sid)).await;
            }
            Err(e) => {
                stop.store(true, Ordering::Relaxed);
                emit(MediaEvent::Error(format!("screen share publish: {e}")));
            }
        }
    } else if !on {
        if let Some(c) = s.share.take() {
            c.stop.store(true, Ordering::Relaxed);
            let _ = s.room.local_participant().unpublish_track(&c.sid).await;
            announce_share(s, false, Some(&c.sid)).await;
        }
    }
    emit(MediaEvent::ShareState(s.share.is_some()));
}
