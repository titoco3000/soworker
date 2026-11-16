//! Where each panel of the window goes: the people list, the main panel (map and meeting), the chat, the settings and
//! the navigation. A wide window has a sidebar on the left (people list, then the chat under a line that can be
//! dragged, then the nav with the availability button, the memory use and the settings button). A window narrower
//! than 500 px shows one panel at a time between a header (availability, app name, settings) and a footer (people,
//! main, chat).
use crate::app::State;
use crate::directory::View;
use crate::gui::icons;
use crate::gui::meeting_ui::R;
use crate::gui::render::Text;
use crate::gui::sidebar::fill;
use crate::gui::theme::*;
use tiny_skia::{FillRule, PathBuilder, PixmapMut, Transform};

/// Narrower windows (logical pixels) get the mobile layout.
pub const MOBILE_MAX: f32 = 500.0;
/// The sidebar's nav row at the bottom.
pub const NAV: f32 = 36.0;
/// The collapsed sidebar's nav: the settings button above the availability button.
pub const NAV_NARROW: f32 = 72.0;
/// The mobile header and footer.
pub const HEADER: f32 = 44.0;
pub const FOOTER: f32 = 56.0;
/// When the line between them is dragged, the people list never gets smaller than this...
pub const SPLIT_MIN: f32 = 80.0;
/// ...and the chat never smaller than this.
pub const CHAT_MIN: f32 = 30.0;
/// How far from the line a press still grabs it.
const SPLIT_GRAB: f32 = 4.0;

pub fn is_mobile(st: &State, w: f32) -> bool {
    w / st.ui.max(0.1) < MOBILE_MAX
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Frame {
    pub mobile: bool,
    /// The whole sidebar column (wide window), nav included; empty when there is none.
    pub side: R,
    /// The people list (empty when hidden).
    pub users: R,
    /// The draggable line between the people list and the chat (empty when there is none).
    pub split: R,
    /// The chat (empty when hidden).
    pub chat: R,
    /// The sidebar's nav row (wide window).
    pub nav: R,
    /// The map and the meeting dock. On a narrow window it is the space between header and footer, even while
    /// another panel covers it, so the map keeps its place.
    pub main: R,
    /// The main panel is on screen (not covered by another panel of the narrow layout).
    pub main_shown: bool,
    /// Mobile: the header and the footer, and the space between them.
    pub header: R,
    pub footer: R,
    pub content: R,
}

/// The empty rectangle: contains nothing.
const NONE: R = R { x: -1.0, y: -1.0, w: 0.0, h: 0.0 };

pub fn empty(r: &R) -> bool {
    r.w <= 0.0 || r.h <= 0.0
}

pub fn frame(st: &State, w: f32, h: f32) -> Frame {
    let u = st.ui;
    let whole = R { x: 0.0, y: 0.0, w, h };
    let none = Frame { side: NONE, users: NONE, split: NONE, chat: NONE, nav: NONE, header: NONE, footer: NONE, content: NONE, ..Frame::default() };
    if st.login.is_some() || st.dock.tile_full {
        return Frame { main: whole, main_shown: true, content: whole, ..none };
    }
    if st.sidebar.mobile {
        let header = R { x: 0.0, y: 0.0, w, h: HEADER * u };
        let footer = R { x: 0.0, y: (h - FOOTER * u).max(header.h), w, h: FOOTER * u };
        let content = R { x: 0.0, y: header.h, w, h: (footer.y - header.h).max(0.0) };
        let pick = |v: View| if st.sidebar.view == v { content } else { NONE };
        return Frame { mobile: true, users: pick(View::Users), chat: pick(View::Chat), main: content, main_shown: st.sidebar.view == View::Main, header, footer, content, ..none };
    }
    let sw = crate::gui::sidebar::full_width(st);
    let nh = if st.sidebar.collapsed { NAV_NARROW } else { NAV } * u;
    let nav = R { x: 0.0, y: (h - nh).max(0.0), w: sw, h: nh };
    let side = R { x: 0.0, y: 0.0, w: sw, h };
    let main = R { x: sw, y: 0.0, w: (w - sw).max(0.0), h };
    let avail = nav.y;
    if st.sidebar.collapsed {
        let users = R { x: 0.0, y: 0.0, w: sw, h: avail };
        return Frame { side, users, nav, main, main_shown: true, content: main, ..none };
    }
    let uh = split_px(st, avail);
    let users = R { x: 0.0, y: 0.0, w: sw, h: uh };
    let chat = R { x: 0.0, y: uh, w: sw, h: avail - uh };
    let split = R { x: 0.0, y: uh - SPLIT_GRAB * u, w: sw, h: 2.0 * SPLIT_GRAB * u };
    Frame { side, users, split, chat, nav, main, main_shown: true, content: main, ..none }
}

/// Height of the people list in pixels, for `avail` pixels shared with the chat.
fn split_px(st: &State, avail: f32) -> f32 {
    let u = st.ui;
    let want = if st.sidebar.split > 0.0 { st.sidebar.split * u } else { avail * 0.55 };
    let (lo, hi) = (SPLIT_MIN * u, (avail - CHAT_MIN * u).max(SPLIT_MIN * u));
    want.clamp(lo, hi).min(avail)
}

/// Settings, Terminal... the buttons of the navigation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nav {
    Settings,
    /// The availability button: available / away.
    Away,
    View(View),
}

