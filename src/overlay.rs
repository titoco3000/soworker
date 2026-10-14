//! A small always-on-top pill ("● Live · 2 listening") for when the mic is open and the window is out of sight.
//! A normal Wayland window cannot stay on top or pick its place, so this uses the compositor's layer-shell protocol
//! (COSMIC, sway, Hyprland, KDE...; not GNOME): a surface on the overlay layer in the top-right corner that ignores
//! the mouse and never takes the keyboard. Where the protocol is missing we fall back to desktop notifications.
use std::sync::mpsc::{channel, Sender};

#[derive(Clone, Debug, PartialEq)]
pub enum Overlay {
    /// The mic is open; this many other people are in the meeting.
    Live { listeners: usize },
    /// The mic just closed (shown briefly, then it disappears).
    Muted,
    Hide,
}

impl Overlay {
    pub fn text(&self) -> String {
        match self {
            Overlay::Live { listeners: 0 } => "Live · nobody else is here".into(),
            Overlay::Live { listeners: 1 } => "Live · 1 listening".into(),
            Overlay::Live { listeners } => format!("Live · {listeners} listening"),
            Overlay::Muted => "Muted".into(),
            Overlay::Hide => String::new(),
        }
    }
}

/// How the overlay is shown on this system.
pub struct Handle {
    tx: Option<Sender<Overlay>>,
}

impl Handle {
    pub fn new() -> Self {
        Handle { tx: start() }
    }

    pub fn is_layer(&self) -> bool {
        self.tx.is_some()
    }

    /// Show or update the overlay. Without layer-shell the same information goes out as a short notification.
    pub fn show(&self, o: Overlay) {
        match &self.tx {
            Some(tx) => drop(tx.send(o)),
            None => {
                if o != Overlay::Hide {
                    crate::notify::status(&o.text());
                }
            }
        }
    }
}

#[cfg(feature = "overlay")]
fn start() -> Option<Sender<Overlay>> {
    let (tx, rx) = channel();
    let (ready_tx, ready_rx) = channel();
    std::thread::Builder::new()
        .name("overlay".into())
        .spawn(move || {
            if let Err(e) = layer::run(rx, ready_tx.clone()) {
                eprintln!("overlay: {e} (using notifications instead)");
                let _ = ready_tx.send(false);
            }
        })
        .ok()?;
    ready_rx.recv_timeout(std::time::Duration::from_secs(3)).unwrap_or(false).then_some(tx)
}

#[cfg(not(feature = "overlay"))]
fn start() -> Option<Sender<Overlay>> {
    let _ = channel::<Overlay>();
    None
}

/// Pixels for the pill: premultiplied RGBA, `W` x `H`.
#[cfg(feature = "overlay")]
pub const W: u32 = 300;
#[cfg(feature = "overlay")]
pub const H: u32 = 52;

#[cfg(feature = "overlay")]
pub fn render(o: &Overlay, text: &mut crate::gui::render::Text) -> tiny_skia::Pixmap {
    use crate::gui::render::paint;
    use crate::gui::theme::*;
    use tiny_skia::{FillRule, PathBuilder, Pixmap, Transform};
    let mut pm = Pixmap::new(W, H).expect("overlay pixmap");
    let id = Transform::identity();
    if let Some(p) = crate::gui::icon::rounded_rect(1.0, 1.0, W as f32 - 2.0, H as f32 - 2.0, H as f32 / 2.0 - 1.0) {
        pm.fill_path(&p, &paint(SURFACE[0], SURFACE[1], SURFACE[2], 240), FillRule::Winding, id, None);
        pm.stroke_path(&p, &paint(LINE[0], LINE[1], LINE[2], 255), &tiny_skia::Stroke { width: 1.0, ..Default::default() }, id, None);
    }
    let dot = if matches!(o, Overlay::Live { .. }) { DANGER } else { INK_FAINT };
    if let Some(c) = PathBuilder::from_circle(26.0, H as f32 / 2.0, 7.0) {
        pm.fill_path(&c, &paint(dot[0], dot[1], dot[2], 255), FillRule::Winding, id, None);
    }
    let label = match o {
        Overlay::Live { .. } => "Mic on",
        _ => "",
    };
    if o == &Overlay::Muted {
        text.draw(&mut pm.as_mut(), "Muted", 46.0, H as f32 / 2.0 + 6.0, 17.0, INK, 0.0);
    } else {
        text.draw(&mut pm.as_mut(), label, 46.0, 22.0, 14.0, INK, 0.0);
        text.draw(&mut pm.as_mut(), &o.text().replacen("Live · ", "", 1), 46.0, 40.0, 12.0, INK_MUTED, 0.0);
    }
    pm
}

