//! Software renderer: draws the office into a tiny-skia pixmap. No GPU, no UI toolkit.
use crate::app::State;
use crate::world::{E, N, S, T, W, ZK};
use std::collections::HashMap;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, PixmapMut, PixmapPaint, Rect, Stroke, StrokeDash, Transform};

/// The logout confirmation dialog: card, Cancel and Log out buttons.
pub fn modal(st: &State, w: f32, h: f32) -> (crate::gui::meeting_ui::R, crate::gui::meeting_ui::R, crate::gui::meeting_ui::R) {
    use crate::gui::meeting_ui::R;
    let u = st.ui;
    let (cw, ch) = (360.0 * u, 150.0 * u);
    let card = R { x: (w - cw) / 2.0, y: (h - ch) / 2.0, w: cw, h: ch };
    let (bw, bh) = (96.0 * u, 32.0 * u);
    let y = card.y + ch - bh - 16.0 * u;
    let cancel = R { x: card.x + cw - 16.0 * u - 2.0 * bw - 8.0 * u, y, w: bw, h: bh };
    let confirm = R { x: card.x + cw - 16.0 * u - bw, y, w: bw, h: bh };
    (card, cancel, confirm)
}

/// `Some(true)` = Log out, `Some(false)` = Cancel (or a click outside), `None` = a click on the card itself.
pub fn modal_hit(st: &State, px: f32, py: f32, w: f32, h: f32) -> Option<bool> {
    let (card, cancel, confirm) = modal(st, w, h);
    if confirm.contains(px, py) {
        Some(true)
    } else if cancel.contains(px, py) || !card.contains(px, py) {
        Some(false)
    } else {
        None
    }
}

/// The font built into the Android app (Liberation Sans, SIL OFL: assets/fonts/LiberationSans-LICENSE).
#[cfg(target_os = "android")]
fn builtin_font() -> Option<Vec<u8>> {
    Some(include_bytes!("../../assets/fonts/LiberationSans-Regular.ttf").to_vec())
}

#[cfg(not(target_os = "android"))]
fn builtin_font() -> Option<Vec<u8>> {
    None
}

pub struct Text {
    font: Option<fontdue::Font>,
    cache: HashMap<(char, u32), (fontdue::Metrics, Vec<u8>)>,
}

impl Text {
    pub fn new() -> Self {
        // Android: one font built in (the system's Roboto is a variable font that differs between vendors)
        #[cfg(target_os = "android")]
        let candidates: Vec<std::path::PathBuf> = vec![];
        #[cfg(not(any(windows, target_os = "android")))]
        let candidates: Vec<std::path::PathBuf> = [
            "/usr/share/fonts/liberation-sans-fonts/LiberationSans-Regular.ttf",
            "/usr/share/fonts/liberation/LiberationSans-Regular.ttf",
            "/usr/share/fonts/dejavu-sans-fonts/DejaVuSans.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/usr/share/fonts/google-droid-sans-fonts/DroidSans.ttf",
            "/usr/share/fonts/TTF/DejaVuSans.ttf",
        ]
        .map(std::path::PathBuf::from)
        .into();
        #[cfg(windows)]
        let candidates: Vec<std::path::PathBuf> = {
            let fonts = std::env::var_os("WINDIR").map_or_else(|| std::path::PathBuf::from(r"C:\Windows"), std::path::PathBuf::from).join("Fonts");
            ["segoeui.ttf", "arial.ttf", "tahoma.ttf"].iter().map(|f| fonts.join(f)).collect()
        };
        let font = candidates
            .iter()
            .find_map(|p| std::fs::read(p).ok())
            .or_else(builtin_font)
            .and_then(|b| fontdue::Font::from_bytes(b, fontdue::FontSettings::default()).ok());
        Text { font, cache: HashMap::new() }
    }

    pub fn width(&mut self, s: &str, px: f32) -> f32 {
        let Some(f) = &self.font else { return 0.0 };
        s.chars().map(|c| f.metrics(c, px).advance_width).sum()
    }

