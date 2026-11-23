//! The window: winit for events, softbuffer to present the tiny-skia pixmap. No GPU context.
pub mod chatui;
pub mod icon;
pub mod icons;
pub mod meeting_ui;
pub mod panels;
pub mod profile_ui;
pub mod render;
pub mod sidebar;
pub mod theme;

use crate::app::{LoginPhase, State};
use crate::proto::Net;
use crate::backend::Backend;
use crate::meeting::Hit;
use render::Text;
use crate::session::{self, Auth};
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::keyboard::{Key, ModifiersState, NamedKey};
use winit::window::{Window, WindowId};

/// The window icon (shown by X11 window managers; Wayland takes it from the .desktop file, see assets/).
fn app_icon() -> Option<winit::window::Icon> {
    let img = image::load_from_memory(include_bytes!("../../assets/icon.webp")).ok()?.to_rgba8();
    winit::window::Icon::from_rgba(img.to_vec(), img.width(), img.height()).ok()
}

/// Does this key event name the configured push-to-talk combination (`Super+'`, `F1`, `Ctrl+Alt+Space`...)? The
/// modifiers must be held when the key goes down; on release only the key counts, since the modifiers often let go first.
fn is_talk_key(k: &Key, mods: &ModifiersState, down: bool, combo: &crate::talk::Combo) -> bool {
    let key_ok = match k {
        Key::Named(NamedKey::Space) => combo.key == "space" || combo.key == " ",
        Key::Named(n) => format!("{n:?}").eq_ignore_ascii_case(&combo.key),
        Key::Character(c) => c.as_str().eq_ignore_ascii_case(&combo.key),
        _ => false,
    };
    key_ok && (!down || (mods.control_key() == combo.ctrl && mods.alt_key() == combo.alt && mods.shift_key() == combo.shift && mods.super_key() == combo.sup))
}

pub use crate::backend::Event as UserEvent;

/// The first finger on the screen, followed until it lifts. A tap clicks where it went down; past `DRAG` px it
/// scrolls what is under it instead; on the line between the people list and the chat, or the dock's header, it
/// drags those at once.
struct Finger {
    id: u64,
    start: (f32, f32),
    last: (f32, f32),
    drag: bool,
    /// It went down on something it drags: the press already happened, its moves are pointer moves.
    pressed: bool,
    /// A second finger came (a pinch): this one no longer taps.
    cancelled: bool,
}

/// How far (logical px) a finger moves before a tap becomes a drag.
const DRAG: f32 = 8.0;

/// The top byte of a softbuffer pixel: Android's surface is RGBA and composites a 0 there as transparent; the
/// desktop backends want it 0.
const OPAQUE: u32 = if cfg!(target_os = "android") { 0xff00_0000 } else { 0 };

/// The zoom during a pinch: the zoom when it started times how much the fingers spread since.
pub fn pinch_scale(s0: f32, d0: f32, d: f32) -> f32 {
    s0 * d / d0.max(1.0)
}

fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    (a.0 - b.0).hypot(a.1 - b.1)
}

struct App {
    st: State,
    /// The threads behind the session (socket, chat, calls, avatar loader) and what the state queues for them.
    be: Backend,
    window: Option<Arc<Window>>,
    surface: Option<softbuffer::Surface<Arc<Window>, Arc<Window>>>,
    text: Text,
    start: Instant,
    last: Instant,
    frames: u32,
    fps: u32,
    fps_t: Instant,
    /// (sum, max, count, since) of the draw times in ms this second, for SOWORKER_PERF
    perf: (f64, f64, u32, Instant),
    map: render::MapCache,
    cursor: (f32, f32),
    /// The pointer's shape: a hand over buttons, a resize arrow over the line between the people list and the chat.
    pointer: winit::window::CursorIcon,
    mods: ModifiersState,
    exit_after: Option<f64>,
    dry: bool,
    /// Test hook: behave as if the user confirmed the logout dialog after N seconds.
    logout_after: Option<f64>,
    /// Fingers on the screen (id, where), the first one's gesture, and a pinch going on (finger distance and zoom
    /// when it started).
    touches: Vec<(u64, (f32, f32))>,
    finger: Option<Finger>,
    pinch: Option<(f32, f32)>,
    /// What we draw while system bars cover the window's edges (it is copied inside them).
    inset_pm: Option<tiny_skia::Pixmap>,
    #[cfg(target_os = "android")]
    droid: Droid,
}

/// What the Android side was last told, so it is only told changes.
#[cfg(target_os = "android")]
struct Droid {
    app: Option<crate::android::AndroidApp>,
    /// The soft keyboard is up, and the text we last gave it or read from it.
    ime: bool,
    ime_text: String,
    /// The call the foreground service runs (title, mic, camera), while there is one.
    call: Option<(String, bool, bool)>,
    asked_mic: bool,
}