#[cfg(feature = "overlay")]
mod layer {
    use super::{render, Overlay, H, W};
    use smithay_client_toolkit::{
        compositor::{CompositorHandler, CompositorState, Region},
        delegate_compositor, delegate_layer, delegate_output, delegate_registry, delegate_shm,
        output::{OutputHandler, OutputState},
        registry::{ProvidesRegistryState, RegistryState},
        registry_handlers,
        shell::{
            wlr_layer::{Anchor, KeyboardInteractivity, Layer, LayerShell, LayerShellHandler, LayerSurface, LayerSurfaceConfigure},
            WaylandSurface,
        },
        shm::{slot::SlotPool, Shm, ShmHandler},
    };
    use std::sync::mpsc::{Receiver, Sender};
    use std::time::{Duration, Instant};
    use wayland_client::{
        globals::registry_queue_init,
        protocol::{wl_output, wl_shm, wl_surface},
        Connection, QueueHandle,
    };

    struct App {
        registry: RegistryState,
        outputs: OutputState,
        shm: Shm,
        compositor: CompositorState,
        layer_shell: LayerShell,
        pool: SlotPool,
        layer: Option<LayerSurface>,
        configured: bool,
        current: Overlay,
        text: crate::gui::render::Text,
        dirty: bool,
    }

    impl App {
        fn open(&mut self, qh: &QueueHandle<Self>) {
            if self.layer.is_some() {
                return;
            }
            let surface = self.compositor.create_surface(qh);
            let layer = self.layer_shell.create_layer_surface(qh, surface, Layer::Overlay, Some("soworker"), None);
            layer.set_anchor(Anchor::TOP | Anchor::RIGHT);
            layer.set_margin(16, 16, 0, 0);
            layer.set_size(W, H);
            layer.set_exclusive_zone(-1); // never push other windows around
            layer.set_keyboard_interactivity(KeyboardInteractivity::None);
            if let Ok(empty) = Region::new(&self.compositor) {
                layer.wl_surface().set_input_region(Some(empty.wl_region())); // clicks go to what is underneath
            }
            layer.commit();
            self.layer = Some(layer);
            self.configured = false;
        }

        fn close(&mut self) {
            self.layer = None; // dropping the surface removes it
            self.configured = false;
        }

        fn draw(&mut self, qh: &QueueHandle<Self>) {
            let Some(layer) = &self.layer else { return };
            if !self.configured {
                return;
            }
            let pm = render(&self.current, &mut self.text);
            let stride = W as i32 * 4;
            let Ok((buffer, canvas)) = self.pool.create_buffer(W as i32, H as i32, stride, wl_shm::Format::Argb8888) else { return };
            for (dst, src) in canvas.chunks_exact_mut(4).zip(pm.data().chunks_exact(4)) {
                dst.copy_from_slice(&[src[2], src[1], src[0], src[3]]); // RGBA -> the compositor's BGRA byte order
            }
            let surface = layer.wl_surface();
            surface.damage_buffer(0, 0, W as i32, H as i32);
            surface.frame(qh, surface.clone());
            if buffer.attach_to(surface).is_ok() {
                layer.commit();
            }
            self.dirty = false;
        }
    }

