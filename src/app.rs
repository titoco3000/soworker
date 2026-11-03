//! Application state and per-frame logic: input, walking, click-to-move, zones, presence.
//! Pure logic (no window, no sockets) so it runs the same in the window and in tests.
use crate::calls::{MediaCmd, MediaEvent};
use crate::meeting::{Ctl, Dock, Hit, TileBtn};
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
    pub photo: Option<String>,
    pub portrait: Option<String>,
    pub status: String,
    pub mode: String,
    pub zone: Option<String>,
}

impl Avatar {
    /// Away = out of calls (`none-away`) or set to busy.
    pub fn away(&self) -> bool {
        self.mode == "none-away" || self.status == "busy"
    }
}

/// Someone who waved at us and has not been answered yet.
#[derive(Clone, Debug)]
pub struct WaveIn {
    pub from: String,
    pub name: String,
    pub at: f64,
}

/// A picture in RGBA, as it arrives from the loaders (avatars are premultiplied, video is opaque).
pub struct Frame {
    pub w: u32,
    pub h: u32,
    pub rgba: Vec<u8>,
}

pub struct Tile {
    pub key: String,
    pub name: String,
    pub img: Frame,
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
    /// Who is in the LiveKit room right now (None until the room reports); the tiles follow this, not the gaia roster.
    pub peers: Option<Vec<(String, String)>>,
    /// A call we left on purpose: ignore further tokens for this room until the server moves us elsewhere.
    pub ignore_room: Option<String>,
    pub tiles: Vec<Tile>,
    pub speakers: std::collections::HashSet<String>,
    /// When we raised our hand (Unix ms), while it is up; and who else in the call has theirs up, with when we first
    /// saw each one go up (Unix ms, our own clock, so the order is the order we noticed them in).
    pub hand: Option<i64>,
    pub hands: HashMap<String, i64>,
    /// Since when LiveKit has listed us as speaking without a break: a second of it lowers our hand.
    pub talking_since: Option<f64>,
    /// Since when we record the call, while we do; and the elapsed seconds last drawn (the clock repaints once a second).
    pub recording: Option<std::time::Instant>,
    pub rec_shown: u64,
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
    /// The saved sign-in could not be checked because there is no connection; retrying may well work.
    Offline,
}

impl LoginPhase {
    /// The screen for a sign-in that ended with `err`.
    pub fn after_error(err: String) -> Self {
        if err.contains("cancelled") {
            LoginPhase::Idle
        } else if crate::session::is_offline(&err) {
            LoginPhase::Offline
        } else {
            LoginPhase::Failed(err)
        }
    }

