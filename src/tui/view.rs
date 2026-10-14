//! Drawing: header, people, map, call, chat, footer. Everything is derived from `Tui`/`State`; nothing here mutates them
//! (except the chat's scroll limit, which is a `Cell` measured while drawing).
use super::{Focus, Modal, Tui};
use crate::app::{LoginPhase, State};
use crate::chatstate::Tab;
use crate::directory::{self, MenuBtn, Row};
use crate::meeting;
use crate::world::{E, N, S, T, W};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};
use ratatui::Frame;
use std::collections::HashSet;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

const FLOOR: Color = Color::Rgb(43, 58, 82);
const MEETING: Color = Color::Rgb(58, 52, 96);
const WALL: Color = Color::Rgb(232, 195, 106);
const SOFT: Color = Color::Rgb(116, 128, 156);
const PROP: Color = Color::Rgb(96, 114, 138);
const BRAND: Color = Color::Rgb(90, 162, 243);
const OK: Color = Color::Rgb(111, 218, 117);
const WARN: Color = Color::Rgb(232, 195, 106);
const DANGER: Color = Color::Rgb(239, 102, 97);
const DIM: Color = Color::Rgb(117, 123, 131);
const MUTED: Color = Color::Rgb(165, 171, 181);

pub const PEOPLE_W: u16 = 30;
pub const CHAT_W: u16 = 42;

/// Where each panel sits; also what the mouse is matched against.
#[derive(Clone, Copy, Debug, Default)]
pub struct Areas {
    pub header: Rect,
    pub people: Option<Rect>,
    pub map: Rect,
    pub call: Option<Rect>,
    /// Where an inline video goes (inside `call`).
    pub video: Option<Rect>,
    pub chat: Option<Rect>,
    pub footer: Rect,
}

pub fn areas(t: &Tui, full: Rect) -> Areas {
    let header = Rect { height: 1.min(full.height), ..full };
    let footer = Rect { y: full.y + full.height.saturating_sub(1), height: 1.min(full.height), ..full };
    let body = Rect { y: full.y + 1, height: full.height.saturating_sub(2), ..full };
    let people_w = if t.show_people && body.width >= 70 { PEOPLE_W } else { 0 };
    let chat_w = if t.st.chat.open && body.width >= 80 { CHAT_W.min(body.width / 3) } else { 0 };
    let centre = Rect { x: body.x + people_w, width: body.width.saturating_sub(people_w + chat_w), ..body };
    let people = (people_w > 0).then_some(Rect { width: people_w, ..body });
    let chat = (chat_w > 0).then_some(Rect { x: body.x + body.width - chat_w, width: chat_w, ..body });
    let in_call = meeting::visible(&t.st) && centre.height >= 12;
    let call_h = if in_call { (centre.height / 3).clamp(8, 16).max(if t.video_inline() { centre.height / 2 } else { 0 }) } else { 0 };
    let map = Rect { height: centre.height - call_h, ..centre };
    let call = in_call.then_some(Rect { y: map.y + map.height, height: call_h, ..centre });
    let video = call.filter(|c| t.video_inline() && c.width >= 40).map(|c| {
        let w = c.width * 3 / 5;
        Rect { x: c.x + c.width - w + 1, y: c.y + 1, width: w.saturating_sub(2), height: c.height.saturating_sub(2) }
    });
    Areas { header, people, map, call, video, chat, footer }
}

pub fn draw(f: &mut Frame, t: &Tui) {
    let a = areas(t, f.area());
    if t.st.login.is_some() {
        return draw_login(f, t);
    }
    header(f, t, a.header);
    if let Some(r) = a.people {
        people(f, t, r);
    }
    map(f, t, a.map);
    if let Some(r) = a.call {
        call(f, t, r, a.video.is_some());
    }
    if let Some(r) = a.chat {
        chat(f, t, r);
    }
    footer(f, t, a.footer);
    match t.modal {
        Modal::Logout => dialog(f, " Log out? ", &["This forgets your saved sign-in on this computer.", "", "y: log out    n: cancel"]),
        Modal::Help => dialog(f, " Keys ", HELP),
        Modal::None => {}
    }
}

