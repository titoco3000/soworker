mod app;
mod avatars;
mod calls;
mod chat;
mod chatui;
mod icon;
mod icons;
mod login;
mod meeting_ui;
mod theme;
#[cfg(feature = "calls")]
mod media;
mod mapobj;
mod net;
mod proto;
mod render;
mod replay;
mod roster;
mod session;
mod waves;
mod sidebar;
mod window;
mod world;

use app::State;
use render::Text;

/// Headless mode: replay a recorder run, optionally walk/click, and write a PNG.
/// soworker --replay recorder/logs/<run> --shot out.png [--script "r:1500,d:600"] [--click X,Y] [--size 1000x700]
fn headless(args: &[String]) -> Result<(), String> {
    let args = args;
    let opt = |n: &str| args.iter().position(|a| a == n).and_then(|i| args.get(i + 1)).cloned();
    let run = opt("--replay").ok_or("--replay <run dir>")?;
    let out = opt("--shot").unwrap_or_else(|| "shot.png".into());
    let (w, h) = opt("--size")
        .and_then(|s| s.split_once('x').map(|(a, b)| (a.parse().unwrap_or(1000), b.parse().unwrap_or(700))))
        .unwrap_or((1000u32, 700u32));
    let mut st = State::new(&replay::own_id(&run)?, true);
    let events = replay::load(&run)?;
    println!("replayed {} events", events.len());
    for e in events {
        st.apply(e, 0.0);
    }
    if args.iter().any(|a| a == "--fake-call") {
        fake_call(&mut st, args);
    }
    if let Some(n) = opt("--away").and_then(|s| s.parse::<usize>().ok()) {
        // mark the n-th other user away (cluster_mode none-away), as SoWork shows a user who went away
        let mut ids: Vec<String> = st.users.keys().filter(|i| **i != st.me_id).cloned().collect();
        ids.sort();
        for id in ids.into_iter().take(n) {
            st.apply(proto::Net::Mode { id, mode: "none-away".into() }, 0.0);
        }
    }
    if args.iter().any(|a| a == "--fake-roster") {
        fake_roster(&mut st, opt("--open"));
    }
    if args.iter().any(|a| a == "--fake-chat") {
        fake_chat(&mut st, opt("--fake-chat").as_deref() == Some("area"));
    }
    st.confirm_logout = args.iter().any(|a| a == "--confirm-logout");
    if args.iter().any(|a| a == "--login-screen") {
        st.login = Some(match opt("--login-screen").as_deref() { Some("waiting") => app::LoginPhase::Waiting, Some("failed") => app::LoginPhase::Failed("The browser window was closed before signing in".into()), _ => app::LoginPhase::Idle });
    }
    st.update(0.0, 0.0);
    println!("world: {} floors, {} solid walls, {} soft walls, {} zones, {} props; me {:?}",
        st.world.floors.len(), st.world.walls.len(), st.world.soft.len(), st.world.zones.len(), st.world.assets.len(),
        st.me().map(|m| (m.x as i32, m.y as i32)));
    if let (Some(me), Some(sp)) = (st.me().cloned(), st.spawn) {
        let _ = (me, sp);
    }
    if let Some(c) = opt("--click") {
        let (a, b) = c.split_once(',').ok_or("--click X,Y (world units)")?;
        st.click(a.parse().map_err(|_| "x")?, b.parse().map_err(|_| "y")?, false, 0.0);
    }
    let mut now = 0.0f64;
    if let Some(script) = opt("--script") {
        for part in script.split(',') {
            let (k, ms) = part.split_once(':').ok_or("script item key:ms")?;
            let set = |st: &mut State, on: bool| match k {
                "r" => st.keys.right = on,
                "l" => st.keys.left = on,
                "u" => st.keys.up = on,
                "d" => st.keys.down = on,
                _ => {}
            };
            set(&mut st, true);
            for _ in 0..(ms.parse::<f64>().map_err(|_| "ms")? / 16.0) as i32 {
                now += 0.016;
                st.update(0.016, now);
            }
            set(&mut st, false);
        }
    }
    // let a click path finish
    for _ in 0..600 {
        if st.path.is_none() { break; }
        now += 0.016;
        st.update(0.016, now);
    }
    println!("me after: {:?}; zone: {:?}; outbox cmds: {}", st.me().map(|m| (m.x as i32, m.y as i32)), st.cur_zone_id, st.outbox.len());
    let mut pm = tiny_skia::Pixmap::new(w, h).ok_or("pixmap")?;
    let mut text = Text::new();
    if let Some(n) = opt("--bench").and_then(|s| s.parse::<u32>().ok()) {
        let t = std::time::Instant::now();
        for _ in 0..n {
            render::draw(&st, &mut pm, &mut text);
        }
        println!("bench: {:.2} ms/frame over {n} frames at {w}x{h}", t.elapsed().as_secs_f64() * 1000.0 / n as f64);
    }
    render::draw(&st, &mut pm, &mut text);
    pm.save_png(&out).map_err(|e| e.to_string())?;
    println!("wrote {out}");
    Ok(())
}

