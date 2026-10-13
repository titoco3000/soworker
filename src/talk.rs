//! Push to talk while the window is not focused (docs/push-to-talk.md). Three ways in, all ending in a `TalkAct`:
//! the window's own key (handled in `window.rs`), `soworker --talk [down|up|toggle]` run by a desktop shortcut and
//! delivered over a local socket, and the xdg-desktop-portal GlobalShortcuts portal (press and release events) on
//! desktops that have it. COSMIC has no such portal yet, so there the desktop shortcut is the way.
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TalkAct {
    /// Key pressed: the mic is on while it is held.
    Down,
    Up,
    /// A single press of a shortcut that cannot report its release: flip the mic.
    Toggle,
}

pub fn parse(s: &str) -> Option<TalkAct> {
    match s.trim() {
        "down" => Some(TalkAct::Down),
        "up" => Some(TalkAct::Up),
        "toggle" => Some(TalkAct::Toggle),
        _ => None,
    }
}

fn socket_path() -> PathBuf {
    std::env::var_os("XDG_RUNTIME_DIR").map_or_else(std::env::temp_dir, PathBuf::from).join("soworker").join("ctl.sock")
}

/// The key shown in hints and used inside the window: `talkKey` of `settings.json` beside the session file, else F1.
/// (A desktop shortcut or the portal binds its own key; this is only what we tell the user and what the window listens to.)
pub fn key_label() -> String {
    let path = std::path::Path::new(&crate::session::default_path()).with_file_name("settings.json");
    std::fs::read_to_string(path)
        .ok()
        .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
        .and_then(|v| v["talkKey"].as_str().map(String::from))
        .filter(|k| !k.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_KEY.into())
}

pub const DEFAULT_KEY: &str = "Super+'";

/// A key with modifiers, written like `Super+'`, `Ctrl+Alt+Space` or `Pause`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Combo {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub sup: bool,
    /// The key itself, lower case (`'`, `f1`, `pause`, `space`).
    pub key: String,
}

impl Combo {
    pub fn parse(label: &str) -> Combo {
        let mut c = Combo::default();
        // split on '+', but a trailing "+" key ("Ctrl++") is the plus key itself
        let mut parts: Vec<&str> = label.split('+').collect();
        if label.ends_with('+') && parts.len() >= 2 {
            parts.truncate(parts.len() - 2);
            c.key = "+".into();
        } else if let Some(k) = parts.pop() {
            c.key = k.trim().to_lowercase();
        }
        for m in parts {
            match m.trim().to_lowercase().as_str() {
                "ctrl" | "control" => c.ctrl = true,
                "alt" => c.alt = true,
                "shift" => c.shift = true,
                "super" | "win" | "windows" | "logo" | "meta" => c.sup = true,
                _ => {}
            }
        }
        c
    }

    /// The portal's trigger syntax (XDG shortcuts): `LOGO+apostrophe`, `CTRL+ALT+space`, `F1`.
    pub fn portal_trigger(&self) -> String {
        let key = match self.key.as_str() {
            "'" => "apostrophe",
            "\"" => "quotedbl",
            " " => "space",
            "+" => "plus",
            "," => "comma",
            "." => "period",
            "-" => "minus",
            "/" => "slash",
            ";" => "semicolon",
            k => k,
        };
        let mut v = Vec::new();
        for (on, name) in [(self.sup, "LOGO"), (self.ctrl, "CTRL"), (self.alt, "ALT"), (self.shift, "SHIFT")] {
            if on {
                v.push(name.to_string());
            }
        }
        v.push(key.to_string());
        v.join("+")
    }
}

/// `soworker --talk [down|up|toggle]`: tell the running app.
pub fn send(act: &str) -> Result<(), String> {
    parse(act).ok_or("usage: soworker --talk [down|up|toggle]")?;
    let mut s = UnixStream::connect(socket_path()).map_err(|_| "SoWorker is not running".to_string())?;
    s.write_all(format!("{act}\n").as_bytes()).map_err(|e| e.to_string())
}

/// Listen for `--talk` commands from other processes of this user.
pub fn listen(emit: impl Fn(TalkAct) + Send + 'static) {
    let path = socket_path();
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    if UnixStream::connect(&path).is_ok() {
        return; // another SoWorker already owns the socket
    }
    let _ = std::fs::remove_file(&path); // a stale file from a crashed run
    let Ok(listener) = UnixListener::bind(&path) else { return };
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
    }
    std::thread::Builder::new()
        .name("talk-ctl".into())
        .spawn(move || {
            for conn in listener.incoming().flatten() {
                let mut line = String::new();
                if conn.take(64).read_to_string(&mut line).is_ok() {
                    if let Some(act) = parse(&line) {
                        emit(act);
                    }
                }
            }
        })
        .expect("talk thread");
}

/// Register a push-to-talk shortcut with the GlobalShortcuts portal. `trigger` is the preferred key, e.g. "F1"; the
/// desktop may let the user choose another. Does nothing (apart from a log line) where the portal does not exist.
#[cfg(feature = "portal")]
pub fn portal(emit: impl Fn(TalkAct) + Send + Sync + 'static, trigger: String) {
    std::thread::Builder::new()
        .name("talk-portal".into())
        .spawn(move || {
            if let Err(e) = portal_run(std::sync::Arc::new(emit), &Combo::parse(&trigger).portal_trigger()) {
                eprintln!("push to talk: no global shortcut portal ({e}); bind `soworker --talk` to a key in your desktop settings");
            }
        })
        .expect("portal thread");
}

