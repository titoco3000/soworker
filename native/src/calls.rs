//! Types shared between the UI and the media thread. No dependency on the LiveKit SDK, so the
//! app (and `--no-default-features` builds) compile without it.

#[derive(Debug, Clone)]
pub enum MediaCmd {
    Join { url: String, token: String, room_id: String },
    Leave,
    Mic(bool),
    Camera(bool),
    Share(bool),
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
    MicState(bool),
    CameraState(bool),
    ShareState(bool),
    /// A remote participant muted or unmuted their microphone.
    PeerMuted(String, bool),
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
