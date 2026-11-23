//! Desktop integration: on Wayland (and in launchers) the app icon comes from a `.desktop` entry whose name matches the
//! window's app id (`soworker`), not from the window. On start we make sure the current user has that entry and icon,
//! pointing at the running binary. Only files under `~/.local/share` are written, and only when they differ.
#![cfg_attr(windows, allow(dead_code))]
use std::path::PathBuf;

const ENTRY: &str = include_str!("../assets/soworker.desktop");
const ICON: &[u8] = include_bytes!("../assets/icon.png");

fn data_dir() -> Option<PathBuf> {
    std::env::var_os("XDG_DATA_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
}

/// The entry text for a binary at `exe` with its icon file at `icon`. Paths with spaces are quoted as the Desktop Entry
/// spec asks. The icon is given by its full path: that works in every launcher, with no icon-theme lookup or cache.
pub fn entry_for(exe: &str, icon: &str) -> String {
    let exec = if exe.contains(' ') { format!("\"{exe}\"") } else { exe.to_string() };
    // Terminal=false on purpose: the app opens a terminal itself when the terminal client is the one to start (see mode.rs)
    ENTRY.replace("@EXEC@", &exec).replace("@ICON@", icon)
}

fn write_if_changed(path: &std::path::Path, bytes: &[u8]) -> bool {
    if std::fs::read(path).is_ok_and(|old| old == bytes) {
        return false;
    }
    path.parent().is_some_and(|d| std::fs::create_dir_all(d).is_ok()) && std::fs::write(path, bytes).is_ok()
}

/// Windows takes the icon from the executable and needs no launcher entry.
#[cfg(windows)]
pub fn ensure() {}

#[cfg(not(windows))]
pub fn ensure() {
    let (Some(data), Ok(exe)) = (data_dir(), std::env::current_exe()) else { return };
    let icon = data.join("icons/hicolor/256x256/apps/soworker.png");
    let entry = data.join("applications/soworker.desktop");
    let icon_new = write_if_changed(&icon, ICON);
    let entry_new = write_if_changed(&entry, entry_for(&exe.to_string_lossy(), &icon.to_string_lossy()).as_bytes());
    if icon_new || entry_new {
        println!("installed the SoWorker launcher entry and icon under {}", data.display());
        // best effort: let the desktop notice right away
        let _ = std::process::Command::new("gtk-update-icon-cache").args(["-q", "-t"]).arg(data.join("icons/hicolor")).status();
        let _ = std::process::Command::new("update-desktop-database").arg(data.join("applications")).status();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn uris_escape_spaces_and_accents() {
        assert_eq!(super::file_uri(std::path::Path::new("/r/Escritório 1.mp4")), "file:///r/Escrit%C3%B3rio%201.mp4");
    }

    #[test]
    fn entry_points_at_the_binary_and_matches_the_app_id() {
        let e = entry_for("/home/me/soworker/target/release/soworker", "/home/me/.local/share/icons/soworker.png");
        assert!(e.contains("Exec=/home/me/soworker/target/release/soworker\n"));
        assert!(e.contains("Icon=/home/me/.local/share/icons/soworker.png\n") && e.contains("StartupWMClass=soworker"));
        assert!(entry_for("/a b/soworker", "/i.png").contains("Exec=\"/a b/soworker\"\n"));
    }
}

/// Opens the file manager on the folder holding `path`, with `path` selected (a finished recording). Best effort, in
/// the background: the freedesktop FileManager1 interface (Nautilus, Dolphin, Nemo, Thunar...), else the folder alone
/// through `xdg-open`; Explorer on Windows. Nothing on Android or in tests.
pub fn reveal(path: &std::path::Path) {
    if cfg!(any(test, target_os = "android")) {
        return;
    }
    let path = path.to_path_buf();
    std::thread::spawn(move || {
        #[cfg(windows)]
        {
            let mut arg = std::ffi::OsString::from("/select,");
            arg.push(&path);
            let _ = std::process::Command::new("explorer").arg(arg).spawn();
        }
        #[cfg(not(windows))]
        {
            if let Err(e) = show_items(&path) {
                eprintln!("reveal: {e}; opening the folder instead");
                if let Some(dir) = path.parent() {
                    let _ = std::process::Command::new("xdg-open").arg(dir).stdin(std::process::Stdio::null()).spawn();
                }
            }
        }
    });
}

#[cfg(all(not(windows), feature = "portal"))]
fn show_items(path: &std::path::Path) -> Result<(), String> {
    let es = |e: zbus::Error| e.to_string();
    let conn = zbus::blocking::Connection::session().map_err(es)?;
    let uri = file_uri(&path.canonicalize().unwrap_or_else(|_| path.to_path_buf()));
    conn.call_method(Some("org.freedesktop.FileManager1"), "/org/freedesktop/FileManager1", Some("org.freedesktop.FileManager1"), "ShowItems", &(vec![uri.as_str()], ""))
        .map_err(es)?;
    Ok(())
}

#[cfg(all(not(windows), not(feature = "portal")))]
fn show_items(_path: &std::path::Path) -> Result<(), String> {
    Err("built without D-Bus".into())
}

/// `file://` URI of an absolute path, bytes outside the unreserved set percent-encoded.
#[cfg(unix)]
pub fn file_uri(path: &std::path::Path) -> String {
    use std::os::unix::ffi::OsStrExt;
    let mut s = String::from("file://");
    for &b in path.as_os_str().as_bytes() {
        if b.is_ascii_alphanumeric() || b"/-._~".contains(&b) {
            s.push(b as char);
        } else {
            s.push_str(&format!("%{b:02X}"));
        }
    }
    s
}