impl App {
    fn now(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    fn flush(&mut self) {
        self.sync_layout();
        if std::mem::take(&mut self.st.pair_wanted) {
            self.show_pair_code();
        }
        #[cfg(target_os = "android")]
        self.android_sync();
        self.be.flush(&mut self.st, self.start);
    }

    /// "Sign in a phone": the saved session as a QR code, on screen for `pairing::TTL` seconds.
    fn show_pair_code(&mut self) {
        let path = &self.be.session_path;
        // the file as saved (no token refresh: that would wait on the network here)
        let file = std::fs::read_to_string(path).map_err(|e| e.to_string()).and_then(|raw| serde_json::from_str::<crate::session::SessionFile>(&raw).map_err(|e| e.to_string()));
        let made = file.map(|f| {
            let (g, al) = (f.find["groupId"].as_str(), f.find["alias"].as_str());
            let (group, alias) = match (g, al) {
                (Some(g), Some(a)) => (g.to_string(), a.to_string()),
                _ => crate::session::load_space(path).unwrap_or_default(),
            };
            crate::pairing::Pairing { api_key: f.api_key.clone(), refresh_token: f.refresh_token.clone(), group, alias }
        });
        let now = self.now();
        match made {
            Ok(p) if !p.group.is_empty() => {
                self.st.pair_code = Some((crate::pairing::encode(&p, crate::pairing::unix_now()), crate::pairing::unix_secs() + crate::pairing::TTL as f64));
            }
            Ok(_) => self.st.say("Cannot make a sign-in code: this session has no office", now),
            Err(e) => self.st.say(format!("Cannot make a sign-in code: {e}"), now),
        }
        self.st.dirty = true;
    }

    /// Follow the window's width into the wide or the narrow (mobile) layout, and keep the chat's read state in step
    /// with whether it is on screen.
    fn sync_layout(&mut self) {
        if self.window.is_some() {
            let (w, h) = self.size();
            // a phone held sideways keeps the phone layout
            let mobile = panels::is_mobile(&self.st, w) || (cfg!(target_os = "android") && panels::is_mobile(&self.st, w.min(h)));
            if mobile != self.st.sidebar.mobile {
                self.st.sidebar.mobile = mobile;
                self.st.sidebar.hover = None;
                self.st.dirty = true;
            }
        }
        self.st.sync_chat_shown();
    }

    /// Connect everything for a signed-in user: gaia socket, calls, avatar loader.
    fn start_session(&mut self, auth: Auth) {
        let me = auth.uid.clone();
        self.be.start(auth, true);
        let ui = self.st.ui;
        self.st = State::new(&me, self.dry);
        self.st.restore_zoom();
        self.st.ui = ui;
    }

    /// Forget the saved sign-in and go back to the sign-in screen. Called when the socket thread confirms it closed, or after a timeout.
    fn finish_logout(&mut self) {
        self.be.finish_logout();
        let ui = self.st.ui;
        self.st = State::new("", self.dry);
        self.st.restore_zoom();
        self.st.ui = ui;
        self.st.login = Some(LoginPhase::Idle);
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }

    /// The pointer moved to `self.cursor`: drags, hover, the pointer's shape.
    fn pointer_moved(&mut self) {
        let (w, h) = self.size();
        if let Some(win) = &self.window {
            let c = self.cursor;
            if let Some(s) = profile_ui::slide(&self.st, c.0, w, h) {
                if s != self.st.noise_strength {
                    let now = self.now();
                    self.st.profile_act(crate::profile::Act::Strength(s), now);
                    self.st.dirty = true;
                }
            } else if self.st.sidebar.split_drag.is_some() {
                panels::drag_split(&mut self.st, c.1, w, h);
            } else if self.st.dock.drag.is_some() {
                meeting_ui::drag_to(&mut self.st, c.0, c.1);
            } else {
                meeting_ui::update_hover(&mut self.st, &mut self.text, c.0, c.1, w, h);
            }
            sidebar::update_hover(&mut self.st, c.0, c.1, w, h);
            let over_photo = self.st.profile.as_ref().is_some_and(|p| profile_ui::over_photo(&profile_ui::layout(&self.st, p, w, h), c.0, c.1));
            if let Some(p) = self.st.profile.as_mut().filter(|p| p.photo_hover != over_photo) {
                p.photo_hover = over_photo;
                self.st.dirty = true;
            }
            use winit::window::CursorIcon;
            let pointer = if self.st.profile.is_none() && panels::over_split(&self.st, c.0, c.1, w, h) {
                CursorIcon::RowResize
            } else if over_photo || panels::nav_hit(&self.st, c.0, c.1, w, h).is_some() || sidebar::hovered(&self.st, w, h).is_some() {
                CursorIcon::Pointer
            } else {
                CursorIcon::Default
            };
            if pointer != self.pointer {
                self.pointer = pointer;
                win.set_cursor(pointer);
            }
        }
    }

    /// The left button (or a finger) let go.
    fn release(&mut self) {
        self.st.end_slide();
        panels::end_split(&mut self.st);
        if self.window.is_some() {
            let (w, h) = self.size();
            meeting_ui::end_drag(&mut self.st, w, h);
        }
    }

    /// Scroll what is under the pointer by `dy` pixels (positive = the wheel turned up). False when nothing there
    /// scrolls.
    fn scroll(&mut self, dy: f32) -> bool {
        let ((w, h), c) = (self.size(), self.cursor);
        self.window.is_some() && (profile_ui::wheel(&mut self.st, dy, w, h) || chatui::wheel(&mut self.st, c.0, c.1, w, h, dy) || sidebar::wheel(&mut self.st, c.0, c.1, w, h, dy) || meeting_ui::wheel(&mut self.st, &mut self.text, c.0, c.1, w, h, dy))
    }

    /// A click (left button, or a tap) at `self.cursor`.
    fn press(&mut self) {
        if self.window.is_some() {
            let (sw, sh) = self.size();
            let now = self.now();
            let c = self.cursor;
            if self.st.login.is_some() {
                self.st.login_act = render::login_hit(&self.st, c.0, c.1, sw, sh);
            } else if self.st.pair_code.is_some() {
                self.st.pair_code = None; // any click puts the code away
                self.st.dirty = true;
            } else if self.st.confirm_logout {
                if let Some(choice) = render::modal_hit(&self.st, c.0, c.1, sw, sh) {
                    self.st.logout_choice(choice);
                }
            } else if self.st.confirm_leave.is_some() {
                if let Some(choice) = render::modal_hit(&self.st, c.0, c.1, sw, sh) {
                    self.st.leave_choice(choice, now);
                }
            } else if let Some(n) = panels::nav_hit(&self.st, c.0, c.1, sw, sh) {
                panels::nav_press(&mut self.st, n, now);
            } else if self.st.profile.is_some() {
                // a click outside the settings closes them (and does nothing else); the bars of a narrow window
                // around them only swallow the click
                let act = profile_ui::hit(&self.st, c.0, c.1, sw, sh).unwrap_or(crate::profile::Act::Close);
                if !(act == crate::profile::Act::Close && self.st.sidebar.mobile && panels::over_nav(&self.st, c.0, c.1, sw, sh)) {
                    self.st.profile_act(act, now);
                }
            } else if render::notice_rect(&self.st, &mut self.text, sw, sh).is_some_and(|r| r.contains(c.0, c.1)) {
                self.st.dismiss_transcript();
            } else if panels::begin_split(&mut self.st, c.0, c.1, sw, sh) {
            } else if let Some(h) = chatui::hit(&self.st, c.0, c.1, sw, sh) {
                chatui::press(&mut self.st, h);
            } else if let Some(h) = sidebar::hit(&self.st, c.0, c.1, sw, sh) {
                self.st.chat.focus = false;
                self.st.sidebar_press(h, now);
            } else if panels::over_nav(&self.st, c.0, c.1, sw, sh) {
                self.st.chat.focus = false;
            } else if let Some(h) = meeting_ui::hit(&self.st, &mut self.text, c.0, c.1, sw, sh) {
                match h {
                    Hit::Header => meeting_ui::begin_drag(&mut self.st, &mut self.text, c.0, c.1, sw, sh),
                    other => self.st.dock_press(other, now),
                }
            } else {
                self.st.chat.focus = false; // a click on the map leaves the chat input
                if let Some((wx, wy)) = self.st.to_world(self.cursor.0, self.cursor.1, sw, sh) {
                    self.st.click(wx, wy, self.mods.shift_key(), now);
                }
            }
        }
    }

    /// The window's edges the system draws over (left, top, right, bottom; the keyboard counts at the bottom), in
    /// pixels. Everything is drawn in the rectangle inside them.
    fn safe(&self) -> (u32, u32, u32, u32) {
        let i = self.st.insets;
        let px = |v: f32| v.max(0.0).round() as u32;
        (px(i.left), px(i.top), px(i.right), px(i.bottom.max(i.ime)))
    }

    /// The size of what we draw: the window minus the system bars and the keyboard.
    fn size(&self) -> (f32, f32) {
        let (l, t, r, b) = self.safe();
        self.window.as_ref().map_or((0.0, 0.0), |w| {
            let s = w.inner_size();
            (s.width.saturating_sub(l + r) as f32, s.height.saturating_sub(t + b) as f32)
        })
    }

    /// A window position in our drawing's coordinates.
    fn at(&self, x: f64, y: f64) -> (f32, f32) {
        let (l, t, _, _) = self.safe();
        (x as f32 - l as f32, y as f32 - t as f32)
    }

    /// Does a finger going down here drag something (the split line, the dock's header) rather than tap?
    fn grabs(&mut self, at: (f32, f32)) -> bool {
        let (w, h) = self.size();
        let st = &self.st;
        if st.login.is_some() || st.profile.is_some() || st.confirm_logout || st.confirm_leave.is_some() || st.pair_code.is_some() {
            return false;
        }
        panels::over_split(st, at.0, at.1, w, h) || meeting_ui::hit(st, &mut self.text, at.0, at.1, w, h) == Some(Hit::Header)
    }

    /// Is this point on the map itself (not a panel, the dock or a button over it)? Only there two fingers zoom.
    fn on_map(&mut self, at: (f32, f32)) -> bool {
        let (w, h) = self.size();
        let st = &self.st;
        let f = panels::frame(st, w, h);
        st.login.is_none()
            && st.profile.is_none()
            && f.main_shown
            && f.main.contains(at.0, at.1)
            && meeting_ui::hit(st, &mut self.text, at.0, at.1, w, h).is_none()
            && sidebar::hit(st, at.0, at.1, w, h).is_none()
            && chatui::hit(st, at.0, at.1, w, h).is_none()
    }

    /// Touch input, mapped onto the mouse paths: tap = click, drag = scroll (or drag the split / the dock), two
    /// fingers on the map = pinch to zoom. A third finger is ignored.
    fn touch(&mut self, t: winit::event::Touch) {
        use winit::event::TouchPhase;
        let pos = self.at(t.location.x, t.location.y);
        match t.phase {
            TouchPhase::Started => {
                if self.touches.len() >= 2 {
                    return;
                }
                self.touches.push((t.id, pos));
                if self.touches.len() == 1 {
                    self.cursor = pos;
                    let pressed = self.grabs(pos);
                    if pressed {
                        self.press();
                    }
                    self.finger = Some(Finger { id: t.id, start: pos, last: pos, drag: false, pressed, cancelled: false });
                } else {
                    let start = self.finger.as_ref().filter(|f| !f.pressed).map(|f| f.start);
                    if let Some(f) = self.finger.as_mut() {
                        f.cancelled = true; // lifting the fingers must not walk
                    }
                    if start.is_some_and(|s| self.on_map(s)) {
                        self.pinch = Some((dist(self.touches[0].1, self.touches[1].1), self.st.scale));
                    }
                }
            }
            TouchPhase::Moved => {
                if let Some(p) = self.touches.iter_mut().find(|(id, _)| *id == t.id) {
                    p.1 = pos;
                }
                if let Some((d0, s0)) = self.pinch {
                    if self.touches.len() == 2 {
                        self.st.set_zoom(pinch_scale(s0, d0, dist(self.touches[0].1, self.touches[1].1)));
                    }
                    return;
                }
                let ui = self.st.ui;
                let Some(f) = self.finger.as_mut().filter(|f| f.id == t.id && !f.cancelled) else { return };
                if f.pressed {
                    self.cursor = pos;
                    return self.pointer_moved();
                }
                if !f.drag && dist(pos, f.start) > DRAG * ui {
                    f.drag = true;
                }
                if f.drag {
                    let dy = pos.1 - f.last.1;
                    f.last = pos;
                    self.cursor = f.start; // scroll what the finger went down on
                    if dy != 0.0 {
                        self.scroll(dy);
                    }
                }
            }
            TouchPhase::Ended | TouchPhase::Cancelled => {
                self.touches.retain(|(id, _)| *id != t.id);
                if self.pinch.is_some() && self.touches.len() < 2 {
                    self.pinch = None;
                    self.st.save_zoom();
                }
                if self.finger.as_ref().is_some_and(|f| f.id == t.id) {
                    let f = self.finger.take().unwrap();
                    if f.pressed {
                        self.cursor = pos;
                        self.release();
                    } else if t.phase == TouchPhase::Ended && !f.drag && !f.cancelled {
                        self.cursor = f.start;
                        self.press();
                        self.release();
                    }
                }
            }
        }
    }

    /// Back: close the innermost thing open; with nothing open, the app goes to the background (Android).
    fn back(&mut self) {
        let now = self.now();
        if !self.st.back(now) {
            #[cfg(target_os = "android")]
            crate::android::move_task_to_back();
        }
    }

    fn draw(&mut self) {
        let t0 = Instant::now();
        self.draw_inner();
        if std::env::var_os("SOWORKER_PERF").is_some() {
            let ms = t0.elapsed().as_secs_f64() * 1000.0;
            let p = &mut self.perf;
            (p.0, p.1, p.2) = (p.0 + ms, p.1.max(ms), p.2 + 1);
            if p.3.elapsed() >= Duration::from_secs(1) {
                eprintln!("perf: {} draws/s, draw avg {:.1} ms, max {:.1} ms", p.2, p.0 / p.2 as f64, p.1);
                *p = (0.0, 0.0, 0, Instant::now());
            }
        }
    }

    fn draw_inner(&mut self) {
        let (l, t, r, b) = self.safe();
        let (Some(win), Some(surface)) = (&self.window, &mut self.surface) else { return };
        let size = win.inner_size();
        let (Some(w), Some(h)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) else { return };
        let _ = surface.resize(w, h); // no-op when the size is unchanged
        if let Ok(mut buf) = surface.buffer_mut() {
            // draw straight into the shared-memory buffer (no second window-sized pixmap), then swap R and B in place
            let (n, wp, hp) = (buf.len(), w.get(), h.get());
            if n == wp as usize * hp as usize {
                // SAFETY: a `[u32]` of n elements is 4n initialised bytes; u8 has no alignment requirement.
                let bytes = unsafe { std::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut u8, n * 4) };
                if self.st.scene || self.st.dirty {
                    self.map.valid = false;
                    self.st.scene = false;
                }
                let (cw, ch) = (wp.saturating_sub(l + r), hp.saturating_sub(t + b));
                if (l, t, r, b) == (0, 0, 0, 0) {
                    if let Some(mut pm) = tiny_skia::PixmapMut::from_bytes(bytes, wp, hp) {
                        render::draw(&self.st, &mut pm, &mut self.text, &mut self.map);
                    }
                    for px in buf.iter_mut() {
                        *px = (px.swap_bytes().rotate_right(8) & 0x00ff_ffff) | OPAQUE; // 0xAABBGGRR -> 0x00RRGGBB
                    }
                } else if cw > 0 && ch > 0 {
                    // system bars (Android) over the edges: draw inside them, then copy that in place
                    if self.inset_pm.as_ref().is_none_or(|p| (p.width(), p.height()) != (cw, ch)) {
                        self.inset_pm = tiny_skia::Pixmap::new(cw, ch);
                    }
                    if let Some(pm) = self.inset_pm.as_mut() {
                        render::draw(&self.st, &mut pm.as_mut(), &mut self.text, &mut self.map);
                        let bg = theme::SURFACE;
                        buf.fill(u32::from_be_bytes([0, bg[0], bg[1], bg[2]]) | OPAQUE);
                        let src = pm.data();
                        for y in 0..ch as usize {
                            let row = &src[y * cw as usize * 4..(y + 1) * cw as usize * 4];
                            let off = (y + t as usize) * wp as usize + l as usize;
                            for (d, s) in buf[off..off + cw as usize].iter_mut().zip(row.chunks_exact(4)) {
                                *d = u32::from_be_bytes([0, s[0], s[1], s[2]]) | OPAQUE;
                            }
                        }
                    }
                }
                let _ = buf.present();
            }
        }
        self.frames += 1;
        if self.fps_t.elapsed() >= Duration::from_secs(1) {
            (self.fps, self.frames, self.fps_t) = (self.frames, 0, Instant::now());
        }
    }
}

