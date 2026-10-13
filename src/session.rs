//! Firebase session: a refresh token on disk, fresh ID tokens on demand (docs/auth-and-rest.md).
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

pub const CLIENT_VERSION: &str = "11.202.0";

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionFile {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    /// Body of `POST /api/v1/gaia/find` (the invite link).
    pub find: serde_json::Value,
}

#[derive(Clone)]
pub struct Auth {
    pub path: String,
    pub file: SessionFile,
    token: String,
    exp: Instant,
    pub uid: String,
    pub name: String,
}

impl Auth {
    pub fn load(path: &str) -> Result<Auth, String> {
        let raw = std::fs::read_to_string(path).map_err(|e| format!("cannot read {path}: {e}"))?;
        let file: SessionFile = serde_json::from_str(&raw).map_err(|e| format!("bad session file: {e}"))?;
        let mut a = Auth { path: path.into(), file, token: String::new(), exp: Instant::now(), uid: String::new(), name: String::new() };
        a.id_token()?;
        Ok(a)
    }

    /// A valid ID token (refreshing it when less than 5 minutes remain).
    pub fn id_token(&mut self) -> Result<String, String> {
        if !self.token.is_empty() && Instant::now() + Duration::from_secs(300) < self.exp {
            return Ok(self.token.clone());
        }
        let url = format!("https://securetoken.googleapis.com/v1/token?key={}", self.file.api_key);
        let resp: serde_json::Value = ureq::post(&url)
            .send_form(&[("grant_type", "refresh_token"), ("refresh_token", &self.file.refresh_token)])
            .map_err(|e| format!("token refresh failed: {e}"))?
            .into_json()
            .map_err(|e| e.to_string())?;
        self.token = resp["id_token"].as_str().ok_or("no id_token in refresh response")?.to_string();
        let secs = resp["expires_in"].as_str().and_then(|s| s.parse().ok()).unwrap_or(3600u64);
        self.exp = Instant::now() + Duration::from_secs(secs);
        if let Some(rt) = resp["refresh_token"].as_str().filter(|r| *r != self.file.refresh_token) {
            self.file.refresh_token = rt.into();
            if let Ok(j) = serde_json::to_string_pretty(&self.file) {
                let _ = std::fs::write(&self.path, j); // keeps the original file mode
            }
        }
        use base64::Engine;
        let claims = self.token.split('.').nth(1).and_then(|p| base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(p).ok());
        if let Some(c) = claims.and_then(|c| serde_json::from_slice::<serde_json::Value>(&c).ok()) {
            self.uid = c["user_id"].as_str().unwrap_or("").into();
            self.name = c["name"].as_str().unwrap_or("").into();
        }
        Ok(self.token.clone())
    }

    /// Find the room for the invite link; returns the gaia room id.
    pub fn find_room(&mut self) -> Result<String, String> {
        let token = self.id_token()?;
        let resp: serde_json::Value = ureq::post("https://api.sowork.com/api/v1/gaia/find")
            .set("source", "web")
            .set("version", CLIENT_VERSION)
            .set("authorization", &format!("Bearer {token}"))
            .send_json(self.file.find.clone())
            .map_err(|e| format!("gaia/find failed: {e}"))?
            .into_json()
            .map_err(|e| e.to_string())?;
        resp["room"]["id"].as_str().map(String::from).ok_or_else(|| "gaia/find returned no room".into())
    }
}

/// Write the session file readable by its owner only.
pub fn save_session(path: &str, file: &SessionFile) -> Result<(), String> {
    use std::io::Write;
    let json = serde_json::to_string_pretty(file).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    let mut f = {
        use std::os::unix::fs::OpenOptionsExt;
        std::fs::OpenOptions::new().write(true).create(true).truncate(true).mode(0o600).open(path).map_err(|e| e.to_string())?
    };
    #[cfg(not(unix))]
    let mut f = std::fs::File::create(path).map_err(|e| e.to_string())?;
    f.write_all(json.as_bytes()).map_err(|e| e.to_string())
}

