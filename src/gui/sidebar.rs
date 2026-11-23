//! The people list (top of the left sidebar, or a whole panel of a narrow window): everyone in the office: people in meetings (grouped by meeting zone), other people who are online, and people who
//! are offline. Clicking a person or a meeting opens a small menu with Join (walk there), Wave and Message.
use crate::app::State;
use crate::gui::meeting_ui::R;
use crate::gui::panels;
use crate::gui::render::{paint, Text};
use crate::gui::theme::*;
use tiny_skia::{FillRule, PathBuilder, PixmapMut, PixmapPaint, Stroke, Transform};
pub use crate::directory::*;


pub const WIDTH: f32 = 280.0;
/// Width of the collapsed sidebar: one column of avatars.
pub const NARROW: f32 = 56.0;
const HEADER: f32 = 56.0;
/// The collapsed header: the expand button, and under it the unread counter when there is one.
const NARROW_HEADER: f32 = 46.0;
const NARROW_HEADER_UNREAD: f32 = 80.0;
const PAD: f32 = 12.0;

pub fn full_width(st: &State) -> f32 {
    if st.sidebar.collapsed { NARROW * st.ui } else { WIDTH * st.ui }
}

/// Avatars only: the collapsed sidebar of a wide window.
fn narrow(st: &State) -> bool {
    st.sidebar.collapsed && !st.sidebar.mobile
}

fn row_h(r: &Row, collapsed: bool) -> f32 {
    match (r, collapsed) {
        (Row::Section(_), false) => 30.0,
        (Row::Section(_), true) => 17.0,
        (Row::Wave { .. }, false) => 66.0,
        (Row::Wave { .. }, true) => 40.0,
        _ => 34.0,
    }
}

pub struct Layout {
    pub panel: R,
    /// The collapse / expand button (none on a narrow window).
    pub toggle: R,
    /// The unread counter of the collapsed sidebar: shows the chat.
    pub chat: R,
    pub list: R,
    pub items: Vec<(Row, R)>,
    pub max_scroll: f32,
}

const NONE: R = R { x: -1.0, y: -1.0, w: 0.0, h: 0.0 };

pub fn layout(st: &State, w: f32, h: f32) -> Layout {
    let (u, narrow) = (st.ui, narrow(st));
    let panel = panels::frame(st, w, h).users;
    let (y0, mobile) = (panel.y, st.sidebar.mobile);
    let (toggle, chat, header) = if narrow {
        let bw = panel.w - 2.0 * PAD * u;
        let b = |y: f32| R { x: PAD * u, y: y0 + y * u, w: bw, h: 28.0 * u };
        if st.unread_total() > 0 { (b(12.0), b(46.0), NARROW_HEADER_UNREAD) } else { (b(12.0), NONE, NARROW_HEADER) }
    } else {
        let toggle = if mobile { NONE } else { R { x: panel.w - (PAD + 24.0) * u, y: y0 + 10.0 * u, w: 24.0 * u, h: 24.0 * u } };
        (toggle, NONE, HEADER)
    };
    let list = R { x: 0.0, y: y0 + header * u, w: panel.w, h: (panel.h - header * u).max(0.0) };
    let rows = rows(st);
    let total: f32 = rows.iter().map(|r| row_h(r, narrow) * u).sum::<f32>() + 8.0 * u;
    let max_scroll = (total - list.h).max(0.0);
    let scroll = st.sidebar.scroll.clamp(0.0, max_scroll);
    let mut y = list.y - scroll + 4.0 * u;
    let items = rows
        .into_iter()
        .map(|r| {
            let rh = row_h(&r, narrow) * u;
            let rect = R { x: 0.0, y, w: panel.w, h: rh };
            y += rh;
            (r, rect)
        })
        .collect();
    Layout { panel, toggle, chat, list, items, max_scroll }
}