/// Synthetic waves and chat for layout checks (`--fake-chat [area]`): the sidebar shows two waves, the panel a conversation.
fn fake_chat(st: &mut State, area: bool) {
    use chat::{ChatEvent, ChatMsg, Kind};
    let mut ids: Vec<String> = st.users.keys().filter(|i| **i != st.me_id).cloned().collect();
    ids.sort();
    let name = |st: &State, i: usize| st.users.get(&ids[i]).map_or("Someone".into(), |u| u.name.clone());
    for (i, w) in ids.iter().take(2).enumerate() {
        let n = name(st, i);
        st.apply(proto::Net::Emote { sender: w.clone(), recipient: st.me_id.clone(), emote: "wave".into() }, 0.0);
        let _ = n;
    }
    let msg = |id: &str, ch: &str, user: &str, text: &str, at: i64| ChatMsg { id: id.into(), channel: ch.into(), user: user.into(), name: None, text: text.into(), at };
    let me = st.me_id.clone();
    let (ch, tab) = if area {
        st.apply(proto::Net::Cluster(Some(proto::ClusterInfo { id: "c".into(), conversation_id: "conv".into(), users: vec![], token: None, unsupported: None })), 0.0);
        st.apply(proto::Net::Chat(ChatEvent::Channel { id: "A1".into(), kind: Kind::Meeting("conv".into()) }), 0.0);
        ("A1", chatui::Tab::Area)
    } else {
        st.apply(proto::Net::Chat(ChatEvent::Channel { id: "D1".into(), kind: Kind::Direct(ids[2].clone()) }), 0.0);
        ("D1", chatui::Tab::Dm(ids[2].clone()))
    };
    let other = ids[2].clone();
    let lines = [
        (&other, "oi! voce viu o novo mapa do escritorio? ficou muito bom"),
        (&other, "da pra entrar na sala de reuniao agora se quiser"),
        (&me, "ja vou, so um minuto"),
        (&other, "supercalifragilisticexpialidocious_um_link_muito_longo_para_quebrar_na_tela"),
    ];
    let msgs = lines.iter().enumerate().map(|(i, (u, t))| msg(&i.to_string(), ch, u, t, i as i64 * 1000)).collect();
    st.apply(proto::Net::Chat(ChatEvent::History { channel: ch.into(), msgs }), 0.0);
    st.apply(proto::Net::Chat(ChatEvent::Message(msg("9", ch, &other, "chegou a mensagem nova", 9000))), 0.0);
    st.chat.open = true;
    st.chat.tab = tab.clone();
    if let chatui::Tab::Dm(p) = &tab {
        st.chat.dms.push(p.clone());
    }
    st.chat.input = "escrevendo uma resposta".into();
    st.chat.focus = true;
}

/// Synthetic member list for sidebar layout checks: some of the recorded users share a meeting, plus offline people.
fn fake_roster(st: &mut State, open: Option<String>) {
    let mut ids: Vec<String> = st.users.keys().filter(|i| **i != st.me_id).cloned().collect();
    ids.sort();
    let zone = st.world.zones.iter().find(|z| z.meeting).map(|z| z.id.clone());
    for id in ids.iter().take(3) {
        st.apply(proto::Net::Zone { id: id.clone(), zone: zone.clone() }, 0.0);
    }
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64);
    let offline = ["Lucas Pacheco", "Fabio", "Kevin", "Amanda", "A very long name that cannot possibly fit in the panel"];
    st.apply(
        proto::Net::Roster(offline.iter().enumerate().map(|(i, n)| roster::Member { id: format!("off{i}"), name: n.to_string(), last_online: Some(now - (i as i64 + 1) * 2_700_000 * (i as i64 + 1)), ..Default::default() }).collect()),
        0.0,
    );
    st.sidebar.open = open.map(|o| if o == "meeting" { zone.as_deref().map_or(o.clone(), sidebar::zone_key) } else if o == "first" { ids.get(3).cloned().unwrap_or(o) } else { o });
}

