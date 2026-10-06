//! The window: winit for events, softbuffer to present the tiny-skia pixmap. No GPU context.
use crate::app::State;
use crate::net;
use crate::calls::{MediaCmd, MediaEvent, MediaTx};
use crate::proto::{Cmd, Net};
use crate::meeting_ui::{self, Hit};
use crate::render::{self, Text};
use crate::session::Auth;
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

pub enum UserEvent {
    Net(Net),
    Media(MediaEvent),
}

struct App {
    st: State,
    cmds: Sender<Cmd>,
    media: Option<MediaTx>,
    avatars: Option<std::sync::mpsc::Sender<(String, String)>>,
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
}

impl App {
    fn now(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    fn flush(&mut self) {
        for c in self.st.outbox.drain(..) {
            let _ = self.cmds.send(c);
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

    fn draw(&mut self) {
        let (Some(win), Some(surface)) = (&self.window, &mut self.surface) else { return };
        let size = win.inner_size();
        let (Some(w), Some(h)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) else { return };
        if self.pixmap.as_ref().map_or(true, |p| p.width() != w.get() || p.height() != h.get()) {
            self.pixmap = tiny_skia::Pixmap::new(w.get(), h.get());
            let _ = surface.resize(w, h);
        }
        let Some(pm) = &mut self.pixmap else { return };
        render::draw(&self.st, pm, &mut self.text, self.fps);
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
        let attrs = Window::default_attributes().with_title("SoWorker").with_inner_size(LogicalSize::new(1000.0, 700.0));
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
        match ev {
            UserEvent::Net(e) => self.st.apply(e, now),
            UserEvent::Media(e) => self.st.apply_media(e, now),
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
                        meeting_ui::wheel(&mut self.st, &mut self.text, c.0, c.1, sz.width as f32, sz.height as f32, dy)
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
                    if let Some(h) = meeting_ui::hit(&self.st, &mut self.text, c.0, c.1, sw, sh) {
                        match h {
                            Hit::Header => meeting_ui::begin_drag(&mut self.st, &mut self.text, c.0, c.1, sw, sh),
                            other => self.st.dock_press(other, now),
                        }
                    } else if let Some(act) = render::hit(&self.st, self.cursor.0, self.cursor.1, s.width as f32, s.height as f32) {
                        self.st.press(act);
                    } else if let Some((wx, wy)) = self.st.to_world(self.cursor.0, self.cursor.1, s.width as f32, s.height as f32) {
                        self.st.click(wx, wy, self.mods.shift_key(), now);
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let down = event.state == ElementState::Pressed;
                let k = &mut self.st.keys;
                match &event.logical_key {
                    Key::Named(NamedKey::ArrowUp) => k.up = down,
                    Key::Named(NamedKey::ArrowDown) => k.down = down,
                    Key::Named(NamedKey::ArrowLeft) => k.left = down,
                    Key::Named(NamedKey::ArrowRight) => k.right = down,
                    Key::Named(NamedKey::F3) if down && !event.repeat => self.st.show_debug = !self.st.show_debug,
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
        } else if let Some(t) = self.exit_after {
            ControlFlow::WaitUntil(self.start + Duration::from_secs_f64(t + 0.05))
        } else {
            ControlFlow::Wait
        });
    }
}

fn default_session() -> String {
    ["native/.session.json", ".session.json", "app/.session.json", "../app/.session.json"]
        .iter()
        .find(|p| std::path::Path::new(p).exists())
        .unwrap_or(&"app/.session.json")
        .to_string()
}

pub fn run(args: &[String]) -> Result<(), String> {
    let opt = |n: &str| args.iter().position(|a| a == n).and_then(|i| args.get(i + 1)).cloned();
    let demo = opt("--demo");
    let dry = demo.is_some() || args.iter().any(|a| a == "--dry-run");
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().map_err(|e| e.to_string())?;
    let proxy: EventLoopProxy<UserEvent> = event_loop.create_proxy();
    let (cmd_tx, cmd_rx) = channel();
    let me_id = if let Some(run) = &demo {
        // offline: feed a recorded run into the window, no network, nothing is sent
        let me = crate::replay::own_id(run)?;
        let events = crate::replay::load(run)?;
        std::thread::spawn(move || {
            let _keep = cmd_rx;
            for e in events {
                if proxy.send_event(UserEvent::Net(e)).is_err() {
                    break;
                }
            }
        });
        me
    } else {
        let auth = Auth::load(&opt("--session").unwrap_or_else(default_session))?;
        let me = auth.uid.clone();
        let p2 = proxy.clone();
        std::thread::spawn(move || net::run(auth, dry, move |e| drop(p2.send_event(UserEvent::Net(e))), cmd_rx));
        me
    };
    #[cfg(feature = "calls")]
    let media = if demo.is_none() && !dry {
        let p = event_loop.create_proxy();
        Some(crate::media::spawn(move |e| drop(p.send_event(UserEvent::Media(e)))))
    } else {
        None
    };
    #[cfg(not(feature = "calls"))]
    let media: Option<MediaTx> = None;
    // pictures are fetched from SoWork's static hosts; the offline demo never touches the network
    let avatars = if demo.is_none() {
        let p = event_loop.create_proxy();
        Some(crate::avatars::spawn(move |e| drop(p.send_event(UserEvent::Net(e)))))
    } else {
        None
    };
    let mut app = App {
        st: State::new(&me_id, dry),
        cmds: cmd_tx,
        media,
        avatars,
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
    };
    event_loop.run_app(&mut app).map_err(|e| e.to_string())
}
