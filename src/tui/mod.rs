//! The terminal client: ratatui for drawing, one channel for everything that happens (keys, mouse, the socket, calls).
//! The loop sleeps until something arrives, applies it to `State`, and redraws only when the state says so. Video is the
//! only graphics: inline where the terminal speaks the kitty protocol, otherwise in a small window of its own.
mod kitty;
mod tags;
mod view;

use crate::app::{Frame, LoginPhase, State};
use crate::backend::{Backend, Emit, Event};
use crate::calls::MediaEvent;
use crate::directory::{Hit as Side, MenuBtn, Row};
use crate::meeting::Ctl;
use crate::proto::Net;
use crate::session::{self, Auth};
use ratatui::crossterm::event::{
    DisableFocusChange, DisableMouseCapture, EnableFocusChange, EnableMouseCapture, Event as Term, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, KeyboardEnhancementFlags, MouseButton, MouseEvent,
    MouseEventKind, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use ratatui::crossterm::execute;
use ratatui::layout::Rect;
use std::collections::HashSet;
use std::sync::mpsc::{channel, Receiver, RecvTimeoutError};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// A key that is not reported as released stops walking this long after the last press or repeat.
const HOLD: f64 = 0.14;
const FRAME: Duration = Duration::from_millis(33);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Focus {
    Map,
    People,
    Chat,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Modal {
    None,
    Logout,
    Help,
    /// Who is signed in, log out, and the speaker / microphone / camera settings (`State::profile`).
    Profile,
}

enum Msg {
    App(Event),
    Term(Term),
}

/// Where video goes.
enum Video {
    Off,
    Inline(kitty::Kitty),
    Window(Option<crate::videowin::VideoWindow>),
}

pub struct Tui {
    pub st: State,
    be: Backend,
    pub focus: Focus,
    pub modal: Modal,
    pub sel: usize,
    pub menu_sel: usize,
    pub show_people: bool,
    pub zoom: i32,
    /// Walking keys that the terminal never reports as released: when each one lapses.
    lapse: [f64; 4],
    /// The terminal reports key releases (kitty keyboard protocol), so keys are held for real.
    enhanced: bool,
    video: Video,
    pick: Option<String>,
    shown: Option<(String, u64, Rect)>,
    frame_no: u64,
    areas: view::Areas,
    start: Instant,
    quit: bool,
    /// Leave for the window client once the terminal is back to normal.
    to_gui: bool,
    dry: bool,
    /// Memory this process uses (MB, proportional share), and when it was read.
    pub mem: (f32, Instant),
    /// Everyone's short tag (map and people list), by id, and the (id, name) list it was made from.
    pub tags: std::collections::HashMap<String, String>,
    tags_of: Vec<(String, String)>,
}

impl Tui {
    fn new(dry: bool, video: Video, be: Backend) -> Self {
        Tui {
            st: State::new("", dry),
            be,
            focus: Focus::Map,
            modal: Modal::None,
            sel: 0,
            menu_sel: 0,
            show_people: true,
            zoom: crate::calls::setting("tuiZoom").as_i64().map_or(1, |z| z.clamp(-2, 3) as i32),
            lapse: [f64::MAX; 4],
            enhanced: false,
            video,
            pick: None,
            shown: None,
            frame_no: 0,
            areas: Default::default(),
            start: Instant::now(),
            quit: false,
            to_gui: false,
            dry,
            mem: (proc_mb(), Instant::now()),
            tags: Default::default(),
            tags_of: vec![],
        }
    }

    #[cfg(test)]
    pub fn for_test() -> Self {
        Self::new(true, Video::Off, Backend::new(Arc::new(|_| {}), true, String::new()))
    }

    pub fn video_inline(&self) -> bool {
        matches!(self.video, Video::Inline(_))
    }

    pub fn window_capable(&self) -> bool {
        matches!(self.video, Video::Window(_))
    }

    pub fn window_open(&self) -> bool {
        matches!(&self.video, Video::Window(Some(w)) if w.alive())
    }

    fn now(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    // ------------------------------------------------------------ video

    /// The tile to show inline: the one picked, else a screen share, else someone's camera, else mine.
    pub fn picked_key(&self) -> Option<String> {
        let tiles = &self.st.call.tiles;
        if let Some(p) = self.pick.as_ref().filter(|p| tiles.iter().any(|t| &t.key == *p)) {
            return Some(p.clone());
        }
        let find = |f: &dyn Fn(&str) -> bool| tiles.iter().find(|t| f(&t.key)).map(|t| t.key.clone());
        find(&|k| k.ends_with(":screen") && !k.starts_with("me:")).or_else(|| find(&|k| k != "me" && !k.starts_with("me:"))).or_else(|| find(&|_| true))
    }

    fn next_picture(&mut self) {
        let keys: Vec<String> = self.st.call.tiles.iter().map(|t| t.key.clone()).collect();
        if keys.is_empty() {
            return;
        }
        let cur = self.picked_key();
        let i = keys.iter().position(|k| Some(k) == cur.as_ref()).map_or(0, |i| (i + 1) % keys.len());
        self.pick = Some(keys[i].clone());
        self.shown = None;
        self.sync_wanted();
    }

    /// Tell the media thread which pictures to convert: all for the window, the one in view inline, none otherwise.
    fn sync_wanted(&self) {
        // the window is big, so it gets sharper pictures; the inline pane is small and every byte goes through the terminal
        match self.video {
            Video::Window(_) => crate::calls::set_video_width(960, 1920),
            _ => crate::calls::set_video_width(640, 1280),
        }
        crate::calls::want_video(match &self.video {
            Video::Off => Some(HashSet::new()),
            Video::Inline(_) => Some(self.picked_key().into_iter().collect()),
            Video::Window(_) => None,
        });
    }

    fn open_window(&mut self) {
        if let Video::Window(w) = &mut self.video {
            if !w.as_ref().is_some_and(|w| w.alive()) {
                *w = crate::videowin::open();
                if let Some(w) = w {
                    for t in self.st.call.tiles.iter().filter(|t| t.img.w > 0) {
                        w.frame(&t.key, &t.name, &t.img);
                    }
                }
            }
        }
    }

    /// A media event, with the video sink kept in step.
    fn media(&mut self, ev: MediaEvent, now: f64) {
        match ev {
            MediaEvent::Video { ref key, w: 0, .. } if self.st.call.tiles.iter().any(|t| &t.key == key) => return, // "still there", nothing to draw
            MediaEvent::Video { ref key, ref name, w, h, .. } => {
                let (key, name) = (key.clone(), name.clone());
                let fresh = !self.st.call.tiles.iter().any(|t| t.key == key);
                self.st.apply_media(ev, now);
                if fresh {
                    self.sync_wanted();
                }
                if w > 0 && h > 0 {
                    if fresh && matches!(&self.video, Video::Window(w) if !w.as_ref().is_some_and(|w| w.alive())) {
                        self.open_window();
                    }
                    if let (Video::Window(Some(win)), Some(t)) = (&self.video, self.st.call.tiles.iter().find(|t| t.key == key)) {
                        win.frame(&key, &name, &t.img);
                    }
                    if self.picked_key().as_deref() == Some(key.as_str()) {
                        self.frame_no += 1;
                    }
                }
                return;
            }
            MediaEvent::VideoGone(ref key) => {
                if let Video::Window(Some(w)) = &self.video {
                    w.gone(key);
                }
                self.shown = None;
            }
            MediaEvent::Left => {
                self.pick = None;
                self.shown = None;
                if let Video::Window(w) = &mut self.video {
                    *w = None;
                }
                if let Video::Inline(k) = &mut self.video {
                    k.hide();
                }
            }
            _ => {}
        }
        self.st.apply_media(ev, now);
        self.sync_wanted();
    }

    /// After a draw: put the picture in view where the layout says.
    fn paint_video(&mut self) {
        let (area, key) = (self.areas.video.filter(|_| self.modal == Modal::None), self.picked_key());
        let Video::Inline(k) = &mut self.video else { return };
        let Some(area) = area else {
            self.shown = None;
            return k.hide();
        };
        let frame: Option<&Frame> = key.as_ref().and_then(|key| self.st.call.tiles.iter().find(|t| &t.key == key)).map(|t| &t.img).filter(|f| f.w > 0);
        match (frame, key) {
            (Some(f), Some(key)) => {
                if self.shown.as_ref() != Some(&(key.clone(), self.frame_no, area)) {
                    k.show(f, area);
                    self.shown = Some((key, self.frame_no, area));
                }
            }
            _ => k.hide(),
        }
    }

    // ------------------------------------------------------------ people panel

    fn rows(&self) -> Vec<Row> {
        view::people_rows(self)
    }

    /// First row shown so that the selected one is in view.
    pub fn people_off(&self, rows: usize, h: usize) -> usize {
        if rows <= h {
            return 0;
        }
        self.sel.saturating_sub(h / 2).min(rows - h)
    }

    fn move_sel(&mut self, down: bool) {
        let rows = self.rows();
        if rows.is_empty() {
            return;
        }
        let mut i = self.sel.min(rows.len() - 1);
        loop {
            i = if down { i + 1 } else { i.wrapping_sub(1) };
            if i >= rows.len() {
                return;
            }
            if !matches!(rows[i], Row::Section(_)) {
                self.sel = i;
                self.menu_sel = 0;
                return;
            }
        }
    }

    /// Enter on the selected row: open its menu, run the highlighted button, or answer a wave.
    fn activate(&mut self) {
        let rows = self.rows();
        let Some(row) = rows.get(self.sel).cloned() else { return };
        let now = self.now();
        match row {
            Row::Section(_) => {}
            Row::Group { key, .. } | Row::Person { p: crate::directory::Person { id: key, .. }, .. } => {
                let same = self.st.sidebar.open.as_deref() == Some(key.as_str());
                self.st.sidebar.open = if same { None } else { Some(key) };
                if !same {
                    self.sel += 1; // onto the menu that just opened under it
                    self.menu_sel = 0;
                    if let Some(Row::Menu { buttons, .. }) = self.rows().get(self.sel) {
                        self.menu_sel = buttons.iter().position(|b| !matches!(b, MenuBtn::Join { enabled: false })).unwrap_or(0);
                    }
                }
            }
            Row::Menu { key, buttons } => {
                let Some(b) = buttons.get(self.menu_sel.min(buttons.len().saturating_sub(1))) else { return };
                let hit = match b {
                    MenuBtn::Join { enabled: true } => Side::Join(key),
                    MenuBtn::Join { enabled: false } => return self.st.say("Cannot find where to walk to", now),
                    MenuBtn::Wave => Side::Wave(key),
                    MenuBtn::Message => Side::Message(key),
                };
                self.st.sidebar_press(hit, now);
                self.sel = self.sel.saturating_sub(1);
                self.sync_focus();
            }
            Row::Wave { from, .. } => self.st.sidebar_press(Side::WaveAccept(from), now),
        }
    }

    fn people_key(&mut self, k: KeyEvent) {
        let rows = self.rows();
        let on_menu = matches!(rows.get(self.sel), Some(Row::Menu { .. }));
        match k.code {
            KeyCode::Up | KeyCode::Char('k') => self.move_sel(false),
            KeyCode::Down | KeyCode::Char('j') => self.move_sel(true),
            KeyCode::Left | KeyCode::Char('h') if on_menu => self.menu_sel = self.menu_sel.saturating_sub(1),
            KeyCode::Right | KeyCode::Char('l') if on_menu => {
                if let Some(Row::Menu { buttons, .. }) = rows.get(self.sel) {
                    self.menu_sel = (self.menu_sel + 1).min(buttons.len().saturating_sub(1));
                }
            }
            KeyCode::Enter | KeyCode::Char(' ') => self.activate(),
            KeyCode::Char('x') | KeyCode::Delete => {
                if let Some(Row::Wave { from, .. }) = rows.get(self.sel) {
                    let now = self.now();
                    self.st.sidebar_press(Side::WaveDismiss(from.clone()), now);
                }
            }
            KeyCode::Esc | KeyCode::Tab => {
                if on_menu && k.code == KeyCode::Esc {
                    self.st.sidebar.open = None;
                    self.sel = self.sel.saturating_sub(1);
                } else {
                    self.focus = Focus::Map;
                }
            }
            _ => {}
        }
        let n = self.rows().len();
        self.sel = self.sel.min(n.saturating_sub(1));
    }

    // ------------------------------------------------------------ input

    fn sync_focus(&mut self) {
        if self.st.chat.focus && self.focus != Focus::Chat {
            self.focus = Focus::Chat;
        } else if !self.st.chat.focus && self.focus == Focus::Chat {
            self.focus = Focus::Map;
        }
        self.st.chat.focus = self.focus == Focus::Chat;
    }

    fn set_dir(&mut self, i: usize, down: bool) {
        let now = self.now();
        let k = &mut self.st.keys;
        match i {
            0 => k.up = down,
            1 => k.down = down,
            2 => k.left = down,
            _ => k.right = down,
        }
        self.lapse[i] = if down && !self.enhanced { now + HOLD } else { f64::MAX };
        self.st.dirty = true;
    }

    fn key(&mut self, k: KeyEvent) {
        let now = self.now();
        let down = k.kind != KeyEventKind::Release;
        if k.modifiers.contains(KeyModifiers::CONTROL) && k.code == KeyCode::Char('c') {
            self.quit = true;
            return;
        }
        if self.modal != Modal::None {
            if !down {
                return;
            }
            if self.modal == Modal::Profile {
                return self.profile_key(k);
            }
            match (self.modal, k.code) {
                (Modal::Logout, KeyCode::Char('y') | KeyCode::Enter) => self.st.logout_choice(true),
                (Modal::Logout, KeyCode::Char('n') | KeyCode::Esc) => self.st.logout_choice(false),
                (Modal::Logout, _) => return,
                _ => {}
            }
            self.modal = Modal::None;
            self.st.dirty = true;
            return;
        }
        if let Some(phase) = self.st.login.clone() {
            if !down {
                return;
            }
            match (k.code, phase) {
                (KeyCode::Char('q'), _) => self.quit = true,
                (KeyCode::Esc, LoginPhase::Waiting) => self.st.login_act = Some(crate::app::LoginAct::Cancel),
                (KeyCode::Enter, LoginPhase::Offline) => self.st.login_act = Some(crate::app::LoginAct::Retry),
                (KeyCode::Enter, LoginPhase::Idle | LoginPhase::Failed(_)) => self.st.login_act = Some(crate::app::LoginAct::Start),
                _ => {}
            }
            self.st.dirty = true;
            return;
        }
        // walking keys also need their release
        let dir = match k.code {
            KeyCode::Up => Some(0),
            KeyCode::Down => Some(1),
            KeyCode::Left => Some(2),
            KeyCode::Right => Some(3),
            KeyCode::Char(c) if self.focus == Focus::Map => match c.to_ascii_lowercase() {
                'w' => Some(0),
                's' => Some(1),
                'a' => Some(2),
                'd' => Some(3),
                _ => None,
            },
            _ => None,
        };
        if let Some(i) = dir.filter(|_| self.focus == Focus::Map) {
            return self.set_dir(i, down);
        }
        if k.code == KeyCode::Char(' ') && self.focus == Focus::Map && self.enhanced {
            let act = if down { crate::talk::TalkAct::Down } else { crate::talk::TalkAct::Up };
            if k.kind != KeyEventKind::Repeat {
                self.st.talk(act, now);
            }
            return;
        }
        if !down {
            return;
        }
        match self.focus {
            Focus::Chat => self.chat_key(k, now),
            Focus::People => self.people_key(k),
            Focus::Map => self.map_key(k, now),
        }
        self.sync_focus();
        self.st.dirty = true;
    }

    /// Zoom the map, and remember it for next time (`tuiZoom` in settings.json).
    fn set_zoom(&mut self, z: i32) {
        let z = z.clamp(-2, 3);
        if z != self.zoom {
            self.zoom = z;
            crate::calls::save_setting("tuiZoom", z.into());
        }
    }

    /// Remake the tags whenever the people list changes.
    pub fn update_tags(&mut self) {
        let mut people: Vec<(String, String)> = crate::directory::people(&self.st).into_iter().map(|p| (p.id, p.name)).collect();
        people.sort();
        if people != self.tags_of {
            self.tags = tags::assign(&people);
            self.tags_of = people;
        }
    }

    fn open_profile(&mut self) {
        if self.st.profile.is_none() {
            self.st.toggle_profile();
        }
        self.modal = Modal::Profile;
        self.st.dirty = true;
    }

    fn close_profile(&mut self) {
        self.st.profile_act(crate::profile::Act::Close);
        self.modal = Modal::None;
        self.st.dirty = true;
    }

    /// Keys in the profile dialog: up / down pick a line, left / right change the device on it, Enter runs it.
    fn profile_key(&mut self, k: KeyEvent) {
        use crate::profile::{Act, Line, LINES};
        let Some(p) = self.st.profile.as_mut() else { return self.close_profile() };
        let line = LINES[p.sel.min(LINES.len() - 1)];
        match (k.code, line) {
            (KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('u'), _) => return self.close_profile(),
            (KeyCode::Up | KeyCode::Char('k'), _) => p.sel = p.sel.saturating_sub(1),
            (KeyCode::Down | KeyCode::Char('j') | KeyCode::Tab, _) => p.sel = (p.sel + 1).min(LINES.len() - 1),
            (KeyCode::Enter | KeyCode::Char(' '), Line::Logout) => {
                self.close_profile();
                self.modal = Modal::Logout;
            }
            (KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('t'), Line::Device(crate::devices::Kind::Output)) => self.st.profile_act(Act::TestSound),
            (KeyCode::Left | KeyCode::Char('h') | KeyCode::Right | KeyCode::Char('l') | KeyCode::Enter | KeyCode::Char(' '), Line::Device(kind)) => {
                let id = p.cycle(kind, !matches!(k.code, KeyCode::Left | KeyCode::Char('h')));
                self.st.profile_act(Act::Pick(kind, id));
            }
            (KeyCode::Char('t'), _) => self.st.profile_act(Act::TestSound),
            _ => {}
        }
        self.st.dirty = true;
    }

    fn chat_key(&mut self, k: KeyEvent, now: f64) {
        let tabs = view::tabs(&self.st);
        match k.code {
            KeyCode::Enter => self.st.chat_key(Some("Enter"), None, now),
            KeyCode::Backspace => self.st.chat_key(Some("Backspace"), None, now),
            KeyCode::Esc => self.st.chat_key(Some("Escape"), None, now),
            KeyCode::Tab | KeyCode::BackTab => {
                let i = tabs.iter().position(|t| *t == self.st.chat.tab).unwrap_or(0);
                let next = if k.code == KeyCode::Tab { (i + 1) % tabs.len() } else { (i + tabs.len() - 1) % tabs.len() };
                self.st.select_tab(tabs[next].clone());
            }
            KeyCode::PageUp => self.scroll_chat(8.0),
            KeyCode::PageDown => self.scroll_chat(-8.0),
            KeyCode::Char(c) if !k.modifiers.contains(KeyModifiers::CONTROL) => self.st.chat_key(None, Some(c.encode_utf8(&mut [0; 4])), now),
            _ => {}
        }
    }

    fn scroll_chat(&mut self, by: f32) {
        let c = &self.st.chat;
        c.scroll.set((c.scroll.get() + by).clamp(0.0, c.max_scroll.get()));
        self.st.dirty = true;
    }

    fn map_key(&mut self, k: KeyEvent, now: f64) {
        let st = &mut self.st;
        match k.code {
            KeyCode::Tab => {
                self.focus = Focus::People;
                self.show_people = true;
            }
            KeyCode::Esc => {}
            KeyCode::Char(c) => match c {
                'q' => self.quit = true,
                'm' => st.toggle_mic(),
                'N' => st.toggle_noise_cancel(now),
                ' ' => st.talk(crate::talk::TalkAct::Toggle, now),
                'c' => st.toggle_cam(),
                'v' => st.toggle_share(),
                'l' => st.dock_press(crate::meeting::Hit::Ctl(Ctl::Leave), now),
                'b' => {
                    let a = !st.available;
                    st.set_available(a);
                }
                'r' => st.respawn(),
                'p' => self.show_people = !self.show_people,
                't' => {
                    if !st.chat.open {
                        st.toggle_chat();
                    }
                    st.chat.focus = true;
                }
                'T' => {
                    if st.chat.open {
                        st.toggle_chat();
                    }
                }
                '+' | '=' => self.set_zoom(self.zoom + 1),
                '-' | '_' => self.set_zoom(self.zoom - 1),
                'n' => self.next_picture(),
                'V' => self.open_window(),
                'G' if cfg!(feature = "gui") => {
                    self.to_gui = true;
                    self.quit = true;
                }
                'O' => self.modal = Modal::Logout,
                'u' => self.open_profile(),
                '?' => self.modal = Modal::Help,
                _ => {}
            },
            KeyCode::Enter => {
                if !st.chat.open {
                    st.toggle_chat();
                }
                st.chat.focus = true;
            }
            _ => {}
        }
    }

    fn mouse(&mut self, m: MouseEvent) {
        if self.modal == Modal::Profile && matches!(m.kind, MouseEventKind::Down(_)) && !self.areas.profile.is_some_and(|r| m.column >= r.x && m.column < r.x + r.width && m.row >= r.y && m.row < r.y + r.height) {
            return self.close_profile(); // a click outside the dialog closes it
        }
        if self.modal != Modal::None || self.st.login.is_some() {
            return;
        }
        let (col, row) = (m.column, m.row);
        let inside = |r: Option<Rect>| r.is_some_and(|r| col >= r.x && col < r.x + r.width && row >= r.y && row < r.y + r.height);
        let a = self.areas;
        let now = self.now();
        match m.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let gui_w = if cfg!(feature = "gui") { view::GUI_BUTTON_W } else { 0 };
                let right = a.header.x + a.header.width;
                if gui_w > 0 && row == a.header.y && col + gui_w >= right {
                    self.to_gui = true;
                    self.quit = true;
                } else if row == a.header.y && col + gui_w + view::PROFILE_BUTTON_W >= right {
                    self.open_profile();
                } else if inside(a.chat) {
                    self.focus = Focus::Chat;
                } else if inside(a.people) {
                    let r = a.people.unwrap_or_default();
                    let rows = self.rows();
                    let off = self.people_off(rows.len(), r.height.saturating_sub(2) as usize);
                    let i = off + row.saturating_sub(r.y + 1) as usize;
                    if row > r.y && i < rows.len() && !matches!(rows[i], Row::Section(_)) {
                        self.focus = Focus::People;
                        self.sel = i;
                        if let Row::Menu { buttons, .. } = &rows[i] {
                            // which button: they are drawn side by side after four blanks
                            let mut x = r.x + 1 + 4;
                            for (bi, b) in buttons.iter().enumerate() {
                                let w = match b { MenuBtn::Join { .. } => 6, MenuBtn::Wave => 6, MenuBtn::Message => 9 };
                                if col >= x && col < x + w {
                                    self.menu_sel = bi;
                                }
                                x += w;
                            }
                        }
                        self.activate();
                    }
                } else if inside(Some(a.map)) {
                    self.focus = Focus::Map;
                    let g = view::Geo::new(&self.st, Rect { x: a.map.x + 1, y: a.map.y + 1, width: a.map.width.saturating_sub(2), height: a.map.height.saturating_sub(2) }, self.zoom);
                    let (wx, wy) = g.to_world(col, row);
                    self.st.chat.focus = false;
                    self.st.click(wx, wy, m.modifiers.contains(KeyModifiers::SHIFT), now);
                }
            }
            MouseEventKind::ScrollUp | MouseEventKind::ScrollDown => {
                let up = m.kind == MouseEventKind::ScrollUp;
                if inside(a.chat) {
                    self.scroll_chat(if up { 3.0 } else { -3.0 });
                } else if inside(a.people) {
                    self.move_sel(!up);
                } else if inside(Some(a.map)) {
                    self.set_zoom(self.zoom + if up { 1 } else { -1 });
                }
            }
            _ => return,
        }
        self.sync_focus();
        self.st.dirty = true;
    }

    fn handle(&mut self, msg: Msg) {
        let now = self.now();
        match msg {
            Msg::Term(Term::Key(k)) => self.key(k),
            Msg::Term(Term::Mouse(m)) => self.mouse(m),
            Msg::Term(Term::Resize(..)) => {
                self.shown = None;
                self.st.dirty = true;
            }
            Msg::Term(Term::FocusGained) => self.st.set_focused(true, now),
            Msg::Term(Term::FocusLost) => {
                self.st.set_focused(false, now);
                self.st.keys = Default::default();
            }
            Msg::Term(Term::Paste(s)) if self.focus == Focus::Chat => self.st.chat_key(None, Some(&s.replace('\n', " ")), now),
            Msg::Term(_) => {}
            Msg::App(Event::Net(Net::LoggedOut)) => self.finish_logout(),
            Msg::App(Event::Net(e)) => self.st.apply(e, now),
            Msg::App(Event::Media(e)) => self.media(e, now),
            Msg::App(Event::Talk(a)) => self.st.talk(a, now),
            Msg::App(Event::Login(Ok(auth))) => self.sign_in(auth),
            Msg::App(Event::Login(Err(e))) => {
                self.st.login = Some(LoginPhase::after_error(e));
                self.st.dirty = true;
            }
        }
    }

    fn sign_in(&mut self, auth: Auth) {
        let (me, key) = (auth.uid.clone(), self.st.talk_key.clone());
        self.be.start(auth, false);
        self.st = State::new(&me, self.dry);
        self.st.talk_key = key;
        self.st.dirty = true;
    }

    fn finish_logout(&mut self) {
        self.be.finish_logout();
        let key = self.st.talk_key.clone();
        self.st = State::new("", self.dry);
        self.st.talk_key = key;
        self.st.login = Some(LoginPhase::Idle);
        self.focus = Focus::Map;
    }
}

/// Memory of this process in MB: the private bytes.
#[cfg(windows)]
fn proc_mb() -> f32 {
    use windows_sys::Win32::System::ProcessStatus::{K32GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS, PROCESS_MEMORY_COUNTERS_EX};
    use windows_sys::Win32::System::Threading::GetCurrentProcess;
    // SAFETY: the struct is plain data and its size is passed along.
    unsafe {
        let mut c: PROCESS_MEMORY_COUNTERS_EX = std::mem::zeroed();
        let size = std::mem::size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32;
        c.cb = size;
        if K32GetProcessMemoryInfo(GetCurrentProcess(), (&mut c as *mut PROCESS_MEMORY_COUNTERS_EX).cast::<PROCESS_MEMORY_COUNTERS>(), size) == 0 {
            return 0.0;
        }
        c.PrivateUsage as f32 / (1024.0 * 1024.0)
    }
}

/// Memory of this process in MB: the proportional share (what it really costs next to shared libraries), else resident.
#[cfg(not(windows))]
fn proc_mb() -> f32 {
    let kb = |path: &str, key: &str| -> Option<f32> {
        std::fs::read_to_string(path).ok()?.lines().find(|l| l.starts_with(key))?.split_whitespace().nth(1)?.parse().ok()
    };
    kb("/proc/self/smaps_rollup", "Pss:").or_else(|| kb("/proc/self/status", "VmRSS:")).unwrap_or(0.0) / 1024.0
}

/// Puts the terminal back however we leave.
struct Restore(bool);
impl Drop for Restore {
    fn drop(&mut self) {
        let mut out = std::io::stdout();
        if self.0 {
            let _ = execute!(out, PopKeyboardEnhancementFlags);
        }
        let _ = execute!(out, DisableFocusChange, DisableMouseCapture, ratatui::crossterm::event::DisableBracketedPaste);
        ratatui::restore();
    }
}

/// `soworker --term-check`: what this terminal can do, and so where video will go.
fn term_check() -> Result<(), String> {
    use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode};
    enable_raw_mode().map_err(|e| format!("needs a terminal: {e}"))?;
    let kitty = kitty::probe();
    let keys = ratatui::crossterm::terminal::supports_keyboard_enhancement().unwrap_or(false);
    let px = kitty::cell_px();
    let _ = disable_raw_mode();
    let env = |k: &str| std::env::var(k).unwrap_or_else(|_| "-".into());
    println!("TERM={} TERM_PROGRAM={} TMUX={}", env("TERM"), env("TERM_PROGRAM"), if std::env::var_os("TMUX").is_some() { "yes" } else { "no" });
    println!("kitty graphics (inline video): {}", if kitty { "yes" } else { "no" });
    println!("key release events (hold to walk / talk): {}", if keys { "yes" } else { "no" });
    println!("cell size in pixels: {}", px.map_or("unknown".into(), |(w, h)| format!("{w:.0}x{h:.0}")));
    let display = std::env::var_os("WAYLAND_DISPLAY").is_some() || std::env::var_os("DISPLAY").is_some();
    println!("display for the video window: {}", if display { "yes" } else { "no" });
    println!("=> video goes {}", if kitty { "inline" } else if display { "to a separate window" } else { "nowhere (no graphics, no display)" });
    Ok(())
}

