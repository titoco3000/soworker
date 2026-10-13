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

    /// The recorded `ask-to-mute` data message (LiveKit DataPacket) from the recording: our encoder must produce the same
    /// payload bytes the official client did. Skipped when that recording is not on this machine.
    #[test]
    fn ask_to_mute_payload_matches_the_official_clients_bytes() {
        use base64::Engine;
        let run = std::env::var("SOWORKER_RUN2").unwrap_or_else(|_| "../recorder/logs/2026-10-06T17-30-38-857Z".into());
        let Ok(text) = std::fs::read_to_string(format!("{run}/events.jsonl")) else { return };
        let mut seen = 0;
        for line in text.lines() {
            let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else { continue };
            if v["k"] != "dc.in" {
                continue;
            }
            let Some(b) = v["b64"].as_str().and_then(|s| base64::engine::general_purpose::STANDARD.decode(s).ok()) else { continue };
            if !b.windows(ASK_TO_MUTE.len()).any(|w| w == ASK_TO_MUTE.as_bytes()) {
                continue;
            }
            // the payload is the msgpack map that starts with 0x81 0xac "targetUserId" inside the packet
            let start = b.windows(14).position(|w| w[0] == 0x81 && w[1] == 0xac && &w[2..14] == b"targetUserId").expect("payload in packet");
            let target = ask_to_mute_target(&b[start..]).expect("decodes");
            assert_eq!(ask_to_mute_payload(&target), b[start..start + 14 + 1 + target.len()].to_vec(), "same bytes as the official client");
            seen += 1;
        }
        assert!(seen > 0, "the recording has ask-to-mute messages");
    }

    #[test]
    fn payload_round_trips() {
        let p = ask_to_mute_payload("RMUYRzBXvUOCqtJdxSAiMdiEZM82");
        assert_eq!(ask_to_mute_target(&p).as_deref(), Some("RMUYRzBXvUOCqtJdxSAiMdiEZM82"));
        assert_eq!(ask_to_mute_target(b"\x90"), None);
    }
}