const HELP: &[&str] = &[
    "Walk        WASD / arrows, or click the map (Shift+click: teleport)",
    "Zoom        + / -  (or the mouse wheel over the map)",
    "People      Tab to focus, up/down, Enter opens Join / Wave / Message",
    "Call        m mic   c camera   v share screen   l leave   Space talk",
    "Video       n next picture   V open the video window",
    "Chat        t open and type, T close, Tab switches between chats, Esc leaves the input",
    "Other       b available/away   r respawn   p people   O log out   q quit",
    "",
    "any key closes this",
];

fn header(f: &mut Frame, t: &Tui, r: Rect) {
    let st = &t.st;
    let mut l = vec![Span::styled(" SoWorker ", Style::new().fg(Color::Black).bg(BRAND).add_modifier(Modifier::BOLD))];
    if !st.room.is_empty() {
        l.push(Span::raw(format!(" {} ", st.room)));
    }
    if st.dry {
        l.push(Span::styled(" dry run ", Style::new().fg(WARN)));
    }
    if !st.status.is_empty() {
        l.push(Span::styled(format!(" {} ", st.status), Style::new().fg(WARN)));
    }
    if st.call.joined.is_some() {
        let flag = |on: bool, label: &'static str, bad: bool| Span::styled(format!(" {label} "), if on { Style::new().fg(if bad { DANGER } else { OK }).add_modifier(Modifier::BOLD) } else { Style::new().fg(DIM) });
        l.push(flag(st.call.mic, if st.call.mic { "MIC ON" } else { "mic off" }, true));
        l.push(flag(st.call.cam, "cam", false));
        l.push(flag(st.call.share, "share", false));
    }
    if !st.available {
        l.push(Span::styled(" away ", Style::new().fg(WARN)));
    }
    let unread = st.unread_total();
    if unread > 0 {
        l.push(Span::styled(format!(" {unread} unread "), Style::new().fg(BRAND).add_modifier(Modifier::BOLD)));
    }
    f.render_widget(Paragraph::new(Line::from(l)), r);
    if let Some((msg, _)) = &st.toast {
        let w = (msg.width() as u16 + 2).min(r.width.saturating_sub(2));
        let tr = Rect { x: r.x + r.width - w, width: w, ..r };
        f.render_widget(Clear, tr);
        f.render_widget(Paragraph::new(Span::styled(format!(" {} ", fit(msg, w.saturating_sub(2) as usize)), Style::new().fg(Color::Black).bg(WARN))), tr);
    }
}

fn footer(f: &mut Frame, t: &Tui, r: Rect) {
    let hint = match t.focus {
        Focus::Chat => "type to chat · Enter send · Tab next chat · PgUp/PgDn scroll · Esc back",
        Focus::People => "↑↓ select · Enter open menu · ←→ choose · x dismiss wave · Esc back · ? keys",
        Focus::Map if t.st.call.joined.is_some() => "wasd walk · m mic · c cam · v share · l leave · t chat · p people · ? keys · q quit",
        Focus::Map => "wasd walk · Tab people · t chat · b away · +/- zoom · ? keys · q quit",
    };
    let mem = format!(" mem {:.1} MB ", t.mem.0);
    let mw = (mem.width() as u16).min(r.width);
    f.render_widget(Paragraph::new(Span::styled(format!(" {}", fit(hint, (r.width.saturating_sub(mw + 1)) as usize)), Style::new().fg(DIM))), r);
    f.render_widget(Paragraph::new(Span::styled(mem, Style::new().fg(DIM))), Rect { x: r.x + r.width - mw, width: mw, ..r });
}

pub fn fit(s: &str, w: usize) -> String {
    if s.width() <= w {
        return s.to_string();
    }
    let mut out = String::new();
    let mut used = 0;
    for c in s.chars() {
        let cw = c.width().unwrap_or(0);
        if used + cw + 1 > w {
            break;
        }
        out.push(c);
        used += cw;
    }
    out.push('…');
    out
}

fn border(focused: bool) -> Style {
    Style::new().fg(if focused { BRAND } else { Color::Rgb(50, 56, 67) })
}

// ---------------------------------------------------------------- people

/// The rows the people panel shows, and the index that is selected among them.
pub fn people_rows(t: &Tui) -> Vec<Row> {
    directory::rows(&t.st)
}

