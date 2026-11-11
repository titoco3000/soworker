//! The WebSocket server the Minecraft mod connects to: localhost only, on a port the system picks, one client at a time,
//! and the first message must carry the token the mod handed us in `SOWORKER_MC_TOKEN`. Each connection runs on its own thread that both
//! writes what is queued and reads what the mod sends; incoming JSON goes to the main loop through `emit`.
//!
//! Outgoing: JSON text messages in order, and video frames that are replaced by a newer frame of the same screen while
//! still queued, so a slow socket drops frames instead of falling behind.
use serde_json::Value;
use std::collections::VecDeque;
use std::io::ErrorKind;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tungstenite::Message;

/// Text messages kept for a client that does not read; past this the oldest go (it gets the full state on reconnect).
const MAX_TEXT: usize = 2000;

/// What the connection threads report to the main loop.
#[derive(Debug)]
pub enum In {
    /// A client signed in with the token (any earlier one is dropped): send it the full state.
    Connected,
    Disconnected,
    Msg(Value),
}

#[derive(Default)]
struct Queue {
    /// The connection whose messages are queued (0 = none).
    conn: u64,
    next: u64,
    text: VecDeque<String>,
    video: Vec<(String, Vec<u8>)>,
}

#[derive(Clone, Default)]
pub struct Link(Arc<Mutex<Queue>>);

impl Link {
    pub fn connected(&self) -> bool {
        self.0.lock().is_ok_and(|q| q.conn != 0)
    }

    /// Queue a JSON message for the client, if there is one.
    pub fn send(&self, v: &Value) {
        let Ok(mut q) = self.0.lock() else { return };
        if q.conn == 0 {
            return;
        }
        if q.text.len() >= MAX_TEXT {
            q.text.pop_front();
        }
        q.text.push_back(v.to_string());
    }

    /// Queue a video frame, replacing one of the same screen that has not gone out yet.
    pub fn video(&self, key: &str, bytes: Vec<u8>) {
        let Ok(mut q) = self.0.lock() else { return };
        if q.conn == 0 {
            return;
        }
        match q.video.iter_mut().find(|(k, _)| k == key) {
            Some(slot) => slot.1 = bytes,
            None => q.video.push((key.to_string(), bytes)),
        }
    }

    /// Text first (state), then one video frame per call so state messages are never stuck behind a burst of frames.
    fn take(&self, conn: u64) -> Option<(Vec<String>, Option<Vec<u8>>)> {
        let mut q = self.0.lock().ok()?;
        if q.conn != conn {
            return None;
        }
        let text = q.text.drain(..).collect();
        let video = (!q.video.is_empty()).then(|| q.video.remove(0).1);
        Some((text, video))
    }
}

/// Listen on 127.0.0.1:`port` (0: any free port) and serve clients on their own threads. Returns the port.
pub fn serve(port: u16, token: String, emit: impl Fn(In) + Send + Sync + 'static) -> Result<(Link, u16), String> {
    let l = TcpListener::bind(("127.0.0.1", port)).map_err(|e| format!("cannot listen on 127.0.0.1:{port}: {e}"))?;
    let port = l.local_addr().map_err(|e| e.to_string())?.port();
    let link = Link::default();
    let (lk, emit) = (link.clone(), Arc::new(emit));
    std::thread::spawn(move || {
        for s in l.incoming().flatten() {
            let (lk, emit, token) = (lk.clone(), emit.clone(), token.clone());
            std::thread::spawn(move || {
                if let Err(e) = client(s, &lk, &token, &*emit) {
                    eprintln!("mc: client: {e}");
                }
            });
        }
    });
    Ok((link, port))
}

fn would_block(e: &tungstenite::Error) -> bool {
    matches!(e, tungstenite::Error::Io(io) if matches!(io.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut))
}

/// Does this first message carry the right token?
pub fn auth_ok(msg: &str, token: &str) -> bool {
    let v: Value = serde_json::from_str(msg).unwrap_or_default();
    let got = v["token"].as_str().unwrap_or("");
    // compare every byte, so the time taken says nothing about how much matched
    v["t"] == "auth" && got.len() == token.len() && got.bytes().zip(token.bytes()).fold(0u8, |a, (x, y)| a | (x ^ y)) == 0
}

