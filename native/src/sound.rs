//! Short UI sounds, played through whatever the desktop offers (`pw-play`, `paplay`, `aplay`): no audio library is
//! linked, so the binary stays small. The wave sounds are SoWork's own (`sendWave-*.mp3`, `receiveWave-*.mp3` from
//! app.sowork.com/static/media, played at volume 0.2 by its client; we play the receive sound louder), converted to 32 kHz mono WAV.
use std::process::{Command, Stdio};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Sound {
    SendWave,
    ReceiveWave,
}

const SEND_WAVE: &[u8] = include_bytes!("../assets/sounds/sendWave.wav");
const RECEIVE_WAVE: &[u8] = include_bytes!("../assets/sounds/receiveWave.wav");

impl Sound {
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

/// The sound as a file on disk (players want a path), written once per run into the private runtime directory.
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
}
