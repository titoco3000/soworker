//! The left sidebar: availability and chat buttons, then everyone in the office: people in meetings (grouped by
//! meeting zone), other people who are online, and people who are offline. Clicking an online person or a meeting
//! opens a small menu with Join (walk there) and Wave (not implemented yet).
use crate::app::State;
use crate::gui::meeting_ui::R;
use crate::gui::render::{paint, Text};
use crate::gui::theme::*;
use tiny_skia::{FillRule, PathBuilder, PixmapMut, PixmapPaint, Stroke, Transform};
pub use crate::directory::*;


pub const WIDTH: f32 = 252.0;
/// Width of the collapsed sidebar: one column of avatars.
pub const NARROW: f32 = 56.0;
const HEADER: f32 = 92.0;
/// The collapsed header stacks its three buttons.
const NARROW_HEADER: f32 = 116.0;
pub const FOOTER: f32 = 28.0;
const PAD: f32 = 12.0;

pub fn width(st: &State) -> f32 {
    if st.login.is_some() || st.dock.tile_full { 0.0 } else { full_width(st) }
}

fn full_width(st: &State) -> f32 {
    if st.sidebar.collapsed { NARROW * st.ui } else { WIDTH * st.ui }
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
    /// The collapse / expand button.
    pub toggle: R,
    pub away: R,
    pub chat: R,
    /// The user icon in the footer, next to the memory use: opens the profile panel.
    pub profile: R,
    /// The "Terminal" button in the footer.
    pub term: R,
    pub list: R,
    pub items: Vec<(Row, R)>,
    pub max_scroll: f32,
}