    /// Draw one line; `align` 0 = left, 0.5 = centre, 1 = right of x. `y` is the baseline.
    pub fn draw(&mut self, pm: &mut PixmapMut<'_>, s: &str, x: f32, y: f32, px: f32, rgb: [u8; 3], align: f32) {
        if self.font.is_none() {
            return;
        }
        let mut cx = x - self.width(s, px) * align;
        let (w, h) = (pm.width() as i32, pm.height() as i32);
        let data = pm.data_mut();
        for c in s.chars() {
            let font = self.font.as_ref().unwrap();
            let key = (c, (px * 4.0) as u32);
            let (m, bmp) = self.cache.entry(key).or_insert_with(|| font.rasterize(c, px));
            let (gx0, gy0) = (cx as i32 + m.xmin, y as i32 - m.height as i32 - m.ymin);
            for row in 0..m.height as i32 {
                for col in 0..m.width as i32 {
                    let cov = bmp[(row * m.width as i32 + col) as usize] as u32;
                    let (gx, gy) = (gx0 + col, gy0 + row);
                    if cov == 0 || gx < 0 || gy < 0 || gx >= w || gy >= h {
                        continue;
                    }
                    let i = ((gy * w + gx) * 4) as usize;
                    for k in 0..3 {
                        data[i + k] = ((rgb[k] as u32 * cov + data[i + k] as u32 * (255 - cov)) / 255) as u8;
                    }
                }
            }
            cx += m.advance_width;
        }
    }

    pub fn shadowed(&mut self, pm: &mut PixmapMut<'_>, s: &str, x: f32, y: f32, px: f32, rgb: [u8; 3], align: f32) {
        self.draw(pm, s, x + 1.0, y + 1.0, px, [0, 0, 0], align);
        self.draw(pm, s, x, y, px, rgb, align);
    }
}

/// The transcript notice's lines: one while it is made or once saved, the whole message (how to install whisper.cpp)
/// when it failed, then how to close it.
fn notice_lines(st: &State) -> Vec<String> {
    use crate::transcribe::Status;
    let Some((s, _)) = &st.transcript else { return vec![] };
    match s {
        Status::Failed(e) => {
            let mut v: Vec<String> = e.lines().map(String::from).collect();
            v[0] = format!("No transcript: {}", v[0]);
            v.push(String::new());
            v.push("click to close".into());
            v
        }
        Status::Done(_) => vec![s.line(), "click to close".into()],
        _ => vec![s.line()],
    }
}

/// Where the transcript notice sits (top left of the map, under the toast), if there is one.
pub fn notice_rect(st: &State, text: &mut Text, sw: f32, sh: f32) -> Option<crate::gui::meeting_ui::R> {
    let ui = st.ui;
    let lines = notice_lines(st);
    if lines.is_empty() {
        return None;
    }
    let w = lines.iter().map(|l| text.width(l, 13.0 * ui)).fold(0.0, f32::max) + 24.0 * ui;
    let m = st.main_rect_for(sw, sh);
    Some(crate::gui::meeting_ui::R { x: m.x + 10.0 * ui, y: m.y + 32.0 * ui, w, h: lines.len() as f32 * 18.0 * ui + 16.0 * ui })
}

fn draw_notice(st: &State, pm: &mut PixmapMut<'_>, text: &mut Text, r: crate::gui::meeting_ui::R) {
    use crate::gui::theme;
    let ui = st.ui;
    let failed = matches!(st.transcript, Some((crate::transcribe::Status::Failed(_), _)));
    if let Some(p) = crate::gui::icon::rounded_rect(r.x, r.y, r.w, r.h, 8.0 * ui) {
        pm.fill_path(&p, &paint(theme::SURFACE[0], theme::SURFACE[1], theme::SURFACE[2], 245), FillRule::Winding, Transform::identity(), None);
        let edge = if failed { theme::DANGER } else { theme::LINE };
        pm.stroke_path(&p, &paint(edge[0], edge[1], edge[2], 255), &Stroke { width: 1.0, ..Stroke::default() }, Transform::identity(), None);
    }
    let lines = notice_lines(st);
    let n = lines.len();
    for (i, l) in lines.iter().enumerate() {
        let col = if i == 0 { theme::INK } else if i + 1 == n && n > 1 { theme::INK_FAINT } else { theme::INK_MUTED };
        text.draw(pm, l, r.x + 12.0 * ui, r.y + 8.0 * ui + (i as f32 + 1.0) * 18.0 * ui - 4.0 * ui, 13.0 * ui, col, 0.0);
    }
}

pub fn paint(r: u8, g: u8, b: u8, a: u8) -> Paint<'static> {
    let mut p = Paint::default();
    p.set_color_rgba8(r, g, b, a);
    p.anti_alias = true;
    p
}

pub fn hue_rgb(id: &str) -> (u8, u8, u8) {
    let mut h: u32 = 0;
    for c in id.chars() {
        h = (h.wrapping_mul(31) + c as u32) % 360;
    }
    let (s, l) = (0.6f32, 0.55f32);
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = h as f32 / 60.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r, g, b) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    (((r + m) * 255.0) as u8, ((g + m) * 255.0) as u8, ((b + m) * 255.0) as u8)
}

