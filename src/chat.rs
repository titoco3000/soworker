//! SoWork's chat (docs/chat-and-waves.md): REST under `api.sowork.com/api/v1/chat-v2` plus a WebSocket at
//! `chat.sowork.com/ws` that pushes new messages. Area chat is the "meeting" channel of the call we are in; private
//! chat is a "direct" channel between two people.
use crate::net::{tls_stream, urlenc};
use crate::proto::Net;
use crate::session::{Auth, CLIENT_VERSION};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::ErrorKind;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tungstenite::{client::IntoClientRequest, Message};

pub type Emit = Arc<dyn Fn(Net) + Send + Sync>;

#[derive(Clone, Debug, PartialEq)]
pub struct ChatMsg {
    pub id: String,
    pub channel: String,
    pub user: String,
    /// The sender's name when the server told us (it sends one with every message and history page).
    pub name: Option<String>,
    pub text: String,
    /// Milliseconds since the Unix epoch.
    pub at: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    /// A private chat; the other person's user id.
    Direct(String),
    /// The chat of a call; the conversation id (the `cluster` event's `conversationId`).
    Meeting(String),
    Other,
}

#[derive(Debug)]
pub enum ChatEvent {
    Channel { id: String, kind: Kind },
    Message(ChatMsg),
    History { channel: String, msgs: Vec<ChatMsg> },
    Error(String),
}

/// What the UI asks the chat thread to do.
#[derive(Debug, Clone)]
pub enum ChatCmd {
    /// The area chat of this conversation was opened: find its channel and load the history.
    Area { conversation: String },
    /// A private chat with this person was opened.
    Dm { peer: String },
    SendArea { conversation: String, text: String },
    SendDm { peer: String, channel: Option<String>, text: String },
    Read { channel: String, message: String },
}

struct Shared {
    auth: Mutex<Auth>,
    emit: Emit,
    group: String,
    me: String,
    channels: Mutex<HashMap<String, Kind>>,
}

impl Shared {
    fn token(&self) -> Result<String, String> {
        self.auth.lock().map_err(|e| e.to_string())?.id_token()
    }

    fn api(&self, method: &str, path: &str, body: Option<Value>) -> Result<Value, String> {
        let url = format!("https://api.sowork.com/api/v1/chat-v2{path}");
        let req = ureq::request(method, &url)
            .set("authorization", &format!("Bearer {}", self.token()?))
            .set("source", "web")
            .set("version", CLIENT_VERSION)
            .timeout(Duration::from_secs(15));
        let res = match body {
            Some(b) => req.send_json(b),
            None => req.call(),
        };
        match res {
            Ok(r) => r.into_json::<Value>().map_err(|e| e.to_string()),
            Err(ureq::Error::Status(code, r)) => Err(format!("chat {method} {path}: HTTP {code} {}", r.into_string().unwrap_or_default().chars().take(160).collect::<String>())),
            Err(e) => Err(format!("chat {method} {path}: {e}")),
        }
    }

    /// Remember a channel and tell the UI what it is.
    fn learn(&self, ch: &Value) -> Option<Kind> {
        let id = ch.get("id")?.as_str()?.to_string();
        let kind = kind_of(ch, &self.me);
        self.channels.lock().ok()?.insert(id.clone(), kind.clone());
        (self.emit)(Net::Chat(ChatEvent::Channel { id, kind: kind.clone() }));
        Some(kind)
    }

    fn known(&self, channel: &str) -> Option<Kind> {
        self.channels.lock().ok()?.get(channel).cloned()
    }

    fn channel_for(&self, kind: &Kind) -> Option<String> {
        self.channels.lock().ok()?.iter().find(|(_, k)| *k == kind).map(|(id, _)| id.clone())
    }

    fn error(&self, e: String) {
        eprintln!("{e}");
        (self.emit)(Net::Chat(ChatEvent::Error(e)));
    }
}