#[cfg(not(feature = "portal"))]
pub fn portal(_emit: impl Fn(TalkAct) + Send + Sync + 'static, _trigger: String) {}

#[cfg(feature = "portal")]
fn portal_run(emit: std::sync::Arc<dyn Fn(TalkAct) + Send + Sync>, trigger: &str) -> Result<(), String> {
    use std::collections::HashMap;
    use zbus::blocking::{Connection, Proxy};
    use zbus::zvariant::{ObjectPath, OwnedValue, Value};
    let es = |e: zbus::Error| e.to_string();
    const DEST: &str = "org.freedesktop.portal.Desktop";
    const PATH: &str = "/org/freedesktop/portal/desktop";
    const IFACE: &str = "org.freedesktop.portal.GlobalShortcuts";
    let conn = Connection::session().map_err(es)?;
    let sender = conn.unique_name().ok_or("no bus name")?.as_str().trim_start_matches(':').replace('.', "_");
    let portal = Proxy::new(&conn, DEST, PATH, IFACE).map_err(es)?;

    // every portal call answers through a `Response` signal on a request object whose path we can compute up front
    let ask = |token: &str, call: &dyn Fn() -> Result<(), zbus::Error>| -> Result<HashMap<String, OwnedValue>, String> {
        let req = Proxy::new(&conn, DEST, format!("{PATH}/request/{sender}/{token}"), "org.freedesktop.portal.Request").map_err(es)?;
        let mut sig = req.receive_signal("Response").map_err(es)?;
        call().map_err(es)?;
        let msg = sig.next().ok_or("portal closed")?;
        let (code, results): (u32, HashMap<String, OwnedValue>) = msg.body().deserialize().map_err(es)?;
        if code != 0 {
            return Err(format!("the portal said no (code {code})"));
        }
        Ok(results)
    };

    let created = ask("soworker_session", &|| {
        let opts: HashMap<&str, Value> = HashMap::from([("handle_token", Value::from("soworker_session")), ("session_handle_token", Value::from("soworker"))]);
        portal.call_method("CreateSession", &(opts,)).map(|_| ())
    })?;
    let session: String = created.get("session_handle").ok_or("no session handle")?.try_clone().map_err(|e| e.to_string())?.try_into().map_err(|e: zbus::zvariant::Error| e.to_string())?;
    let session_path = ObjectPath::try_from(session.as_str()).map_err(|e| e.to_string())?;

    ask("soworker_bind", &|| {
        let props: HashMap<&str, Value> = HashMap::from([("description", Value::from("Push to talk")), ("preferred_trigger", Value::from(trigger))]);
        let opts: HashMap<&str, Value> = HashMap::from([("handle_token", Value::from("soworker_bind"))]);
        portal.call_method("BindShortcuts", &(&session_path, vec![("talk", props)], "", opts)).map(|_| ())
    })?;

    let mut down = portal.receive_signal("Activated").map_err(es)?;
    let mut up = portal.receive_signal("Deactivated").map_err(es)?;
    let emit2 = emit.clone();
    std::thread::spawn(move || {
        while let Some(msg) = up.next() {
            if let Ok((_, id, _, _)) = msg.body().deserialize::<(ObjectPath, String, u64, HashMap<String, OwnedValue>)>() {
                if id == "talk" {
                    emit2(TalkAct::Up);
                }
            }
        }
    });
    while let Some(msg) = down.next() {
        if let Ok((_, id, _, _)) = msg.body().deserialize::<(ObjectPath, String, u64, HashMap<String, OwnedValue>)>() {
            if id == "talk" {
                emit(TalkAct::Down);
            }
        }
    }
    Err("portal connection closed".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combos_parse_and_map_to_portal_triggers() {
        let c = Combo::parse("Super+'");
        assert_eq!((c.sup, c.ctrl, c.key.as_str()), (true, false, "'"));
        assert_eq!(c.portal_trigger(), "LOGO+apostrophe");
        assert_eq!(Combo::parse("Ctrl+Alt+Space").portal_trigger(), "CTRL+ALT+space");
        assert_eq!(Combo::parse("F1").portal_trigger(), "f1");
        let plus = Combo::parse("Ctrl++");
        assert_eq!((plus.ctrl, plus.key.as_str()), (true, "+"));
        assert_eq!(Combo::parse("Pause"), Combo { key: "pause".into(), ..Combo::default() });
    }

    #[test]
    fn commands_parse() {
        assert_eq!(parse("down\n"), Some(TalkAct::Down));
        assert_eq!(parse("up"), Some(TalkAct::Up));
        assert_eq!(parse("toggle"), Some(TalkAct::Toggle));
        assert_eq!(parse(""), None, "a probe connection that sends nothing is not a command");
        assert_eq!(parse("shout"), None);
    }

    #[test]
    fn a_command_reaches_the_listener_and_a_second_app_does_not_steal_the_socket() {
        let dir = std::env::temp_dir().join(format!("soworker-talk-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::env::set_var("XDG_RUNTIME_DIR", &dir);
        let (tx, rx) = std::sync::mpsc::channel();
        let tx2 = std::sync::Mutex::new(tx);
        listen(move |a| drop(tx2.lock().unwrap().send(a)));
        listen(|_| panic!("the second listener must not take over")); // returns at once: the socket answers
        send("down").unwrap();
        send("up").unwrap();
        assert_eq!(rx.recv_timeout(std::time::Duration::from_secs(2)), Ok(TalkAct::Down));
        assert_eq!(rx.recv_timeout(std::time::Duration::from_secs(2)), Ok(TalkAct::Up));
        assert!(send("shout").is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
