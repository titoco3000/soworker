//! The left sidebar: availability and log out buttons, then everyone in the office: people in meetings (grouped by
//! meeting zone), other people who are online, and people who are offline. Clicking an online person or a meeting
//! opens a small menu with Join (walk there) and Wave (not implemented yet).
use crate::app::State;
use crate::meeting_ui::R;
use crate::render::{paint, Text};
use crate::theme::*;
use std::collections::HashMap;
use tiny_skia::{FillRule, PathBuilder, Pixmap, PixmapPaint, Stroke, Transform};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Act {
    Away,
    Logout,
}

/// What the user is doing in the sidebar (kept in `State`).
#[derive(Default)]
pub struct Ui {
    /// The row whose menu is open: a user id, or `zone:<zone id>` for a meeting.
    pub open: Option<String>,
    pub scroll: f32,
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
}

#[derive(Clone, Debug, PartialEq)]
pub enum Row {
    Section(String),
    Group { key: String, title: String, count: usize },
    Person { p: Person, indent: bool },
    /// The Join / Wave buttons under the open row.
    Menu { key: String, can_join: bool },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Hit {
    Away,
    Logout,
    Row(String),
    Join(String),
    Wave(String),
    /// A click on the panel itself: swallowed so it does not walk the avatar.
    Panel,
}

pub const WIDTH: f32 = 252.0;
const HEADER: f32 = 92.0;
const PAD: f32 = 12.0;

pub fn width(st: &State) -> f32 {
    if st.login.is_some() { 0.0 } else { WIDTH * st.ui }
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
    let menu = |out: &mut Vec<Row>, key: &str, can_join: bool| {
        if open == Some(key) {
            out.push(Row::Menu { key: key.to_string(), can_join });
        }
    };
    if !groups.is_empty() {
        out.push(Row::Section(format!("IN MEETINGS · {}", groups.iter().map(|g| g.1.len()).sum::<usize>())));
        for (z, ps) in groups {
            let key = zone_key(&z);
            out.push(Row::Group { key: key.clone(), title: zone_title(st, &z), count: ps.len() });
            let can_join = join_target(st, &key).is_some();
            menu(&mut out, &key, can_join);
            for p in ps {
                let key = p.id.clone();
                let can_join = !p.me && join_target(st, &key).is_some();
                out.push(Row::Person { p, indent: true });
                menu(&mut out, &key, can_join);
            }
        }
    }
    if !loose.is_empty() {
        out.push(Row::Section(format!("ONLINE · {}", loose.len())));
        for p in loose {
            let key = p.id.clone();
            let can_join = !p.me && join_target(st, &key).is_some();
            out.push(Row::Person { p, indent: false });
            menu(&mut out, &key, can_join);
        }
    }
    if !offline.is_empty() {
        out.push(Row::Section(format!("OFFLINE · {}", offline.len())));
        for p in offline {
            out.push(Row::Person { p, indent: false });
        }
    }
    out
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

fn row_h(r: &Row) -> f32 {
    match r {
        Row::Section(_) => 30.0,
        _ => 34.0,
    }
}

pub struct Layout {
    pub panel: R,
    pub away: R,
    pub logout: R,
    pub list: R,
    pub items: Vec<(Row, R)>,
    pub max_scroll: f32,
}

pub fn layout(st: &State, h: f32) -> Layout {
    let u = st.ui;
    let panel = R { x: 0.0, y: 0.0, w: WIDTH * u, h };
    let bw = (panel.w - 3.0 * PAD * u) / 2.0;
    let by = (HEADER - 36.0) * u;
    let away = R { x: PAD * u, y: by, w: bw, h: 28.0 * u };
    let logout = R { x: PAD * u * 2.0 + bw, y: by, w: bw, h: 28.0 * u };
    let list = R { x: 0.0, y: HEADER * u, w: panel.w, h: (h - HEADER * u).max(0.0) };
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
    Layout { panel, away, logout, list, items, max_scroll }
}

/// The two menu buttons inside a Menu row.
fn menu_buttons(rect: &R, u: f32) -> (R, R) {
    let bw = (rect.w - 3.0 * PAD * u) / 2.0;
    let (y, h) = (rect.y + 3.0 * u, rect.h - 6.0 * u);
    (R { x: PAD * u, y, w: bw, h }, R { x: PAD * u * 2.0 + bw, y, w: bw, h })
}

pub fn hit(st: &State, px: f32, py: f32, h: f32) -> Option<Hit> {
    let l = layout(st, h);
    if !l.panel.contains(px, py) {
        return None;
    }
    if l.away.contains(px, py) {
        return Some(Hit::Away);
    }
    if l.logout.contains(px, py) {
        return Some(Hit::Logout);
    }
    if l.list.contains(px, py) {
        for (row, rect) in &l.items {
            if !rect.contains(px, py) {
                continue;
            }
            return Some(match row {
                Row::Group { key, .. } => Hit::Row(key.clone()),
                Row::Person { p, .. } if p.online && !p.me => Hit::Row(p.id.clone()),
                Row::Menu { key, can_join } => {
                    let (join, wave) = menu_buttons(rect, st.ui);
                    if join.contains(px, py) && *can_join {
                        Hit::Join(key.clone())
                    } else if wave.contains(px, py) {
                        Hit::Wave(key.clone())
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

fn ago(now_ms: i64, then: Option<i64>) -> String {
    let Some(t) = then else { return "offline".into() };
    let mins = ((now_ms - t) / 60_000).max(0);
    match mins {
        0 => "just now".into(),
        1..=59 => format!("{mins}m ago"),
        60..=2879 => format!("{}h ago", mins / 60),
        _ => format!("{}d ago", mins / 1440),
    }
}

fn fit(text: &mut Text, s: &str, px: f32, max_w: f32) -> String {
    if text.width(s, px) <= max_w {
        return s.to_string();
    }
    let mut out: String = s.to_string();
    while !out.is_empty() && text.width(&format!("{out}…"), px) > max_w {
        out.pop();
    }
    format!("{}…", out.trim_end())
}

fn fill(pm: &mut Pixmap, r: &R, radius: f32, col: Rgb, a: u8) {
    if let Some(p) = crate::icon::rounded_rect(r.x, r.y, r.w, r.h, radius) {
        let c = paint(col[0], col[1], col[2], a);
        pm.fill_path(&p, &c, FillRule::Winding, Transform::identity(), None);
    }
}

fn outline(pm: &mut Pixmap, r: &R, radius: f32, col: Rgb) {
    if let Some(p) = crate::icon::rounded_rect(r.x, r.y, r.w, r.h, radius) {
        pm.stroke_path(&p, &paint(col[0], col[1], col[2], 255), &Stroke { width: 1.0, ..Stroke::default() }, Transform::identity(), None);
    }
}

fn dot(pm: &mut Pixmap, x: f32, y: f32, r: f32, col: Rgb) {
    if let Some(c) = PathBuilder::from_circle(x, y, r) {
        pm.fill_path(&c, &paint(col[0], col[1], col[2], 255), FillRule::Winding, Transform::identity(), None);
    }
}

const AMBER: Rgb = [240, 170, 60];

pub fn draw(st: &State, pm: &mut Pixmap, text: &mut Text, now_ms: i64) {
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
                        let s = (2.0 * r) / pic.width() as f32;
                        let tf = Transform::from_scale(s, s).post_translate(cx - r, mid - r);
                        pm.draw_pixmap(0, 0, pic.as_ref(), &PixmapPaint { opacity: alpha, quality: tiny_skia::FilterQuality::Bilinear, ..PixmapPaint::default() }, tf, None);
                    }
                    None => {
                        let (a, b, c) = crate::render::hue_rgb(&p.id);
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
                let rw = text.width(&right, 11.0 * u);
                let name = fit(text, &p.name, 13.0 * u, rect.w - (x0 + 2.0 * r + 12.0 * u + 14.0 * u) - rw);
                text.draw(pm, &name, x0 + 2.0 * r + 10.0 * u, mid + 4.5 * u, 13.0 * u, if p.online { INK } else { INK_FAINT }, 0.0);
                text.draw(pm, &right, rect.w - PAD * u - 6.0 * u, mid + 4.0 * u, 11.0 * u, INK_FAINT, 1.0);
            }
            Row::Menu { can_join, .. } => {
                let (join, wave) = menu_buttons(rect, u);
                if *can_join {
                    fill(pm, &join, 6.0 * u, BRAND, 255);
                } else {
                    outline(pm, &join, 6.0 * u, LINE);
                }
                text.draw(pm, "Join", join.x + join.w / 2.0, join.y + join.h / 2.0 + 4.5 * u, 12.0 * u, if *can_join { ON_BRAND } else { INK_FAINT }, 0.5);
                outline(pm, &wave, 6.0 * u, LINE);
                text.draw(pm, "Wave (soon)", wave.x + wave.w / 2.0, wave.y + wave.h / 2.0 + 4.5 * u, 12.0 * u, INK_FAINT, 0.5);
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
    outline(pm, &l.logout, 6.0 * u, LINE);
    text.draw(pm, "Log out", l.logout.x + l.logout.w / 2.0, l.logout.y + l.logout.h / 2.0 + 4.5 * u, 12.0 * u, INK_MUTED, 0.5);
    let edge = R { x: l.panel.w - 1.0, y: 0.0, w: 1.0, h };
    fill(pm, &edge, 0.0, LINE, 255);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::{Net, User};
    use crate::roster::Member;

    fn user(id: &str, name: &str, zone: Option<&str>) -> User {
        User { id: id.into(), name: name.into(), zone: zone.map(String::from), status: "online".into(), mode: "cluster".into(), ..User::default() }
    }

    fn state() -> State {
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
    fn rows_react_to_clicks() {
        let mut st = state();
        st.ui = 1.0;
        let l = layout(&st, 700.0);
        let (_, ana) = l.items.iter().find(|(r, _)| matches!(r, Row::Person { p, .. } if p.id == "d")).unwrap();
        assert_eq!(hit(&st, 100.0, ana.y + 5.0, 700.0), Some(Hit::Row("d".into())));
        let (_, me) = l.items.iter().find(|(r, _)| matches!(r, Row::Person { p, .. } if p.me)).unwrap();
        assert_eq!(hit(&st, 100.0, me.y + 5.0, 700.0), Some(Hit::Panel), "I cannot open a menu on myself");
        let (_, eve) = l.items.iter().find(|(r, _)| matches!(r, Row::Person { p, .. } if p.id == "e")).unwrap();
        assert_eq!(hit(&st, 100.0, eve.y + 5.0, 700.0), Some(Hit::Panel), "offline people are not clickable");
        assert_eq!(hit(&st, l.away.x + 2.0, l.away.y + 2.0, 700.0), Some(Hit::Away));
        assert_eq!(hit(&st, l.logout.x + 2.0, l.logout.y + 2.0, 700.0), Some(Hit::Logout));
        assert_eq!(hit(&st, 600.0, 300.0, 700.0), None, "the map starts right of the panel");
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