pub fn kind_of(ch: &Value, me: &str) -> Kind {
    match ch.get("type").and_then(Value::as_str) {
        Some("direct") => {
            let peer = ch
                .get("dm_member_user_ids")
                .and_then(Value::as_array)
                .and_then(|a| a.iter().filter_map(Value::as_str).find(|u| *u != me))
                .map(String::from)
                .or_else(|| ch.get("member_key").and_then(Value::as_str).and_then(|k| k.split(':').find(|u| *u != me)).map(String::from));
            peer.map_or(Kind::Other, Kind::Direct)
        }
        Some("meeting") => ch.get("conversation_id").and_then(Value::as_str).map_or(Kind::Other, |c| Kind::Meeting(c.to_string())),
        _ => Kind::Other,
    }
}

/// One message object (REST or WebSocket) -> `ChatMsg`; `users` is the `{id: {name}}` map sent beside it.
pub fn parse_msg(m: &Value, users: Option<&Value>) -> Option<ChatMsg> {
    let user = m.get("user_id").and_then(Value::as_str)?.to_string();
    Some(ChatMsg {
        id: m.get("id")?.as_str()?.to_string(),
        channel: m.get("channel_id")?.as_str()?.to_string(),
        name: users.and_then(|u| u.get(&user)).and_then(|u| u.get("name")).and_then(Value::as_str).map(String::from),
        user,
        text: m.get("text").and_then(Value::as_str).unwrap_or("").to_string(),
        at: m.get("created_at").and_then(Value::as_str).and_then(crate::roster::iso_to_ms).unwrap_or(0),
    })
}

fn doc(text: &str) -> Value {
    json!({"type": "doc", "content": [{"type": "paragraph", "content": [{"type": "text", "text": text}]}]})
}

fn uuid() -> String {
    let mut b = [0u8; 16];
    let _ = getrandom::getrandom(&mut b);
    b[6] = (b[6] & 0x0f) | 0x40;
    b[8] = (b[8] & 0x3f) | 0x80;
    let h: String = b.iter().map(|x| format!("{x:02x}")).collect();
    format!("{}-{}-{}-{}-{}", &h[0..8], &h[8..12], &h[12..16], &h[16..20], &h[20..32])
}

fn history(sh: &Shared, channel: &str) {
    match sh.api("GET", &format!("/{channel}/messages?limit=50"), None) {
        Ok(v) => {
            let users = v.get("users");
            let mut msgs: Vec<ChatMsg> = v.get("messages").and_then(Value::as_array).map(|a| a.iter().filter_map(|m| parse_msg(m, users)).collect()).unwrap_or_default();
            msgs.sort_by_key(|m| m.at);
            (sh.emit)(Net::Chat(ChatEvent::History { channel: channel.to_string(), msgs }));
        }
        Err(e) => sh.error(e),
    }
}

/// A sent message comes back with its channel (which is created by the first message of a chat).
fn after_send(sh: &Shared, r: Value) {
    if let Some(ch) = r.get("channel") {
        sh.learn(ch);
    }
    if let Some(m) = r.get("message").and_then(|m| parse_msg(m, None)) {
        (sh.emit)(Net::Chat(ChatEvent::Message(m)));
    }
}

