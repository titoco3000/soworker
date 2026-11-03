#![cfg_attr(not(feature = "gui"), allow(dead_code, unused_mut))]
//! Meeting recording. The media thread feeds a `Tap` with every remote audio track, our microphone and the latest
//! picture of every camera and screen share, at their own size and whatever the UI happens to show. While a recording
//! runs, a thread of its own mixes the audio into a 48 kHz mono WAV and (in builds with the window client's drawing,
//! when ffmpeg is installed) composes the call into a fixed 1280x800 (16:10) picture 15 times a second, which ffmpeg
//! encodes. When it stops, ffmpeg muxes the two into an MP4 and whisper.cpp writes the transcript next to it (see
//! `transcribe`). Without ffmpeg or the drawing, the recording is the WAV alone.
use crate::calls::MediaEvent;
use crate::transcribe::{self, sibling, Wav};
use livekit::webrtc::video_frame::native::VideoFrameBufferExt;
use livekit::webrtc::video_frame::{I420Buffer, VideoBuffer, VideoFormatType};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

type Emit = Arc<dyn Fn(MediaEvent) + Send + Sync>;

const RATE: usize = 48_000;
/// The recorded picture: always this size, whatever the window or the terminal shows.
pub const W: u32 = 1280;
pub const H: u32 = 800;
const FPS: u32 = 15;
/// A track is mixed only once this much of it is buffered (150 ms), and again after it runs dry: a cushion against
/// network jitter. The audio is therefore that much late, which the mux takes back off.
const PRIME: usize = RATE * 15 / 100;
const LATE: &str = "0.150";
/// At most this much waits per track; older samples are dropped so clock drift cannot pile up.
const QUEUE_MAX: usize = RATE / 2;

#[derive(Default)]
struct Queue {
    buf: VecDeque<i16>,
    primed: bool,
}

pub struct Pic {
    w: u32,
    h: u32,
    rgba: Vec<u8>,
}

#[derive(Default)]
struct Inner {
    audio: HashMap<String, Queue>,
    pics: HashMap<String, Arc<Pic>>,
    pic_at: HashMap<String, Instant>,
    /// (key, name) of everyone in the call, us first as "me"; and who is speaking, by the same keys.
    people: Vec<(String, String)>,
    speaking: HashSet<String>,
}

/// What the recorder takes from the call. Pushing to it costs nothing while no recording runs.
#[derive(Default)]
pub struct Tap {
    on: AtomicBool,
    video: AtomicBool,
    inner: Mutex<Inner>,
}

impl Tap {
    /// 48 kHz mono samples of one audio track (`me` for our microphone).
    pub fn push_audio(&self, key: &str, samples: &[i16]) {
        if !self.on.load(Ordering::Relaxed) {
            return;
        }
        if let Ok(mut g) = self.inner.lock() {
            let q = g.audio.entry(key.to_string()).or_default();
            q.buf.extend(samples);
            let extra = q.buf.len().saturating_sub(QUEUE_MAX);
            q.buf.drain(..extra);
        }
    }

    pub fn forget_audio(&self, key: &str) {
        if let Ok(mut g) = self.inner.lock() {
            g.audio.remove(key);
        }
    }

    /// Whether to hand over a new picture of `key` now: a recording with video runs and the last one is a frame old.
    pub fn wants_pic(&self, key: &str) -> bool {
        self.video.load(Ordering::Relaxed) && self.inner.lock().is_ok_and(|g| g.pic_at.get(key).is_none_or(|t| t.elapsed() >= Duration::from_millis(1000 / FPS as u64 - 5)))
    }

