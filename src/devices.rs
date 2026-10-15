//! Audio and video devices: what is there to choose from (the profile panel lists them), which ones the user chose
//! (`audioOutput`, `audioInput` and `camera` in settings.json; unset = the system default), our own microphone capture
//! (used by the noise-cancelled mic and by the level meter) and the level meter itself.
//!
//! Device ids are what the platform uses to open the device: PulseAudio / PipeWire node names and `/dev/videoN` on
//! Linux, the device names on Windows.
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, RwLock};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Output,
    Input,
    Camera,
}

impl Kind {
    pub const ALL: [Kind; 3] = [Kind::Output, Kind::Input, Kind::Camera];

    fn key(self) -> &'static str {
        match self {
            Kind::Output => "audioOutput",
            Kind::Input => "audioInput",
            Kind::Camera => "camera",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Kind::Output => "Speaker",
            Kind::Input => "Microphone",
            Kind::Camera => "Camera",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Device {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Default)]
pub struct Devices {
    pub outputs: Vec<Device>,
    pub inputs: Vec<Device>,
    pub cameras: Vec<Device>,
}

impl Devices {
    pub fn of(&self, k: Kind) -> &[Device] {
        match k {
            Kind::Output => &self.outputs,
            Kind::Input => &self.inputs,
            Kind::Camera => &self.cameras,
        }
    }
}

/// The devices to use; `None` = the system default.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Choice {
    pub output: Option<String>,
    pub input: Option<String>,
    pub camera: Option<String>,
}

impl Choice {
    pub fn get(&self, k: Kind) -> Option<&str> {
        match k {
            Kind::Output => self.output.as_deref(),
            Kind::Input => self.input.as_deref(),
            Kind::Camera => self.camera.as_deref(),
        }
        .filter(|id| !id.is_empty())
    }

    fn set(&mut self, k: Kind, id: Option<String>) {
        *match k {
            Kind::Output => &mut self.output,
            Kind::Input => &mut self.input,
            Kind::Camera => &mut self.camera,
        } = id;
    }
}

/// The choice, read from settings.json once and kept here (the sound player, the mic thread and the camera read it).
static CHOICE: RwLock<Option<Choice>> = RwLock::new(None);

pub fn chosen() -> Choice {
    if let Some(c) = CHOICE.read().ok().and_then(|c| c.clone()) {
        return c;
    }
    let mut c = Choice::default();
    for k in Kind::ALL {
        c.set(k, crate::calls::setting(k.key()).as_str().filter(|s| !s.is_empty()).map(String::from));
    }
    if let Ok(mut g) = CHOICE.write() {
        *g = Some(c.clone());
    }
    c
}

/// Use (and remember) this device; `None` goes back to the system default.
pub fn choose(k: Kind, id: Option<String>) {
    let mut c = chosen();
    c.set(k, id.clone());
    if let Ok(mut g) = CHOICE.write() {
        *g = Some(c);
    }
    crate::calls::save_setting(k.key(), id.map_or(serde_json::Value::Null, Into::into));
}

/// Everything that is plugged in right now. Asks the sound server and the kernel, so it takes a moment: call it when
/// the list is about to be shown, not every frame.
pub fn list() -> Devices {
    Devices { outputs: outputs(), inputs: inputs(), cameras: cameras() }
}

/// Speakers and microphones as PulseAudio / PipeWire (`pactl`) sees them; the monitors of outputs are left out.
#[cfg(not(windows))]
fn pulse(what: &str) -> Vec<Device> {
    use std::process::{Command, Stdio};
    let Ok(out) = Command::new("pactl").args(["-f", "json", "list", what]).stdin(Stdio::null()).stderr(Stdio::null()).output() else { return vec![] };
    parse_pulse(&out.stdout)
}

#[cfg_attr(windows, allow(dead_code))]
fn parse_pulse(json: &[u8]) -> Vec<Device> {
    let v: serde_json::Value = serde_json::from_slice(json).unwrap_or_default();
    v.as_array()
        .into_iter()
        .flatten()
        .filter_map(|d| {
            let id = d["name"].as_str()?.to_string();
            if id.ends_with(".monitor") || d["properties"]["device.class"].as_str() == Some("monitor") {
                return None;
            }
            let name = d["description"].as_str().filter(|n| !n.is_empty() && *n != "(null)").unwrap_or(&id).to_string();
            Some(Device { id, name })
        })
        .collect()
}

#[cfg(not(windows))]
fn outputs() -> Vec<Device> {
    pulse("sinks")
}

#[cfg(not(windows))]
fn inputs() -> Vec<Device> {
    pulse("sources")
}

