//! Application state and per-frame logic: input, walking, click-to-move, zones, presence.
//! Pure logic (no window, no sockets) so it runs the same in the window, in tests and in headless replays.
use crate::avatars;
use crate::calls::{MediaCmd, MediaEvent};
use crate::meeting_ui::{Ctl, Dock, Hit, TileBtn};
use crate::proto::{ClusterInfo, Cmd, Net, User};
use crate::world::{tile, World, T};
use std::collections::HashMap;
use tiny_skia::Pixmap;

pub const SPEED: f32 = 800.0; // world units / second, same as the official client
const TICK: f64 = 0.205; // official client sends `move` about every 205 ms while walking

#[derive(Clone, Debug, Default)]
pub struct Avatar {
    pub id: String,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub tx: f32,
    pub ty: f32,
    pub state: String,
    pub speaking: bool,
    pub photo: Option<String>,
    pub portrait: Option<String>,
}

pub struct Tile {
    pub key: String,
    pub name: String,
    pub img: tiny_skia::Pixmap,
}

/// Call state shown in the UI. `want` is the LiveKit room the server asked us to be in.
#[derive(Default)]
pub struct CallUi {
    pub want: Option<String>,
    pub joined: Option<String>,
    pub mic: bool,
    pub cam: bool,
    pub share: bool,
    pub peer_muted: std::collections::HashSet<String>,
    /// A call we left on purpose: ignore further tokens for this room until the server moves us elsewhere.
    pub ignore_room: Option<String>,
    pub tiles: Vec<Tile>,
    pub speakers: std::collections::HashSet<String>,
}

#[derive(Default, Clone, Copy)]
pub struct Keys {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoginPhase {
    Idle,
    Waiting,
    Failed(String),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LoginAct {
    Start,
    Cancel,
}

pub struct State {
    pub world: World,
    pub me_id: String,
    pub room: String,
    pub spawn: Option<(f32, f32)>,
    pub users: HashMap<String, Avatar>,
    pub cluster: Option<ClusterInfo>,
    pub scale: f32,
    pub keys: Keys,
    pub path: Option<Vec<(f32, f32)>>,
    pub path_goal: (f32, f32),
    pub available: bool,
    pub dry: bool,
    pub show_debug: bool,
    pub status: String,
    pub toast: Option<(String, f64)>,
    pub outbox: Vec<Cmd>,
    pub media_out: Vec<MediaCmd>,
    pub call: CallUi,
    pub dock: Dock,
    /// Circle-masked avatar pictures by user id, and what we already asked the loader for.
    pub pics: HashMap<String, Pixmap>,
    pub avatar_req: Vec<(String, String)>,
    avatar_asked: HashMap<String, String>,
    left_zone: Option<String>,
    pub dirty: bool,
    /// Logout flow: the confirmation dialog, then the request the window acts on.
    pub confirm_logout: bool,
    pub logout_requested: bool,
    /// Some = show the sign-in screen instead of the world; `login_act` is what the user just asked for.
    pub login: Option<LoginPhase>,
    pub login_act: Option<LoginAct>,
    /// Diagnostics: how many zone changes we sent and cluster events we got.
    pub zone_sent: u32,
    pub clusters_seen: u32,
    /// Display scale factor (HiDPI): multiplies zoom and UI text sizes.
    pub ui: f32,
    moving: bool,
    last_send: f64,
    stuck: f32,
    cur_zone: Option<Option<String>>, // None = not reported yet
    pub cur_zone_id: Option<String>,
}

impl State {
    pub fn new(me_id: &str, dry: bool) -> Self {
        State {
            world: World::default(),
            me_id: me_id.into(),
            room: String::new(),
            spawn: None,
            users: HashMap::new(),
            cluster: None,
            scale: 0.35,
            keys: Keys::default(),
            path: None,
            path_goal: (0.0, 0.0),
            available: true,
            dry,
            show_debug: true,
            status: "connecting…".into(),
            toast: None,
            outbox: vec![],
            media_out: vec![],
            call: CallUi::default(),
            dock: Dock::default(),
            pics: HashMap::new(),
            avatar_req: vec![],
            avatar_asked: HashMap::new(),
            left_zone: None,
            dirty: true,
            confirm_logout: false,
            logout_requested: false,
            login: None,
            login_act: None,
            zone_sent: 0,
            clusters_seen: 0,
            ui: 1.0,
            moving: false,
            last_send: 0.0,
            stuck: 0.0,
            cur_zone: None,
            cur_zone_id: None,
        }
    }