/// (working set, private) memory of this process in bytes.
#[cfg(windows)]
pub fn proc_mem() -> (u64, u64) {
    use windows_sys::Win32::System::ProcessStatus::{K32GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS, PROCESS_MEMORY_COUNTERS_EX};
    use windows_sys::Win32::System::Threading::GetCurrentProcess;
    // SAFETY: the struct is plain data and its size is passed along.
    unsafe {
        let mut c: PROCESS_MEMORY_COUNTERS_EX = std::mem::zeroed();
        let size = std::mem::size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32;
        c.cb = size;
        if K32GetProcessMemoryInfo(GetCurrentProcess(), (&mut c as *mut PROCESS_MEMORY_COUNTERS_EX).cast::<PROCESS_MEMORY_COUNTERS>(), size) == 0 {
            return (0, 0);
        }
        (c.WorkingSetSize as u64, c.PrivateUsage as u64)
    }
}

/// (resident, proportional) memory of this process in bytes, from /proc.
#[cfg(not(windows))]
pub fn proc_mem() -> (u64, u64) {
    let rss = std::fs::read_to_string("/proc/self/statm")
        .ok()
        .and_then(|s| s.split_whitespace().nth(1).and_then(|v| v.parse::<u64>().ok()))
        .unwrap_or(0)
        * 4096;
    let pss = std::fs::read_to_string("/proc/self/smaps_rollup")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("Pss:"))
                .and_then(|l| l.split_whitespace().nth(1)?.parse::<u64>().ok())
        })
        .unwrap_or(0)
        * 1024;
    (rss, pss)
}

/// The sign-in screen: card, and its main button (Start / Cancel / Try again).
pub fn login_layout(st: &State, w: f32, h: f32) -> (crate::gui::meeting_ui::R, crate::gui::meeting_ui::R) {
    use crate::gui::meeting_ui::R;
    let u = st.ui;
    let cw = (440.0 * u).min(w - 24.0 * u);
    let ch = if !cfg!(target_os = "android") {
        250.0 * u
    } else if st.login == Some(crate::app::LoginPhase::Scanning) {
        (h - 24.0 * u).min(460.0 * u) // room for the camera picture
    } else {
        296.0 * u // and the scan button
    };
    let card = R { x: (w - cw) / 2.0, y: ((h - ch) / 2.0).max(12.0 * u), w: cw, h: ch };
    let below = if login_second(st, w, h).is_some() { 46.0 * u } else { 0.0 };
    let button = R { x: card.x + 24.0 * u, y: card.y + ch - 24.0 * u - 38.0 * u - below, w: cw - 48.0 * u, h: 38.0 * u };
    (card, button)
}

/// Android: "Scan a code from your computer", under the main button (Google accounts cannot sign in in a WebView).
pub fn login_second(st: &State, w: f32, h: f32) -> Option<crate::gui::meeting_ui::R> {
    use crate::app::LoginPhase;
    if !cfg!(target_os = "android") || !matches!(st.login, Some(LoginPhase::Idle | LoginPhase::Failed(_))) {
        return None;
    }
    let u = st.ui;
    let cw = (440.0 * u).min(w - 24.0 * u);
    let ch = 296.0 * u;
    let (x, y) = ((w - cw) / 2.0, ((h - ch) / 2.0).max(12.0 * u));
    Some(crate::gui::meeting_ui::R { x: x + 24.0 * u, y: y + ch - 24.0 * u - 38.0 * u, w: cw - 48.0 * u, h: 38.0 * u })
}

pub fn login_hit(st: &State, px: f32, py: f32, w: f32, h: f32) -> Option<crate::app::LoginAct> {
    use crate::app::{LoginAct, LoginPhase};
    if login_second(st, w, h).is_some_and(|r| r.contains(px, py)) {
        return Some(LoginAct::Scan);
    }
    let (_, b) = login_layout(st, w, h);
    if !b.contains(px, py) {
        return None;
    }
    Some(match st.login {
        Some(LoginPhase::Waiting | LoginPhase::Scanning) => LoginAct::Cancel,
        Some(LoginPhase::Offline) => LoginAct::Retry,
        _ => LoginAct::Start,
    })
}