/// Video capture devices. A camera usually has two nodes (pictures and metadata); only the first (`index` 0) is listed.
#[cfg(not(windows))]
fn cameras() -> Vec<Device> {
    let Ok(dir) = std::fs::read_dir("/sys/class/video4linux") else { return vec![] };
    let mut found: Vec<(u32, Device)> = dir
        .flatten()
        .filter_map(|e| {
            let node = e.file_name().to_string_lossy().to_string();
            let n: u32 = node.strip_prefix("video")?.parse().ok()?;
            let read = |f: &str| std::fs::read_to_string(e.path().join(f)).map(|s| s.trim().to_string()).unwrap_or_default();
            if read("index").parse::<u32>().unwrap_or(0) != 0 {
                return None;
            }
            let name = read("name");
            Some((n, Device { id: format!("/dev/{node}"), name: if name.is_empty() { node } else { name } }))
        })
        .collect();
    found.sort_by_key(|(n, _)| *n);
    found.into_iter().map(|(_, d)| d).collect()
}

#[cfg(all(windows, feature = "calls"))]
fn cpal_devices(input: bool) -> Vec<Device> {
    use cpal::traits::{DeviceTrait, HostTrait};
    let host = cpal::default_host();
    let list = if input { host.input_devices().map(|d| d.collect::<Vec<_>>()) } else { host.output_devices().map(|d| d.collect::<Vec<_>>()) };
    list.unwrap_or_default()
        .into_iter()
        .filter_map(|d| d.description().ok().map(|n| n.name().to_string()))
        .map(|name| Device { id: name.clone(), name })
        .collect()
}

#[cfg(all(windows, feature = "calls"))]
fn outputs() -> Vec<Device> {
    cpal_devices(false)
}

#[cfg(all(windows, feature = "calls"))]
fn inputs() -> Vec<Device> {
    cpal_devices(true)
}

#[cfg(all(windows, feature = "calls"))]
fn cameras() -> Vec<Device> {
    nokhwa::query(nokhwa::utils::ApiBackend::MediaFoundation)
        .unwrap_or_default()
        .into_iter()
        .map(|c| Device { id: c.human_name(), name: c.human_name() })
        .collect()
}

#[cfg(all(windows, not(feature = "calls")))]
fn outputs() -> Vec<Device> {
    vec![]
}

#[cfg(all(windows, not(feature = "calls")))]
fn inputs() -> Vec<Device> {
    vec![]
}

#[cfg(all(windows, not(feature = "calls")))]
fn cameras() -> Vec<Device> {
    vec![]
}

// ---- microphone capture

/// The sample rate of our own capture (48 kHz mono 16-bit).
#[cfg_attr(not(feature = "calls"), allow(dead_code))]
pub const RATE: u32 = 48_000;

/// A microphone (the chosen one, else the default) as 48 kHz mono 16-bit: whichever of PulseAudio's `parec` and
/// PipeWire's `pw-record` is there (both reach the same server on a PipeWire desktop), so no audio library is linked.
#[cfg(not(windows))]
pub struct Capture {
    child: std::process::Child,
    out: std::process::ChildStdout,
    buf: Vec<u8>,
}

#[cfg(not(windows))]
impl Capture {
    pub fn open(device: Option<&str>) -> Result<Self, String> {
        use std::process::{Command, Stdio};
        let parec_dev = device.map(|d| format!("--device={d}"));
        let mut parec = vec!["--raw", "--format=s16le", "--rate=48000", "--channels=1", "--latency-msec=20", "--client-name=SoWorker", "--stream-name=Microphone"];
        parec.extend(parec_dev.as_deref());
        let mut pw = vec!["--raw", "--format", "s16", "--rate", "48000", "--channels", "1", "--latency", "20ms"];
        if let Some(d) = device {
            pw.extend(["--target", d]);
        }
        pw.push("-");
        let mut errors = vec![];
        for (tool, args) in [("parec", parec), ("pw-record", pw)] {
            match Command::new(tool).args(args).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn() {
                Ok(mut child) => {
                    let out = child.stdout.take().expect("piped stdout");
                    eprintln!("mic: capturing with {tool} from {}", device.unwrap_or("the default microphone"));
                    return Ok(Capture { child, out, buf: vec![] });
                }
                Err(e) => errors.push(format!("{tool}: {e}")),
            }
        }
        Err(format!("cannot open the microphone ({})", errors.join(", ")))
    }

