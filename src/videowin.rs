//! A bare window that only shows call video: no text, no buttons, no GPU. The terminal client opens it where the
//! terminal cannot draw pictures itself. It lives on its own thread and closes when the call ends or the user closes it.
//! A screen share fills the window with the cameras in a strip underneath; with no share the cameras form a grid.
use crate::app::Frame;
use std::num::NonZeroU32;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};
use winit::window::{Window, WindowId};

enum Msg {
    Open(Arc<AtomicBool>),
    Frame { key: String, name: String, w: u32, h: u32, rgba: Vec<u8> },
    Gone(String),
    /// Close the window that belongs to this flag.
    Close(Arc<AtomicBool>),
}

/// winit allows one event loop per process, ever: opening a second one after the first ended fails. So a single loop
/// thread starts with the first window and lives on; closing a window only drops the window, and the next one is
/// created inside the same loop.
static LOOP: OnceLock<Option<EventLoopProxy<Msg>>> = OnceLock::new();

fn event_loop() -> Option<&'static EventLoopProxy<Msg>> {
    LOOP.get_or_init(|| {
        let (ready_tx, ready_rx) = std::sync::mpsc::channel();
        std::thread::Builder::new()
            .name("video-window".into())
            .spawn(move || {
                let mut b = EventLoop::<Msg>::with_user_event();
                winit::platform::x11::EventLoopBuilderExtX11::with_any_thread(&mut b, true);
                winit::platform::wayland::EventLoopBuilderExtWayland::with_any_thread(&mut b, true);
                let el = match b.build() {
                    Ok(el) => el,
                    Err(e) => {
                        eprintln!("video window: {e}");
                        return drop(ready_tx.send(None));
                    }
                };
                let _ = ready_tx.send(Some(el.create_proxy()));
                let mut app = Win { tiles: vec![], window: None, surface: None, alive: None };
                let _ = el.run_app(&mut app);
            })
            .ok()?;
        ready_rx.recv_timeout(std::time::Duration::from_secs(3)).ok().flatten()
    })
    .as_ref()
}

pub struct VideoWindow {
    tx: EventLoopProxy<Msg>,
    alive: Arc<AtomicBool>,
}

impl VideoWindow {
    pub fn alive(&self) -> bool {
        self.alive.load(Ordering::Relaxed)
    }

    pub fn frame(&self, key: &str, name: &str, f: &Frame) {
        if self.alive() && self.tx.send_event(Msg::Frame { key: key.into(), name: name.into(), w: f.w, h: f.h, rgba: f.rgba.clone() }).is_err() {
            self.alive.store(false, Ordering::Relaxed);
        }
    }

    pub fn gone(&self, key: &str) {
        let _ = self.tx.send_event(Msg::Gone(key.into()));
    }
}

impl Drop for VideoWindow {
    fn drop(&mut self) {
        let _ = self.tx.send_event(Msg::Close(self.alive.clone()));
    }
}

/// Open a video window. None when there is no display to open it on.
pub fn open() -> Option<VideoWindow> {
    if std::env::var_os("DISPLAY").is_none() && std::env::var_os("WAYLAND_DISPLAY").is_none() {
        return None;
    }
    let tx = event_loop()?.clone();
    let alive = Arc::new(AtomicBool::new(true));
    tx.send_event(Msg::Open(alive.clone())).ok()?;
    Some(VideoWindow { tx, alive })
}

struct Win {
    tiles: Vec<(String, String, Frame)>,
    window: Option<Arc<Window>>,
    surface: Option<softbuffer::Surface<Arc<Window>, Arc<Window>>>,
    /// The flag of the handle whose window this is; cleared when the window goes away.
    alive: Option<Arc<AtomicBool>>,
}

impl Win {
    fn title(&self) -> String {
        let names: Vec<String> = self.tiles.iter().map(|(k, n, _)| if k.ends_with(":screen") { format!("{n}'s screen") } else if k == "me" { "you".to_string() } else { n.clone() }).collect();
        format!("SoWorker · {}", names.join(", "))
    }