fn draw_login(st: &State, pm: &mut PixmapMut<'_>, text: &mut Text) {
    use crate::app::LoginPhase;
    use crate::gui::theme::*;
    let (w, h) = (pm.width() as f32, pm.height() as f32);
    let (card, button) = login_layout(st, w, h);
    let (u, id) = (st.ui, Transform::identity());
    pm.fill(Color::from_rgba8(BG[0], BG[1], BG[2], 255));
    let c = |r: Rgb, a: u8| paint(r[0], r[1], r[2], a);
    if let Some(p) = crate::gui::icon::rounded_rect(card.x, card.y, card.w, card.h, RADIUS_CARD * u) {
        pm.fill_path(&p, &c(SURFACE, 255), FillRule::Winding, id, None);
        pm.stroke_path(&p, &c(LINE, 255), &Stroke { width: 1.0, ..Stroke::default() }, id, None);
    }
    let x = card.x + 24.0 * u;
    text.draw(pm, "SoWorker", x, card.y + 44.0 * u, 24.0 * u, INK, 0.0);
    text.draw(pm, "A lightweight client for your SoWork office", x, card.y + 68.0 * u, 13.0 * u, INK_MUTED, 0.0);
    let phone = cfg!(target_os = "android");
    let (lines, label): (Vec<(String, Rgb)>, &str) = match &st.login {
        Some(LoginPhase::Waiting) if phone => (vec![("Finish signing in on SoWork's page…".into(), INK)], "Cancel"),
        Some(LoginPhase::Waiting) => (
            vec![("Waiting for you to sign in in the browser window…".into(), INK), ("It closes by itself once you are in.".into(), INK_MUTED)],
            "Cancel",
        ),
        Some(LoginPhase::Scanning) => (vec![("Point the camera at the code on your computer:".into(), INK), ("in SoWorker there, Profile, then Sign in a phone.".into(), INK_MUTED)], "Cancel"),
        Some(LoginPhase::Failed(e)) => (vec![(e.clone(), DANGER)], "Try again"),
        Some(LoginPhase::Offline) => (
            vec![("No connection.".into(), DANGER), ("Your sign-in is saved; check your network and retry.".into(), INK_MUTED)],
            "Retry",
        ),
        _ if phone => (
            vec![
                ("Email and password: sign in on SoWork's page.".into(), INK),
                ("Google account: scan the code shown by".into(), INK_MUTED),
                ("SoWorker on your computer.".into(), INK_MUTED),
            ],
            "Sign in with SoWork",
        ),
        _ => (
            vec![
                ("A browser window opens on SoWork's own sign-in page.".into(), INK),
                ("Only the sign-in token is kept on this computer.".into(), INK_MUTED),
            ],
            "Sign in with SoWork",
        ),
    };
    let room = card.w - 48.0 * u;
    for (i, (l, col)) in lines.iter().enumerate() {
        let l = crate::gui::sidebar::fit(text, l, 13.0 * u, room);
        text.draw(pm, &l, x, card.y + 108.0 * u + i as f32 * 20.0 * u, 13.0 * u, *col, 0.0);
    }
    #[cfg(all(target_os = "android", feature = "calls"))]
    if st.login == Some(LoginPhase::Scanning) {
        // the camera picture, fitted between the text and the button
        let area = crate::gui::meeting_ui::R { x, y: card.y + 150.0 * u, w: room, h: button.y - 12.0 * u - (card.y + 150.0 * u) };
        if let Some((pw, ph, rgba)) = crate::android_scan::preview().filter(|_| area.h > 0.0) {
            if let Some(img) = tiny_skia::Pixmap::from_vec(rgba, tiny_skia::IntSize::from_wh(pw, ph).unwrap_or(tiny_skia::IntSize::from_wh(1, 1).unwrap())) {
                let k = (area.w / pw as f32).min(area.h / ph as f32);
                let (dx, dy) = (area.x + (area.w - pw as f32 * k) / 2.0, area.y + (area.h - ph as f32 * k) / 2.0);
                let paint = tiny_skia::PixmapPaint { quality: tiny_skia::FilterQuality::Bilinear, ..Default::default() };
                pm.draw_pixmap(0, 0, img.as_ref(), &paint, Transform::from_row(k, 0.0, 0.0, k, dx, dy), None);
            }
        } else {
            text.draw(pm, "Starting the camera…", area.x + area.w / 2.0, area.y + area.h / 2.0, 13.0 * u, INK_FAINT, 0.5);
        }
    }
    let waiting = matches!(st.login, Some(LoginPhase::Waiting | LoginPhase::Scanning));
    if let Some(p) = crate::gui::icon::rounded_rect(button.x, button.y, button.w, button.h, RADIUS_CARD * u) {
        if !waiting {
            pm.fill_path(&p, &c(BRAND, 255), FillRule::Winding, id, None);
        } else {
            pm.stroke_path(&p, &c(LINE, 255), &Stroke { width: 1.0, ..Stroke::default() }, id, None);
        }
    }
    let tc = if waiting { INK_MUTED } else { ON_BRAND };
    text.draw(pm, label, button.x + button.w / 2.0, button.y + button.h / 2.0 + 5.0 * u, 14.0 * u, tc, 0.5);
    if let Some(b) = login_second(st, w, h) {
        if let Some(p) = crate::gui::icon::rounded_rect(b.x, b.y, b.w, b.h, RADIUS_CARD * u) {
            pm.stroke_path(&p, &c(LINE, 255), &Stroke { width: 1.0, ..Stroke::default() }, id, None);
        }
        text.draw(pm, "Scan a code from your computer", b.x + b.w / 2.0, b.y + b.h / 2.0 + 5.0 * u, 14.0 * u, INK, 0.5);
    }
}

