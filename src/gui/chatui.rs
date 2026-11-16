//! The chat panel (under the people list in the sidebar, or a whole panel of a narrow window): one conversation at a
//! time (the area chat of the meeting we are in, or a private chat picked in the people list), its messages, and a
//! one-line input. State and logic live in `chatstate.rs` (`impl State`), the transport is `chat.rs`.
use crate::app::State;
use crate::chat::ChatMsg;
use crate::gui::meeting_ui::R;
use crate::gui::panels;
use crate::gui::render::{paint, Text};
use crate::gui::theme::*;
use tiny_skia::{FillRule, PixmapMut, Stroke, Transform};
pub use crate::chatstate::*;


const HEADER: f32 = 40.0;
const INPUT_H: f32 = 52.0;
const PAD: f32 = 12.0;

#[derive(Clone, Debug, PartialEq)]
pub enum Hit {
    /// Back from a private chat to the area chat.
    Close,
    Input,
    Panel,
}

/// What the header calls the conversation on show.
fn title(st: &State) -> String {
    match &st.chat.tab {
        Tab::Area => match st.cur_zone_id.as_deref() {
            Some(z) if st.conversation().is_some() => crate::directory::zone_title(st, z),
            _ => "Area chat".into(),
        },
        Tab::Dm(p) => st.name_of(p, None),
    }
}

struct Layout {
    panel: R,
    close: R,
    input: R,
    msgs: R,
}

fn layout(st: &State, w: f32, h: f32) -> Layout {
    let u = st.ui;
    let panel = panels::frame(st, w, h).chat;
    let close = if matches!(st.chat.tab, Tab::Dm(_)) { R { x: panel.x + panel.w - 36.0 * u, y: panel.y, w: 36.0 * u, h: HEADER * u } } else { R { x: -1.0, y: -1.0, w: 0.0, h: 0.0 } };
    let bottom = panel.y + panel.h;
    let input = R { x: panel.x + PAD * u, y: bottom - (INPUT_H - 8.0) * u, w: panel.w - 2.0 * PAD * u, h: (INPUT_H - 16.0) * u };
    let msgs = R { x: panel.x, y: panel.y + HEADER * u, w: panel.w, h: (panel.h - (HEADER + INPUT_H) * u).max(0.0) };
    Layout { panel, close, input, msgs }
}

pub fn hit(st: &State, px: f32, py: f32, w: f32, h: f32) -> Option<Hit> {
    let l = layout(st, w, h);
    if panels::empty(&l.panel) || !l.panel.contains(px, py) {
        return None;
    }
    if !panels::empty(&l.close) && l.close.contains(px, py) {
        return Some(Hit::Close);
    }
    // with room for the input only when the panel is tall enough to show it
    Some(if l.msgs.h > 0.0 && l.input.contains(px, py) { Hit::Input } else { Hit::Panel })
}

pub fn wheel(st: &mut State, px: f32, py: f32, w: f32, h: f32, dy: f32) -> bool {
    let l = layout(st, w, h);
    if panels::empty(&l.panel) || !l.panel.contains(px, py) {
        return false;
    }
    st.chat.scroll.set((st.chat.scroll.get() + dy.signum() * 40.0 * st.ui).clamp(0.0, st.chat.max_scroll.get()));
    st.dirty = true;
    true
}

pub fn press(st: &mut State, hit: Hit) {
    match hit {
        Hit::Close => {
            st.select_tab(Tab::Area);
            st.chat.focus = false;
        }
        Hit::Input => st.chat.focus = true,
        Hit::Panel => {}
    }
    st.dirty = true;
}

/// Greedy word wrap; a word wider than the line is broken by characters.
pub fn wrap(text: &mut Text, s: &str, px: f32, max_w: f32) -> Vec<String> {
    let mut lines = Vec::new();
    for para in s.split('\n') {
        let mut cur = String::new();
        for word in para.split_whitespace() {
            let try_line = if cur.is_empty() { word.to_string() } else { format!("{cur} {word}") };
            if text.width(&try_line, px) <= max_w {
                cur = try_line;
                continue;
            }
            if !cur.is_empty() {
                lines.push(std::mem::take(&mut cur));
            }
            let mut piece = String::new();
            for ch in word.chars() {
                piece.push(ch);
                if text.width(&piece, px) > max_w && piece.chars().count() > 1 {
                    let last = piece.pop().unwrap();
                    lines.push(std::mem::take(&mut piece));
                    piece.push(last);
                }
            }
            cur = piece;
        }
        lines.push(cur);
    }
    lines
}

fn fill(pm: &mut PixmapMut<'_>, r: &R, radius: f32, col: Rgb) {
    if let Some(p) = crate::gui::icon::rounded_rect(r.x, r.y, r.w, r.h, radius) {
        pm.fill_path(&p, &paint(col[0], col[1], col[2], 255), FillRule::Winding, Transform::identity(), None);
    }
}

fn outline(pm: &mut PixmapMut<'_>, r: &R, radius: f32, col: Rgb) {
    if let Some(p) = crate::gui::icon::rounded_rect(r.x, r.y, r.w, r.h, radius) {
        pm.stroke_path(&p, &paint(col[0], col[1], col[2], 255), &Stroke { width: 1.0, ..Stroke::default() }, Transform::identity(), None);
    }
}

