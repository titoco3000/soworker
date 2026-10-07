//! The window: winit for events, softbuffer to present the tiny-skia pixmap. No GPU context.
use crate::app::{LoginAct, LoginPhase, State};
use crate::login;
use crate::net;
use crate::calls::{MediaCmd, MediaEvent, MediaTx};
use crate::proto::{Cmd, Net};
use crate::meeting_ui::{self, Hit};
use crate::render::{self, Text};
use crate::{chatui, sidebar};
use crate::session::{self, Auth, SessionFile};
use std::sync::atomic::{AtomicBool, Ordering};
use std::num::NonZeroU32;
use std::sync::mpsc::{channel, Sender};
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
    let img = image::load_from_memory(include_bytes!("../assets/icon.webp")).ok()?.to_rgba8();
    winit::window::Icon::from_rgba(img.to_vec(), img.width(), img.height()).ok()
}

pub enum UserEvent {
    Net(Net),
    Media(MediaEvent),
    /// The browser sign-in finished: a ready session, or why it did not work.
    Login(Result<Auth, String>),
}

struct App {
    st: State,
    cmds: Sender<Cmd>,
    media: Option<MediaTx>,
    avatars: Option<std::sync::mpsc::Sender<(String, String)>>,
    chat: Option<std::sync::mpsc::Sender<crate::chat::ChatCmd>>,
    window: Option<Arc<Window>>,
    surface: Option<softbuffer::Surface<Arc<Window>, Arc<Window>>>,
    pixmap: Option<tiny_skia::Pixmap>,
    text: Text,
    start: Instant,
    last: Instant,
    frames: u32,
    fps: u32,
    fps_t: Instant,
    cursor: (f32, f32),
    mods: ModifiersState,
    exit_after: Option<f64>,
    /// Where the saved sign-in lives (live mode only); deleted on logout.
    session_path: String,
    proxy: EventLoopProxy<UserEvent>,
    dry: bool,
    demo: bool,
    login_cancel: Option<Arc<AtomicBool>>,
    logout_started: Option<Instant>,
    /// Test hook for the offline demo: behave as if the user confirmed the logout dialog after N seconds.
    logout_after: Option<f64>,
}

