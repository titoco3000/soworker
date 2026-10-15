//! The profile panel, shared by both clients: who is signed in, log out, and the settings: which speaker (with a test
//! sound), microphone (with a level meter) and camera to use.
use crate::devices::{self, Choice, Devices, Kind, Meter};

/// What a click or key in the panel asks for.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(not(feature = "gui"), allow(dead_code))]
pub enum Act {
    Logout,
    /// Open or close a device list.
    Toggle(Kind),
    /// Use this device (`None` = the system default).
    Pick(Kind, Option<String>),
    TestSound,
    Close,
    /// A click on the panel itself: nothing to do.
    Panel,
}

pub struct Profile {
    pub devices: Devices,
    pub chosen: Choice,
    /// The device list that is open (window client).
    pub open: Option<Kind>,
    /// The microphone level, measured while the panel is open.
    pub meter: Option<Meter>,
    /// The selected line (terminal client): see `LINES`.
    #[cfg_attr(not(feature = "tui"), allow(dead_code))]
    pub sel: usize,
}

/// The terminal client's lines, top to bottom.
#[cfg_attr(not(feature = "tui"), allow(dead_code))]
pub const LINES: [Line; 4] = [Line::Logout, Line::Device(Kind::Output), Line::Device(Kind::Input), Line::Device(Kind::Camera)];

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(not(feature = "tui"), allow(dead_code))]
pub enum Line {
    Logout,
    Device(Kind),
}

pub const DEFAULT: &str = "System default";

impl Profile {
    pub fn new() -> Self {
        let chosen = devices::chosen();
        Profile { devices: devices::list(), meter: Meter::start(chosen.input.clone()), chosen, open: None, sel: 1 }
    }

    /// The choices of a list: the system default, then every device that is there.
    pub fn options(&self, k: Kind) -> Vec<(Option<String>, String)> {
        std::iter::once((None, DEFAULT.to_string())).chain(self.devices.of(k).iter().map(|d| (Some(d.id.clone()), d.name.clone()))).collect()
    }

    /// What a list shows as chosen.
    pub fn label(&self, k: Kind) -> String {
        match self.chosen.get(k) {
            None => DEFAULT.into(),
            Some(id) => self.devices.of(k).iter().find(|d| d.id == id).map_or_else(|| format!("{id} (not connected)"), |d| d.name.clone()),
        }
    }

    /// The choice next to the current one (terminal client: left / right on a line).
    #[cfg_attr(not(feature = "tui"), allow(dead_code))]
    pub fn cycle(&self, k: Kind, forward: bool) -> Option<String> {
        let opts = self.options(k);
        let cur = opts.iter().position(|(id, _)| id.as_deref() == self.chosen.get(k)).unwrap_or(0);
        let n = opts.len();
        opts[if forward { (cur + 1) % n } else { (cur + n - 1) % n }].0.clone()
    }

    /// 0..1, or None when the microphone cannot be read (or this build cannot measure it).
    pub fn level(&self) -> Option<f32> {
        self.meter.as_ref().and_then(|m| m.level())
    }
}
