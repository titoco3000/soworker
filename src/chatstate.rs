//! Chat state and logic: tabs, threads, the input line, sending and receiving. The panel that draws it is per frontend.
use crate::app::State;
use crate::chat::{ChatCmd, ChatEvent, ChatMsg, Kind};
use std::cell::Cell;
use std::collections::HashMap;

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
    pub max_scroll: Cell<f32>,
}

impl Default for Chat {
    fn default() -> Self {
        Chat { open: false, tab: Tab::Area, dms: vec![], area: HashMap::new(), dm_channel: HashMap::new(), threads: HashMap::new(), input: String::new(), focus: false, scroll: Cell::new(0.0), max_scroll: Cell::new(0.0) }
    }
}
impl State {
    pub fn conversation(&self) -> Option<String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat::{ChatCmd, ChatEvent, Kind};

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
}
