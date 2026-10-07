//! Gaia events: Socket.IO v4 packets in msgpack (docs/gaia-protocol.md), decoded into app-level `Net` events.
//! Shared by the live socket and by the log replayer, so both exercise the same code.
use crate::mapobj::{decode_set_map_object, SetObj};
use rmpv::Value;

#[derive(Clone, Debug, Default)]
pub struct User {
    pub id: String,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub state: String,
    /// `uploadedPhotoURL` (an S3 profile photo), if the user set one.
    /// Presence status (`online`, `busy`, ...) and cluster mode (`cluster`, `none-away`, ...): together they say who is away.
    pub status: String,
    pub mode: String,
    pub photo: Option<String>,
    /// SoWork's compact character string (`characterA!body:~v1+typeB!...`), used to build the portrait URL.
    pub portrait: Option<String>,
    /// `meetingZoneId`: the meeting zone the user stands in.
    pub zone: Option<String>,
}

#[derive(Clone, Debug)]
pub struct LkToken {
    pub token: String,
    pub api_host: String,
    pub room_id: String,
}

#[derive(Clone, Debug)]
pub struct ClusterInfo {
    pub id: String,
    pub conversation_id: String,
    pub users: Vec<(String, String)>,
    pub token: Option<LkToken>,
    pub unsupported: Option<String>,
}

#[derive(Debug)]
pub enum Net {
    Connected,
    Disconnected,
    Error(String),
    Room { name: String, spawn: Option<(f32, f32)>, users: Vec<User> },
    Object(SetObj),
    Move(String, f32, f32),
    State(String, String),
    Join(User),
    Leave(String),
    Speaking(String, bool),
    Cluster(Option<ClusterInfo>),
    Presence { id: String, status: String },
    Mode { id: String, mode: String },
    /// A user entered (`Some`) or left (`None`) a meeting zone (`s` code 9).
    Zone { id: String, zone: Option<String> },
    /// The office's member list (online and offline), from Firestore.
    Roster(Vec<crate::roster::Member>),
    /// The socket thread closed the connection after a logout request.
    LoggedOut,
    /// Profile update for a user (name, photo, avatar config).
    UserInfo { id: String, name: Option<String>, photo: Option<Option<String>>, portrait: Option<String> },
    /// A decoded, circle-masked, premultiplied RGBA avatar picture.
    Avatar { id: String, size: u32, rgba: Vec<u8> },
}

/// Commands from the UI to the socket thread.
#[derive(Debug, Clone)]
pub enum Cmd {
    Move(f32, f32),
    State(&'static str),
    Zone(Option<String>),
    Available(bool),
    Camera(bool),
    Speech(bool),
    /// Tell SoWork our status over REST (`true` = away/busy, `false` = online); the gaia `cluster_mode` goes separately.
    Presence { away: bool },
    /// Close the socket cleanly and stop reconnecting.
    Logout,
}

fn get<'a>(v: &'a Value, k: &str) -> Option<&'a Value> {
    v.as_map()?.iter().find(|(kk, _)| kk.as_str() == Some(k)).map(|(_, v)| v)
}
/// msgpack `undefined` arrives as fixext type 0; treat it like nil.
fn present(v: Option<&Value>) -> Option<&Value> {
    v.filter(|v| !matches!(v, Value::Nil | Value::Ext(..)))
}
fn text(v: Option<&Value>) -> String {
    present(v).and_then(Value::as_str).unwrap_or("").to_string()
}
fn num(v: Option<&Value>) -> f32 {
    present(v).and_then(Value::as_f64).unwrap_or(0.0) as f32
}

/// `characterConfig` -> SoWork's compact string: slots sorted, each `slot:~item+variant` with the
/// `<bodyType>_<slot>_` prefix replaced by `~`. Verified against the portrait URLs the official client requests.
pub fn portrait_string(cfg: &Value) -> Option<String> {
    let body = present(get(cfg, "bodyType"))?.as_str()?;
    let slots = present(get(cfg, "slots"))?.as_map()?;
    let mut parts: Vec<(String, String)> = slots
        .iter()
        .filter_map(|(k, v)| {
            let (slot, val) = (k.as_str()?, v.as_str()?);
            let prefix = format!("{body}_{slot}_");
            Some((slot.to_string(), val.strip_prefix(&prefix).map_or(val.to_string(), |r| format!("~{r}"))))
        })
        .collect();
    parts.sort();
    Some(format!("{body}!{}", parts.iter().map(|(s, v)| format!("{s}:{v}")).collect::<Vec<_>>().join("!")))
}