    pub fn me(&self) -> Option<&Avatar> {
        self.users.get(&self.me_id)
    }

    fn avatar(u: User) -> Avatar {
        Avatar { tx: u.x, ty: u.y, id: u.id, name: u.name, x: u.x, y: u.y, state: u.state, speaking: false, photo: u.photo, portrait: u.portrait }
    }

    /// Ask the loader for a user's picture when we know where it is and have not asked for that URL yet.
    fn want_avatar(&mut self, id: &str) {
        let Some(u) = self.users.get(id) else { return };
        let Some(url) = avatars::source_url(u.photo.as_deref(), u.portrait.as_deref()) else { return };
        if self.avatar_asked.get(id) != Some(&url) {
            self.avatar_asked.insert(id.to_string(), url.clone());
            self.avatar_req.push((id.to_string(), url));
        }
    }

    pub fn say(&mut self, msg: impl Into<String>, now: f64) {
        self.toast = Some((msg.into(), now + 4.0));
        self.dirty = true;
    }

    pub fn apply(&mut self, ev: Net, now: f64) {
        self.dirty = true;
        match ev {
            Net::Connected => self.status.clear(),
            Net::Disconnected => self.status = "reconnecting…".into(),
            Net::Error(e) => {
                eprintln!("[{now:6.1}] net: {e}");
                self.say(e, now)
            }
            Net::Object(o) => self.world.apply(o),
            Net::Room { name, spawn, users } => {
                self.room = name;
                self.spawn = spawn;
                let mine = self.users.get(&self.me_id).cloned();
                self.users = users.into_iter().map(|u| (u.id.clone(), Self::avatar(u))).collect();
                if let Some(m) = mine.filter(|_| !self.users.contains_key(&self.me_id)) {
                    self.users.insert(m.id.clone(), m);
                }
                self.cur_zone = None;
                self.world.rebuild_if_dirty();
                self.check_spawn();
                for id in self.users.keys().cloned().collect::<Vec<_>>() {
                    self.want_avatar(&id);
                }
            }
            Net::Move(id, x, y) if id != self.me_id => {
                if let Some(u) = self.users.get_mut(&id) {
                    u.tx = x;
                    u.ty = y;
                    if (u.tx - u.x).hypot(u.ty - u.y) > 1000.0 {
                        (u.x, u.y) = (x, y);
                    }
                }
            }
            Net::State(id, s) if id != self.me_id => {
                if let Some(u) = self.users.get_mut(&id) {
                    u.state = s;
                }
            }
            Net::Join(u) if u.id != self.me_id => {
                let id = u.id.clone();
                self.users.insert(id.clone(), Self::avatar(u));
                self.want_avatar(&id);
            }
            Net::UserInfo { id, name, photo, portrait } => {
                if let Some(u) = self.users.get_mut(&id) {
                    if let Some(n) = name { u.name = n; }
                    if let Some(p) = photo { u.photo = p; }
                    if portrait.is_some() { u.portrait = portrait; }
                }
                self.want_avatar(&id);
            }
            Net::Avatar { id, size, rgba } => {
                if let Some(p) = tiny_skia::IntSize::from_wh(size, size).and_then(|s| Pixmap::from_vec(rgba, s)) {
                    self.pics.insert(id, p);
                }
            }
            Net::Leave(id) if id != self.me_id => {
                self.users.remove(&id);
            }
            Net::Speaking(id, on) if id != self.me_id => {
                if let Some(u) = self.users.get_mut(&id) {
                    u.speaking = on;
                }
            }
            Net::Cluster(c) => {
                self.clusters_seen += 1;
                eprintln!("[{now:6.1}] gaia cluster event: {}", match &c {
                    None => "none (left / alone)".to_string(),
                    Some(c) => format!("{} user(s), token: {}", c.users.len(), if c.token.is_some() { "livekit" } else { c.unsupported.as_deref().unwrap_or("missing") }),
                });
                self.cluster = c;
                self.sync_call();
            }
            _ => {}
        }
    }

    /// If the saved position is off the map (or inside a wall), move to the room's spawn point.
    fn check_spawn(&mut self) {
        let Some(me) = self.users.get(&self.me_id) else { return };
        if self.world.blocked(me.x, me.y) {
            self.respawn();
        }
    }

