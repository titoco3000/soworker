//! Types shared between the UI and the media thread. No dependency on the LiveKit SDK, so the
//! app (and `--no-default-features` builds) compile without it.

#[derive(Debug, Clone)]
pub enum MediaCmd {
    Join { url: String, token: String, room_id: String },
    Leave,
    Mic(bool),
    Camera(bool),
    Share(bool),
    /// Ask another participant (by user id) to mute their microphone.
    AskToMute(String),
    /// Noise cancellation (DeepFilterNet) on the microphone, for this call and the next ones.
    NoiseCancel(bool),
}

#[derive(Debug)]
pub enum MediaEvent {
    Joined(String),
    Left,
    Error(String),
    /// Identities (user ids) currently speaking.
    Speakers(Vec<String>),
    /// A video frame to show: RGBA, already scaled to its tile size.
    Video { key: String, name: String, w: u32, h: u32, rgba: Vec<u8> },
    VideoGone(String),
    /// Everyone else currently in the room (identity, name): sent on every join / leave, so tiles follow the room itself.
    Peers(Vec<(String, String)>),
    MicState(bool),
    CameraState(bool),
    ShareState(bool),
    /// A remote participant muted or unmuted their microphone.
    PeerMuted(String, bool),
    /// Someone asked us to mute (the request targeted our user id). The UI decides whether our mic is on.
    AskedToMute { from: String },
}

/// Which video tiles the frontend wants to see; frames of any other tile are not converted at all (they are dropped
/// before the YUV to RGBA work). `None` = all of them (the window client), `Some(empty)` = none (no video shown).
static WANTED: std::sync::RwLock<Option<std::collections::HashSet<String>>> = std::sync::RwLock::new(None);

#[cfg(feature = "tui")]
pub fn want_video(keys: Option<std::collections::HashSet<String>>) {
    if let Ok(mut w) = WANTED.write() {
        *w = keys;
    }
}

pub fn wants_video(key: &str) -> bool {
    WANTED.read().map_or(true, |w| w.as_ref().map_or(true, |s| s.contains(key)))
}

static CAM_W: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(480);
static SCREEN_W: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(960);

/// Widest picture (pixels) handed to the frontend for a camera / a screen share. Bigger = sharper and more work.
#[cfg(feature = "tui")]
pub fn set_video_width(cam: u32, screen: u32) {
    CAM_W.store(cam, std::sync::atomic::Ordering::Relaxed);
    SCREEN_W.store(screen, std::sync::atomic::Ordering::Relaxed);
}

pub fn video_width(screen: bool) -> u32 {
    (if screen { &SCREEN_W } else { &CAM_W }).load(std::sync::atomic::Ordering::Relaxed)
}

fn settings_path() -> std::path::PathBuf {
    std::path::Path::new(&crate::session::default_path()).with_file_name("settings.json")
}

/// The `noiseCancellation` setting of settings.json (on unless set to false).
pub fn noise_cancel_setting() -> bool {
    std::fs::read_to_string(settings_path())
        .ok()
        .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
        .and_then(|v| v["noiseCancellation"].as_bool())
        .unwrap_or(true)
}

/// Remembers the noise cancellation choice in settings.json, keeping everything else in the file.
pub fn save_noise_cancel(on: bool) {
    let path = settings_path();
    let mut v = std::fs::read_to_string(&path).ok().and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok()).filter(|v| v.is_object()).unwrap_or_else(|| serde_json::json!({}));
    v["noiseCancellation"] = on.into();
    let saved = path.parent().map_or(Ok(()), std::fs::create_dir_all).and_then(|_| std::fs::write(&path, serde_json::to_string_pretty(&v).unwrap_or_default() + "\n"));
    if let Err(e) = saved {
        eprintln!("settings: cannot save {}: {e}", path.display());
    }
}

/// Sender for media commands. Without the `calls` feature it is a stub, so the UI code compiles either way.
#[cfg(feature = "calls")]
pub type MediaTx = tokio::sync::mpsc::UnboundedSender<MediaCmd>;
#[cfg(not(feature = "calls"))]
pub struct MediaTx;
#[cfg(not(feature = "calls"))]
impl MediaTx {
    pub fn send(&self, _: MediaCmd) -> Result<(), ()> {
        Ok(())
    }
}

/// LiveKit data-message topic SoWork uses to ask someone to mute (found in its client: `signaler.send("ask-to-mute", ...)`).
pub const ASK_TO_MUTE: &str = "ask-to-mute";

/// The message body: msgpack `{targetUserId: <user id>}`, byte-for-byte what the official client sends.
pub fn ask_to_mute_payload(target: &str) -> Vec<u8> {
    let mut out = Vec::new();
    let v = rmpv::Value::Map(vec![("targetUserId".into(), target.into())]);
    let _ = rmpv::encode::write_value(&mut out, &v);
    out
}

/// The user id an `ask-to-mute` message is addressed to.
pub fn ask_to_mute_target(payload: &[u8]) -> Option<String> {
    let v = rmpv::decode::read_value(&mut &payload[..]).ok()?;
    v.as_map()?.iter().find(|(k, _)| k.as_str() == Some("targetUserId"))?.1.as_str().map(String::from)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn payload_round_trips() {
        let p = ask_to_mute_payload("RMUYRzBXvUOCqtJdxSAiMdiEZM82");
        assert_eq!(ask_to_mute_target(&p).as_deref(), Some("RMUYRzBXvUOCqtJdxSAiMdiEZM82"));
        assert_eq!(ask_to_mute_target(b"\x90"), None);
    }
}
