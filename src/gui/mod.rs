//! The window: winit for events, softbuffer to present the tiny-skia pixmap. No GPU context.
pub mod chatui;
pub mod icon;
pub mod icons;
pub mod meeting_ui;
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
    cursor: (f32, f32),
    /// The mouse is over the user icon, so the pointer is a hand.
    hand: bool,
    mods: ModifiersState,
    exit_after: Option<f64>,
    dry: bool,
    /// Test hook: behave as if the user confirmed the logout dialog after N seconds.
    logout_after: Option<f64>,
}

impl App {
    fn now(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    fn flush(&mut self) {
        self.be.flush(&mut self.st, self.start);
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

    fn draw(&mut self) {
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
                if let Some(mut pm) = tiny_skia::PixmapMut::from_bytes(bytes, wp, hp) {
                    render::draw(&self.st, &mut pm, &mut self.text);
                }
                for px in buf.iter_mut() {
                    *px = ((*px & 0xff) << 16) | (*px & 0xff00) | ((*px >> 16) & 0xff);
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

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.window.is_some() {
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

    fn user_event(&mut self, _el: &ActiveEventLoop, ev: UserEvent) {
        let now = self.now();
        if matches!(ev, UserEvent::Net(Net::LoggedOut)) {
            return self.finish_logout();
        }
        match ev {
            UserEvent::Net(e) => self.st.apply(e, now),
            UserEvent::Media(e) => self.st.apply_media(e, now),
            UserEvent::Talk(a) => self.st.talk(a, now),
            UserEvent::Login(Ok(auth)) => self.start_session(auth),
            UserEvent::Login(Err(e)) => {
                self.st.login = Some(if e.contains("cancelled") { LoginPhase::Idle } else { LoginPhase::Failed(e) });
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
                self.cursor = (position.x as f32, position.y as f32);
                if let Some(win) = &self.window {
                    let s = win.inner_size();
                    let (c, w, h) = (self.cursor, s.width as f32, s.height as f32);
                    if self.st.dock.drag.is_some() {
                        meeting_ui::drag_to(&mut self.st, c.0, c.1);
                    } else {
                        meeting_ui::update_hover(&mut self.st, &mut self.text, c.0, c.1, w, h);
                    }
                    let hand = sidebar::hit(&self.st, c.0, c.1, h) == Some(crate::directory::Hit::Profile);
                    if hand != self.hand {
                        self.hand = hand;
                        win.set_cursor(if hand { winit::window::CursorIcon::Pointer } else { winit::window::CursorIcon::Default });
                    }
                }
            }
            WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. } => {
                if let Some(win) = &self.window {
                    let s = win.inner_size();
                    meeting_ui::end_drag(&mut self.st, s.width as f32, s.height as f32);
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let dy = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32,
                };
                if dy != 0.0 {
                    let consumed = self.window.as_ref().is_some_and(|win| {
                        let sz = win.inner_size();
                        let c = self.cursor;
                        chatui::wheel(&mut self.st, c.0, c.1, sz.width as f32, sz.height as f32, dy) || sidebar::wheel(&mut self.st, c.0, c.1, sz.height as f32, dy) || meeting_ui::wheel(&mut self.st, &mut self.text, c.0, c.1, sz.width as f32, sz.height as f32, dy)
                    });
                    if !consumed {
                        self.st.zoom(dy > 0.0);
                    }
                }
            }
            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                if let Some(win) = &self.window {
                    let s = win.inner_size();
                    let now = self.now();
                    let (c, sw, sh) = (self.cursor, s.width as f32, s.height as f32);
                    if self.st.login.is_some() {
                        self.st.login_act = render::login_hit(&self.st, c.0, c.1, sw, sh);
                    } else if self.st.confirm_logout {
                        if let Some(choice) = render::modal_hit(&self.st, c.0, c.1, sw, sh) {
                            self.st.logout_choice(choice);
                        }
                    } else if self.st.profile.is_some() {
                        // a click outside the profile panel closes it (and does nothing else)
                        let act = profile_ui::hit(&self.st, c.0, c.1, sh).unwrap_or(crate::profile::Act::Close);
                        self.st.profile_act(act);
                    } else if let Some(h) = chatui::hit(&self.st, c.0, c.1, sw, sh) {
                        chatui::press(&mut self.st, h);
                    } else if let Some(h) = sidebar::hit(&self.st, c.0, c.1, sh) {
                        self.st.sidebar_press(h, now);
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
            WindowEvent::KeyboardInput { event, .. } => {
                let down = event.state == ElementState::Pressed;
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
                    Key::Named(NamedKey::Escape) if down && self.st.confirm_logout => self.st.logout_choice(false),
                    Key::Named(NamedKey::Escape) if down && self.st.profile.is_some() => {
                        let list_open = self.st.profile.as_ref().is_some_and(|p| p.open.is_some());
                        self.st.profile_act(if list_open { crate::profile::Act::Panel } else { crate::profile::Act::Close });
                    }
                    Key::Character(c) => match (c.to_lowercase().as_str(), down, event.repeat) {
                        ("w", ..) => k.up = down,
                        ("s", ..) => k.down = down,
                        ("a", ..) => k.left = down,
                        ("d", ..) => k.right = down,
                        ("r", true, false) => self.st.respawn(),
                        ("m", true, false) => self.st.toggle_mic(),
                        ("c", true, false) => self.st.toggle_cam(),
                        ("b", true, false) => {
                            let a = !self.st.available;
                            self.st.set_available(a);
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
        let redraw = self.st.update(dt, now);
        self.flush();
        if redraw {
            if let Some(w) = &self.window {
                w.request_redraw();
            }
        }
        // tick at ~60 Hz only while something moves; otherwise sleep until the next event
        el.set_control_flow(if self.st.is_active() {
            ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(16))
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
    let opt = |n: &str| args.iter().position(|a| a == n).and_then(|i| args.get(i + 1)).cloned();
    let dry = args.iter().any(|a| a == "--dry-run");
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().map_err(|e| e.to_string())?;
    let proxy: EventLoopProxy<UserEvent> = event_loop.create_proxy();
    let path = opt("--session").unwrap_or_else(default_session);
    let emit: crate::backend::Emit = {
        let p = proxy.clone();
        Arc::new(move |e| drop(p.send_event(e)))
    };
    let mut app = App {
        st: State::new("", dry),
        be: Backend::new(emit, dry, path.clone()),
        window: None,
        surface: None,
        text: Text::new(),
        start: Instant::now(),
        last: Instant::now(),
        frames: 0,
        fps: 0,
        fps_t: Instant::now(),
        cursor: (0.0, 0.0),
        hand: false,
        mods: ModifiersState::default(),
        exit_after: opt("--exit-after").and_then(|s| s.parse().ok()),
        dry,
        logout_after: opt("--logout-after").and_then(|s| s.parse().ok()),
    };
    app.st.talk_key = crate::talk::key_label();
    let p = proxy.clone();
    crate::talk::listen(move |a| drop(p.send_event(UserEvent::Talk(a))));
    let p = proxy.clone();
    crate::talk::portal(move |a| drop(p.send_event(UserEvent::Talk(a))), app.st.talk_key.clone());
    if args.iter().any(|a| a == "--show-login") {
        app.st.login = Some(LoginPhase::Idle); // layout check without touching any saved session
    } else {
        match Auth::load(&path) {
            Ok(auth) => app.start_session(auth),
            Err(e) => {
                // no saved sign-in (or it no longer works): show the sign-in screen
                app.st.login = Some(if std::path::Path::new(&path).exists() { LoginPhase::Failed(format!("Your saved sign-in no longer works ({e}).")) } else { LoginPhase::Idle });
            }
        }
    }
    event_loop.run_app(&mut app).map_err(|e| e.to_string())
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