/// The buttons of a Menu row, side by side.
fn menu_buttons(rect: &R, u: f32, buttons: &[MenuBtn]) -> Vec<(MenuBtn, R)> {
    let gap = 6.0 * u;
    let n = buttons.len().max(1) as f32;
    let bw = (rect.w - 2.0 * PAD * u - (n - 1.0) * gap) / n;
    let (y, h) = (rect.y + 3.0 * u, rect.h - 6.0 * u);
    buttons.iter().enumerate().map(|(i, b)| (*b, R { x: PAD * u + i as f32 * (bw + gap), y, w: bw, h })).collect()
}

/// Accept and Dismiss inside a Wave row.
fn wave_buttons(rect: &R, u: f32) -> (R, R) {
    let bw = (rect.w - 2.0 * PAD * u - 8.0 * u) / 2.0;
    let (y, h) = (rect.y + rect.h - 30.0 * u, 24.0 * u);
    (R { x: PAD * u, y, w: bw, h }, R { x: PAD * u + bw + 8.0 * u, y, w: bw, h })
}

/// The floating desk button at the top left of the map (empty when not shown): only while we have a desk in this
/// room and are not at it, and the map is on screen (not covered by settings on a narrow window, nor by a full dock).
pub fn map_desk(st: &State, w: f32, h: f32) -> R {
    if st.dock.tile_full || st.login.is_some() || (st.sidebar.mobile && st.profile.is_some()) || !st.away_from_desk() {
        return NONE;
    }
    if crate::meeting::visible(st) && (st.dock.full || st.sidebar.mobile) {
        return NONE;
    }
    let f = panels::frame(st, w, h);
    if !f.main_shown || panels::empty(&f.main) {
        return NONE;
    }
    let (u, s) = (st.ui, 40.0 * st.ui);
    R { x: f.main.x + 10.0 * u, y: f.main.y + 10.0 * u, w: s, h: s }
}

pub fn draw_map_desk(st: &State, pm: &mut PixmapMut<'_>) {
    let r = map_desk(st, pm.width() as f32, pm.height() as f32);
    if panels::empty(&r) {
        return;
    }
    let u = st.ui;
    fill(pm, &r, r.w / 2.0, SURFACE, 235);
    outline(pm, &r, r.w / 2.0, LINE);
    let s = 20.0 * u;
    crate::gui::icon::draw(pm, crate::gui::icons::TABLE, r.x + (r.w - s) / 2.0, r.y + (r.h - s) / 2.0, s, INK);
}

pub fn hit(st: &State, px: f32, py: f32, w: f32, h: f32) -> Option<Hit> {
    if st.dock.tile_full || st.login.is_some() {
        return None;
    }
    if map_desk(st, w, h).contains(px, py) {
        return Some(Hit::Desk);
    }
    let l = layout(st, w, h);
    if panels::empty(&l.panel) || !l.panel.contains(px, py) {
        return None;
    }
    if l.toggle.contains(px, py) {
        return Some(Hit::Collapse);
    }
    if l.chat.contains(px, py) {
        return Some(Hit::Chat);
    }
    if l.list.contains(px, py) {
        for (row, rect) in &l.items {
            if !rect.contains(px, py) {
                continue;
            }
            return Some(match row {
                Row::Group { key, .. } => Hit::Row(key.clone()),
                Row::Person { p, .. } if !p.me => Hit::Row(p.id.clone()),
                Row::Menu { key, buttons } => menu_buttons(rect, st.ui, buttons)
                    .into_iter()
                    .find(|(_, r)| r.contains(px, py))
                    .map_or(Hit::Panel, |(b, _)| match b {
                        MenuBtn::Join { enabled: true } => Hit::Join(key.clone()),
                        MenuBtn::Join { enabled: false } => Hit::Panel,
                        MenuBtn::Wave => Hit::Wave(key.clone()),
                        MenuBtn::Message => Hit::Message(key.clone()),
                    }),
                // too narrow for Accept and Dismiss: widen the sidebar to show them
                Row::Wave { .. } if narrow(st) => Hit::Collapse,
                Row::Wave { from, .. } => {
                    let (accept, dismiss) = wave_buttons(rect, st.ui);
                    if accept.contains(px, py) {
                        Hit::WaveAccept(from.clone())
                    } else if dismiss.contains(px, py) {
                        Hit::WaveDismiss(from.clone())
                    } else {
                        Hit::Panel
                    }
                }
                _ => Hit::Panel,
            });
        }
    }
    Some(Hit::Panel)
}