#[cfg(target_os = "android")]
impl App {
    /// Something Kotlin reported.
    fn android_msg(&mut self, m: crate::android::Msg, now: f64) {
        use crate::android::{self as droid, Msg};
        use crate::meeting::Ctl;
        match m {
            Msg::Back => self.back(),
            Msg::Media(kind) if self.st.call.joined.is_some() => match kind {
                1 if !self.st.call.mic => self.st.talk(crate::talk::TalkAct::Toggle, now),
                2 if self.st.call.mic => self.st.talk(crate::talk::TalkAct::Toggle, now),
                0 => self.st.talk(crate::talk::TalkAct::Toggle, now),
                3 => self.st.dock_press(Hit::Ctl(Ctl::Leave), now),
                _ => droid::set_mic_state(self.st.call.mic), // already so: let the session show the truth
            },
            Msg::Media(_) => {}
            Msg::Screen { locked } => self.st.locked = locked,
            Msg::Insets => {
                self.st.insets = droid::insets();
                (self.st.dirty, self.st.scene) = (true, true);
            }
            Msg::Permission { name, granted } => {
                eprintln!("[{now:6.1}] permission {name}: {}", if granted { "granted" } else { "denied" });
                match (name.as_str(), granted) {
                    (droid::RECORD_AUDIO, false) => self.st.say("No microphone permission: you can listen, not talk", now),
                    (droid::RECORD_AUDIO, true) if self.st.call.joined.is_some() => {
                        self.st.say("Microphone allowed: tap the mic to talk", now);
                        if let Some(c) = &self.droid.call {
                            droid::start_call(&c.0, c.1, c.2); // the service may now use the microphone
                        }
                    }
                    (droid::CAMERA, false) if self.st.login == Some(LoginPhase::Scanning) => {
                        self.st.login = Some(LoginPhase::Failed("SoWorker needs the camera to read the code.".into()));
                    }
                    (droid::CAMERA, true) if self.st.login == Some(LoginPhase::Scanning) => crate::android_scan::start(),
                    (droid::CAMERA, false) => self.st.say("No camera permission", now),
                    (droid::CAMERA, true) => self.st.toggle_cam(now),
                    _ => {}
                }
            }
            // the WebView closed without a session (back pressed): only matters while we wait for it
            Msg::Login { refresh_token, .. } if refresh_token.is_empty() => {
                if self.st.login == Some(LoginPhase::Waiting) {
                    self.st.login = Some(LoginPhase::Idle);
                }
            }
            Msg::Login { refresh_token, api_key, group, alias } => {
                self.st.login = Some(LoginPhase::Waiting);
                self.be.adopt_login(api_key, refresh_token, group, alias);
            }
            Msg::Qr(text) if self.st.login == Some(LoginPhase::Scanning) => self.pair(&text, now),
            Msg::Qr(_) => {}
            Msg::ScanFrame => {}
            Msg::ScanFailed(e) => {
                if self.st.login == Some(LoginPhase::Scanning) {
                    self.st.login = Some(LoginPhase::Failed(e));
                }
            }
        }
        self.st.dirty = true;
    }