/// Screen geometry: the map fills the main panel, the camera centred on me.
impl State {
    pub fn main_rect_for(&self, w: f32, h: f32) -> crate::gui::meeting_ui::R {
        crate::gui::panels::frame(self, w, h).main
    }

    /// Screen position of the map's centre: the middle of the main panel.
    pub fn view_centre(&self, w: f32, h: f32) -> (f32, f32) {
        let m = self.main_rect_for(w, h);
        (m.x + m.w / 2.0, m.y + m.h / 2.0)
    }

    /// Screen position -> world position for a window of the given size, camera centred on me.
    pub fn to_world(&self, px: f32, py: f32, w: f32, h: f32) -> Option<(f32, f32)> {
        let me = self.me()?;
        let s = self.scale * self.ui;
        let (vx, vy) = self.view_centre(w, h);
        Some((me.x + (px - vx) / s, me.y + (py - vy) / s))
    }
}

/// The last picture of the map layer, kept while only video frames change.
#[derive(Default)]
pub struct MapCache {
    data: Vec<u8>,
    size: (u32, u32),
    /// False once the scene changed: the next draw rasterizes the map again.
    pub valid: bool,
}

/// The map, its walls, zones, the walking path and the people on it (everything under the panels).
fn draw_world(st: &State, pm: &mut PixmapMut<'_>, text: &mut Text, w: f32, h: f32) {
    pm.fill(Color::from_rgba8(20, 22, 27, 255));
    let me = st.me();
    let (cx, cy) = me.map_or((3000.0, 3000.0), |m| (m.x, m.y));
    let ui = st.ui;
    let sc = st.scale * ui;
    let (vx, vy) = st.view_centre(w, h); // the map lives in the main panel
    let xf = |x: f32| (x - cx) * sc + vx;
    let yf = |y: f32| (y - cy) * sc + vy;
    let px = T * sc;
    let (i0, i1) = (((cx - vx / sc) / T).floor() as i32, ((cx + (w - vx) / sc) / T).floor() as i32); // generous: the panels cover the edges
    let (j0, j1) = (((cy - vy / sc) / T).floor() as i32, ((cy + (h - vy) / sc) / T).floor() as i32);
    let id = Transform::identity();

    let floor = paint(43, 58, 82, 255);
    for j in j0..=j1 {
        for i in i0..=i1 {
            if st.world.floors.contains(&(i, j)) {
                if let Some(r) = Rect::from_xywh(xf(i as f32 * T) + 0.5, yf(j as f32 * T) + 0.5, px - 1.0, px - 1.0) {
                    pm.fill_rect(r, &floor, id, None);
                }
            }
        }
    }
    let edges = |pm: &mut PixmapMut<'_>, map: &std::collections::HashMap<(i32, i32), u8>, col: [u8; 3], width: f32| {
        let mut pb = PathBuilder::new();
        for j in j0..=j1 {
            for i in i0..=i1 {
                let m = map.get(&(i, j)).copied().unwrap_or(0);
                if m == 0 {
                    continue;
                }
                let (x, y) = (xf(i as f32 * T), yf(j as f32 * T));
                let mut seg = |a: (f32, f32), b: (f32, f32)| {
                    pb.move_to(a.0, a.1);
                    pb.line_to(b.0, b.1);
                };
                if m & N != 0 { seg((x, y), (x + px, y)); }
                if m & S != 0 { seg((x, y + px), (x + px, y + px)); }
                if m & W != 0 { seg((x, y), (x, y + px)); }
                if m & E != 0 { seg((x + px, y), (x + px, y + px)); }
            }
        }
        if let Some(p) = pb.finish() {
            pm.stroke_path(&p, &paint(col[0], col[1], col[2], 255), &Stroke { width, ..Stroke::default() }, Transform::identity(), None);
        }
    };
    edges(pm, &st.world.soft, [116, 128, 156], (2.5 * sc).max(1.5 * ui)); // walk-through walls, faint
    edges(pm, &st.world.walls, [232, 195, 106], (5.0 * sc).max(2.0 * ui)); // solid walls

    for z in &st.world.zones {
        let (zx, zy) = (xf(z.x - z.sx * ZK), yf(z.y - z.sy * ZK));
        let (zw, zh) = (z.sx * ZK * 2.0 * sc, z.sy * ZK * 2.0 * sc);
        if let Some(r) = Rect::from_xywh(zx, zy, zw, zh) {
            if st.cur_zone_id.as_deref() == Some(z.id.as_str()) {
                pm.fill_rect(r, &paint(217, 87, 127, 46), id, None);
            }
            let path = PathBuilder::from_rect(r);
            let col = if z.meeting { (217, 87, 127) } else { (102, 102, 119) };
            pm.stroke_path(&path, &paint(col.0, col.1, col.2, 255), &Stroke { width: 1.0, ..Stroke::default() }, id, None);
        }
        if let Some(n) = &z.name {
            text.draw(pm, n, xf(z.x), zy - 3.0 * ui, 11.0 * ui, [217, 87, 127], 0.5);
        }
    }

    if let (Some(path), Some(m)) = (&st.path, me) {
        let mut pb = PathBuilder::new();
        pb.move_to(xf(m.x), yf(m.y));
        for &(px_, py_) in path {
            pb.line_to(xf(px_), yf(py_));
        }
        if let Some(p) = pb.finish() {
            let stroke = Stroke { width: 2.0, dash: StrokeDash::new(vec![5.0, 5.0], 0.0), ..Stroke::default() };
            pm.stroke_path(&p, &paint(127, 214, 168, 255), &stroke, id, None);
        }
        if let Some(c) = PathBuilder::from_circle(xf(st.path_goal.0), yf(st.path_goal.1), 6.0) {
            pm.stroke_path(&c, &paint(127, 214, 168, 255), &Stroke { width: 2.0, ..Stroke::default() }, id, None);
        }
    }

    let mut order: Vec<_> = st.users.values().collect();
    order.sort_by(|a, b| (a.id == st.me_id).cmp(&(b.id == st.me_id))); // draw ourselves last
    for u in order {
        let (sx, sy, r) = (xf(u.x), yf(u.y), (40.0 * sc).max(6.0 * ui));
        let (cr, cg, cb) = hue_rgb(&u.id);
        let away = if u.id == st.me_id { !st.available } else { u.away() };
        let alpha = if away { 0.45 } else { 1.0 };
        match st.pics.get(&u.id) {
            Some(p) => {
                // the picture is already cut to a circle; draw it scaled to the avatar size
                let s = (2.0 * r) / p.w as f32;
                let tf = Transform::from_scale(s, s).post_translate(sx - r, sy - r);
                pm.draw_pixmap(0, 0, p.px(), &PixmapPaint { opacity: alpha, quality: tiny_skia::FilterQuality::Bilinear, ..PixmapPaint::default() }, tf, None);
            }
            None => {
                if let Some(c) = PathBuilder::from_circle(sx, sy, r) {
                    pm.fill_path(&c, &paint(cr, cg, cb, (255.0 * alpha) as u8), FillRule::Winding, id, None);
                }
                let ch = u.name.trim().chars().next().map_or("?".to_string(), |c| c.to_uppercase().to_string());
                text.draw(pm, &ch, sx, sy + r * 0.38, r * 1.1, [255, 255, 255], 0.5);
            }
        }
        if away {
            // small amber dot: out of calls / busy
            if let Some(c) = PathBuilder::from_circle(sx + r * 0.72, sy + r * 0.72, (r * 0.28).max(3.0)) {
                pm.fill_path(&c, &paint(240, 170, 60, 255), FillRule::Winding, id, None);
            }
        }
        if u.speaking {
            if let Some(c) = PathBuilder::from_circle(sx, sy, r + 4.0) {
                pm.stroke_path(&c, &paint(127, 214, 168, 255), &Stroke { width: 3.0, ..Stroke::default() }, id, None);
            }
        }
        if u.id == st.me_id {
            if let Some(c) = PathBuilder::from_circle(sx, sy, r) {
                pm.stroke_path(&c, &paint(255, 255, 255, 255), &Stroke { width: 2.0, ..Stroke::default() }, id, None);
            }
        }
        let label = if u.state == "walk" { format!("{} ·", u.name) } else { u.name.clone() };
        text.draw(pm, &label, sx, sy - r - 4.0 * ui, 11.0 * ui, [221, 221, 238], 0.5);
    }

}