    fn close(&mut self) {
        self.surface = None;
        self.window = None;
        self.tiles.clear();
        if let Some(a) = self.alive.take() {
            a.store(false, Ordering::Relaxed);
        }
    }

    fn draw(&mut self) {
        let (Some(win), Some(surface)) = (&self.window, &mut self.surface) else { return };
        let size = win.inner_size();
        let (Some(w), Some(h)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) else { return };
        let _ = surface.resize(w, h);
        let Ok(mut buf) = surface.buffer_mut() else { return };
        buf.fill(0x0014161b);
        let slots = layout(&self.tiles, w.get(), h.get());
        for (i, r) in slots {
            blit(&mut buf, w.get(), h.get(), &self.tiles[i].2, r, self.tiles[i].0 == "me");
        }
        let _ = buf.present();
    }
}

/// Which tile goes where: (index, x, y, w, h).
fn layout(tiles: &[(String, String, Frame)], w: u32, h: u32) -> Vec<(usize, (u32, u32, u32, u32))> {
    let n = tiles.len() as u32;
    if n == 0 {
        return vec![];
    }
    let screen = tiles.iter().position(|(k, ..)| k.ends_with(":screen") && !k.starts_with("me:")).or_else(|| tiles.iter().position(|(k, ..)| k.ends_with(":screen")));
    let pad = 4;
    if let Some(s) = screen {
        let others: Vec<usize> = (0..tiles.len()).filter(|i| *i != s).collect();
        if others.is_empty() {
            return vec![(s, (0, 0, w, h))];
        }
        let strip = h / 5;
        let mut v = vec![(s, (0, 0, w, h - strip - pad))];
        let cw = w / others.len() as u32;
        for (j, i) in others.into_iter().enumerate() {
            v.push((i, (j as u32 * cw + pad / 2, h - strip, cw.saturating_sub(pad), strip)));
        }
        return v;
    }
    let cols = (n as f32).sqrt().ceil() as u32;
    let rows = n.div_ceil(cols);
    let (cw, ch) = (w / cols, h / rows);
    (0..tiles.len()).map(|i| (i, ((i as u32 % cols) * cw + pad / 2, (i as u32 / cols) * ch + pad / 2, cw.saturating_sub(pad), ch.saturating_sub(pad)))).collect()
}

/// Scale a frame into a rectangle (contain, centred, nearest neighbour; `mirror` flips it like a mirror).
fn blit(buf: &mut [u32], bw: u32, bh: u32, f: &Frame, (rx, ry, rw, rh): (u32, u32, u32, u32), mirror: bool) {
    if f.w == 0 || f.h == 0 || rw == 0 || rh == 0 {
        return;
    }
    let s = (rw as f32 / f.w as f32).min(rh as f32 / f.h as f32);
    let (dw, dh) = (((f.w as f32 * s) as u32).max(1), ((f.h as f32 * s) as u32).max(1));
    let (ox, oy) = (rx + (rw - dw.min(rw)) / 2, ry + (rh - dh.min(rh)) / 2);
    for y in 0..dh.min(rh) {
        let py = oy + y;
        if py >= bh {
            break;
        }
        let sy = (y as u64 * f.h as u64 / dh as u64) as usize;
        let row = &f.rgba[sy * f.w as usize * 4..(sy + 1) * f.w as usize * 4];
        for x in 0..dw.min(rw) {
            let px = ox + x;
            if px >= bw {
                break;
            }
            let sx = (x as u64 * f.w as u64 / dw as u64) as usize;
            let sx = if mirror { f.w as usize - 1 - sx } else { sx };
            let p = &row[sx * 4..sx * 4 + 4];
            buf[(py * bw + px) as usize] = (p[0] as u32) << 16 | (p[1] as u32) << 8 | p[2] as u32;
        }
    }
}

impl ApplicationHandler<Msg> for Win {
    fn resumed(&mut self, _el: &ActiveEventLoop) {}

