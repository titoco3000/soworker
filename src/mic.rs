//! Our own microphone path, used while noise cancellation is on. libwebrtc's platform audio gives no way to process the
//! mic before it is sent, so we capture it ourselves (`devices::Capture`: `parec` / `pw-record` on Linux, WASAPI on Windows),
//! run WebRTC's audio processing on it (echo cancellation against what we play, gain control, no noise suppression),
//! then DeepFilterNet 3, and push the result into a `NativeAudioSource`. Playout stays with the platform audio.
//!
//! Echo cancellation needs what the speakers play: every remote audio track is also read here (`Far`), mixed, and fed
//! to the processing as its reverse stream, 10 ms for every 10 ms of microphone.
use crate::calls::MediaEvent;
use crate::devices::{self, Capture};
use df::tract::{DfParams, DfTract, RuntimeParams};
use futures_util::FutureExt;
use livekit::webrtc::audio_frame::AudioFrame;
use livekit::webrtc::audio_source::native::NativeAudioSource;
use livekit::webrtc::native::apm::AudioProcessingModule;
use ndarray::Array2;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

type Emit = Arc<dyn Fn(MediaEvent) + Send + Sync>;

pub use crate::devices::RATE;
/// 10 ms at 48 kHz mono: the unit of everything here (WebRTC processing, DeepFilterNet's hop, the source's fast path).
const FRAME: usize = 480;
/// At most this much remote audio waits per track (200 ms); older samples are dropped so the clocks cannot drift apart.
const FAR_MAX: usize = FRAME * 20;

/// The remote audio as the speakers get it, per track, waiting to be mixed into the echo reference.
#[derive(Default)]
pub struct Far {
    on: AtomicBool,
    queues: Mutex<HashMap<String, VecDeque<i16>>>,
}

impl Far {
    /// Samples of one remote track (48 kHz mono). Ignored while no noise-cancelled mic runs.
    pub fn push(&self, key: &str, samples: &[i16]) {
        if !self.on.load(Ordering::Relaxed) {
            return;
        }
        if let Ok(mut q) = self.queues.lock() {
            let q = q.entry(key.to_string()).or_default();
            q.extend(samples);
            let extra = q.len().saturating_sub(FAR_MAX);
            q.drain(..extra);
        }
    }

    pub fn remove(&self, key: &str) {
        if let Ok(mut q) = self.queues.lock() {
            q.remove(key);
        }
    }

    pub(crate) fn set_on(&self, on: bool) {
        self.on.store(on, Ordering::Relaxed);
        if let (false, Ok(mut q)) = (on, self.queues.lock()) {
            q.clear();
        }
    }

    /// The next 10 ms of everything mixed (silence where a track has nothing yet).
    fn take(&self, out: &mut [i16; FRAME]) {
        let mut mix = [0i32; FRAME];
        if let Ok(mut q) = self.queues.lock() {
            for q in q.values_mut() {
                for (m, s) in mix.iter_mut().zip(q.drain(..FRAME.min(q.len()))) {
                    *m += s as i32;
                }
            }
        }
        for (o, m) in out.iter_mut().zip(mix) {
            *o = m.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        }
    }
}

fn load_model() -> Result<DfTract, String> {
    let t = std::time::Instant::now();
    let m = DfTract::new(DfParams::default(), &RuntimeParams::default_with_ch(1)).map_err(|e| format!("{e:#}"))?;
    eprintln!("mic: DeepFilterNet loaded in {} ms (sample rate {}, hop {})", t.elapsed().as_millis(), m.sr, m.hop_size);
    if m.sr != RATE as usize || m.hop_size != FRAME {
        return Err(format!("unexpected model: {} Hz, hop {}", m.sr, m.hop_size));
    }
    Ok(m)
}

/// The noise-cancelled microphone of one call: a thread that holds the model (loaded once, it takes a moment) and
/// opens the device while `capture` is on. Dropping it stops the thread and closes the device.
pub struct Mic {
    stop: Arc<AtomicBool>,
    want: Arc<AtomicBool>,
}