    /// The newest picture of a camera (`me`, a user id) or a screen share (`…:screen`).
    pub fn pic(&self, key: &str, i420: &mut I420Buffer, screen: bool) {
        let max_w = if screen { W } else { W / 2 };
        let (sw, sh) = (i420.width().max(1), i420.height().max(1));
        let w = max_w.min(sw) & !1;
        let h = ((w as u64 * sh as u64 / sw as u64) as u32 & !1).max(2);
        let scaled = i420.scale(w as i32, h as i32);
        let mut rgba = vec![0u8; (w * h * 4) as usize];
        scaled.to_argb(VideoFormatType::ABGR, &mut rgba, w * 4, w as i32, h as i32);
        if let Ok(mut g) = self.inner.lock() {
            g.pic_at.insert(key.to_string(), Instant::now());
            g.pics.insert(key.to_string(), Arc::new(Pic { w, h, rgba }));
        }
    }

    pub fn forget_pic(&self, key: &str) {
        if let Ok(mut g) = self.inner.lock() {
            g.pics.remove(key);
            g.pic_at.remove(key);
        }
    }

    pub fn set_people(&self, people: Vec<(String, String)>) {
        if let Ok(mut g) = self.inner.lock() {
            g.people = people;
        }
    }

    pub fn set_speaking(&self, keys: HashSet<String>) {
        if let Ok(mut g) = self.inner.lock() {
            g.speaking = keys;
        }
    }

    fn start(&self, video: bool) {
        if let Ok(mut g) = self.inner.lock() {
            g.audio.clear();
            g.pics.clear();
            g.pic_at.clear();
        }
        self.video.store(video, Ordering::Relaxed);
        self.on.store(true, Ordering::Relaxed);
    }

    fn stop(&self) {
        self.on.store(false, Ordering::Relaxed);
        self.video.store(false, Ordering::Relaxed);
        if let Ok(mut g) = self.inner.lock() {
            g.audio.clear();
            g.pics.clear();
            g.pic_at.clear();
        }
    }

    /// The next `out.len()` samples of every track mixed (silence where a track has nothing buffered).
    fn mix(&self, out: &mut [i16]) {
        let mut acc = vec![0i32; out.len()];
        if let Ok(mut g) = self.inner.lock() {
            for q in g.audio.values_mut() {
                if !q.primed {
                    if q.buf.len() < PRIME {
                        continue;
                    }
                    q.primed = true;
                }
                let n = out.len().min(q.buf.len());
                for (a, s) in acc.iter_mut().zip(q.buf.drain(..n)) {
                    *a += s as i32;
                }
                if n < out.len() {
                    q.primed = false; // ran dry: wait for a new cushion
                }
            }
        }
        for (o, a) in out.iter_mut().zip(acc) {
            *o = a.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        }
    }
}

/// Recordings finishing (mux included) when the program quits: `shutdown` stops them and waits.
static RUNNING: Mutex<Vec<std::thread::JoinHandle<()>>> = Mutex::new(Vec::new());
static SHUTDOWN: AtomicBool = AtomicBool::new(false);

/// Stops every recording and waits until its files are written. Transcripts not started yet are skipped
/// (`soworker --transcribe` makes them later).
pub fn shutdown() {
    SHUTDOWN.store(true, Ordering::Relaxed);
    let handles: Vec<_> = RUNNING.lock().map(|mut r| r.drain(..).collect()).unwrap_or_default();
    for h in handles {
        let _ = h.join();
    }
}

/// A running recording; dropping it does not stop it, `stop` does.
pub struct Recorder {
    stop: Arc<AtomicBool>,
}