pub fn wheel(st: &mut State, px: f32, py: f32, w: f32, h: f32, dy: f32) -> bool {
    let l = layout(st, w, h);
    if panels::empty(&l.panel) || !l.panel.contains(px, py) {
        return false;
    }
    st.sidebar.scroll = (st.sidebar.scroll - dy).clamp(0.0, l.max_scroll);
    st.dirty = true;
    true
}


pub fn fit(text: &mut Text, s: &str, px: f32, max_w: f32) -> String {
    if text.width(s, px) <= max_w {
        return s.to_string();
    }
    let mut out: String = s.to_string();
    while !out.is_empty() && text.width(&format!("{out}…"), px) > max_w {
        out.pop();
    }
    format!("{}…", out.trim_end())
}

pub fn fill(pm: &mut PixmapMut<'_>, r: &R, radius: f32, col: Rgb, a: u8) {
    if let Some(p) = crate::gui::icon::rounded_rect(r.x, r.y, r.w, r.h, radius) {
        let c = paint(col[0], col[1], col[2], a);
        pm.fill_path(&p, &c, FillRule::Winding, Transform::identity(), None);
    }
}

pub fn outline(pm: &mut PixmapMut<'_>, r: &R, radius: f32, col: Rgb) {
    if let Some(p) = crate::gui::icon::rounded_rect(r.x, r.y, r.w, r.h, radius) {
        pm.stroke_path(&p, &paint(col[0], col[1], col[2], 255), &Stroke { width: 1.0, ..Stroke::default() }, Transform::identity(), None);
    }
}

fn dot(pm: &mut PixmapMut<'_>, x: f32, y: f32, r: f32, col: Rgb) {
    if let Some(c) = PathBuilder::from_circle(x, y, r) {
        pm.fill_path(&c, &paint(col[0], col[1], col[2], 255), FillRule::Winding, Transform::identity(), None);
    }
}

/// Proportional memory of this whole process in MB, re-read at most once a second (smaps is not free).
pub fn pss_mb() -> f64 {
    thread_local!(static LAST: std::cell::Cell<Option<(std::time::Instant, u64)>> = const { std::cell::Cell::new(None) });
    LAST.with(|c| {
        let bytes = match c.get() {
            Some((t, b)) if t.elapsed() < std::time::Duration::from_secs(1) => b,
            _ => {
                let b = crate::gui::render::proc_mem().1;
                c.set(Some((std::time::Instant::now(), b)));
                b
            }
        };
        bytes as f64 / 1_048_576.0
    })
}

const AMBER: Rgb = [240, 170, 60];

/// A round picture (or a coloured circle with the first letter) of radius `r`, faded for someone offline.
#[allow(clippy::too_many_arguments)]
fn avatar(st: &State, pm: &mut PixmapMut<'_>, text: &mut Text, id: &str, name: &str, cx: f32, cy: f32, r: f32, alpha: f32) {
    match st.pics.get(id) {
        Some(pic) => {
            let s = (2.0 * r) / pic.w as f32;
            let tf = Transform::from_scale(s, s).post_translate(cx - r, cy - r);
            pm.draw_pixmap(0, 0, pic.px(), &PixmapPaint { opacity: alpha, quality: tiny_skia::FilterQuality::Bilinear, ..PixmapPaint::default() }, tf, None);
        }
        None => {
            let (a, b, c) = crate::gui::render::hue_rgb(id);
            if let Some(circle) = PathBuilder::from_circle(cx, cy, r) {
                pm.fill_path(&circle, &paint(a, b, c, (255.0 * alpha) as u8), FillRule::Winding, Transform::identity(), None);
            }
            let ch = name.trim().chars().next().map_or("?".to_string(), |c| c.to_uppercase().to_string());
            text.draw(pm, &ch, cx, cy + r * 0.38, r * 1.1, [255, 255, 255], 0.5);
        }
    }
}

