//! Changing the profile photo (docs/hand-desk-photo.md): pick a picture, crop it square and shrink it, encode it as
//! WebP, upload it to S3 with the presigned POST SoWork's API hands out, then point the user's `uploadedPhotoURL` at it.
use crate::session::CLIENT_VERSION;
use serde_json::{json, Value};
use std::time::Duration;

/// Side of the square we upload, in pixels: photos are only ever shown small.
const SIDE: u32 = 256;

/// Asks the user for a picture with the desktop's file picker. `Ok(None)` = they cancelled.
pub fn pick() -> Result<Option<std::path::PathBuf>, String> {
    #[cfg(windows)]
    return win::pick();
    #[cfg(not(windows))]
    {
        #[cfg(all(feature = "portal", target_os = "linux"))]
        match portal_pick() {
            Ok(p) => return Ok(p),
            Err(e) => eprintln!("photo: no file chooser portal ({e}), trying zenity / kdialog"),
        }
        command_pick()
    }
}

/// The xdg-desktop-portal FileChooser: works on every desktop that has a portal, sandboxed or not.
#[cfg(all(feature = "portal", target_os = "linux"))]
fn portal_pick() -> Result<Option<std::path::PathBuf>, String> {
    use std::collections::HashMap;
    use zbus::blocking::{Connection, Proxy};
    use zbus::zvariant::{OwnedValue, Value};
    let es = |e: zbus::Error| e.to_string();
    const DEST: &str = "org.freedesktop.portal.Desktop";
    const PATH: &str = "/org/freedesktop/portal/desktop";
    let conn = Connection::session().map_err(es)?;
    let sender = conn.unique_name().ok_or("no bus name")?.as_str().trim_start_matches(':').replace('.', "_");
    let token = "soworker_photo";
    // the answer comes as a `Response` signal on a request object whose path we know before asking
    let req = Proxy::new(&conn, DEST, format!("{PATH}/request/{sender}/{token}"), "org.freedesktop.portal.Request").map_err(es)?;
    let mut sig = req.receive_signal("Response").map_err(es)?;
    let chooser = Proxy::new(&conn, DEST, PATH, "org.freedesktop.portal.FileChooser").map_err(es)?;
    let filters = vec![("Images", vec![(1u32, "image/png"), (1, "image/jpeg"), (1, "image/webp")])];
    let opts: HashMap<&str, Value> = HashMap::from([("handle_token", Value::from(token)), ("modal", Value::from(true)), ("filters", Value::from(filters))]);
    chooser.call_method("OpenFile", &("", "Choose a profile photo", opts)).map_err(es)?;
    let msg = sig.next().ok_or("portal closed")?;
    let (code, results): (u32, HashMap<String, OwnedValue>) = msg.body().deserialize().map_err(es)?;
    if code != 0 {
        return Ok(None); // 1 = cancelled
    }
    let uris: Vec<String> = results.get("uris").ok_or("no file in the answer")?.try_clone().map_err(|e| e.to_string())?.try_into().map_err(|e: zbus::zvariant::Error| e.to_string())?;
    Ok(uris.first().and_then(|u| file_path(u)))
}

/// `file:///home/a/My%20Photo.png` -> `/home/a/My Photo.png`.
#[cfg_attr(not(all(feature = "portal", target_os = "linux")), allow(dead_code))]
fn file_path(uri: &str) -> Option<std::path::PathBuf> {
    let raw = uri.strip_prefix("file://")?.as_bytes();
    let mut out = Vec::with_capacity(raw.len());
    let mut i = 0;
    while i < raw.len() {
        if raw[i] == b'%' && i + 2 < raw.len() {
            if let Ok(b) = u8::from_str_radix(std::str::from_utf8(&raw[i + 1..i + 3]).ok()?, 16) {
                out.push(b);
                i += 3;
                continue;
            }
        }
        out.push(raw[i]);
        i += 1;
    }
    #[cfg(unix)]
    return Some(<std::ffi::OsString as std::os::unix::ffi::OsStringExt>::from_vec(out).into());
    #[cfg(not(unix))]
    Some(String::from_utf8(out).ok()?.into())
}