impl Recorder {
    /// `title` names the files (after the date and time).
    pub fn start(tap: Arc<Tap>, title: &str, emit: Emit) -> Result<Recorder, String> {
        let dir = recordings_dir();
        std::fs::create_dir_all(&dir).map_err(|e| format!("cannot record to {}: {e}", dir.display()))?;
        let base = dir.join(file_name(&local_stamp(), title));
        let enc = if !cfg!(feature = "gui") {
            None
        } else if !has_ffmpeg() {
            emit(MediaEvent::Info("ffmpeg is not installed: recording the sound only".into()));
            None
        } else {
            Some(Encoder::start(&base)?)
        };
        let wav = Wav::create(&sibling(&base, ".wav"), RATE as u32)?;
        eprintln!("record: {} ({})", base.display(), if enc.is_some() { "video" } else { "sound only" });
        tap.start(enc.is_some());
        let stop = Arc::new(AtomicBool::new(false));
        let stop2 = stop.clone();
        let handle = std::thread::Builder::new().name("record".into()).spawn(move || run(tap, base, wav, enc, stop2, emit)).map_err(|e| e.to_string())?;
        if let Ok(mut r) = RUNNING.lock() {
            r.retain(|h| !h.is_finished());
            r.push(handle);
        }
        Ok(Recorder { stop })
    }

    pub fn stop(self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

/// The `recordingsDir` setting, else Videos/SoWorker in the home folder.
fn recordings_dir() -> PathBuf {
    if let Some(d) = crate::calls::setting("recordingsDir").as_str().filter(|s| !s.is_empty()) {
        return PathBuf::from(d);
    }
    let home = std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" }).map_or_else(std::env::temp_dir, PathBuf::from);
    home.join("Videos").join("SoWorker")
}

/// "2026-11-03 14-05 Design review": no characters a file system refuses, and no dots (they would read as extensions).
fn file_name(stamp: &str, title: &str) -> String {
    let title: String = title.chars().map(|c| if c.is_alphanumeric() || " -_()'&,".contains(c) { c } else { ' ' }).collect();
    let title = title.split_whitespace().collect::<Vec<_>>().join(" ");
    let title: String = title.chars().take(60).collect();
    if title.is_empty() { format!("{stamp} Meeting") } else { format!("{stamp} {title}") }
}

/// The local date and time as "2026-11-03 14-05".
#[cfg(unix)]
fn local_stamp() -> String {
    // SAFETY: localtime_r only writes the struct we hand it.
    let tm = unsafe {
        let t = libc::time(std::ptr::null_mut());
        let mut tm: libc::tm = std::mem::zeroed();
        libc::localtime_r(&t, &mut tm);
        tm
    };
    format!("{:04}-{:02}-{:02} {:02}-{:02}", tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday, tm.tm_hour, tm.tm_min)
}

#[cfg(windows)]
fn local_stamp() -> String {
    // SAFETY: GetLocalTime only fills the struct we hand it.
    let t = unsafe {
        let mut t: windows_sys::Win32::Foundation::SYSTEMTIME = std::mem::zeroed();
        windows_sys::Win32::System::SystemInformation::GetLocalTime(&mut t);
        t
    };
    format!("{:04}-{:02}-{:02} {:02}-{:02}", t.wYear, t.wMonth, t.wDay, t.wHour, t.wMinute)
}

fn has_ffmpeg() -> bool {
    Command::new("ffmpeg").arg("-version").stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).status().is_ok_and(|s| s.success())
}

/// ffmpeg turning our raw pictures into a video-only MP4 (muxed with the sound at the end).
struct Encoder {
    child: Child,
    path: PathBuf,
}

impl Encoder {
    fn start(base: &Path) -> Result<Encoder, String> {
        let path = sibling(base, ".video.mp4");
        // H.264 where this ffmpeg has an encoder for it (distributions differ), else MPEG-4 part 2, which every build has
        let encoders = Command::new("ffmpeg").args(["-hide_banner", "-encoders"]).stdin(Stdio::null()).stderr(Stdio::null()).output().map(|o| String::from_utf8_lossy(&o.stdout).into_owned()).unwrap_or_default();
        let has = |name: &str| encoders.lines().any(|l| l.split_whitespace().nth(1) == Some(name));
        let codec: &[&str] = if has("libx264") {
            &["-c:v", "libx264", "-preset", "veryfast", "-crf", "26"]
        } else if has("libopenh264") {
            &["-c:v", "libopenh264", "-b:v", "1500k"]
        } else {
            &["-c:v", "mpeg4", "-q:v", "5"]
        };
        let size = format!("{W}x{H}");
        let fps = FPS.to_string();
        let child = Command::new("ffmpeg")
            .args(["-hide_banner", "-loglevel", "error", "-y", "-f", "rawvideo", "-pix_fmt", "rgba", "-s", &size, "-r", &fps, "-i", "-"])
            .args(codec)
            .args(["-pix_fmt", "yuv420p"])
            .arg(&path)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
            .map_err(|e| format!("cannot start ffmpeg: {e}"))?;
        eprintln!("record: encoding with {}", codec[1]);
        Ok(Encoder { child, path })
    }

