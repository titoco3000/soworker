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
}
