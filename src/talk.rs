//! Push to talk while the window is not focused (docs/push-to-talk.md). Three ways in, all ending in a `TalkAct`:
//! the window's own key (handled in `window.rs`), `soworker --talk [down|up|toggle]` run by a desktop shortcut and
//! delivered over a local socket, and the xdg-desktop-portal GlobalShortcuts portal (press and release events) on
//! desktops that have it. COSMIC has no such portal yet, so there the desktop shortcut is the way. On Windows the
//! socket is a localhost TCP port guarded by a token (both in a file only this user can read), and the global shortcut
//! is a low-level keyboard hook.
use std::io::{Read, Write};
#[cfg(unix)]
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

#[cfg(unix)]
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
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
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
#[cfg(unix)]
pub fn send(act: &str) -> Result<(), String> {
    parse(act).ok_or("usage: soworker --talk [down|up|toggle]")?;
    let mut s = UnixStream::connect(socket_path()).map_err(|_| "SoWorker is not running".to_string())?;
    s.write_all(format!("{act}\n").as_bytes()).map_err(|e| e.to_string())
}

/// Listen for `--talk` commands from other processes of this user.
#[cfg(unix)]
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

/// `%TEMP%\soworker\ctl.port`: "<port> <token>" of the running app. %TEMP% is private to the user.
#[cfg(windows)]
fn port_path() -> PathBuf {
    std::env::temp_dir().join("soworker").join("ctl.port")
}

/// The running app's address and token, if it wrote them and still answers.
#[cfg(windows)]
fn connect() -> Option<(std::net::TcpStream, String)> {
    let text = std::fs::read_to_string(port_path()).ok()?;
    let (port, token) = text.trim().split_once(' ')?;
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port.parse().ok()?));
    let s = std::net::TcpStream::connect_timeout(&addr, std::time::Duration::from_millis(500)).ok()?;
    Some((s, token.to_string()))
}

/// `soworker --talk [down|up|toggle]`: tell the running app.
#[cfg(windows)]
pub fn send(act: &str) -> Result<(), String> {
    parse(act).ok_or("usage: soworker --talk [down|up|toggle]")?;
    let (mut s, token) = connect().ok_or("SoWorker is not running")?;
    s.write_all(format!("{token} {act}\n").as_bytes()).map_err(|e| e.to_string())
}

/// Listen for `--talk` commands from other processes of this user.
#[cfg(windows)]
pub fn listen(emit: impl Fn(TalkAct) + Send + 'static) {
    if connect().is_some() {
        return; // another SoWorker already listens
    }
    let path = port_path();
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let Ok(listener) = std::net::TcpListener::bind(("127.0.0.1", 0)) else { return };
    let Ok(addr) = listener.local_addr() else { return };
    let mut b = [0u8; 16];
    if getrandom::getrandom(&mut b).is_err() {
        return;
    }
    let token: String = b.iter().map(|x| format!("{x:02x}")).collect();
    if std::fs::write(&path, format!("{} {token}", addr.port())).is_err() {
        return;
    }
    std::thread::Builder::new()
        .name("talk-ctl".into())
        .spawn(move || {
            for conn in listener.incoming().flatten() {
                let _ = conn.set_read_timeout(Some(std::time::Duration::from_secs(2)));
                let mut line = String::new();
                if conn.take(128).read_to_string(&mut line).is_ok() {
                    if let Some(act) = line.trim().split_once(' ').filter(|(t, _)| *t == token).and_then(|(_, a)| parse(a)) {
                        emit(act);
                    }
                }
            }
        })
        .expect("talk thread");
}

/// Register a push-to-talk shortcut with the GlobalShortcuts portal. `trigger` is the preferred key, e.g. "F1"; the
/// desktop may let the user choose another. Does nothing (apart from a log line) where the portal does not exist.
#[cfg(all(feature = "portal", target_os = "linux"))]
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