pub fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--term-check") {
        return term_check();
    }
    crate::desktop::ensure();
    let opt = |n: &str| args.iter().position(|a| a == n).and_then(|i| args.get(i + 1)).cloned();
    let dry = args.iter().any(|a| a == "--dry-run");
    let path = opt("--session").unwrap_or_else(session::default_path);
    session::ensure_private_dir(&path);
    let exit_after: Option<f64> = opt("--exit-after").and_then(|s| s.parse().ok());
    if std::env::var_os("SOWORKER_LOG_STDERR").is_none() {
        crate::mode::log_to_file();
    }

    let (tx, rx) = channel::<Msg>();
    let emit: Emit = {
        let tx = tx.clone();
        Arc::new(move |e| drop(tx.send(Msg::App(e))))
    };

    let mut terminal = ratatui::try_init().map_err(|e| format!("the terminal client needs a terminal: {e}"))?;
    let _ = execute!(std::io::stdout(), EnableMouseCapture, EnableFocusChange, ratatui::crossterm::event::EnableBracketedPaste);
    // ask the terminal what it can do before the input thread starts reading stdin
    let want = opt("--video").unwrap_or_else(|| "auto".into());
    let inline = want != "window" && want != "off" && kitty::probe();
    let enhanced = ratatui::crossterm::terminal::supports_keyboard_enhancement().unwrap_or(false);
    if enhanced {
        let _ = execute!(std::io::stdout(), PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES));
    }
    let _restore = Restore(enhanced);
    let video = match (want.as_str(), inline) {
        ("off", _) => Video::Off,
        (_, true) => Video::Inline(kitty::Kitty::default()),
        _ => Video::Window(None),
    };

    let be = Backend::new(emit.clone(), dry, path.clone());
    let mut t = Tui::new(dry, video, be);
    t.enhanced = enhanced;
    t.st.talk_key = crate::talk::key_label();
    t.sync_wanted();
    {
        let tx = tx.clone();
        std::thread::spawn(move || {
            while let Ok(e) = ratatui::crossterm::event::read() {
                if tx.send(Msg::Term(e)).is_err() {
                    break;
                }
            }
        });
    }
    let e = emit.clone();
    crate::talk::listen(move |a| e(Event::Talk(a)));
    let e = emit.clone();
    crate::talk::portal(move |a| e(Event::Talk(a)), t.st.talk_key.clone());
    match Auth::load(&path) {
        Ok(auth) => t.sign_in(auth),
        Err(e) => t.st.login = Some(LoginPhase::after_load_error(std::path::Path::new(&path).exists(), e)),
    }
    let res = event_loop(&mut t, &mut terminal, rx, exit_after);
    if t.to_gui && res.is_ok() {
        drop(_restore); // the terminal back to normal first
        return crate::mode::relaunch(crate::mode::UiMode::Gui, args);
    }
    res
}