fn people(f: &mut Frame, t: &Tui, r: Rect) {
    let focused = t.focus == Focus::People;
    let block = Block::default().borders(Borders::ALL).border_style(border(focused)).title(" People ");
    let inner = block.inner(r);
    f.render_widget(block, r);
    let rows = people_rows(t);
    let h = inner.height as usize;
    let off = t.people_off(rows.len(), h);
    let w = inner.width as usize;
    let lines: Vec<Line> = rows.iter().enumerate().skip(off).take(h).map(|(i, row)| row_line(t, row, i == t.sel && focused, w)).collect();
    f.render_widget(Paragraph::new(lines), inner);
}

fn now_ms() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

fn row_line(t: &Tui, row: &Row, sel: bool, w: usize) -> Line<'static> {
    let hi = |s: Style| if sel { s.add_modifier(Modifier::REVERSED) } else { s };
    match row {
        Row::Section(s) => Line::from(Span::styled(fit(s, w), Style::new().fg(DIM).add_modifier(Modifier::BOLD))),
        Row::Group { title, count, .. } => Line::from(Span::styled(fit(&format!("▸ {title} ({count})"), w), hi(Style::new().fg(Color::Rgb(160, 150, 230)).add_modifier(Modifier::BOLD)))),
        Row::Person { p, indent } => {
            let (dot, col) = if !p.online { ("○", DIM) } else if p.away { ("●", WARN) } else { ("●", OK) };
            let pad = if *indent { "  " } else { "" };
            let mut name = p.name.clone();
            if p.me {
                name.push_str(" (you)");
            }
            let tail = if p.unread > 0 { format!(" ✉{}", p.unread) } else if !p.online { p.last_online.map(|ms| format!(" {}", directory::ago(now_ms(), Some(ms)))).unwrap_or_default() } else { String::new() };
            let name = fit(&name, w.saturating_sub(4 + pad.len() + tail.width()));
            Line::from(vec![
                Span::raw(pad.to_string()),
                Span::styled(dot, Style::new().fg(col)),
                Span::raw(" "),
                Span::styled(name, hi(Style::new().fg(if p.online { Color::White } else { MUTED }))),
                Span::styled(tail, Style::new().fg(if p.unread > 0 { BRAND } else { DIM })),
            ])
        }
        Row::Menu { buttons, .. } => {
            let mut v = vec![Span::raw("    ")];
            for (i, b) in buttons.iter().enumerate() {
                let (label, enabled) = match b {
                    MenuBtn::Join { enabled } => ("Join", *enabled),
                    MenuBtn::Wave => ("Wave", true),
                    MenuBtn::Message => ("Message", true),
                };
                let mut s = Style::new().fg(if enabled { Color::White } else { DIM });
                if sel && i == t.menu_sel {
                    s = s.bg(BRAND).fg(Color::Black);
                }
                v.push(Span::styled(format!(" {label} "), s));
            }
            Line::from(v)
        }
        Row::Wave { name, .. } => Line::from(vec![Span::styled("👋 ", Style::new()), Span::styled(fit(&format!("{name} waved"), w.saturating_sub(5)), hi(Style::new().fg(WARN))), Span::styled(" ⏎ go", Style::new().fg(DIM))]),
    }
}

// ---------------------------------------------------------------- map

/// How the map's cells line up with the world. Every tile is a block of `cw`x`ch` cells (zoom >= 1), or `k`x`k` tiles
/// share one block of 2x1 cells (zoomed out). The camera sits on my tile, so walls (tile edges) stay on cell borders.
#[derive(Clone, Copy, Debug)]
pub struct Geo {
    pub area: Rect,
    /// My tile, and the cell where my tile's block starts.
    ti: i32,
    tj: i32,
    ox: i32,
    oy: i32,
    cw: i32,
    ch: i32,
    k: i32,
}

impl Geo {
    pub fn new(st: &State, area: Rect, zoom: i32) -> Geo {
        let (mx, my) = st.me().map_or((3000.0, 3000.0), |m| (m.x, m.y));
        let (cw, ch, k) = if zoom >= 1 { (2 * zoom, zoom, 1) } else { (2, 1, 2 - zoom) };
        let (cx, cy) = (area.x as i32 + area.width as i32 / 2, area.y as i32 + area.height as i32 / 2);
        Geo { area, ti: (mx / T).floor() as i32, tj: (my / T).floor() as i32, ox: cx - cw / 2, oy: cy - ch / 2, cw, ch, k }
    }