    fn user_event(&mut self, el: &ActiveEventLoop, ev: Msg) {
        match ev {
            Msg::Open(flag) => {
                if self.window.is_some() {
                    // one window at a time: the new handle shares it, the old one is told it is gone
                    if let Some(old) = self.alive.replace(flag) {
                        old.store(false, Ordering::Relaxed);
                    }
                    return;
                }
                let attrs = Window::default_attributes().with_title("SoWorker").with_inner_size(LogicalSize::new(800.0, 450.0));
                let attrs = winit::platform::wayland::WindowAttributesExtWayland::with_name(attrs, "soworker", "soworker-video");
                let attrs = winit::platform::x11::WindowAttributesExtX11::with_name(attrs, "soworker", "soworker-video");
                match el.create_window(attrs) {
                    Ok(window) => {
                        let window = Arc::new(window);
                        if let Ok(ctx) = softbuffer::Context::new(window.clone()) {
                            self.surface = softbuffer::Surface::new(&ctx, window.clone()).ok();
                        }
                        self.window = Some(window);
                        self.alive = Some(flag);
                    }
                    Err(e) => {
                        eprintln!("video window: {e}");
                        flag.store(false, Ordering::Relaxed);
                    }
                }
            }
            Msg::Frame { key, name, w, h, rgba } => {
                if self.window.is_none() || rgba.len() != (w * h * 4) as usize {
                    return;
                }
                let f = Frame { w, h, rgba };
                match self.tiles.iter_mut().find(|t| t.0 == key) {
                    Some(t) => (t.1, t.2) = (name, f),
                    None => self.tiles.push((key, name, f)),
                }
                if let Some(win) = &self.window {
                    win.set_title(&self.title());
                    win.request_redraw();
                }
            }
            Msg::Gone(key) => {
                self.tiles.retain(|t| t.0 != key);
                if self.tiles.is_empty() {
                    return self.close();
                }
                if let Some(win) = &self.window {
                    win.set_title(&self.title());
                    win.request_redraw();
                }
            }
            Msg::Close(flag) => {
                if self.alive.as_ref().is_some_and(|a| Arc::ptr_eq(a, &flag)) {
                    self.close();
                }
            }
        }
    }

    fn window_event(&mut self, _el: &ActiveEventLoop, _id: WindowId, ev: WindowEvent) {
        match ev {
            WindowEvent::CloseRequested => self.close(),
            WindowEvent::RedrawRequested => self.draw(),
            WindowEvent::Resized(_) => {
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tile(key: &str) -> (String, String, Frame) {
        (key.into(), key.into(), Frame { w: 4, h: 2, rgba: vec![255; 32] })
    }

    #[test]
    fn a_share_fills_the_window_with_cameras_underneath() {
        let tiles = vec![tile("a"), tile("b:screen"), tile("me")];
        let l = layout(&tiles, 1000, 500);
        assert_eq!(l[0].0, 1, "the share comes first");
        assert_eq!(l.len(), 3);
        assert!(l[1].1 .1 > 350, "cameras sit in the bottom strip");
    }

    #[test]
    fn cameras_without_a_share_form_a_grid() {
        let tiles = vec![tile("a"), tile("b"), tile("c")];
        let l = layout(&tiles, 1000, 500);
        assert_eq!(l.len(), 3);
        assert!(l[2].1 .1 > 200, "third one wraps to the second row");
    }

    #[test]
    fn blit_stays_inside_the_buffer_and_mirrors() {
        let f = Frame { w: 2, h: 1, rgba: vec![255, 0, 0, 255, 0, 0, 255, 255] };
        let mut buf = vec![0u32; 4 * 2];
        blit(&mut buf, 4, 2, &f, (0, 0, 4, 2), false);
        assert_eq!(buf[0], 0xff0000);
        assert_eq!(buf[3], 0x0000ff);
        let mut m = vec![0u32; 4 * 2];
        blit(&mut m, 4, 2, &f, (0, 0, 4, 2), true);
        assert_eq!(m[0], 0x0000ff);
    }
}
