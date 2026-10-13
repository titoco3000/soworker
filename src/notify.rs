//! Desktop notifications through `notify-send` (no D-Bus library needed for this): short messages that disappear by
//! themselves, shown over whatever window is in front.
use std::process::{Command, Stdio};

pub const HINT_SECONDS: u32 = 5;

pub fn talk_text(key: &str) -> String {
    format!("Press {key} to talk")
}

/// "Press F1 to talk", gone after a few seconds. A new one replaces the previous instead of stacking.
pub fn talk_hint(key: &str) {
    let text = talk_text(key);
    std::thread::spawn(move || {
        let _ = Command::new("notify-send")
            .args(["--app-name=SoWorker", "--icon=soworker", "--urgency=low"])
            .arg(format!("--expire-time={}", HINT_SECONDS * 1000))
            .args(["--hint=string:x-canonical-private-synchronous:soworker-talk", "SoWorker"])
            .arg(text)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    });
}

/// A short status line ("Live · 2 listening"), replacing the previous one.
pub fn status(text: &str) {
    let text = text.to_string();
    std::thread::spawn(move || {
        let _ = Command::new("notify-send")
            .args(["--app-name=SoWorker", "--icon=soworker", "--urgency=low", "--expire-time=2500"])
            .args(["--hint=string:x-canonical-private-synchronous:soworker-talk", "SoWorker"])
            .arg(text)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn hint_names_the_key() {
        assert_eq!(super::talk_text("F1"), "Press F1 to talk");
    }
}