fn handle(sh: &Shared, cmd: ChatCmd) {
    match cmd {
        ChatCmd::Area { conversation } => {
            let kind = Kind::Meeting(conversation.clone());
            // the official client creates the channel on first use; ask for it so existing history shows
            let ch = match sh.channel_for(&kind) {
                Some(c) => Some(c),
                None => match sh.api("POST", &format!("/meetings/{conversation}/channel"), None) {
                    Ok(v) => v.get("channel").or(Some(&v)).and_then(|c| sh.learn(c).map(|_| c["id"].as_str().unwrap_or("").to_string())).filter(|s| !s.is_empty()),
                    Err(e) => {
                        eprintln!("{e} (no area chat history)");
                        None
                    }
                },
            };
            if let Some(c) = ch {
                history(sh, &c);
            }
        }
        ChatCmd::Dm { peer } => {
            if let Some(c) = sh.channel_for(&Kind::Direct(peer)) {
                history(sh, &c);
            }
        }
        ChatCmd::SendArea { conversation, text } => {
            let body = json!({"content": doc(&text), "idempotencyKey": uuid()});
            match sh.api("POST", &format!("/meetings/{conversation}/messages"), Some(body)) {
                Ok(r) => after_send(sh, r),
                Err(e) => sh.error(e),
            }
        }
        ChatCmd::SendDm { peer, channel, text } => {
            let channel = channel.or_else(|| sh.channel_for(&Kind::Direct(peer.clone())));
            let res = match channel {
                Some(c) => sh.api("POST", &format!("/{c}/messages"), Some(json!({"content": doc(&text), "idempotencyKey": uuid()}))),
                None => sh.api("POST", "/recipients/messages", Some(json!({"groupId": sh.group, "userIds": [peer], "content": doc(&text), "idempotencyKey": uuid()}))),
            };
            match res {
                Ok(r) => after_send(sh, r),
                Err(e) => sh.error(e),
            }
        }
        ChatCmd::Read { channel, message } => {
            if let Err(e) = sh.api("POST", &format!("/{channel}/read"), Some(json!({"lastReadMessageId": message}))) {
                eprintln!("{e}");
            }
        }
    }
}

/// Channel list at start: learn every direct and meeting channel we are already in.
fn list_channels(sh: &Shared) {
    match sh.api("GET", &format!("/?groupId={}", urlenc(&sh.group)), None) {
        Ok(v) => {
            for ch in v.get("channels").and_then(Value::as_array).into_iter().flatten() {
                sh.learn(ch);
            }
        }
        Err(e) => sh.error(e),
    }
}

/// A pushed frame: only new messages matter to us.
fn on_frame(sh: &Arc<Shared>, text: &str) {
    let Ok(v) = serde_json::from_str::<Value>(text) else { return };
    if v["type"] != "message.new" {
        return;
    }
    let Some(msg) = v.pointer("/data/message").and_then(|m| parse_msg(m, v.pointer("/data/users"))) else { return };
    if msg.channel.is_empty() || sh.known(&msg.channel).is_some() {
        return (sh.emit)(Net::Chat(ChatEvent::Message(msg)));
    }
    // a channel we have not seen (someone's first message to us): learn what it is, then deliver
    let sh = sh.clone();
    std::thread::spawn(move || {
        match sh.api("GET", &format!("/{}", msg.channel), None) {
            Ok(v) => {
                sh.learn(v.get("channel").unwrap_or(&v));
            }
            Err(e) => eprintln!("{e}"),
        }
        (sh.emit)(Net::Chat(ChatEvent::Message(msg)));
    });
}

fn socket(sh: &Arc<Shared>, stop: &dyn Fn() -> bool) -> Result<(), String> {
    let url = format!("wss://chat.sowork.com/ws?token={}", urlenc(&sh.token()?));
    let mut req = url.into_client_request().map_err(|e| e.to_string())?;
    req.headers_mut().insert("Origin", "https://app.sowork.com".parse().map_err(|_| "origin")?);
    let (mut ws, _) = tungstenite::client(req, tls_stream("chat.sowork.com")?).map_err(|e| format!("chat websocket: {e}"))?;
    ws.get_mut().sock.set_read_timeout(Some(Duration::from_millis(250))).ok();
    let report = |ws: &mut tungstenite::WebSocket<_>| {
        let m = json!({"type": "activity.report", "data": {"idle_seconds": 0, "locked": false, "notifiable": true}});
        ws.send(Message::Text(m.to_string()))
    };
    let (mut last_report, mut last_frame) = (Instant::now(), Instant::now());
    loop {
        if stop() {
            let _ = ws.close(None);
            return Ok(());
        }
        match ws.read() {
            Ok(Message::Text(t)) => {
                last_frame = Instant::now();
                if t.contains("\"connection.ready\"") {
                    report(&mut ws).map_err(|e| e.to_string())?;
                    last_report = Instant::now();
                } else {
                    on_frame(sh, &t);
                }
            }
            Ok(Message::Ping(p)) => ws.send(Message::Pong(p)).map_err(|e| e.to_string())?,
            Ok(Message::Close(_)) => return Err("chat socket closed by server".into()),
            Ok(_) => {}
            Err(tungstenite::Error::Io(e)) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(e) => return Err(format!("chat socket: {e}")),
        }
        if last_report.elapsed() > Duration::from_secs(60) {
            report(&mut ws).map_err(|e| e.to_string())?;
            last_report = Instant::now();
        }
        if last_frame.elapsed() > Duration::from_secs(100) {
            return Err("chat socket silent (no heartbeat)".into()); // the server beats every 30 s
        }
    }
}