/// Without a portal: zenity (GNOME and most others), else kdialog (KDE).
#[cfg(not(windows))]
fn command_pick() -> Result<Option<std::path::PathBuf>, String> {
    use std::process::Command;
    let attempts: [(&str, &[&str]); 2] = [
        ("zenity", &["--file-selection", "--title=Choose a profile photo", "--file-filter=Images | *.png *.jpg *.jpeg *.webp"]),
        ("kdialog", &["--getopenfilename", ".", "Images (*.png *.jpg *.jpeg *.webp)"]),
    ];
    for (cmd, args) in attempts {
        match Command::new(cmd).args(args).output() {
            Ok(o) if o.status.success() => {
                let p = String::from_utf8_lossy(&o.stdout).trim().to_string();
                return Ok((!p.is_empty()).then(|| p.into()));
            }
            Ok(_) => return Ok(None), // the dialog was there and the user cancelled it
            Err(_) => continue,
        }
    }
    Err("no file picker found (install xdg-desktop-portal, zenity or kdialog)".into())
}

#[cfg(windows)]
mod win {
    use windows_sys::Win32::UI::Controls::Dialogs::{GetOpenFileNameW, OFN_FILEMUSTEXIST, OFN_NOCHANGEDIR, OFN_PATHMUSTEXIST, OPENFILENAMEW};

    pub fn pick() -> Result<Option<std::path::PathBuf>, String> {
        use std::os::windows::ffi::OsStringExt;
        let wide = |s: &str| s.encode_utf16().collect::<Vec<u16>>();
        // pairs of description and pattern, each NUL-terminated, the list ending with an extra NUL
        let filter = wide("Images\0*.png;*.jpg;*.jpeg;*.webp\0\0");
        let title = wide("Choose a profile photo\0");
        let mut file = vec![0u16; 1024];
        // SAFETY: an all-zero OPENFILENAMEW is valid (null pointers, no flags); the buffers outlive the call.
        let mut ofn: OPENFILENAMEW = unsafe { std::mem::zeroed() };
        ofn.lStructSize = std::mem::size_of::<OPENFILENAMEW>() as u32;
        ofn.lpstrFilter = filter.as_ptr();
        ofn.lpstrFile = file.as_mut_ptr();
        ofn.nMaxFile = file.len() as u32;
        ofn.lpstrTitle = title.as_ptr();
        ofn.Flags = OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST | OFN_NOCHANGEDIR;
        // SAFETY: `ofn` is filled in as the API documents; it blocks until the dialog closes.
        if unsafe { GetOpenFileNameW(&mut ofn) } == 0 {
            return Ok(None); // cancelled (or failed, which shows its own message)
        }
        let len = file.iter().position(|&c| c == 0).unwrap_or(file.len());
        Ok(Some(std::ffi::OsString::from_wide(&file[..len]).into()))
    }
}

/// Any PNG, JPEG or WebP -> a centred square of at most `SIDE` pixels, as lossless WebP.
pub fn prepare(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let img = image::load_from_memory(bytes).map_err(|e| format!("not a picture this client can read: {e}"))?.to_rgba8();
    let (w, h) = img.dimensions();
    let side = w.min(h);
    let sq = image::imageops::crop_imm(&img, (w - side) / 2, (h - side) / 2, side, side).to_image();
    let sq = if side > SIDE { image::imageops::resize(&sq, SIDE, SIDE, image::imageops::FilterType::Lanczos3) } else { sq };
    let mut out = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(sq).write_to(&mut out, image::ImageFormat::WebP).map_err(|e| e.to_string())?;
    Ok(out.into_inner())
}

fn api(token: &str, path: &str, body: Value) -> Result<Value, String> {
    ureq::post(&format!("https://api.sowork.com/api/v1/{path}"))
        .set("authorization", &format!("Bearer {token}"))
        .set("source", "web")
        .set("version", CLIENT_VERSION)
        .timeout(Duration::from_secs(20))
        .send_json(body)
        .map_err(|e| format!("{path}: {e}"))?
        .into_json()
        .map_err(|e| format!("{path}: {e}"))
}

