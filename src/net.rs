//! The gaia socket on its own thread: Engine.IO/Socket.IO over a TLS WebSocket, msgpack packets.
use crate::proto::{cmd_packet, connect_packet, emote_packet, event_packet, events, init_packets, parse_packet, Cmd, Net};
use crate::session::{Auth, CLIENT_VERSION};
use std::io::ErrorKind;
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tungstenite::{client::IntoClientRequest, Message};

type Tls = rustls::StreamOwned<rustls::ClientConnection, TcpStream>;

pub fn tls_stream(host: &str) -> Result<Tls, String> {
    let roots = rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
    let cfg = rustls::ClientConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
        .with_safe_default_protocol_versions()
        .map_err(|e| e.to_string())?
        .with_root_certificates(roots)
        .with_no_client_auth();
    let name = rustls::pki_types::ServerName::try_from(host.to_string()).map_err(|e| e.to_string())?;
    let conn = rustls::ClientConnection::new(Arc::new(cfg), name).map_err(|e| e.to_string())?;
    let tcp = TcpStream::connect((host, 443)).map_err(|e| format!("connect {host}: {e}"))?;
    tcp.set_nodelay(true).ok();
    Ok(rustls::StreamOwned::new(conn, tcp))
}

fn random_id() -> String {
    use base64::Engine;
    let mut b = [0u8; 16];
    let _ = getrandom::getrandom(&mut b);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(b)[..21].to_string()
}

struct Link {
    room_id: String,
    auto_force_id: String,
    first: bool,
    pos: Option<(f32, f32)>,
    zone: Option<String>,
    available: bool,
    logout: bool,
    /// Member list polling: the group, when we last asked, and where the answers arrive.
    group_id: String,
    roster_at: Option<Instant>,
    roster_tx: Sender<Net>,
    roster_rx: Receiver<Net>,
}

const ROSTER_EVERY: Duration = Duration::from_secs(30);

