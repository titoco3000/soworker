//! Short UI sounds, played through whatever the desktop offers (`pw-play`, `paplay`, `aplay`; `PlaySound` on Windows):
//! no audio library is linked, so the binary stays small. The wave sounds are SoWork's own (`sendWave-*.mp3`, `receiveWave-*.mp3` from
//! app.sowork.com/static/media, played at volume 0.2 by its client; we play the receive sound louder), converted to 32 kHz mono WAV.
#[cfg(not(windows))]
use std::process::{Command, Stdio};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Sound {
    SendWave,
    ReceiveWave,
}

const SEND_WAVE: &[u8] = include_bytes!("../assets/sounds/sendWave.wav");
const RECEIVE_WAVE: &[u8] = include_bytes!("../assets/sounds/receiveWave.wav");

impl Sound {
    #[cfg(not(windows))]
    fn name(self) -> &'static str {
        match self {
            Sound::SendWave => "sendWave",
            Sound::ReceiveWave => "receiveWave",
        }
    }

    /// Player volume. SoWork's client plays both at 0.2; receiving a wave is 10 times that (2.0), which still keeps the
    /// clip's peak (-7.9 dB) below full scale.
    fn volume(self) -> f32 {
        match self {
            Sound::SendWave => 0.2,
            Sound::ReceiveWave => 3.0,
        }
    }

    fn bytes(self) -> &'static [u8] {
        match self {
            Sound::SendWave => SEND_WAVE,
            Sound::ReceiveWave => RECEIVE_WAVE,
        }
    }
}

/// The clip with its samples scaled by `vol` (16-bit PCM, clipped), for players that take no volume.
#[cfg_attr(not(any(windows, test)), allow(dead_code))]
fn scaled(wav: &[u8], vol: f32) -> Vec<u8> {
    let mut out = wav.to_vec();
    // RIFF header (12 bytes), then chunks: 4-byte id, 4-byte little-endian size, body (padded to even)
    let mut at = 12;
    while at + 8 <= out.len() {
        let len = u32::from_le_bytes([out[at + 4], out[at + 5], out[at + 6], out[at + 7]]) as usize;
        if &out[at..at + 4] == b"data" {
            let end = (at + 8 + len).min(out.len());
            for s in out[at + 8..end].chunks_exact_mut(2) {
                let v = (i16::from_le_bytes([s[0], s[1]]) as f32 * vol).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
                s.copy_from_slice(&v.to_le_bytes());
            }
            break;
        }
        at += 8 + len + (len & 1);
    }
    out
}

/// Fire and forget; a missing audio device is not an error worth showing. `PlaySound` only knows the default output,
/// so a speaker chosen in the profile panel is played through WASAPI (cpal) instead.
#[cfg(windows)]
pub fn play(s: Sound) {
    use windows_sys::Win32::Media::Audio::{PlaySoundW, SND_MEMORY, SND_NODEFAULT};
    let device = crate::devices::chosen().output;
    std::thread::spawn(move || {
        #[cfg(feature = "calls")]
        if let Some(d) = &device {
            match play_on(s.bytes(), s.volume(), d) {
                Ok(()) => return,
                Err(e) => eprintln!("sound: cannot play on {d}: {e}"),
            }
        }
        let _ = &device;
        let wav = scaled(s.bytes(), s.volume());
        // SAFETY: with SND_MEMORY the first argument points at a whole WAV file in memory; playing synchronously keeps
        // `wav` alive until the sound is done.
        unsafe { PlaySoundW(wav.as_ptr().cast(), std::ptr::null_mut(), SND_MEMORY | SND_NODEFAULT) };
    });
}

/// The samples of a 16-bit mono WAV file (-1..1) and its sample rate.
#[cfg_attr(not(all(windows, feature = "calls")), allow(dead_code))]
fn pcm(wav: &[u8]) -> Option<(u32, Vec<f32>)> {
    let (mut rate, mut at) = (None, 12);
    while at + 8 <= wav.len() {
        let len = u32::from_le_bytes(wav[at + 4..at + 8].try_into().ok()?) as usize;
        let body = wav.get(at + 8..(at + 8 + len).min(wav.len()))?;
        match &wav[at..at + 4] {
            b"fmt " if body.len() >= 16 => {
                let (channels, bits) = (u16::from_le_bytes([body[2], body[3]]), u16::from_le_bytes([body[14], body[15]]));
                if channels != 1 || bits != 16 {
                    return None;
                }
                rate = Some(u32::from_le_bytes(body[4..8].try_into().ok()?));
            }
            b"data" => return Some((rate?, body.chunks_exact(2).map(|b| i16::from_le_bytes([b[0], b[1]]) as f32 / 32768.0).collect())),
            _ => {}
        }
        at += 8 + len + (len & 1);
    }
    None
}