#[cfg(not(all(feature = "portal", any(target_os = "linux", windows))))]
pub fn portal(_emit: impl Fn(TalkAct) + Send + Sync + 'static, _trigger: String) {}

#[cfg(all(feature = "portal", target_os = "linux"))]
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

/// Watch the keyboard for the push-to-talk combination, whichever window has focus (press and release events). The
/// combination is swallowed so it does not also type into the focused app.
#[cfg(all(feature = "portal", windows))]
pub fn portal(emit: impl Fn(TalkAct) + Send + Sync + 'static, trigger: String) {
    std::thread::Builder::new()
        .name("talk-hook".into())
        .spawn(move || {
            if let Err(e) = hook::run(Box::new(emit), Combo::parse(&trigger)) {
                eprintln!("push to talk: no global shortcut ({e})");
            }
        })
        .expect("hook thread");
}

/// The Windows virtual-key code for a key as `Combo` writes it (US layout for the punctuation keys).
#[cfg_attr(not(windows), allow(dead_code))]
fn virtual_key(key: &str) -> Option<u32> {
    let c = |ch: char| key.chars().count() == 1 && key.starts_with(ch);
    Some(match key {
        k if k.len() == 1 && k.as_bytes()[0].is_ascii_alphanumeric() => k.as_bytes()[0].to_ascii_uppercase() as u32,
        k if k.len() >= 2 && k.starts_with('f') && k[1..].parse::<u32>().is_ok_and(|n| (1..=24).contains(&n)) => 0x6F + k[1..].parse::<u32>().ok()?,
        "space" => 0x20,
        "pause" => 0x13,
        "scrolllock" => 0x91,
        "insert" => 0x2D,
        "home" => 0x24,
        "end" => 0x23,
        "pageup" => 0x21,
        "pagedown" => 0x22,
        _ if c(' ') => 0x20,
        _ if c('\'') => 0xDE,
        _ if c(';') => 0xBA,
        _ if c('=') || c('+') => 0xBB,
        _ if c(',') => 0xBC,
        _ if c('-') => 0xBD,
        _ if c('.') => 0xBE,
        _ if c('/') => 0xBF,
        _ if c('`') => 0xC0,
        _ if c('[') => 0xDB,
        _ if c('\\') => 0xDC,
        _ if c(']') => 0xDD,
        _ => return None,
    })
}

#[cfg(all(feature = "portal", windows))]
mod hook {
    use super::{Combo, TalkAct};
    use std::cell::RefCell;
    use windows_sys::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_CONTROL, VK_LWIN, VK_MENU, VK_RWIN, VK_SHIFT,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage, HC_ACTION, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
        WM_SYSKEYDOWN, WM_SYSKEYUP,
    };

    struct State {
        emit: Box<dyn Fn(TalkAct) + Send + Sync>,
        combo: Combo,
        vk: u32,
        held: bool,
    }

    thread_local! {
        // the hook procedure runs on the thread that installed it, inside its message loop
        static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
    }

    pub fn run(emit: Box<dyn Fn(TalkAct) + Send + Sync>, combo: Combo) -> Result<(), String> {
        let vk = super::virtual_key(&combo.key).ok_or_else(|| format!("unknown key {:?}", combo.key))?;
        STATE.with(|s| *s.borrow_mut() = Some(State { emit, combo, vk, held: false }));
        // SAFETY: a low-level hook needs no module handle; the procedure is a plain function.
        let hook = unsafe { SetWindowsHookExW(WH_KEYBOARD_LL, Some(on_key), std::ptr::null_mut(), 0) };
        if hook.is_null() {
            return Err("cannot install the keyboard hook".into());
        }
        // SAFETY: an ordinary message loop on this thread, which is what keeps the hook called.
        unsafe {
            let mut msg: MSG = std::mem::zeroed();
            while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
        Err("message loop ended".into())
    }

    fn pressed(vk: u16) -> bool {
        // SAFETY: only reads key state.
        unsafe { GetAsyncKeyState(vk as i32) < 0 }
    }

    /// Tap an unassigned key, so releasing the Windows key after the combination does not open the Start menu.
    fn mask_start_menu() {
        let key = |flags| INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: 0xE8, wScan: 0, dwFlags: flags, time: 0, dwExtraInfo: 0 } } };
        let inputs = [key(0), key(KEYEVENTF_KEYUP)];
        // SAFETY: two well-formed keyboard inputs.
        unsafe { SendInput(inputs.len() as u32, inputs.as_ptr(), std::mem::size_of::<INPUT>() as i32) };
    }

    unsafe extern "system" fn on_key(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        if code == HC_ACTION as i32 {
            // SAFETY: for HC_ACTION, lparam points at the event's KBDLLHOOKSTRUCT.
            let k = unsafe { &*(lparam as *const KBDLLHOOKSTRUCT) };
            let down = matches!(wparam as u32, WM_KEYDOWN | WM_SYSKEYDOWN);
            let up = matches!(wparam as u32, WM_KEYUP | WM_SYSKEYUP);
            let swallow = STATE.with(|s| {
                let mut s = s.borrow_mut();
                let Some(st) = s.as_mut().filter(|st| st.vk == k.vkCode) else { return false };
                if down {
                    if st.held {
                        return true; // auto-repeat
                    }
                    let c = &st.combo;
                    let mods = pressed(VK_CONTROL) == c.ctrl && pressed(VK_MENU) == c.alt && pressed(VK_SHIFT) == c.shift && (pressed(VK_LWIN) || pressed(VK_RWIN)) == c.sup;
                    if !mods {
                        return false;
                    }
                    st.held = true;
                    if c.sup {
                        mask_start_menu();
                    }
                    (st.emit)(TalkAct::Down);
                    true
                } else if up && st.held {
                    st.held = false;
                    (st.emit)(TalkAct::Up);
                    true
                } else {
                    false
                }
            });
            if swallow {
                return 1;
            }
        }
        // SAFETY: passes the event on unchanged.
        unsafe { CallNextHookEx(std::ptr::null_mut(), code, wparam, lparam) }
    }
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
    fn keys_map_to_windows_virtual_keys() {
        assert_eq!(virtual_key("'"), Some(0xDE));
        assert_eq!(virtual_key("a"), Some(0x41));
        assert_eq!(virtual_key("7"), Some(0x37));
        assert_eq!(virtual_key("f1"), Some(0x70));
        assert_eq!(virtual_key("f24"), Some(0x87));
        assert_eq!(virtual_key("space"), Some(0x20));
        assert_eq!(virtual_key("+"), Some(0xBB));
        assert_eq!(virtual_key("f25"), None);
        assert_eq!(virtual_key("nope"), None);
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