/// A person's avatar with the online / away dot.
fn person_avatar(st: &State, pm: &mut PixmapMut<'_>, text: &mut Text, p: &Person, cx: f32, cy: f32, r: f32) {
    avatar(st, pm, text, &p.id, &p.name, cx, cy, r, if p.online { 1.0 } else { 0.5 });
    if p.online {
        dot(pm, cx + r * 0.75, cy + r * 0.75, 3.5 * st.ui, if p.away { AMBER } else { SPEAKING });
    }
    if p.unread > 0 {
        panels::unread_mark(pm, cx + r * 0.8, cy - r * 0.8, st.ui);
    }
}

/// The row of the collapsed sidebar under the mouse, if it is a person, a meeting or a wave.
pub fn hovered(st: &State, w: f32, h: f32) -> Option<(Row, R)> {
    let (px, py) = st.sidebar.hover?;
    if !narrow(st) || st.login.is_some() || st.dock.tile_full {
        return None;
    }
    let l = layout(st, w, h);
    if !l.list.contains(px, py) {
        return None;
    }
    l.items.into_iter().find(|(r, rect)| rect.contains(px, py) && matches!(r, Row::Person { .. } | Row::Group { .. } | Row::Wave { .. }))
}

/// The mouse moved: remember where, and redraw when it reaches another row of the collapsed sidebar.
pub fn update_hover(st: &mut State, px: f32, py: f32, w: f32, h: f32) {
    let before = hovered(st, w, h).map(|(r, _)| r);
    st.sidebar.hover = Some((px, py));
    if hovered(st, w, h).map(|(r, _)| r) != before {
        st.dirty = true;
    }
}

/// The card next to the hovered avatar of the collapsed sidebar: the name and whatever else the wide sidebar shows.
pub fn draw_hover_card(st: &State, pm: &mut PixmapMut<'_>, text: &mut Text, now_ms: i64) {
    let Some((row, rect)) = hovered(st, pm.width() as f32, pm.height() as f32) else { return };
    let u = st.ui;
    let (title, lines): (String, Vec<String>) = match &row {
        Row::Person { p, .. } => {
            let mut v = vec![if !p.online {
                format!("Offline · last seen {}", ago(now_ms, p.last_online))
            } else if p.me {
                format!("You · {}", if p.away { "away" } else { "available" })
            } else if p.away {
                "Away".into()
            } else {
                "Available".into()
            }];
            if let Some(z) = &p.zone {
                v.push(format!("In {}", zone_title(st, z)));
            }
            if p.unread > 0 {
                v.push(format!("{} unread message{}", p.unread, if p.unread == 1 { "" } else { "s" }));
            }
            (p.name.clone(), v)
        }
        Row::Group { key, title, count, unread } => {
            let zone = key.strip_prefix("zone:");
            let mut names: Vec<String> = people(st).into_iter().filter(|p| p.online && p.zone.as_deref() == zone).map(|p| p.name).collect();
            names.sort_by_key(|n| n.to_lowercase());
            let mut v = vec![format!("{count} people in this meeting")];
            v.extend(names.into_iter().take(8));
            if *count > 8 {
                v.push(format!("and {} more", count - 8));
            }
            if *unread > 0 {
                v.push(format!("{} unread message{}", unread, if *unread == 1 { "" } else { "s" }));
            }
            (title.clone(), v)
        }
        Row::Wave { name, .. } => (format!("{name} waved at you"), vec!["Click to answer".into()]),
        _ => return,
    };
    let (pad, line_h, max_w) = (10.0 * u, 16.0 * u, 260.0 * u);
    let title = fit(text, &title, 13.0 * u, max_w);
    let lines: Vec<String> = lines.iter().map(|l| fit(text, l, 12.0 * u, max_w)).collect();
    let title_w = text.width(&title, 13.0 * u);
    let w = lines.iter().map(|l| text.width(l, 12.0 * u)).fold(title_w, f32::max) + 2.0 * pad;
    let ch = 2.0 * pad + 18.0 * u + lines.len() as f32 * line_h - 4.0 * u;
    let y = (rect.y + rect.h / 2.0 - ch / 2.0).clamp(4.0 * u, (pm.height() as f32 - ch - 4.0 * u).max(4.0 * u));
    let card = R { x: rect.w + 6.0 * u, y, w, h: ch };
    for (i, a) in [(4.0, 18u8), (2.0, 26)] {
        fill(pm, &R { x: card.x - i * u, y: card.y - i * u * 0.3, w: card.w + 2.0 * i * u, h: card.h + 2.0 * i * u }, 8.0 * u + i * u, [0, 0, 0], a);
    }
    fill(pm, &card, 8.0 * u, SURFACE, 255);
    outline(pm, &card, 8.0 * u, LINE);
    text.draw(pm, &title, card.x + pad, card.y + pad + 12.0 * u, 13.0 * u, INK, 0.0);
    for (i, l) in lines.iter().enumerate() {
        text.draw(pm, l, card.x + pad, card.y + pad + 18.0 * u + (i as f32 + 1.0) * line_h - 4.0 * u, 12.0 * u, INK_MUTED, 0.0);
    }
}