    fn write(&mut self, rgba: &[u8]) -> std::io::Result<()> {
        self.child.stdin.as_mut().ok_or(std::io::ErrorKind::BrokenPipe)?.write_all(rgba)
    }

    fn finish(mut self) -> Result<PathBuf, String> {
        drop(self.child.stdin.take()); // end of input: ffmpeg writes the file out and exits
        match self.child.wait() {
            Ok(s) if s.success() => Ok(self.path),
            Ok(s) => Err(format!("ffmpeg ended with {s}")),
            Err(e) => Err(e.to_string()),
        }
    }
}

fn run(tap: Arc<Tap>, base: PathBuf, mut wav: Wav, mut enc: Option<Encoder>, stop: Arc<AtomicBool>, emit: Emit) {
    #[cfg(feature = "gui")]
    let mut canvas = enc.as_ref().and_then(|_| compose::Canvas::new());
    #[cfg(feature = "gui")]
    if canvas.is_none() {
        enc = None;
    }
    let t0 = Instant::now();
    let (mut samples, mut frames) = (0usize, 0u64);
    let mut buf = Vec::new();
    // paced against the clock: both streams are as long as the wall time, whatever a tick costs
    let (step, mut next) = (Duration::from_secs(1) / FPS, t0);
    while !stop.load(Ordering::Relaxed) && !SHUTDOWN.load(Ordering::Relaxed) {
        next += step;
        match next.checked_duration_since(Instant::now()) {
            Some(d) => std::thread::sleep(d),
            None => next = Instant::now(),
        }
        let secs = t0.elapsed().as_secs_f64();
        let target = (secs * RATE as f64) as usize;
        buf.clear();
        buf.resize(target - samples, 0);
        tap.mix(&mut buf);
        samples = target;
        if let Err(e) = wav.write(&buf) {
            emit(MediaEvent::Error(format!("recording stopped: {e}")));
            break;
        }
        #[cfg(feature = "gui")]
        if let (Some(e), Some(c)) = (enc.as_mut(), canvas.as_mut()) {
            let want = (secs * FPS as f64) as u64 + 1;
            if frames < want {
                let img = c.draw(&tap);
                // behind (a slow tick): repeat the picture so the video keeps time with the sound
                while frames < want {
                    if let Err(err) = e.write(img) {
                        eprintln!("record: ffmpeg input: {err}");
                        emit(MediaEvent::Error("the video encoder stopped; recording the sound only".into()));
                        enc = None;
                        break;
                    }
                    frames += 1;
                }
            }
        }
    }
    tap.stop();
    eprintln!("record: stopped after {:.0}s, {frames} frames", t0.elapsed().as_secs_f64());
    finish(base, wav, enc, emit);
}

/// Writes the files out: the video muxed with the sound (else the WAV alone), then the transcript in the background.
fn finish(base: PathBuf, wav: Wav, enc: Option<Encoder>, emit: Emit) {
    let wav = match wav.finish() {
        Ok(p) => p,
        Err(e) => return emit(MediaEvent::Error(format!("recording lost: {e}"))),
    };
    let mut saved = wav.clone();
    if let Some(e) = enc {
        match e.finish().and_then(|video| mux(&video, &wav, &sibling(&base, ".mp4")).map(|mp4| (video, mp4))) {
            Ok((video, mp4)) => {
                let _ = std::fs::remove_file(video);
                saved = mp4;
            }
            Err(err) => {
                eprintln!("record: {err}");
                emit(MediaEvent::Error(format!("could not make the video ({err}); kept the sound")));
            }
        }
    }
    emit(MediaEvent::Info(format!("Recording saved: {}", saved.display())));
    if SHUTDOWN.load(Ordering::Relaxed) {
        if saved != wav {
            let _ = std::fs::remove_file(&wav);
        }
        eprintln!("record: quitting, no transcript (run soworker --transcribe \"{}\")", saved.display());
        return;
    }
    // not tracked by `shutdown`: a transcript can take minutes, and is easy to make again
    std::thread::spawn(move || {
        // the MP4 has the sound; without whisper, `soworker --transcribe` reads it from there later
        let res = transcribe::transcribe(&wav, &base, &|st| emit(MediaEvent::Transcript(st)));
        if saved != wav {
            let _ = std::fs::remove_file(&wav);
        }
        let res = res.and_then(|t| transcribe::attach(&saved, &t)).map_err(|e| e.replace(&format!("\"{}\"", wav.display()), &format!("\"{}\"", saved.display())));
        if let Err(e) = &res {
            eprintln!("transcribe: {e}");
        }
        emit(MediaEvent::Transcript(match res {
            Ok(path) => transcribe::Status::Done(path),
            Err(e) => transcribe::Status::Failed(e),
        }));
    });
}

fn mux(video: &Path, wav: &Path, out: &Path) -> Result<PathBuf, String> {
    let res = Command::new("ffmpeg")
        .args(["-hide_banner", "-loglevel", "error", "-y", "-i"])
        .arg(video)
        .args(["-ss", LATE, "-i"])
        .arg(wav)
        .args(["-map", "0:v", "-map", "1:a", "-c:v", "copy", "-c:a", "aac", "-b:a", "96k", "-movflags", "+faststart"])
        .arg(out)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("cannot run ffmpeg: {e}"))?;
    if !res.status.success() {
        return Err(format!("ffmpeg could not mux: {}", String::from_utf8_lossy(&res.stderr).trim()));
    }
    Ok(out.to_path_buf())
}