pub fn draw(st: &State, pm: &mut PixmapMut<'_>, text: &mut Text, cache: &mut MapCache) {
    if st.login.is_some() {
        return draw_login(st, pm, text);
    }
    let (w, h) = (pm.width() as f32, pm.height() as f32);
    let (id, ui) = (Transform::identity(), st.ui);
    // The map and the people on it only change with the scene, not with each video frame: while a call shows video,
    // keep the last picture of them and copy it back (a memcpy) instead of rasterizing it again.
    let keep = !st.call.tiles.is_empty();
    let size = (pm.width(), pm.height());
    if keep && cache.valid && cache.size == size && cache.data.len() == pm.data_mut().len() {
        pm.data_mut().copy_from_slice(&cache.data);
    } else {
        draw_world(st, pm, text, w, h);
        cache.valid = keep;
        if keep {
            cache.size = size;
            cache.data.clear();
            cache.data.extend_from_slice(pm.data_mut());
        }
    }
    let main = st.main_rect_for(w, h);
    let now_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64);
    // transient messages (e.g. "Walking to ...")
    if let Some((t, _)) = &st.toast {
        // right of the desk button when it is there, which shares the corner
        let desk = crate::gui::sidebar::map_desk(st, w, h);
        let (tx, ty) = if crate::gui::panels::empty(&desk) { (main.x + 10.0 * ui, main.y + 20.0 * ui) } else { (desk.x + desk.w + 10.0 * ui, desk.y + desk.h / 2.0 + 4.5 * ui) };
        text.shadowed(pm, t, tx, ty, 13.0 * ui, [240, 200, 120], 0.0);
    }
    if let Some(r) = notice_rect(st, text, w, h) {
        draw_notice(st, pm, text, r);
    }
    crate::gui::panels::draw_back(st, pm);
    crate::gui::sidebar::draw(st, pm, text, now_ms);
    crate::gui::chatui::draw(st, pm, text);
    crate::gui::meeting_ui::draw(st, pm, text, w, h);
    crate::gui::sidebar::draw_map_desk(st, pm);
    if st.sidebar.mobile {
        // settings fill the space between header and footer, which stay on top
        crate::gui::profile_ui::draw(st, pm, text);
        crate::gui::panels::draw_nav(st, pm, text);
    } else {
        crate::gui::panels::draw_nav(st, pm, text);
        crate::gui::sidebar::draw_hover_card(st, pm, text, now_ms);
        crate::gui::profile_ui::draw(st, pm, text); // settings over everything else
    }
    #[cfg(not(target_os = "android"))]
    draw_pair_code(st, pm, text, now_ms as f64 / 1000.0);
    if st.confirm_logout || st.confirm_leave.is_some() {
        let leave = st.confirm_leave.is_some();
        let (title, l1, l2, yes) = if leave {
            ("Leave the meeting?", "You are in a call in this area.", "Walking out of it leaves the call.", "Leave")
        } else {
            ("Log out?", "This forgets your saved sign-in on this computer.", "You will be taken back to the sign-in screen.", "Log out")
        };
        pm.fill_rect(Rect::from_xywh(0.0, 0.0, w, h).unwrap(), &paint(0, 0, 0, 140), id, None);
        let (card, cancel, confirm) = modal(st, w, h);
        let rc = |r: &crate::gui::meeting_ui::R, rad: f32| crate::gui::icon::rounded_rect(r.x, r.y, r.w, r.h, rad);
        if let Some(p) = rc(&card, crate::gui::theme::RADIUS_CARD * ui) {
            pm.fill_path(&p, &paint(crate::gui::theme::SURFACE[0], crate::gui::theme::SURFACE[1], crate::gui::theme::SURFACE[2], 255), FillRule::Winding, id, None);
            pm.stroke_path(&p, &paint(crate::gui::theme::LINE[0], crate::gui::theme::LINE[1], crate::gui::theme::LINE[2], 255), &Stroke { width: 1.0, ..Stroke::default() }, id, None);
        }
        text.draw(pm, title, card.x + 16.0 * ui, card.y + 30.0 * ui, 16.0 * ui, crate::gui::theme::INK, 0.0);
        text.draw(pm, l1, card.x + 16.0 * ui, card.y + 56.0 * ui, 13.0 * ui, crate::gui::theme::INK_MUTED, 0.0);
        text.draw(pm, l2, card.x + 16.0 * ui, card.y + 76.0 * ui, 13.0 * ui, crate::gui::theme::INK_MUTED, 0.0);
        for (r, label, col) in [(&cancel, "Cancel", crate::gui::theme::INK_MUTED), (&confirm, yes, crate::gui::theme::DANGER)] {
            if let Some(p) = rc(r, crate::gui::theme::RADIUS_CARD * ui) {
                pm.stroke_path(&p, &paint(crate::gui::theme::LINE[0], crate::gui::theme::LINE[1], crate::gui::theme::LINE[2], 255), &Stroke { width: 1.0, ..Stroke::default() }, id, None);
            }
            text.draw(pm, label, r.x + r.w / 2.0, r.y + r.h / 2.0 + 5.0 * ui, 14.0 * ui, col, 0.5);
        }
    }
}