pub fn layout(st: &State, h: f32) -> Layout {
    let (u, narrow) = (st.ui, st.sidebar.collapsed);
    let panel = R { x: 0.0, y: 0.0, w: full_width(st), h };
    let (toggle, away, chat, header) = if narrow {
        // toggle, availability and chat stacked in one column
        let bw = panel.w - 2.0 * PAD * u;
        let b = |y: f32| R { x: PAD * u, y: y * u, w: bw, h: 28.0 * u };
        (b(12.0), b(46.0), b(80.0), NARROW_HEADER)
    } else {
        let gap = 8.0 * u;
        let bw = (panel.w - 2.0 * PAD * u - gap) / 2.0;
        let by = (HEADER - 36.0) * u;
        let toggle = R { x: panel.w - (PAD + 24.0) * u, y: 10.0 * u, w: 24.0 * u, h: 24.0 * u };
        (toggle, R { x: PAD * u, y: by, w: bw, h: 28.0 * u }, R { x: PAD * u + bw + gap, y: by, w: bw, h: 28.0 * u }, HEADER)
    };
    let list = R { x: 0.0, y: header * u, w: panel.w, h: (h - (header + FOOTER) * u).max(0.0) };
    let profile_x = if narrow { (panel.w - 22.0 * u) / 2.0 } else { (PAD - 4.0) * u };
    let profile = R { x: profile_x, y: list.y + list.h + 3.0 * u, w: 22.0 * u, h: (FOOTER - 6.0) * u };
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
    let term = if cfg!(feature = "tui") && !narrow { R { x: panel.w - (PAD + 84.0) * u, y: list.y + list.h + 3.0 * u, w: 84.0 * u, h: (FOOTER - 6.0) * u } } else { R { x: -1.0, y: -1.0, w: 0.0, h: 0.0 } };
    Layout { panel, toggle, away, chat, profile, term, list, items, max_scroll }
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

pub fn hit(st: &State, px: f32, py: f32, h: f32) -> Option<Hit> {
    if st.dock.tile_full {
        return None;
    }
    let l = layout(st, h);
    if !l.panel.contains(px, py) {
        return None;
    }
    if l.toggle.contains(px, py) {
        return Some(Hit::Collapse);
    }
    if l.away.contains(px, py) {
        return Some(Hit::Away);
    }
    if l.chat.contains(px, py) {
        return Some(Hit::Chat);
    }
    if l.profile.contains(px, py) {
        return Some(Hit::Profile);
    }
    if l.term.w > 0.0 && l.term.contains(px, py) {
        return Some(Hit::Terminal);
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
                Row::Wave { .. } if st.sidebar.collapsed => Hit::Collapse,
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

pub fn wheel(st: &mut State, px: f32, py: f32, h: f32, dy: f32) -> bool {
    let l = layout(st, h);
    if !l.panel.contains(px, py) {
        return false;
    }
    st.sidebar.scroll = (st.sidebar.scroll - dy.signum() * 40.0 * st.ui).clamp(0.0, l.max_scroll);
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
fn pss_mb() -> f64 {
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
}

/// The row of the collapsed sidebar under the mouse, if it is a person, a meeting or a wave.
pub fn hovered(st: &State, h: f32) -> Option<(Row, R)> {
    let (px, py) = st.sidebar.hover?;
    if !st.sidebar.collapsed || st.login.is_some() || st.dock.tile_full {
        return None;
    }
    let l = layout(st, h);
    if !l.list.contains(px, py) {
        return None;
    }
    l.items.into_iter().find(|(r, rect)| rect.contains(px, py) && matches!(r, Row::Person { .. } | Row::Group { .. } | Row::Wave { .. }))
}

/// The mouse moved: remember where, and redraw when it reaches another row of the collapsed sidebar.
pub fn update_hover(st: &mut State, px: f32, py: f32, h: f32) {
    let before = hovered(st, h).map(|(r, _)| r);
    st.sidebar.hover = Some((px, py));
    if hovered(st, h).map(|(r, _)| r) != before {
        st.dirty = true;
    }
}

/// The card next to the hovered avatar of the collapsed sidebar: the name and whatever else the wide sidebar shows.
pub fn draw_hover_card(st: &State, pm: &mut PixmapMut<'_>, text: &mut Text, now_ms: i64) {
    let Some((row, rect)) = hovered(st, pm.height() as f32) else { return };
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
        Row::Group { key, title, count } => {
            let zone = key.strip_prefix("zone:");
            let mut names: Vec<String> = people(st).into_iter().filter(|p| p.online && p.zone.as_deref() == zone).map(|p| p.name).collect();
            names.sort_by_key(|n| n.to_lowercase());
            let mut v = vec![format!("{count} people in this meeting")];
            v.extend(names.into_iter().take(8));
            if *count > 8 {
                v.push(format!("and {} more", count - 8));
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
        Row::Group { count, .. } => {
            fill(pm, &card, 8.0 * u, MUTED, 255);
            dot(pm, card.x + 7.0 * u, mid, 3.0 * u, BRAND);
            text.draw(pm, &count.to_string(), cx + 4.0 * u, mid + 4.5 * u, 12.0 * u, INK, 0.5);
        }
        Row::Person { p, .. } => {
            person_avatar(st, pm, text, p, cx, mid, r);
            if p.unread > 0 {
                dot(pm, cx + r * 0.8, mid - r * 0.8, 4.5 * u, BRAND);
            }
        }
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
    let (u, h, narrow) = (st.ui, pm.height() as f32, st.sidebar.collapsed);
    let l = layout(st, h);
    fill(pm, &l.panel, 0.0, SURFACE, 255);

    // rows, clipped to the list area by skipping what falls outside it
    for (row, rect) in &l.items {
        if rect.y + rect.h < l.list.y || rect.y > h {
            continue;
        }
        if narrow {
            draw_narrow_row(st, pm, text, row, rect);
            continue;
        }
        let mid = rect.y + rect.h / 2.0;
        match row {
            Row::Section(t) => text.draw(pm, t, PAD * u, rect.y + rect.h - 8.0 * u, 11.0 * u, INK_FAINT, 0.0),
            Row::Group { key, title, count } => {
                let inner = R { x: 6.0 * u, y: rect.y + 2.0 * u, w: rect.w - 12.0 * u, h: rect.h - 4.0 * u };
                fill(pm, &inner, 8.0 * u, if st.sidebar.open.as_deref() == Some(key) { LINE } else { MUTED }, 255);
                dot(pm, PAD * u + 8.0 * u, mid, 4.0 * u, BRAND);
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
                if p.unread > 0 {
                    let label = p.unread.to_string();
                    let bw = (text.width(&label, 11.0 * u) + 12.0 * u).max(18.0 * u);
                    let badge = R { x: rect.w - PAD * u - 6.0 * u - bw, y: mid - 8.0 * u, w: bw, h: 16.0 * u };
                    fill(pm, &badge, 8.0 * u, BRAND, 255);
                    text.draw(pm, &label, badge.x + bw / 2.0, badge.y + 12.0 * u, 11.0 * u, ON_BRAND, 0.5);
                }
                let right = if p.unread > 0 { String::new() } else { right };
                let rw = text.width(&right, 11.0 * u).max(if p.unread > 0 { 26.0 * u } else { 0.0 });
                let name = fit(text, &p.name, 13.0 * u, rect.w - (x0 + 2.0 * r + 12.0 * u + 14.0 * u) - rw);
                text.draw(pm, &name, x0 + 2.0 * r + 10.0 * u, mid + 4.5 * u, 13.0 * u, if p.online { INK } else { INK_FAINT }, 0.0);
                text.draw(pm, &right, rect.w - PAD * u - 6.0 * u, mid + 4.0 * u, 11.0 * u, INK_FAINT, 1.0);
            }
            Row::Menu { buttons, .. } => {
                for (b, r) in menu_buttons(rect, u, buttons) {
                    let (label, primary) = match b {
                        MenuBtn::Join { enabled } => ("Join", enabled),
                        MenuBtn::Wave => ("Wave", false),
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
    fill(pm, &R { x: 0.0, y: 0.0, w: l.panel.w, h: l.list.y }, 0.0, SURFACE, 255);
    outline(pm, &l.toggle, 6.0 * u, LINE);
    let s = 14.0 * u;
    let chevron = if narrow { crate::gui::icons::CHEVRON_RIGHT } else { crate::gui::icons::CHEVRON_LEFT };
    crate::gui::icon::draw(pm, chevron, l.toggle.x + (l.toggle.w - s) / 2.0, l.toggle.y + (l.toggle.h - s) / 2.0, s, INK_MUTED);
    if !narrow {
        let room = if st.room.is_empty() { "SoWorker".to_string() } else { st.room.clone() };
        let room = fit(text, &room, 15.0 * u, l.toggle.x - (PAD + 6.0) * u);
        text.draw(pm, &room, PAD * u, 26.0 * u, 15.0 * u, INK, 0.0);
        let online = people(st).iter().filter(|p| p.online).count();
        let sub = if st.status.is_empty() { format!("{online} online{}", if st.dry { " · dry run" } else { "" }) } else { st.status.clone() };
        text.draw(pm, &sub, PAD * u, 44.0 * u, 12.0 * u, INK_MUTED, 0.0);
    }
    let (label, on) = if st.available { ("Available", true) } else { ("Away", false) };
    if on {
        fill(pm, &l.away, 6.0 * u, [47, 111, 79], 255);
    } else {
        fill(pm, &l.away, 6.0 * u, [106, 47, 58], 255);
    }
    if narrow {
        dot(pm, l.away.x + l.away.w / 2.0, l.away.y + l.away.h / 2.0, 4.0 * u, if on { SPEAKING } else { INK });
    } else {
        text.draw(pm, label, l.away.x + l.away.w / 2.0, l.away.y + l.away.h / 2.0 + 4.5 * u, 12.0 * u, INK, 0.5);
    }
    let unread = st.unread_total();
    if st.chat.open {
        fill(pm, &l.chat, 6.0 * u, LINE, 255);
    }
    outline(pm, &l.chat, 6.0 * u, LINE);
    let chat_col = if unread > 0 { BRAND } else { INK_MUTED };
    if narrow {
        crate::gui::icon::draw(pm, crate::gui::icons::MESSAGE_CIRCLE, l.chat.x + (l.chat.w - s) / 2.0, l.chat.y + (l.chat.h - s) / 2.0, s, chat_col);
        if unread > 0 {
            dot(pm, l.chat.x + l.chat.w - 4.0 * u, l.chat.y + 4.0 * u, 3.5 * u, BRAND);
        }
    } else {
        let chat_label = if unread > 0 { format!("Chat ({unread})") } else { "Chat".to_string() };
        text.draw(pm, &chat_label, l.chat.x + l.chat.w / 2.0, l.chat.y + l.chat.h / 2.0 + 4.5 * u, 12.0 * u, chat_col, 0.5);
    }
    let footer = R { x: 0.0, y: l.list.y + l.list.h, w: l.panel.w, h: FOOTER * u };
    fill(pm, &footer, 0.0, SURFACE, 255);
    fill(pm, &R { h: 1.0, ..footer }, 0.0, LINE, 255);
    let open = st.profile.is_some();
    // a solid round button, brand-coloured while the profile panel is open
    fill(pm, &l.profile, l.profile.w.min(l.profile.h) / 2.0, if open { BRAND } else { LINE }, 255);
    crate::gui::icon::draw(pm, crate::gui::icons::USER, l.profile.x + (l.profile.w - s) / 2.0, l.profile.y + (l.profile.h - s) / 2.0, s, if open { ON_BRAND } else { INK });
    if !narrow {
        text.draw(pm, &format!("Memory {:.1} MB", pss_mb()), l.profile.x + l.profile.w + 6.0 * u, footer.y + footer.h / 2.0 + 4.0 * u, 11.0 * u, INK_FAINT, 0.0);
    }
    if l.term.w > 0.0 {
        outline(pm, &l.term, 6.0 * u, LINE);
        text.draw(pm, "Terminal", l.term.x + l.term.w / 2.0, l.term.y + l.term.h / 2.0 + 4.0 * u, 11.0 * u, INK_MUTED, 0.5);
    }
    let edge = R { x: l.panel.w - 1.0, y: 0.0, w: 1.0, h };
    fill(pm, &edge, 0.0, LINE, 255);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::directory::tests::state;

    #[test]
    fn rows_react_to_clicks() {
        let mut st = state();
        st.ui = 1.0;
        st.sidebar.collapsed = false;
        let l = layout(&st, 700.0);
        let (_, ana) = l.items.iter().find(|(r, _)| matches!(r, Row::Person { p, .. } if p.id == "d")).unwrap();
        assert_eq!(hit(&st, 100.0, ana.y + 5.0, 700.0), Some(Hit::Row("d".into())));
        let (_, me) = l.items.iter().find(|(r, _)| matches!(r, Row::Person { p, .. } if p.me)).unwrap();
        assert_eq!(hit(&st, 100.0, me.y + 5.0, 700.0), Some(Hit::Panel), "I cannot open a menu on myself");
        let (_, eve) = l.items.iter().find(|(r, _)| matches!(r, Row::Person { p, .. } if p.id == "e")).unwrap();
        assert_eq!(hit(&st, 100.0, eve.y + 5.0, 700.0), Some(Hit::Row("e".into())), "offline people can be messaged");
        assert_eq!(hit(&st, l.away.x + 2.0, l.away.y + 2.0, 700.0), Some(Hit::Away));
        assert_eq!(hit(&st, l.profile.x + 2.0, l.profile.y + 2.0, 700.0), Some(Hit::Profile));
        assert_eq!(hit(&st, 600.0, 300.0, 700.0), None, "the map starts right of the panel");
        assert_eq!(hit(&st, l.toggle.x + 2.0, l.toggle.y + 2.0, 700.0), Some(Hit::Collapse));
    }

    #[test]
    fn collapsed_sidebar_is_a_column_of_avatars() {
        let mut st = state();
        st.ui = 1.0;
        st.sidebar.collapsed = true;
        assert_eq!(width(&st), NARROW);
        let l = layout(&st, 700.0);
        assert!(l.items.iter().all(|(_, r)| r.w == NARROW));
        let (_, di) = l.items.iter().find(|(r, _)| matches!(r, Row::Person { p, .. } if p.id == "d")).unwrap();
        assert_eq!(hit(&st, NARROW / 2.0, di.y + 5.0, 700.0), Some(Hit::Row("d".into())));
        assert_eq!(hit(&st, l.toggle.x + 2.0, l.toggle.y + 2.0, 700.0), Some(Hit::Collapse));
        assert_eq!(hit(&st, 100.0, 300.0, 700.0), None, "the map starts right of the narrow panel");
        update_hover(&mut st, NARROW / 2.0, di.y + 5.0, 700.0);
        assert!(matches!(hovered(&st, 700.0), Some((Row::Person { p, .. }, _)) if p.id == "d"), "hovering an avatar shows its card");
        st.sidebar.collapsed = false;
        assert!(hovered(&st, 700.0).is_none(), "the wide sidebar shows names, so no card");
    }
}
