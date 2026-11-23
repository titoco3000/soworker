#![cfg_attr(not(feature = "gui"), allow(dead_code))]
//! The people list: who is where, grouped into meetings, online and offline, plus the actions on a row. No drawing.
use crate::app::State;
use std::collections::HashMap;

/// What the user is doing in the sidebar (kept in `State`).
#[derive(Default)]
pub struct Ui {
    /// The row whose menu is open: a user id, or `zone:<zone id>` for a meeting.
    pub open: Option<String>,
    pub scroll: f32,
    /// Narrow sidebar: only the avatars, no names.
    pub collapsed: bool,
    /// Where the mouse is over the collapsed sidebar: the row under it gets a card with its details.
    pub hover: Option<(f32, f32)>,
    /// Height of the people list above the chat in the wide sidebar, in logical pixels (0 = the default split).
    pub split: f32,
    /// The line between the people list and the chat is being dragged: where it was grabbed, relative to the line.
    pub split_drag: Option<f32>,
    /// The window is narrower than a phone (window client): one panel at a time, picked in the footer.
    pub mobile: bool,
    /// The panel shown in the narrow (mobile) layout.
    pub view: View,
}

/// What fills a narrow (mobile) window between its header and footer. Settings opens over whichever it is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum View {
    Users,
    #[default]
    Main,
    Chat,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub online: bool,
    pub away: bool,
    pub me: bool,
    pub zone: Option<String>,
    pub last_online: Option<i64>,
    pub unread: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MenuBtn {
    Join { enabled: bool },
    Wave,
    Message,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Row {
    Section(String),
    Group { key: String, title: String, count: usize, unread: usize },
    Person { p: Person, indent: bool },
    /// The buttons under the open row.
    Menu { key: String, buttons: Vec<MenuBtn> },
    /// Someone waved at us: Accept (walk to them) or Dismiss.
    Wave { from: String, name: String },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Hit {
    /// Walk back to our desk.
    Desk,
    /// The collapse / expand button.
    Collapse,
    /// Show the chat (the unread counter of the collapsed sidebar).
    Chat,
    Row(String),
    Join(String),
    Wave(String),
    Message(String),
    WaveAccept(String),
    WaveDismiss(String),
    /// A click on the panel itself: swallowed so it does not walk the avatar.
    Panel,
}

pub fn zone_key(zone: &str) -> String {
    format!("zone:{zone}")
}

/// Everyone we know: the gaia room (live) plus the member list (names, offline people, people not in this room).
pub fn people(st: &State) -> Vec<Person> {
    let mut out: Vec<Person> = st
        .users
        .values()
        .map(|u| {
            let me = u.id == st.me_id;
            let name = if u.name.is_empty() { st.roster.iter().find(|m| m.id == u.id).map(|m| m.name.clone()).unwrap_or_default() } else { u.name.clone() };
            Person {
                id: u.id.clone(),
                name,
                online: true,
                away: if me { !st.available } else { u.away() },
                me,
                zone: if me { st.cur_zone_id.clone() } else { u.zone.clone() },
                last_online: None,
                unread: st.unread_dm(&u.id),
            }
        })
        .collect();
    for m in &st.roster {
        if st.users.contains_key(&m.id) {
            continue;
        }
        out.push(Person {
            id: m.id.clone(),
            name: m.name.clone(),
            online: m.online,
            away: m.status == "busy" || m.status == "away",
            me: m.id == st.me_id,
            zone: m.zone.clone().filter(|_| m.online),
            last_online: m.last_online,
            unread: st.unread_dm(&m.id),
        });
    }
    out
}

pub fn zone_title(st: &State, zone: &str) -> String {
    st.world.zones.iter().find(|z| z.id == zone).and_then(|z| z.name.clone()).unwrap_or_else(|| "Meeting".into())
}

/// The sidebar's rows, top to bottom. `open` is the row with its menu showing.
pub fn rows(st: &State) -> Vec<Row> {
    let all = people(st);
    let (online, mut offline): (Vec<Person>, Vec<Person>) = all.into_iter().partition(|p| p.online);
    let mut by_zone: HashMap<String, Vec<Person>> = HashMap::new();
    let mut loose = Vec::new();
    for p in online {
        match p.zone.clone() {
            Some(z) => by_zone.entry(z).or_default().push(p),
            None => loose.push(p),
        }
    }
    // a zone with a single person is just an online person
    let mut groups: Vec<(String, Vec<Person>)> = Vec::new();
    for (z, ps) in by_zone {
        if ps.len() >= 2 { groups.push((z, ps)) } else { loose.extend(ps) }
    }
    let by_name = |a: &Person, b: &Person| (!a.me, a.name.to_lowercase()).cmp(&(!b.me, b.name.to_lowercase()));
    loose.sort_by(by_name);
    for (_, ps) in &mut groups {
        ps.sort_by(by_name);
    }
    groups.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then_with(|| zone_title(st, &a.0).cmp(&zone_title(st, &b.0))).then_with(|| a.0.cmp(&b.0)));
    offline.sort_by(|a, b| b.last_online.cmp(&a.last_online).then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase())));

    let open = st.sidebar.open.as_deref();
    let mut out = Vec::new();
    if !st.waves.is_empty() {
        out.push(Row::Section(format!("WAVES · {}", st.waves.len())));
        for w in &st.waves {
            out.push(Row::Wave { from: w.from.clone(), name: w.name.clone() });
        }
    }
    let menu = |out: &mut Vec<Row>, key: &str, buttons: Vec<MenuBtn>| {
        if open == Some(key) {
            out.push(Row::Menu { key: key.to_string(), buttons });
        }
    };
    let person_menu = |p: &Person| {
        if p.online {
            vec![MenuBtn::Join { enabled: join_target(st, &p.id).is_some() }, MenuBtn::Wave, MenuBtn::Message]
        } else {
            vec![MenuBtn::Message]
        }
    };
    if !groups.is_empty() {
        out.push(Row::Section(format!("IN MEETINGS · {}", groups.iter().map(|g| g.1.len()).sum::<usize>())));
        for (z, ps) in groups {
            let key = zone_key(&z);
            let unread = if st.cur_zone_id.as_deref() == Some(z.as_str()) { area_unread(st) } else { 0 };
            out.push(Row::Group { key: key.clone(), title: zone_title(st, &z), count: ps.len(), unread });
            // the area chat is only reachable for the meeting we are in
            let mut buttons = vec![MenuBtn::Join { enabled: join_target(st, &key).is_some() }, MenuBtn::Wave];
            if st.cur_zone_id.as_deref() == Some(z.as_str()) && st.conversation().is_some() {
                buttons.push(MenuBtn::Message);
            }
            menu(&mut out, &key, buttons);
            for p in ps {
                let key = p.id.clone();
                let buttons = person_menu(&p);
                out.push(Row::Person { p, indent: true });
                menu(&mut out, &key, buttons);
            }
        }
    }
    if !loose.is_empty() {
        out.push(Row::Section(format!("ONLINE · {}", loose.len())));
        for p in loose {
            let key = p.id.clone();
            let buttons = person_menu(&p);
            out.push(Row::Person { p, indent: false });
            menu(&mut out, &key, buttons);
        }
    }
    if !offline.is_empty() {
        out.push(Row::Section(format!("OFFLINE · {}", offline.len())));
        for p in offline {
            let key = p.id.clone();
            out.push(Row::Person { p, indent: false });
            menu(&mut out, &key, vec![MenuBtn::Message]);
        }
    }
    out
}