fn client(s: TcpStream, link: &Link, token: &str, emit: &dyn Fn(In)) -> Result<(), String> {
    let peer = s.peer_addr().map_or_else(|_| "?".into(), |a| a.to_string());
    s.set_read_timeout(Some(Duration::from_secs(5))).map_err(|e| e.to_string())?;
    let _ = s.set_nodelay(true);
    let mut ws = tungstenite::accept(s).map_err(|e| format!("handshake from {peer}: {e}"))?;
    let first = ws.read().map_err(|e| format!("no auth from {peer}: {e}"))?;
    if !matches!(&first, Message::Text(t) if auth_ok(t, token)) {
        let _ = ws.close(None);
        let _ = ws.flush();
        return Err(format!("{peer}: bad or missing token, closed"));
    }
    let conn = {
        let mut q = link.0.lock().map_err(|e| e.to_string())?;
        q.next += 1;
        q.conn = q.next;
        q.text.clear();
        q.video.clear();
        q.conn
    };
    eprintln!("mc: client {peer} signed in (connection {conn})");
    emit(In::Connected);
    ws.get_ref().set_read_timeout(Some(Duration::from_millis(15))).map_err(|e| e.to_string())?;
    let res = (|| -> Result<(), String> {
        loop {
            let Some((text, video)) = link.take(conn) else {
                let _ = ws.close(None);
                let _ = ws.flush();
                return Err("replaced by a newer client".into());
            };
            let wrote = !text.is_empty() || video.is_some();
            for t in text {
                ws.write(Message::Text(t)).map_err(|e| e.to_string())?;
            }
            if let Some(v) = video {
                ws.write(Message::Binary(v)).map_err(|e| e.to_string())?;
            }
            if wrote {
                ws.flush().map_err(|e| e.to_string())?;
            }
            match ws.read() {
                Ok(Message::Text(t)) => match serde_json::from_str::<Value>(&t) {
                    Ok(v) => emit(In::Msg(v)),
                    Err(e) => eprintln!("mc: bad message from the mod: {e}"),
                },
                Ok(Message::Close(_)) => return Ok(()),
                Ok(_) => {}
                Err(e) if would_block(&e) => {}
                Err(tungstenite::Error::ConnectionClosed) => return Ok(()),
                Err(e) => return Err(e.to_string()),
            }
        }
    })();
    let mine = link.0.lock().is_ok_and(|mut q| {
        let mine = q.conn == conn;
        if mine {
            q.conn = 0;
            q.text.clear();
            q.video.clear();
        }
        mine
    });
    if mine {
        eprintln!("mc: client {peer} gone");
        emit(In::Disconnected);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_needs_the_exact_token() {
        assert!(auth_ok(r#"{"t":"auth","token":"abc"}"#, "abc"));
        assert!(!auth_ok(r#"{"t":"auth","token":"abd"}"#, "abc"));
        assert!(!auth_ok(r#"{"t":"auth","token":"ab"}"#, "abc"));
        assert!(!auth_ok(r#"{"t":"pos","token":"abc"}"#, "abc"));
        assert!(!auth_ok("not json", "abc"));
    }

    #[test]
    fn a_newer_frame_replaces_the_queued_one() {
        let l = Link::default();
        l.video("a", vec![1]);
        assert!(l.take(0).is_some_and(|(t, v)| t.is_empty() && v.is_none()), "nothing queued without a client");
        l.0.lock().unwrap().conn = 7;
        l.video("a", vec![1]);
        l.video("b", vec![2]);
        l.video("a", vec![3]);
        l.send(&serde_json::json!({"t": "x"}));
        let (text, v) = l.take(7).unwrap();
        assert_eq!(text, [r#"{"t":"x"}"#]);
        assert_eq!(v, Some(vec![3]));
        assert_eq!(l.take(7).unwrap().1, Some(vec![2]));
        assert!(l.take(8).is_none(), "another connection's queue");
    }
}
