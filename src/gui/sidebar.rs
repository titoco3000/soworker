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
const HEADER: f32 = 92.0;
pub const FOOTER: f32 = 28.0;
const PAD: f32 = 12.0;

pub fn width(st: &State) -> f32 {
    if st.login.is_some() { 0.0 } else { WIDTH * st.ui }
}

fn row_h(r: &Row) -> f32 {
    match r {
        Row::Section(_) => 30.0,
        Row::Wave { .. } => 66.0,
        _ => 34.0,
    }
}

pub struct Layout {
    pub panel: R,
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
    let u = st.ui;
    let panel = R { x: 0.0, y: 0.0, w: WIDTH * u, h };
    let gap = 8.0 * u;
    let bw = (panel.w - 2.0 * PAD * u - gap) / 2.0;
    let by = (HEADER - 36.0) * u;
    let away = R { x: PAD * u, y: by, w: bw, h: 28.0 * u };
    let chat = R { x: PAD * u + bw + gap, y: by, w: bw, h: 28.0 * u };
    let list = R { x: 0.0, y: HEADER * u, w: panel.w, h: (h - (HEADER + FOOTER) * u).max(0.0) };
    let profile = R { x: (PAD - 4.0) * u, y: list.y + list.h + 3.0 * u, w: 22.0 * u, h: (FOOTER - 6.0) * u };
    let rows = rows(st);
    let total: f32 = rows.iter().map(|r| row_h(r) * u).sum::<f32>() + 8.0 * u;
    let max_scroll = (total - list.h).max(0.0);
    let scroll = st.sidebar.scroll.clamp(0.0, max_scroll);
    let mut y = list.y - scroll + 4.0 * u;
    let items = rows
        .into_iter()
        .map(|r| {
            let rh = row_h(&r) * u;
            let rect = R { x: 0.0, y, w: panel.w, h: rh };
            y += rh;
            (r, rect)
        })
        .collect();
    let term = if cfg!(feature = "tui") { R { x: panel.w - (PAD + 84.0) * u, y: list.y + list.h + 3.0 * u, w: 84.0 * u, h: (FOOTER - 6.0) * u } } else { R { x: -1.0, y: -1.0, w: 0.0, h: 0.0 } };
    Layout { panel, away, chat, profile, term, list, items, max_scroll }
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
    let l = layout(st, h);
    if !l.panel.contains(px, py) {
        return None;
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

pub fn draw(st: &State, pm: &mut PixmapMut<'_>, text: &mut Text, now_ms: i64) {
    if st.login.is_some() {
        return;
    }
    let (u, h) = (st.ui, pm.height() as f32);
    let l = layout(st, h);
    fill(pm, &l.panel, 0.0, SURFACE, 255);

    // rows, clipped to the list area by skipping what falls outside it
    for (row, rect) in &l.items {
        if rect.y + rect.h < l.list.y || rect.y > h {
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
                let (cx, alpha) = (x0 + r, if p.online { 1.0 } else { 0.5 });
                match st.pics.get(&p.id) {
                    Some(pic) => {
                        let s = (2.0 * r) / pic.w as f32;
                        let tf = Transform::from_scale(s, s).post_translate(cx - r, mid - r);
                        pm.draw_pixmap(0, 0, pic.px(), &PixmapPaint { opacity: alpha, quality: tiny_skia::FilterQuality::Bilinear, ..PixmapPaint::default() }, tf, None);
                    }
                    None => {
                        let (a, b, c) = crate::gui::render::hue_rgb(&p.id);
                        if let Some(circle) = PathBuilder::from_circle(cx, mid, r) {
                            pm.fill_path(&circle, &paint(a, b, c, (255.0 * alpha) as u8), FillRule::Winding, Transform::identity(), None);
                        }
                        let ch = p.name.trim().chars().next().map_or("?".to_string(), |c| c.to_uppercase().to_string());
                        text.draw(pm, &ch, cx, mid + r * 0.38, r * 1.1, [255, 255, 255], 0.5);
                    }
                }
                if p.online {
                    dot(pm, cx + r * 0.75, mid + r * 0.75, 3.5 * u, if p.away { AMBER } else { SPEAKING });
                }
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
    let room = if st.room.is_empty() { "SoWorker".to_string() } else { st.room.clone() };
    let room = fit(text, &room, 15.0 * u, l.panel.w - 2.0 * PAD * u);
    text.draw(pm, &room, PAD * u, 26.0 * u, 15.0 * u, INK, 0.0);
    let online = people(st).iter().filter(|p| p.online).count();
    let sub = if st.status.is_empty() { format!("{online} online{}", if st.dry { " · dry run" } else { "" }) } else { st.status.clone() };
    text.draw(pm, &sub, PAD * u, 44.0 * u, 12.0 * u, INK_MUTED, 0.0);
    let (label, on) = if st.available { ("Available", true) } else { ("Away", false) };
    if on {
        fill(pm, &l.away, 6.0 * u, [47, 111, 79], 255);
    } else {
        fill(pm, &l.away, 6.0 * u, [106, 47, 58], 255);
    }
    text.draw(pm, label, l.away.x + l.away.w / 2.0, l.away.y + l.away.h / 2.0 + 4.5 * u, 12.0 * u, INK, 0.5);
    let unread = st.unread_total();
    let chat_label = if unread > 0 { format!("Chat ({unread})") } else { "Chat".to_string() };
    if st.chat.open {
        fill(pm, &l.chat, 6.0 * u, LINE, 255);
    }
    outline(pm, &l.chat, 6.0 * u, LINE);
    text.draw(pm, &chat_label, l.chat.x + l.chat.w / 2.0, l.chat.y + l.chat.h / 2.0 + 4.5 * u, 12.0 * u, if unread > 0 { BRAND } else { INK_MUTED }, 0.5);
    let footer = R { x: 0.0, y: l.list.y + l.list.h, w: l.panel.w, h: FOOTER * u };
    fill(pm, &footer, 0.0, SURFACE, 255);
    fill(pm, &R { h: 1.0, ..footer }, 0.0, LINE, 255);
    let open = st.profile.is_some();
    if open {
        fill(pm, &l.profile, 6.0 * u, LINE, 255);
    }
    let s = 16.0 * u;
    crate::gui::icon::draw(pm, crate::gui::icons::USER, l.profile.x + (l.profile.w - s) / 2.0, l.profile.y + (l.profile.h - s) / 2.0, s, if open { INK } else { INK_MUTED });
    text.draw(pm, &format!("Memory {:.1} MB", pss_mb()), l.profile.x + l.profile.w + 6.0 * u, footer.y + footer.h / 2.0 + 4.0 * u, 11.0 * u, INK_FAINT, 0.0);
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
    }
}