/// A row of the collapsed sidebar: avatars only, a line for a section, the head count for a meeting.
fn draw_narrow_row(st: &State, pm: &mut PixmapMut<'_>, text: &mut Text, row: &Row, rect: &R) {
    let (u, mid) = (st.ui, rect.y + rect.h / 2.0);
    let (cx, r) = (rect.w / 2.0, 11.0 * u);
    let card = R { x: 6.0 * u, y: rect.y + 2.0 * u, w: rect.w - 12.0 * u, h: rect.h - 4.0 * u };
    match row {
        Row::Section(_) => fill(pm, &R { x: PAD * u, y: mid, w: rect.w - 2.0 * PAD * u, h: 1.0 }, 0.0, LINE, 255),
        Row::Group { count, unread, .. } => {
            fill(pm, &card, 8.0 * u, MUTED, 255);
            dot(pm, card.x + 7.0 * u, mid, 3.0 * u, BRAND);
            text.draw(pm, &count.to_string(), cx + 4.0 * u, mid + 4.5 * u, 12.0 * u, INK, 0.5);
            if *unread > 0 {
                panels::unread_mark(pm, card.x + card.w - 2.0 * u, card.y + 2.0 * u, u);
            }
        }
        Row::Person { p, .. } => person_avatar(st, pm, text, p, cx, mid, r),
        Row::Wave { from, name } => {
            fill(pm, &card, 8.0 * u, MUTED, 255);
            outline(pm, &card, 8.0 * u, BRAND);
            avatar(st, pm, text, from, name, cx, mid, r, 1.0);
        }
        Row::Menu { .. } => {}
    }
}