    /// The tiles one cell stands for (x range, y range) and its place inside a tile block.
    fn cell(&self, col: i32, row: i32) -> ((i32, i32), (i32, i32), (i32, i32)) {
        let (dc, dr) = (col - self.ox, row - self.oy);
        if self.k == 1 {
            let (bx, by) = (dc.div_euclid(self.cw), dr.div_euclid(self.ch));
            ((self.ti + bx, self.ti + bx + 1), (self.tj + by, self.tj + by + 1), (dc.rem_euclid(self.cw), dr.rem_euclid(self.ch)))
        } else {
            let (bx, by) = (dc.div_euclid(2), dr.div_euclid(1));
            let (x0, y0) = (self.ti + bx * self.k - self.k / 2, self.tj + by * self.k - self.k / 2);
            ((x0, x0 + self.k), (y0, y0 + self.k), (dc.rem_euclid(2), 0))
        }
    }

    /// World position under a cell (centre of what it shows).
    pub fn to_world(&self, col: u16, row: u16) -> (f32, f32) {
        let ((x0, _), (y0, y1), (lc, lr)) = self.cell(col as i32, row as i32);
        if self.k == 1 {
            ((x0 as f32 + (lc as f32 + 0.5) / self.cw as f32) * T, (y0 as f32 + (lr as f32 + 0.5) / self.ch as f32) * T)
        } else {
            ((x0 as f32 + (lc as f32 + 0.5) * self.k as f32 / 2.0) * T, (y0 + y1) as f32 / 2.0 * T)
        }
    }

    /// The cell a world position falls in, if it is on screen.
    pub fn from_world(&self, x: f32, y: f32) -> Option<(u16, u16)> {
        let (tx, ty) = (x / T, y / T);
        let (i, j) = (tx.floor() as i32, ty.floor() as i32);
        let (col, row) = if self.k == 1 {
            (self.ox + (i - self.ti) * self.cw + ((tx - i as f32) * self.cw as f32) as i32, self.oy + (j - self.tj) * self.ch + ((ty - j as f32) * self.ch as f32) as i32)
        } else {
            let (bx, by) = ((i - self.ti + self.k / 2).div_euclid(self.k), (j - self.tj + self.k / 2).div_euclid(self.k));
            let frac = (tx - (self.ti + bx * self.k - self.k / 2) as f32) / self.k as f32;
            (self.ox + bx * 2 + (frac >= 0.5) as i32, self.oy + by)
        };
        let a = self.area;
        (col >= a.x as i32 && col < (a.x + a.width) as i32 && row >= a.y as i32 && row < (a.y + a.height) as i32).then_some((col as u16, row as u16))
    }
}

const QUADRANT: [char; 16] = [' ', '▘', '▝', '▀', '▖', '▌', '▞', '▛', '▗', '▚', '▐', '▜', '▄', '▙', '▟', '█'];

fn edges(mask: u8, lc: i32, lr: i32, cw: i32, ch: i32) -> usize {
    let mut q = 0;
    if mask & N != 0 && lr == 0 {
        q |= 0b0011;
    }
    if mask & S != 0 && lr == ch - 1 {
        q |= 0b1100;
    }
    if mask & W != 0 && lc == 0 {
        q |= 0b0101;
    }
    if mask & E != 0 && lc == cw - 1 {
        q |= 0b1010;
    }
    q
}

pub fn hue(id: &str) -> Color {
    let mut h: u32 = 0;
    for c in id.chars() {
        h = (h.wrapping_mul(31) + c as u32) % 360;
    }
    let (s, l) = (0.6f32, 0.6f32);
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = h as f32 / 60.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r, g, b) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    Color::Rgb(((r + m) * 255.0) as u8, ((g + m) * 255.0) as u8, ((b + m) * 255.0) as u8)
}

struct MapView<'a> {
    st: &'a State,
    geo: Geo,
}

