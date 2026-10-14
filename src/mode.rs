//! Which client to start (terminal or window), remembering the last one used, and hopping between the two.
//! Started from a terminal you get the terminal client; started from a launcher icon (no terminal) you get whichever
//! you used last, and the terminal client is opened inside a terminal emulator.
use std::process::{Command, Stdio};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiMode {
    Tui,
    Gui,
}

impl UiMode {
    pub fn built(self) -> bool {
        match self {
            UiMode::Tui => cfg!(feature = "tui"),
            UiMode::Gui => cfg!(feature = "gui"),
        }
    }

    fn flag(self) -> &'static str {
        match self {
            UiMode::Tui => "--tui",
            UiMode::Gui => "--gui",
        }
    }

    fn other(self) -> UiMode {
        match self {
            UiMode::Tui => UiMode::Gui,
            UiMode::Gui => UiMode::Tui,
        }
    }
}

fn file() -> std::path::PathBuf {
    let session = crate::session::default_path();
    std::path::Path::new(&session).parent().map_or_else(|| "last-ui".into(), |d| d.join("last-ui"))
}

pub fn last() -> Option<UiMode> {
    match std::fs::read_to_string(file()).ok()?.trim() {
        "tui" => Some(UiMode::Tui),
        "gui" => Some(UiMode::Gui),
        _ => None,
    }
}

pub fn remember(m: UiMode) {
    let f = file();
    if let Some(d) = f.parent() {
        let _ = std::fs::create_dir_all(d);
    }
    let _ = std::fs::write(f, if m == UiMode::Tui { "tui" } else { "gui" });
}

pub fn in_terminal() -> bool {
    // SAFETY: isatty only inspects a descriptor number.
    unsafe { libc::isatty(0) == 1 && libc::isatty(1) == 1 }
}

/// What to start: an explicit `--tui`/`--gui`, else the terminal client when run from a terminal, else the one used
/// last (the window when there is no history). Only built-in clients are considered.
pub fn choose(args: &[String], terminal: bool, last: Option<UiMode>) -> UiMode {
    let want = if args.iter().any(|a| a == "--gui") {
        UiMode::Gui
    } else if args.iter().any(|a| a == "--tui") || terminal {
        UiMode::Tui
    } else {
        last.unwrap_or(UiMode::Gui)
    };
    if want.built() { want } else { want.other() }
}

fn exe() -> Option<String> {
    std::env::current_exe().ok().map(|p| p.to_string_lossy().into_owned())
}

/// Start another copy in `mode` after a short pause (so this one has let go of the sockets), detached from us and from
/// our terminal. Pass `terminal_ok` to wrap the terminal client in a terminal emulator.
pub fn relaunch(mode: UiMode, forward: &[String]) -> Result<(), String> {
    let exe = exe().ok_or("cannot find my own binary")?;
    let mut args: Vec<String> = forward.iter().filter(|a| !matches!(a.as_str(), "--gui" | "--tui")).cloned().collect();
    args.insert(0, mode.flag().into());
    let mut cmd = match mode {
        UiMode::Gui => {
            let mut c = Command::new("sh");
            c.args(["-c", "sleep 0.5; exec \"$@\""]).arg("soworker").arg(&exe).args(&args);
            c
        }
        UiMode::Tui => terminal_command(&exe, &args).ok_or("no usable terminal emulator: set `terminal` in ~/.config/soworker/settings.json (the command, with {} where the soworker command goes)")?,
    };
    use std::os::unix::process::CommandExt;
    cmd.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).process_group(0).spawn().map(drop).map_err(|e| format!("cannot start {}: {e}", mode.flag()))
}

/// The command that runs `exe args` inside a terminal emulator: the `terminal` setting (`{}` stands for the command),
/// `$TERMINAL`, then the usual suspects.
fn terminal_command(exe: &str, args: &[String]) -> Option<Command> {
    let line = format!("{exe} {}", args.join(" "));
    let setting = std::fs::read_to_string(std::path::Path::new(&crate::session::default_path()).with_file_name("settings.json"))
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .and_then(|v| v["terminal"].as_str().map(String::from));
    if let Some(t) = setting {
        let t = if t.contains("{}") { t.replace("{}", &line) } else { format!("{t} {line}") };
        let mut c = Command::new("sh");
        c.args(["-c", &t]);
        return Some(c);
    }
    let on_path = |b: &str| std::env::var_os("PATH").is_some_and(|p| std::env::split_paths(&p).any(|d| d.join(b).is_file()));
    // (binary, what comes before the command). cosmic-term 1.3 lists no option for it in --help but accepts `-e`
    // (checked by hand); a bare command is ignored.
    let known: [(&str, &[&str]); 11] = [
        ("xdg-terminal-exec", &[]),
        ("cosmic-term", &["-e"]),
        ("gnome-terminal", &["--"]),
        ("konsole", &["-e"]),
        ("kitty", &[]),
        ("alacritty", &["-e"]),
        ("foot", &[]),
        ("wezterm", &["start", "--"]),
        ("ptyxis", &["--"]),
        ("xterm", &["-e"]),
        ("x-terminal-emulator", &["-e"]),
    ];
    let env = std::env::var("TERMINAL").ok().filter(|t| !t.is_empty());
    let (bin, pre): (String, &[&str]) = match env {
        Some(t) => (t, &["-e"]),
        None => known.iter().find(|(b, _)| on_path(b)).map(|(b, p)| (b.to_string(), *p))?,
    };
    let mut c = Command::new(bin);
    c.args(pre).arg(exe).args(args);
    Some(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a(s: &[&str]) -> Vec<String> {
        s.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn flags_win_then_the_terminal_then_the_last_one() {
        let both = cfg!(all(feature = "tui", feature = "gui"));
        if !both {
            return;
        }
        assert_eq!(choose(&a(&["--gui"]), true, None), UiMode::Gui);
        assert_eq!(choose(&a(&["--tui"]), false, Some(UiMode::Gui)), UiMode::Tui);
        assert_eq!(choose(&a(&[]), true, Some(UiMode::Gui)), UiMode::Tui, "run from a terminal: terminal client");
        assert_eq!(choose(&a(&[]), false, Some(UiMode::Tui)), UiMode::Tui, "from an icon: as last time");
        assert_eq!(choose(&a(&[]), false, Some(UiMode::Gui)), UiMode::Gui);
        assert_eq!(choose(&a(&[]), false, None), UiMode::Gui);
    }

    #[test]
    fn a_missing_client_falls_back_to_the_one_built() {
        if cfg!(all(feature = "tui", not(feature = "gui"))) {
            assert_eq!(choose(&a(&["--gui"]), false, None), UiMode::Tui);
        }
    }
}
