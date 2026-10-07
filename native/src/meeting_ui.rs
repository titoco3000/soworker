//! The meeting dock, ported from omoffice (frontend/src/features/meetings: MeetingDock, MeetingPanel, MeetingTile,
//! MeetingControls, MeetingChat). Same structure and measurements (CSS px x display scale), same controls:
//! a floating draggable card that snaps to one of eight slots (or full window), a header with a fullscreen toggle,
//! one tile per attendee in a gallery grid or pinned + sidebar, hover buttons (mute / camera off / pin), a status line,
//! the controls row (leave, lock, camera, mic, share screen) and the chat toggle with its column.
use crate::app::State;
use crate::icon;
use crate::icons::{self, Shape};
use crate::render::Text;
use crate::theme::{self, Rgb};
use tiny_skia::{FillRule, FilterQuality, Paint, Path, PathBuilder, Pattern, Pixmap, PixmapPaint, SpreadMode, Stroke, Transform};

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
}

pub struct Dock {
    pub full: bool,
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
        Dock { full: false, slot: Slot::BotRight, drag: None, chat_open: false, pinned: None, last_auto: None, hover: Hover::default(), scroll: 0.0 }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ctl {
    Leave,
    Lock,
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
}

#[derive(Clone, PartialEq, Debug)]
pub enum Hit {
    Header,
    Ctl(Ctl),
    Tile(String),
    TileBtn(String, TileBtn),
    Inside,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct R {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
impl R {
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.w && py >= self.y && py <= self.y + self.h
    }
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

pub struct Layout {
    pub dock: R,
    pub header: R,
    pub full_btn: R,
    pub tiles: Vec<(TileSpec, R, bool)>, // spec, rect, pinned
    pub status: (f32, f32),
    pub controls: Vec<(Ctl, R)>,
    pub chat_col: Option<R>,
    pub send_btn: Option<R>,
    pub input: Option<R>,
    pub bar: R,
    /// The tiles area, and the sidebar column inside it (pinned layout) with how far it can scroll.
    pub area: R,
    pub side: Option<R>,
    pub max_scroll: f32,
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
        let name = st.users.get(&id).map(|u| u.name.clone()).or_else(|| st.cluster.as_ref().and_then(|c| c.users.iter().find(|(i, _)| *i == id).map(|(_, n)| n.clone()))).unwrap_or_else(|| id.clone());
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
            name: "Screen share".into(),
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

fn slot_pos(slot: Slot, dw: f32, dh: f32, w: f32, h: f32, margin: f32) -> (f32, f32) {
    let (left, center, right) = (margin, (w - dw) / 2.0, w - dw - margin);
    let (top, mid, bot) = (margin, (h - dh) / 2.0, h - dh - margin);
    match slot {
        Slot::TopLeft => (left, top),
        Slot::TopCenter => (center, top),
        Slot::TopRight => (right, top),
        Slot::MidLeft => (left, mid),
        Slot::MidRight => (right, mid),
        Slot::BotLeft => (left, bot),
        Slot::BotCenter => (center, bot),
        Slot::BotRight => (right, bot),
    }
}

/// Which slot a dropped dock centre is nearest to (the 3x3 grid without its centre square).
pub fn nearest_slot(cx: f32, cy: f32, w: f32, h: f32) -> Slot {
    let col = if cx < w / 3.0 { 0 } else if cx < 2.0 * w / 3.0 { 1 } else { 2 };
    let row = if cy < h / 3.0 { 0 } else if cy < 2.0 * h / 3.0 { 1 } else { 2 };
    match (row, col) {
        (0, 0) => Slot::TopLeft,
        (0, 1) => Slot::TopCenter,
        (0, _) => Slot::TopRight,
        (1, 1) => if cx < w / 2.0 { Slot::MidLeft } else { Slot::MidRight },
        (1, 0) => Slot::MidLeft,
        (1, _) => Slot::MidRight,
        (_, 0) => Slot::BotLeft,
        (_, 1) => Slot::BotCenter,
        _ => Slot::BotRight,
    }
}

fn text_w(text: &mut Text, s: &str, px: f32) -> f32 {
    text.width(s, px)
}

pub fn layout(st: &State, text: &mut Text, w: f32, h: f32) -> Option<Layout> {
    if !visible(st) {
        return None;
    }
    let u = st.ui;
    let d = &st.dock;
    let (sb, right) = st.map_edges(w); // the dock lives on the map, between the sidebar and the chat panel
    let mw = right - sb;
    let (dw, dh) = if d.full { (mw, h) } else { ((500.0 * u).min(0.7 * mw), (300.0 * u).min(0.7 * h)) };
    let (dx, dy) = if d.full {
        (sb, 0.0)
    } else if let Some(dr) = d.drag {
        dr.pos
    } else {
        let (x, y) = slot_pos(d.slot, dw, dh, mw, h, 16.0 * u);
        (x + sb, y)
    };
    let dock = R { x: dx, y: dy, w: dw, h: dh };
    let header = R { x: dx, y: dy, w: dw, h: 37.0 * u };
    let full_btn = R { x: dx + dw - 12.0 * u - 30.0 * u, y: dy + (37.0 * u - 26.0 * u) / 2.0, w: 30.0 * u, h: 26.0 * u };
    let bar_h = 59.0 * u; // p-3 + 34px buttons + border
    let bar = R { x: dx, y: dy + dh - bar_h, w: dw, h: bar_h };

    // chat column on the right (w-chat = 20rem, 10rem when compact)
    let chat_w = if d.chat_open { Some(if d.full { 320.0 * u } else { 160.0 * u }) } else { None };
    let body_top = dy + header.h;
    let body_h = dh - header.h - bar_h;
    let chat_col = chat_w.map(|cw| R { x: dx + dw - cw, y: body_top, w: cw, h: body_h });
    let main_w = dw - chat_w.unwrap_or(0.0);

    // tiles area: p-3, then the status line (mt-3 + 16px)
    let pad = 12.0 * u;
    let status_h = 28.0 * u;
    let area = R { x: dx + pad, y: body_top + pad, w: main_w - 2.0 * pad, h: body_h - 2.0 * pad - status_h };
    let specs = tiles(st);
    let gap = 8.0 * u;
    let pinned_idx = d.pinned.as_ref().and_then(|p| specs.iter().position(|t| &t.id == p));
    let mut out = Vec::new();
    let (mut max_scroll, mut side) = (0.0f32, None);
    match pinned_idx {
        Some(pi) => {
            let side_w = (192.0 * u).min(area.w * 0.45);
            let main = R { x: area.x, y: area.y, w: area.w - side_w - gap, h: area.h };
            let th = side_w * 9.0 / 16.0;
            let n_side = specs.len().saturating_sub(1);
            let content = if n_side > 0 { n_side as f32 * (th + gap) - gap } else { 0.0 };
            max_scroll = (content - area.h).max(0.0);
            let mut y = area.y - d.scroll.clamp(0.0, max_scroll);
            for (i, s) in specs.into_iter().enumerate() {
                if i == pi {
                    out.push((s, main, true));
                } else {
                    out.push((s, R { x: area.x + area.w - side_w, y, w: side_w, h: th }, false));
                    y += th + gap;
                }
            }
            side = Some(R { x: area.x + area.w - side_w, y: area.y, w: side_w, h: area.h });
        }
        None => {
            let n = specs.len().max(1);
            let cols = (n as f32).sqrt().ceil() as usize;
            let rows = n.div_ceil(cols);
            let (cw, ch) = ((area.w - gap * (cols as f32 - 1.0)) / cols as f32, (area.h - gap * (rows as f32 - 1.0)) / rows as f32);
            for (i, s) in specs.into_iter().enumerate() {
                let (c, r) = (i % cols, i / cols);
                out.push((s, R { x: area.x + c as f32 * (cw + gap), y: area.y + r as f32 * (ch + gap), w: cw, h: ch }, false));
            }
        }
    }

    // controls row: Leave, lock, camera, mic, share on the left; chat toggle on the right
    let bh = 34.0 * u;
    let by = bar.y + 12.0 * u + 1.0 * u;
    let (fs, icon_sz, gp) = (14.0 * u, 14.0 * u, 6.0 * u);
    let mut x = dx + 12.0 * u;
    let mut controls = Vec::new();
    let icon_w = 12.0 * u + icon_sz + 12.0 * u + 2.0;
    let leave_w = 12.0 * u + icon_sz + gp + text_w(text, "Leave", fs) + 12.0 * u + 2.0;
    let share_label = if st.call.share { "Stop sharing screen" } else { "Share screen" };
    let share_w = 12.0 * u + icon_sz + gp + text_w(text, share_label, fs) + 12.0 * u + 2.0;
    for (c, wd) in [(Ctl::Leave, leave_w), (Ctl::Lock, icon_w), (Ctl::Cam, icon_w), (Ctl::Mic, icon_w), (Ctl::Share, share_w)] {
        controls.push((c, R { x, y: by, w: wd, h: bh }));
        x += wd + 8.0 * u; // gap-2
    }
    let chat_btn = 18.0 * u + 16.0 * u + 2.0;
    controls.push((Ctl::Chat, R { x: dx + dw - 12.0 * u - chat_btn, y: bar.y + (bar_h - chat_btn) / 2.0, w: chat_btn, h: chat_btn }));

    let (mut send_btn, mut input) = (None, None);
    if let Some(c) = chat_col {
        let (fh, send_w) = (34.0 * u, 52.0 * u);
        let y = c.y + c.h - fh - 8.0 * u;
        send_btn = Some(R { x: c.x + c.w - 8.0 * u - send_w, y, w: send_w, h: fh });
        input = Some(R { x: c.x + 8.0 * u, y, w: c.w - 16.0 * u - send_w - 4.0 * u, h: fh });
    }
    Some(Layout { dock, header, full_btn, tiles: out, status: (dx + pad, body_top + body_h - 10.0 * u), controls, chat_col, send_btn, input, bar, area, side, max_scroll })
}

fn tile_buttons(spec: &TileSpec) -> Vec<TileBtn> {
    // omoffice: self and content tiles only get pin; other attendees also get mute and camera off
    if spec.is_self || spec.content { vec![TileBtn::Pin] } else { vec![TileBtn::Mute, TileBtn::CameraOff, TileBtn::Pin] }
}

fn tile_button_rects(r: R, n: usize, u: f32) -> Vec<R> {
    let (bw, bh, gap) = (28.0 * u, 24.0 * u, 8.0 * u);
    let total = n as f32 * bw + (n as f32 - 1.0) * gap;
    (0..n).map(|i| R { x: r.x + (r.w - total) / 2.0 + i as f32 * (bw + gap), y: r.y + (r.h - bh) / 2.0, w: bw, h: bh }).collect()
}

pub fn hit(st: &State, text: &mut Text, px: f32, py: f32, w: f32, h: f32) -> Option<Hit> {
    let l = layout(st, text, w, h)?;
    if !l.dock.contains(px, py) {
        return None;
    }
    if !st.dock.full && l.full_btn.contains(px, py) || st.dock.full && l.full_btn.contains(px, py) {
        return Some(Hit::Ctl(Ctl::FullToggle));
    }
    if l.header.contains(px, py) {
        return Some(Hit::Header);
    }
    for (c, r) in &l.controls {
        if r.contains(px, py) {
            return Some(Hit::Ctl(*c));
        }
    }
    if l.send_btn.is_some_and(|r| r.contains(px, py)) {
        return Some(Hit::Ctl(Ctl::Send));
    }
    for (spec, r, pinned) in &l.tiles {
        let clipped = !*pinned && l.side.is_some() && !l.area.contains(px, py);
        if r.contains(px, py) && !clipped {
            let btns = tile_buttons(spec);
            for (b, br) in btns.iter().zip(tile_button_rects(*r, btns.len(), st.ui)) {
                if br.contains(px, py) {
                    return Some(Hit::TileBtn(spec.id.clone(), *b));
                }
            }
            return Some(Hit::Tile(spec.id.clone()));
        }
    }
    Some(Hit::Inside)
}

// ---- drawing

fn paint(rgb: Rgb, a: u8) -> Paint<'static> {
    let mut p = Paint::default();
    p.set_color_rgba8(rgb[0], rgb[1], rgb[2], a);
    p.anti_alias = true;
    p
}

fn rr(r: R, radius: f32) -> Option<Path> {
    icon::rounded_rect(r.x, r.y, r.w, r.h, radius)
}

fn fill_rr(pm: &mut Pixmap, r: R, radius: f32, rgb: Rgb, a: u8) {
    if let Some(p) = rr(r, radius) {
        pm.fill_path(&p, &paint(rgb, a), FillRule::Winding, Transform::identity(), None);
    }
}

fn stroke_rr(pm: &mut Pixmap, r: R, radius: f32, rgb: Rgb, width: f32) {
    let h = width / 2.0;
    if let Some(p) = rr(R { x: r.x + h, y: r.y + h, w: r.w - width, h: r.h - width }, (radius - h).max(0.0)) {
        pm.stroke_path(&p, &paint(rgb, 255), &Stroke { width, ..Stroke::default() }, Transform::identity(), None);
    }
}

fn faux_bold(pm: &mut Pixmap, text: &mut Text, s: &str, x: f32, y: f32, px: f32, rgb: Rgb, align: f32) {
    text.draw(pm, s, x, y, px, rgb, align);
    text.draw(pm, s, x + 0.6, y, px, rgb, align);
}

/// A camera/screen frame filling a rounded rect: `cover` crops to fill (cameras), otherwise letterboxes (screens).
fn draw_video(pm: &mut Pixmap, img: &Pixmap, r: R, radius: f32, cover: bool, mirror: bool) {
    let (iw, ih) = (img.width() as f32, img.height() as f32);
    let s = if cover { (r.w / iw).max(r.h / ih) } else { (r.w / iw).min(r.h / ih) };
    let (dw, dh) = (iw * s, ih * s);
    let (ox, oy) = (r.x + (r.w - dw) / 2.0, r.y + (r.h - dh) / 2.0);
    let tf = if mirror { Transform::from_scale(-s, s).post_translate(ox + dw, oy) } else { Transform::from_scale(s, s).post_translate(ox, oy) };
    let pat = Pattern::new(img.as_ref(), SpreadMode::Pad, FilterQuality::Bilinear, 1.0, tf);
    let p = Paint { shader: pat, anti_alias: true, ..Paint::default() };
    let path = if cover { rr(r, radius) } else { PathBuilder::from_rect(tiny_skia::Rect::from_xywh(ox, oy, dw, dh).unwrap_or(tiny_skia::Rect::from_xywh(r.x, r.y, r.w, r.h).unwrap())).into() };
    if let Some(path) = path {
        pm.fill_path(&path, &p, FillRule::Winding, Transform::identity(), None);
    }
}

/// A circular picture or the first-letter placeholder, centred in the rect (omoffice `size-12` = 48px).
fn draw_avatar(pm: &mut Pixmap, text: &mut Text, pic: Option<&Pixmap>, initial: &str, cx: f32, cy: f32, size: f32, u: f32) {
    match pic {
        Some(p) => {
            let s = size / p.width() as f32;
            let tf = Transform::from_scale(s, s).post_translate(cx - size / 2.0, cy - size / 2.0);
            pm.draw_pixmap(0, 0, p.as_ref(), &PixmapPaint { quality: FilterQuality::Bilinear, ..PixmapPaint::default() }, tf, None);
        }
        None => {
            if let Some(c) = PathBuilder::from_circle(cx, cy, size / 2.0) {
                pm.fill_path(&c, &paint(theme::BRAND, 255), FillRule::Winding, Transform::identity(), None);
            }
            let ch = initial.trim().chars().next().map_or("?".to_string(), |c| c.to_uppercase().to_string());
            faux_bold(pm, text, &ch, cx, cy + 6.0 * u, 18.0 * u, theme::ON_BRAND, 0.5);
        }
    }
}

fn button_icon(pm: &mut Pixmap, shapes: &[Shape], r: R, size: f32, rgb: Rgb, alpha_dim: bool) {
    let rgb = if alpha_dim { mix(rgb, theme::SURFACE, 0.5) } else { rgb };
    icon::draw(pm, shapes, r.x + (r.w - size) / 2.0, r.y + (r.h - size) / 2.0, size, rgb);
}

fn mix(a: Rgb, b: Rgb, t: f32) -> Rgb {
    [0, 1, 2].map(|i| (a[i] as f32 * (1.0 - t) + b[i] as f32 * t) as u8)
}

pub fn draw(st: &State, pm: &mut Pixmap, text: &mut Text, w: f32, h: f32) {
    let Some(l) = layout(st, text, w, h) else { return };
    let u = st.ui;
    let radius = theme::RADIUS_CARD * u;
    let full = st.dock.full;
    let rad = if full { 0.0 } else { radius };

    // shadow, card, border
    if !full {
        for (i, a) in [(6.0, 14u8), (3.0, 20), (1.0, 26)] {
            fill_rr(pm, R { x: l.dock.x - i * u, y: l.dock.y - i * u * 0.3, w: l.dock.w + 2.0 * i * u, h: l.dock.h + 2.0 * i * u }, radius + i * u, [0, 0, 0], a);
        }
    }
    fill_rr(pm, l.dock, rad, theme::SURFACE, 255);

    // header: bg-bg, border-b, "Meeting" + fullscreen toggle
    if let Some(p) = icon::rounded_rect(l.header.x, l.header.y, l.header.w, l.header.h + rad, rad) {
        pm.fill_path(&p, &paint(theme::BG, 255), FillRule::Winding, Transform::identity(), None);
    }
    fill_rr(pm, R { x: l.dock.x, y: l.header.y + l.header.h - 1.0, w: l.dock.w, h: 1.0 }, 0.0, theme::LINE, 255);
    fill_rr(pm, R { x: l.dock.x, y: l.header.y + l.header.h, w: l.dock.w, h: l.dock.h - l.header.h - l.bar.h }, 0.0, theme::SURFACE, 255);
    faux_bold(pm, text, "Meeting", l.header.x + 12.0 * u, l.header.y + 24.0 * u, 14.0 * u, theme::INK, 0.0);
    stroke_rr(pm, l.full_btn, radius, theme::LINE, 1.0);
    button_icon(pm, if full { icons::MINIMIZE } else { icons::MAXIMIZE }, l.full_btn, 12.0 * u, theme::INK_MUTED, false);

    // controls bar background first (rounded bottom corners over the card), then the body on top
    if let Some(p) = icon::rounded_rect(l.bar.x, l.bar.y - rad, l.bar.w, l.bar.h + rad, rad) {
        pm.fill_path(&p, &paint(theme::BG, 255), FillRule::Winding, Transform::identity(), None);
    }
    if rad > 0.0 {
        fill_rr(pm, R { x: l.bar.x, y: l.bar.y - rad, w: l.bar.w, h: rad }, 0.0, theme::SURFACE, 255);
    }
    fill_rr(pm, R { x: l.bar.x, y: l.bar.y, w: l.bar.w, h: 1.0 }, 0.0, theme::LINE, 255);

    // tiles
    let hover = st.dock.hover.tile.as_deref();
    for (spec, r, pinned) in l.tiles.iter().filter(|t| t.2 || l.side.is_none()) {
        draw_tile(st, pm, text, spec, *r, *pinned, hover == Some(spec.id.as_str()), u, radius);
    }
    if let Some(side) = l.side {
        // the sidebar column scrolls and is clipped to the tiles area: draw it on its own canvas, then composite
        if let Some(mut tmp) = Pixmap::new(side.w.ceil().max(1.0) as u32, side.h.ceil().max(1.0) as u32) {
            for (spec, r, _) in l.tiles.iter().filter(|t| !t.2) {
                let local = R { x: r.x - side.x, y: r.y - side.y, w: r.w, h: r.h };
                draw_tile(st, &mut tmp, text, spec, local, false, hover == Some(spec.id.as_str()), u, radius);
            }
            pm.draw_pixmap(side.x as i32, side.y as i32, tmp.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
        }
    }
    text.draw(pm, status_label(st), l.status.0, l.status.1, 12.0 * u, theme::INK_MUTED, 0.0);

    // chat column
    if let Some(c) = l.chat_col {
        fill_rr(pm, R { x: c.x, y: c.y, w: 1.0, h: c.h }, 0.0, theme::LINE, 255);
        text.draw(pm, "No messages yet.", c.x + 8.0 * u, c.y + 24.0 * u, 14.0 * u, theme::INK_MUTED, 0.0);
        text.draw(pm, "(chat is not available in this client yet)", c.x + 8.0 * u, c.y + 42.0 * u, 11.0 * u, theme::INK_FAINT, 0.0);
        fill_rr(pm, R { x: c.x, y: c.y + c.h - 34.0 * u - 8.0 * u - 8.0 * u, w: c.w, h: 1.0 }, 0.0, theme::LINE, 255);
        if let Some(i) = l.input {
            fill_rr(pm, i, radius, theme::BG, 255);
            stroke_rr(pm, i, radius, theme::LINE, 1.0);
            text.draw(pm, "Message everyone", i.x + 8.0 * u, i.y + 22.0 * u, 14.0 * u, theme::INK_FAINT, 0.0);
        }
        if let Some(s) = l.send_btn {
            stroke_rr(pm, s, radius, theme::LINE, 1.0);
            text.draw(pm, "Send", s.x + s.w / 2.0, s.y + 22.0 * u, 14.0 * u, mix(theme::BRAND, theme::SURFACE, 0.5), 0.5);
        }
    }

    // controls row
    let joined = st.call.joined.is_some();
    for (c, r) in &l.controls {
        let isz = 14.0 * u;
        let (shapes, label, rgb, dim): (&[Shape], Option<&str>, Rgb, bool) = match c {
            Ctl::Leave => (icons::PHONE_OFF, Some("Leave"), theme::DANGER, false),
            Ctl::Lock => (icons::UNLOCK, None, theme::INK_MUTED, false),
            Ctl::Cam => (if st.call.cam { icons::VIDEO } else { icons::VIDEO_OFF }, None, if st.call.cam { theme::INK } else { theme::INK_MUTED }, !joined),
            Ctl::Mic => (if st.call.mic { icons::MIC } else { icons::MIC_OFF }, None, if st.call.mic { theme::INK } else { theme::INK_MUTED }, !joined),
            Ctl::Share => (
                if st.call.share { icons::STOP_CIRCLE } else { icons::MONITOR },
                Some(if st.call.share { "Stop sharing screen" } else { "Share screen" }),
                if st.call.share { theme::INK } else { theme::INK_MUTED },
                !joined,
            ),
            Ctl::Chat => (icons::MESSAGE_CIRCLE, None, if st.chat.open { theme::BRAND } else { theme::INK_MUTED }, false),
            _ => continue,
        };
        stroke_rr(pm, *r, radius, theme::LINE, 1.0);
        let col = if dim { mix(rgb, theme::BG, 0.5) } else { rgb };
        match label {
            Some(t) => {
                icon::draw(pm, shapes, r.x + 12.0 * u + 1.0, r.y + (r.h - isz) / 2.0, isz, col);
                text.draw(pm, t, r.x + 12.0 * u + isz + 6.0 * u + 1.0, r.y + r.h / 2.0 + 5.0 * u, 14.0 * u, col, 0.0);
            }
            None => {
                let sz = if *c == Ctl::Chat { 18.0 * u } else { isz };
                icon::draw(pm, shapes, r.x + (r.w - sz) / 2.0, r.y + (r.h - sz) / 2.0, sz, col);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_tile(st: &State, pm: &mut Pixmap, text: &mut Text, spec: &TileSpec, r: R, pinned: bool, hover: bool, u: f32, radius: f32) {
    fill_rr(pm, r, radius, theme::MUTED, 255);
    let video = spec.video_key.as_ref().and_then(|k| st.call.tiles.iter().find(|t| &t.key == k));
    match video {
        Some(t) => draw_video(pm, &t.img, r, radius, !spec.content, spec.is_self),
        None => {
            let pic = spec.user.as_ref().and_then(|id| st.pics.get(id));
            draw_avatar(pm, text, pic, &spec.initial, r.x + r.w / 2.0, r.y + r.h / 2.0, 48.0 * u, u);
        }
    }
    if spec.speaking {
        stroke_rr(pm, r, radius, theme::SPEAKING, 2.0);
    }
    // name label (bottom-left) and muted badge (bottom-right)
    let lw = text.width(&spec.name, 12.0 * u) + 12.0 * u;
    fill_rr(pm, R { x: r.x + 6.0 * u, y: r.y + r.h - 4.0 * u - 20.0 * u, w: lw, h: 20.0 * u }, 4.0 * u, [0, 0, 0], 128);
    text.draw(pm, &spec.name, r.x + 12.0 * u, r.y + r.h - 4.0 * u - 6.0 * u, 12.0 * u, [255, 255, 255], 0.0);
    if spec.muted && !spec.content {
        let c = R { x: r.x + r.w - 4.0 * u - 20.0 * u, y: r.y + r.h - 4.0 * u - 20.0 * u, w: 20.0 * u, h: 20.0 * u };
        if let Some(p) = PathBuilder::from_circle(c.x + 10.0 * u, c.y + 10.0 * u, 10.0 * u) {
            pm.fill_path(&p, &paint([0, 0, 0], 128), FillRule::Winding, Transform::identity(), None);
        }
        icon::draw(pm, icons::MIC_OFF, c.x + 4.0 * u, c.y + 4.0 * u, 12.0 * u, [255, 255, 255]);
    }
    // hover overlay: bg-black/40 with the tile's buttons
    if hover {
        fill_rr(pm, r, radius, [0, 0, 0], 102);
        let btns = tile_buttons(spec);
        for (b, br) in btns.iter().zip(tile_button_rects(r, btns.len(), u)) {
            let disabled = match b {
                TileBtn::Mute => spec.muted,
                TileBtn::CameraOff => spec.video_key.is_none(),
                TileBtn::Pin => false,
            };
            let a = if disabled { 115 } else { 230 };
            fill_rr(pm, br, radius, theme::SURFACE, a);
            let (shapes, col) = match b {
                TileBtn::Mute => (icons::MIC_OFF, theme::INK),
                TileBtn::CameraOff => (icons::VIDEO_OFF, theme::INK),
                TileBtn::Pin => (if pinned { icons::MINIMIZE } else { icons::MAXIMIZE }, if pinned { theme::BRAND } else { theme::INK }),
            };
            let col = if disabled { mix(col, theme::SURFACE, 0.5) } else { col };
            icon::draw(pm, shapes, br.x + (br.w - 12.0 * u) / 2.0, br.y + (br.h - 12.0 * u) / 2.0, 12.0 * u, col);
        }
    }
}

// ---- interaction

/// Start dragging the dock by its header (mini mode only).
pub fn begin_drag(st: &mut State, text: &mut Text, px: f32, py: f32, w: f32, h: f32) {
    if st.dock.full {
        return;
    }
    if let Some(l) = layout(st, text, w, h) {
        st.dock.drag = Some(Drag { grab: (px - l.dock.x, py - l.dock.y), pos: (l.dock.x, l.dock.y) });
    }
}

pub fn drag_to(st: &mut State, px: f32, py: f32) {
    if let Some(d) = &mut st.dock.drag {
        d.pos = (px - d.grab.0, py - d.grab.1);
        st.dirty = true;
    }
}

/// Drop: snap to the slot nearest the dock's centre.
pub fn end_drag(st: &mut State, w: f32, h: f32) {
    if let Some(d) = st.dock.drag.take() {
        let u = st.ui;
        let (sb, right) = st.map_edges(w);
        let (dw, dh) = ((500.0 * u).min(0.7 * (right - sb)), (300.0 * u).min(0.7 * h));
        st.dock.slot = nearest_slot(d.pos.0 - sb + dw / 2.0, d.pos.1 + dh / 2.0, right - sb, h);
        st.dirty = true;
    }
}

/// Track which tile the pointer is over (drives the hover overlay).
pub fn update_hover(st: &mut State, text: &mut Text, px: f32, py: f32, w: f32, h: f32) {
    let new = match hit(st, text, px, py, w, h) {
        Some(Hit::Tile(id)) | Some(Hit::TileBtn(id, _)) => Some(id),
        _ => None,
    };
    if new != st.dock.hover.tile {
        st.dock.hover.tile = new;
        st.dirty = true;
    }
}

/// Mouse wheel over the dock: scrolls the pinned layout's sidebar. Returns true if the wheel was consumed.
pub fn wheel(st: &mut State, text: &mut Text, px: f32, py: f32, w: f32, h: f32, dy: f32) -> bool {
    let Some(l) = layout(st, text, w, h) else { return false };
    if !l.dock.contains(px, py) {
        return false;
    }
    if l.side.is_some_and(|s| s.contains(px, py)) {
        st.dock.scroll = (st.dock.scroll - dy.signum() * 40.0 * st.ui).clamp(0.0, l.max_scroll);
        st.dirty = true;
    }
    true
}