impl Widget for MapView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (st, g) = (self.st, self.geo);
        let world = &st.world;
        let props: HashSet<(i32, i32)> = world.assets.iter().map(|&(x, y)| ((x / T).floor() as i32, (y / T).floor() as i32)).collect();
        let in_meeting = |i: i32, j: i32| world.zone_at((i as f32 + 0.5) * T, (j as f32 + 0.5) * T).is_some();
        for row in area.y..area.y + area.height {
            for col in area.x..area.x + area.width {
                let ((x0, x1), (y0, y1), (lc, lr)) = g.cell(col as i32, row as i32);
                let (mut floor, mut meeting, mut wall, mut soft, mut prop) = (false, false, 0u8, 0u8, false);
                for j in y0..y1 {
                    for i in x0..x1 {
                        if world.floors.contains(&(i, j)) {
                            floor = true;
                            meeting |= in_meeting(i, j);
                        }
                        wall |= world.walls.get(&(i, j)).copied().unwrap_or(0);
                        soft |= world.soft.get(&(i, j)).copied().unwrap_or(0);
                        prop |= props.contains(&(i, j));
                    }
                }
                let cell = &mut buf[(col, row)];
                if floor {
                    cell.set_bg(if meeting { MEETING } else { FLOOR });
                }
                let (cw, ch) = if g.k == 1 { (g.cw, g.ch) } else { (2, 1) };
                let (w, s) = (edges(wall, lc, lr, cw, ch), edges(soft & !wall, lc, lr, cw, ch));
                if w != 0 {
                    cell.set_char(QUADRANT[w]).set_fg(WALL);
                } else if s != 0 {
                    cell.set_char(QUADRANT[s]).set_fg(SOFT);
                } else if prop && lc == 0 && lr == ch / 2 {
                    cell.set_char('·').set_fg(PROP);
                }
            }
        }
        // meeting names, then people on top
        for z in world.zones.iter().filter(|z| z.meeting) {
            let Some(name) = z.name.as_deref() else { continue };
            if let Some((c, r)) = g.from_world(z.x, z.y) {
                let name = fit(name, 14);
                let c0 = (c as i32 - name.width() as i32 / 2).max(area.x as i32) as u16;
                put(buf, area, c0, r, &name, Style::new().fg(Color::Rgb(160, 150, 230)).bg(MEETING));
            }
        }
        let me_id = &st.me_id;
        let mut users: Vec<_> = st.users.values().collect();
        users.sort_by_key(|u| (u.id == *me_id, u.speaking)); // me last, so I am never covered
        for u in users {
            let Some((c, r)) = g.from_world(u.x, u.y) else { continue };
            let me = u.id == *me_id;
            let away = if me { !st.available } else { u.away() };
            let mut style = Style::new().fg(if me { Color::White } else { hue(&u.id) }).bg(Color::Rgb(30, 36, 48)).add_modifier(Modifier::BOLD);
            if away {
                style = style.add_modifier(Modifier::DIM);
            }
            if u.speaking {
                style = style.fg(OK).add_modifier(Modifier::UNDERLINED);
            }
            put(buf, area, c, r, if me { "@" } else { "●" }, style);
            if g.k == 1 && !me {
                put(buf, area, c + 2, r, &fit(&u.name, 12), Style::new().fg(if away { DIM } else { MUTED }).bg(Color::Rgb(30, 36, 48)));
            }
        }
    }
}

/// Write a string clipped to `area`.
fn put(buf: &mut Buffer, area: Rect, x: u16, y: u16, s: &str, style: Style) {
    if y < area.y || y >= area.y + area.height {
        return;
    }
    let mut x = x;
    for ch in s.chars() {
        let w = ch.width().unwrap_or(0) as u16;
        if w == 0 {
            continue;
        }
        if x < area.x || x + w > area.x + area.width {
            x += w;
            continue;
        }
        buf[(x, y)].set_char(ch).set_style(style);
        x += w;
    }
}

fn map(f: &mut Frame, t: &Tui, r: Rect) {
    let block = Block::default().borders(Borders::ALL).border_style(border(t.focus == Focus::Map));
    let inner = block.inner(r);
    f.render_widget(block, r);
    if t.st.world.floors.is_empty() {
        let msg = if t.st.status.is_empty() { "loading the office…" } else { "connecting…" };
        f.render_widget(Paragraph::new(Span::styled(msg, Style::new().fg(DIM))), inner);
        return;
    }
    f.render_widget(MapView { st: &t.st, geo: Geo::new(&t.st, inner, t.zoom) }, inner);
}

// ---------------------------------------------------------------- call