/// The recorded picture, drawn with the window client's software renderer: the same layout whatever the UI shows.
#[cfg(feature = "gui")]
mod compose {
    use super::{Pic, Tap, H, W};
    use crate::gui::icon::rounded_rect;
    use crate::gui::meeting_ui::R;
    use crate::gui::render::{hue_rgb, Text};
    use crate::gui::theme;
    use std::sync::Arc;
    use tiny_skia::{Color, FillRule, FilterQuality, Paint, PathBuilder, Pattern, Pixmap, PixmapRef, SpreadMode, Stroke, Transform};

    const PAD: f32 = 16.0;
    const GAP: f32 = 12.0;
    /// Width of the column of people beside a screen share.
    const SIDE: f32 = 288.0;

    pub struct Canvas {
        pm: Pixmap,
        text: Text,
    }

    impl Canvas {
        pub fn new() -> Option<Canvas> {
            Some(Canvas { pm: Pixmap::new(W, H)?, text: Text::new() })
        }

        /// The picture now, as RGBA.
        pub fn draw(&mut self, tap: &Tap) -> &[u8] {
            let (people, speaking, pics) = match tap.inner.lock() {
                Ok(g) => (g.people.clone(), g.speaking.clone(), g.pics.clone()),
                Err(_) => Default::default(),
            };
            let [r, g, b] = theme::BG;
            self.pm.fill(Color::from_rgba8(r, g, b, 255));
            let mut shares: Vec<&String> = pics.keys().filter(|k| k.ends_with(":screen")).collect();
            shares.sort();
            let tile = |c: &mut Canvas, (key, name): &(String, String), r: R| {
                draw_tile(c, r, key, name, pics.get(key), speaking.contains(key));
            };
            match shares.first() {
                Some(share) => {
                    let (main, side) = with_share(people.len());
                    let owner = share.trim_end_matches(":screen");
                    let label = people.iter().find(|(k, _)| k == owner).map_or_else(|| "Screen share".to_string(), |(_, n)| format!("{n}'s screen"));
                    draw_share(self, main, &pics[*share], &label);
                    // speakers first, so whoever talks stays in the column when it is full
                    let mut order: Vec<&(String, String)> = people.iter().collect();
                    order.sort_by_key(|(k, _)| !speaking.contains(k));
                    for (p, r) in order.into_iter().zip(side) {
                        tile(self, p, r);
                    }
                }
                None => {
                    for (p, r) in people.iter().zip(gallery(people.len().max(1))) {
                        tile(self, p, r);
                    }
                }
            }
            self.pm.data()
        }
    }