fn user(v: &Value) -> User {
    User {
        id: text(get(v, "id")),
        name: text(get(v, "name")),
        x: num(get(v, "x")),
        y: num(get(v, "y")),
        state: present(get(v, "characterState"))
            .and_then(|c| get(c, "state"))
            .and_then(Value::as_str)
            .unwrap_or("idle")
            .to_string(),
        status: present(get(v, "presence")).and_then(|x| get(x, "status")).and_then(Value::as_str).unwrap_or("online").to_string(),
        mode: present(get(v, "clusterMode")).and_then(Value::as_str).unwrap_or("cluster").to_string(),
        photo: present(get(v, "uploadedPhotoURL")).and_then(Value::as_str).filter(|s| !s.is_empty()).map(String::from),
        portrait: present(get(v, "characterConfig")).and_then(portrait_string),
        zone: present(get(v, "meetingZoneId")).and_then(Value::as_str).filter(|s| !s.is_empty()).map(String::from),
    }
}

/// Parse one binary Socket.IO packet. Returns (packet type, event name + payload for EVENT packets).
pub fn parse_packet(bytes: &[u8]) -> Option<(i64, Option<(String, Value)>)> {
    let v = rmpv::decode::read_value(&mut &bytes[..]).ok()?;
    get(&v, "nsp")?; // rejects LiveKit protobuf frames in replays
    let ty = get(&v, "type")?.as_i64()?;
    let ev = (ty == 2)
        .then(|| {
            let d = get(&v, "data")?.as_array()?;
            Some((d.first()?.as_str()?.to_string(), d.get(1).cloned().unwrap_or(Value::Nil)))
        })
        .flatten();
    Some((ty, ev))
}

