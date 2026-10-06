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

pub struct Auth {
    path: String,
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