/// Unread messages in the area chat of the meeting we are in.
pub fn area_unread(st: &State) -> usize {
    st.channel_of(&crate::chatstate::Tab::Area).and_then(|c| st.chat.threads.get(&c)).map_or(0, |t| t.unread)
}

impl State {
    /// Is the chat panel on screen? The wide sidebar always shows it; the collapsed one never does; a narrow window
    /// shows it when it is the chosen panel and settings are not open over it.
    pub fn chat_shown(&self) -> bool {
        if self.sidebar.mobile {
            self.sidebar.view == View::Chat && self.profile.is_none()
        } else {
            !self.sidebar.collapsed
        }
    }

    /// Keep `chat.open` (what marks messages read) in step with what the window shows.
    pub fn sync_chat_shown(&mut self) {
        let shown = self.chat_shown();
        if shown == self.chat.open {
            return;
        }
        if shown {
            let tab = self.chat.tab.clone();
            self.select_tab(tab);
            self.chat.open = true;
        } else {
            self.chat.open = false;
            self.chat.focus = false;
        }
        self.dirty = true;
    }

    /// Bring the chat panel on screen: widen the collapsed sidebar, or switch a narrow window to it.
    pub fn reveal_chat(&mut self) {
        if self.sidebar.mobile {
            self.sidebar.view = View::Chat;
            self.profile = None;
        } else if self.sidebar.collapsed {
            self.toggle_sidebar();
        }
        self.sync_chat_shown();
        self.dirty = true;
    }
}