struct NavLayout {
    gear: R,
    away: R,
    /// Mobile footer: people, main, chat.
    views: Vec<(View, R)>,
}

fn nav_layout(st: &State, f: &Frame) -> NavLayout {
    let u = st.ui;
    if f.mobile {
        let s = 36.0 * u;
        let gear = R { x: f.header.w - s - 6.0 * u, y: f.header.y + (f.header.h - s) / 2.0, w: s, h: s };
        let a = 28.0 * u;
        let away = R { x: 10.0 * u, y: f.header.y + (f.header.h - a) / 2.0, w: a, h: a };
        let bw = f.footer.w / 3.0;
        let views = [View::Users, View::Main, View::Chat].into_iter().enumerate().map(|(i, v)| (v, R { x: i as f32 * bw, y: f.footer.y, w: bw, h: f.footer.h })).collect();
        return NavLayout { gear, away, views };
    }
    if empty(&f.nav) {
        return NavLayout { gear: NONE, away: NONE, views: vec![] };
    }
    let s = 28.0 * u;
    if st.sidebar.collapsed {
        // one column: the settings button above the availability button
        let x = (f.nav.w - s) / 2.0;
        let gear = R { x, y: f.nav.y + 4.0 * u, w: s, h: s };
        let away = R { x, y: f.nav.y + f.nav.h - s - 4.0 * u, w: s, h: s };
        return NavLayout { gear, away, views: vec![] };
    }
    let y = f.nav.y + (f.nav.h - s) / 2.0;
    let away = R { x: 8.0 * u, y, w: s, h: s };
    let gear = R { x: f.nav.w - s - 8.0 * u, y, w: s, h: s };
    NavLayout { gear, away, views: vec![] }
}

pub fn nav_hit(st: &State, px: f32, py: f32, w: f32, h: f32) -> Option<Nav> {
    let f = frame(st, w, h);
    let l = nav_layout(st, &f);
    if l.gear.contains(px, py) {
        return Some(Nav::Settings);
    }
    if l.away.contains(px, py) {
        return Some(Nav::Away);
    }
    l.views.iter().find(|(_, r)| r.contains(px, py)).map(|(v, _)| Nav::View(*v))
}

/// The nav bar itself (not a button): swallows the click.
pub fn over_nav(st: &State, px: f32, py: f32, w: f32, h: f32) -> bool {
    let f = frame(st, w, h);
    f.nav.contains(px, py) || f.header.contains(px, py) || f.footer.contains(px, py)
}

pub fn nav_press(st: &mut State, n: Nav, now: f64) {
    match n {
        Nav::Settings => st.toggle_profile(),
        Nav::Away => st.toggle_available(now),
        Nav::View(v) => {
            st.sidebar.view = v;
            st.profile = None;
        }
    }
    st.sync_chat_shown();
    st.dirty = true;
}

/// A press on the line between the people list and the chat starts dragging it.
pub fn begin_split(st: &mut State, px: f32, py: f32, w: f32, h: f32) -> bool {
    let f = frame(st, w, h);
    if empty(&f.split) || !f.split.contains(px, py) {
        return false;
    }
    st.sidebar.split_drag = Some(py - f.chat.y);
    true
}

pub fn over_split(st: &State, px: f32, py: f32, w: f32, h: f32) -> bool {
    let f = frame(st, w, h);
    st.sidebar.split_drag.is_some() || (!empty(&f.split) && f.split.contains(px, py))
}

