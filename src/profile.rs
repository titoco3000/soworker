//! The profile panel, shared by both clients: who is signed in, log out, and the settings: which speaker (with a test
//! sound), microphone (with a level meter) and camera to use, on Windows the screen a share captures, and how hard
//! noise cancellation works (with a test that records you and plays it back).
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
    /// Noise cancellation strength, 0..=1 (a click or a drag on the slider).
    Strength(f32),
    /// Record a few seconds of the microphone and play them back.
    TestMic,
    /// Low-data mode on or off.
    LowData,
    /// Pick a new profile photo.
    ChangePhoto,
    /// Move to the terminal client.
    Terminal,
    /// Show the QR code that signs a phone in to this account (desktop window client).
    PairPhone,
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
    /// The pointer is over our picture (window client): it shows a pencil.
    #[cfg_attr(not(feature = "gui"), allow(dead_code))]
    pub photo_hover: bool,
    /// How far the panel is scrolled down, in pixels (window client, phone layout: it can be taller than the screen).
    #[cfg_attr(not(feature = "gui"), allow(dead_code))]
    pub scroll: f32,
    /// The noise cancellation slider is being dragged (window client).
    #[cfg_attr(not(feature = "gui"), allow(dead_code))]
    pub sliding: bool,
    /// The running (or last) microphone test: a `calls::MicTest` code.
    pub mic_test: Option<std::sync::Arc<std::sync::atomic::AtomicU8>>,
}

/// The terminal client's lines, top to bottom.
#[cfg_attr(not(feature = "tui"), allow(dead_code))]
#[cfg(windows)]
pub const LINES: [Line; 6] = [Line::Logout, Line::Device(Kind::Output), Line::Device(Kind::Input), Line::Device(Kind::Camera), Line::Device(Kind::Screen), Line::Noise];
#[cfg(not(windows))]
pub const LINES: [Line; 5] = [Line::Logout, Line::Device(Kind::Output), Line::Device(Kind::Input), Line::Device(Kind::Camera), Line::Noise];

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(not(feature = "tui"), allow(dead_code))]
pub enum Line {
    Logout,
    Device(Kind),
    /// Noise cancellation strength, with the microphone test.
    Noise,
}

pub const DEFAULT: &str = "System default";

/// What "no choice" is called in a list: a share with no screen chosen takes the first one.
fn default_label(k: Kind) -> &'static str {
    match k {
        Kind::Screen => "First screen",
        // a phone: Bluetooth, else a wired headset, else the speaker; the front camera
        Kind::Output if cfg!(target_os = "android") => "Automatic",
        Kind::Camera if cfg!(target_os = "android") => "Front camera",
        _ => DEFAULT,
    }
}

impl Profile {
    pub fn new() -> Self {
        let chosen = devices::chosen();
        Profile { meter: Meter::start(chosen.input.clone()), chosen, ..Profile::with(devices::list()) }
    }

    /// A panel over these devices, measuring nothing (and for tests).
    pub fn with(devices: Devices) -> Self {
        Profile { devices, chosen: Choice::default(), open: None, meter: None, sel: 1, photo_hover: false, scroll: 0.0, sliding: false, mic_test: None }
    }

    /// Where the microphone test is (None before the first one).
    pub fn mic_test(&self) -> Option<crate::calls::MicTest> {
        self.mic_test.as_ref().map(|c| crate::calls::MicTest::from_code(c.load(std::sync::atomic::Ordering::Relaxed)))
    }

    /// A test is recording or playing: another one waits.
    pub fn testing(&self) -> bool {
        use crate::calls::MicTest;
        matches!(self.mic_test(), Some(MicTest::Preparing | MicTest::Recording(_) | MicTest::Playing))
    }

    /// The choices of a list: the system default, then every device that is there.
    pub fn options(&self, k: Kind) -> Vec<(Option<String>, String)> {
        std::iter::once((None, default_label(k).to_string())).chain(self.devices.of(k).iter().map(|d| (Some(d.id.clone()), d.name.clone()))).collect()
    }

    /// What a list shows as chosen.
    pub fn label(&self, k: Kind) -> String {
        match self.chosen.get(k) {
            None => default_label(k).into(),
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