pub fn draw(st: &State, pm: &mut PixmapMut<'_>, text: &mut Text, now_ms: i64) {
    if st.login.is_some() || st.dock.tile_full {
        return;
    }
    let (u, w, h, narrow) = (st.ui, pm.width() as f32, pm.height() as f32, narrow(st));
    let l = layout(st, w, h);
    if panels::empty(&l.panel) {
        return;
    }
    fill(pm, &l.panel, 0.0, SURFACE, 255);
    let bottom = l.list.y + l.list.h;

    // rows, clipped to the list area by skipping what falls outside it (what sticks out below is covered by the
    // chat or the nav, drawn later)
    for (row, rect) in &l.items {
        if rect.y + rect.h < l.list.y || rect.y > bottom {
            continue;
        }
        if narrow {
            draw_narrow_row(st, pm, text, row, rect);
            continue;
        }
        let mid = rect.y + rect.h / 2.0;
        match row {
            Row::Section(t) => text.draw(pm, t, PAD * u, rect.y + rect.h - 8.0 * u, 11.0 * u, INK_FAINT, 0.0),
            Row::Group { key, title, count, unread } => {
                let inner = R { x: 6.0 * u, y: rect.y + 2.0 * u, w: rect.w - 12.0 * u, h: rect.h - 4.0 * u };
                fill(pm, &inner, 8.0 * u, if st.sidebar.open.as_deref() == Some(key) { LINE } else { MUTED }, 255);
                let (dx, r) = (PAD * u + 8.0 * u, 4.0 * u);
                dot(pm, dx, mid, r, BRAND);
                if *unread > 0 {
                    panels::unread_mark(pm, dx + 5.0 * u, mid - 5.0 * u, u);
                }
                let name = fit(text, title, 13.0 * u, rect.w - 90.0 * u);
                text.draw(pm, &name, PAD * u + 22.0 * u, mid + 4.5 * u, 13.0 * u, INK, 0.0);
                text.draw(pm, &count.to_string(), rect.w - PAD * u - 6.0 * u, mid + 4.5 * u, 12.0 * u, INK_MUTED, 1.0);
            }
            Row::Person { p, indent } => {
                let x0 = (PAD + if *indent { 14.0 } else { 0.0 }) * u;
                if st.sidebar.open.as_deref() == Some(p.id.as_str()) {
                    fill(pm, &R { x: 6.0 * u, y: rect.y + 2.0 * u, w: rect.w - 12.0 * u, h: rect.h - 4.0 * u }, 8.0 * u, MUTED, 255);
                }
                let r = 11.0 * u;
                let cx = x0 + r;
                person_avatar(st, pm, text, p, cx, mid, r);
                let right = if !p.online {
                    ago(now_ms, p.last_online)
                } else if p.me {
                    "you".into()
                } else if p.away {
                    "away".into()
                } else {
                    String::new()
                };
                let rw = text.width(&right, 11.0 * u);
                let name = fit(text, &p.name, 13.0 * u, rect.w - (x0 + 2.0 * r + 12.0 * u + 14.0 * u) - rw);
                text.draw(pm, &name, x0 + 2.0 * r + 10.0 * u, mid + 4.5 * u, 13.0 * u, if p.online { INK } else { INK_FAINT }, 0.0);
                text.draw(pm, &right, rect.w - PAD * u - 6.0 * u, mid + 4.0 * u, 11.0 * u, INK_FAINT, 1.0);
            }
            Row::Menu { key, buttons } => {
                for (b, r) in menu_buttons(rect, u, buttons) {
                    let (label, primary) = match b {
                        MenuBtn::Join { enabled } => ("Join", enabled),
                        MenuBtn::Wave => ("Wave", false),
                        MenuBtn::Message if key.starts_with("zone:") => ("Chat", false),
                        MenuBtn::Message => ("Message", false),
                    };
                    let dead = matches!(b, MenuBtn::Join { enabled: false });
                    if primary {
                        fill(pm, &r, 6.0 * u, BRAND, 255);
                    } else {
                        outline(pm, &r, 6.0 * u, LINE);
                    }
                    let col = if primary { ON_BRAND } else if dead { INK_FAINT } else { INK_MUTED };
                    text.draw(pm, label, r.x + r.w / 2.0, r.y + r.h / 2.0 + 4.5 * u, 12.0 * u, col, 0.5);
                }
            }
            Row::Wave { name, .. } => {
                let card = R { x: 6.0 * u, y: rect.y + 2.0 * u, w: rect.w - 12.0 * u, h: rect.h - 4.0 * u };
                fill(pm, &card, 8.0 * u, MUTED, 255);
                let who = fit(text, &format!("{name} waved at you"), 13.0 * u, rect.w - 2.0 * PAD * u - 8.0 * u);
                text.draw(pm, &who, PAD * u + 4.0 * u, rect.y + 20.0 * u, 13.0 * u, INK, 0.0);
                let (accept, dismiss) = wave_buttons(rect, u);
                fill(pm, &accept, 6.0 * u, BRAND, 255);
                text.draw(pm, "Accept", accept.x + accept.w / 2.0, accept.y + accept.h / 2.0 + 4.5 * u, 12.0 * u, ON_BRAND, 0.5);
                outline(pm, &dismiss, 6.0 * u, LINE);
                text.draw(pm, "Dismiss", dismiss.x + dismiss.w / 2.0, dismiss.y + dismiss.h / 2.0 + 4.5 * u, 12.0 * u, INK_MUTED, 0.5);
            }
        }
    }

    // header on top of the scrolled rows
    fill(pm, &R { x: 0.0, y: l.panel.y, w: l.panel.w, h: l.list.y - l.panel.y }, 0.0, SURFACE, 255);
    let s = 14.0 * u;
    if !panels::empty(&l.toggle) {
        outline(pm, &l.toggle, 6.0 * u, LINE);
        let chevron = if narrow { crate::gui::icons::CHEVRON_RIGHT } else { crate::gui::icons::CHEVRON_LEFT };
        crate::gui::icon::draw(pm, chevron, l.toggle.x + (l.toggle.w - s) / 2.0, l.toggle.y + (l.toggle.h - s) / 2.0, s, INK_MUTED);
    }
    if narrow {
        // the only thing about the chat a collapsed sidebar shows: how many messages wait
        let unread = st.unread_total();
        if unread > 0 && !panels::empty(&l.chat) {
            fill(pm, &l.chat, 6.0 * u, BRAND, 255);
            let label = if unread > 99 { "99+".to_string() } else { unread.to_string() };
            text.draw(pm, &label, l.chat.x + l.chat.w / 2.0, l.chat.y + l.chat.h / 2.0 + 4.5 * u, 12.0 * u, ON_BRAND, 0.5);
        }
        return;
    }
    let y0 = l.panel.y;
    let room = if st.room.is_empty() { "SoWorker".to_string() } else { st.room.clone() };
    let room_w = if panels::empty(&l.toggle) { l.panel.w - 2.0 * PAD * u } else { l.toggle.x - (PAD + 6.0) * u };
    let room = fit(text, &room, 15.0 * u, room_w);
    text.draw(pm, &room, PAD * u, y0 + 26.0 * u, 15.0 * u, INK, 0.0);
    let online = people(st).iter().filter(|p| p.online).count();
    let sub = if st.status.is_empty() { format!("{online} online{}", if st.dry { " · dry run" } else { "" }) } else { st.status.clone() };
    text.draw(pm, &sub, PAD * u, y0 + 44.0 * u, 12.0 * u, INK_MUTED, 0.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::directory::tests::state;

    const W: f32 = 1000.0;

    #[test]
    fn rows_react_to_clicks() {
        let mut st = state();
        st.ui = 1.0;
        st.sidebar.collapsed = false;
        let l = layout(&st, W, 700.0);
        let (_, ana) = l.items.iter().find(|(r, _)| matches!(r, Row::Person { p, .. } if p.id == "d")).unwrap();
        assert_eq!(hit(&st, 100.0, ana.y + 5.0, W, 700.0), Some(Hit::Row("d".into())));
        let (_, me) = l.items.iter().find(|(r, _)| matches!(r, Row::Person { p, .. } if p.me)).unwrap();
        assert_eq!(hit(&st, 100.0, me.y + 5.0, W, 700.0), Some(Hit::Panel), "I cannot open a menu on myself");
        let (_, eve) = l.items.iter().find(|(r, _)| matches!(r, Row::Person { p, .. } if p.id == "e")).unwrap();
        if eve.y + 5.0 < l.list.y + l.list.h {
            assert_eq!(hit(&st, 100.0, eve.y + 5.0, W, 700.0), Some(Hit::Row("e".into())), "offline people can be messaged");
        }
        assert_eq!(hit(&st, 600.0, 300.0, W, 700.0), None, "the map starts right of the panel");
        assert_eq!(hit(&st, 100.0, 690.0, W, 700.0), None, "the chat and the nav are below the people list");
        assert_eq!(hit(&st, l.toggle.x + 2.0, l.toggle.y + 2.0, W, 700.0), Some(Hit::Collapse));
    }

    #[test]
    fn collapsed_sidebar_is_a_column_of_avatars() {
        let mut st = state();
        st.ui = 1.0;
        st.sidebar.collapsed = true;
        assert_eq!(full_width(&st), NARROW);
        let l = layout(&st, W, 700.0);
        assert!(l.items.iter().all(|(_, r)| r.w == NARROW));
        let (_, di) = l.items.iter().find(|(r, _)| matches!(r, Row::Person { p, .. } if p.id == "d")).unwrap();
        assert_eq!(hit(&st, NARROW / 2.0, di.y + 5.0, W, 700.0), Some(Hit::Row("d".into())));
        assert_eq!(hit(&st, l.toggle.x + 2.0, l.toggle.y + 2.0, W, 700.0), Some(Hit::Collapse));
        assert_eq!(hit(&st, 100.0, 300.0, W, 700.0), None, "the map starts right of the narrow panel");
        update_hover(&mut st, NARROW / 2.0, di.y + 5.0, W, 700.0);
        assert!(matches!(hovered(&st, W, 700.0), Some((Row::Person { p, .. }, _)) if p.id == "d"), "hovering an avatar shows its card");
        st.sidebar.collapsed = false;
        assert!(hovered(&st, W, 700.0).is_none(), "the wide sidebar shows names, so no card");
    }

    #[test]
    fn the_desk_button_shows_only_away_from_our_desk() {
        let mut st = state();
        st.ui = 1.0;
        st.sidebar.collapsed = false;
        assert!(panels::empty(&map_desk(&st, W, 700.0)), "no desk here");
        st.room_def = "R".into();
        st.desks.insert("k".into(), (5000.0, 5000.0));
        st.desk_owners = Some(vec![crate::desks::Desk { room: "R".into(), desk: "k".into(), owner: "me".into() }]);
        let d = map_desk(&st, W, 700.0);
        let main = panels::frame(&st, W, 700.0).main;
        assert!(d.x < main.x + 20.0 && d.y < 20.0, "top left of the map");
        assert_eq!(hit(&st, d.x + 5.0, d.y + 5.0, W, 700.0), Some(Hit::Desk));
        st.sidebar.mobile = true;
        st.sidebar.view = View::Main;
        let d = map_desk(&st, 360.0, 700.0);
        assert!(d.x < 20.0 && d.y < panels::HEADER + 20.0, "top left of the map on a narrow window too");
        st.sidebar.view = View::Users;
        assert!(panels::empty(&map_desk(&st, 360.0, 700.0)), "not over the people list");
        st.sidebar.mobile = false;
        let me = st.users.get_mut("me").unwrap();
        (me.x, me.y) = (5300.0, 5300.0);
        assert!(!panels::empty(&map_desk(&st, W, 700.0)), "near the desk, but it is in no zone");
        st.world.zones.push(crate::world::Zone { id: "z".into(), x: 5200.0, y: 5200.0, sx: 300.0, sy: 300.0, meeting: true, name: None });
        assert!(panels::empty(&map_desk(&st, W, 700.0)), "in our desk's zone");
    }

    #[test]
    fn narrow_window_shows_the_list_as_a_whole_panel() {
        let mut st = state();
        st.ui = 1.0;
        st.sidebar.mobile = true;
        st.sidebar.view = View::Users;
        let l = layout(&st, 360.0, 700.0);
        assert_eq!(l.panel.w, 360.0);
        assert!(crate::gui::panels::empty(&l.toggle), "nothing to collapse");
        st.sidebar.view = View::Main;
        assert_eq!(hit(&st, 100.0, 300.0, 360.0, 700.0), None, "the people list is not on screen");
    }
}