pub fn events(name: &str, p: &Value) -> Vec<Net> {
    match name {
        "b_set_obj" => match p {
            Value::Binary(b) => decode_set_map_object(b).map(|o| vec![Net::Object(o)]).unwrap_or_default(),
            _ => vec![],
        },
        "room" => {
            let spawn = present(get(p, "spawns"))
                .and_then(Value::as_array)
                .and_then(|a| a.first())
                .map(|s| (num(get(s, "x")), num(get(s, "y"))));
            let users = present(get(p, "users"))
                .and_then(Value::as_map)
                .map(|m| m.iter().map(|(_, u)| user(u)).collect())
                .unwrap_or_default();
            vec![Net::Room { name: text(get(p, "name")), spawn, users }]
        }
        "s" => p
            .as_array()
            .map(|batch| {
                batch
                    .iter()
                    .filter_map(|item| {
                        let a = item.as_array()?;
                        let (code, d) = (a.first()?.as_i64()?, a.get(1)?);
                        let id = text(get(d, "userId"));
                        Some(match code {
                            1 => Net::Move(id, num(get(d, "x")), num(get(d, "y"))),
                            2 => Net::State(
                                id,
                                present(get(d, "characterState"))
                                    .and_then(|c| get(c, "state"))
                                    .and_then(Value::as_str)
                                    .unwrap_or("idle")
                                    .into(),
                            ),
                            3 => Net::Join(user(present(get(d, "user"))?)),
                            9 => Net::Zone { id, zone: present(get(d, "meetingZoneId")).and_then(Value::as_str).filter(|s| !s.is_empty()).map(String::from) },
                            10 => Net::Mode { id, mode: text(get(d, "mode")) },
                            6 => Net::Speaking(id, present(get(d, "isSpeaking")).and_then(Value::as_bool).unwrap_or(false)),
                            _ => return None,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default(),
        "presence" => vec![Net::Presence {
            id: text(get(p, "userId")),
            status: present(get(p, "presence")).and_then(|x| get(x, "status")).and_then(Value::as_str).unwrap_or("online").to_string(),
        }],
        "update_info" => vec![Net::UserInfo {
            id: text(get(p, "userId")),
            name: present(get(p, "name")).and_then(Value::as_str).map(String::from),
            photo: Some(present(get(p, "uploadedPhotoURL")).and_then(Value::as_str).filter(|s| !s.is_empty()).map(String::from)),
            portrait: None,
        }],
        "update" => vec![Net::UserInfo {
            id: text(get(p, "userId")),
            name: None,
            photo: None,
            portrait: present(get(p, "characterConfig")).and_then(portrait_string),
        }],
        "user_disconnect" | "leave" => vec![Net::Leave(text(get(p, "userId")))],
        "cluster" => {
            let (cid, conv) = (text(get(p, "clusterId")), text(get(p, "conversationId")));
            if cid.is_empty() || conv.is_empty() {
                return vec![Net::Cluster(None)];
            }
            let users = present(get(p, "users"))
                .and_then(Value::as_array)
                .map(|a| a.iter().map(|u| (text(get(u, "id")), text(get(u, "name")))).collect())
                .unwrap_or_default();
            let t = present(get(p, "token"));
            let provider = t.map(|t| text(get(t, "provider")));
            let token = t.filter(|_| provider.as_deref() == Some("livekit")).map(|t| LkToken {
                token: text(get(t, "token")),
                api_host: text(get(t, "apiHost")),
                room_id: text(get(t, "roomId")),
            });
            let unsupported = provider.filter(|p| p != "livekit");
            vec![Net::Cluster(Some(ClusterInfo { id: cid, conversation_id: conv, users, token, unsupported }))]
        }
        _ => vec![],
    }
}

// ---- outgoing packets

fn map(pairs: Vec<(&str, Value)>) -> Value {
    Value::Map(pairs.into_iter().map(|(k, v)| (Value::from(k), v)).collect())
}

pub fn packet(p: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    rmpv::encode::write_value(&mut out, p).expect("msgpack encode");
    out
}

pub fn connect_packet(token: &str) -> Vec<u8> {
    packet(&map(vec![("type", 0.into()), ("data", map(vec![("token", token.into())])), ("nsp", "/".into())]))
}

pub fn event_packet(name: &str, payload: Option<Value>, ack: Option<i64>) -> Vec<u8> {
    let data = Value::Array(match payload {
        Some(p) => vec![name.into(), p],
        None => vec![name.into()],
    });
    let mut pairs = vec![("type", 2.into()), ("data", data), ("options", map(vec![("compress", true.into())]))];
    if let Some(id) = ack {
        pairs.push(("id", id.into()));
    }
    pairs.push(("nsp", "/".into()));
    packet(&map(pairs))
}

pub fn cmd_packet(c: &Cmd) -> Vec<u8> {
    match c {
        Cmd::Move(x, y) => event_packet("move", Some(map(vec![("x", Value::F64(*x as f64)), ("y", Value::F64(*y as f64))])), None),
        Cmd::State(s) => event_packet("character_state", Some(map(vec![("characterState", map(vec![("state", (*s).into())]))])), None),
        Cmd::Zone(z) => event_packet(
            "set_meeting_zone",
            Some(map(vec![("zoneId", z.as_deref().map_or(Value::Nil, Value::from))])),
            None,
        ),
        Cmd::Available(a) => event_packet(
            "cluster_mode",
            Some(map(vec![("mode", (if *a { "cluster" } else { "none-away" }).into())])),
            None,
        ),
        Cmd::Camera(on) => event_packet("camera_on", Some(map(vec![("cameraOn", (*on).into())])), None),
        Cmd::Speech(on) => event_packet("speech", Some(map(vec![("isSpeaking", (*on).into())])), None),
        Cmd::Presence { .. } | Cmd::Logout => Vec::new(), // handled by the socket thread, never sent as a packet
    }
}

pub fn init_packets() -> Vec<Vec<u8>> {
    vec![
        event_packet("should_update", Some(map(vec![("v", true.into()), ("t", 0.into()), ("lastObjectId", "".into())])), None),
        event_packet("active_app", Some(map(vec![("activeApp", Value::Nil)])), None),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Rebuild portrait strings from the recording's character configs and compare them with the portrait URLs
    /// the official client requested in the same run. Skipped when the recording is not on this machine.
    #[test]
    fn portrait_strings_match_the_official_clients_urls() {
        let run = std::env::var("SOWORKER_RUN").unwrap_or_else(|_| "../recorder/logs/2026-10-06T12-48-44-206Z".into());
        let Ok(text) = std::fs::read_to_string(format!("{run}/events.jsonl")) else { return };
        let mut wanted = std::collections::HashSet::new();
        let mut made = std::collections::HashSet::new();
        for line in text.lines() {
            let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else { continue };
            if v["k"] == "req" {
                if let Some(rest) = v["url"].as_str().and_then(|u| u.split("/portrait/v1/meeting!idle/").nth(1)) {
                    wanted.insert(rest.to_string());
                }
            }
            if v["k"] == "ws.in" && v["op"] == 2 {
                use base64::Engine;
                let Some(b) = v["data"].as_str().and_then(|d| base64::engine::general_purpose::STANDARD.decode(d).ok()) else { continue };
                if let Some((2, Some((name, p)))) = parse_packet(&b) {
                    if name == "room" {
                        for u in present(get(&p, "users")).and_then(Value::as_map).into_iter().flatten() {
                            if let Some(s) = present(get(&u.1, "characterConfig")).and_then(portrait_string) {
                                made.insert(s);
                            }
                        }
                    }
                }
            }
        }
        let hits = wanted.iter().filter(|w| made.contains(*w)).count();
        println!("official client requested {} idle portraits; {} of them rebuilt exactly", wanted.len(), hits);
        assert!(!wanted.is_empty() && hits > 0, "no portrait string matched");
        assert!(made.iter().any(|m| wanted.contains(m)));
    }
}
