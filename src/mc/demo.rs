//! `soworker --mc --mc-demo`: a made-up office with a meeting room and a few coworkers who walk about, without any
//! connection to SoWork. For working on the mod (map, villagers, movement) without signing in.
use crate::app::{Avatar, State};
use crate::calls::{MediaCmd, MediaEvent};
use crate::desks::Desk;
use crate::proto::Net;
use crate::world::{Zone, S, T, W};

pub const ME: &str = "demo-me";

fn at(i: f32, j: f32) -> (f32, f32) {
    (i * T, j * T)
}

fn avatar(id: &str, name: &str, (x, y): (f32, f32)) -> Avatar {
    Avatar { id: id.into(), name: name.into(), x, y, tx: x, ty: y, state: "idle".into(), status: "online".into(), mode: "cluster".into(), ..Default::default() }
}

/// A 20×12 office; a 7×6 meeting room in the north-east corner, walled on its west and south sides with a door in each,
/// and an open phone booth (3×2) just south of it.
pub fn setup(st: &mut State) {
    let key = std::mem::take(&mut st.talk_key);
    *st = State::new(ME, true);
    st.talk_key = key;
    st.set_focused(false, 0.0);
    st.room = "Demo office".into();
    let w = &mut st.world;
    w.floors = (0..20).flat_map(|i| (0..12).map(move |j| (i, j))).collect();
    for j in 0..=5 {
        if j != 3 {
            *w.walls.entry((13, j)).or_insert(0) |= W;
        }
    }
    for i in 13..20 {
        if i != 16 {
            *w.walls.entry((i, 5)).or_insert(0) |= S;
        }
    }
    let (cx, cy) = at(16.5, 3.0);
    w.zones.push(Zone { id: "demo-zone".into(), x: cx, y: cy, sx: 3.0 * T / 1.1, sy: 2.5 * T / 1.1, meeting: true, name: Some("Meeting room".into()) });
    // a phone booth right outside the meeting room's south wall: touching it, so it gets the next floor material
    let (bx, by) = at(18.5, 7.0);
    w.zones.push(Zone { id: "demo-zone-booth".into(), x: bx, y: by, sx: 1.5 * T / 1.1, sy: 1.0 * T / 1.1, meeting: true, name: Some("Phone booth".into()) });
    w.generation += 1;
    st.spawn = Some(at(2.5, 9.5));
    st.users.insert(ME.into(), avatar(ME, "Me (demo)", at(2.5, 9.5)));
    st.users.insert("demo-ana".into(), avatar("demo-ana", "Ana", at(5.5, 2.5)));
    let mut bo = avatar("demo-bo", "Bo", at(17.5, 2.5));
    bo.zone = Some("demo-zone".into());
    st.users.insert(bo.id.clone(), bo);
    let mut cy = avatar("demo-cy", "Cy", at(9.5, 9.5));
    cy.mode = "none-away".into();
    st.users.insert(cy.id.clone(), cy);
    st.apply(Net::Desks { definition: "demo-room".into(), desks: vec![("demo-desk".into(), at(7.5, 6.5).0, at(7.5, 6.5).1)] }, 0.0);
    st.apply(Net::DeskOwners(vec![Desk { room: "demo-room".into(), desk: "demo-desk".into(), owner: ME.into() }]), 0.0);
    st.status.clear();
}