/// Plays a WAV clip on the named output device and waits until it is done.
#[cfg(all(windows, feature = "calls"))]
fn play_on(wav: &[u8], vol: f32, want: &str) -> Result<(), String> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use std::sync::atomic::{AtomicUsize, Ordering};
    let (rate, clip) = pcm(wav).ok_or("not a 16-bit mono WAV")?;
    let dev = cpal::default_host()
        .output_devices()
        .map_err(|e| e.to_string())?
        .find(|d| d.description().is_ok_and(|n| n.name() == want))
        .ok_or("not connected")?;
    let cfg = dev.default_output_config().map_err(|e| e.to_string())?;
    let (ch, out_rate) = (cfg.channels().max(1) as usize, cfg.sample_rate());
    // to the device's rate (linear interpolation), with the volume applied
    let n = (clip.len() as u64 * out_rate as u64 / rate.max(1) as u64) as usize;
    let mono: Vec<f32> = (0..n)
        .map(|i| {
            let p = i as f64 * rate as f64 / out_rate as f64;
            let (j, f) = (p as usize, (p - (p as usize) as f64) as f32);
            let a = clip.get(j).copied().unwrap_or(0.0);
            let b = clip.get(j + 1).copied().unwrap_or(a);
            ((a + (b - a) * f) * vol).clamp(-1.0, 1.0)
        })
        .collect();
    let mono = std::sync::Arc::new(mono);
    let pos = std::sync::Arc::new(AtomicUsize::new(0));
    let err = |e| eprintln!("sound: stream error: {e}");
    macro_rules! build {
        ($t:ty, $from:expr) => {{
            let (mono, pos) = (mono.clone(), pos.clone());
            dev.build_output_stream(
                cfg.config(),
                move |out: &mut [$t], _: &_| {
                    for frame in out.chunks_mut(ch) {
                        let i = pos.fetch_add(1, Ordering::Relaxed);
                        let v = mono.get(i).copied().unwrap_or(0.0);
                        frame.iter_mut().for_each(|s| *s = $from(v));
                    }
                },
                err,
                None,
            )
        }};
    }
    let stream = match cfg.sample_format() {
        cpal::SampleFormat::F32 => build!(f32, |v: f32| v),
        cpal::SampleFormat::I16 => build!(i16, |v: f32| (v * 32767.0) as i16),
        cpal::SampleFormat::I32 => build!(i32, |v: f32| (v as f64 * 2147483647.0) as i32),
        f => return Err(format!("unsupported output format {f}")),
    }
    .map_err(|e| e.to_string())?;
    stream.play().map_err(|e| e.to_string())?;
    std::thread::sleep(std::time::Duration::from_millis((n as u64 * 1000 / out_rate.max(1) as u64) + 200));
    Ok(())
}

/// The sound as a file on disk (players want a path), written once per run into the private runtime directory.
#[cfg(not(windows))]
fn file(s: Sound) -> Option<std::path::PathBuf> {
    let dir = std::env::var_os("XDG_RUNTIME_DIR").map_or_else(std::env::temp_dir, std::path::PathBuf::from).join("soworker");
    std::fs::create_dir_all(&dir).ok()?;
    let path = dir.join(format!("{}.wav", s.name()));
    if std::fs::metadata(&path).map_or(true, |m| m.len() != s.bytes().len() as u64) {
        std::fs::write(&path, s.bytes()).ok()?;
    }
    Some(path)
}

/// Fire and forget; a missing player or audio device is not an error worth showing. Plays on the speaker chosen in
/// the profile panel (`aplay`, the last resort, only knows the default).
#[cfg(not(windows))]
pub fn play(s: Sound) {
    let Some(path) = file(s) else { return };
    let device = crate::devices::chosen().output;
    std::thread::spawn(move || {
        let vol = format!("{}", s.volume());
        let pa_vol = format!("{}", (s.volume() * 65536.0) as u32);
        let mut pw = vec!["--volume", &vol];
        let mut pa = vec!["--volume", &pa_vol];
        let pa_dev = device.as_ref().map(|d| format!("--device={d}"));
        if let (Some(d), Some(pd)) = (&device, &pa_dev) {
            pw.extend(["--target", d]);
            pa.push(pd);
        }
        let attempts: [(&str, Vec<&str>); 3] = [("pw-play", pw), ("paplay", pa), ("aplay", vec!["-q"])];
        for (cmd, args) in attempts {
            let started = Command::new(cmd).args(args).arg(&path).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).status();
            if started.is_ok_and(|st| st.success()) {
                return;
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_embedded_sounds_are_wav_files() {
        for s in [Sound::SendWave, Sound::ReceiveWave] {
            assert!(s.bytes().starts_with(b"RIFF") && s.bytes().len() > 10_000, "{s:?}");
        }
    }

    #[test]
    fn the_clips_decode_to_samples() {
        let (rate, samples) = pcm(Sound::ReceiveWave.bytes()).unwrap();
        assert_eq!(rate, 32_000);
        assert!(samples.len() > 5_000 && samples.iter().any(|s| s.abs() > 0.1));
    }

    #[test]
    fn scaling_touches_only_the_samples() {
        let wav = Sound::SendWave.bytes();
        let quiet = scaled(wav, 0.5);
        assert_eq!(quiet.len(), wav.len());
        let data = wav.windows(4).position(|w| w == b"data").unwrap() + 8;
        assert_eq!(quiet[..data], wav[..data], "headers unchanged");
        assert_ne!(quiet[data..], wav[data..]);
        assert_eq!(scaled(wav, 1.0), wav);
    }
}