    /// A QR code from the desktop's "Sign in a phone" (see `pairing`): sign in with the session it carries.
    fn pair(&mut self, text: &str, now: f64) {
        match crate::pairing::open(text, crate::pairing::unix_now()) {
            Ok(p) => {
                eprintln!("[{now:6.1}] pairing: signing in to {}/{}", p.group, p.alias);
                self.st.login = Some(LoginPhase::Waiting);
                self.be.adopt_login(p.api_key, p.refresh_token, p.group, p.alias);
            }
            Err(e) => self.st.login = Some(LoginPhase::Failed(e)),
        }
    }

    /// Before the state's queues go out: ask for permissions a command needs, and keep the foreground service, the
    /// media session and the soft keyboard in step with the state.
    fn android_sync(&mut self) {
        use crate::android::{self as droid};
        use crate::app::LoginAct;
        use crate::calls::MediaCmd;
        // sign-in: SoWork's page in a WebView instead of Chrome, or the desktop's code through the camera
        match (self.st.login_act, &self.st.login) {
            (Some(LoginAct::Start), _) => {
                self.st.login_act = None;
                self.st.login = Some(LoginPhase::Waiting);
                droid::open_login();
            }
            (Some(LoginAct::Scan), _) => {
                self.st.login_act = None;
                self.st.login = Some(LoginPhase::Scanning);
                if droid::has_permission(droid::CAMERA) {
                    crate::android_scan::start();
                } else {
                    droid::request_permissions(&[droid::CAMERA]); // the scan starts on the yes
                }
            }
            (Some(LoginAct::Cancel), Some(LoginPhase::Scanning | LoginPhase::Waiting)) => {
                self.st.login_act = None;
                crate::android_scan::stop();
                self.st.login = Some(LoginPhase::Idle); // the WebView, if open, reports its own end and is ignored
            }
            _ => {}
        }
        let st = &mut self.st;
        // the mic and the camera need the user's yes first; the command is dropped, they press again once allowed
        let (mut need_mic, mut need_cam) = (false, false);
        st.media_out.retain(|c| match c {
            MediaCmd::Mic(true) if !droid::has_permission(droid::RECORD_AUDIO) => {
                need_mic = true;
                false
            }
            MediaCmd::Camera(true) if !droid::has_permission(droid::CAMERA) => {
                need_cam = true;
                false
            }
            _ => true,
        });
        let joining = st.media_out.iter().any(|c| matches!(c, MediaCmd::Join { .. }));
        let mut ask: Vec<&str> = vec![];
        if need_mic || (joining && !self.droid.asked_mic && !droid::has_permission(droid::RECORD_AUDIO)) {
            ask.push(droid::RECORD_AUDIO);
            ask.push(droid::POST_NOTIFICATIONS); // for the ongoing-call notification
            self.droid.asked_mic = true;
        }
        if need_cam {
            ask.push(droid::CAMERA);
        }
        if !ask.is_empty() {
            droid::request_permissions(&ask);
        }
        // the foreground service runs while we are in a call (or joining one)
        let want = (st.call.want.is_some() || st.call.joined.is_some()).then(|| {
            let zone = st.cur_zone_id.as_ref().and_then(|id| st.world.zones.iter().find(|z| &z.id == id)).and_then(|z| z.name.clone());
            (zone.unwrap_or_else(|| "Meeting".into()), st.call.mic, st.call.cam)
        });
        if want != self.droid.call {
            match &want {
                Some((title, mic, cam)) => {
                    let same_call = self.droid.call.as_ref().is_some_and(|c| c.0 == *title && c.2 == *cam);
                    if same_call {
                        droid::set_mic_state(*mic);
                    } else {
                        if self.droid.call.is_none() {
                            // the route chosen in the profile panel (saved), before the service starts routing
                            droid::set_audio_route(crate::devices::chosen().output.as_deref().unwrap_or("auto"));
                        }
                        droid::start_call(title, *mic, *cam);
                    }
                }
                None => droid::stop_call(),
            }
            self.droid.call = want;
        }
        self.sync_ime();
    }

