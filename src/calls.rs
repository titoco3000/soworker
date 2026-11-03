//! Types shared between the UI and the media thread. No dependency on the LiveKit SDK, so the
//! app (and `--no-default-features` builds) compile without it.

#[derive(Debug, Clone)]
pub enum MediaCmd {
    /// `portrait` is our compact character string, for the participant metadata other clients read.
    Join { url: String, token: String, room_id: String, portrait: Option<String> },
    Leave,
    /// Raise our hand (`Some(unix ms)` = when) or lower it (`None`): the `raisedHandAt` participant metadata.
    Hand(Option<i64>),
    /// Lower another participant's hand (by user id): a `raise-hand` data message they act on.
    LowerHand(String),
    Mic(bool),
    Camera(bool),
    Share(bool),
    /// Ask another participant (by user id) to mute their microphone.
    AskToMute(String),
    /// Noise cancellation (DeepFilterNet) on the microphone, for this call and the next ones.
    NoiseCancel(bool),
    /// The chosen speaker, microphone or camera changed (see `devices::chosen`): switch the call over to them.
    Devices,
    /// Start recording the call (`Some(title)`, which names the files) or stop (`None`). See `record.rs`.
    Record(Option<String>),
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
    /// Everyone else in the room whose hand is raised (identities), sent whenever it may have changed.
    Hands(Vec<String>),
    /// Someone raised (`true`) or lowered our hand for us with a `raise-hand` message.
    HandSetBy { from: String, raised: bool },
    /// A recording started or stopped (also when the call ends).
    Recording(bool),
    /// Something to tell the user that is not an error (a recording saved).
    Info(String),
    /// How the transcript of a recording is coming along.
    Transcript(crate::transcribe::Status),
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

/// One value of settings.json (`Null` when the file or the key is missing).
pub fn setting(key: &str) -> serde_json::Value {
    std::fs::read_to_string(settings_path())
        .ok()
        .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
        .map_or(serde_json::Value::Null, |v| v[key].clone())
}

/// Sets one value in settings.json (`Null` removes it), keeping everything else in the file.
pub fn save_setting(key: &str, value: serde_json::Value) {
    let path = settings_path();
    let mut v = std::fs::read_to_string(&path).ok().and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok()).filter(|v| v.is_object()).unwrap_or_else(|| serde_json::json!({}));
    match (value, v.as_object_mut()) {
        (serde_json::Value::Null, Some(o)) => drop(o.remove(key)),
        (value, _) => v[key] = value,
    }
    let saved = path.parent().map_or(Ok(()), std::fs::create_dir_all).and_then(|_| std::fs::write(&path, serde_json::to_string_pretty(&v).unwrap_or_default() + "\n"));
    if let Err(e) = saved {
        eprintln!("settings: cannot save {}: {e}", path.display());
    }
}

/// The `noiseCancellation` setting of settings.json (on unless set to false).
pub fn noise_cancel_setting() -> bool {
    setting("noiseCancellation").as_bool().unwrap_or(true)
}

/// Remembers the noise cancellation choice in settings.json.
pub fn save_noise_cancel(on: bool) {
    save_setting("noiseCancellation", on.into());
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

/// LiveKit data-message topic SoWork uses to raise or lower someone else's hand (`signaler.send("raise-hand", ...)`).
pub const RAISE_HAND: &str = "raise-hand";

/// The message body: msgpack `{raised, userId}`, encoded like `ask-to-mute` (SoWork's signaler sends msgpack maps).
pub fn raise_hand_payload(target: &str, raised: bool) -> Vec<u8> {
    let mut out = Vec::new();
    let v = rmpv::Value::Map(vec![("raised".into(), raised.into()), ("userId".into(), target.into())]);
    let _ = rmpv::encode::write_value(&mut out, &v);
    out
}

/// Who a `raise-hand` message is for, and whether it raises or lowers their hand.
pub fn raise_hand_signal(payload: &[u8]) -> Option<(String, bool)> {
    let v = rmpv::decode::read_value(&mut &payload[..]).ok()?;
    let field = |k: &str| v.as_map()?.iter().find(|(kk, _)| kk.as_str() == Some(k)).map(|(_, v)| v.clone());
    Some((field("userId")?.as_str()?.to_string(), field("raised")?.as_bool()?))
}

/// Our LiveKit participant metadata, as SoWork's client sends it (docs/hand-desk-photo.md). It is always sent whole,
/// so the hand and the screen share must both be in it.
pub fn participant_meta(raised_at: Option<i64>, portrait: Option<&str>, share: bool) -> String {
    let mut v = serde_json::json!({
        "raisedHandAt": raised_at,
        "audioOnly": false,
        "allowsScreenDrawing": false,
        "desktop": false,
        "screenshare": if share { serde_json::json!({"type": "screen"}) } else { serde_json::Value::Null },
    });
    if let Some(p) = portrait {
        v["portrait"] = serde_json::json!({"url": format!("https://app.sowork.com/portrait/v1/meeting!idle/{p}"), "kind": "character"});
    }
    v.to_string()
}

/// Whether a participant's metadata says their hand is up.
pub fn hand_raised(meta: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(meta).is_ok_and(|v| v["raisedHandAt"].as_i64().is_some())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raise_hand_messages_round_trip() {
        assert_eq!(raise_hand_signal(&raise_hand_payload("ana", false)), Some(("ana".into(), false)));
        assert_eq!(raise_hand_signal(&raise_hand_payload("bo", true)), Some(("bo".into(), true)));
        assert_eq!(raise_hand_signal(b"\x90"), None);
    }

    #[test]
    fn the_metadata_carries_the_hand_and_the_share() {
        let up = participant_meta(Some(1793734498692), Some("characterA!body:~v1+typeB"), false);
        assert!(hand_raised(&up));
        let v: serde_json::Value = serde_json::from_str(&up).unwrap();
        assert_eq!(v["portrait"]["url"], "https://app.sowork.com/portrait/v1/meeting!idle/characterA!body:~v1+typeB");
        assert!(v["screenshare"].is_null());
        let down = participant_meta(None, None, true);
        assert!(!hand_raised(&down));
        assert_eq!(serde_json::from_str::<serde_json::Value>(&down).unwrap()["screenshare"]["type"], "screen");
        // as recorded from the official client
        assert!(hand_raised(r#"{"raisedHandAt":1793734498692,"audioOnly":false,"screenshare":null}"#));
        assert!(!hand_raised(r#"{"raisedHandAt":null}"#) && !hand_raised("") && !hand_raised("not json"));
    }


    #[test]
    fn payload_round_trips() {
        let p = ask_to_mute_payload("RMUYRzBXvUOCqtJdxSAiMdiEZM82");
        assert_eq!(ask_to_mute_target(&p).as_deref(), Some("RMUYRzBXvUOCqtJdxSAiMdiEZM82"));
        assert_eq!(ask_to_mute_target(b"\x90"), None);
    }
}
