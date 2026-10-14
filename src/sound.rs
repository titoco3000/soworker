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

/// Fire and forget; a missing audio device is not an error worth showing.
#[cfg(windows)]
pub fn play(s: Sound) {
    use windows_sys::Win32::Media::Audio::{PlaySoundW, SND_MEMORY, SND_NODEFAULT};
    std::thread::spawn(move || {
        let wav = scaled(s.bytes(), s.volume());
        // SAFETY: with SND_MEMORY the first argument points at a whole WAV file in memory; playing synchronously keeps
        // `wav` alive until the sound is done.
        unsafe { PlaySoundW(wav.as_ptr().cast(), std::ptr::null_mut(), SND_MEMORY | SND_NODEFAULT) };
    });
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

/// Fire and forget; a missing player or audio device is not an error worth showing.
#[cfg(not(windows))]
pub fn play(s: Sound) {
    let Some(path) = file(s) else { return };
    std::thread::spawn(move || {
        let vol = format!("{}", s.volume());
        let pa_vol = format!("{}", (s.volume() * 65536.0) as u32);
        let attempts: [(&str, Vec<&str>); 3] = [
            ("pw-play", vec!["--volume", &vol]),
            ("paplay", vec!["--volume", &pa_vol]),
            ("aplay", vec!["-q"]),
        ];
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