fn session(auth: &mut Auth, link: &mut Link, emit: &dyn Fn(Net), cmds: &Receiver<Cmd>, dry: bool) -> Result<(), String> {
    let token = auth.id_token()?;
    let mut query = vec![
        ("roomId", link.room_id.clone()),
        ("mediaRegion", "eu-central-1".into()),
        ("password", String::new()),
        ("clientVersion", CLIENT_VERSION.into()),
        ("autoForceId", link.auto_force_id.clone()),
        ("clusterMode", if link.available { "cluster" } else { "none-away" }.into()),
        ("meetingZoneId", link.zone.clone().unwrap_or_default()),
    ];
    if link.first {
        query.push(("joinType", "force".into())); // takes over any other session of this account
    }
    if let Some((x, y)) = link.pos {
        query.push(("x", x.to_string()));
        query.push(("y", y.to_string()));
    }
    query.push(("EIO", "4".into()));
    query.push(("transport", "websocket".into()));
    let qs: Vec<String> = query.iter().map(|(k, v)| format!("{k}={}", urlenc(v))).collect();
    let url = format!("wss://api.sowork.com/gaia/g5/v2/?{}", qs.join("&"));
    let reconnecting = !link.first;
    link.first = false;

    let req = url.into_client_request().map_err(|e| e.to_string())?;
    let (mut ws, _) = tungstenite::client(req, tls_stream("api.sowork.com")?).map_err(|e| format!("websocket: {e}"))?;
    ws.get_mut().sock.set_read_timeout(Some(Duration::from_millis(25))).ok();

    loop {
        while let Ok(e) = link.roster_rx.try_recv() {
            emit(e);
        }
        if !link.group_id.is_empty() && link.roster_at.map_or(true, |t| t.elapsed() >= ROSTER_EVERY) {
            link.roster_at = Some(Instant::now());
            if let Ok(token) = auth.id_token() {
                crate::roster::fetch_async(token.clone(), link.group_id.clone(), link.roster_tx.clone());
                crate::waves::pending_async(token, auth.uid.clone(), link.roster_tx.clone());
            }
        }
        match ws.read() {
            Ok(Message::Text(t)) => {
                if t == "2" {
                    ws.send(Message::Text("3".into())).map_err(|e| e.to_string())?; // pong
                } else if t.starts_with('0') {
                    ws.send(Message::Binary(connect_packet(&token))).map_err(|e| e.to_string())?;
                }
            }
            Ok(Message::Binary(b)) => {
                let Some((ty, ev)) = parse_packet(&b) else { continue };
                match (ty, ev) {
                    (0, _) => {
                        emit(Net::Connected);
                        if reconnecting {
                            let p = rmpv::Value::Map(vec![
                                ("clusterMode".into(), if link.available { "cluster" } else { "none-away" }.into()),
                                ("zoneId".into(), link.zone.as_deref().map_or(rmpv::Value::Nil, rmpv::Value::from)),
                                ("shouldUpdate".into(), true.into()),
                            ]);
                            ws.send(Message::Binary(event_packet("user_reconnected", Some(p), None))).map_err(|e| e.to_string())?;
                        }
                        let t = rmpv::Value::Map(vec![("clientTime".into(), (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)).into())]);
                        ws.send(Message::Binary(event_packet("sync_time", Some(t), Some(0)))).map_err(|e| e.to_string())?;
                    }
                    (4, _) => return Err("gaia rejected the connection (token or version)".into()),
                    (2, Some((name, payload))) => {
                        if name == "init" {
                            for p in init_packets() {
                                ws.send(Message::Binary(p)).map_err(|e| e.to_string())?;
                            }
                            continue;
                        }
                        for e in events(&name, &payload) {
                            if let (Net::Room { users, .. }, true) = (&e, !dry) {
                                // be available for calls once we are in the room; remember our server-side position
                                if let Some(me) = users.iter().find(|u| u.id == auth.uid) {
                                    link.pos.get_or_insert((me.x, me.y));
                                }
                                ws.send(Message::Binary(cmd_packet(&Cmd::Available(link.available)))).map_err(|e| e.to_string())?;
                            }
                            if let Net::Cluster(Some(c)) = &e {
                                if c.token.is_none() && c.unsupported.is_none() {
                                    // a busy meeting comes without a token: ask for it like the official client does
                                    if let Ok(t) = auth.id_token() {
                                        let (conv, tx) = (c.conversation_id.clone(), link.roster_tx.clone());
                                        std::thread::spawn(move || fetch_call_token(&t, &conv, &tx));
                                    }
                                }
                            }
                            emit(e);
                        }
                    }
                    _ => {}
                }
            }
            Ok(Message::Ping(p)) => {
                ws.send(Message::Pong(p)).map_err(|e| e.to_string())?;
            }
            Ok(Message::Close(_)) => return Err("closed by server".into()),
            Ok(_) => {}
            Err(tungstenite::Error::Io(e)) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(e) => return Err(format!("socket: {e}")),
        }
        while let Ok(c) = cmds.try_recv() {
            if matches!(c, Cmd::Logout) {
                // say goodbye properly so we drop out of the room for everyone else, then stop for good
                link.logout = true;
                let _ = ws.close(None);
                for _ in 0..20 {
                    if ws.read().is_err_and(|e| !matches!(&e, tungstenite::Error::Io(io) if matches!(io.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut))) {
                        break;
                    }
                }
                return Ok(());
            }
            match &c {
                Cmd::Wave(to) => {
                    if !dry {
                        if let Ok(token) = auth.id_token() {
                            let to = to.clone();
                            std::thread::spawn(move || crate::waves::send(&token, &to));
                        }
                        ws.send(Message::Binary(emote_packet(&auth.uid, to))).map_err(|e| e.to_string())?;
                    }
                    continue;
                }
                Cmd::WaveDone(from) => {
                    if !dry {
                        if let Ok(token) = auth.id_token() {
                            let (uid, from) = (auth.uid.clone(), from.clone());
                            std::thread::spawn(move || crate::waves::clear(&token, &uid, &from));
                        }
                    }
                    continue;
                }
                _ => {}
            }
            if let Cmd::Presence { away } = c {
                if !dry {
                    if let Ok(token) = auth.id_token() {
                        let (uid, away) = (auth.uid.clone(), away);
                        std::thread::spawn(move || rest_presence(&token, &uid, away)); // keep the socket loop responsive
                    }
                }
                continue;
            }
            match &c {
                Cmd::Move(x, y) => link.pos = Some((*x, *y)),
                Cmd::Zone(z) => link.zone = z.clone(),
                Cmd::Available(a) => link.available = *a,
                _ => {}
            }
            if !dry {
                ws.send(Message::Binary(cmd_packet(&c))).map_err(|e| e.to_string())?;
            }
        }
    }
}