fn call(f: &mut Frame, t: &Tui, r: Rect, inline: bool) {
    let st = &t.st;
    let title = format!(" Call · {} ", meeting::status_label(st));
    let block = Block::default().borders(Borders::ALL).border_style(border(false)).title(title);
    let inner = block.inner(r);
    f.render_widget(block, r);
    let list_w = if inline { inner.width * 2 / 5 } else { inner.width };
    let picked = t.picked_key();
    let mut lines = Vec::new();
    for tile in meeting::tiles(st) {
        let shown = tile.video_key.is_some() && tile.video_key == picked;
        let mark = if shown { "▶" } else if tile.video_key.is_some() { "▣" } else { " " };
        let (icon, col) = if tile.speaking { ("♪", OK) } else if tile.muted && !tile.content { ("×", DIM) } else { (" ", MUTED) };
        let name = if tile.is_self { "You".to_string() } else { tile.name.clone() };
        let name = fit(&name, (list_w as usize).saturating_sub(6));
        lines.push(Line::from(vec![
            Span::styled(format!("{mark} "), Style::new().fg(BRAND)),
            Span::styled(format!("{icon} "), Style::new().fg(col)),
            Span::styled(name, Style::new().fg(if tile.content { WARN } else { Color::White })),
        ]));
    }
    if inline || t.window_capable() {
        let how = if inline { "n: next picture" } else if t.window_open() { "video is in its own window · n: next" } else { "V: open the video window" };
        lines.push(Line::from(Span::styled(how, Style::new().fg(DIM))));
    }
    f.render_widget(Paragraph::new(lines), Rect { width: list_w, ..inner });
}

// ---------------------------------------------------------------- chat

pub fn tabs(st: &State) -> Vec<Tab> {
    std::iter::once(Tab::Area).chain(st.chat.dms.iter().cloned().map(Tab::Dm)).collect()
}

fn tab_label(st: &State, tab: &Tab) -> String {
    match tab {
        Tab::Area => "Area".into(),
        Tab::Dm(p) => {
            let unread = st.unread_dm(p);
            let name = st.name_of(p, None);
            if unread > 0 { format!("{name} ({unread})") } else { name }
        }
    }
}

/// Greedy word wrap by display width; a word wider than the line is broken.
pub fn wrap(s: &str, w: usize) -> Vec<String> {
    let w = w.max(1);
    let mut out = Vec::new();
    for para in s.split('\n') {
        let mut line = String::new();
        let mut used = 0;
        for word in para.split(' ') {
            let ww = word.width();
            if used > 0 && used + 1 + ww > w {
                out.push(std::mem::take(&mut line));
                used = 0;
            }
            if used > 0 {
                line.push(' ');
                used += 1;
            }
            if ww > w {
                for c in word.chars() {
                    let cw = c.width().unwrap_or(0);
                    if used + cw > w {
                        out.push(std::mem::take(&mut line));
                        used = 0;
                    }
                    line.push(c);
                    used += cw;
                }
            } else {
                line.push_str(word);
                used += ww;
            }
        }
        out.push(line);
    }
    out
}

