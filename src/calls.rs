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