/// Where Join walks to, and what to call it: a meeting's centre, or for a person the centre of the meeting they are
/// in, else their spot on the map.
pub fn join_target(st: &State, key: &str) -> Option<((f32, f32), String)> {
    let zone_centre = |z: &str| st.world.zones.iter().find(|w| w.id == z).map(|w| (w.x, w.y));
    if let Some(z) = key.strip_prefix("zone:") {
        return zone_centre(z).map(|c| (c, zone_title(st, z)));
    }
    let p = people(st).into_iter().find(|p| p.id == key)?;
    if !p.online || p.me {
        return None;
    }
    let at = p.zone.as_deref().and_then(zone_centre).or_else(|| st.users.get(key).map(|u| (u.x, u.y)))?;
    Some((at, p.name))
}

/// "5m ago", "3d ago": how long since someone was online.
pub fn ago(now_ms: i64, then: Option<i64>) -> String {
    let Some(t) = then else { return "offline".into() };
    let mins = ((now_ms - t) / 60_000).max(0);
    match mins {
        0 => "just now".into(),
        1..=59 => format!("{mins}m ago"),
        60..=2879 => format!("{}h ago", mins / 60),
        _ => format!("{}d ago", mins / 1440),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::proto::{Net, User};
    use crate::roster::Member;

    fn user(id: &str, name: &str, zone: Option<&str>) -> User {
        User { id: id.into(), name: name.into(), zone: zone.map(String::from), status: "online".into(), mode: "cluster".into(), ..User::default() }
    }

    pub fn state() -> State {
        let mut st = State::new("me", true);
        st.apply(
            Net::Room {
                name: "Office".into(),
                spawn: None,
                users: vec![user("me", "Me", None), user("a", "Ana", Some("Z1")), user("b", "Bo", Some("Z1")), user("c", "Cy", Some("Z2")), user("d", "Di", None)],
            },
            0.0,
        );
        st.apply(
            Net::Roster(vec![
                Member { id: "e".into(), name: "Eve".into(), online: false, last_online: Some(1_000), ..Member::default() },
                Member { id: "f".into(), name: "Fay".into(), online: false, last_online: Some(5_000), ..Member::default() },
                Member { id: "a".into(), name: "Ana".into(), online: true, ..Member::default() },
            ]),
            0.0,
        );
        st
    }

    fn names(rows: &[Row]) -> Vec<String> {
        rows.iter()
            .map(|r| match r {
                Row::Section(s) => format!("# {s}"),
                Row::Group { title, count, .. } => format!("[{title} {count}]"),
                Row::Person { p, indent } => format!("{}{}", if *indent { "  " } else { "" }, p.name),
                Row::Menu { .. } => "<menu>".into(),
                Row::Wave { name, .. } => format!("~{name} waved"),
            })
            .collect()
    }

    #[test]
    fn groups_meetings_then_online_then_offline() {
        let st = state();
        assert_eq!(
            names(&rows(&st)),
            ["# IN MEETINGS · 2", "[Meeting 2]", "  Ana", "  Bo", "# ONLINE · 3", "Me", "Cy", "Di", "# OFFLINE · 2", "Fay", "Eve"],
            "a lone person in a zone is just online; offline is newest first; I come first among the online"
        );
    }

    #[test]
    fn clicking_opens_a_menu_under_the_row() {
        let mut st = state();
        st.sidebar.open = Some("d".into());
        let n = names(&rows(&st));
        let at = n.iter().position(|s| s == "Di").unwrap();
        assert_eq!(n[at + 1], "<menu>");
        st.sidebar.open = Some(zone_key("Z1"));
        assert_eq!(names(&rows(&st))[2], "<menu>", "a meeting's menu sits right under its header");
    }

    #[test]
    fn last_seen_reads_naturally() {
        let now = 10 * 86_400_000;
        assert_eq!(ago(now, Some(now - 30_000)), "just now");
        assert_eq!(ago(now, Some(now - 5 * 60_000)), "5m ago");
        assert_eq!(ago(now, Some(now - 3 * 3_600_000)), "3h ago");
        assert_eq!(ago(now, Some(now - 3 * 86_400_000)), "3d ago");
        assert_eq!(ago(now, None), "offline");
    }
}