pub fn drag_split(st: &mut State, py: f32, w: f32, h: f32) {
    let Some(grab) = st.sidebar.split_drag else { return };
    let f = frame(st, w, h);
    let avail = f.nav.y;
    let u = st.ui;
    let px = (py - grab).clamp(SPLIT_MIN * u, (avail - CHAT_MIN * u).max(SPLIT_MIN * u));
    st.sidebar.split = px / u;
    st.dirty = true;
}

/// Let go of the line, and remember where it is.
pub fn end_split(st: &mut State) {
    if st.sidebar.split_drag.take().is_some() {
        crate::calls::save_setting("sidebarSplit", (st.sidebar.split as f64).into());
    }
}

fn badge(pm: &mut PixmapMut<'_>, text: &mut Text, n: usize, cx: f32, cy: f32, u: f32) {
    let label = if n > 99 { "99+".to_string() } else { n.to_string() };
    let bw = (text.width(&label, 10.0 * u) + 8.0 * u).max(16.0 * u);
    let r = R { x: cx - bw / 2.0, y: cy - 8.0 * u, w: bw, h: 16.0 * u };
    fill(pm, &r, 8.0 * u, BRAND, 255);
    text.draw(pm, &label, cx, cy + 3.5 * u, 10.0 * u, ON_BRAND, 0.5);
}

/// The small chat bubble on the top right of an avatar that has unread messages.
pub fn unread_mark(pm: &mut PixmapMut<'_>, cx: f32, cy: f32, u: f32) {
    let r = 6.5 * u;
    if let Some(c) = PathBuilder::from_circle(cx, cy, r) {
        pm.fill_path(&c, &crate::gui::render::paint(SURFACE[0], SURFACE[1], SURFACE[2], 255), FillRule::Winding, Transform::identity(), None);
    }
    if let Some(c) = PathBuilder::from_circle(cx, cy, r - 1.0 * u) {
        pm.fill_path(&c, &crate::gui::render::paint(BRAND[0], BRAND[1], BRAND[2], 255), FillRule::Winding, Transform::identity(), None);
    }
    let s = 8.0 * u;
    crate::gui::icon::draw(pm, icons::MESSAGE_CIRCLE, cx - s / 2.0, cy - s / 2.0, s, ON_BRAND);
}

fn gear(st: &State, pm: &mut PixmapMut<'_>, r: &R) {
    let u = st.ui;
    let open = st.profile.is_some();
    if open {
        fill(pm, r, 6.0 * u, LINE, 255);
    }
    let s = 18.0 * u;
    crate::gui::icon::draw(pm, icons::SETTINGS, r.x + (r.w - s) / 2.0, r.y + (r.h - s) / 2.0, s, if open { INK } else { INK_MUTED });
}

/// The availability button: a user icon on green (available) or red (away).
fn away(st: &State, pm: &mut PixmapMut<'_>, r: &R) {
    let u = st.ui;
    fill(pm, r, 6.0 * u, if st.available { [47, 111, 79] } else { [106, 47, 58] }, 255);
    let s = 16.0 * u;
    crate::gui::icon::draw(pm, icons::USER, r.x + (r.w - s) / 2.0, r.y + (r.h - s) / 2.0, s, INK);
}

/// The sidebar background and its line on the right; drawn under the people list and the chat.
pub fn draw_back(st: &State, pm: &mut PixmapMut<'_>) {
    let (w, h) = (pm.width() as f32, pm.height() as f32);
    let f = frame(st, w, h);
    if !empty(&f.side) {
        fill(pm, &f.side, 0.0, SURFACE, 255);
    }
}