pub fn draw(st: &State, pm: &mut PixmapMut<'_>, text: &mut Text) {
    let (u, w, h) = (st.ui, pm.width() as f32, pm.height() as f32);
    let l = layout(st, w, h);
    if panels::empty(&l.panel) {
        return;
    }
    fill(pm, &l.panel, 0.0, SURFACE);
    let header_h = (HEADER * u).min(l.panel.h);
    let shown_title = crate::gui::sidebar::fit(text, &title(st), 13.0 * u, l.panel.w - 2.0 * PAD * u - if panels::empty(&l.close) { 0.0 } else { 30.0 * u });
    if l.msgs.h <= 0.0 {
        // dragged almost shut: only as much of the header as fits
        if header_h >= 24.0 * u {
            text.draw(pm, &shown_title, l.panel.x + PAD * u, l.panel.y + header_h / 2.0 + 4.5 * u, 13.0 * u, INK_MUTED, 0.0);
        }
        return;
    }

    // messages, newest at the bottom, scrolled up by `scroll`
    let (px, line_h) = (13.0 * u, 17.0 * u);
    let inner = l.msgs.w - 2.0 * PAD * u;
    let channel = st.channel_of(&st.chat.tab);
    let msgs: &[ChatMsg] = channel.as_ref().and_then(|c| st.chat.threads.get(c)).map_or(&[], |t| &t.msgs[..]);
    let mut lines: Vec<(String, Rgb, bool)> = Vec::new(); // text, colour, is a name line
    let mut last_user: Option<&str> = None;
    for m in msgs {
        if last_user != Some(m.user.as_str()) {
            let me = m.user == st.me_id;
            let name = if me { "You".to_string() } else { st.name_of(&m.user, m.name.as_deref()) };
            lines.push((name, if me { BRAND } else { INK_MUTED }, true));
        }
        last_user = Some(&m.user);
        for ln in wrap(text, &m.text, px, inner) {
            lines.push((ln, INK, false));
        }
    }
    let total: f32 = lines.iter().map(|(_, _, name)| if *name { line_h + 6.0 * u } else { line_h }).sum();
    st.chat.max_scroll.set((total - l.msgs.h + 12.0 * u).max(0.0));
    let scroll = st.chat.scroll.get().clamp(0.0, st.chat.max_scroll.get());
    let mut y = l.msgs.y + l.msgs.h - 8.0 * u - total + scroll;
    for (s, col, name) in &lines {
        if *name {
            y += 6.0 * u;
        }
        if y + line_h > l.msgs.y && y < l.msgs.y + l.msgs.h {
            text.draw(pm, s, l.msgs.x + PAD * u, y + line_h - 4.0 * u, if *name { 11.5 * u } else { px }, *col, 0.0);
        }
        y += line_h;
    }
    if msgs.is_empty() {
        let hint = match (&st.chat.tab, st.conversation()) {
            (Tab::Area, None) => "Walk into a meeting to chat with the people in it, or pick someone in the list to message them.".to_string(),
            (Tab::Area, Some(_)) => "No messages yet. Say hi to the area.".to_string(),
            (Tab::Dm(p), _) => format!("No messages yet. Say hi to {}.", st.name_of(p, None)),
        };
        for (i, ln) in wrap(text, &hint, 12.0 * u, inner).iter().enumerate() {
            let y = l.msgs.y + 24.0 * u + i as f32 * 16.0 * u;
            if y < l.msgs.y + l.msgs.h {
                text.draw(pm, ln, l.msgs.x + PAD * u, y, 12.0 * u, INK_FAINT, 0.0);
            }
        }
    }

    // header and input cover whatever scrolled under them
    fill(pm, &R { x: l.panel.x, y: l.panel.y, w: l.panel.w, h: l.msgs.y - l.panel.y }, 0.0, SURFACE);
    text.draw(pm, &shown_title, l.panel.x + PAD * u, l.panel.y + header_h / 2.0 + 4.5 * u, 13.0 * u, INK, 0.0);
    if !panels::empty(&l.close) {
        text.draw(pm, "×", l.close.x + l.close.w / 2.0, l.close.y + l.close.h / 2.0 + 6.0 * u, 20.0 * u, INK_MUTED, 0.5);
    }
    fill(pm, &R { x: l.panel.x, y: l.msgs.y - 1.0, w: l.panel.w, h: 1.0 }, 0.0, LINE);
    let bottom = l.panel.y + l.panel.h;
    fill(pm, &R { x: l.panel.x, y: l.msgs.y + l.msgs.h, w: l.panel.w, h: bottom - (l.msgs.y + l.msgs.h) }, 0.0, SURFACE);
    fill(pm, &R { x: l.panel.x, y: l.msgs.y + l.msgs.h, w: l.panel.w, h: 1.0 }, 0.0, LINE);
    fill(pm, &l.input, 8.0 * u, MUTED);
    outline(pm, &l.input, 8.0 * u, if st.chat.focus { BRAND } else { LINE });
    let max_w = l.input.w - 24.0 * u;
    let mut shown: String = st.chat.input.clone();
    while !shown.is_empty() && text.width(&shown, px) > max_w {
        shown.remove(0); // show the end of what is being typed
    }
    let base = l.input.y + l.input.h / 2.0 + 4.5 * u;
    if st.chat.input.is_empty() && !st.chat.focus {
        text.draw(pm, "Write a message…", l.input.x + 12.0 * u, base, px, INK_FAINT, 0.0);
    } else {
        let caret = if st.chat.focus { "|" } else { "" };
        text.draw(pm, &format!("{shown}{caret}"), l.input.x + 12.0 * u, base, px, INK, 0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_words_and_lines_wrap() {
        let mut t = Text::new();
        let max = t.width("aaa bbb", 13.0) + 1.0;
        let lines = wrap(&mut t, "aaa bbb ccc", 13.0, max);
        assert_eq!(lines, ["aaa bbb", "ccc"]);
        let broken = wrap(&mut t, "abcdefghijklmnopqrstuvwxyz", 13.0, 40.0);
        assert!(broken.len() > 1 && broken.concat() == "abcdefghijklmnopqrstuvwxyz");
    }
}
