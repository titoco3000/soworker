#![cfg_attr(not(feature = "gui"), allow(dead_code))]
//! The meeting dock's model: who is in the call, which tiles exist, what the controls are. No drawing.
use crate::app::State;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Slot {
    TopLeft,
    TopCenter,
    TopRight,
    MidLeft,
    MidRight,
    BotLeft,
    BotCenter,
    BotRight,
}

#[derive(Clone, Copy, Debug)]
pub struct Drag {
    pub grab: (f32, f32),
    pub pos: (f32, f32),
}

#[derive(Default)]
pub struct Hover {
    pub tile: Option<String>,
    /// When the pointer last moved; a fullscreen tile shows its buttons only for a moment after that.
    pub moved: Option<std::time::Instant>,
}

impl Hover {
    pub const SHOW: std::time::Duration = std::time::Duration::from_millis(500);

    pub fn recent(&self) -> bool {
        self.moved.is_some_and(|t| t.elapsed() < Self::SHOW)
    }
}

pub struct Dock {
    pub full: bool,
    /// The pinned tile fills the whole meeting area, with no header or controls.
    pub tile_full: bool,
    pub slot: Slot,
    pub drag: Option<Drag>,
    pub chat_open: bool,
    pub pinned: Option<String>,
    /// The first screen-share tile that was auto-pinned, so a new share pins once but a manual unpin sticks.
    pub last_auto: Option<String>,
    pub hover: Hover,
    /// Scroll offset of the pinned layout's sidebar column (px).
    pub scroll: f32,
}

impl Default for Dock {
    fn default() -> Self {
        Dock { full: false, tile_full: false, slot: Slot::BotRight, drag: None, chat_open: false, pinned: None, last_auto: None, hover: Hover::default(), scroll: 0.0 }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ctl {
    Leave,
    NoiseCancel,
    Cam,
    Mic,
    Share,
    Chat,
    FullToggle,
    Send,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileBtn {
    Mute,
    CameraOff,
    Pin,
    Fullscreen,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Hit {
    Header,
    Ctl(Ctl),
    Tile(String),
    TileBtn(String, TileBtn),
    Inside,
}

pub struct TileSpec {
    pub id: String,
    pub video_key: Option<String>,
    pub name: String,
    pub initial: String,
    pub user: Option<String>,
    pub is_self: bool,
    pub content: bool,
    pub speaking: bool,
    pub muted: bool,
}

pub fn visible(st: &State) -> bool {
    st.call.joined.is_some() || st.call.want.is_some()
}

pub fn status_label(st: &State) -> &'static str {
    let others = others(st).len();
    match (st.call.joined.is_some(), others) {
        (false, _) => "Connecting…",
        (true, 0) => "Waiting for others",
        _ => "Connected",
    }
}

fn others(st: &State) -> Vec<String> {
    if let Some(p) = &st.call.peers {
        return p.iter().map(|(i, _)| i.clone()).filter(|i| *i != st.me_id).collect();
    }
    st.cluster.as_ref().map(|c| c.users.iter().filter(|(i, _)| *i != st.me_id).map(|(i, _)| i.clone()).collect()).unwrap_or_default()
}

pub fn tiles(st: &State) -> Vec<TileSpec> {
    let me_name = st.me().map(|m| m.name.clone()).unwrap_or_default();
    let mut v = vec![TileSpec {
        id: "self".into(),
        video_key: st.call.cam.then(|| "me".to_string()),
        name: "You".into(),
        initial: me_name,
        user: Some(st.me_id.clone()),
        is_self: true,
        content: false,
        speaking: st.call.speakers.contains(&st.me_id),
        muted: !st.call.mic,
    }];
    for id in others(st) {
        let name = st.users.get(&id).map(|u| u.name.clone()).or_else(|| st.call.peers.as_ref().and_then(|p| p.iter().find(|(i, n)| *i == id && !n.is_empty()).map(|(_, n)| n.clone()))).or_else(|| st.cluster.as_ref().and_then(|c| c.users.iter().find(|(i, _)| *i == id).map(|(_, n)| n.clone()))).unwrap_or_else(|| id.clone());
        v.push(TileSpec {
            video_key: st.call.tiles.iter().any(|t| t.key == id).then(|| id.clone()),
            initial: name.clone(),
            name,
            user: Some(id.clone()),
            id: id.clone(),
            is_self: false,
            content: false,
            speaking: st.call.speakers.contains(&id),
            muted: st.call.peer_muted.contains(&id),
        });
    }
    for t in st.call.tiles.iter().filter(|t| t.key.ends_with(":screen")) {
        v.push(TileSpec {
            id: format!("content:{}", t.key),
            video_key: Some(t.key.clone()),
            name: if t.key.starts_with("me:") { "Your screen".into() } else { "Screen share".into() },
            initial: String::new(),
            user: None,
            is_self: false,
            content: true,
            speaking: false,
            muted: false,
        });
    }
    v
}
