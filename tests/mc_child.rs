//! `soworker --mc-child`, the process the Minecraft mod starts: the port line, the token from the environment, the
//! end on stdin EOF, and sign-in failures that keep it running. Only in the mod's build:
//! `cargo test --no-default-features --features mc,calls --target-dir target/mc`.
#![cfg(feature = "mc")]

use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::time::{Duration, Instant};
use tungstenite::Message;

const TOKEN: &str = "0123456789abcdef0123456789abcdef0123456789abcdef";

struct Proc {
    child: Child,
    stdin: Option<ChildStdin>,
    port: u16,
    _dir: Dir,
}

impl Drop for Proc {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// A scratch directory, removed afterwards.
struct Dir(PathBuf);

impl Dir {
    fn new(name: &str) -> Dir {
        let d = std::env::temp_dir().join(format!("soworker-mc-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        Dir(d)
    }
}

impl Drop for Dir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Start the child with everything (config, state, runtime dirs, PATH) inside a scratch directory.
fn start(name: &str, args: &[&str], path: Option<&Path>) -> Proc {
    let dir = Dir::new(name);
    let d = &dir.0;
    let mut c = Command::new(env!("CARGO_BIN_EXE_soworker"));
    c.arg("--mc-child")
        .args(["--session", &d.join("config/soworker/session.json").to_string_lossy()])
        .args(args)
        .env("SOWORKER_MC_TOKEN", TOKEN)
        .env("SOWORKER_LOG_STDERR", "1")
        .env("HOME", d)
        .env("XDG_CONFIG_HOME", d.join("config"))
        .env("XDG_STATE_HOME", d.join("state"))
        .env("XDG_RUNTIME_DIR", d.join("run"))
        .env("TMPDIR", d)
        .env("APPDATA", d.join("config"))
        .env("LOCALAPPDATA", d.join("state"))
        .env("PATH", path.map_or_else(|| d.join("no-bin"), Path::to_path_buf))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    let mut child = c.spawn().expect("start soworker");
    let mut out = BufReader::new(child.stdout.take().unwrap());
    let mut first = String::new();
    out.read_line(&mut first).unwrap();
    let port = first.trim().strip_prefix("port ").and_then(|p| p.parse().ok()).unwrap_or_else(|| panic!("first line {first:?}"));
    std::thread::spawn(move || for _ in out.lines() {}); // keep stdout drained
    let stdin = child.stdin.take();
    Proc { child, stdin, port, _dir: dir }
}

type Ws = tungstenite::WebSocket<TcpStream>;

fn connect(port: u16, token: &str) -> Ws {
    let s = TcpStream::connect(("127.0.0.1", port)).unwrap();
    s.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
    let (mut ws, _) = tungstenite::client(format!("ws://127.0.0.1:{port}/"), s).unwrap();
    ws.send(Message::Text(serde_json::json!({"t": "auth", "token": token}).to_string())).unwrap();
    ws
}

/// The next JSON message of kind `t` (others are skipped), or None when the socket closes first.
fn next(ws: &mut Ws, t: &str) -> Option<serde_json::Value> {
    loop {
        match ws.read() {
            Ok(Message::Text(s)) => {
                let v: serde_json::Value = serde_json::from_str(&s).unwrap();
                if v["t"] == t {
                    return Some(v);
                }
            }
            Ok(Message::Close(_)) | Err(_) => return None,
            Ok(_) => {}
        }
    }
}

fn exits_within(p: &mut Proc, d: Duration) -> bool {
    let until = Instant::now() + d;
    while Instant::now() < until {
        if p.child.try_wait().unwrap().is_some() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    false
}

#[test]
fn prints_its_port_and_takes_only_the_token_from_the_environment() {
    let p = start("token", &["--mc-demo"], None);
    let mut good = connect(p.port, TOKEN);
    let login = next(&mut good, "login").expect("login state on connect");
    assert_eq!(login["state"], "signed_in");
    let hello = next(&mut good, "hello").expect("hello");
    assert_eq!(hello["proto"], 1);
    let mut bad = connect(p.port, "not-the-token-not-the-token-not-the-token-xxxxxx");
    assert!(next(&mut bad, "hello").is_none(), "a wrong token is closed");
}

#[test]
fn ends_when_stdin_closes() {
    let mut p = start("stdin", &["--mc-demo"], None);
    assert!(!exits_within(&mut p, Duration::from_millis(300)), "runs while stdin is open");
    drop(p.stdin.take());
    assert!(exits_within(&mut p, Duration::from_secs(2)), "gone within 2 s of stdin closing");
}

#[test]
fn no_browser_is_reported() {
    let mut p = start("nobrowser", &["--dry-run"], None);
    let mut ws = connect(p.port, TOKEN);
    assert_eq!(next(&mut ws, "login").unwrap()["state"], "no_browser");
    assert!(p.child.try_wait().unwrap().is_none());
}

#[cfg(unix)]
#[test]
fn a_failed_sign_in_keeps_it_running_and_login_tries_again() {
    use std::os::unix::fs::PermissionsExt;
    // a "browser" that opens its debugging port and closes at once: the sign-in fails quickly
    let bin = Dir::new("fakebrowser");
    let fake = bin.0.join("chromium");
    std::fs::write(&fake, "#!/bin/sh\nfor a in \"$@\"; do case \"$a\" in --user-data-dir=*) d=\"${a#--user-data-dir=}\";; esac; done\nprintf '1\\n' > \"$d/DevToolsActivePort\"\n").unwrap();
    std::fs::set_permissions(&fake, std::fs::Permissions::from_mode(0o755)).unwrap();
    let mut p = start("failed", &["--dry-run"], Some(&bin.0));
    let mut ws = connect(p.port, TOKEN);
    // on connect: the state as it is now, `browser` (then `failed`) or already `failed`
    let mut last = next(&mut ws, "login").expect("login state");
    if last["state"] == "browser" {
        last = next(&mut ws, "login").expect("login state");
    }
    assert_eq!(last["state"], "failed", "{last}");
    assert!(last["error"].as_str().is_some_and(|e| !e.is_empty()));
    assert!(p.child.try_wait().unwrap().is_none(), "still running");
    ws.send(Message::Text(r#"{"t":"cmd","name":"login"}"#.into())).unwrap();
    assert_eq!(next(&mut ws, "login").unwrap()["state"], "browser");
    assert_eq!(next(&mut ws, "login").unwrap()["state"], "failed");
    assert!(p.child.try_wait().unwrap().is_none(), "still running");
}