fn chat(f: &mut Frame, t: &Tui, r: Rect) {
    let st = &t.st;
    let focused = t.focus == Focus::Chat;
    let mut title = vec![Span::raw(" ")];
    for tab in tabs(st) {
        let on = tab == st.chat.tab;
        title.push(Span::styled(format!(" {} ", tab_label(st, &tab)), if on { Style::new().bg(BRAND).fg(Color::Black) } else { Style::new().fg(MUTED) }));
    }
    let block = Block::default().borders(Borders::ALL).border_style(border(focused)).title(Line::from(title));
    let inner = block.inner(r);
    f.render_widget(block, r);
    if inner.height < 3 {
        return;
    }
    let msgs_area = Rect { height: inner.height - 2, ..inner };
    let input_area = Rect { y: inner.y + inner.height - 1, height: 1, ..inner };
    let channel = st.channel_of(&st.chat.tab);
    let hint = match (&st.chat.tab, st.conversation()) {
        (Tab::Area, None) => Some("Walk into a meeting to chat with the area."),
        _ => None,
    };
    let mut lines: Vec<Line> = Vec::new();
    if let Some(h) = hint {
        lines.push(Line::from(Span::styled(h, Style::new().fg(DIM))));
    }
    let list: &[crate::chat::ChatMsg] = channel.as_ref().and_then(|c| st.chat.threads.get(c)).map_or(&[], |t| &t.msgs[..]);
    for m in list {
        let mine = m.user == st.me_id;
        let who = if mine { "you".to_string() } else { st.name_of(&m.user, m.name.as_deref()) };
        let color = if mine { BRAND } else { hue(&m.user) };
        let prefix = format!("{who}: ");
        let pw = prefix.width();
        let wrapped = wrap(&m.text, (inner.width as usize).saturating_sub(pw).max(8));
        for (i, l) in wrapped.into_iter().enumerate() {
            if i == 0 {
                lines.push(Line::from(vec![Span::styled(prefix.clone(), Style::new().fg(color).add_modifier(Modifier::BOLD)), Span::raw(l)]));
            } else {
                lines.push(Line::from(vec![Span::raw(" ".repeat(pw)), Span::raw(l)]));
            }
        }
    }
    let h = msgs_area.height as usize;
    let max = lines.len().saturating_sub(h);
    st.chat.max_scroll.set(max as f32);
    let up = (st.chat.scroll.get() as usize).min(max);
    st.chat.scroll.set(up as f32);
    let end = lines.len() - up;
    let start = end.saturating_sub(h);
    f.render_widget(Paragraph::new(lines[start..end].to_vec()), msgs_area);
    // input
    let prompt = if focused { "› " } else { "  " };
    let room = (input_area.width as usize).saturating_sub(3);
    let text: String = {
        let w = st.chat.input.width();
        if w <= room { st.chat.input.clone() } else { st.chat.input.chars().rev().scan(0, |u, c| { *u += c.width().unwrap_or(0); (*u <= room).then_some(c) }).collect::<Vec<_>>().into_iter().rev().collect() }
    };
    let shown = if text.is_empty() && !focused { "press t, then type…".to_string() } else { text.clone() };
    let style = if text.is_empty() && !focused { Style::new().fg(DIM) } else { Style::new() };
    f.render_widget(Paragraph::new(Line::from(vec![Span::styled(prompt, Style::new().fg(BRAND)), Span::styled(shown, style)])), input_area);
    if focused {
        f.set_cursor_position((input_area.x + 2 + text.width() as u16, input_area.y));
    }
}

// ---------------------------------------------------------------- login, dialogs

fn centred(area: Rect, w: u16, h: u16) -> Rect {
    let (w, h) = (w.min(area.width), h.min(area.height));
    Rect { x: area.x + (area.width - w) / 2, y: area.y + (area.height - h) / 2, width: w, height: h }
}

fn dialog(f: &mut Frame, title: &str, lines: &[&str]) {
    let w = lines.iter().map(|l| l.width()).max().unwrap_or(10) as u16 + 4;
    let r = centred(f.area(), w, lines.len() as u16 + 2);
    f.render_widget(Clear, r);
    let block = Block::default().borders(Borders::ALL).border_style(Style::new().fg(BRAND)).title(title.to_string());
    let inner = block.inner(r);
    f.render_widget(block, r);
    f.render_widget(Paragraph::new(lines.iter().map(|l| Line::from(format!(" {l}"))).collect::<Vec<_>>()), inner);
}