/// The nav: the sidebar's bottom row, or the mobile header and footer. Drawn over the panels it borders.
pub fn draw_nav(st: &State, pm: &mut PixmapMut<'_>, text: &mut Text) {
    let (w, h) = (pm.width() as f32, pm.height() as f32);
    let f = frame(st, w, h);
    let u = st.ui;
    let l = nav_layout(st, &f);
    if f.mobile {
        fill(pm, &f.header, 0.0, SURFACE, 255);
        fill(pm, &R { y: f.header.y + f.header.h - 1.0, h: 1.0, ..f.header }, 0.0, LINE, 255);
        text.draw(pm, "SoWorker", f.header.w / 2.0, f.header.y + f.header.h / 2.0 + 5.5 * u, 16.0 * u, INK, 0.5);
        gear(st, pm, &l.gear);
        away(st, pm, &l.away);
        fill(pm, &f.footer, 0.0, SURFACE, 255);
        fill(pm, &R { h: 1.0, ..f.footer }, 0.0, LINE, 255);
        let unread = st.unread_total();
        for (v, r) in &l.views {
            let on = st.sidebar.view == *v && st.profile.is_none();
            let (shape, label) = match v {
                View::Users => (icons::USERS, "People"),
                View::Main => (icons::MAP, "Office"),
                View::Chat => (icons::MESSAGE_CIRCLE, "Chat"),
            };
            let col = if on { BRAND } else { INK_MUTED };
            let s = 20.0 * u;
            let (cx, iy) = (r.x + r.w / 2.0, r.y + 8.0 * u);
            crate::gui::icon::draw(pm, shape, cx - s / 2.0, iy, s, col);
            text.draw(pm, label, cx, r.y + r.h - 9.0 * u, 11.0 * u, col, 0.5);
            if *v == View::Chat && unread > 0 {
                badge(pm, text, unread, cx + s * 0.75, iy + 2.0 * u, u);
            }
        }
        return;
    }
    if empty(&f.nav) {
        return;
    }
    fill(pm, &f.nav, 0.0, SURFACE, 255);
    fill(pm, &R { h: 1.0, ..f.nav }, 0.0, LINE, 255);
    gear(st, pm, &l.gear);
    away(st, pm, &l.away);
    if !st.sidebar.collapsed {
        text.draw(pm, &format!("Memory {:.1} MB", crate::gui::sidebar::pss_mb()), f.nav.w / 2.0, f.nav.y + f.nav.h / 2.0 + 4.0 * u, 11.0 * u, INK_FAINT, 0.5);
    }
    // the line between the people list and the chat
    if !empty(&f.split) {
        let y = f.chat.y;
        let hot = st.sidebar.split_drag.is_some();
        fill(pm, &R { x: 0.0, y: y - if hot { 1.0 } else { 0.0 }, w: f.side.w, h: if hot { 2.0 } else { 1.0 } }, 0.0, if hot { BRAND } else { LINE }, 255);
        let gw = 28.0 * u;
        fill(pm, &R { x: (f.side.w - gw) / 2.0, y: y - 1.5 * u, w: gw, h: 3.0 * u }, 1.5 * u, if hot { BRAND } else { INK_FAINT }, 255);
    }
    fill(pm, &R { x: f.side.w - 1.0, y: 0.0, w: 1.0, h: f.side.h }, 0.0, LINE, 255);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::directory::tests::state;

    #[test]
    fn wide_window_has_a_sidebar_with_people_chat_and_nav() {
        let mut st = state();
        st.ui = 1.0;
        st.sidebar.collapsed = false;
        let f = frame(&st, 1000.0, 700.0);
        assert!(!f.mobile);
        assert_eq!((f.users.y, f.users.y + f.users.h), (0.0, f.chat.y), "the chat sits right under the people list");
        assert_eq!(f.chat.y + f.chat.h, f.nav.y, "then the nav");
        assert_eq!(f.nav.y + f.nav.h, 700.0);
        assert_eq!(f.main.x, f.side.w);
        assert_eq!(nav_hit(&st, f.nav.x + 12.0, f.nav.y + f.nav.h / 2.0, 1000.0, 700.0), Some(Nav::Away), "availability on the left");
        assert_eq!(nav_hit(&st, f.nav.w - 12.0, f.nav.y + f.nav.h / 2.0, 1000.0, 700.0), Some(Nav::Settings), "settings on the right");
        nav_press(&mut st, Nav::Away, 0.0);
        assert!(!st.available && st.toast.as_ref().is_some_and(|(t, _)| t.contains("away")), "the new state is announced");
    }

    #[test]
    fn the_split_is_dragged_down_to_twenty_pixels() {
        let mut st = state();
        st.ui = 1.0;
        st.sidebar.collapsed = false;
        let f = frame(&st, 1000.0, 700.0);
        assert!(begin_split(&mut st, 50.0, f.chat.y, 1000.0, 700.0));
        drag_split(&mut st, 2.0, 1000.0, 700.0);
        assert_eq!(frame(&st, 1000.0, 700.0).users.h, SPLIT_MIN);
        drag_split(&mut st, 699.0, 1000.0, 700.0);
        let f = frame(&st, 1000.0, 700.0);
        assert_eq!(f.chat.h, CHAT_MIN);
        st.sidebar.split_drag = None;
    }

    #[test]
    fn collapsed_sidebar_hides_the_chat() {
        let mut st = state();
        st.ui = 1.0;
        st.sidebar.collapsed = true;
        let f = frame(&st, 1000.0, 700.0);
        assert!(empty(&f.chat) && empty(&f.split));
        let mid = f.nav.x + f.nav.w / 2.0;
        assert_eq!(nav_hit(&st, mid, f.nav.y + 10.0, 1000.0, 700.0), Some(Nav::Settings), "settings on top");
        assert_eq!(nav_hit(&st, mid, f.nav.y + f.nav.h - 10.0, 1000.0, 700.0), Some(Nav::Away), "availability under them");
        assert!(!st.chat_shown());
    }

    #[test]
    fn narrow_window_shows_one_panel_between_header_and_footer() {
        let mut st = state();
        st.ui = 1.0;
        st.sidebar.mobile = true;
        st.sidebar.view = View::Users;
        let f = frame(&st, 360.0, 700.0);
        assert!(f.mobile && !f.main_shown && empty(&f.chat));
        assert_eq!((f.users.y, f.users.h), (HEADER, 700.0 - HEADER - FOOTER));
        let chat = f.footer.x + f.footer.w * 5.0 / 6.0;
        assert_eq!(nav_hit(&st, chat, f.footer.y + 10.0, 360.0, 700.0), Some(Nav::View(View::Chat)));
        nav_press(&mut st, Nav::View(View::Chat), 0.0);
        assert!(st.chat.open, "the chat is on screen, so it is read");
        assert_eq!(nav_hit(&st, 350.0, 20.0, 360.0, 700.0), Some(Nav::Settings));
        assert_eq!(nav_hit(&st, 15.0, 20.0, 360.0, 700.0), Some(Nav::Away), "availability on the left of the header");
        nav_press(&mut st, Nav::Settings, 0.0);
        assert!(st.profile.is_some() && !st.chat.open, "settings cover the chat");
    }

    /// Layout check: `SOWORKER_SHOTS=<dir> cargo test panels::tests::shots -- --ignored` writes the window at a
    /// desktop and a phone size, each layout, as PNG files.
    #[test]
    #[ignore]
    fn shots() {
        let Some(dir) = std::env::var_os("SOWORKER_SHOTS") else { return };
        let dir = std::path::PathBuf::from(dir);
        let mut text = Text::new();
        let shot = |st: &mut State, text: &mut Text, w: u32, h: u32, name: &str| {
            st.sidebar.mobile = is_mobile(st, w as f32);
            st.sync_chat_shown();
            let mut pm = tiny_skia::Pixmap::new(w, h).unwrap();
            crate::gui::render::draw(st, &mut pm.as_mut(), text, &mut crate::gui::render::MapCache::default());
            pm.save_png(dir.join(format!("{name}.png"))).unwrap();
        };
        let mut st = state();
        st.ui = 1.0;
        st.sidebar.collapsed = false;
        // a desk we are not at, so the desk button shows
        st.room_def = "R".into();
        st.desks.insert("k".into(), (5000.0, 5000.0));
        st.desk_owners = Some(vec![crate::desks::Desk { room: "R".into(), desk: "k".into(), owner: "me".into() }]);
        st.apply_chat(crate::chat::ChatEvent::Channel { id: "D1".into(), kind: crate::chat::Kind::Direct("d".into()) }, 0.0);
        st.apply_chat(crate::chat::ChatEvent::Message(crate::chat::ChatMsg { id: "1".into(), channel: "D1".into(), user: "d".into(), name: Some("Di".into()), text: "hello there, are you around for a quick call?".into(), at: 1 }), 0.0);
        shot(&mut st, &mut text, 1000, 700, "desktop");
        st.open_dm("d");
        st.sidebar.open = Some("c".into());
        shot(&mut st, &mut text, 1000, 700, "desktop-dm");
        st.sidebar.split = 120.0;
        st.toggle_profile();
        shot(&mut st, &mut text, 1000, 700, "desktop-settings");
        st.profile = None;
        st.sidebar.collapsed = true;
        st.chat.tab = crate::chatstate::Tab::Area;
        st.apply_chat(crate::chat::ChatEvent::Message(crate::chat::ChatMsg { id: "2".into(), channel: "D1".into(), user: "d".into(), name: Some("Di".into()), text: "ping".into(), at: 2 }), 0.0);
        shot(&mut st, &mut text, 1000, 700, "desktop-collapsed");
        for v in [View::Users, View::Main, View::Chat] {
            st.sidebar.view = v;
            shot(&mut st, &mut text, 360, 700, &format!("mobile-{v:?}").to_lowercase());
        }
        st.toggle_profile();
        shot(&mut st, &mut text, 360, 700, "mobile-settings");
    }
}
