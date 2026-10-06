mod app;
mod calls;
#[cfg(feature = "calls")]
mod media;
mod mapobj;
mod net;
mod proto;
mod render;
mod replay;
mod session;
mod window;
mod world;

use app::State;
use render::Text;

/// Headless mode: replay a recorder run, optionally walk/click, and write a PNG.
/// soworker --replay recorder/logs/<run> --shot out.png [--script "r:1500,d:600"] [--click X,Y] [--size 1000x700]
fn headless(args: &[String]) -> Result<(), String> {
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
        st.call.joined = Some("test".into());
        st.call.mic = true;
        st.call.cam = true;
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
    render::draw(&st, &mut pm, &mut text, 60);
    pm.save_png(&out).map_err(|e| e.to_string())?;
    println!("wrote {out}");
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
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