    /// Cells for `n` people over the whole picture: the grid whose cells hold the biggest 16:9 video.
    pub fn gallery(n: usize) -> Vec<R> {
        let area = R { x: PAD, y: PAD, w: W as f32 - 2.0 * PAD, h: H as f32 - 2.0 * PAD };
        let fit = |cols: usize| {
            let rows = n.div_ceil(cols);
            let (cw, ch) = ((area.w - GAP * (cols - 1) as f32) / cols as f32, (area.h - GAP * (rows - 1) as f32) / rows as f32);
            (cw.min(ch * 16.0 / 9.0), cw, ch, rows)
        };
        let cols = (1..=n).max_by(|a, b| fit(*a).0.total_cmp(&fit(*b).0)).unwrap_or(1);
        let (_, cw, ch, rows) = fit(cols);
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let (row, col) = (i / cols, i % cols);
            // a short last row is centred
            let in_row = if row == rows - 1 { n - row * cols } else { cols };
            let x0 = area.x + (area.w - (in_row as f32 * cw + (in_row - 1) as f32 * GAP)) / 2.0;
            out.push(R { x: x0 + col as f32 * (cw + GAP), y: area.y + row as f32 * (ch + GAP), w: cw, h: ch });
        }
        out
    }

    /// The share's area, and the people's cells in the column beside it (as many as fit, at 16:10).
    pub fn with_share(n: usize) -> (R, Vec<R>) {
        let main = R { x: PAD, y: PAD, w: W as f32 - 2.0 * PAD - GAP - SIDE, h: H as f32 - 2.0 * PAD };
        let th = SIDE * 10.0 / 16.0;
        let fits = ((main.h + GAP) / (th + GAP)) as usize;
        let side = (0..n.min(fits)).map(|i| R { x: main.x + main.w + GAP, y: PAD + i as f32 * (th + GAP), w: SIDE, h: th }).collect();
        (main, side)
    }

    fn fill(pm: &mut Pixmap, r: R, radius: f32, rgb: [u8; 3], a: u8) {
        if let Some(p) = rounded_rect(r.x, r.y, r.w, r.h, radius) {
            let mut paint = Paint::default();
            paint.set_color_rgba8(rgb[0], rgb[1], rgb[2], a);
            paint.anti_alias = true;
            pm.fill_path(&p, &paint, FillRule::Winding, Transform::identity(), None);
        }
    }

    /// The picture scaled into `r`: filling it (cropped) or whole (letterboxed).
    fn image(pm: &mut Pixmap, pic: &Pic, r: R, radius: f32, cover: bool) {
        let Some(src) = PixmapRef::from_bytes(&pic.rgba, pic.w, pic.h) else { return };
        let (iw, ih) = (pic.w as f32, pic.h as f32);
        let s = if cover { (r.w / iw).max(r.h / ih) } else { (r.w / iw).min(r.h / ih) };
        let (dw, dh) = (iw * s, ih * s);
        let (ox, oy) = (r.x + (r.w - dw) / 2.0, r.y + (r.h - dh) / 2.0);
        let shader = Pattern::new(src, SpreadMode::Pad, FilterQuality::Bilinear, 1.0, Transform::from_scale(s, s).post_translate(ox, oy));
        let paint = Paint { shader, anti_alias: true, ..Paint::default() };
        let shape = if cover { r } else { R { x: ox, y: oy, w: dw, h: dh } };
        if let Some(p) = rounded_rect(shape.x, shape.y, shape.w, shape.h, if cover { radius } else { 0.0 }) {
            pm.fill_path(&p, &paint, FillRule::Winding, Transform::identity(), None);
        }
    }

    fn label(c: &mut Canvas, s: &str, x: f32, bottom: f32, px: f32) {
        let w = c.text.width(s, px) + px;
        let h = px * 1.6;
        fill(&mut c.pm, R { x, y: bottom - h, w, h }, h / 2.0, [0, 0, 0], 150);
        c.text.draw(&mut c.pm.as_mut(), s, x + px / 2.0, bottom - h / 2.0 + px * 0.36, px, [255, 255, 255], 0.0);
    }

    fn draw_tile(c: &mut Canvas, r: R, key: &str, name: &str, pic: Option<&Arc<Pic>>, speaking: bool) {
        let radius = (r.h * 0.05).clamp(6.0, 14.0);
        fill(&mut c.pm, r, radius, theme::SURFACE, 255);
        match pic {
            Some(p) => image(&mut c.pm, p, r, radius, true),
            None => {
                // no camera: the first letter on a disc in the person's colour, as the meeting tiles show it
                let d = r.w.min(r.h) * 0.36;
                let (cr, cg, cb) = hue_rgb(key);
                if let Some(disc) = PathBuilder::from_circle(r.x + r.w / 2.0, r.y + r.h / 2.0 - r.h * 0.04, d / 2.0) {
                    let mut paint = Paint::default();
                    paint.set_color_rgba8(cr, cg, cb, 255);
                    paint.anti_alias = true;
                    c.pm.fill_path(&disc, &paint, FillRule::Winding, Transform::identity(), None);
                }
                let initial: String = name.chars().next().map(|ch| ch.to_uppercase().collect()).unwrap_or_default();
                let px = d * 0.5;
                c.text.draw(&mut c.pm.as_mut(), &initial, r.x + r.w / 2.0, r.y + r.h / 2.0 - r.h * 0.04 + px * 0.36, px, [255, 255, 255], 0.5);
            }
        }
        let px = (r.h * 0.07).clamp(12.0, 20.0);
        label(c, name, r.x + px * 0.5, r.y + r.h - px * 0.5, px);
        if speaking {
            if let Some(p) = rounded_rect(r.x + 1.5, r.y + 1.5, r.w - 3.0, r.h - 3.0, radius) {
                let mut paint = Paint::default();
                let [sr, sg, sb] = theme::SPEAKING;
                paint.set_color_rgba8(sr, sg, sb, 255);
                paint.anti_alias = true;
                c.pm.stroke_path(&p, &paint, &Stroke { width: 3.0, ..Stroke::default() }, Transform::identity(), None);
            }
        }
    }

    fn draw_share(c: &mut Canvas, r: R, pic: &Pic, name: &str) {
        fill(&mut c.pm, r, 10.0, [0, 0, 0], 255);
        image(&mut c.pm, pic, r, 10.0, false);
        label(c, name, r.x + 10.0, r.y + r.h - 10.0, 16.0);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn the_gallery_stays_inside_the_picture() {
            for n in 1..=20 {
                let cells = gallery(n);
                assert_eq!(cells.len(), n);
                for c in &cells {
                    assert!(c.x >= PAD - 0.01 && c.y >= PAD - 0.01 && c.x + c.w <= W as f32 - PAD + 0.01 && c.y + c.h <= H as f32 - PAD + 0.01, "{n} people: {c:?}");
                }
            }
            let one = gallery(1)[0];
            assert_eq!((one.w, one.h), (W as f32 - 2.0 * PAD, H as f32 - 2.0 * PAD), "one person fills the picture");
            let four = gallery(4);
            assert_eq!(four[0].y, four[1].y, "four people make two rows of two");
            assert!(four[2].y > four[0].y);
        }

        #[test]
        fn a_share_keeps_a_column_of_people() {
            let (main, side) = with_share(10);
            assert_eq!(side.len(), 4, "four 16:10 tiles fit beside it");
            assert!(side.iter().all(|r| r.x > main.x + main.w && r.y + r.h <= H as f32 - PAD + 0.01));
            assert_eq!(with_share(2).1.len(), 2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_track_is_mixed_once_it_has_a_cushion() {
        let tap = Tap::default();
        tap.start(false);
        tap.push_audio("a", &vec![100; PRIME - 1]);
        let mut out = vec![0i16; 10];
        tap.mix(&mut out);
        assert!(out.iter().all(|&s| s == 0), "not enough buffered yet");
        tap.push_audio("a", &[100]);
        tap.push_audio("b", &vec![i16::MAX; PRIME]);
        tap.mix(&mut out);
        assert!(out.iter().all(|&s| s == i16::MAX), "both mixed, clipped");
        tap.stop();
        tap.push_audio("a", &[1]);
        assert!(tap.inner.lock().unwrap().audio.is_empty(), "nothing kept while not recording");
    }

    /// Records two seconds of a made-up call through ffmpeg into `$SOWORKER_REC_TEST` (needs ffmpeg; writes files).
    #[test]
    #[ignore]
    fn records_a_short_call() {
        let dir = PathBuf::from(std::env::var("SOWORKER_REC_TEST").expect("set SOWORKER_REC_TEST to an empty folder"));
        std::env::set_var("XDG_CONFIG_HOME", &dir);
        crate::calls::save_setting("recordingsDir", dir.to_string_lossy().into_owned().into());
        let tap = Arc::new(Tap::default());
        tap.set_people(vec![("me".into(), "Ana".into()), ("bo".into(), "Bo".into()), ("cy".into(), "Cy".into())]);
        tap.set_speaking(["bo".to_string()].into());
        let log = Arc::new(Mutex::new(Vec::new()));
        let l2 = log.clone();
        let rec = Recorder::start(tap.clone(), "Test: room", Arc::new(move |e| l2.lock().unwrap().push(format!("{e:?}")))).unwrap();
        let mut cam = I420Buffer::new(640, 480);
        let (y, u, v) = cam.data_mut();
        y.fill(150);
        u.fill(60);
        v.fill(200);
        for i in 0..100 {
            let tone: Vec<i16> = (0..960).map(|n| ((((i * 960 + n) as f32) * 440.0 * std::f32::consts::TAU / RATE as f32).sin() * 8000.0) as i16).collect();
            tap.push_audio("bo:TR", &tone);
            if tap.wants_pic("bo") {
                tap.pic("bo", &mut cam, false);
            }
            if i >= 60 && tap.wants_pic("cy:screen") {
                tap.pic("cy:screen", &mut I420Buffer::new(1920, 1080), true); // a share in the last second
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        rec.stop();
        shutdown();
        println!("{:#?}", log.lock().unwrap());
    }

    #[test]
    fn file_names_are_safe() {
        assert_eq!(file_name("2026-11-03 14-05", "Design: review v2.0 / Q4"), "2026-11-03 14-05 Design review v2 0 Q4");
        assert_eq!(file_name("2026-11-03 14-05", "  "), "2026-11-03 14-05 Meeting");
    }
}