    /// The soft keyboard is up while the chat input has focus. GameActivity keeps the text the keyboard edits; it is
    /// read back here, and a newline in it sends the message.
    fn sync_ime(&mut self) {
        use winit::platform::android::activity::input::{ImeOptions, InputType, TextInputAction, TextInputState, TextSpan};
        let Some(w) = self.window.clone() else { return };
        let focus = self.st.chat.focus && self.st.login.is_none();
        let Some(app) = self.droid.app.clone() else { return };
        let give = |text: &str| {
            let n = text.chars().count();
            app.set_text_input_state(TextInputState { text: text.to_string(), selection: TextSpan { start: n, end: n }, compose_region: None });
        };
        if focus && !self.droid.ime {
            // multi-line text: the keyboard's return key types a newline, which we take as Send
            app.set_ime_editor_info(InputType::from_bits_retain(0x1 | 0x4000 | 0x8000 | 0x20000), TextInputAction::Unspecified, ImeOptions::from_bits_retain(0x4000_0000 | 0x0200_0000));
            give(&self.st.chat.input);
            self.droid.ime_text = self.st.chat.input.clone();
            w.set_ime_allowed(true);
            self.droid.ime = true;
        } else if !focus && self.droid.ime {
            w.set_ime_allowed(false);
            self.droid.ime = false;
        }
        if !self.droid.ime {
            return;
        }
        let ts = app.text_input_state();
        if ts.text != self.droid.ime_text {
            let now = self.now();
            match ts.text.split_once('\n') {
                Some((line, _)) => {
                    self.st.chat.input = line.to_string();
                    self.st.chat_key(Some("Enter"), None, now);
                    self.st.chat.focus = true; // keep typing
                }
                None => {
                    self.st.chat.input = ts.text.chars().filter(|c| !c.is_control()).collect();
                    self.st.dirty = true;
                }
            }
            self.droid.ime_text = ts.text;
        }
        if self.st.chat.input != self.droid.ime_text {
            give(&self.st.chat.input); // we changed it (sent, cleared)
            self.droid.ime_text = self.st.chat.input.clone();
        }
    }
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if let Some(window) = self.window.clone() {
            // Android: back on screen with a new native window. Only the drawing surface is made again.
            if let Ok(ctx) = softbuffer::Context::new(window.clone()) {
                self.surface = softbuffer::Surface::new(&ctx, window.clone()).ok();
            }
            self.st.ui = window.scale_factor() as f32;
            let now = self.now();
            self.st.set_visible(true, now);
            (self.st.dirty, self.st.scene) = (true, true);
            self.flush();
            window.request_redraw();
            return;
        }
        let attrs = Window::default_attributes().with_title("SoWorker").with_inner_size(LogicalSize::new(1000.0, 700.0)).with_window_icon(app_icon());
        // the app id/class `soworker` is how the desktop finds assets/soworker.desktop (and with it the icon) on Wayland
        #[cfg(target_os = "linux")]
        let attrs = winit::platform::wayland::WindowAttributesExtWayland::with_name(attrs, "soworker", "soworker");
        #[cfg(target_os = "linux")]
        let attrs = winit::platform::x11::WindowAttributesExtX11::with_name(attrs, "soworker", "soworker");
        let Ok(window) = el.create_window(attrs) else { return el.exit() };
        let window = Arc::new(window);
        if let Ok(ctx) = softbuffer::Context::new(window.clone()) {
            self.surface = softbuffer::Surface::new(&ctx, window.clone()).ok();
        }
        self.st.ui = window.scale_factor() as f32;
        self.window = Some(window);
    }

    /// Android took the window away (home, lock, another app): the surface must not touch it any more. Nothing is
    /// on screen, so video stops (low-data mode, see `State::set_visible`); the call and the socket go on.
    fn suspended(&mut self, _el: &ActiveEventLoop) {
        self.surface = None;
        self.touches.clear();
        (self.finger, self.pinch) = (None, None);
        let now = self.now();
        self.st.set_visible(false, now);
        self.flush();
    }

    fn user_event(&mut self, _el: &ActiveEventLoop, ev: UserEvent) {
        let now = self.now();
        if matches!(ev, UserEvent::Net(Net::LoggedOut)) {
            return self.finish_logout();
        }
        match ev {
            UserEvent::Net(e) => self.st.apply(e, now),
            UserEvent::Media(e) => self.st.apply_media(e, now),
            UserEvent::Talk(a) => self.st.talk(a, now),
            #[cfg(target_os = "android")]
            UserEvent::Android(m) => self.android_msg(m, now),
            UserEvent::Login(Ok(auth)) => self.start_session(auth),
            UserEvent::Login(Err(e)) => {
                self.st.login = Some(LoginPhase::after_error(e));
                self.st.dirty = true;
            }
        }
        self.flush();
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, ev: WindowEvent) {
        match ev {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::Resized(_) => self.st.dirty = true,
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.st.ui = scale_factor as f32;
                self.st.dirty = true;
            }
            WindowEvent::RedrawRequested => self.draw(),
            WindowEvent::ModifiersChanged(m) => self.mods = m.state(),
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor = self.at(position.x, position.y);
                self.pointer_moved();
            }
            WindowEvent::CursorLeft { .. } => {
                if self.st.sidebar.hover.take().is_some() {
                    self.st.dirty = true;
                }
            }
            WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. } => self.release(),
            WindowEvent::MouseWheel { delta, .. } => {
                let dy = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y.signum() * 40.0 * self.st.ui,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32,
                };
                if dy != 0.0 && !self.scroll(dy) {
                    self.st.zoom(dy > 0.0);
                }
            }
            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => self.press(),
            WindowEvent::Touch(t) => self.touch(t),
            WindowEvent::KeyboardInput { event, .. } => {
                let down = event.state == ElementState::Pressed;
                // a hardware back key (Kotlin's back callback takes the gesture and the on-screen button first)
                if event.logical_key == Key::Named(NamedKey::BrowserBack) || event.logical_key == Key::Named(NamedKey::GoBack) {
                    if down {
                        self.back();
                    }
                    return self.flush();
                }
                // Android: play/pause comes through the media session only (else it would toggle twice)
                if cfg!(target_os = "android") && matches!(event.logical_key, Key::Named(NamedKey::MediaPlayPause | NamedKey::MediaPlay | NamedKey::MediaPause)) {
                    return;
                }
                if self.st.chat.focus {
                    // typing into the chat: nothing here walks the avatar or toggles the mic
                    if down {
                        let named = match &event.logical_key {
                            Key::Named(NamedKey::Enter) => Some("Enter"),
                            Key::Named(NamedKey::Backspace) => Some("Backspace"),
                            Key::Named(NamedKey::Escape) => Some("Escape"),
                            _ => None,
                        };
                        let text = event.text.as_ref().map(|t| t.as_str());
                        let now = self.now();
                        self.st.chat_key(named, text, now);
                    }
                    self.st.keys = Default::default();
                    self.flush();
                    return;
                }
                if is_talk_key(&event.logical_key, &self.mods, down, &crate::talk::Combo::parse(&self.st.talk_key)) {
                    if !event.repeat {
                        let now = self.now();
                        self.st.talk(if down { crate::talk::TalkAct::Down } else { crate::talk::TalkAct::Up }, now);
                    }
                    self.flush();
                    return;
                }
                let k = &mut self.st.keys;
                match &event.logical_key {
                    Key::Named(NamedKey::ArrowUp) => k.up = down,
                    Key::Named(NamedKey::ArrowDown) => k.down = down,
                    Key::Named(NamedKey::ArrowLeft) => k.left = down,
                    Key::Named(NamedKey::ArrowRight) => k.right = down,
                    Key::Named(NamedKey::Escape) if down && self.st.dock.tile_full => {
                        self.st.dock.tile_full = false;
                        self.st.dirty = true;
                    }
                    Key::Named(NamedKey::Escape) if down && self.st.pair_code.is_some() => {
                        self.st.pair_code = None;
                        self.st.dirty = true;
                    }
                    Key::Named(NamedKey::Escape) if down && self.st.confirm_logout => self.st.logout_choice(false),
                    Key::Named(NamedKey::Escape) if down && self.st.confirm_leave.is_some() => {
                        let now = self.now();
                        self.st.leave_choice(false, now);
                    }
                    Key::Named(NamedKey::Enter) if down && self.st.confirm_leave.is_some() => {
                        let now = self.now();
                        self.st.leave_choice(true, now);
                    }
                    Key::Named(NamedKey::Escape) if down && self.st.profile.is_some() => {
                        let list_open = self.st.profile.as_ref().is_some_and(|p| p.open.is_some());
                        let now = self.now();
                        self.st.profile_act(if list_open { crate::profile::Act::Panel } else { crate::profile::Act::Close }, now);
                    }
                    Key::Character(c) => match (c.to_lowercase().as_str(), down, event.repeat) {
                        ("d", true, false) if self.mods.control_key() => {
                            let now = self.now();
                            self.st.go_to_desk(now);
                        }
                        ("d", ..) if self.mods.control_key() => {}
                        ("h", true, false) => {
                            let now = self.now();
                            self.st.toggle_hand(now);
                        }
                        ("w", ..) => k.up = down,
                        ("s", ..) => k.down = down,
                        ("a", ..) => k.left = down,
                        ("d", ..) => k.right = down,
                        ("r", true, false) => self.st.respawn(),
                        ("m", true, false) => self.st.toggle_mic(),
                        ("c", true, false) => {
                            let now = self.now();
                            self.st.toggle_cam(now);
                        }
                        ("b", true, false) => {
                            let now = self.now();
                            self.st.toggle_available(now);
                        }
                        _ => {}
                    },
                    _ => {}
                }
                self.st.dirty = true;
            }
            WindowEvent::Focused(f) => {
                let now = self.now();
                self.st.set_focused(f, now);
                if !f {
                    self.st.keys = Default::default();
                }
            }
            _ => {}
        }
        self.flush();
    }

    fn about_to_wait(&mut self, el: &ActiveEventLoop) {
        let (now, dt) = (self.now(), self.last.elapsed().as_secs_f32().min(0.05));
        self.last = Instant::now();
        if self.exit_after.is_some_and(|t| now > t) {
            let (rss, pss) = render::proc_mem();
            println!("exit-after: rss {:.1} MB, pss {:.1} MB, fps {}", rss as f64 / 1048576.0, pss as f64 / 1048576.0, self.fps);
            return el.exit();
        }
        if let Some(to) = self.st.switch.take() {
            match crate::mode::relaunch(to, &std::env::args().skip(1).collect::<Vec<_>>()) {
                Ok(()) => return el.exit(),
                Err(e) => self.st.say(e, now),
            }
        }
        if self.logout_after.is_some_and(|t| now > t) && !self.st.logout_requested {
            self.st.logout_choice(true);
        }
        if self.be.logging_out().is_some_and(|d| d > Duration::from_millis(2500)) {
            return self.finish_logout(); // the socket thread did not answer in time (offline?): leave anyway
        }
        // a fullscreen tile takes the whole OS window with it, and leaving either leaves both
        if let Some(w) = self.window.as_ref().filter(|_| !cfg!(target_os = "android")) {
            if self.st.dock.tile_full != w.fullscreen().is_some() {
                w.set_fullscreen(self.st.dock.tile_full.then_some(winit::window::Fullscreen::Borderless(None)));
            }
        }
        if let Some(t) = self.st.dock.hover.moved {
            if t.elapsed() >= crate::meeting::Hover::SHOW {
                self.st.dock.hover.moved = None; // the fullscreen tile's buttons fade away
                self.st.dirty = true;
            }
        }
        let redraw = self.st.update(dt, now);
        self.flush();
        if self.window.is_some() && self.surface.is_none() {
            // in the background (Android): nothing to draw; wake up for events only
            return el.set_control_flow(ControlFlow::Wait);
        }
        if redraw {
            if let Some(w) = &self.window {
                w.request_redraw();
            }
        }
        // tick at ~60 Hz only while something moves; otherwise sleep until the next event
        el.set_control_flow(if self.st.is_active() {
            ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(16))
        } else if let Some(t) = self.st.dock.hover.moved {
            ControlFlow::WaitUntil(t + crate::meeting::Hover::SHOW)
        } else if self.st.pair_code.is_some() {
            ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(250)) // the code's countdown
        } else if self.be.logging_out().is_some() {
            ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(100))
        } else if let Some(t) = self.logout_after.filter(|_| !self.st.logout_requested) {
            ControlFlow::WaitUntil(self.start + Duration::from_secs_f64(t + 0.05))
        } else if let Some(t) = self.exit_after {
            ControlFlow::WaitUntil(self.start + Duration::from_secs_f64(t + 0.05))
        } else {
            ControlFlow::Wait
        });
    }
}

