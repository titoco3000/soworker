//! Signing in a phone from the desktop (docs/android-plan.md, 3.2): Google refuses its sign-in inside an app's
//! WebView, so the desktop, already signed in, shows its session as a QR code and the phone reads it.
//!
//! The code is the session itself (a long-lived credential), so it is only made on request, says what it is, and
//! expires: the phone refuses a code older than `TTL` seconds. It is not encrypted: whatever key would decrypt it
//! would have to travel in the same picture, which adds nothing; the short life and showing it only on a click are
//! what protect it.
use base64::Engine;

/// How long a code is good for, in seconds (the desktop hides it after this too).
pub const TTL: u64 = 60;
/// Allowed difference between the two clocks.
const SKEW: u64 = 120;
const PREFIX: &str = "soworker-pair:1:";

#[derive(Debug, Clone, PartialEq)]
pub struct Pairing {
    pub api_key: String,
    pub refresh_token: String,
    pub group: String,
    pub alias: String,
}

pub fn unix_now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

/// The same, with fractions.
pub fn unix_secs() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0.0, |d| d.as_secs_f64())
}

/// The text of the QR code for this session, made at `now` (Unix seconds).
pub fn encode(p: &Pairing, now: u64) -> String {
    let v = serde_json::json!({"k": p.api_key, "r": p.refresh_token, "g": p.group, "a": p.alias, "t": now});
    format!("{PREFIX}{}", base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(v.to_string()))
}

/// Read a QR code's text at `now`: the session in it, or why it cannot be used.
pub fn open(text: &str, now: u64) -> Result<Pairing, String> {
    let body = text.trim().strip_prefix(PREFIX).ok_or("This QR code is not a SoWorker sign-in code.")?;
    let raw = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(body).map_err(|_| "The sign-in code is damaged.")?;
    let v: serde_json::Value = serde_json::from_slice(&raw).map_err(|_| "The sign-in code is damaged.")?;
    let field = |k: &str| v[k].as_str().filter(|s| !s.is_empty()).map(String::from).ok_or("The sign-in code is incomplete.");
    let made = v["t"].as_u64().ok_or("The sign-in code is incomplete.")?;
    if now > made + TTL + SKEW {
        return Err("This sign-in code has expired. Show a new one on the computer.".into());
    }
    if made > now + SKEW {
        return Err("This sign-in code comes from the future: check the clocks of both devices.".into());
    }
    Ok(Pairing { api_key: field("k")?, refresh_token: field("r")?, group: field("g")?, alias: field("a")? })
}

/// The QR code for `text`: its width in modules and the modules row by row (true = dark). Medium error correction
/// (the session is about 1 kB, so a bigger level would make the code hard to read off a screen).
#[cfg(feature = "gui")]
pub fn qr(text: &str) -> Option<(usize, Vec<bool>)> {
    let code = qrcodegen::QrCode::encode_text(text, qrcodegen::QrCodeEcc::Medium).ok()?;
    let n = code.size();
    Some((n as usize, (0..n).flat_map(|y| (0..n).map(move |x| (x, y))).map(|(x, y)| code.get_module(x, y)).collect()))
}

/// The text of the first readable QR code in a greyscale picture (`w` x `h`, one byte per pixel).
#[cfg(any(target_os = "android", test))]
pub fn read_qr(luma: &[u8], w: usize, h: usize) -> Option<String> {
    let mut img = rqrr::PreparedImage::prepare_from_greyscale(w, h, |x, y| luma.get(y * w + x).copied().unwrap_or(255));
    img.detect_grids().into_iter().find_map(|g| g.decode().ok().map(|(_, text)| text))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p() -> Pairing {
        Pairing { api_key: "AIza".into(), refresh_token: "AMf-long".into(), group: "g1".into(), alias: "office".into() }
    }

    #[test]
    fn a_code_round_trips_while_fresh() {
        let code = encode(&p(), 1_000_000);
        assert!(code.starts_with("soworker-pair:1:"));
        assert_eq!(open(&code, 1_000_030), Ok(p()));
        assert_eq!(open(&format!(" {code}\n"), 1_000_000), Ok(p()), "whitespace around it is fine");
    }

    #[test]
    fn old_foreign_and_damaged_codes_are_refused() {
        let code = encode(&p(), 1_000_000);
        assert!(open(&code, 1_000_000 + TTL + SKEW + 1).unwrap_err().contains("expired"));
        assert!(open(&code, 1_000_000 - SKEW - 1).unwrap_err().contains("future"));
        assert!(open("https://example.com", 0).unwrap_err().contains("not a SoWorker"));
        assert!(open("soworker-pair:1:!!!", 0).unwrap_err().contains("damaged"));
        let partial = format!("soworker-pair:1:{}", base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(r#"{"k":"a","t":5}"#));
        assert!(open(&partial, 5).unwrap_err().contains("incomplete"));
    }

    #[cfg(feature = "gui")]
    #[test]
    fn a_shown_code_reads_back_into_the_session() {
        let code = encode(&Pairing { refresh_token: "AMf-".repeat(60), ..p() }, 1_000_000);
        let (n, dark) = qr(&code).expect("fits in a QR code");
        // drawn as on screen: 4 pixels a module, a quiet zone of 4 modules
        let (px, quiet) = (4, 4);
        let side = (n + 2 * quiet) * px;
        let luma: Vec<u8> = (0..side * side)
            .map(|i| {
                let (x, y) = ((i % side) / px, (i / side) / px);
                let inside = x >= quiet && y >= quiet && x < n + quiet && y < n + quiet;
                if inside && dark[(y - quiet) * n + x - quiet] { 0 } else { 255 }
            })
            .collect();
        let text = read_qr(&luma, side, side).expect("the code reads back");
        assert_eq!(text, code);
        assert_eq!(open(&text, 1_000_010).unwrap().refresh_token, "AMf-".repeat(60));
    }
}