fn draw_login(f: &mut Frame, t: &Tui) {
    let (head, body, key): (&str, Vec<String>, &str) = match t.st.login.as_ref() {
        Some(LoginPhase::Waiting) => ("Waiting for you…", vec!["Finish signing in in the Chrome window that just opened.".into(), "It closes by itself when you are done.".into()], "Esc cancel · q quit"),
        Some(LoginPhase::Failed(e)) => ("Sign-in failed", wrap(e, 56), "Enter try again · q quit"),
        _ => ("Sign in to SoWork", vec!["SoWorker needs you to sign in once.".into(), "Enter opens Chrome on SoWork's login page;".into(), "the token is saved in ~/.config/soworker/.".into()], "Enter sign in · q quit"),
    };
    let mut lines: Vec<String> = body;
    lines.push(String::new());
    lines.push(key.into());
    let refs: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    dialog(f, &format!(" {head} "), &refs);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::{Net, User};
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn tui() -> Tui {
        let mut t = Tui::for_test();
        let user = |id: &str, name: &str, zone: Option<&str>, x: f32, y: f32| User { id: id.into(), name: name.into(), zone: zone.map(String::from), x, y, status: "online".into(), mode: "cluster".into(), ..User::default() };
        t.st = State::new("me", true);
        t.st.apply(Net::Room { name: "Office".into(), spawn: None, users: vec![user("me", "Me", None, 3000.0, 3000.0), user("a", "Ana", Some("Z1"), 3400.0, 3000.0), user("b", "Bo", Some("Z1"), 3000.0, 3400.0)] }, 0.0);
        t
    }

    fn render(t: &Tui, w: u16, h: u16) -> String {
        let mut term = Terminal::new(TestBackend::new(w, h)).unwrap();
        term.draw(|f| draw(f, t)).unwrap();
        let buf = term.backend().buffer().clone();
        (0..h).map(|y| (0..w).map(|x| buf[(x, y)].symbol().to_string()).collect::<String>()).collect::<Vec<_>>().join("\n")
    }

    #[test]
    fn the_screen_shows_the_header_people_and_hints() {
        let s = render(&tui(), 100, 30);
        assert!(s.contains("SoWorker"), "{s}");
        assert!(s.contains("Office"));
        assert!(s.contains("People"));
        assert!(s.contains("Ana"), "people list names");
        assert!(s.contains("wasd walk"));
        assert!(s.contains(" MB"), "memory shows in the footer");
    }

    #[test]
    fn me_and_the_others_are_on_the_map() {
        let mut t = tui();
        t.st.world.floors.insert((28, 28));
        let s = render(&t, 100, 30);
        assert!(s.contains('@'), "{s}");
    }

    #[test]
    fn narrow_terminals_drop_the_side_panels() {
        let t = tui();
        let a = areas(&t, Rect::new(0, 0, 60, 20));
        assert!(a.people.is_none());
        assert_eq!(a.map.width, 60);
    }

    #[test]
    fn wrapping_breaks_long_words() {
        assert_eq!(wrap("aaaaaaaaaa", 4), vec!["aaaa", "aaaa", "aa"]);
        assert_eq!(wrap("hello big world", 9), vec!["hello big", "world"]);
        assert_eq!(wrap("a\nb", 9), vec!["a", "b"]);
    }

    #[test]
    fn world_and_cells_round_trip() {
        let t = tui();
        for zoom in [-2, 0, 1, 3] {
            let g = Geo::new(&t.st, Rect::new(0, 0, 80, 30), zoom);
            let (wx, wy) = g.to_world(41, 14);
            let back = g.from_world(wx, wy).expect("on screen");
            assert_eq!(back, (41, 14), "zoom {zoom}");
        }
    }

    #[test]
    fn the_login_screen_explains_itself() {
        let mut t = tui();
        t.st.login = Some(LoginPhase::Idle);
        let s = render(&t, 80, 20);
        assert!(s.contains("Sign in to SoWork"));
        assert!(s.contains("Enter sign in"));
    }

    /// `cargo test demo_screen -- --nocapture --ignored` prints a populated screen, to look at the layout.
    #[test]
    #[ignore]
    fn demo_screen() {
        use crate::world::{Zone, E, N, S, W};
        let mut t = tui();
        for i in 20..40 {
            for j in 22..36 {
                t.st.world.floors.insert((i, j));
            }
        }
        for i in 20..40 {
            *t.st.world.walls.entry((i, 22)).or_default() |= N;
            *t.st.world.walls.entry((i, 35)).or_default() |= S;
        }
        for j in 22..36 {
            *t.st.world.walls.entry((20, j)).or_default() |= W;
            *t.st.world.walls.entry((39, j)).or_default() |= E;
        }
        for j in 22..28 {
            *t.st.world.walls.entry((30, j)).or_default() |= E;
        }
        t.st.world.assets = vec![(25.5 * T, 30.5 * T), (26.5 * T, 30.5 * T)];
        t.st.world.zones.push(Zone { id: "Z1".into(), x: 34.0 * T, y: 31.0 * T, sx: 2.0 * T, sy: 2.0 * T, meeting: true, name: Some("Standup".into()) });
        t.st.chat.open = true;
        for zoom in [1, 2, -1] {
            t.zoom = zoom;
            println!("zoom {zoom}\n{}\n", render(&t, 120, 32));
        }
    }
}
