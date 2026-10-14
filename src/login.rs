//! Sign-in without any credential typing in our window: open Chrome with a throwaway profile on SoWork's own login
//! page, watch it over the DevTools protocol, and once the Firebase login exists in the page's IndexedDB take the
//! refresh token. Chrome is closed and its profile deleted afterwards, so no Google cookies stay behind.
use serde_json::Value;
use std::net::TcpStream;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use tungstenite::Message;

pub struct Options {
    /// Where the browser starts (SoWork's login page, or `/login?returnTo=` for a known space).
    pub start_url: String,
    /// The page origin whose IndexedDB holds the Firebase login.
    pub origin: String,
    /// `(groupId, alias)` when the space is already known; otherwise we wait for the user to open one (`/s/<g>/<a>`).
    pub known_space: Option<(String, String)>,
    pub headless: bool,
}

pub struct Captured {
    pub api_key: String,
    pub refresh_token: String,
    pub space: (String, String),
}

#[cfg(not(windows))]
fn find_chrome() -> Option<std::path::PathBuf> {
    let names = ["google-chrome-stable", "google-chrome", "chromium", "chromium-browser", "chrome"];
    std::env::var_os("PATH").and_then(|p| {
        std::env::split_paths(&p).find_map(|d| names.iter().map(|n| d.join(n)).find(|c| c.is_file()))
    })
}

/// Chrome where its installer puts it, else Edge (Chromium too, and always there on Windows).
#[cfg(windows)]
fn find_chrome() -> Option<std::path::PathBuf> {
    let installs = [r"Google\Chrome\Application\chrome.exe", r"Chromium\Application\chrome.exe", r"Microsoft\Edge\Application\msedge.exe"];
    let roots: Vec<std::path::PathBuf> = ["LOCALAPPDATA", "ProgramFiles", "ProgramFiles(x86)"].iter().filter_map(std::env::var_os).map(Into::into).collect();
    installs.iter().find_map(|i| roots.iter().map(|r| r.join(i)).find(|c| c.is_file()))
}

/// Reads the Firebase login out of the page, but only when the page really is on `origin`
/// (opening the database elsewhere would create an empty one there).
const READ_LOGIN: &str = r#"(async () => {
  if (location.origin !== ORIGIN) return null;
  // never create the database ourselves: Firebase makes it (version 1, with its object store) on its first sign-in
  const known = await indexedDB.databases();
  if (!known.some((d) => d.name === 'firebaseLocalStorageDb')) return null;
  const db = await new Promise((res, rej) => { const r = indexedDB.open('firebaseLocalStorageDb'); r.onsuccess = () => res(r.result); r.onerror = () => rej(r.error); });
  if (!db.objectStoreNames.contains('firebaseLocalStorage')) { db.close(); return JSON.stringify({ href: location.href, entries: [] }); }
  const entries = await new Promise((res) => { const q = db.transaction('firebaseLocalStorage').objectStore('firebaseLocalStorage').getAll(); q.onsuccess = () => res(q.result); q.onerror = () => res([]); });
  db.close();
  return JSON.stringify({ href: location.href, entries });
})()"#;

fn cdp_eval(ws_url: &str, expr: &str) -> Result<Option<String>, String> {
    let host = ws_url.trim_start_matches("ws://").split('/').next().unwrap_or("127.0.0.1:9222").to_string();
    let tcp = TcpStream::connect(&host).map_err(|e| e.to_string())?;
    tcp.set_read_timeout(Some(Duration::from_secs(5))).ok();
    let (mut ws, _) = tungstenite::client(ws_url, tcp).map_err(|e| e.to_string())?;
    let req = serde_json::json!({"id": 1, "method": "Runtime.evaluate", "params": {"expression": expr, "awaitPromise": true, "returnByValue": true}});
    ws.send(Message::Text(req.to_string())).map_err(|e| e.to_string())?;
    loop {
        match ws.read().map_err(|e| e.to_string())? {
            Message::Text(t) => {
                let v: Value = serde_json::from_str(&t).map_err(|e| e.to_string())?;
                if v["id"] == 1 {
                    let _ = ws.close(None);
                    return Ok(v["result"]["result"]["value"].as_str().map(String::from));
                }
            }
            Message::Close(_) => return Err("devtools closed".into()),
            _ => {}
        }
    }
}

/// The default Firebase app's login: key `firebase:authUser:<apiKey>:[DEFAULT]`.
fn parse_login(json: &str) -> Option<(String, String, String)> {
    let v: Value = serde_json::from_str(json).ok()?;
    let href = v["href"].as_str()?.to_string();
    for e in v["entries"].as_array()? {
        let key = e["fbase_key"].as_str().unwrap_or("");
        if key.starts_with("firebase:authUser:") && key.ends_with(":[DEFAULT]") {
            let value = &e["value"];
            let (api, refresh) = (value["apiKey"].as_str()?, value["stsTokenManager"]["refreshToken"].as_str()?);
            if !api.is_empty() && !refresh.is_empty() {
                return Some((api.into(), refresh.into(), href));
            }
        }
    }
    None
}