impl App {
    fn now(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    fn flush(&mut self) {
        match self.st.login_act.take() {
            Some(LoginAct::Start) => self.begin_login(),
            Some(LoginAct::Cancel) => {
                if let Some(c) = &self.login_cancel {
                    c.store(true, Ordering::Relaxed);
                }
            }
            None => {}
        }
        if self.st.logout_requested && self.logout_started.is_none() {
            // leave the call, close the socket cleanly, then forget the session (see finish_logout)
            self.logout_started = Some(Instant::now());
            if let Some(tx) = &self.media {
                let _ = tx.send(MediaCmd::Leave);
            }
            let _ = self.cmds.send(Cmd::Logout);
        }
        for c in self.st.outbox.drain(..) {
            let _ = self.cmds.send(c);
        }
        for snd in self.st.sounds.drain(..) {
            if !self.demo {
                crate::sound::play(snd);
            }
        }
        for c in self.st.chat_out.drain(..) {
            if let Some(tx) = &self.chat {
                let _ = tx.send(c);
            }
        }
        for req in self.st.avatar_req.drain(..) {
            if let Some(tx) = &self.avatars {
                let _ = tx.send(req);
            }
        }
        for m in self.st.media_out.drain(..) {
            eprintln!("[{:6.1}] media command: {}", self.start.elapsed().as_secs_f64(), match &m {
                MediaCmd::Join { url, room_id, .. } => format!("join {url} room {room_id}{}", if self.media.is_none() { "  (media disabled: dry run / demo)" } else { "" }),
                other => format!("{other:?}"),
            });
            if let Some(tx) = &self.media {
                let _ = tx.send(m);
            }
        }
    }

    /// Connect everything for a signed-in user: gaia socket, calls, avatar loader.
    fn start_session(&mut self, auth: Auth) {
        let me = auth.uid.clone();
        self.session_path = auth.path.clone();
        let (tx, rx) = channel();
        self.cmds = tx;
        let (p, dry) = (self.proxy.clone(), self.dry);
        let emit: crate::chat::Emit = Arc::new(move |e| drop(p.send_event(UserEvent::Net(e))));
        if !dry {
            let (ctx, crx) = channel();
            self.chat = Some(ctx);
            crate::chat::spawn(auth.clone(), emit.clone(), crx);
        }
        std::thread::spawn(move || net::run(auth, dry, move |e| emit(e), rx));
        #[cfg(feature = "calls")]
        if !self.dry {
            let p = self.proxy.clone();
            self.media = Some(crate::media::spawn(move |e| drop(p.send_event(UserEvent::Media(e)))));
        }
        if self.avatars.is_none() {
            let p = self.proxy.clone();
            self.avatars = Some(crate::avatars::spawn(move |e| drop(p.send_event(UserEvent::Net(e)))));
        }
        let ui = self.st.ui;
        self.st = State::new(&me, self.dry);
        self.st.ui = ui;
        self.logout_started = None;
        self.login_cancel = None;
    }

    /// Open the browser sign-in on its own thread; the outcome comes back as `UserEvent::Login`.
    fn begin_login(&mut self) {
        let cancel = Arc::new(AtomicBool::new(false));
        self.login_cancel = Some(cancel.clone());
        self.st.login = Some(LoginPhase::Waiting);
        let (path, p) = (self.session_path.clone(), self.proxy.clone());
        std::thread::spawn(move || {
            let known = session::load_space(&path);
            let opts = login::Options { start_url: login::login_url(&known), origin: "https://app.sowork.com".into(), known_space: known, headless: false };
            let res = login::capture(&opts, &cancel).and_then(|c| {
                let find = serde_json::json!({"type": "group", "groupId": c.space.0, "alias": c.space.1, "redirect": true});
                session::save_session(&path, &SessionFile { api_key: c.api_key, refresh_token: c.refresh_token, find })?;
                session::save_space(&path, &c.space.0, &c.space.1);
                Auth::load(&path)
            });
            let _ = p.send_event(UserEvent::Login(res));
        });
    }

    /// Forget the saved sign-in and go back to the sign-in screen (the offline demo just quits). Called when the socket thread confirms it closed, or after a timeout.
    fn finish_logout(&mut self, el: &ActiveEventLoop) {
        if self.media.is_some() {
            std::thread::sleep(Duration::from_millis(300)); // let the LiveKit leave message go out
        }
        if self.demo {
            println!("logged out (offline demo: nothing to forget)");
            return el.exit();
        }
        let p = self.session_path.clone();
        match std::fs::remove_file(&p) {
            Ok(()) => println!("logged out: removed {p}"),
            Err(e) => eprintln!("logged out, but could not remove {p}: {e}"),
        }
        self.media = None; // ends the media thread
        self.chat = None; // ends the chat threads
        self.cmds = channel().0;
        let ui = self.st.ui;
        self.st = State::new("", self.dry);
        self.st.ui = ui;
        self.st.login = Some(LoginPhase::Idle);
        self.logout_started = None;
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }

    fn draw(&mut self) {
        let (Some(win), Some(surface)) = (&self.window, &mut self.surface) else { return };
        let size = win.inner_size();
        let (Some(w), Some(h)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) else { return };
        if self.pixmap.as_ref().map_or(true, |p| p.width() != w.get() || p.height() != h.get()) {
            self.pixmap = tiny_skia::Pixmap::new(w.get(), h.get());
            let _ = surface.resize(w, h);
        }
        let Some(pm) = &mut self.pixmap else { return };
        render::draw(&self.st, pm, &mut self.text);
        if let Ok(mut buf) = surface.buffer_mut() {
            for (dst, px) in buf.iter_mut().zip(pm.data().chunks_exact(4)) {
                *dst = u32::from_be_bytes([0, px[0], px[1], px[2]]);
            }
            let _ = buf.present();
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
        let attrs = winit::platform::wayland::WindowAttributesExtWayland::with_name(attrs, "soworker", "soworker");
        let attrs = winit::platform::x11::WindowAttributesExtX11::with_name(attrs, "soworker", "soworker");
        let Ok(window) = el.create_window(attrs) else { return el.exit() };
        let window = Arc::new(window);
        if let Ok(ctx) = softbuffer::Context::new(window.clone()) {
            self.surface = softbuffer::Surface::new(&ctx, window.clone()).ok();
        }
        self.st.ui = window.scale_factor() as f32;
        self.window = Some(window);
    }

    fn user_event(&mut self, el: &ActiveEventLoop, ev: UserEvent) {
        let now = self.now();
        if matches!(ev, UserEvent::Net(Net::LoggedOut)) {
            return self.finish_logout(el);
        }
        match ev {
            UserEvent::Net(e) => self.st.apply(e, now),
            UserEvent::Media(e) => self.st.apply_media(e, now),
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
                let k = &mut self.st.keys;
                match &event.logical_key {
                    Key::Named(NamedKey::ArrowUp) => k.up = down,
                    Key::Named(NamedKey::ArrowDown) => k.down = down,
                    Key::Named(NamedKey::ArrowLeft) => k.left = down,
                    Key::Named(NamedKey::ArrowRight) => k.right = down,
                    Key::Named(NamedKey::Escape) if down && self.st.confirm_logout => self.st.logout_choice(false),
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
            WindowEvent::Focused(false) => self.st.keys = Default::default(),
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
        if self.logout_after.is_some_and(|t| now > t) && !self.st.logout_requested {
            self.st.logout_choice(true);
        }
        if self.logout_started.is_some_and(|t| t.elapsed() > Duration::from_millis(2500)) {
            return self.finish_logout(el); // the socket thread did not answer in time (offline?): leave anyway
        }
        if self.st.logout_requested && self.demo && self.logout_started.is_some() {
            return self.finish_logout(el); // offline demo: there is no socket to wait for
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
        } else if self.logout_started.is_some() {
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
    if !args.iter().any(|a| a == "--demo") {
        crate::desktop::ensure();
    }
    let opt = |n: &str| args.iter().position(|a| a == n).and_then(|i| args.get(i + 1)).cloned();
    let demo = opt("--demo");
    let dry = demo.is_some() || args.iter().any(|a| a == "--dry-run");
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().map_err(|e| e.to_string())?;
    let proxy: EventLoopProxy<UserEvent> = event_loop.create_proxy();
    let path = opt("--session").unwrap_or_else(default_session);
    let mut app = App {
        st: State::new("", dry),
        cmds: channel().0, // replaced when a session starts
        media: None,
        avatars: None,
        chat: None,
        window: None,
        surface: None,
        pixmap: None,
        text: Text::new(),
        start: Instant::now(),
        last: Instant::now(),
        frames: 0,
        fps: 0,
        fps_t: Instant::now(),
        cursor: (0.0, 0.0),
        mods: ModifiersState::default(),
        exit_after: opt("--exit-after").and_then(|s| s.parse().ok()),
        session_path: path.clone(),
        proxy: proxy.clone(),
        dry,
        demo: demo.is_some(),
        login_cancel: None,
        logout_started: None,
        logout_after: opt("--logout-after").and_then(|s| s.parse().ok()),
    };
    if let Some(run) = &demo {
        // offline: feed a recorded run into the window, no network, nothing is sent
        let me = crate::replay::own_id(run)?;
        let events = crate::replay::load(run)?;
        app.st = State::new(&me, true);
        std::thread::spawn(move || {
            for e in events {
                if proxy.send_event(UserEvent::Net(e)).is_err() {
                    break;
                }
            }
        });
    } else if args.iter().any(|a| a == "--show-login") {
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
    fn the_app_icon_decodes() {
        assert!(super::app_icon().is_some());
    }
}