    pub fn run(rx: Receiver<Overlay>, ready: Sender<bool>) -> Result<(), String> {
        let conn = Connection::connect_to_env().map_err(|e| e.to_string())?;
        let (globals, mut queue) = registry_queue_init::<App>(&conn).map_err(|e| e.to_string())?;
        let qh = queue.handle();
        let compositor = CompositorState::bind(&globals, &qh).map_err(|e| e.to_string())?;
        let layer_shell = LayerShell::bind(&globals, &qh).map_err(|_| "the compositor has no layer-shell".to_string())?;
        let shm = Shm::bind(&globals, &qh).map_err(|e| e.to_string())?;
        let pool = SlotPool::new((W * H * 4) as usize, &shm).map_err(|e| e.to_string())?;
        let mut app = App {
            registry: RegistryState::new(&globals),
            outputs: OutputState::new(&globals, &qh),
            shm,
            compositor,
            layer_shell,
            pool,
            layer: None,
            configured: false,
            current: Overlay::Hide,
            text: crate::gui::render::Text::new(),
            dirty: false,
        };
        let _ = ready.send(true);
        let mut hide_at: Option<Instant> = None;
        loop {
            // commands from the app: the newest wins
            loop {
                match rx.recv_timeout(Duration::from_millis(40)) {
                    Ok(o) => {
                        hide_at = (o == Overlay::Muted).then(|| Instant::now() + Duration::from_millis(1800));
                        if o == Overlay::Hide {
                            app.close();
                        } else {
                            app.current = o;
                            app.dirty = true;
                            app.open(&qh);
                        }
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => break,
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => return Ok(()),
                }
            }
            if hide_at.is_some_and(|t| Instant::now() >= t) {
                hide_at = None;
                app.close();
            }
            queue.roundtrip(&mut app).map_err(|e| e.to_string())?;
            if app.dirty {
                app.draw(&qh);
                queue.flush().map_err(|e| e.to_string())?;
            }
        }
    }

    impl CompositorHandler for App {
        fn scale_factor_changed(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &wl_surface::WlSurface, _: i32) {}
        fn transform_changed(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &wl_surface::WlSurface, _: wl_output::Transform) {}
        fn frame(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &wl_surface::WlSurface, _: u32) {}
        fn surface_enter(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &wl_surface::WlSurface, _: &wl_output::WlOutput) {}
        fn surface_leave(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &wl_surface::WlSurface, _: &wl_output::WlOutput) {}
    }

    impl OutputHandler for App {
        fn output_state(&mut self) -> &mut OutputState {
            &mut self.outputs
        }
        fn new_output(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_output::WlOutput) {}
        fn update_output(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_output::WlOutput) {}
        fn output_destroyed(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_output::WlOutput) {}
    }

    impl LayerShellHandler for App {
        fn closed(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &LayerSurface) {
            self.layer = None;
            self.configured = false;
        }
        fn configure(&mut self, _: &Connection, qh: &QueueHandle<Self>, _: &LayerSurface, _: LayerSurfaceConfigure, _: u32) {
            self.configured = true;
            self.draw(qh);
        }
    }

    impl ShmHandler for App {
        fn shm_state(&mut self) -> &mut Shm {
            &mut self.shm
        }
    }

    delegate_compositor!(App);
    delegate_output!(App);
    delegate_shm!(App);
    delegate_layer!(App);
    delegate_registry!(App);

    impl ProvidesRegistryState for App {
        fn registry(&mut self) -> &mut RegistryState {
            &mut self.registry
        }
        registry_handlers![OutputState];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texts_say_who_is_listening() {
        assert_eq!(Overlay::Live { listeners: 0 }.text(), "Live · nobody else is here");
        assert_eq!(Overlay::Live { listeners: 1 }.text(), "Live · 1 listening");
        assert_eq!(Overlay::Live { listeners: 3 }.text(), "Live · 3 listening");
    }

    #[cfg(feature = "overlay")]
    #[test]
    fn the_pill_draws_something_opaque_in_the_middle_and_clear_in_the_corner() {
        let mut t = crate::gui::render::Text::new();
        let pm = render(&Overlay::Live { listeners: 2 }, &mut t);
        let a = |x: u32, y: u32| pm.data()[((y * W + x) * 4 + 3) as usize];
        assert_eq!(a(0, 0), 0, "rounded corner is transparent");
        assert!(a(W / 2, H / 2) > 200, "body is filled");
    }
}