/// The presigned POST as multipart/form-data: `Content-Type`, then every field S3 gave us, then the file (S3 wants
/// the file last).
fn multipart(fields: &serde_json::Map<String, Value>, webp: &[u8], boundary: &str) -> Vec<u8> {
    let mut body = Vec::new();
    let mut field = |name: &str, value: &str| {
        body.extend_from_slice(format!("--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n").as_bytes());
    };
    field("Content-Type", "image/webp");
    for (k, v) in fields {
        field(k, v.as_str().unwrap_or_default());
    }
    body.extend_from_slice(format!("--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"profile.webp\"\r\nContent-Type: image/webp\r\n\r\n").as_bytes());
    body.extend_from_slice(webp);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
    body
}

/// What `info_update` needs after the change: the user's name, role and affiliation (from `user/update`'s answer).
pub struct Changed {
    pub url: String,
    pub name: String,
    pub role: String,
    pub affiliation: String,
}

/// The three calls SoWork's client makes. Returns the new photo URL (with the `?v=` that busts caches).
pub fn upload(token: &str, uid: &str, group_id: &str, webp: &[u8]) -> Result<Changed, String> {
    let grant = api(token, "user/getUploadProfilePhotoURL", json!({"type": "profile_photo", "fileType": "image/webp"}))?;
    let (to, fields) = (grant["upload"]["url"].as_str().ok_or("no upload URL in the answer")?, grant["upload"]["fields"].as_object().ok_or("no upload fields in the answer")?);
    let download = grant["download"]["url"].as_str().ok_or("no download URL in the answer")?;
    let mut b = [0u8; 12];
    let _ = getrandom::getrandom(&mut b);
    let boundary = format!("----SoWorker{}", b.iter().map(|x| format!("{x:02x}")).collect::<String>());
    let sent = ureq::post(to)
        .set("content-type", &format!("multipart/form-data; boundary={boundary}"))
        .timeout(Duration::from_secs(60))
        .send_bytes(&multipart(fields, webp, &boundary));
    if let Err(e) = sent {
        let why = match e {
            ureq::Error::Status(code, r) => format!("HTTP {code} {}", r.into_string().unwrap_or_default().chars().take(200).collect::<String>()),
            e => e.to_string(),
        };
        return Err(format!("photo upload: {why}"));
    }
    let v = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis());
    let url = format!("{download}?v={v}");
    let user = api(token, "user/update", json!({"userId": uid, "groupId": group_id, "data": {"uploadedPhotoURL": url}}))?;
    let s = |k: &str| user[k].as_str().unwrap_or_default().to_string();
    Ok(Changed { url, name: s("name"), role: s("role"), affiliation: s("affiliation") })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_picture_becomes_a_small_square_webp() {
        let img = image::RgbaImage::from_fn(900, 600, |x, _| image::Rgba([(x % 256) as u8, 40, 200, 255]));
        let mut png = std::io::Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(img).write_to(&mut png, image::ImageFormat::Png).unwrap();
        let webp = prepare(&png.into_inner()).unwrap();
        assert_eq!(&webp[..4], b"RIFF");
        assert_eq!(&webp[8..12], b"WEBP");
        assert_eq!(image::load_from_memory(&webp).unwrap().to_rgba8().dimensions(), (SIDE, SIDE));
        assert!(prepare(b"not a picture").is_err());
    }

    #[test]
    fn the_form_puts_the_file_last() {
        let fields = json!({"key": "profile_photos/u/profile.webp", "acl": "public-read", "Policy": "p"});
        let body = String::from_utf8_lossy(&multipart(fields.as_object().unwrap(), b"DATA", "B")).into_owned();
        assert!(body.starts_with("--B\r\nContent-Disposition: form-data; name=\"Content-Type\"\r\n\r\nimage/webp\r\n"));
        let (key, file) = (body.find("name=\"key\"").unwrap(), body.find("name=\"file\"").unwrap());
        assert!(key < file && body.ends_with("DATA\r\n--B--\r\n"));
    }

    #[test]
    fn portal_uris_become_paths() {
        assert_eq!(file_path("file:///home/a/My%20Photo.png"), Some("/home/a/My Photo.png".into()));
        assert_eq!(file_path("https://x"), None);
    }
}