    /// The screen when loading the saved sign-in at start-up failed with `err`.
    pub fn after_load_error(session_exists: bool, err: String) -> Self {
        if !session_exists {
            LoginPhase::Idle
        } else if crate::session::is_offline(&err) {
            LoginPhase::Offline
        } else {
            LoginPhase::Failed(format!("Your saved sign-in no longer works ({err})."))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LoginAct {
    Start,
    Cancel,
    /// Check the saved sign-in again (after it could not be checked offline).
    Retry,
}

#[cfg_attr(not(feature = "gui"), allow(dead_code))]
pub struct State {
    pub world: World,
    pub me_id: String,
    pub room: String,
    pub spawn: Option<(f32, f32)>,
    /// The room's desks by id, the room's definition id, and who owns which desk (None until Firestore answers).
    pub desks: HashMap<String, (f32, f32)>,
    pub room_def: String,
    pub desk_owners: Option<Vec<crate::desks::Desk>>,
    pub users: HashMap<String, Avatar>,
    /// The office's member list (online and offline) and what the sidebar is showing.
    pub roster: Vec<crate::roster::Member>,
    pub sidebar: crate::directory::Ui,
    pub waves: Vec<WaveIn>,
    pub chat: crate::chatstate::Chat,
    pub chat_out: Vec<crate::chat::ChatCmd>,
    /// UI sounds to play; the window drains this.
    pub sounds: Vec<crate::sound::Sound>,
    /// Window focus, the push-to-talk key's name, whether we opened the mic for a held key, and a pending "Press F1 to talk" notification.
    pub focused: bool,
    pub talk_key: String,
    talk_held: bool,
    last_hint: f64,
    pub hint_out: bool,
    pub cluster: Option<ClusterInfo>,
    pub scale: f32,
    pub keys: Keys,
    pub path: Option<Vec<(f32, f32)>>,
    pub path_goal: (f32, f32),
    pub available: bool,
    pub dry: bool,
    pub status: String,
    pub toast: Option<(String, f64)>,
    /// The latest recording's transcript and when that news came: on screen while it is made, a little while once
    /// saved, and a failure until dismissed.
    pub transcript: Option<(crate::transcribe::Status, f64)>,
    pub outbox: Vec<Cmd>,
    pub media_out: Vec<MediaCmd>,
    /// Noise cancellation on the microphone (the `noiseCancellation` setting); the media thread is told at start.
    pub noise_cancel: bool,
    pub call: CallUi,
    pub dock: Dock,
    /// Circle-masked avatar pictures by user id, and what we already asked the loader for.
    pub pics: HashMap<String, Frame>,
    pub avatar_req: Vec<(String, String)>,
    avatar_asked: HashMap<String, String>,
    left_zone: Option<String>,
    pub dirty: bool,
    /// Something besides a video frame changed since the last draw (the window redraws the map layer, not just the video).
    pub scene: bool,
    /// Set when the user asked to move to the other client; the frontend hands over and closes.
    pub switch: Option<crate::mode::UiMode>,
    /// The profile panel (log out, devices), while it is open.
    pub profile: Option<crate::profile::Profile>,
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
        let noise_cancel = crate::calls::noise_cancel_setting();
        State {
            world: World::default(),
            me_id: me_id.into(),
            room: String::new(),
            spawn: None,
            desks: HashMap::new(),
            room_def: String::new(),
            desk_owners: None,
            users: HashMap::new(),
            roster: vec![],
            sidebar: crate::directory::Ui { collapsed: crate::calls::setting("sidebarCollapsed").as_bool().unwrap_or(false), ..Default::default() },
            waves: vec![],
            chat: Default::default(),
            chat_out: vec![],
            sounds: vec![],
            focused: true,
            talk_key: crate::talk::DEFAULT_KEY.into(),
            talk_held: false,
            last_hint: -1e9,
            hint_out: false,
            cluster: None,
            scale: 0.35,
            keys: Keys::default(),
            path: None,
            path_goal: (0.0, 0.0),
            available: true,
            dry,
            status: "connecting…".into(),
            toast: None,
            transcript: None,
            outbox: vec![],
            media_out: vec![MediaCmd::NoiseCancel(noise_cancel)],
            noise_cancel,
            call: CallUi::default(),
            dock: Dock::default(),
            pics: HashMap::new(),
            avatar_req: vec![],
            avatar_asked: HashMap::new(),
            left_zone: None,
            dirty: true,
            scene: true,
            switch: None,
            profile: None,
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
        Avatar { tx: u.x, ty: u.y, id: u.id, name: u.name, x: u.x, y: u.y, state: u.state, speaking: false, photo: u.photo, portrait: u.portrait, status: u.status, mode: u.mode, zone: u.zone }
    }

    /// Ask the loader for a user's picture when we know where it is and have not asked for that URL yet.
    #[cfg(not(feature = "gui"))]
    fn want_avatar(&mut self, _id: &str) {}

    #[cfg(feature = "gui")]
    fn want_avatar(&mut self, id: &str) {
        use crate::avatars;
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
            Net::Desks { definition, desks } => {
                self.room_def = definition;
                self.desks = desks.into_iter().map(|(id, x, y)| (id, (x, y))).collect();
            }
            Net::DeskOwners(d) => self.desk_owners = Some(d),
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
            Net::PhotoCancelled => self.toast = None,
            Net::PhotoChanged { url, .. } => {
                if let Some(me) = self.users.get_mut(&self.me_id) {
                    me.photo = Some(url);
                }
                let me = self.me_id.clone();
                self.want_avatar(&me);
                self.say("Profile photo updated", now);
            }
            Net::Avatar { id, size, rgba } => {
                if rgba.len() == (size * size * 4) as usize {
                    self.pics.insert(id, Frame { w: size, h: size, rgba });
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
            Net::Presence { id, status } => {
                if let Some(u) = self.users.get_mut(&id) {
                    u.status = status;
                }
            }
            Net::Zone { id, zone } => {
                if let Some(u) = self.users.get_mut(&id) {
                    u.zone = zone;
                }
            }
            Net::Roster(members) => self.roster = members,
            Net::Emote { sender, recipient, emote } => {
                if emote == "wave" && recipient == self.me_id && sender != self.me_id && !self.waves.iter().any(|w| w.from == sender) {
                    let name = self.name_of(&sender, None);
                    self.say(format!("{name} waved at you"), now);
                    self.sounds.push(crate::sound::Sound::ReceiveWave);
                    self.waves.push(WaveIn { from: sender, name, at: now });
                }
            }
            Net::Waves(notes) => {
                // the notification list is the truth, except for a wave that arrived over gaia a moment ago
                let fresh: Vec<WaveIn> = self.waves.iter().filter(|w| now - w.at < 20.0 && !notes.iter().any(|n| n.from == w.from)).cloned().collect();
                self.waves = notes.into_iter().map(|n| WaveIn { name: if n.name.is_empty() { self.name_of(&n.from, None) } else { n.name }, from: n.from, at: now }).collect();
                self.waves.dedup_by(|a, b| a.from == b.from);
                self.waves.extend(fresh);
            }
            Net::Chat(ev) => self.apply_chat(ev, now),
            Net::Mode { id, mode } => {
                if let Some(u) = self.users.get_mut(&id) {
                    u.mode = mode;
                }
            }
            Net::ClusterToken { conversation_id, token } => {
                if let Some(c) = self.cluster.as_mut().filter(|c| c.conversation_id == conversation_id && c.token.is_none()) {
                    eprintln!("[{now:6.1}] call token for the meeting fetched over REST");
                    c.token = Some(token);
                    self.sync_call();
                }
            }
            Net::Cluster(c) => {
                self.clusters_seen += 1;
                eprintln!("[{now:6.1}] gaia cluster event: {}", match &c {
                    None => "none (left / alone)".to_string(),
                    Some(c) => format!("{} user(s), token: {}", c.users.len(), if c.token.is_some() { "livekit" } else { c.unsupported.as_deref().unwrap_or("missing") }),
                });
                let before = self.cluster.as_ref().map(|c| c.conversation_id.clone());
                self.cluster = c;
                self.sync_call();
                let after = self.cluster.as_ref().map(|c| c.conversation_id.clone());
                if after != before && after.is_some() && self.chat.open && self.chat.tab == crate::chatstate::Tab::Area {
                    let tab = self.chat.tab.clone();
                    self.select_tab(tab); // asks for the new meeting's chat
                }
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

    /// Left and right edge of the map: between the sidebar and the chat panel.
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

    /// Walk to the desk assigned to us in this room (Ctrl+D), the way SoWork's own client does: an ordinary walk.
    pub fn go_to_desk(&mut self, now: f64) {
        let Some(owners) = &self.desk_owners else { return self.say("Still looking up the desks, try again in a moment", now) };
        let mine = owners.iter().find(|d| d.owner == self.me_id && d.room == self.room_def).and_then(|d| self.desks.get(&d.desk)).copied();
        match mine {
            Some((x, y)) => {
                self.say("Walking to your desk", now);
                self.click(x, y, false, now);
            }
            None => self.say("You have no desk in this room", now),
        }
    }

    /// Start or stop recording the call (R, or the record button). The files are named after the meeting area.
    pub fn toggle_record(&mut self, now: f64) {
        if self.call.joined.is_none() {
            return self.say("Not in a meeting", now);
        }
        if self.call.recording.is_some() {
            self.media_out.push(MediaCmd::Record(None));
        } else {
            let zone = self.cur_zone_id.as_ref().and_then(|id| self.world.zones.iter().find(|z| &z.id == id)).and_then(|z| z.name.clone());
            self.media_out.push(MediaCmd::Record(Some(zone.unwrap_or_else(|| "Meeting".into()))));
            self.say("Starting the recording…", now);
        }
    }

    /// Closes the transcript notice, unless a transcript is still being made.
    pub fn dismiss_transcript(&mut self) {
        use crate::transcribe::Status;
        if self.transcript.as_ref().is_some_and(|(s, _)| matches!(s, Status::Done(_) | Status::Failed(_))) {
            self.transcript = None;
            self.dirty = true;
        }
    }

    /// The recording's elapsed time as "12:34", while one runs.
    pub fn rec_clock(&self) -> Option<String> {
        self.call.recording.map(|t| {
            let s = t.elapsed().as_secs();
            if s >= 3600 { format!("{}:{:02}:{:02}", s / 3600, s / 60 % 60, s % 60) } else { format!("{}:{:02}", s / 60, s % 60) }
        })
    }

    /// Raise or lower our hand in the meeting (H, or the hand button).
    pub fn toggle_hand(&mut self, now: f64) {
        if self.call.joined.is_none() {
            return self.say("Not in a meeting", now);
        }
        self.call.hand = match self.call.hand {
            Some(_) => None,
            None => Some(unix_ms()),
        };
        self.media_out.push(MediaCmd::Hand(self.call.hand));
        if self.call.hand.is_some() {
            self.sounds.push(crate::sound::Sound::RaiseHand);
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
                let portrait = self.me().and_then(|m| m.portrait.clone());
                self.media_out.push(MediaCmd::Join { url: format!("wss://{}", t.api_host), token: t.token, room_id: t.room_id, portrait });
            }
            (None, Some(_)) => {
                self.call.want = None;
                self.media_out.push(MediaCmd::Leave);
            }
            _ => {}
        }
    }

    /// Ask for a "Press F1 to talk" notification, unless we are unmuted or just showed one.
    fn hint_talk(&mut self, now: f64) {
        if self.call.mic || now - self.last_hint < 60.0 {
            return;
        }
        self.last_hint = now;
        self.hint_out = true;
    }

    /// The window gained or lost focus. Leaving it during a meeting while muted reminds the user of the talk key.
    pub fn set_focused(&mut self, focused: bool, now: f64) {
        self.focused = focused;
        if !focused && self.call.joined.is_some() {
            self.hint_talk(now);
        }
    }

    /// Push to talk. `Down`/`Up` hold the mic open (only if it was muted); `Toggle` is for shortcuts that cannot report a release.
    pub fn talk(&mut self, act: crate::talk::TalkAct, now: f64) {
        use crate::talk::TalkAct;
        if self.call.joined.is_none() {
            return self.say("Not in a meeting", now);
        }
        match act {
            TalkAct::Down if !self.call.mic => {
                self.talk_held = true;
                self.media_out.push(MediaCmd::Mic(true));
            }
            TalkAct::Up if self.talk_held => {
                self.talk_held = false;
                self.media_out.push(MediaCmd::Mic(false));
            }
            TalkAct::Toggle => self.toggle_mic(),
            _ => {}
        }
    }

    pub fn toggle_noise_cancel(&mut self, now: f64) {
        self.noise_cancel = !self.noise_cancel;
        crate::calls::save_noise_cancel(self.noise_cancel);
        self.media_out.push(MediaCmd::NoiseCancel(self.noise_cancel));
        self.say(if self.noise_cancel { "Noise cancellation on" } else { "Noise cancellation off" }, now);
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
            Hit::Ctl(Ctl::Chat) => self.toggle_chat(),
            Hit::Ctl(Ctl::Mic) => self.toggle_mic(),
            Hit::Ctl(Ctl::Cam) => self.toggle_cam(),
            Hit::Ctl(Ctl::Share) => self.toggle_share(),
            Hit::Ctl(Ctl::NoiseCancel) => self.toggle_noise_cancel(now),
            Hit::Ctl(Ctl::Hand) => self.toggle_hand(now),
            Hit::Ctl(Ctl::Record) => self.toggle_record(now),
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
            Hit::TileBtn(_, TileBtn::Fullscreen) => self.dock.tile_full = !self.dock.tile_full,
            Hit::TileBtn(id, TileBtn::Pin) => {
                self.dock.tile_full = false;
                self.dock.pinned = if self.dock.pinned.as_deref() == Some(id.as_str()) { None } else { Some(id) };
            }
            // stubs, as requested: the buttons exist (as in omoffice) but acting on other people is not implemented
            Hit::TileBtn(id, TileBtn::Mute) => {
                if id != "self" && !self.call.peer_muted.contains(&id) {
                    let name = self.users.get(&id).map_or_else(|| "them".to_string(), |u| u.name.clone());
                    self.media_out.push(MediaCmd::AskToMute(id));
                    self.say(format!("Asked {name} to mute"), now);
                }
            }
            Hit::TileBtn(id, TileBtn::LowerHand) => {
                let name = self.name_of(&id, None);
                self.media_out.push(MediaCmd::LowerHand(id));
                self.say(format!("Lowered {name}'s hand"), now);
            }
            Hit::TileBtn(_, TileBtn::CameraOff) => self.say("SoWork has no way to switch off someone else's camera (it can only ask them to turn it on)", now),
            _ => {}
        }
    }

    /// Open the profile panel, or close it if it is open.
    pub fn toggle_profile(&mut self) {
        self.profile = match self.profile {
            Some(_) => None,
            None => Some(crate::profile::Profile::new()),
        };
        self.dirty = true;
    }

    /// Something in the profile panel was clicked or chosen.
    pub fn profile_act(&mut self, act: crate::profile::Act) {
        use crate::devices::{self, Kind};
        use crate::profile::Act;
        let Some(p) = self.profile.as_mut() else { return };
        match act {
            Act::Logout => {
                self.profile = None;
                self.confirm_logout = true;
            }
            Act::Toggle(k) => p.open = if p.open == Some(k) { None } else { Some(k) },
            Act::Pick(k, id) => {
                p.open = None;
                if p.chosen.get(k) != id.as_deref() {
                    devices::choose(k, id.clone());
                    p.chosen = devices::chosen();
                    if k == Kind::Input {
                        p.meter = None; // let go of the old microphone first
                        p.meter = devices::Meter::start(id);
                    }
                    if k != Kind::Screen {
                        self.media_out.push(MediaCmd::Devices); // a share picks its screen when it starts
                    }
                }
            }
            Act::TestSound => self.sounds.push(crate::sound::Sound::ReceiveWave),
            Act::ChangePhoto => {
                p.photo_hover = false;
                self.outbox.push(Cmd::ChangePhoto);
                self.toast = Some(("Choose a picture for your profile…".into(), f64::MAX)); // until the upload answers
            }
            Act::Close => self.profile = None,
            Act::Panel => p.open = None,
        }
        self.dirty = true;
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

    /// Collapses the sidebar to its avatars, or expands it again, and remembers the choice.
    pub fn toggle_sidebar(&mut self) {
        self.sidebar.collapsed = !self.sidebar.collapsed;
        self.sidebar.open = None;
        crate::calls::save_setting("sidebarCollapsed", self.sidebar.collapsed.into());
        self.dirty = true;
    }

    /// An on-screen button was pressed.
    pub fn sidebar_press(&mut self, hit: crate::directory::Hit, now: f64) {
        use crate::directory::{join_target, Hit};
        match hit {
            Hit::Away => {
                let a = !self.available;
                self.set_available(a);
            }
            Hit::Collapse => self.toggle_sidebar(),
            Hit::Profile => self.toggle_profile(),
            Hit::Terminal => self.switch = Some(crate::mode::UiMode::Tui),
            // a row of the narrow sidebar has no room for its menu: widen the sidebar first
            Hit::Row(key) if self.sidebar.collapsed => {
                self.toggle_sidebar();
                self.sidebar.open = Some(key);
            }
            Hit::Row(key) => self.sidebar.open = if self.sidebar.open.as_deref() == Some(key.as_str()) { None } else { Some(key) },
            Hit::Join(key) => {
                self.sidebar.open = None;
                match join_target(self, &key) {
                    Some(((x, y), name)) => {
                        self.say(format!("Walking to {name}"), now);
                        self.click(x, y, false, now);
                    }
                    None => self.say("Cannot find where to walk to", now),
                }
            }
            Hit::Wave(key) => {
                // a person, or everyone else in a meeting
                let targets: Vec<String> = match key.strip_prefix("zone:") {
                    Some(z) => crate::directory::people(self).into_iter().filter(|p| p.online && !p.me && p.zone.as_deref() == Some(z)).map(|p| p.id).collect(),
                    None => vec![key.clone()],
                };
                let names: Vec<String> = targets.iter().map(|t| self.name_of(t, None)).collect();
                if !targets.is_empty() {
                    self.sounds.push(crate::sound::Sound::SendWave);
                }
                for t in targets {
                    self.outbox.push(Cmd::Wave(t));
                }
                self.sidebar.open = None;
                if !names.is_empty() {
                    self.say(format!("Waved at {}", names.join(", ")), now);
                }
            }
            Hit::Message(key) => {
                self.sidebar.open = None;
                self.open_dm(&key);
            }
            Hit::Chat => self.toggle_chat(),
            Hit::WaveAccept(from) => {
                // accepting a wave means going to the person, as the official client does
                if let Some(((x, y), name)) = join_target(self, &from) {
                    self.say(format!("Walking to {name}"), now);
                    self.click(x, y, false, now);
                } else {
                    self.say("Cannot find where they are", now);
                }
                self.waves.retain(|w| w.from != from);
                self.outbox.push(Cmd::WaveDone(from));
            }
            Hit::WaveDismiss(from) => {
                self.waves.retain(|w| w.from != from);
                self.outbox.push(Cmd::WaveDone(from));
            }
            Hit::Panel => {}
        }
        self.dirty = true;
    }

    pub fn apply_media(&mut self, ev: MediaEvent, now: f64) {
        // a new frame of a tile already shown repaints that tile only: the rest of the scene stays as drawn
        self.dirty = !matches!(&ev, MediaEvent::Video { key, .. } if self.call.tiles.iter().any(|t| &t.key == key));
        match ev {
            MediaEvent::Joined(r) => {
                eprintln!("[{now:6.1}] livekit joined {r}");
                self.call.joined = Some(r);
                if !self.focused {
                    self.hint_talk(now); // we joined while the window is out of sight: say how to unmute
                }
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
                self.call.hand = None;
                self.call.hands.clear();
                self.call.talking_since = None;
                self.call.recording = None;
                self.call.peers = None;
                self.dock.pinned = None;
                self.dock.tile_full = false;
                self.dock.last_auto = None;
                for u in self.users.values_mut() {
                    u.speaking = false;
                }
            }
            MediaEvent::Error(e) => {
                eprintln!("[{now:6.1}] media error: {e}");
                self.say(e, now)
            }
            MediaEvent::MicState(on) => {
                self.call.mic = on;
            }
            MediaEvent::CameraState(on) => {
                if on != self.call.cam {
                    self.outbox.push(Cmd::Camera(on)); // gaia shows the camera indicator to others
                }
                self.call.cam = on;
            }
            MediaEvent::ShareState(on) => self.call.share = on,
            MediaEvent::Recording(on) => {
                if on != self.call.recording.is_some() {
                    // the others cannot see that we record: say so in the meeting's chat
                    let me = self.me().map_or_else(|| "Someone".to_string(), |m| m.name.clone());
                    if let Some(conversation) = self.conversation() {
                        let text = format!("{me} {} recording this meeting", if on { "started" } else { "stopped" });
                        self.chat_out.push(crate::chat::ChatCmd::SendArea { conversation, text });
                    }
                    self.say(if on { "Recording" } else { "Recording stopped" }, now);
                }
                self.call.recording = on.then(std::time::Instant::now);
            }
            MediaEvent::Transcript(status) => {
                use crate::transcribe::Status;
                if matches!(status, Status::Done(_) | Status::Failed(_)) {
                    eprintln!("[{now:6.1}] {}", status.line());
                    crate::notify::message(&status.line()); // it may well finish after the call, with the window hidden
                }
                self.transcript = Some((status, now));
            }
            MediaEvent::Info(msg) => {
                eprintln!("[{now:6.1}] {msg}");
                self.say(msg, now);
            }
            MediaEvent::AskedToMute { from } => {
                // like the official client: only act if our mic is actually on
                if self.call.mic {
                    let who = self.users.get(&from).map_or_else(|| "Someone".to_string(), |u| u.name.clone());
                    self.media_out.push(MediaCmd::Mic(false));
                    self.say(format!("{who} asked you to mute"), now);
                }
            }
            MediaEvent::Peers(p) => self.call.peers = Some(p),
            MediaEvent::HandSetBy { from, raised } => {
                // like SoWork's client: the message says what our hand should be, and we set it ourselves
                if raised != self.call.hand.is_some() {
                    self.toggle_hand(now);
                    let who = self.name_of(&from, None);
                    self.say(format!("{who} {} your hand", if raised { "raised" } else { "lowered" }), now);
                }
            }
            MediaEvent::Hands(ids) => {
                // a chime for each hand that just went up, as SoWork's client does
                if ids.iter().any(|id| !self.call.hands.contains_key(id)) {
                    self.sounds.push(crate::sound::Sound::RaiseHand);
                }
                let at = unix_ms();
                self.call.hands = ids.into_iter().map(|id| { let t = self.call.hands.get(&id).copied().unwrap_or(at); (id, t) }).collect();
            }
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
                self.call.talking_since = if is { self.call.talking_since.or(Some(now)) } else { None };
            }
            MediaEvent::Video { key, name, w, h, rgba } => {
                if rgba.len() != (w * h * 4) as usize {
                    return;
                }
                let img = Frame { w, h, rgba };
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
                    self.dock.tile_full = false;
                }
            }
        }
    }

    #[cfg_attr(not(feature = "gui"), allow(dead_code))]
    pub fn zoom(&mut self, up: bool) {
        self.scale = (self.scale * if up { 1.1 } else { 1.0 / 1.1 }).clamp(0.08, 1.5);
        crate::calls::save_setting("guiZoom", self.scale.into());
        self.dirty = true;
    }

    /// The window's zoom as it was last left (`guiZoom` in settings.json).
    #[cfg_attr(not(feature = "gui"), allow(dead_code))]
    pub fn restore_zoom(&mut self) {
        if let Some(s) = crate::calls::setting("guiZoom").as_f64() {
            self.scale = (s as f32).clamp(0.08, 1.5);
        }
    }

    pub fn set_available(&mut self, a: bool) {
        self.available = a;
        self.outbox.push(Cmd::Available(a)); // gaia cluster_mode
        self.outbox.push(Cmd::Presence { away: !a }); // REST presence status, as SoWork's own client does
        self.dirty = true;
    }

    /// True while something animates (walking, a path, avatars gliding) so the window keeps ticking.
    pub fn is_active(&self) -> bool {
        let k = self.keys;
        self.moving
            || self.path.is_some()
            || self.profile.as_ref().is_some_and(|p| p.meter.is_some()) // the level meter moves
            || (self.call.hand.is_some() && self.call.talking_since.is_some()) // the hand may go down in a moment
            || k.up || k.down || k.left || k.right
            || self.users.values().any(|u| u.id != self.me_id && (u.tx - u.x).hypot(u.ty - u.y) > 0.5)
    }

    /// Advance the simulation. Returns true if the screen needs a redraw (movement or events).
    pub fn update(&mut self, dt: f32, now: f64) -> bool {
        if self.world.rebuild_if_dirty() {
            self.check_spawn();
            self.dirty = true;
        }
        if self.call.hand.is_some() && self.call.talking_since.is_some_and(|t| now - t >= 1.0) {
            // we got the word: after a second of speaking the hand goes down by itself
            self.call.hand = None;
            self.media_out.push(MediaCmd::Hand(None));
            self.say("Hand lowered", now);
        }
        if self.toast.as_ref().is_some_and(|(_, until)| now > *until) {
            self.toast = None;
            self.dirty = true;
        }
        if self.transcript.as_ref().is_some_and(|(s, at)| matches!(s, crate::transcribe::Status::Done(_)) && now - at > 15.0) {
            self.transcript = None;
            self.dirty = true;
        }
        if let Some(t) = self.call.recording {
            let secs = t.elapsed().as_secs();
            if secs != self.call.rec_shown {
                self.call.rec_shown = secs; // the recording clock ticks
                self.dirty = true;
            }
        }
        let mut active = self.moving || self.path.is_some() || self.profile.as_ref().is_some_and(|p| p.meter.is_some());
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
        self.scene |= redraw;
        self.dirty = false;
        redraw
    }
}

fn unix_ms() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::desks::Desk;

    #[test]
    fn ctrl_d_finds_our_desk_in_this_room() {
        let mut st = State::new("me", true);
        st.apply(Net::Desks { definition: "R1".into(), desks: vec![("d1".into(), 100.0, 200.0), ("d2".into(), 300.0, 400.0)] }, 0.0);
        st.go_to_desk(0.0);
        assert!(st.toast.as_ref().is_some_and(|(m, _)| m.contains("looking up")));
        let desk = |room: &str, desk: &str, owner: &str| Desk { room: room.into(), desk: desk.into(), owner: owner.into() };
        st.apply(Net::DeskOwners(vec![desk("R1", "d1", "ana"), desk("R2", "d2", "me")]), 0.0);
        st.go_to_desk(0.0);
        assert_eq!(st.toast.as_ref().unwrap().0, "You have no desk in this room", "our desk is in another room");
        st.apply(Net::DeskOwners(vec![desk("R1", "d2", "me")]), 0.0);
        st.go_to_desk(0.0);
        assert_eq!(st.toast.as_ref().unwrap().0, "Walking to your desk");
    }

    #[test]
    fn the_hand_goes_up_and_down_only_in_a_call() {
        let mut st = State::new("me", true);
        st.media_out.clear();
        st.toggle_hand(0.0);
        assert!(st.call.hand.is_none() && st.media_out.is_empty());
        st.apply_media(MediaEvent::Joined("RM".into()), 0.0);
        st.toggle_hand(0.0);
        assert!(st.call.hand.is_some_and(|t| t > 1_700_000_000_000));
        assert!(matches!(st.media_out.last(), Some(MediaCmd::Hand(Some(_)))));
        st.toggle_hand(0.0);
        assert!(matches!(st.media_out.last(), Some(MediaCmd::Hand(None))));
        st.toggle_hand(0.0);
        assert_eq!(st.sounds.iter().filter(|s| **s == crate::sound::Sound::RaiseHand).count(), 2, "a chime per raise, none for lowering");
        st.sounds.clear();
        st.apply_media(MediaEvent::Hands(vec!["ana".into()]), 0.0);
        assert_eq!(st.sounds, [crate::sound::Sound::RaiseHand], "ana raised hers");
        let order = |st: &State| crate::meeting::tiles(st).iter().map(|t| (t.id.clone(), t.hand)).collect::<Vec<_>>();
        st.call.peers = Some(vec![("ana".into(), "Ana".into()), ("bo".into(), "Bo".into()), ("cy".into(), "Cy".into())]);
        let mine = st.call.hand.unwrap();
        assert!(st.call.hands["ana"] >= mine, "ana's hand was noticed after ours went up");
        st.call.hands.insert("ana".into(), mine + 5);
        st.apply_media(MediaEvent::Hands(vec!["ana".into(), "bo".into()]), 0.0);
        assert_eq!(st.call.hands["ana"], mine + 5, "a hand already up keeps its time");
        st.call.hands.insert("bo".into(), mine - 5);
        assert_eq!(order(&st), [("self".into(), Some(2)), ("ana".into(), Some(3)), ("bo".into(), Some(1)), ("cy".into(), None)]);
        st.apply_media(MediaEvent::Hands(vec!["ana".into()]), 0.0);
        assert_eq!(order(&st)[1..3], [("ana".into(), Some(2)), ("bo".into(), None)], "bo lowered: ana moves up");
        st.call.peers = None;
        st.sounds.clear();
        st.apply_media(MediaEvent::Hands(vec![]), 0.0);
        assert!(st.sounds.is_empty(), "no chime when a hand goes down");
        st.apply_media(MediaEvent::Speakers(vec!["ana".into()]), 0.0);
        assert!(st.call.hand.is_some(), "someone else talking keeps our hand up");
        st.apply_media(MediaEvent::Speakers(vec!["me".into()]), 10.0);
        st.update(0.0, 10.5);
        assert!(st.call.hand.is_some(), "half a second is not enough");
        st.apply_media(MediaEvent::Speakers(vec![]), 10.6);
        st.apply_media(MediaEvent::Speakers(vec!["me".into()]), 10.8);
        st.update(0.0, 11.5);
        assert!(st.call.hand.is_some(), "a pause starts the second again");
        assert!(st.is_active(), "keeps ticking so the second is noticed");
        st.update(0.0, 11.9);
        assert!(st.call.hand.is_none() && matches!(st.media_out.last(), Some(MediaCmd::Hand(None))), "a second of speaking lowers it");
        st.toggle_hand(0.0);
        st.apply_media(MediaEvent::HandSetBy { from: "ana".into(), raised: false }, 0.0);
        assert!(st.call.hand.is_none() && matches!(st.media_out.last(), Some(MediaCmd::Hand(None))), "ana lowered it for us");
        st.dock_press(Hit::TileBtn("ana".into(), TileBtn::LowerHand), 0.0);
        assert!(matches!(st.media_out.last(), Some(MediaCmd::LowerHand(id)) if id == "ana"));
        st.toggle_hand(0.0);
        st.apply_media(MediaEvent::Left, 0.0);
        assert!(st.call.hand.is_none() && st.call.hands.is_empty(), "leaving the call lowers the hand");
    }
}