    /// Fills `frame` (blocks until that much audio came in).
    pub fn read(&mut self, frame: &mut [i16]) -> std::io::Result<()> {
        use std::io::Read;
        self.buf.resize(frame.len() * 2, 0);
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

/// A microphone (the chosen one, else the default) through WASAPI (cpal): whatever format the device offers, mixed
/// down to mono and brought to 48 kHz (linear interpolation; shared-mode devices almost always run at 48 kHz already).
#[cfg(all(windows, feature = "calls"))]
pub struct Capture {
    _stream: cpal::Stream,
    rx: std::sync::mpsc::Receiver<Vec<f32>>,
    pending: std::collections::VecDeque<f32>,
}

#[cfg(all(windows, feature = "calls"))]
impl Capture {
    pub fn open(device: Option<&str>) -> Result<Self, String> {
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
        let host = cpal::default_host();
        let named = device.and_then(|want| host.input_devices().ok()?.find(|d| d.description().is_ok_and(|n| n.name() == want)));
        let dev = named.or_else(|| host.default_input_device()).ok_or("no microphone")?;
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
        eprintln!("mic: capturing with WASAPI from {} ({ch} channels, {rate} Hz)", device.unwrap_or("the default microphone"));
        Ok(Capture { _stream: stream, rx, pending: Default::default() })
    }

    /// Fills `frame` (blocks until that much audio came in).
    pub fn read(&mut self, frame: &mut [i16]) -> std::io::Result<()> {
        while self.pending.len() < frame.len() {
            let chunk = self.rx.recv_timeout(std::time::Duration::from_secs(2)).map_err(|_| std::io::Error::new(std::io::ErrorKind::TimedOut, "no audio from the microphone"))?;
            self.pending.extend(chunk);
        }
        let n = frame.len();
        for (s, v) in frame.iter_mut().zip(self.pending.drain(..n)) {
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

// ---- level meter

/// How loud the microphone is, measured on a thread of its own while the profile panel shows it. Dropping it closes
/// the device.
pub struct Meter {
    /// 0..1 as f32 bits; negative = the microphone could not be opened.
    level: Arc<AtomicU32>,
    stop: Arc<AtomicBool>,
}

/// Peak level (0..1, full scale = 1) on a meter's scale: -60 dB and below is 0, full scale is 1.
pub fn meter_scale(peak: f32) -> f32 {
    if peak <= 0.0 {
        return 0.0;
    }
    ((20.0 * peak.log10() + 60.0) / 60.0).clamp(0.0, 1.0)
}

impl Meter {
    /// Starts measuring `device` (None = the default microphone). None where this build cannot capture audio itself.
    pub fn start(device: Option<String>) -> Option<Meter> {
        let (level, stop) = (Arc::new(AtomicU32::new(0)), Arc::new(AtomicBool::new(false)));
        #[cfg(any(not(windows), feature = "calls"))]
        {
            let (lv, st) = (level.clone(), stop.clone());
            std::thread::Builder::new()
                .name("meter".into())
                .spawn(move || {
                    let (level, stop) = (lv, st);
                    let mut cap = match Capture::open(device.as_deref()) {
                        Ok(c) => c,
                        Err(e) => {
                            eprintln!("meter: {e}");
                            return level.store((-1.0f32).to_bits(), Ordering::Relaxed);
                        }
                    };
                    // 20 ms blocks; the bar falls back slowly so short sounds stay visible
                    let (mut frame, mut shown) = ([0i16; 960], 0.0f32);
                    while !stop.load(Ordering::Relaxed) {
                        if cap.read(&mut frame).is_err() {
                            return level.store((-1.0f32).to_bits(), Ordering::Relaxed);
                        }
                        let peak = frame.iter().map(|s| (*s as i32).unsigned_abs()).max().unwrap_or(0) as f32 / 32768.0;
                        shown = meter_scale(peak).max(shown - 0.04);
                        level.store(shown.to_bits(), Ordering::Relaxed);
                    }
                })
                .ok()?;
            Some(Meter { level, stop })
        }
        #[cfg(all(windows, not(feature = "calls")))]
        {
            let _ = (device, level, stop);
            None
        }
    }

    /// 0..1, or None if the microphone could not be opened.
    pub fn level(&self) -> Option<f32> {
        let l = f32::from_bits(self.level.load(Ordering::Relaxed));
        (l >= 0.0).then_some(l)
    }
}

impl Drop for Meter {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pulse_lists_skip_monitors() {
        let json = br#"[
            {"name": "alsa_output.analog", "description": "Built-in Audio", "properties": {"device.class": "sound"}},
            {"name": "alsa_output.analog.monitor", "description": "Monitor of Built-in Audio", "properties": {"device.class": "monitor"}},
            {"name": "raop_sink.mac", "description": "(null)", "properties": {}}
        ]"#;
        let d = parse_pulse(json);
        assert_eq!(d, vec![Device { id: "alsa_output.analog".into(), name: "Built-in Audio".into() }, Device { id: "raop_sink.mac".into(), name: "raop_sink.mac".into() }]);
        assert!(parse_pulse(b"not json").is_empty());
    }

    #[test]
    fn the_meter_scale_is_in_decibels() {
        assert_eq!(meter_scale(0.0), 0.0);
        assert_eq!(meter_scale(1.0), 1.0);
        assert!((meter_scale(0.001) - 0.0).abs() < 1e-6, "-60 dB is the bottom");
        assert!((meter_scale(0.031_622_8) - 0.5).abs() < 1e-3, "-30 dB is half way");
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
