//! `soworker --mc`: the office in Minecraft. A headless frontend that runs the same `Backend` and `State` loop as the
//! terminal client and serves a local WebSocket that the Fabric mod (`mc/`) connects to. It sends the map, the other
//! users, our own state, the call and its video; the mod sends the player's position and the tool commands back.
//! See docs/minecraft-bridge-plan.md for the whole design and the protocol.
mod demo;
mod geom;
mod palette;
mod ws;

use crate::app::State;
use crate::backend::{Backend, Emit, Event};
use crate::calls::MediaEvent;
use crate::proto::Net;
use crate::session::{self, Auth};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::Arc;
use std::time::{Duration, Instant};
use ws::In;

pub const DEFAULT_PORT: u16 = 25599;
const TICK: Duration = Duration::from_millis(33);
/// Users, our own state and the call go out at most this often (seconds).
const STATE_EVERY: f64 = 0.1;
/// Video frames per screen per second, at most.
const VIDEO_FPS: f64 = 8.0;
/// Frames of the share watched full screen: big JPEGs.
const WATCH_FPS: f64 = 8.0;
/// The widest share picture asked for while one is watched full screen, and otherwise (the wall is 512 wide).
const WATCH_WIDTH: u32 = 1920;
const WALL_WIDTH: u32 = 640;
/// Camera frames on faces: small, so a little faster.
const FACE_FPS: f64 = 10.0;

enum Msg {
    App(Event),
    Mc(In),
}

struct Bridge {
    st: State,
    be: Backend,
    link: ws::Link,
    dry: bool,
    start: Instant,
    /// The last message of each kind sent to the current client: only changes go out.
    sent: HashMap<&'static str, Value>,
    /// The map as last built, and the world generation it was built from.
    map: geom::Map,
    world_gen: Option<u64>,
    last_state: f64,
    /// The player's position has been seen where our avatar is since the client connected (until then it is ignored:
    /// the player is still on its way to where `teleport` put it).
    synced: bool,
    /// The walk target we set from the player's position; any other path is a walk soworker started (follow it).
    our_goal: Option<(f32, f32)>,
    following: bool,
    /// Where the avatar was when it last made headway (0.3 blocks), and when: see `pos`.
    headway: Option<((f32, f32), f64)>,
    /// A walk target the avatar could not reach.
    blocked_goal: Option<(f32, f32)>,
    /// When we last told the mod to put the player at our avatar.
    teleport_at: f64,
    video_at: HashMap<String, f64>,
    dither: bool,
    toast_seen: Option<(String, f64)>,
    /// Offline at start: when to check the saved sign-in again.
    retry_login_at: Option<f64>,
    /// When we last waved at each user from Minecraft (a snowball spray must not become a burst of real waves).
    waved_at: HashMap<String, f64>,
    /// The screen share shown full screen in Minecraft (a snowball hit its wall), and when its last frame went out.
    watching: Option<String>,
    watch_at: f64,
    /// `/sw exit`: close the bridge and start the client used last again.
    exit: bool,
}

/// Seconds between two snowball waves at the same user.
const WAVE_COOLDOWN: f64 = 5.0;

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

