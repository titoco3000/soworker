//! Our own microphone path, used while noise cancellation is on. libwebrtc's platform audio gives no way to process the
//! mic before it is sent, so we capture it ourselves (`parec` / `pw-record` on Linux, WASAPI through cpal on Windows),
//! run WebRTC's audio processing on it (echo cancellation against what we play, gain control, no noise suppression),
//! then DeepFilterNet 3, and push the result into a `NativeAudioSource`. Playout stays with the platform audio.
//!
//! Echo cancellation needs what the speakers play: every remote audio track is also read here (`Far`), mixed, and fed
//! to the processing as its reverse stream, 10 ms for every 10 ms of microphone.
use crate::calls::MediaEvent;
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

pub const RATE: u32 = 48_000;
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
        let c = match cap.as_mut() {
            Some(c) => c,
            None => {
                far.set_on(true);
                cap.insert(Capture::open()?)
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

/// The default microphone as 48 kHz mono 16-bit: whichever of PulseAudio's `parec` and PipeWire's `pw-record` is there
/// (both reach the same server on a PipeWire desktop), so no audio library is linked.
#[cfg(not(windows))]
struct Capture {
    child: std::process::Child,
    out: std::process::ChildStdout,
    buf: [u8; FRAME * 2],
}

#[cfg(not(windows))]
impl Capture {
    fn open() -> Result<Self, String> {
        use std::process::{Command, Stdio};
        let tools: [(&str, &[&str]); 2] = [
            ("parec", &["--raw", "--format=s16le", "--rate=48000", "--channels=1", "--latency-msec=20", "--client-name=SoWorker", "--stream-name=Microphone"]),
            ("pw-record", &["--raw", "--format", "s16", "--rate", "48000", "--channels", "1", "--latency", "20ms", "-"]),
        ];
        let mut errors = vec![];
        for (tool, args) in tools {
            match Command::new(tool).args(args).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn() {
                Ok(mut child) => {
                    let out = child.stdout.take().expect("piped stdout");
                    eprintln!("mic: capturing with {tool}");
                    return Ok(Capture { child, out, buf: [0; FRAME * 2] });
                }
                Err(e) => errors.push(format!("{tool}: {e}")),
            }
        }
        Err(format!("cannot open the microphone ({})", errors.join(", ")))
    }

    fn read(&mut self, frame: &mut [i16; FRAME]) -> std::io::Result<()> {
        use std::io::Read;
        self.out.read_exact(&mut self.buf)?;
        for (s, b) in frame.iter_mut().zip(self.buf.chunks_exact(2)) {
            *s = i16::from_le_bytes([b[0], b[1]]);
        }
        Ok(())
    }
}

#[cfg(not(windows))]
impl Drop for Capture {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// The default microphone through WASAPI (cpal): whatever format the device offers, mixed down to mono and brought to
/// 48 kHz (linear interpolation; shared-mode devices almost always run at 48 kHz already).
#[cfg(windows)]
struct Capture {
    _stream: cpal::Stream,
    rx: std::sync::mpsc::Receiver<Vec<f32>>,
    pending: VecDeque<f32>,
}

#[cfg(windows)]
impl Capture {
    fn open() -> Result<Self, String> {
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
        let dev = cpal::default_host().default_input_device().ok_or("no microphone")?;
        let cfg = dev.default_input_config().map_err(|e| e.to_string())?;
        let (ch, rate) = (cfg.channels().max(1) as usize, cfg.sample_rate());
        let (tx, rx) = std::sync::mpsc::channel();
        let mut rs = Resampler::new(rate, RATE);
        let mut push = move |mono: Vec<f32>| {
            let _ = tx.send(rs.run(&mono));
        };
        let err = |e| eprintln!("mic: stream error: {e}");
        macro_rules! build {
            ($t:ty, $to:expr) => {
                dev.build_input_stream(cfg.config(), move |d: &[$t], _: &_| push(d.chunks(ch).map(|f| f.iter().map(|&s| $to(s)).sum::<f32>() / ch as f32).collect()), err, None)
            };
        }
        let stream = match cfg.sample_format() {
            cpal::SampleFormat::F32 => build!(f32, |s: f32| s),
            cpal::SampleFormat::I16 => build!(i16, |s: i16| s as f32 / 32768.0),
            cpal::SampleFormat::I32 => build!(i32, |s: i32| s as f32 / 2147483648.0),
            f => return Err(format!("unsupported microphone format {f}")),
        }
        .map_err(|e| e.to_string())?;
        stream.play().map_err(|e| e.to_string())?;
        eprintln!("mic: capturing with WASAPI ({ch} channels, {rate} Hz)");
        Ok(Capture { _stream: stream, rx, pending: VecDeque::new() })
    }

    fn read(&mut self, frame: &mut [i16; FRAME]) -> std::io::Result<()> {
        while self.pending.len() < FRAME {
            let chunk = self.rx.recv_timeout(std::time::Duration::from_secs(2)).map_err(|_| std::io::Error::new(std::io::ErrorKind::TimedOut, "no audio from the microphone"))?;
            self.pending.extend(chunk);
        }
        for (s, v) in frame.iter_mut().zip(self.pending.drain(..FRAME)) {
            *s = (v * 32768.0).clamp(-32768.0, 32767.0) as i16;
        }
        Ok(())
    }
}

/// Linear-interpolation resampler for one continuous mono stream.
#[cfg_attr(not(any(windows, test)), allow(dead_code))]
struct Resampler {
    step: f64,
    pos: f64,
    last: f32,
}

#[cfg_attr(not(any(windows, test)), allow(dead_code))]
impl Resampler {
    fn new(from: u32, to: u32) -> Self {
        Resampler { step: from as f64 / to as f64, pos: 0.0, last: 0.0 }
    }

    /// `pos` runs over [last, input...]: index 0 is the previous call's final sample.
    fn run(&mut self, input: &[f32]) -> Vec<f32> {
        if self.step == 1.0 {
            return input.to_vec();
        }
        let at = |i: usize| if i == 0 { self.last } else { input[i - 1] };
        let mut out = Vec::with_capacity((input.len() as f64 / self.step) as usize + 1);
        while self.pos + 1.0 <= input.len() as f64 {
            let i = self.pos as usize;
            let f = (self.pos - i as f64) as f32;
            out.push(at(i) + (at(i + 1) - at(i)) * f);
            self.pos += self.step;
        }
        self.pos -= input.len() as f64;
        if let Some(&l) = input.last() {
            self.last = l;
        }
        out
    }
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

    #[test]
    fn resampler_keeps_the_rate() {
        let mut r = Resampler::new(44_100, 48_000);
        let n: usize = (0..100).map(|_| r.run(&[0.5; 441]).len()).sum();
        assert!((n as i64 - 48_000).abs() <= 1, "{n}");
        let mut same = Resampler::new(48_000, 48_000);
        assert_eq!(same.run(&[0.25; 10]), vec![0.25; 10]);
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