/// Start the chat: a socket thread that reconnects, and a thread that runs commands (each on its own worker).
pub fn spawn(auth: Auth, emit: Emit, cmds: Receiver<ChatCmd>) {
    let sh = Arc::new(Shared {
        group: auth.file.find["groupId"].as_str().unwrap_or("").to_string(),
        me: auth.uid.clone(),
        auth: Mutex::new(auth),
        emit,
        channels: Mutex::new(HashMap::new()),
    });
    if sh.group.is_empty() {
        return;
    }
    let done = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let (s2, d2) = (sh.clone(), done.clone());
    std::thread::Builder::new()
        .name("chat-socket".into())
        .spawn(move || {
            list_channels(&s2);
            while !d2.load(std::sync::atomic::Ordering::Relaxed) {
                if let Err(e) = socket(&s2, &|| d2.load(std::sync::atomic::Ordering::Relaxed)) {
                    eprintln!("{e}");
                }
                for _ in 0..12 {
                    if d2.load(std::sync::atomic::Ordering::Relaxed) {
                        return;
                    }
                    std::thread::sleep(Duration::from_millis(250));
                }
            }
        })
        .expect("chat socket thread");
    std::thread::Builder::new()
        .name("chat-cmds".into())
        .spawn(move || {
            for c in cmds {
                let sh = sh.clone();
                std::thread::spawn(move || handle(&sh, c));
            }
            done.store(true, std::sync::atomic::Ordering::Relaxed); // the window dropped its sender: logged out
        })
        .expect("chat commands thread");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_pushed_message_like_the_recording() {
        let frame = json!({"type": "message.new", "channel_id": "C1", "data": {
            "message": {"id": "M1", "channel_id": "C1", "user_id": "U2", "text": "oi direto", "created_at": "2026-10-07T12:33:32.489Z"},
            "users": {"U2": {"name": "Kury"}}}});
        let m = parse_msg(frame.pointer("/data/message").unwrap(), frame.pointer("/data/users")).unwrap();
        assert_eq!((m.id.as_str(), m.channel.as_str(), m.user.as_str(), m.text.as_str(), m.name.as_deref()), ("M1", "C1", "U2", "oi direto", Some("Kury")));
        assert_eq!(m.at, 1_791_376_412_489);
    }

    #[test]
    fn tells_direct_and_meeting_channels_apart() {
        let dm = json!({"type": "direct", "dm_member_user_ids": ["U2"], "member_key": "U1:U2"});
        assert_eq!(kind_of(&dm, "U1"), Kind::Direct("U2".into()));
        let dm_key_only = json!({"type": "direct", "member_key": "U1:U2"});
        assert_eq!(kind_of(&dm_key_only, "U2"), Kind::Direct("U1".into()));
        assert_eq!(kind_of(&json!({"type": "meeting", "conversation_id": "conv"}), "U1"), Kind::Meeting("conv".into()));
        assert_eq!(kind_of(&json!({"type": "general"}), "U1"), Kind::Other);
    }

    #[test]
    fn message_body_is_a_one_paragraph_doc() {
        assert_eq!(doc("oi"), json!({"type": "doc", "content": [{"type": "paragraph", "content": [{"type": "text", "text": "oi"}]}]}));
        let u = uuid();
        assert_eq!((u.len(), u.as_bytes()[14]), (36, b'4'));
    }
}