    pub fn respawn(&mut self) {
        if let (Some((sx, sy)), Some(me)) = (self.spawn, self.users.get_mut(&self.me_id)) {
            (me.x, me.y, me.tx, me.ty) = (sx, sy, sx, sy);
            self.path = None;
            self.outbox.push(Cmd::Move(sx, sy));
            self.dirty = true;
        }
    }

    pub fn teleport(&mut self, mut gx: f32, mut gy: f32) {
        if !self.world.floors.contains(&(tile(gx), tile(gy))) {
            return;
        }
        if self.world.blocked(gx, gy) {
            gx = (tile(gx) as f32 + 0.5) * T;
            gy = (tile(gy) as f32 + 0.5) * T;
        }
        if let Some(me) = self.users.get_mut(&self.me_id) {
            (me.x, me.y, me.tx, me.ty) = (gx, gy, gx, gy);
        }
        self.path = None;
        self.outbox.push(Cmd::Move(gx, gy));
        self.dirty = true;
    }

    /// Screen position -> world position for a window of the given size, camera centred on me.
    pub fn to_world(&self, px: f32, py: f32, w: f32, h: f32) -> Option<(f32, f32)> {
        let me = self.me()?;
        let s = self.scale * self.ui;
        Some((me.x + (px - w / 2.0) / s, me.y + (py - h / 2.0) / s))
    }

    pub fn click(&mut self, wx: f32, wy: f32, shift: bool, now: f64) {
        if shift {
            return self.teleport(wx, wy);
        }
        let Some(me) = self.me() else { return };
        let from = (me.x, me.y);
        match self.world.find_path(from, (wx, wy)) {
            Some((p, goal)) => {
                self.path = Some(p);
                self.path_goal = goal;
                self.stuck = 0.0;
            }
            None if self.world.floors.contains(&(tile(wx), tile(wy))) => {
                self.say("no walking route: Shift+click to teleport", now)
            }
            None => {}
        }
        self.dirty = true;
    }

    /// Join or leave the LiveKit room that matches the latest `cluster` event.
    fn sync_call(&mut self) {
        let want = self.cluster.as_ref().and_then(|c| c.token.clone());
        if want.as_ref().map(|t| &t.room_id) != self.call.ignore_room.as_ref() && want.is_some() {
            self.call.ignore_room = None; // the server moved us to a different conversation
        }
        let want = want.filter(|t| Some(&t.room_id) != self.call.ignore_room.as_ref());
        match (want, &self.call.want) {
            (Some(t), cur) if cur.as_deref() != Some(t.room_id.as_str()) => {
                self.call.want = Some(t.room_id.clone());
                self.media_out.push(MediaCmd::Join { url: format!("wss://{}", t.api_host), token: t.token, room_id: t.room_id });
            }
            (None, Some(_)) => {
                self.call.want = None;
                self.media_out.push(MediaCmd::Leave);
            }
            _ => {}
        }
    }

    pub fn toggle_mic(&mut self) {
        if self.call.joined.is_some() {
            self.media_out.push(MediaCmd::Mic(!self.call.mic));
        }
    }

    /// A click on the meeting dock. Header drags are handled by the window.
    pub fn dock_press(&mut self, hit: Hit, now: f64) {
        self.dirty = true;
        match hit {
            Hit::Ctl(Ctl::FullToggle) => self.dock.full = !self.dock.full,
            Hit::Ctl(Ctl::Chat) => self.dock.chat_open = !self.dock.chat_open,
            Hit::Ctl(Ctl::Mic) => self.toggle_mic(),
            Hit::Ctl(Ctl::Cam) => self.toggle_cam(),
            Hit::Ctl(Ctl::Share) => self.toggle_share(),
            Hit::Ctl(Ctl::Lock) => self.say("Locking a meeting is not something SoWork supports", now),
            Hit::Ctl(Ctl::Send) => self.say("Chat is not available in this client yet", now),
            Hit::Ctl(Ctl::Leave) => {
                // leaving = tell the server we are in no zone, drop the call, and do not rejoin until we walk out and back
                self.call.ignore_room = self.call.want.clone().or_else(|| self.call.joined.clone());
                self.call.want = None;
                self.media_out.push(MediaCmd::Leave);
                if self.cur_zone_id.is_some() {
                    self.left_zone = self.cur_zone_id.clone();
                    self.outbox.push(Cmd::Zone(None));
                }
            }
            Hit::TileBtn(id, TileBtn::Pin) => {
                self.dock.pinned = if self.dock.pinned.as_deref() == Some(id.as_str()) { None } else { Some(id) };
            }
            // stubs, as requested: the buttons exist (as in omoffice) but acting on other people is not implemented
            Hit::TileBtn(_, TileBtn::Mute) => self.say("Muting another person is not implemented yet", now),
            Hit::TileBtn(_, TileBtn::CameraOff) => self.say("Turning off another person's camera is not implemented yet", now),
            _ => {}
        }
    }