const LEGACY: [&str; 4] = ["native/.session.json", ".session.json", "app/.session.json", "../app/.session.json"];

/// The session file to use. Sign-ins now live outside the project (`~/.config/soworker/session.json`); one an
/// older version left inside the project is moved there on first start.
fn default_session() -> String {
    let target = session::default_path();
    if let Some(from) = session::migrate_legacy(&target, &LEGACY) {
        eprintln!("moved your saved sign-in from {from} to {target}");
    }
    session::ensure_private_dir(&target);
    target
}

pub fn run(args: &[String]) -> Result<(), String> {
    crate::desktop::ensure();
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().map_err(|e| e.to_string())?;
    let mut app = make_app(&event_loop, args);
    let proxy = event_loop.create_proxy();
    app.st.talk_key = crate::talk::key_label();
    let p = proxy.clone();
    crate::talk::listen(move |a| drop(p.send_event(UserEvent::Talk(a))));
    let p = proxy.clone();
    crate::talk::portal(move |a| drop(p.send_event(UserEvent::Talk(a))), app.st.talk_key.clone());
    sign_in(&mut app, args);
    event_loop.run_app(&mut app).map_err(|e| e.to_string())
}

/// The Android app (`android_main`, after `android::main` set things up): the same client in the activity's window.
#[cfg(target_os = "android")]
pub fn run_android(droid: crate::android::AndroidApp, args: &[String]) -> Result<(), String> {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    let event_loop = EventLoop::<UserEvent>::with_user_event().with_android_app(droid.clone()).build().map_err(|e| e.to_string())?;
    crate::android::set_proxy(event_loop.create_proxy());
    let mut app = make_app(&event_loop, args);
    app.droid.app = Some(droid);
    app.st.insets = crate::android::insets();
    sign_in(&mut app, args);
    event_loop.run_app(&mut app).map_err(|e| e.to_string())
}