/// `POST /api/v1/conversation/token {conversationId}`: the fallback the official client uses when the `cluster` event
/// carries no token. The answer's format was never captured, so it is read loosely: an object with a `token` string
/// (maybe nested one level), `apiHost` and `roomId` beside it. Only the field names are logged, never values.
fn fetch_call_token(id_token: &str, conversation_id: &str, tx: &Sender<Net>) {
    let res = ureq::post("https://api.sowork.com/api/v1/conversation/token")
        .set("authorization", &format!("Bearer {id_token}"))
        .set("source", "web")
        .set("version", CLIENT_VERSION)
        .timeout(Duration::from_secs(15))
        .send_json(serde_json::json!({"conversationId": conversation_id}));
    let body: serde_json::Value = match res {
        Ok(r) => r.into_json().unwrap_or_default(),
        Err(ureq::Error::Status(code, r)) => {
            let msg = format!("call token: HTTP {code} {}", r.into_string().unwrap_or_default().chars().take(160).collect::<String>());
            eprintln!("{msg}");
            return drop(tx.send(Net::Error(msg)));
        }
        Err(e) => {
            eprintln!("call token: {e}");
            return drop(tx.send(Net::Error(format!("call token: {e}"))));
        }
    };
    match parse_call_token(&body) {
        Some(token) => drop(tx.send(Net::ClusterToken { conversation_id: conversation_id.into(), token })),
        None => {
            let keys = |v: &serde_json::Value| v.as_object().map(|m| m.keys().cloned().collect::<Vec<_>>().join(",")).unwrap_or_default();
            eprintln!("call token: no usable token in the answer (fields: {})", keys(&body));
            let _ = tx.send(Net::Error("could not get the call token for this meeting".into()));
        }
    }
}

fn parse_call_token(body: &serde_json::Value) -> Option<crate::proto::LkToken> {
    let obj = [body, &body["data"], &body["token"], &body["result"]].into_iter().find(|v| v["token"].as_str().is_some_and(|t| !t.is_empty()) || v["accessToken"].as_str().is_some())?;
    if obj["provider"].as_str().is_some_and(|p| p != "livekit") {
        return None;
    }
    let get = |names: &[&str]| names.iter().find_map(|n| obj[*n].as_str()).unwrap_or("").to_string();
    let host = get(&["apiHost", "api_host", "host"]);
    Some(crate::proto::LkToken {
        token: get(&["token", "accessToken"]),
        api_host: if host.is_empty() { "sowork-production-7a7l3zs5.livekit.cloud".into() } else { host },
        room_id: get(&["roomId", "room_id", "room"]),
    })
}

/// SoWork's own sequence for a status change (observed in the recording): for "away" first cancel any scheduled
/// status clear and store "never clear", then set the status; "available" is just the status update.
fn rest_presence(token: &str, uid: &str, away: bool) {
    let post = |path: &str, body: serde_json::Value| {
        let r = ureq::post(&format!("https://api.sowork.com/api/v1/{path}"))
            .set("authorization", &format!("Bearer {token}"))
            .set("source", "web")
            .set("version", CLIENT_VERSION)
            .send_json(body);
        if let Err(e) = r {
            eprintln!("presence: {path} failed: {e}");
        }
    };
    if away {
        post("user-presence/scheduleClear", serde_json::json!({"type": "cancel"}));
        post("user/updateUiSettings", serde_json::json!({"userId": uid, "data": {"presenceStatus": {"clearAfter": "never"}}}));
        post("user-presence/update", serde_json::json!({"status": "busy"}));
    } else {
        post("user-presence/update", serde_json::json!({"status": "online"}));
    }
}

pub fn urlenc(s: &str) -> String {
    s.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
            _ => format!("%{b:02X}"),
        })
        .collect()
}

/// Runs forever: connect, serve, and reconnect after a drop (the server resends the whole room on reconnect).
pub fn run(mut auth: Auth, dry: bool, emit: impl Fn(Net) + Send + 'static, cmds: Receiver<Cmd>) {
    let room_id = loop {
        match auth.find_room() {
            Ok(id) => break id,
            Err(e) => {
                emit(Net::Error(e));
                std::thread::sleep(Duration::from_secs(5));
            }
        }
    };
    let (roster_tx, roster_rx) = channel();
    let group_id = auth.file.find["groupId"].as_str().unwrap_or("").to_string();
    let mut link = Link { room_id, auto_force_id: random_id(), first: true, pos: None, zone: None, available: true, logout: false, group_id, roster_at: None, roster_tx, roster_rx };
    loop {
        let res = session(&mut auth, &mut link, &emit, &cmds, dry);
        if link.logout {
            emit(Net::LoggedOut);
            return;
        }
        if let Err(e) = res {
            emit(Net::Disconnected);
            emit(Net::Error(e));
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}

#[cfg(test)]
mod token_tests {
    use super::*;

    #[test]
    fn reads_the_token_flat_or_nested() {
        let flat = serde_json::json!({"provider": "livekit", "token": "jwt", "apiHost": "h.livekit.cloud", "roomId": "RM_1"});
        let t = parse_call_token(&flat).unwrap();
        assert_eq!((t.token.as_str(), t.api_host.as_str(), t.room_id.as_str()), ("jwt", "h.livekit.cloud", "RM_1"));
        let nested = serde_json::json!({"data": {"token": "jwt2", "roomId": "RM_2"}});
        assert_eq!(parse_call_token(&nested).unwrap().token, "jwt2");
        assert!(parse_call_token(&serde_json::json!({"provider": "chime", "token": "x"})).is_none());
        assert!(parse_call_token(&serde_json::json!({})).is_none());
    }
}