impl Mic {
    /// Starts the thread (the device stays closed until `capture(true)`); cleaned audio goes into `source`.
    pub fn start(source: NativeAudioSource, far: Arc<Far>, emit: Emit) -> Mic {
        let (stop, want) = (Arc::new(AtomicBool::new(false)), Arc::new(AtomicBool::new(false)));
        let (stop2, want2) = (stop.clone(), want.clone());
        std::thread::Builder::new()
            .name("mic".into())
            .spawn(move || {
                if let Err(e) = run(&source, &far, &stop2, &want2) {
                    eprintln!("mic: {e}");
                    emit(MediaEvent::Error(format!("noise cancellation: {e} (turn it off to use the plain microphone)")));
                }
                far.set_on(false);
            })
            .expect("mic thread");
        Mic { stop, want }
    }

    /// Open (true) or close the microphone.
    pub fn capture(&self, on: bool) {
        self.want.store(on, Ordering::Relaxed);
    }
}

impl Drop for Mic {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

fn run(source: &NativeAudioSource, far: &Far, stop: &AtomicBool, want: &AtomicBool) -> Result<(), String> {
    let mut model = load_model()?;
    let mut cap: Option<Capture> = None;
    // the microphone chosen in the profile panel; a new choice reopens the capture
    let mut device = devices::chosen().input;
    let mut apm = AudioProcessingModule::new(true, true, true, false);
    let (mut near, mut rev) = ([0i16; FRAME], [0i16; FRAME]);
    let (mut noisy, mut clean) = (Array2::<f32>::zeros((1, FRAME)), Array2::<f32>::zeros((1, FRAME)));
    while !stop.load(Ordering::Relaxed) {
        if !want.load(Ordering::Relaxed) {
            if cap.take().is_some() {
                far.set_on(false);
                eprintln!("mic: closed");
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
            continue;
        }
        let now = devices::chosen().input;
        if now != device {
            device = now;
            cap = None;
        }
        let c = match cap.as_mut() {
            Some(c) => c,
            None => {
                far.set_on(true);
                cap.insert(Capture::open(device.as_deref())?)
            }
        };
        if let Err(e) = c.read(&mut near) {
            return Err(format!("microphone: {e}"));
        }
        far.take(&mut rev);
        let _ = apm.process_reverse_stream(&mut rev, RATE as i32, 1);
        let _ = apm.process_stream(&mut near, RATE as i32, 1);
        for (n, s) in noisy.iter_mut().zip(near) {
            *n = s as f32 / 32768.0;
        }
        if model.process(noisy.view(), clean.view_mut()).is_ok() {
            for (s, c) in near.iter_mut().zip(clean.iter()) {
                *s = (c * 32767.0).clamp(-32768.0, 32767.0) as i16;
            }
        }
        // no queue on the source: a 10 ms frame goes straight through, the future is ready at once
        let frame = AudioFrame { data: near[..].into(), sample_rate: RATE, num_channels: 1, samples_per_channel: FRAME as u32 };
        let _ = source.capture_frame(&frame).now_or_never();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn far_mixes_and_caps() {
        let far = Far::default();
        far.push("a", &[100; FRAME]);
        assert!(far.queues.lock().unwrap().is_empty(), "nothing is kept while off");
        far.set_on(true);
        far.push("a", &[100; FRAME]);
        far.push("b", &[30_000; FRAME / 2]);
        far.push("c", &[0; FRAME * 30]);
        let mut out = [0; FRAME];
        far.take(&mut out);
        assert_eq!((out[0], out[FRAME - 1]), (30_100, 100));
        assert_eq!(far.queues.lock().unwrap()["c"].len(), FAR_MAX - FRAME);
        far.set_on(false);
        assert!(far.queues.lock().unwrap().is_empty());
    }
}

#[cfg(test)]
mod speed {
    #[test]
    #[ignore = "loads the model; run with --ignored to see the cost per 10 ms frame"]
    fn model_keeps_up() {
        let mut m = super::load_model().unwrap();
        let noisy = ndarray::Array2::<f32>::from_shape_fn((1, super::FRAME), |(_, i)| ((i * 7919) % 200) as f32 / 1000.0 - 0.1);
        let mut clean = ndarray::Array2::<f32>::zeros((1, super::FRAME));
        let t = std::time::Instant::now();
        for _ in 0..300 {
            m.process(noisy.view(), clean.view_mut()).unwrap();
        }
        let per = t.elapsed().as_secs_f64() * 1000.0 / 300.0;
        eprintln!("{per:.2} ms per 10 ms frame");
        assert!(per < 10.0);
    }
}