impl Bridge {
    fn now(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    fn sign_in(&mut self, auth: Auth) {
        let (me, key) = (auth.uid.clone(), self.st.talk_key.clone());
        println!("signed in, connecting to SoWork{}", if self.dry { " (dry run: nothing is sent)" } else { "" });
        self.be.start(auth, false);
        self.st = State::new(&me, self.dry);
        self.st.talk_key = key;
        self.st.set_focused(false, self.now()); // the user is looking at Minecraft
        self.world_gen = None;
        self.sent.clear();
    }

    /// A SoWork point as rounded block coordinates.
    fn block(&self, x: f32, y: f32) -> (f64, f64) {
        let (bx, bz) = geom::to_block(self.map.origin, x, y);
        (round2(bx), round2(bz))
    }

    fn me_pos(&self) -> Option<(f32, f32)> {
        self.st.me().map(|m| (m.x, m.y))
    }

    /// Send `v` as message kind `kind` unless it is what the client already has.
    fn put(&mut self, kind: &'static str, mut v: Value) {
        if self.sent.get(kind) == Some(&v) {
            return;
        }
        self.sent.insert(kind, v.clone());
        v["t"] = kind.into();
        self.link.send(&v);
    }

    fn say(&self, text: &str) {
        self.link.send(&json!({"t": "say", "text": text}));
    }

    // ------------------------------------------------------------ messages to the mod

    fn hello(&self) -> Value {
        let name = self.st.me().map(|m| m.name.clone()).unwrap_or_default();
        json!({"me": self.st.me_id, "name": name, "room": self.st.room, "dry": self.dry})
    }

    fn world_msg(&self) -> Value {
        let m = &self.map;
        let pairs = |v: &[(i32, i32)]| v.iter().map(|&(i, j)| json!([i, j])).collect::<Vec<_>>();
        let zones: Vec<Value> =
            m.zones.iter().map(|z| json!({"id": z.id, "name": z.name, "i0": z.i0, "j0": z.j0, "i1": z.i1, "j1": z.j1, "material": geom::MATERIALS[z.material]})).collect();
        json!({
            "origin": [m.origin.0, m.origin.1],
            "floors": pairs(&m.floors),
            "meeting": m.meeting.iter().map(|&(i, j, k)| json!([i, j, k])).collect::<Vec<_>>(),
            "materials": geom::MATERIALS,
            "fences": pairs(&m.fences),
            "barriers": pairs(&m.barriers),
            "zones": zones,
        })
    }

    fn users_msg(&self) -> Value {
        let queue = crate::meeting::hand_queue(&self.st);
        let mut list: Vec<&crate::app::Avatar> = self.st.users.values().filter(|u| u.id != self.st.me_id).collect();
        list.sort_by(|a, b| a.id.cmp(&b.id));
        let list: Vec<Value> = list
            .into_iter()
            .map(|u| {
                let (x, z) = self.block(u.tx, u.ty); // where they are going: the villager walks there itself
                json!({
                    "id": u.id, "name": u.name, "x": x, "z": z,
                    "walking": u.state == "walk",
                    "away": u.away(),
                    "speaking": u.speaking,
                    "muted": self.st.call.peer_muted.contains(&u.id),
                    "hand": queue.iter().position(|q| *q == u.id).map(|i| i + 1),
                    "zone": u.zone,
                })
            })
            .collect();
        json!({ "list": list })
    }

    fn has_desk(&self) -> bool {
        let st = &self.st;
        st.desk_owners.as_ref().is_some_and(|o| o.iter().any(|d| d.owner == st.me_id && d.room == st.room_def && st.desks.contains_key(&d.desk)))
    }

    fn me_msg(&self) -> Option<Value> {
        let (x, z) = self.me_pos().map(|(x, y)| self.block(x, y))?;
        Some(json!({"x": x, "z": z, "available": self.st.available, "desk": self.has_desk()}))
    }

    fn call_msg(&self) -> Value {
        let st = &self.st;
        let joined = st.call.joined.as_ref().and(st.cur_zone_id.clone());
        let zone_name = joined.as_ref().and_then(|id| st.world.zones.iter().find(|z| &z.id == id)).and_then(|z| z.name.clone());
        let me_name = st.me().map(|m| m.name.clone()).unwrap_or_default();
        let participants: Vec<Value> = if st.call.joined.is_some() {
            crate::meeting::tiles(st)
                .into_iter()
                .filter(|t| !t.content)
                .map(|t| json!({"id": t.user, "name": if t.is_self { me_name.clone() } else { t.name }, "mic": !t.muted, "cam": t.video_key.is_some(), "speaking": t.speaking}))
                .collect()
        } else {
            vec![]
        };
        json!({
            "joined": joined,
            "in_call": st.call.joined.is_some(),
            "zone_name": zone_name,
            "mic": st.call.mic, "cam": st.call.cam, "share": st.call.share,
            "rec": st.call.recording.is_some(),
            "rec_clock": st.rec_clock(),
            "hand": st.call.hand.is_some(),
            "hands": crate::meeting::hand_queue(st),
            "participants": participants,
        })
    }

    /// The screens to show: a share first, then cameras by name.
    fn screens_msg(&self) -> Value {
        let mut tiles: Vec<&crate::app::Tile> = self.st.call.tiles.iter().filter(|t| t.img.w > 0 && t.key.ends_with(":screen")).collect(); // cameras are on faces
        tiles.sort_by_key(|t| (!t.key.ends_with(":screen"), t.name.to_lowercase(), t.key.clone()));
        let list: Vec<Value> = tiles
            .into_iter()
            .map(|t| {
                let (w, h) = palette::screen_size(&t.key);
                json!({"key": t.key, "name": t.name, "kind": if t.key.ends_with(":screen") { "share" } else { "camera" }, "w": w, "h": h})
            })
            .collect();
        json!({ "list": list })
    }

    /// Everything the client should know right after it connects.
    fn full_state(&mut self) {
        self.sent.clear();
        self.synced = false;
        self.following = false;
        self.headway = None;
        self.video_at.clear();
        self.sync_out(true);
        self.teleport();
    }

    /// Put the player where our avatar is. Until its position is seen there, it is ignored (see `pos`).
    fn teleport(&mut self) {
        if let Some((x, z)) = self.me_pos().map(|(x, y)| self.block(x, y)) {
            self.link.send(&json!({"t": "teleport", "x": x, "z": z}));
            self.synced = false;
            self.teleport_at = self.now();
        }
    }

    /// Send whatever changed. The map whenever the world was rebuilt; the rest at most every `STATE_EVERY`.
    fn sync_out(&mut self, force: bool) {
        if !self.link.connected() || self.st.me_id.is_empty() {
            return;
        }
        let h = self.hello();
        self.put("hello", h);
        if self.world_gen != Some(self.st.world.generation) || force {
            self.world_gen = Some(self.st.world.generation);
            self.map = geom::build(&self.st.world);
            let w = self.world_msg();
            self.put("world", w);
        }
        let now = self.now();
        if !force && now - self.last_state < STATE_EVERY {
            return;
        }
        self.last_state = now;
        let u = self.users_msg();
        self.put("users", u);
        if let Some(m) = self.me_msg() {
            self.put("me", m);
        }
        let c = self.call_msg();
        self.put("call", c);
        let s = self.screens_msg();
        self.put("screens", s);
        self.follow();
    }

    /// A walk soworker started (to the desk at start-up, the desk tool): the mod carries the player along.
    fn follow(&mut self) {
        if self.walk_of_ours() {
            if let Some((x, z)) = self.me_pos().map(|(x, y)| self.block(x, y)) {
                self.following = true;
                self.put("follow", json!({"x": x, "z": z}));
            }
        } else if self.following {
            self.following = false;
            self.sent.remove("follow");
            self.link.send(&json!({"t": "follow_end"}));
        }
    }

    fn toasts(&mut self) {
        if self.st.toast != self.toast_seen {
            if let Some((text, _)) = &self.st.toast {
                self.say(text);
            }
            self.toast_seen = self.st.toast.clone();
        }
    }

    // ------------------------------------------------------------ messages from the mod

    fn from_mod(&mut self, v: Value) {
        let now = self.now();
        match v["t"].as_str().unwrap_or("") {
            "pos" => {
                if let (Some(x), Some(z)) = (v["x"].as_f64(), v["z"].as_f64()) {
                    self.pos(x, z, v["input"].as_bool().unwrap_or(false), now);
                }
            }
            // the mod was switched on in a world, or the player joined: everything again
            "resync" => self.full_state(),
            "cmd" => {
                let name = v["name"].as_str().unwrap_or("").to_string();
                let arg = v["arg"].as_str().unwrap_or("").trim().to_string();
                self.cmd(&name, &arg, now);
            }
            other => eprintln!("mc: unknown message {other:?}"),
        }
    }

    /// A walk soworker started (not one we set from the player's position) is under way.
    fn walk_of_ours(&self) -> bool {
        self.st.path.is_some() && self.our_goal != Some(self.st.path_goal)
    }

    /// The player is at block (bx, bz). `input`: the mod saw the player press a movement key.
    fn pos(&mut self, bx: f64, bz: f64, input: bool, now: f64) {
        let Some(me) = self.me_pos() else { return };
        if self.st.world.floors.is_empty() {
            return;
        }
        let (x, y) = geom::from_block(self.map.origin, bx, bz);
        let apart = (x - me.0).hypot(y - me.1) / geom::BLOCK; // blocks
        if !self.synced {
            if apart < 1.5 {
                self.synced = true;
            } else {
                if now - self.teleport_at > 2.0 {
                    self.teleport(); // the last one did not arrive, or the player was moved since
                }
                return;
            }
        }
        if self.walk_of_ours() && !input {
            // being carried along (or about to be: `follow` goes out with the next state): the player's own position
            // is not a wish to go anywhere. A movement key takes over.
            if !self.following || apart < 3.0 {
                return;
            }
        }
        // Put the player back only when the avatar cannot follow: it made no headway for a second while the player is
        // away from it (a wall Minecraft does not have). An avatar behind but still walking is just slower (sprinting).
        // An avatar standing with nowhere to go (not one that gave up on a wall) is not stalled: it is about to set off.
        let idle = self.st.path.is_none() && self.blocked_goal.is_none();
        if idle || self.headway.is_none_or(|(p, _)| (me.0 - p.0).hypot(me.1 - p.1) > 0.3 * geom::BLOCK) {
            self.headway = Some((me, now));
        }
        let stalled = self.headway.is_some_and(|(_, t)| now - t > 1.0);
        if (apart > 2.0 && stalled) || apart > 16.0 {
            self.headway = None;
            return self.teleport();
        }
        if (x - me.0).hypot(y - me.1) > 12.0 {
            // a spot the avatar already failed to reach: wait for the player to move on rather than bump again
            if self.st.path.is_none() && self.blocked_goal.is_some_and(|g| (g.0 - x).hypot(g.1 - y) < 0.25 * geom::BLOCK) {
                return;
            }
            self.blocked_goal = None;
            // Straight at the player, sliding along walls. The fences stand a little inside the tile whose edge the
            // wall is on, so a player pressed against one can be on the far side of SoWork's wall line: the slide
            // stops at the wall instead of routing all the way round to the other side (or giving up and snapping
            // the player back). Only when the slide ends well short (the player walked round a corner) a route.
            let reach = self.st.world.step(me, x - me.0, y - me.1);
            let (path, goal) = if (reach.0 - x).hypot(reach.1 - y) < 1.5 * geom::BLOCK {
                (vec![reach], reach)
            } else {
                self.st.world.find_path(me, (x, y)).unwrap_or_else(|| (vec![reach], reach))
            };
            self.st.path = Some(path);
            self.st.path_goal = goal;
            self.our_goal = Some(goal);
        }
    }

    /// The desk walk has started: a minecart track to the desk for the player, when there is a way along the floor
    /// blocks. The mod lays it, seats the player and drives the cart; the avatar walks there in SoWork meanwhile.
    fn rail_to_desk(&mut self) {
        if !self.walk_of_ours() {
            return; // no desk, already there, or no walking route
        }
        let Some(me) = self.me_pos() else { return };
        let goal = self.st.path_goal;
        let from = geom::block_of(self.map.origin, me.0, me.1);
        let to = geom::block_of(self.map.origin, goal.0, goal.1);
        match geom::rail_route(&self.map, from, to) {
            Some(r) if r.len() >= 3 => {
                self.link.send(&json!({"t": "rail", "path": r.iter().map(|&(x, z)| json!([x, z])).collect::<Vec<_>>()}));
            }
            _ => {} // too close, or the floor blocks do not connect: an ordinary walk
        }
    }

    fn cmd(&mut self, name: &str, arg: &str, now: f64) {
        use crate::meeting::{Ctl, Hit};
        let st = &mut self.st;
        match name {
            "mic" => st.toggle_mic(),
            "share" => st.toggle_share(),
            "rec" => st.toggle_record(now),
            "cam" => st.toggle_cam(),
            "hand" => st.toggle_hand(now),
            "desk" => {
                st.go_to_desk(now);
                self.rail_to_desk();
            }
            "away" => {
                let a = !st.available;
                st.set_available(a);
                st.say(if a { "You are available" } else { "You are away" }, now);
            }
            "leave" if st.call.joined.is_some() || st.call.want.is_some() => st.dock_press(Hit::Ctl(Ctl::Leave), now),
            "leave" => st.say("Not in a meeting", now),
            // a snowball hitting a coworker's villager: the arg is the user id
            "wave_id" => {
                if arg == st.me_id || !st.users.contains_key(arg) {
                    return;
                }
                if self.waved_at.get(arg).is_some_and(|&t| now - t < WAVE_COOLDOWN) {
                    return;
                }
                self.waved_at.insert(arg.to_string(), now);
                st.sidebar_press(crate::directory::Hit::Wave(arg.to_string()), now);
            }
            "wave" => {
                let want = arg.to_lowercase();
                let found = st.users.values().filter(|u| u.id != st.me_id).find(|u| u.name.to_lowercase() == want).or_else(|| st.users.values().filter(|u| u.id != st.me_id).find(|u| u.name.to_lowercase().starts_with(&want) && !want.is_empty())).map(|u| u.id.clone());
                match found {
                    Some(id) => st.sidebar_press(crate::directory::Hit::Wave(id), now),
                    None => st.say(format!("Nobody called {arg:?} is here"), now),
                }
            }
            // a snowball hit a screen share's wall: show it full screen until `unwatch`
            "watch" if st.call.tiles.iter().any(|t| t.key == arg && arg.ends_with(":screen")) => {
                self.watching = Some(arg.to_string());
                self.watch_at = 0.0;
                crate::calls::set_video_width(640, WATCH_WIDTH);
            }
            "watch" => self.say("That screen share has ended"),
            "unwatch" => self.unwatch(),
            "exit" => {
                self.say("Back to SoWorker: closing Minecraft mode");
                self.exit = true;
            }
            // an ender pearl landed at block (x, z): SoWork's teleport (Shift+click) there, then put the player where
            // the avatar ended up (back where they were when the spot is off the floor)
            "pearl" => {
                let mut it = arg.split_whitespace().filter_map(|v| v.parse::<f64>().ok());
                if let (Some(bx), Some(bz)) = (it.next(), it.next()) {
                    let (x, y) = geom::from_block(self.map.origin, bx, bz);
                    let before = self.me_pos();
                    self.st.click(x, y, true, now);
                    if self.st.confirm_leave.is_some() {
                        self.st.leave_choice(true, now); // landing outside the meeting is the answer to "leave it?"
                    }
                    if self.me_pos() == before {
                        self.say("You cannot teleport there");
                    }
                    self.our_goal = None;
                    self.blocked_goal = None;
                    self.headway = None;
                    self.teleport();
                }
            }
            "say" | "chat" if arg.is_empty() => {}
            // `chat`: a plain Minecraft chat line, which Minecraft shows itself; `say`: `/sw say`, which it does not
            "say" | "chat" => match st.conversation() {
                Some(conversation) => {
                    st.chat_out.push(crate::chat::ChatCmd::SendArea { conversation, text: arg.to_string() });
                    // the chat socket does not echo our own lines back: show it the way others' lines are shown
                    if name == "say" {
                        let me = st.users.get(&st.me_id).map(|u| u.name.clone()).unwrap_or_else(|| "You".into());
                        self.say(&format!("{me}: {arg}"));
                    }
                }
                None => st.say("Room chat works in a meeting", now),
            },
            "status" => {
                let s = self.status_line();
                self.say(&s);
            }
            other => {
                eprintln!("mc: unknown command {other:?}");
                self.say(&format!("Unknown command {other:?}"));
            }
        }
    }

    fn unwatch(&mut self) {
        if self.watching.take().is_some() {
            crate::calls::set_video_width(640, WALL_WIDTH);
        }
    }

    fn status_line(&self) -> String {
        let st = &self.st;
        let mut parts = vec![if st.available { "available".to_string() } else { "away".to_string() }];
        if !st.room.is_empty() {
            parts.push(format!("room {}", st.room));
        }
        parts.push(format!("{} other(s) here", st.users.len().saturating_sub(1)));
        if st.call.joined.is_some() {
            parts.push(format!("in a call, mic {}, camera {}", if st.call.mic { "on" } else { "muted" }, if st.call.cam { "on" } else { "off" }));
        }
        if let Some(c) = st.rec_clock() {
            parts.push(format!("recording {c}"));
        }
        if self.dry {
            parts.push("dry run".into());
        }
        if !st.status.is_empty() {
            parts.push(st.status.clone());
        }
        parts.join(", ")
    }

    // ------------------------------------------------------------ events

    fn media(&mut self, ev: MediaEvent, now: f64) {
        match &ev {
            MediaEvent::Video { key, w: 0, .. } if self.st.call.tiles.iter().any(|t| &t.key == key) => return, // "still there"
            MediaEvent::Video { key, w, h, rgba, .. } if *w > 0 && self.link.connected() => {
                if self.watching.as_ref() == Some(key) && now - self.watch_at >= 1.0 / WATCH_FPS && rgba.len() == (*w * *h * 4) as usize {
                    self.watch_at = now;
                    match palette::jpeg(*w, *h, rgba) {
                        Ok(j) => self.link.video(&format!("{key}#full"), palette::full_message(key, *w, *h, &j)),
                        Err(e) => eprintln!("mc: jpeg: {e}"),
                    }
                }
                let fps = if key.ends_with(":screen") { VIDEO_FPS } else { FACE_FPS };
                if self.video_at.get(key).is_none_or(|t| now - t >= 1.0 / fps) && rgba.len() == (*w * *h * 4) as usize {
                    self.video_at.insert(key.clone(), now);
                    if key.ends_with(":screen") {
                        let (sw, sh) = palette::screen_size(key);
                        let px = palette::encode_pixels(*w, *h, rgba, sw, sh, self.dither);
                        self.link.video(key, palette::frame_message(key, sw, sh, &px));
                    } else {
                        // a camera goes on its owner's face (key = user id, or "me")
                        let px = palette::face_pixels(*w, *h, rgba, palette::FACE);
                        self.link.video(key, palette::face_message(key, palette::FACE, palette::FACE, &px));
                    }
                }
            }
            MediaEvent::VideoGone(key) => {
                self.video_at.remove(key);
                if self.watching.as_ref() == Some(key) {
                    self.unwatch();
                    self.link.send(&json!({"t": "watch_end"}));
                }
                self.link.send(&json!({"t": "video_gone", "key": key}));
            }
            _ => {}
        }
        self.st.apply_media(ev, now);
    }

    fn handle(&mut self, m: Msg) -> Result<(), String> {
        let now = self.now();
        match m {
            Msg::Mc(In::Connected) => self.full_state(),
            Msg::Mc(In::Disconnected) => {
                self.unwatch();
                self.following = false;
                self.our_goal = None;
            }
            Msg::Mc(In::Msg(v)) => self.from_mod(v),
            Msg::App(Event::Net(Net::LoggedOut)) => {
                self.be.finish_logout();
                return Err("logged out".into());
            }
            Msg::App(Event::Net(Net::Chat(crate::chat::ChatEvent::Message(m)))) => {
                if m.user != self.st.me_id {
                    let name = self.st.name_of(&m.user, m.name.as_deref());
                    self.say(&format!("{name}: {}", m.text));
                }
                self.st.apply(Net::Chat(crate::chat::ChatEvent::Message(m)), now);
            }
            Msg::App(Event::Net(Net::Emote { sender, recipient, emote })) => {
                if emote == "wave" && sender != self.st.me_id {
                    self.link.send(&json!({"t": "emote", "from": sender, "to": recipient, "emote": emote}));
                }
                let to_me = emote == "wave" && recipient == self.st.me_id && sender != self.st.me_id;
                let name = self.st.name_of(&sender, None);
                let before = self.st.toast.clone();
                self.st.apply(Net::Emote { sender, recipient, emote }, now);
                // State only toasts a wave from someone with no unanswered wave yet, and Minecraft cannot answer
                // waves, so every later one would pass in silence
                if to_me && self.st.toast == before {
                    self.say(&format!("{name} waved at you"));
                }
            }
            Msg::App(Event::Net(e)) => self.st.apply(e, now),
            Msg::App(Event::Media(e)) => self.media(e, now),
            Msg::App(Event::Talk(a)) => self.st.talk(a, now),
            Msg::App(Event::Login(Ok(auth))) => self.sign_in(auth),
            Msg::App(Event::Login(Err(e))) if session::is_offline(&e) => {
                eprintln!("[{now:6.1}] still offline: {e}");
                self.retry_login_at = Some(now + 5.0);
            }
            Msg::App(Event::Login(Err(e))) => return Err(format!("sign-in failed: {e}")),
        }
        Ok(())
    }
}

pub fn run(args: &[String]) -> Result<(), String> {
    let opt = |n: &str| args.iter().position(|a| a == n).and_then(|i| args.get(i + 1)).cloned();
    let demo = args.iter().any(|a| a == "--mc-demo");
    let dry = demo || args.iter().any(|a| a == "--dry-run");
    let path = opt("--session").unwrap_or_else(session::default_path);
    session::ensure_private_dir(&path);
    let port = opt("--mc-port").and_then(|p| p.parse().ok()).unwrap_or(DEFAULT_PORT);
    let exit_after: Option<f64> = opt("--exit-after").and_then(|s| s.parse().ok());
    let token = ws::token(&path)?;

    let (tx, rx) = channel::<Msg>();
    let emit: Emit = {
        let tx = tx.clone();
        Arc::new(move |e| drop(tx.send(Msg::App(e))))
    };
    let link = {
        let tx = tx.clone();
        ws::serve(port, token, move |m| drop(tx.send(Msg::Mc(m))))?
    };
    println!("soworker --mc: waiting for Minecraft on ws://127.0.0.1:{port} (token in {}){}", ws::token_path(&path).display(), if dry { ", dry run: nothing is sent to SoWork" } else { "" });
    if std::env::var_os("SOWORKER_LOG_STDERR").is_none() {
        println!("log: soworker.log in the state directory (SOWORKER_LOG_STDERR=1 to log here)");
        crate::mode::log_to_file();
    }
    // all video, sized for the map screens (a share is 512 wide on the wall, a camera 256)
    crate::calls::want_video(None);
    crate::calls::set_video_width(640, WALL_WIDTH);

    let be = Backend::new(emit.clone(), dry, path.clone());
    let mut b = Bridge {
        st: State::new("", dry),
        be,
        link,
        dry,
        start: Instant::now(),
        sent: HashMap::new(),
        map: geom::Map::default(),
        world_gen: None,
        last_state: -1.0,
        synced: false,
        our_goal: None,
        following: false,
        teleport_at: 0.0,
        headway: None,
        blocked_goal: None,
        video_at: HashMap::new(),
        dither: !args.iter().any(|a| a == "--mc-no-dither"),
        toast_seen: None,
        retry_login_at: None,
        waved_at: HashMap::new(),
        watching: None,
        watch_at: 0.0,
        exit: false,
    };
    b.st.talk_key = crate::talk::key_label();
    let e = emit.clone();
    crate::talk::listen(move |a| e(Event::Talk(a)));
    let e = emit.clone();
    crate::talk::portal(move |a| e(Event::Talk(a)), b.st.talk_key.clone());
    if demo {
        println!("demo office: no SoWork connection at all");
        demo::setup(&mut b.st);
    } else {
        match Auth::load(&path) {
            Ok(auth) => b.sign_in(auth),
            Err(e) if session::is_offline(&e) => {
                println!("offline ({e}): trying again every 5 s");
                b.retry_login_at = Some(b.now() + 5.0);
            }
            Err(e) => {
                println!("no usable saved sign-in ({e}): opening the browser to sign in");
                b.be.begin_login(&mut b.st);
            }
        }
    }

    let mut last_tick = Instant::now();
    let mut demo_frame = 0.0;
    let mut demo_wave = 0.0;
    loop {
        match rx.recv_timeout(TICK) {
            Ok(m) => {
                b.handle(m)?;
                b.toasts(); // each message's toast, before the next one replaces it
                while let Ok(m) = rx.try_recv() {
                    b.handle(m)?;
                    b.toasts();
                }
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => break,
        }
        if b.exit {
            // the goodbye line has a moment to reach Minecraft, then the client used last opens again
            std::thread::sleep(Duration::from_millis(300));
            return crate::mode::relaunch(crate::mode::back(args), args);
        }
        if exit_after.is_some_and(|x| b.now() > x) {
            break;
        }
        let (now, dt) = (b.now(), last_tick.elapsed().as_secs_f32().min(0.05));
        last_tick = Instant::now();
        if b.retry_login_at.is_some_and(|t| now >= t) {
            b.retry_login_at = None;
            b.st.login_act = Some(crate::app::LoginAct::Retry); // the backend loads the session again on a thread
        }
        if demo {
            demo::tick(&mut b.st, now);
            for ev in demo::media(&mut b.st, now, &mut demo_frame) {
                b.media(ev, now);
            }
            for e in demo::emotes(now, &mut demo_wave) {
                b.handle(Msg::App(Event::Net(e)))?;
            }
        }
        b.st.update(dt, now);
        if b.st.confirm_leave.is_some() {
            // walking out in Minecraft (or the desk tool) is the answer to "leave the meeting?"
            b.st.leave_choice(true, now);
        }
        if b.st.path.is_none() {
            // a walk the player asked for that stopped short of its goal: SoWork's walls are in the way
            if let (Some(g), Some(me)) = (b.our_goal.take(), b.me_pos()) {
                if (g.0 - me.0).hypot(g.1 - me.1) > 12.0 {
                    b.blocked_goal = Some(g);
                }
            }
        }
        b.sync_out(false);
        b.toasts();
        let start = b.start;
        b.be.flush(&mut b.st, start);
    }
    Ok(())
}