    /// The logout dialog's buttons.
    pub fn logout_choice(&mut self, confirm: bool) {
        self.confirm_logout = false;
        if confirm {
            self.logout_requested = true;
            self.status = "logging out…".into();
        }
        self.dirty = true;
    }

    pub fn toggle_cam(&mut self) {
        if self.call.joined.is_some() {
            self.media_out.push(MediaCmd::Camera(!self.call.cam));
        }
    }

    pub fn toggle_share(&mut self) {
        if self.call.joined.is_some() {
            self.media_out.push(MediaCmd::Share(!self.call.share));
        }
    }

    /// An on-screen button was pressed.
    pub fn press(&mut self, act: crate::render::Act) {
        use crate::render::Act;
        match act {
            Act::Away => {
                let a = !self.available;
                self.set_available(a);
            }
            Act::Logout => self.confirm_logout = true,
        }
        self.dirty = true;
    }

    pub fn apply_media(&mut self, ev: MediaEvent, now: f64) {
        self.dirty = true;
        match ev {
            MediaEvent::Joined(r) => {
                eprintln!("[{now:6.1}] livekit joined {r}");
                self.call.joined = Some(r);
            }
            MediaEvent::Left => {
                eprintln!("[{now:6.1}] livekit left");
                self.call.joined = None;
                self.call.mic = false;
                self.call.cam = false;
                self.call.share = false;
                self.call.tiles.clear();
                self.call.speakers.clear();
                self.call.peer_muted.clear();
                self.dock.pinned = None;
                self.dock.last_auto = None;
                for u in self.users.values_mut() {
                    u.speaking = false;
                }
            }
            MediaEvent::Error(e) => {
                eprintln!("[{now:6.1}] media error: {e}");
                self.say(e, now)
            }
            MediaEvent::MicState(on) => self.call.mic = on,
            MediaEvent::CameraState(on) => {
                if on != self.call.cam {
                    self.outbox.push(Cmd::Camera(on)); // gaia shows the camera indicator to others
                }
                self.call.cam = on;
            }
            MediaEvent::ShareState(on) => self.call.share = on,
            MediaEvent::PeerMuted(id, m) => {
                if m { self.call.peer_muted.insert(id); } else { self.call.peer_muted.remove(&id); }
            }
            MediaEvent::Speakers(ids) => {
                let was = self.users.get(&self.me_id).is_some_and(|m| m.speaking);
                self.call.speakers = ids.into_iter().collect();
                for u in self.users.values_mut() {
                    u.speaking = self.call.speakers.contains(&u.id);
                }
                let is = self.call.speakers.contains(&self.me_id);
                if is != was {
                    self.outbox.push(Cmd::Speech(is)); // tells gaia, which draws our ring for everyone else
                }
            }
            MediaEvent::Video { key, name, w, h, rgba } => {
                let Some(img) = tiny_skia::IntSize::from_wh(w, h).and_then(|s| tiny_skia::Pixmap::from_vec(rgba, s)) else { return };
                match self.call.tiles.iter_mut().find(|t| t.key == key) {
                    Some(t) => (t.name, t.img) = (name, img),
                    None => {
                        // a screen share starting is pinned once by default (omoffice MeetingPanel)
                        if key.ends_with(":screen") && self.dock.last_auto.as_deref() != Some(key.as_str()) {
                            self.dock.pinned = Some(format!("content:{key}"));
                            self.dock.last_auto = Some(key.clone());
                        }
                        self.call.tiles.push(Tile { key, name, img })
                    }
                }
            }
            MediaEvent::VideoGone(key) => {
                self.call.tiles.retain(|t| t.key != key);
                if self.dock.last_auto.as_deref() == Some(key.as_str()) {
                    self.dock.last_auto = None;
                }
                if self.dock.pinned.as_deref() == Some(format!("content:{key}").as_str()) {
                    self.dock.pinned = None;
                }
            }
        }
    }