/// The fake call: walking into the meeting room joins it (Bo is there with a camera and a screen share), the call
/// controls answer the way the media thread would, and walking out (or `leave`) ends it. Returns what the media
/// thread would have reported.
pub fn media(st: &mut State, now: f64, last_frame: &mut f64) -> Vec<MediaEvent> {
    let mut out = vec![];
    for m in st.media_out.drain(..) {
        match m {
            MediaCmd::Mic(on) => out.push(MediaEvent::MicState(on)),
            MediaCmd::Camera(on) => {
                out.push(MediaEvent::CameraState(on));
                if !on {
                    out.push(MediaEvent::VideoGone("me".into()));
                }
            }
            MediaCmd::Share(on) => {
                out.push(MediaEvent::ShareState(on));
                if !on {
                    out.push(MediaEvent::VideoGone("me:screen".into()));
                }
            }
            MediaCmd::Record(r) => out.push(MediaEvent::Recording(r.is_some())),
            MediaCmd::Leave => out.push(MediaEvent::Left),
            _ => {}
        }
    }
    let in_zone = st.cur_zone_id.is_some();
    if in_zone && st.call.joined.is_none() && !out.iter().any(|e| matches!(e, MediaEvent::Left)) {
        out.push(MediaEvent::Joined("demo-call".into()));
        out.push(MediaEvent::Peers(vec![("demo-bo".into(), "Bo".into())]));
        out.push(MediaEvent::Hands(vec!["demo-bo".into()]));
    } else if !in_zone && st.call.joined.is_some() {
        out.push(MediaEvent::Left);
    }
    if st.call.joined.is_some() && now - *last_frame >= 0.12 {
        *last_frame = now;
        let mut keys = vec![("demo-bo", 320, 240), ("demo-bo:screen", 640, 400)];
        if st.call.cam {
            keys.push(("me", 320, 240));
        }
        if st.call.share {
            keys.push(("me:screen", 640, 400));
        }
        for (k, w, h) in keys {
            out.push(MediaEvent::Video { key: k.into(), name: if k.starts_with("me") { "You".into() } else { "Bo".into() }, w, h, rgba: picture(k, w, h, now) });
        }
    }
    out
}

/// A moving test picture: colour bars sliding by, with a bouncing white square.
fn picture(key: &str, w: u32, h: u32, now: f64) -> Vec<u8> {
    let mut v = vec![0u8; (w * h * 4) as usize];
    let shift = (now * 60.0) as u32;
    let hue = key.len() as u32 * 40;
    let (bx, by) = (((now * 1.3).sin() * 0.4 + 0.5) * w as f64, ((now * 1.7).cos() * 0.4 + 0.5) * h as f64);
    for y in 0..h {
        for x in 0..w {
            let i = ((y * w + x) * 4) as usize;
            let band = ((x + shift + hue) / (w / 8).max(1)) % 8;
            let c = [[230, 60, 60], [240, 160, 40], [230, 220, 60], [80, 200, 90], [60, 170, 220], [70, 90, 210], [160, 80, 200], [235, 235, 235]][band as usize];
            let lit = 0.55 + 0.45 * (y as f32 / h as f32);
            let inside = (x as f64 - bx).abs() < w as f64 / 10.0 && (y as f64 - by).abs() < h as f64 / 10.0;
            for k in 0..3 {
                v[i + k] = if inside { 255 } else { (c[k] as f32 * lit) as u8 };
            }
            v[i + 3] = 255;
        }
    }
    v
}

/// Ana walks a loop round the office; Bo talks now and then.
pub fn tick(st: &mut State, now: f64) {
    const LOOP: [(f32, f32); 4] = [(5.5, 2.5), (10.5, 2.5), (10.5, 8.5), (5.5, 8.5)];
    let leg = (now / 5.0) as usize % LOOP.len();
    let (tx, ty) = at(LOOP[leg].0, LOOP[leg].1);
    if let Some(a) = st.users.get_mut("demo-ana") {
        (a.tx, a.ty) = (tx, ty);
        a.state = if (a.x - tx).hypot(a.y - ty) > 1.0 { "walk" } else { "idle" }.into();
    }
    if let Some(b) = st.users.get_mut("demo-bo") {
        b.speaking = (now / 3.0) as u64 % 2 == 1;
    }
}

/// Every 15 s someone waves, as gaia would report it: Cy at us, then Ana at Bo (a wave between two others).
pub fn emotes(now: f64, last: &mut f64) -> Vec<Net> {
    if now - *last < 15.0 {
        return vec![];
    }
    *last = now;
    let (sender, recipient) = if (now / 15.0) as u64 % 2 == 1 { ("demo-cy", ME) } else { ("demo-ana", "demo-bo") };
    vec![Net::Emote { sender: sender.into(), recipient: recipient.into(), emote: "wave".into() }]
}