/// Use the saved sign-in, or show the sign-in screen.
fn sign_in(app: &mut App, args: &[String]) {
    let path = app.be.session_path.clone();
    if args.iter().any(|a| a == "--show-login") {
        app.st.login = Some(LoginPhase::Idle); // layout check without touching any saved session
    } else {
        match Auth::load(&path) {
            Ok(auth) => app.start_session(auth),
            Err(e) => {
                // no saved sign-in (or it no longer works): show the sign-in screen
                app.st.login = Some(LoginPhase::after_load_error(std::path::Path::new(&path).exists(), e));
            }
        }
    }
}

fn make_app(event_loop: &EventLoop<UserEvent>, args: &[String]) -> App {
    let opt = |n: &str| args.iter().position(|a| a == n).and_then(|i| args.get(i + 1)).cloned();
    let dry = args.iter().any(|a| a == "--dry-run");
    let proxy: EventLoopProxy<UserEvent> = event_loop.create_proxy();
    let path = opt("--session").unwrap_or_else(default_session);
    let emit: crate::backend::Emit = {
        let p = proxy.clone();
        Arc::new(move |e| drop(p.send_event(e)))
    };
    App {
        st: State::new("", dry),
        be: Backend::new(emit, dry, path.clone()),
        window: None,
        surface: None,
        text: Text::new(),
        start: Instant::now(),
        last: Instant::now(),
        frames: 0,
        fps: 0,
        perf: (0.0, 0.0, 0, Instant::now()),
        map: render::MapCache::default(),
        fps_t: Instant::now(),
        cursor: (0.0, 0.0),
        pointer: winit::window::CursorIcon::Default,
        mods: ModifiersState::default(),
        exit_after: opt("--exit-after").and_then(|s| s.parse().ok()),
        dry,
        logout_after: opt("--logout-after").and_then(|s| s.parse().ok()),
        touches: vec![],
        inset_pm: None,
        finger: None,
        pinch: None,
        #[cfg(target_os = "android")]
        droid: Droid { app: None, ime: false, ime_text: String::new(), call: None, asked_mic: false },
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn talk_combo_needs_its_modifiers_on_press_but_not_on_release() {
        use winit::keyboard::{Key, ModifiersState};
        let combo = crate::talk::Combo::parse("Super+'");
        let key = Key::Character("'".into());
        assert!(super::is_talk_key(&key, &ModifiersState::SUPER, true, &combo));
        assert!(!super::is_talk_key(&key, &ModifiersState::empty(), true, &combo), "a bare ' is just typing");
        assert!(!super::is_talk_key(&key, &(ModifiersState::SUPER | ModifiersState::SHIFT), true, &combo));
        assert!(super::is_talk_key(&key, &ModifiersState::empty(), false, &combo), "Super may already be up on release");
        assert!(super::is_talk_key(&Key::Named(winit::keyboard::NamedKey::F1), &ModifiersState::empty(), true, &crate::talk::Combo::parse("F1")));
    }

    #[test]
    fn the_app_icon_decodes() {
        assert!(super::app_icon().is_some());
    }
}
