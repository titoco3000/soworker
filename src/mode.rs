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

#[cfg(unix)]
pub fn in_terminal() -> bool {
    // SAFETY: isatty only inspects a descriptor number.
    unsafe { libc::isatty(0) == 1 && libc::isatty(1) == 1 }
}

/// A console Windows opened just for us (started from Explorer or the Start menu) does not count: we are its only
/// process. Run from a shell, the shell shares it.
#[cfg(windows)]
pub fn in_terminal() -> bool {
    use std::io::IsTerminal;
    std::io::stdin().is_terminal() && std::io::stdout().is_terminal() && !own_console()
}

#[cfg(windows)]
fn own_console() -> bool {
    let mut ids = [0u32; 2];
    // SAFETY: the buffer holds as many ids as we say.
    unsafe { windows_sys::Win32::System::Console::GetConsoleProcessList(ids.as_mut_ptr(), ids.len() as u32) == 1 }
}

/// Whether the terminal client can draw here: on Windows a console of our own will do.
pub fn has_terminal() -> bool {
    #[cfg(windows)]
    {
        use std::io::IsTerminal;
        std::io::stdin().is_terminal() && std::io::stdout().is_terminal()
    }
    #[cfg(unix)]
    in_terminal()
}

/// The window client needs no console: close the one Windows opened for us when started from an icon. Its standard
/// handles go with it: children (the browser for signing in, the terminal client) must not inherit them (spawning
/// with them fails with os error 50), and diagnostics go to the log file instead, as nobody would see them.
pub fn drop_console() {
    #[cfg(windows)]
    if own_console() {
        use windows_sys::Win32::System::Console::{FreeConsole, SetStdHandle, STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
        // SAFETY: detaching from the console has no preconditions; it closes once no process uses it. A null standard
        // handle is what a process without a console has.
        unsafe {
            FreeConsole();
            for h in [STD_INPUT_HANDLE, STD_OUTPUT_HANDLE, STD_ERROR_HANDLE] {
                SetStdHandle(h, std::ptr::null_mut());
            }
        }
        log_to_file();
    }
}

/// Windows: a console made for us (the terminal client started by the window client) may come without standard
/// handles pointing at it; take its input and screen directly.
pub fn adopt_console() {
    #[cfg(windows)]
    {
        use std::io::IsTerminal;
        use std::os::windows::io::IntoRawHandle;
        use windows_sys::Win32::System::Console::{GetConsoleWindow, SetStdHandle, STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
        // SAFETY: only asks whether this process has a console window.
        if unsafe { GetConsoleWindow() }.is_null() || (std::io::stdin().is_terminal() && std::io::stdout().is_terminal()) {
            return;
        }
        let open = |name: &str| std::fs::OpenOptions::new().read(true).write(true).open(name).ok().map(IntoRawHandle::into_raw_handle);
        if let (Some(input), Some(output)) = (open("CONIN$"), open("CONOUT$")) {
            // SAFETY: both handles are open and stay open (ownership goes to the process' standard handles).
            unsafe {
                SetStdHandle(STD_INPUT_HANDLE, input);
                SetStdHandle(STD_OUTPUT_HANDLE, output);
                SetStdHandle(STD_ERROR_HANDLE, output);
            }
        }
    }
}

/// Everything that goes to stderr (a lot does) goes to a log file instead, so it cannot scribble over the screen.
#[cfg(unix)]
#[cfg_attr(not(feature = "tui"), allow(dead_code))]
pub fn log_to_file() {
    use std::os::fd::AsRawFd;
    let base = std::env::var_os("XDG_STATE_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("HOME").map(|h| std::path::Path::new(&h).join(".local/state")));
    let dir = base.map_or_else(|| std::path::PathBuf::from("/nonexistent"), |b| b.join("soworker"));
    let _ = std::fs::create_dir_all(&dir);
    // no writable log: drop the noise rather than let it scribble over the screen
    let file = std::fs::OpenOptions::new().create(true).append(true).open(dir.join("soworker.log")).or_else(|_| std::fs::OpenOptions::new().write(true).open("/dev/null"));
    if let Ok(f) = file {
        // SAFETY: both descriptors are open; dup2 just points fd 2 at the file.
        unsafe { libc::dup2(f.as_raw_fd(), 2) };
    }
}

/// Windows: `%LOCALAPPDATA%\soworker\soworker.log`. Rust's stderr looks the standard handle up on every write.
#[cfg(windows)]
pub fn log_to_file() {
    use std::os::windows::io::IntoRawHandle;
    use windows_sys::Win32::System::Console::{SetStdHandle, STD_ERROR_HANDLE};
    let dir = std::env::var_os("LOCALAPPDATA").map_or_else(std::env::temp_dir, std::path::PathBuf::from).join("soworker");
    let _ = std::fs::create_dir_all(&dir);
    let file = std::fs::OpenOptions::new().create(true).append(true).open(dir.join("soworker.log")).or_else(|_| std::fs::OpenOptions::new().write(true).open("NUL"));
    if let Ok(f) = file {
        // SAFETY: the handle is open and stays open (ownership goes to the process' standard error).
        unsafe { SetStdHandle(STD_ERROR_HANDLE, f.into_raw_handle()) };
    }
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

/// The arguments for starting `mode`: its flag and what is worth keeping from ours.
fn next_args(mode: UiMode, forward: &[String]) -> Vec<String> {
    let mut args = client_args(forward);
    args.insert(0, mode.flag().into());
    args
}

/// The arguments worth handing to the next client: not the old client's own flag.
fn client_args(forward: &[String]) -> Vec<String> {
    forward.iter().filter(|a| !matches!(a.as_str(), "--gui" | "--tui")).cloned().collect()
}

fn exe() -> Option<String> {
    std::env::current_exe().ok().map(|p| p.to_string_lossy().into_owned())
}

/// Start another copy in `mode` after a short pause (so this one has let go of the sockets), detached from us and from
/// our terminal. The terminal client is wrapped in a terminal emulator.
#[cfg(unix)]
pub fn relaunch(mode: UiMode, forward: &[String]) -> Result<(), String> {
    let exe = exe().ok_or("cannot find my own binary")?;
    let args = next_args(mode, forward);
    let mut cmd = match mode {
        UiMode::Gui => {
            let mut c = Command::new("sh");
            c.args(["-c", "sleep 0.5; exec \"$@\""]).arg("soworker").arg(&exe).args(&args);
            c
        }
        UiMode::Tui => terminal_command(&exe, &args).ok_or("no usable terminal emulator: set `terminal` in ~/.config/soworker/settings.json (the command, with {} where the soworker command goes)")?,
    };
    use std::os::unix::process::CommandExt;
    // Nothing of ours may live on in the next client: the camera (v4l opens it without O_CLOEXEC) would stay busy
    // in it, unused, and its own camera would fail with EBUSY.
    // SAFETY: runs in the child between fork and exec, and only makes async-signal-safe system calls.
    unsafe { cmd.pre_exec(close_inherited) };
    cmd.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).process_group(0).spawn().map(drop).map_err(|e| format!("cannot start {}: {e}", mode.flag()))
}

/// In a forked child: close every descriptor but standard input, output and error.
#[cfg(unix)]
fn close_inherited() -> std::io::Result<()> {
    #[cfg(target_os = "linux")]
    // SAFETY: close_range only closes descriptors.
    if unsafe { libc::syscall(libc::SYS_close_range, 3u32, u32::MAX, 0u32) } == 0 {
        return Ok(());
    }
    // older kernels (before 5.9), other systems: one by one
    // SAFETY: sysconf and close are async-signal-safe; closing a descriptor that is not open is harmless.
    let max = unsafe { libc::sysconf(libc::_SC_OPEN_MAX) }.clamp(256, 65536) as i32;
    for fd in 3..max {
        unsafe { libc::close(fd) };
    }
    Ok(())
}

/// Windows: the terminal client gets a console of its own (shown in the default terminal app), the window client none.
/// The pause is asked for through `SOWORKER_START_DELAY_MS` (see `main`), as there is no `sh` to sleep in.
#[cfg(windows)]
pub fn relaunch(mode: UiMode, forward: &[String]) -> Result<(), String> {
    use std::os::windows::process::CommandExt;
    use windows_sys::Win32::System::Threading::{CREATE_NEW_CONSOLE, CREATE_NEW_PROCESS_GROUP, DETACHED_PROCESS};
    let exe = exe().ok_or("cannot find my own binary")?;
    let args = next_args(mode, forward);
    let setting = terminal_setting().filter(|_| mode == UiMode::Tui);
    let mut cmd = match setting {
        Some(t) => {
            let line = format!("\"{exe}\" {}", args.join(" "));
            let mut c = Command::new("cmd");
            c.arg("/C").raw_arg(if t.contains("{}") { t.replace("{}", &line) } else { format!("{t} {line}") });
            c.creation_flags(DETACHED_PROCESS);
            c
        }
        None => {
            let mut c = Command::new(&exe);
            c.args(&args).env("SOWORKER_START_DELAY_MS", "500");
            c.creation_flags(if mode == UiMode::Tui { CREATE_NEW_CONSOLE } else { DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP });
            c
        }
    };
    if mode != UiMode::Tui {
        cmd.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    }
    cmd.spawn().map(drop).map_err(|e| format!("cannot start {}: {e}", mode.flag()))
}

/// The `terminal` setting of settings.json: the command that runs another inside a terminal, `{}` standing for it.
fn terminal_setting() -> Option<String> {
    std::fs::read_to_string(std::path::Path::new(&crate::session::default_path()).with_file_name("settings.json"))
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .and_then(|v| v["terminal"].as_str().map(String::from))
}

/// The command that runs `exe args` inside a terminal emulator: the `terminal` setting (`{}` stands for the command),
/// `$TERMINAL`, then the usual suspects.
#[cfg(unix)]
fn terminal_command(exe: &str, args: &[String]) -> Option<Command> {
    let line = format!("{exe} {}", args.join(" "));
    if let Some(t) = terminal_setting() {
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
    fn the_next_client_does_not_get_the_old_flag() {
        assert_eq!(client_args(&a(&["--gui", "--dry-run"])), a(&["--dry-run"]));
        assert_eq!(client_args(&a(&["--tui", "--session", "s.json"])), a(&["--session", "s.json"]));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn the_next_client_inherits_no_descriptor_of_ours() {
        use std::os::unix::process::CommandExt;
        // like the camera: opened without O_CLOEXEC, so a plain spawn would hand it down
        // (high up, so it cannot be the one `ls` opens for the listing)
        let low = unsafe { libc::open(c"/dev/null".as_ptr(), libc::O_RDONLY) };
        let fd = unsafe { libc::fcntl(low, libc::F_DUPFD, 900) };
        unsafe { libc::close(low) };
        assert!(fd >= 900);
        let listed = |close: bool| {
            let mut c = Command::new("ls");
            c.arg("/proc/self/fd");
            if close {
                unsafe { c.pre_exec(close_inherited) };
            }
            String::from_utf8(c.output().unwrap().stdout).unwrap().split_whitespace().any(|f| f == fd.to_string())
        };
        assert!(listed(false), "the test needs a descriptor that is inherited");
        assert!(!listed(true));
        unsafe { libc::close(fd) };
    }

    #[test]
    fn a_missing_client_falls_back_to_the_one_built() {
        if cfg!(all(feature = "tui", not(feature = "gui"))) {
            assert_eq!(choose(&a(&["--gui"]), false, None), UiMode::Tui);
        }
    }
}
