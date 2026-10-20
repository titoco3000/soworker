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
    last_overlay: Option<crate::overlay::Overlay>,
    last_hint: f64,
    pub hint_out: bool,
    /// What the always-on-top pill should show next (the window drains this), and whether it is showing now.
    pub overlay_out: Option<crate::overlay::Overlay>,
    overlay_shown: bool,
    pub cluster: Option<ClusterInfo>,
    pub scale: f32,
    pub keys: Keys,
    pub path: Option<Vec<(f32, f32)>>,
    pub path_goal: (f32, f32),
    pub available: bool,
    pub dry: bool,
    pub status: String,
    pub toast: Option<(String, f64)>,
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
            users: HashMap::new(),
            roster: vec![],
            sidebar: Default::default(),
            waves: vec![],
            chat: Default::default(),
            chat_out: vec![],
            sounds: vec![],
            focused: true,
            talk_key: crate::talk::DEFAULT_KEY.into(),
            talk_held: false,
            last_overlay: None,
            last_hint: -1e9,
            hint_out: false,
            overlay_out: None,
            overlay_shown: false,
            cluster: None,
            scale: 0.35,
            keys: Keys::default(),
            path: None,
            path_goal: (0.0, 0.0),
            available: true,
            dry,
            status: "connecting…".into(),
            toast: None,
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
                self.sync_overlay(); // the number of listeners may have changed
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

    /// Ask for a "Press F1 to talk" notification, unless we are unmuted or just showed one.
    fn hint_talk(&mut self, now: f64) {
        if self.call.mic || now - self.last_hint < 60.0 {
            return;
        }
        self.last_hint = now;
        self.hint_out = true;
    }

    /// Keep the always-on-top pill in step: it shows while the mic is open and the window is out of sight, so you can
    /// see that you are live (and how many are listening); when the mic closes it says "Muted" briefly.
    fn sync_overlay(&mut self) {
        use crate::overlay::Overlay;
        let live = !self.focused && self.call.joined.is_some() && self.call.mic;
        if live {
            let listeners = self.cluster.as_ref().map_or(0, |c| c.users.iter().filter(|(i, _)| *i != self.me_id).count());
            let o = Overlay::Live { listeners };
            if !self.overlay_shown || self.overlay_out.is_some() || self.last_overlay.as_ref() != Some(&o) {
                self.last_overlay = Some(o.clone());
                self.overlay_out = Some(o);
            }
            self.overlay_shown = true;
        } else if self.overlay_shown {
            self.overlay_shown = false;
            self.last_overlay = None;
            self.overlay_out = Some(if self.focused { Overlay::Hide } else { Overlay::Muted });
        }
    }

    /// The window gained or lost focus. Leaving it during a meeting while muted reminds the user of the talk key.
    pub fn set_focused(&mut self, focused: bool, now: f64) {
        self.focused = focused;
        self.sync_overlay();
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
            Hit::TileBtn(id, TileBtn::Mute) => {
                if id != "self" && !self.call.peer_muted.contains(&id) {
                    let name = self.users.get(&id).map_or_else(|| "them".to_string(), |u| u.name.clone());
                    self.media_out.push(MediaCmd::AskToMute(id));
                    self.say(format!("Asked {name} to mute"), now);
                }
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
                    self.media_out.push(MediaCmd::Devices);
                }
            }
            Act::TestSound => self.sounds.push(crate::sound::Sound::ReceiveWave),
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

    /// An on-screen button was pressed.
    pub fn sidebar_press(&mut self, hit: crate::directory::Hit, now: f64) {
        use crate::directory::{join_target, Hit};
        match hit {
            Hit::Away => {
                let a = !self.available;
                self.set_available(a);
            }
            Hit::Profile => self.toggle_profile(),
            Hit::Terminal => self.switch = Some(crate::mode::UiMode::Tui),
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
        self.dirty = true;
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
                self.call.peers = None;
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
            MediaEvent::MicState(on) => {
                self.call.mic = on;
                self.sync_overlay();
            }
            MediaEvent::CameraState(on) => {
                if on != self.call.cam {
                    self.outbox.push(Cmd::Camera(on)); // gaia shows the camera indicator to others
                }
                self.call.cam = on;
            }
            MediaEvent::ShareState(on) => self.call.share = on,
            MediaEvent::AskedToMute { from } => {
                // like the official client: only act if our mic is actually on
                if self.call.mic {
                    let who = self.users.get(&from).map_or_else(|| "Someone".to_string(), |u| u.name.clone());
                    self.media_out.push(MediaCmd::Mic(false));
                    self.say(format!("{who} asked you to mute"), now);
                }
            }
            MediaEvent::Peers(p) => self.call.peers = Some(p),
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
        self.dirty = false;
        redraw
    }
}
