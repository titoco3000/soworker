mod app;
mod backend;
mod calls;
mod chat;
mod chatstate;
mod desktop;
mod devices;
mod directory;
mod login;
mod mapobj;
mod meeting;
mod mode;
mod net;
mod notify;
mod overlay;
mod profile;
mod proto;
mod roster;
mod session;
mod sound;
mod talk;
mod waves;
mod world;
#[cfg(feature = "gui")]
mod avatars;
#[cfg(feature = "gui")]
mod gui;
#[cfg(feature = "calls")]
mod media;
#[cfg(feature = "calls")]
mod mic;
#[cfg(feature = "tui")]
mod tui;
#[cfg(feature = "tui")]
mod videowin;

#[cfg(not(any(feature = "tui", feature = "gui")))]
compile_error!("enable a frontend: feature `tui` and/or `gui`");

#[cfg(feature = "gui")]
fn icon_sheet(out: &str) {
    use gui::icons::*;
    let mut pm = tiny_skia::Pixmap::new(420, 80).unwrap();
    pm.fill(tiny_skia::Color::from_rgba8(26, 30, 38, 255));
    for (i, s) in [MIC, MIC_OFF, VIDEO, VIDEO_OFF, MONITOR, PHONE_OFF, STOP_CIRCLE, NOISE_CANCEL, NOISE_CANCEL_OFF, MAXIMIZE, MINIMIZE, MESSAGE_CIRCLE].iter().enumerate() {
        gui::icon::draw(&mut pm.as_mut(), s, 10.0 + i as f32 * 34.0, 10.0, 24.0, gui::theme::INK);
        gui::icon::draw(&mut pm.as_mut(), s, 10.0 + i as f32 * 34.0, 48.0, 16.0, gui::theme::BRAND);
    }
    let _ = pm.save_png(out);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    // started by the other client as it quits (see `mode::relaunch`): give it a moment to let go of its sockets
    if let Some(ms) = std::env::var("SOWORKER_START_DELAY_MS").ok().and_then(|v| v.parse().ok()) {
        std::env::remove_var("SOWORKER_START_DELAY_MS");
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
    if let Some(i) = args.iter().position(|a| a == "--login-test") {
        // capture a login from a local test page (see tools/login-test) without touching SoWork
        let origin = args.get(i + 1).cloned().unwrap_or_default();
        let opts = login::Options { start_url: format!("{origin}/s/testgroup/testoffice"), origin, known_space: None, headless: true };
        let cancel = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let c2 = cancel.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(40));
            c2.store(true, std::sync::atomic::Ordering::Relaxed);
        });
        match login::capture(&opts, &cancel) {
            Ok(c) => println!("captured: apiKey {} chars, refresh token {} chars, space {:?}", c.api_key.len(), c.refresh_token.len(), c.space),
            Err(e) => {
                eprintln!("login test failed: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    if args.iter().any(|a| a == "--overlay-test") {
        // show the "you are live" pill for a few seconds, to check it works on this desktop
        let h = overlay::Handle::new();
        println!("{}", if h.is_layer() { "layer-shell overlay: look at the top-right corner" } else { "no layer-shell here: using notifications" });
        h.show(overlay::Overlay::Live { listeners: 2 });
        std::thread::sleep(std::time::Duration::from_secs(3));
        h.show(overlay::Overlay::Muted);
        std::thread::sleep(std::time::Duration::from_millis(3000));
        return;
    }
    if let Some(i) = args.iter().position(|a| a == "--talk") {
        // push to talk from a desktop shortcut: `soworker --talk` (toggle), `--talk down`, `--talk up`
        let act = args.get(i + 1).filter(|a| !a.starts_with("--")).map_or("toggle", |s| s.as_str());
        if let Err(e) = talk::send(act) {
            eprintln!("{e}");
            std::process::exit(1);
        }
        return;
    }
    if args.iter().any(|a| a == "--roster") {
        // print the member list the sidebar would show (uses the saved sign-in)
        let path = args.iter().position(|a| a == "--session").and_then(|i| args.get(i + 1)).cloned().unwrap_or_else(session::default_path);
        let res = session::Auth::load(&path).and_then(|mut a| {
            let group = a.file.find["groupId"].as_str().unwrap_or("").to_string();
            roster::fetch(&a.id_token()?, &group)
        });
        match res {
            Ok(m) => m.iter().for_each(|m| println!("{:<16} {:<8} {:<7} last {:?} zone {:?}", m.name, if m.online { "online" } else { "offline" }, m.status, m.last_online, m.zone)),
            Err(e) => eprintln!("roster: {e}"),
        }
        return;
    }
    #[cfg(feature = "gui")]
    if let Some(i) = args.iter().position(|a| a == "--icons") {
        icon_sheet(args.get(i + 1).map_or("icons.png", |s| s));
        return;
    }
    if let Err(e) = frontend(&args) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

/// Pick the client (see `mode::choose`), remember it, and start it. From a launcher icon with no terminal, the
/// terminal client is started inside a terminal emulator instead and this process ends.
fn frontend(args: &[String]) -> Result<(), String> {
    use mode::UiMode;
    mode::adopt_console();
    let terminal = mode::in_terminal();
    let mut m = mode::choose(args, terminal, mode::last());
    if m == UiMode::Tui && !mode::has_terminal() {
        match mode::relaunch(UiMode::Tui, args) {
            Ok(()) => return Ok(()),
            Err(e) if UiMode::Gui.built() => {
                eprintln!("{e}; opening the window instead");
                m = UiMode::Gui;
            }
            Err(e) => return Err(e),
        }
    }
    mode::remember(m);
    if m == UiMode::Gui {
        mode::drop_console();
    }
    match m {
        #[cfg(feature = "gui")]
        UiMode::Gui => gui::run(args),
        #[cfg(feature = "tui")]
        UiMode::Tui => tui::run(args),
        #[allow(unreachable_patterns)]
        _ => Err("this build has no client for that mode".into()),
    }
}