/// `/s/<groupId>/<alias>` in a URL path.
fn space_of(href: &str) -> Option<(String, String)> {
    let after = href.split_once("/s/")?.1;
    let mut it = after.split(['/', '?', '#']);
    let (g, a) = (it.next()?, it.next()?);
    (!g.is_empty() && !a.is_empty()).then(|| (g.to_string(), a.to_string()))
}

struct Browser {
    child: Child,
    profile: std::path::PathBuf,
}

impl Drop for Browser {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        for _ in 0..5 {
            if std::fs::remove_dir_all(&self.profile).is_ok() || !self.profile.exists() {
                break;
            }
            std::thread::sleep(Duration::from_millis(200)); // Chrome may still be flushing files
        }
    }
}

pub fn capture(opts: &Options, cancel: &AtomicBool) -> Result<Captured, String> {
    let missing = if cfg!(windows) { "Chrome or Edge is needed for signing in, but neither was found" } else { "Chrome or Chromium is needed for signing in, but none was found in PATH" };
    let chrome = find_chrome().ok_or(missing)?;
    let profile = std::env::temp_dir().join(format!("soworker-login-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&profile);
    std::fs::create_dir_all(&profile).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&profile, std::fs::Permissions::from_mode(0o700));
    }
    // Same flags as the recorder, whose Chrome Google accepted for sign-in. No --app window, no --disable-extensions,
    // no forced size: anything that makes the browser look unusual risks "this browser may not be secure".
    let mut cmd = Command::new(chrome);
    cmd.arg(format!("--user-data-dir={}", profile.display()))
        .args(["--remote-debugging-port=0", "--no-first-run", "--no-default-browser-check"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if opts.headless {
        cmd.args(["--headless=new", "--no-sandbox", "--disable-gpu"]);
    }
    cmd.arg(&opts.start_url);
    let child = cmd.spawn().map_err(|e| format!("could not start Chrome: {e}"))?;
    let mut browser = Browser { child, profile: profile.clone() };

    // DevToolsActivePort: first line is the port Chrome picked
    let port_file = profile.join("DevToolsActivePort");
    let port = (0..150)
        .find_map(|_| {
            std::thread::sleep(Duration::from_millis(100));
            std::fs::read_to_string(&port_file).ok().and_then(|s| s.lines().next().and_then(|l| l.trim().parse::<u16>().ok()))
        })
        .ok_or("Chrome did not open its debugging port")?;

    let expr = READ_LOGIN.replace("ORIGIN", &serde_json::to_string(&opts.origin).unwrap_or_default());
    let started = Instant::now();
    loop {
        if cancel.load(Ordering::Relaxed) {
            return Err("sign-in cancelled".into());
        }
        if let Ok(Some(_)) = browser.child.try_wait() {
            return Err("the browser window was closed before signing in".into());
        }
        if started.elapsed() > Duration::from_secs(15 * 60) {
            return Err("sign-in timed out".into());
        }
        let targets: Vec<Value> = ureq::get(&format!("http://127.0.0.1:{port}/json"))
            .timeout(Duration::from_secs(3))
            .call()
            .ok()
            .and_then(|r| r.into_json().ok())
            .unwrap_or_default();
        let page = targets.iter().find(|t| t["type"] == "page" && t["url"].as_str().is_some_and(|u| u.starts_with(&opts.origin)));
        if let Some(ws_url) = page.and_then(|t| t["webSocketDebuggerUrl"].as_str()) {
            if let Ok(Some(json)) = cdp_eval(ws_url, &expr) {
                if let Some((api_key, refresh_token, href)) = parse_login(&json) {
                    // signed in; the space comes from the page address, or from the one we already knew
                    let space = space_of(&href).or_else(|| opts.known_space.clone());
                    if let Some(space) = space {
                        return Ok(Captured { api_key, refresh_token, space });
                    }
                }
            }
        }
        for _ in 0..10 {
            if cancel.load(Ordering::Relaxed) {
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

pub fn login_url(known: &Option<(String, String)>) -> String {
    match known {
        Some((g, a)) => format!("https://app.sowork.com/login?returnTo=%2Fs%2F{g}%2F{a}"),
        None => "https://app.sowork.com/login".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_default_firebase_login_and_ignores_other_apps() {
        let json = r#"{"href":"https://app.sowork.com/s/grp1/office?x=1","entries":[
          {"fbase_key":"firebase:authUser:KEY2:[calendar]","value":{"apiKey":"KEY2","stsTokenManager":{"refreshToken":"other"}}},
          {"fbase_key":"firebase:authUser:KEY1:[DEFAULT]","value":{"apiKey":"KEY1","stsTokenManager":{"refreshToken":"RT"}}}]}"#;
        let (api, rt, href) = parse_login(json).unwrap();
        assert_eq!((api.as_str(), rt.as_str()), ("KEY1", "RT"));
        assert_eq!(space_of(&href), Some(("grp1".into(), "office".into())));
        assert!(parse_login(r#"{"href":"x","entries":[]}"#).is_none());
        assert_eq!(space_of("https://app.sowork.com/login?returnTo=%2Fs%2Fg%2Fa"), None);
    }
}