fn event_loop(t: &mut Tui, terminal: &mut ratatui::DefaultTerminal, rx: Receiver<Msg>, exit_after: Option<f64>) -> Result<(), String> {
    let (mut last_tick, mut last_draw) = (Instant::now(), Instant::now() - FRAME);
    let mut pending = true;
    loop {
        let timeout = if pending {
            FRAME.saturating_sub(last_draw.elapsed())
        } else if t.st.is_active() {
            FRAME
        } else if t.st.toast.is_some() || t.lapse.iter().any(|l| *l != f64::MAX) || t.be.logging_out().is_some() {
            Duration::from_millis(100)
        } else {
            Duration::from_secs(2)
        };
        match rx.recv_timeout(timeout) {
            Ok(m) => {
                t.handle(m);
                while let Ok(m) = rx.try_recv() {
                    t.handle(m);
                }
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => break,
        }
        if t.quit || exit_after.is_some_and(|x| t.now() > x) {
            break;
        }
        let (now, dt) = (t.now(), last_tick.elapsed().as_secs_f32().min(0.05));
        last_tick = Instant::now();
        // keys the terminal never releases lapse by themselves
        for i in 0..4 {
            if t.lapse[i] < now {
                t.set_dir(i, false);
            }
        }
        if t.be.logging_out().is_some_and(|d| d > Duration::from_millis(2500)) {
            t.finish_logout(); // the socket thread did not answer in time (offline?): leave anyway
        }
        if t.mem.1.elapsed() >= Duration::from_secs(2) {
            let mb = proc_mb();
            pending |= format!("{mb:.1}") != format!("{:.1}", t.mem.0);
            t.mem = (mb, Instant::now());
        }
        pending |= t.st.update(dt, now);
        let start = t.start;
        t.be.flush(&mut t.st, start);
        t.sync_focus();
        if pending && last_draw.elapsed() >= FRAME {
            pending = false;
            last_draw = Instant::now();
            let area = terminal.size().map(|s| Rect::new(0, 0, s.width, s.height)).unwrap_or_default();
            t.areas = view::areas(t, area);
            t.update_tags();
            terminal.draw(|f| view::draw(f, t)).map_err(|e| e.to_string())?;
            t.paint_video();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::crossterm::event::KeyEventState;

    fn press(c: KeyCode) -> KeyEvent {
        KeyEvent { code: c, modifiers: KeyModifiers::NONE, kind: KeyEventKind::Press, state: KeyEventState::NONE }
    }

    #[test]
    fn wasd_walk_and_lapse_without_release_events() {
        let mut t = Tui::for_test();
        t.key(press(KeyCode::Char('w')));
        assert!(t.st.keys.up);
        assert!(t.lapse[0] < f64::MAX, "no release events: the key lapses by itself");
        t.set_dir(0, false);
        assert!(!t.st.keys.up);
    }

    #[test]
    fn with_release_events_the_key_is_held_until_released() {
        let mut t = Tui::for_test();
        t.enhanced = true;
        t.key(press(KeyCode::Char('d')));
        assert!(t.st.keys.right);
        assert_eq!(t.lapse[3], f64::MAX);
        t.key(KeyEvent { kind: KeyEventKind::Release, ..press(KeyCode::Char('d')) });
        assert!(!t.st.keys.right);
    }

    #[test]
    fn typing_in_the_chat_does_not_walk() {
        let mut t = Tui::for_test();
        t.st.chat.open = true;
        t.st.chat.focus = true;
        t.sync_focus();
        t.key(press(KeyCode::Char('w')));
        assert!(!t.st.keys.up);
        assert_eq!(t.st.chat.input, "w");
        t.key(press(KeyCode::Esc));
        assert_eq!(t.focus, Focus::Map);
    }

    #[test]
    fn the_logout_dialog_asks_first() {
        let mut t = Tui::for_test();
        t.key(press(KeyCode::Char('O')));
        assert_eq!(t.modal, Modal::Logout);
        t.key(press(KeyCode::Char('n')));
        assert!(!t.st.logout_requested);
        t.key(press(KeyCode::Char('O')));
        t.key(press(KeyCode::Char('y')));
        assert!(t.st.logout_requested);
    }

    #[test]
    fn the_profile_dialog_leads_to_the_logout_question() {
        let mut t = Tui::for_test();
        t.st.profile = Some(crate::profile::Profile { devices: Default::default(), chosen: Default::default(), open: None, meter: None, sel: 1 });
        t.modal = Modal::Profile;
        t.key(press(KeyCode::Up));
        assert_eq!(t.st.profile.as_ref().unwrap().sel, 0, "the first line is Log out");
        t.key(press(KeyCode::Enter));
        assert_eq!(t.modal, Modal::Logout);
        assert!(t.st.profile.is_none(), "the profile closed (and its meter with it)");
        assert!(!t.st.logout_requested, "still asks first");
    }

    #[test]
    fn media_sync_is_harmless_without_a_call() {
        let mut t = Tui::for_test();
        t.media(MediaEvent::Left, 0.0);
        assert!(t.picked_key().is_none());
    }
}