/// Synthetic content for layout checks: other attendees, avatar pictures, video frames, a screen share.
fn fake_call(st: &mut State, args: &[String]) {
    use calls::MediaEvent;
    let has = |f: &str| args.iter().any(|a| a == f);
    let opt = |n: &str| args.iter().position(|a| a == n).and_then(|i| args.get(i + 1)).cloned();
    let nothers: usize = opt("--others").and_then(|s| s.parse().ok()).unwrap_or(2);
    st.call.joined = Some("test".into());
    st.call.cam = true;
    st.call.mic = !has("--muted");
    let mut ids: Vec<(String, String)> = st.users.values().filter(|u| u.id != st.me_id).map(|u| (u.id.clone(), u.name.clone())).collect();
    ids.sort();
    ids.truncate(nothers);
    st.cluster = Some(proto::ClusterInfo {
        id: "c".into(),
        conversation_id: "v".into(),
        users: ids.iter().cloned().chain(std::iter::once((st.me_id.clone(), "me".into()))).collect(),
        token: None,
        unsupported: None,
    });
    // gradient pictures cut to circles, standing in for avatars loaded from the network
    let all: Vec<String> = st.users.keys().cloned().collect();
    for (k, id) in all.iter().enumerate() {
        let mut rgba = vec![0u8; (avatars::SIZE * avatars::SIZE * 4) as usize];
        let r = avatars::SIZE as f32 / 2.0;
        for (i, px) in rgba.chunks_exact_mut(4).enumerate() {
            let (x, y) = ((i as u32 % avatars::SIZE) as f32, (i as u32 / avatars::SIZE) as f32);
            let inside = (x - r).hypot(y - r) <= r;
            let (a, c) = (if inside { 255u8 } else { 0 }, [(x * 2.5) as u8 ^ (k as u8 * 40), (y * 2.5) as u8, 150 + (k as u8 * 20) % 100]);
            px.copy_from_slice(&[c[0] / 255 * a / 1 * 0 + (c[0] as u32 * a as u32 / 255) as u8, (c[1] as u32 * a as u32 / 255) as u8, (c[2] as u32 * a as u32 / 255) as u8, a]);
        }
        st.apply(proto::Net::Avatar { id: id.clone(), size: avatars::SIZE, rgba }, 0.0);
    }
    let frame = |w: u32, h: u32, seed: u8| -> Vec<u8> {
        (0..w * h).flat_map(|i| { let (x, y) = (i % w, i / w); [(x * 255 / w) as u8 ^ seed, (y * 255 / h) as u8, seed.wrapping_mul(7), 255] }).collect()
    };
    st.apply_media(MediaEvent::Video { key: "me".into(), name: "you".into(), w: 480, h: 270, rgba: frame(480, 270, 40) }, 0.0);
    if let Some((id, name)) = ids.first() {
        st.apply_media(MediaEvent::Video { key: id.clone(), name: name.clone(), w: 480, h: 270, rgba: frame(480, 270, 140) }, 0.0);
    }
    if let Some((id, _)) = ids.get(1) {
        st.call.peer_muted.insert(id.clone());
        st.call.speakers.insert(id.clone());
    }
    if has("--screen") {
        let who = ids.first().map(|(i, _)| i.clone()).unwrap_or_default();
        st.apply_media(MediaEvent::Video { key: format!("{who}:screen"), name: "Screen".into(), w: 960, h: 540, rgba: frame(960, 540, 200) }, 0.0);
    }
    if let Some(p) = opt("--pin") {
        st.dock.pinned = Some(match p.parse::<usize>() {
            Ok(0) => "self".to_string(),
            Ok(n) => ids.get(n - 1).map(|(i, _)| i.clone()).unwrap_or_default(),
            Err(_) => p,
        });
    }
    st.dock.full = has("--dock-full");
    st.dock.chat_open = has("--chat");
    if let Some(h) = opt("--hover") {
        st.dock.hover.tile = Some(match h.parse::<usize>() {
            Ok(0) => "self".to_string(),
            Ok(n) => ids.get(n - 1).map(|(i, _)| i.clone()).unwrap_or_default(),
            Err(_) => h,
        });
    }
}

fn icon_sheet(out: &str) {
    use icons::*;
    let mut pm = tiny_skia::Pixmap::new(420, 80).unwrap();
    pm.fill(tiny_skia::Color::from_rgba8(26, 30, 38, 255));
    for (i, s) in [MIC, MIC_OFF, VIDEO, VIDEO_OFF, MONITOR, PHONE_OFF, STOP_CIRCLE, LOCK, UNLOCK, MAXIMIZE, MINIMIZE, MESSAGE_CIRCLE].iter().enumerate() {
        icon::draw(&mut pm, s, 10.0 + i as f32 * 34.0, 10.0, 24.0, theme::INK);
        icon::draw(&mut pm, s, 10.0 + i as f32 * 34.0, 48.0, 16.0, theme::BRAND);
    }
    let _ = pm.save_png(out);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
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
    if let Some(i) = args.iter().position(|a| a == "--icons") {
        icon_sheet(args.get(i + 1).map_or("icons.png", |s| s));
        return;
    }
    if args.iter().any(|a| a == "--replay") {
        if let Err(e) = headless(&args) {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
        return;
    }
    if let Err(e) = window::run(&args) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
