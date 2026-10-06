//! Application state and per-frame logic: input, walking, click-to-move, zones, presence.
//! Pure logic (no window, no sockets) so it runs the same in the window, in tests and in headless replays.
use crate::calls::{MediaCmd, MediaEvent};
use crate::proto::{ClusterInfo, Cmd, Net, User};
use crate::world::{tile, World, T};
use std::collections::HashMap;

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
    pub dirty: bool,
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
            dirty: true,
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
        Avatar { tx: u.x, ty: u.y, id: u.id, name: u.name, x: u.x, y: u.y, state: u.state, speaking: false }
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
                self.users.insert(u.id.clone(), Self::avatar(u));
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
            Act::Mic => self.toggle_mic(),
            Act::Cam => self.toggle_cam(),
            Act::Share => self.toggle_share(),
            Act::Away => {
                let a = !self.available;
                self.set_available(a);
            }
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
                    None => self.call.tiles.push(Tile { key, name, img }),
                }
            }
            MediaEvent::VideoGone(key) => self.call.tiles.retain(|t| t.key != key),
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
            let z = self.world.zone_at(p.0, p.1).map(|z| z.id.clone());
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