    pub fn zoom(&mut self, up: bool) {
        self.scale = (self.scale * if up { 1.1 } else { 1.0 / 1.1 }).clamp(0.08, 1.5);
        self.dirty = true;
    }

    pub fn set_available(&mut self, a: bool) {
        self.available = a;
        self.outbox.push(Cmd::Available(a));
        self.dirty = true;
    }

    /// True while something animates (walking, a path, avatars gliding) so the window keeps ticking.
    pub fn is_active(&self) -> bool {
        let k = self.keys;
        self.moving
            || self.path.is_some()
            || k.up || k.down || k.left || k.right
            || self.users.values().any(|u| u.id != self.me_id && (u.tx - u.x).hypot(u.ty - u.y) > 0.5)
    }

    /// Advance the simulation. Returns true if the screen needs a redraw (movement or events).
    pub fn update(&mut self, dt: f32, now: f64) -> bool {
        if self.world.rebuild_if_dirty() {
            self.check_spawn();
            self.dirty = true;
        }
        if self.toast.as_ref().is_some_and(|(_, until)| now > *until) {
            self.toast = None;
            self.dirty = true;
        }
        let mut active = self.moving || self.path.is_some();
        let k = self.keys;
        let (mut dx, mut dy) = ((k.right as i32 - k.left as i32) as f32, (k.down as i32 - k.up as i32) as f32);
        if (dx != 0.0 || dy != 0.0) && self.path.take().is_some() {
            self.dirty = true;
        }
        let me_id = self.me_id.clone();
        if let Some(me) = self.users.get(&me_id).cloned() {
            let mut p = (me.x, me.y);
            if dx == 0.0 && dy == 0.0 {
                if let Some(path) = &mut self.path {
                    while path.first().is_some_and(|w| (w.0 - p.0).hypot(w.1 - p.1) < 12.0) {
                        path.remove(0);
                    }
                    match path.first() {
                        Some(w) => (dx, dy) = (w.0 - p.0, w.1 - p.1),
                        None => self.path = None,
                    }
                }
            }
            let want = dx != 0.0 || dy != 0.0;
            if want {
                let l = dx.hypot(dy);
                let before = p;
                p = self.world.step(p, dx / l * SPEED * dt, dy / l * SPEED * dt);
                if self.path.is_some() {
                    self.stuck = if (p.0 - before.0).hypot(p.1 - before.1) < SPEED * dt * 0.2 { self.stuck + dt } else { 0.0 };
                    if self.stuck > 0.4 {
                        self.path = None;
                    }
                }
                active = true;
            }
            if want != self.moving {
                self.moving = want;
                self.outbox.push(Cmd::Move(p.0, p.1));
                self.outbox.push(Cmd::State(if want { "walk" } else { "idle" }));
                self.last_send = now;
            } else if self.moving && now - self.last_send >= TICK {
                self.outbox.push(Cmd::Move(p.0, p.1));
                self.last_send = now;
            }
            if let Some(m) = self.users.get_mut(&me_id) {
                (m.x, m.y, m.tx, m.ty) = (p.0, p.1, p.0, p.1);
                m.state = if self.moving { "walk" } else { "idle" }.into();
            }
            // meeting zone: tell the server when we enter or leave one
            let real = self.world.zone_at(p.0, p.1).map(|z| z.id.clone());
            if real.is_none() || real != self.left_zone {
                self.left_zone = None; // walked out of the zone we left: a new visit may start a call again
            }
            let z = if real.is_some() && real == self.left_zone { None } else { real };
            if self.cur_zone.as_ref() != Some(&z) {
                let first = self.cur_zone.is_none();
                self.cur_zone = Some(z.clone());
                self.cur_zone_id = z.clone();
                if !first || z.is_some() {
                    self.zone_sent += 1;
                    eprintln!("[{now:6.1}] zone -> {:?}{}", z, if self.dry { "  (dry run: NOT sent)" } else { "" });
                    self.outbox.push(Cmd::Zone(z));
                }
                self.dirty = true;
            }
        }
        for u in self.users.values_mut().filter(|u| u.id != me_id) {
            let d = (u.tx - u.x).hypot(u.ty - u.y);
            if d > 0.5 {
                let f = (SPEED * 1.2 * dt / d).min(1.0);
                u.x += (u.tx - u.x) * f;
                u.y += (u.ty - u.y) * f;
                active = true;
            }
        }
        let redraw = active || self.dirty;
        self.dirty = false;
        redraw
    }
}