/// The phone sign-in code (desktop): what it is, the code, and how long it stays. Any click puts it away.
#[cfg(not(target_os = "android"))]
fn draw_pair_code(st: &State, pm: &mut PixmapMut<'_>, text: &mut Text, now: f64) {
    use crate::gui::theme::*;
    let Some((code, until)) = &st.pair_code else { return };
    let Some((n, dark)) = crate::pairing::qr(code) else { return };
    let (w, h, u, id) = (pm.width() as f32, pm.height() as f32, st.ui, Transform::identity());
    pm.fill_rect(Rect::from_xywh(0.0, 0.0, w, h).unwrap(), &paint(0, 0, 0, 160), id, None);
    // whole pixels per module, with the 4-module quiet zone the readers need
    let room = (h - 190.0 * u).min(w - 64.0 * u).min(420.0 * u).max(0.0);
    let m = (room / (n + 8) as f32).floor().max(2.0);
    let side = m * (n + 8) as f32;
    let (cw, ch) = ((side + 40.0 * u).max(420.0 * u), side + 150.0 * u);
    let card = crate::gui::meeting_ui::R { x: ((w - cw) / 2.0).round(), y: ((h - ch) / 2.0).round(), w: cw, h: ch };
    if let Some(p) = crate::gui::icon::rounded_rect(card.x, card.y, card.w, card.h, RADIUS_CARD * u) {
        pm.fill_path(&p, &paint(SURFACE[0], SURFACE[1], SURFACE[2], 255), FillRule::Winding, id, None);
        pm.stroke_path(&p, &paint(LINE[0], LINE[1], LINE[2], 255), &Stroke { width: 1.0, ..Stroke::default() }, id, None);
    }
    let x = card.x + 20.0 * u;
    text.draw(pm, "Sign in your phone", x, card.y + 32.0 * u, 16.0 * u, INK, 0.0);
    text.draw(pm, "In SoWorker on the phone, tap \"Scan a code from your computer\".", x, card.y + 56.0 * u, 13.0 * u, INK_MUTED, 0.0);
    text.draw(pm, "This code is your sign-in: anyone who scans it gets your account.", x, card.y + 76.0 * u, 13.0 * u, DANGER, 0.0);
    let (qx, qy) = ((card.x + (cw - side) / 2.0).round(), card.y + 92.0 * u);
    pm.fill_rect(Rect::from_xywh(qx, qy, side, side).unwrap(), &paint(255, 255, 255, 255), id, None);
    let black = paint(0, 0, 0, 255);
    for (i, _) in dark.iter().enumerate().filter(|(_, d)| **d) {
        let (mx, my) = ((i % n) as f32, (i / n) as f32);
        if let Some(r) = Rect::from_xywh(qx + (mx + 4.0) * m, qy + (my + 4.0) * m, m, m) {
            pm.fill_rect(r, &black, id, None);
        }
    }
    let left = (until - now).clamp(0.0, crate::pairing::TTL as f64).ceil() as u32;
    text.draw(pm, &format!("Disappears in {left} s. Click to close."), card.x + cw / 2.0, qy + side + 24.0 * u, 12.0 * u, INK_MUTED, 0.5);
    text.draw(pm, "Only one of the two devices can be connected at a time.", card.x + cw / 2.0, qy + side + 44.0 * u, 12.0 * u, INK_FAINT, 0.5);
}

impl crate::app::Frame {
    /// The picture as a tiny-skia pixmap, without copying.
    pub fn px(&self) -> tiny_skia::PixmapRef<'_> {
        tiny_skia::PixmapRef::from_bytes(&self.rgba, self.w, self.h).expect("frame size checked when stored")
    }
}
