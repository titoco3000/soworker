//! The gaia socket on its own thread: Engine.IO/Socket.IO over a TLS WebSocket, msgpack packets.
use crate::proto::{cmd_packet, connect_packet, event_packet, events, init_packets, parse_packet, Cmd, Net};
use crate::session::{Auth, CLIENT_VERSION};
use std::io::ErrorKind;
use std::net::TcpStream;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::time::Duration;
use tungstenite::{client::IntoClientRequest, Message};

type Tls = rustls::StreamOwned<rustls::ClientConnection, TcpStream>;

fn tls_stream(host: &str) -> Result<Tls, String> {
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
    if let Ok(mut f) = std::fs::File::open("/dev/urandom") {
        use std::io::Read;
        let _ = f.read_exact(&mut b);
    }
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(b)[..21].to_string()
}

struct Link {
    room_id: String,
    auto_force_id: String,
    first: bool,
    pos: Option<(f32, f32)>,
    zone: Option<String>,
    available: bool,
}

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

fn urlenc(s: &str) -> String {
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
    let mut link = Link { room_id, auto_force_id: random_id(), first: true, pos: None, zone: None, available: true };
    loop {
        if let Err(e) = session(&mut auth, &mut link, &emit, &cmds, dry) {
            emit(Net::Disconnected);
            emit(Net::Error(e));
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}