/// The space (office) the user last signed in to, remembered beside the session so the next sign-in can go straight
/// to it. It holds no credential, so logging out keeps it.
pub fn space_path(session_path: &str) -> std::path::PathBuf {
    std::path::Path::new(session_path).with_file_name(".space.json")
}

pub fn load_space(session_path: &str) -> Option<(String, String)> {
    let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(space_path(session_path)).ok()?).ok()?;
    Some((v["groupId"].as_str()?.to_string(), v["alias"].as_str()?.to_string()))
}

pub fn save_space(session_path: &str, group: &str, alias: &str) {
    let _ = std::fs::write(space_path(session_path), serde_json::json!({"groupId": group, "alias": alias}).to_string());
}

/// Where the sign-in lives by default: `$XDG_CONFIG_HOME/soworker/` (else `~/.config/soworker/`), outside any repository.
pub fn default_path() -> String {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .filter(|v| !v.is_empty())
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| std::path::Path::new(&h).join(".config")))
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    base.join("soworker").join("session.json").to_string_lossy().into_owned()
}

/// Make sure the folder for `path` exists and is private (0700).
pub fn ensure_private_dir(path: &str) {
    if let Some(dir) = std::path::Path::new(path).parent().filter(|d| !d.as_os_str().is_empty()) {
        let _ = std::fs::create_dir_all(dir);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700));
        }
    }
}

/// Move a session an older version left inside the project (`legacy`) to `target`, along with its `.space.json`.
/// Does nothing if `target` already exists. Returns the legacy path that was moved.
pub fn migrate_legacy(target: &str, legacy: &[&str]) -> Option<String> {
    if std::path::Path::new(target).exists() {
        return None;
    }
    let from = legacy.iter().find(|p| std::path::Path::new(p).is_file())?;
    ensure_private_dir(target);
    let file: SessionFile = serde_json::from_str(&std::fs::read_to_string(from).ok()?).ok()?;
    save_session(target, &file).ok()?; // written with mode 600
    if let Some((g, a)) = load_space(from) {
        save_space(target, &g, &a);
        let _ = std::fs::remove_file(space_path(from));
    }
    std::fs::remove_file(from).ok()?;
    Some((*from).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrates_a_legacy_session_to_a_private_location() {
        let base = std::env::temp_dir().join(format!("soworker-mig-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(base.join("repo")).unwrap();
        let legacy = base.join("repo/.session.json").to_string_lossy().into_owned();
        let target = base.join("cfg/soworker/session.json").to_string_lossy().into_owned();
        let file = SessionFile { api_key: "k".into(), refresh_token: "r".into(), find: serde_json::json!({"type": "group"}) };
        save_session(&legacy, &file).unwrap();
        save_space(&legacy, "g", "a");
        assert_eq!(migrate_legacy(&target, &[&legacy]).as_deref(), Some(legacy.as_str()));
        assert!(!std::path::Path::new(&legacy).exists(), "the copy inside the project is gone");
        assert!(!space_path(&legacy).exists());
        let moved: SessionFile = serde_json::from_str(&std::fs::read_to_string(&target).unwrap()).unwrap();
        assert_eq!(moved.refresh_token, "r");
        assert_eq!(load_space(&target), Some(("g".into(), "a".into())));
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(std::fs::metadata(&target).unwrap().permissions().mode() & 0o777, 0o600);
            assert_eq!(std::fs::metadata(base.join("cfg/soworker")).unwrap().permissions().mode() & 0o777, 0o700);
        }
        // a second run must not overwrite an existing target
        save_session(&legacy, &SessionFile { refresh_token: "other".into(), ..file }).unwrap();
        assert!(migrate_legacy(&target, &[&legacy]).is_none());
        let _ = std::fs::remove_dir_all(&base);
    }
}
