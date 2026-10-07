//! The chat panel on the right: tabs (the area chat of the call we are in, then one per private chat), the
//! messages, and a one-line input. State and logic live here too (`impl State`), the transport is `chat.rs`.
use crate::app::State;
use crate::chat::{ChatCmd, ChatEvent, ChatMsg, Kind};
use crate::meeting_ui::R;
use crate::render::{paint, Text};
use crate::theme::*;
use std::cell::Cell;
use std::collections::HashMap;
use tiny_skia::{FillRule, Pixmap, Stroke, Transform};

pub const WIDTH: f32 = 320.0;
const HEADER: f32 = 44.0;
const INPUT_H: f32 = 52.0;
const PAD: f32 = 12.0;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tab {
    Area,
    Dm(String),
}

#[derive(Default)]
pub struct Thread {
    pub msgs: Vec<ChatMsg>,
    pub unread: usize,
}

pub struct Chat {
    pub open: bool,
    pub tab: Tab,
    /// People whose private chat has a tab.
    pub dms: Vec<String>,
    /// Conversation id -> its meeting channel; person id -> the direct channel with them.
    pub area: HashMap<String, String>,
    pub dm_channel: HashMap<String, String>,
    pub threads: HashMap<String, Thread>,
    pub input: String,
    pub focus: bool,
    /// Pixels scrolled up from the newest message; the maximum is measured while drawing.
    pub scroll: Cell<f32>,
    max_scroll: Cell<f32>,
}

impl Default for Chat {
    fn default() -> Self {
        Chat { open: false, tab: Tab::Area, dms: vec![], area: HashMap::new(), dm_channel: HashMap::new(), threads: HashMap::new(), input: String::new(), focus: false, scroll: Cell::new(0.0), max_scroll: Cell::new(0.0) }
    }
}

pub fn width(st: &State) -> f32 {
    if st.login.is_none() && st.chat.open { WIDTH * st.ui } else { 0.0 }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Hit {
    Tab(Tab),
    Close,
    Input,
    Panel,
}

impl State {
    fn conversation(&self) -> Option<String> {
        self.cluster.as_ref().map(|c| c.conversation_id.clone()).filter(|c| !c.is_empty())
    }

    /// The channel behind a tab, once the server told us about it.
    pub fn channel_of(&self, tab: &Tab) -> Option<String> {
        match tab {
            Tab::Area => self.conversation().and_then(|c| self.chat.area.get(&c).cloned()),
            Tab::Dm(p) => self.chat.dm_channel.get(p).cloned(),
        }
    }

    pub fn name_of(&self, id: &str, hint: Option<&str>) -> String {
        hint.map(String::from)
            .or_else(|| self.users.get(id).map(|u| u.name.clone()).filter(|n| !n.is_empty()))
            .or_else(|| self.roster.iter().find(|m| m.id == id).map(|m| m.name.clone()))
            .unwrap_or_else(|| "Someone".into())
    }

    pub fn unread_dm(&self, peer: &str) -> usize {
        self.chat.dm_channel.get(peer).and_then(|c| self.chat.threads.get(c)).map_or(0, |t| t.unread)
    }

    pub fn unread_total(&self) -> usize {
        self.chat.threads.values().map(|t| t.unread).sum()
    }

    pub fn toggle_chat(&mut self) {
        if self.chat.open {
            self.chat.open = false;
            self.chat.focus = false;
        } else {
            let tab = self.chat.tab.clone();
            self.select_tab(tab);
            self.chat.open = true;
        }
        self.dirty = true;
    }

    /// Open the private chat with someone and put the cursor in the input.
    pub fn open_dm(&mut self, peer: &str) {
        if !self.chat.dms.iter().any(|p| p == peer) {
            self.chat.dms.push(peer.to_string());
        }
        self.select_tab(Tab::Dm(peer.to_string()));
        self.chat.open = true;
        self.chat.focus = true;
        self.dirty = true;
    }

    pub fn select_tab(&mut self, tab: Tab) {
        self.chat.tab = tab.clone();
        self.chat.scroll.set(0.0);
        match &tab {
            Tab::Area => {
                if let Some(conversation) = self.conversation() {
                    self.chat_out.push(ChatCmd::Area { conversation });
                }
            }
            Tab::Dm(peer) => self.chat_out.push(ChatCmd::Dm { peer: peer.clone() }),
        }
        self.mark_read(&tab);
        self.dirty = true;
    }

    fn mark_read(&mut self, tab: &Tab) {
        let Some(ch) = self.channel_of(tab) else { return };
        if let Some(t) = self.chat.threads.get_mut(&ch) {
            t.unread = 0;
            if let Some(last) = t.msgs.last() {
                self.chat_out.push(ChatCmd::Read { channel: ch, message: last.id.clone() });
            }
        }
    }

    pub fn send_chat(&mut self, now: f64) {
        let text = self.chat.input.trim().to_string();
        if text.is_empty() {
            return;
        }
        match self.chat.tab.clone() {
            Tab::Area => match self.conversation() {
                Some(conversation) => self.chat_out.push(ChatCmd::SendArea { conversation, text }),
                None => return self.say("Walk into a meeting to chat with the area", now),
            },
            Tab::Dm(peer) => {
                let channel = self.chat.dm_channel.get(&peer).cloned();
                self.chat_out.push(ChatCmd::SendDm { peer, channel, text });
            }
        }
        self.chat.input.clear();
        self.chat.scroll.set(0.0);
        self.dirty = true;
    }

    /// A key press while the input has the focus. `text` is what the key types, if anything.
    pub fn chat_key(&mut self, named: Option<&str>, text: Option<&str>, now: f64) {
        match named {
            Some("Enter") => self.send_chat(now),
            Some("Backspace") => {
                self.chat.input.pop();
            }
            Some("Escape") => self.chat.focus = false,
            _ => {
                if let Some(t) = text {
                    self.chat.input.extend(t.chars().filter(|c| !c.is_control()));
                }
            }
        }
        self.dirty = true;
    }

    pub fn apply_chat(&mut self, ev: ChatEvent, now: f64) {
        self.dirty = true;
        match ev {
            ChatEvent::Channel { id, kind } => match kind {
                Kind::Direct(peer) => {
                    self.chat.dm_channel.insert(peer, id);
                }
                Kind::Meeting(conv) => {
                    self.chat.area.insert(conv, id);
                }
                Kind::Other => {}
            },
            ChatEvent::History { channel, msgs } => {
                for m in msgs {
                    self.store_message(m);
                }
                let _ = channel;
            }
            ChatEvent::Message(m) => {
                let mine = m.user == self.me_id;
                let name = self.name_of(&m.user, m.name.as_deref());
                let (channel, text) = (m.channel.clone(), m.text.clone());
                if !self.store_message(m) || mine {
                    return;
                }
                let viewing = self.chat.open && self.channel_of(&self.chat.tab.clone()).as_deref() == Some(channel.as_str());
                if viewing {
                    self.mark_read(&self.chat.tab.clone());
                } else if let Some(t) = self.chat.threads.get_mut(&channel) {
                    t.unread += 1;
                    let short: String = text.chars().take(60).collect();
                    self.say(format!("{name}: {short}"), now);
                }
            }
            ChatEvent::Error(e) => self.say(e, now),
        }
    }

    /// Add to the channel's thread unless it is a duplicate or a channel we do not show. True if it was new.
    fn store_message(&mut self, m: ChatMsg) -> bool {
        let known = self.chat.dm_channel.values().any(|c| *c == m.channel) || self.chat.area.values().any(|c| *c == m.channel);
        if !known {
            return false;
        }
        let t = self.chat.threads.entry(m.channel.clone()).or_default();
        if t.msgs.iter().any(|x| x.id == m.id) {
            return false;
        }
        t.msgs.push(m);
        t.msgs.sort_by_key(|x| x.at);
        true
    }
}

fn tabs(st: &State) -> Vec<Tab> {
    std::iter::once(Tab::Area).chain(st.chat.dms.iter().cloned().map(Tab::Dm)).collect()
}

fn tab_label(st: &State, t: &Tab) -> String {
    match t {
        Tab::Area => "Area".into(),
        Tab::Dm(p) => st.name_of(p, None),
    }
}

struct Layout {
    panel: R,
    close: R,
    tabs: Vec<(Tab, R)>,
    input: R,
    msgs: R,
}

fn layout(st: &State, w: f32, h: f32) -> Layout {
    let u = st.ui;
    let panel = R { x: w - WIDTH * u, y: 0.0, w: WIDTH * u, h };
    let close = R { x: panel.x + panel.w - 36.0 * u, y: 0.0, w: 36.0 * u, h: HEADER * u };
    let all = tabs(st);
    let tw = (panel.w - 36.0 * u) / all.len() as f32;
    let tabs = all.into_iter().enumerate().map(|(i, t)| (t, R { x: panel.x + i as f32 * tw, y: 0.0, w: tw, h: HEADER * u })).collect();
    let input = R { x: panel.x + PAD * u, y: h - (INPUT_H - 8.0) * u, w: panel.w - 2.0 * PAD * u, h: (INPUT_H - 16.0) * u };
    let msgs = R { x: panel.x, y: HEADER * u, w: panel.w, h: (h - (HEADER + INPUT_H) * u).max(0.0) };
    Layout { panel, close, tabs, input, msgs }
}

pub fn hit(st: &State, px: f32, py: f32, w: f32, h: f32) -> Option<Hit> {
    if width(st) == 0.0 {
        return None;
    }
    let l = layout(st, w, h);
    if !l.panel.contains(px, py) {
        return None;
    }
    if l.close.contains(px, py) {
        return Some(Hit::Close);
    }
    if let Some((t, _)) = l.tabs.iter().find(|(_, r)| r.contains(px, py)) {
        return Some(Hit::Tab(t.clone()));
    }
    Some(if l.input.contains(px, py) { Hit::Input } else { Hit::Panel })
}

pub fn wheel(st: &mut State, px: f32, py: f32, w: f32, h: f32, dy: f32) -> bool {
    if width(st) == 0.0 || !layout(st, w, h).panel.contains(px, py) {
        return false;
    }
    st.chat.scroll.set((st.chat.scroll.get() + dy.signum() * 40.0 * st.ui).clamp(0.0, st.chat.max_scroll.get()));
    st.dirty = true;
    true
}

pub fn press(st: &mut State, hit: Hit) {
    match hit {
        Hit::Close => st.toggle_chat(),
        Hit::Tab(t) => {
            st.select_tab(t);
            st.chat.focus = true;
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

fn fill(pm: &mut Pixmap, r: &R, radius: f32, col: Rgb) {
    if let Some(p) = crate::icon::rounded_rect(r.x, r.y, r.w, r.h, radius) {
        pm.fill_path(&p, &paint(col[0], col[1], col[2], 255), FillRule::Winding, Transform::identity(), None);
    }
}

fn outline(pm: &mut Pixmap, r: &R, radius: f32, col: Rgb) {
    if let Some(p) = crate::icon::rounded_rect(r.x, r.y, r.w, r.h, radius) {
        pm.stroke_path(&p, &paint(col[0], col[1], col[2], 255), &Stroke { width: 1.0, ..Stroke::default() }, Transform::identity(), None);
    }
}

pub fn draw(st: &State, pm: &mut Pixmap, text: &mut Text) {
    if width(st) == 0.0 {
        return;
    }
    let (u, w, h) = (st.ui, pm.width() as f32, pm.height() as f32);
    let l = layout(st, w, h);
    fill(pm, &l.panel, 0.0, SURFACE);

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
            (Tab::Area, None) => "Walk into a meeting to chat with the people in it.".to_string(),
            (Tab::Area, Some(_)) => "No messages yet. Say hi to the area.".to_string(),
            (Tab::Dm(p), _) => format!("No messages yet. Say hi to {}.", st.name_of(p, None)),
        };
        for (i, ln) in wrap(text, &hint, 12.0 * u, inner).iter().enumerate() {
            text.draw(pm, ln, l.msgs.x + PAD * u, l.msgs.y + 28.0 * u + i as f32 * 16.0 * u, 12.0 * u, INK_FAINT, 0.0);
        }
    }

    // header and input cover whatever scrolled under them
    fill(pm, &R { x: l.panel.x, y: 0.0, w: l.panel.w, h: l.msgs.y }, 0.0, SURFACE);
    for (t, r) in &l.tabs {
        let active = *t == st.chat.tab;
        let label = tab_label(st, t);
        let unread = match t {
            Tab::Area => st.channel_of(t).and_then(|c| st.chat.threads.get(&c)).map_or(0, |x| x.unread),
            Tab::Dm(p) => st.unread_dm(p),
        };
        let label = if unread > 0 { format!("{label} ({unread})") } else { label };
        let shown = crate::sidebar::fit(text, &label, 13.0 * u, r.w - 14.0 * u);
        text.draw(pm, &shown, r.x + r.w / 2.0, r.y + r.h / 2.0 + 4.5 * u, 13.0 * u, if active { INK } else { INK_MUTED }, 0.5);
        if active {
            fill(pm, &R { x: r.x + 8.0 * u, y: r.h - 2.0 * u, w: r.w - 16.0 * u, h: 2.0 * u }, 1.0 * u, BRAND);
        }
    }
    text.draw(pm, "×", l.close.x + l.close.w / 2.0, l.close.y + l.close.h / 2.0 + 6.0 * u, 20.0 * u, INK_MUTED, 0.5);
    fill(pm, &R { x: l.panel.x, y: l.msgs.y - 1.0, w: l.panel.w, h: 1.0 }, 0.0, LINE);
    fill(pm, &R { x: l.panel.x, y: l.msgs.y + l.msgs.h, w: l.panel.w, h: h - (l.msgs.y + l.msgs.h) }, 0.0, SURFACE);
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
    fill(pm, &R { x: l.panel.x, y: 0.0, w: 1.0, h }, 0.0, LINE);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat::ChatMsg;

    fn msg(id: &str, ch: &str, user: &str, text: &str, at: i64) -> ChatMsg {
        ChatMsg { id: id.into(), channel: ch.into(), user: user.into(), name: Some(user.into()), text: text.into(), at }
    }

    fn state() -> State {
        let mut st = State::new("me", true);
        st.apply_chat(ChatEvent::Channel { id: "D1".into(), kind: Kind::Direct("kury".into()) }, 0.0);
        st
    }

    #[test]
    fn messages_are_deduplicated_and_ordered() {
        let mut st = state();
        st.apply_chat(ChatEvent::Message(msg("2", "D1", "kury", "second", 20)), 0.0);
        st.apply_chat(ChatEvent::History { channel: "D1".into(), msgs: vec![msg("1", "D1", "kury", "first", 10), msg("2", "D1", "kury", "second", 20)] }, 0.0);
        let t = &st.chat.threads["D1"];
        assert_eq!(t.msgs.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(), ["first", "second"]);
        assert_eq!(t.unread, 1, "the live message counted once; history does not count as unread");
    }

    #[test]
    fn unread_clears_when_the_chat_is_opened() {
        let mut st = state();
        st.apply_chat(ChatEvent::Message(msg("1", "D1", "kury", "oi", 10)), 0.0);
        assert_eq!((st.unread_dm("kury"), st.unread_total()), (1, 1));
        st.chat_out.clear();
        st.open_dm("kury");
        assert_eq!(st.unread_dm("kury"), 0);
        assert!(st.chat_out.iter().any(|c| matches!(c, ChatCmd::Read { message, .. } if message == "1")), "the server is told it was read");
    }

    #[test]
    fn typing_and_sending_a_private_message() {
        let mut st = state();
        st.open_dm("kury");
        st.chat_out.clear();
        st.chat_key(None, Some("o"), 0.0);
        st.chat_key(None, Some("i"), 0.0);
        st.chat_key(Some("Backspace"), None, 0.0);
        st.chat_key(None, Some("k"), 0.0);
        st.chat_key(Some("Enter"), None, 0.0);
        assert!(matches!(&st.chat_out[..], [ChatCmd::SendDm { peer, channel: Some(c), text }] if peer == "kury" && c == "D1" && text == "ok"));
        assert!(st.chat.input.is_empty());
    }

    #[test]
    fn area_chat_needs_a_meeting() {
        let mut st = state();
        st.chat.tab = Tab::Area;
        st.chat.input = "hello".into();
        st.send_chat(0.0);
        assert!(st.chat_out.is_empty() && st.chat.input == "hello", "nothing sent outside a meeting; the text stays");
    }

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
