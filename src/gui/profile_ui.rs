//! The settings panel, over everything else: a card that opens above the gear in the sidebar's nav (or fills the space
//! between header and footer of a narrow window). A big user icon and the name, log out and terminal buttons, then
//! the devices: speaker (with a test sound), microphone (with a level meter) and camera, each a
//! drop-down list of the devices that are plugged in, low-data mode, and the noise cancellation strength (a slider,
//! with a button that records you and plays it back).
use crate::app::State;
use crate::devices::Kind;
use crate::gui::icons;
use crate::gui::meeting_ui::R;
use crate::gui::render::{paint, Text};
use crate::gui::panels;
use crate::gui::sidebar::{fill, fit, outline};
use crate::gui::theme::*;
use crate::profile::{Act, Profile};
use tiny_skia::{FillRule, PathBuilder, PixmapMut, PixmapPaint, Transform};

const W: f32 = 400.0;
const PAD: f32 = 18.0;
const AVATAR: f32 = 64.0;
/// Width of the control beside a list (Play sound, the level meter).
const SIDE: f32 = 112.0;
const CTRL_H: f32 = 30.0;
/// One setting: its label, the control and the gap below.
const ROW: f32 = 58.0;
/// The last row has no gap under it.
const ROW_LAST: f32 = ROW - 10.0;
const OPT_H: f32 = 28.0;
/// The noise cancellation row (not on a phone: it has no capture of ours, see `media::own_capture`).
const NOISE_ROW: bool = !cfg!(target_os = "android");

pub struct Layout {
    pub card: R,
    pub avatar: R,
    pub logout: R,
    /// Move to the terminal client (none when it is not built in).
    pub term: R,
    /// The line between the account and the settings.
    pub divider: f32,
    pub lists: Vec<(Kind, R)>,
    pub test: R,
    pub meter: R,
    /// "Sign in a phone" (desktop only).
    pub pair: R,
    /// The low-data mode switch, under the device lists.
    pub low: R,
    /// The noise cancellation slider (its whole hit box) and the test button beside it (empty on a phone).
    pub noise: R,
    pub mic_test: R,
    /// The open list: its box and each choice in it.
    pub options: Option<(R, Vec<(Option<String>, String, R)>)>,
}

pub fn layout(st: &State, p: &Profile, w: f32, h: f32) -> Layout {
    let u = st.ui;
    let rows = Kind::ALL.len() + 1 + NOISE_ROW as usize; // the devices, low data, noise cancellation
    let ch = (PAD + AVATAR + 16.0 + CTRL_H + 16.0 + 36.0 + (rows - 1) as f32 * ROW + ROW_LAST + PAD) * u;
    let f = panels::frame(st, w, h);
    let card = if f.mobile {
        R { y: f.content.y - p.scroll.clamp(0.0, (ch - f.content.h).max(0.0)), h: f.content.h.max(ch), ..f.content }
    } else {
        R { x: 8.0 * u, y: (f.nav.y - 8.0 * u - ch).max(8.0 * u), w: (W * u).min(w - 16.0 * u), h: ch }
    };
    let (x, inner) = (card.x + PAD * u, card.w - 2.0 * PAD * u);
    let avatar = R { x, y: card.y + PAD * u, w: AVATAR * u, h: AVATAR * u };
    let logout = R { x, y: avatar.y + avatar.h + 16.0 * u, w: SIDE * u, h: CTRL_H * u };
    let term = if cfg!(feature = "tui") { R { x: x + logout.w + 8.0 * u, ..logout } } else { R { x: -1.0, y: -1.0, w: 0.0, h: 0.0 } };
    let pair = if cfg!(target_os = "android") {
        R { x: -1.0, y: -1.0, w: 0.0, h: 0.0 }
    } else {
        let after = if panels::empty(&term) { logout } else { term };
        R { x: after.x + after.w + 8.0 * u, w: 132.0 * u, ..logout }
    };
    let divider = logout.y + logout.h + 16.0 * u;
    let row = |i: usize| divider + 36.0 * u + i as f32 * ROW * u + 18.0 * u;
    let narrow = inner - (SIDE + 8.0) * u;
    // speaker and microphone leave room for their test beside them; the rest span the card
    let beside = |k: Kind| k == Kind::Output || (k == Kind::Input && !cfg!(target_os = "android"));
    let lists: Vec<(Kind, R)> = Kind::ALL.iter().enumerate().map(|(i, &k)| (k, R { x, y: row(i), w: if beside(k) { narrow } else { inner }, h: CTRL_H * u })).collect();
    let side = |i: usize| R { x: x + narrow + 8.0 * u, y: row(i), w: SIDE * u, h: CTRL_H * u };
    let options = p.open.and_then(|k| lists.iter().find(|(l, _)| *l == k)).map(|&(k, b)| {
        let opts = p.options(k);
        let total = opts.len() as f32 * OPT_H * u + 8.0 * u;
        let below = b.y + b.h + 4.0 * u;
        let y0 = if below + total > h - 4.0 * u { (b.y - 4.0 * u - total).max(4.0 * u) } else { below };
        let rects = opts.into_iter().enumerate().map(|(i, (id, name))| (id, name, R { x: b.x, y: y0 + 4.0 * u + i as f32 * OPT_H * u, w: b.w, h: OPT_H * u })).collect();
        (R { x: b.x, y: y0, w: b.w, h: total }, rects)
    });
    let low = R { x, y: row(Kind::ALL.len()), w: inner, h: CTRL_H * u };
    let none = R { x: -1.0, y: -1.0, w: 0.0, h: 0.0 };
    let (noise, mic_test) = if NOISE_ROW { (R { x, y: row(Kind::ALL.len() + 1), w: narrow, h: CTRL_H * u }, side(Kind::ALL.len() + 1)) } else { (none, none) };
    Layout { card, avatar, logout, term, divider, lists, test: side(0), meter: if cfg!(target_os = "android") { none } else { side(1) }, pair, low, noise, mic_test, options }
}

/// The slider's track inside its hit box: inset so the knob stays inside at both ends.
fn track(l: &Layout, u: f32) -> (f32, f32) {
    let inset = 10.0 * u;
    (l.noise.x + inset, l.noise.w - 2.0 * inset)
}

/// The strength under a pointer at `px` (clamped to the track).
fn strength_at(l: &Layout, px: f32, u: f32) -> f32 {
    let (x, w) = track(l, u);
    ((px - x) / w.max(1.0)).clamp(0.0, 1.0)
}

/// While the slider is dragged: the strength at the pointer.
pub fn slide(st: &State, px: f32, w: f32, h: f32) -> Option<f32> {
    let p = st.profile.as_ref().filter(|p| p.sliding)?;
    Some(strength_at(&layout(st, p, w, h), px, st.ui))
}

/// Phone layout: scroll the panel by `dy` pixels (positive = towards the top). False when it is not open there.
pub fn wheel(st: &mut State, dy: f32, w: f32, h: f32) -> bool {
    let f = panels::frame(st, w, h);
    let Some(p) = st.profile.as_ref().filter(|_| f.mobile) else { return false };
    let max = (layout(st, p, w, h).card.h - f.content.h).max(0.0);
    let s = (p.scroll - dy).clamp(0.0, max);
    if let Some(p) = st.profile.as_mut() {
        if s != p.scroll {
            (p.scroll, p.open) = (s, None);
            st.dirty = true;
        }
    }
    true
}

/// What a click at (px, py) does; None = it fell outside the panel (which closes it).
pub fn hit(st: &State, px: f32, py: f32, w: f32, h: f32) -> Option<Act> {
    let p = st.profile.as_ref()?;
    let l = layout(st, p, w, h);
    if let Some((list, opts)) = &l.options {
        if let Some((id, ..)) = opts.iter().find(|(_, _, r)| r.contains(px, py)) {
            return Some(Act::Pick(p.open?, id.clone()));
        }
        if list.contains(px, py) {
            return Some(Act::Panel);
        }
    }
    if !l.card.contains(px, py) {
        return None;
    }
    if l.logout.contains(px, py) {
        return Some(Act::Logout);
    }
    if !panels::empty(&l.term) && l.term.contains(px, py) {
        return Some(Act::Terminal);
    }
    if over_photo(&l, px, py) {
        return Some(Act::ChangePhoto);
    }
    if l.test.contains(px, py) {
        return Some(Act::TestSound);
    }
    if l.low.contains(px, py) {
        return Some(Act::LowData);
    }
    if !panels::empty(&l.noise) && l.noise.contains(px, py) {
        return Some(Act::Strength(strength_at(&l, px, st.ui)));
    }
    if !panels::empty(&l.mic_test) && l.mic_test.contains(px, py) {
        return Some(Act::TestMic);
    }
    if !panels::empty(&l.pair) && l.pair.contains(px, py) {
        return Some(Act::PairPhone);
    }
    Some(l.lists.iter().find(|(_, r)| r.contains(px, py)).map_or(Act::Panel, |(k, _)| Act::Toggle(*k)))
}

/// The pointer is on our round picture.
pub fn over_photo(l: &Layout, px: f32, py: f32) -> bool {
    let r = l.avatar.w / 2.0;
    (px - l.avatar.x - r).hypot(py - l.avatar.y - r) <= r
}

fn centred(text: &mut Text, pm: &mut PixmapMut<'_>, s: &str, r: &R, px: f32, col: Rgb) {
    text.draw(pm, s, r.x + r.w / 2.0, r.y + r.h / 2.0 + px * 0.36, px, col, 0.5);
}

pub fn draw(st: &State, pm: &mut PixmapMut<'_>, text: &mut Text) {
    let Some(p) = st.profile.as_ref() else { return };
    let (u, w, h) = (st.ui, pm.width() as f32, pm.height() as f32);
    let l = layout(st, p, w, h);
    let id = Transform::identity();
    if st.sidebar.mobile {
        fill(pm, &l.card, 0.0, SURFACE, 255); // a whole panel
    } else {
        fill(pm, &R { x: l.card.x + 2.0 * u, y: l.card.y + 4.0 * u, ..l.card }, RADIUS_CARD * u, [0, 0, 0], 90); // shadow
        fill(pm, &l.card, RADIUS_CARD * u, SURFACE, 255);
        outline(pm, &l.card, RADIUS_CARD * u, LINE);
    }

    // who: the picture (or a big user icon) and the name
    let (cx, cy, r) = (l.avatar.x + l.avatar.w / 2.0, l.avatar.y + l.avatar.h / 2.0, l.avatar.w / 2.0);
    match st.pics.get(&st.me_id) {
        Some(pic) => {
            let s = (2.0 * r) / pic.w as f32;
            let tf = Transform::from_scale(s, s).post_translate(cx - r, cy - r);
            pm.draw_pixmap(0, 0, pic.px(), &PixmapPaint { quality: tiny_skia::FilterQuality::Bilinear, ..PixmapPaint::default() }, tf, None);
        }
        None => {
            if let Some(c) = PathBuilder::from_circle(cx, cy, r) {
                pm.fill_path(&c, &paint(MUTED[0], MUTED[1], MUTED[2], 255), FillRule::Winding, id, None);
            }
            let s = r * 1.15;
            crate::gui::icon::draw(pm, icons::USER, cx - s / 2.0, cy - s / 2.0 - u, s, INK_MUTED);
        }
    }
    if p.photo_hover {
        // darken the picture and offer to change it
        if let Some(c) = PathBuilder::from_circle(cx, cy, r) {
            pm.fill_path(&c, &paint(0, 0, 0, 110), FillRule::Winding, id, None);
        }
        let s = r * 0.8;
        crate::gui::icon::draw(pm, icons::PENCIL, cx - s / 2.0, cy - s / 2.0, s, [255, 255, 255]);
    }
    let tx = l.avatar.x + l.avatar.w + 14.0 * u;
    let name_w = l.card.x + l.card.w - PAD * u - tx;
    let name = st.name_of(&st.me_id, None);
    let name = if st.me_id.is_empty() { "Not signed in".to_string() } else { name };
    let name = fit(text, &name, 18.0 * u, name_w);
    text.draw(pm, &name, tx, cy - 2.0 * u, 18.0 * u, INK, 0.0);
    let sub = match (st.available, st.room.is_empty()) {
        (true, true) => "Available".to_string(),
        (false, true) => "Away".to_string(),
        (a, false) => format!("{} · {}", if a { "Available" } else { "Away" }, st.room),
    };
    let sub = fit(text, &sub, 12.0 * u, name_w);
    text.draw(pm, &sub, tx, cy + 18.0 * u, 12.0 * u, INK_MUTED, 0.0);

    outline(pm, &l.logout, 6.0 * u, LINE);
    centred(text, pm, "Log out", &l.logout, 13.0 * u, DANGER);
    if !panels::empty(&l.term) {
        outline(pm, &l.term, 6.0 * u, LINE);
        centred(text, pm, "Terminal", &l.term, 13.0 * u, INK_MUTED);
    }
    if !panels::empty(&l.pair) {
        outline(pm, &l.pair, 6.0 * u, LINE);
        centred(text, pm, "Sign in a phone", &l.pair, 13.0 * u, INK_MUTED);
    }

    // settings
    fill(pm, &R { x: l.card.x + PAD * u, y: l.divider, w: l.card.w - 2.0 * PAD * u, h: 1.0 }, 0.0, LINE, 255);
    text.draw(pm, "Settings", l.card.x + PAD * u, l.divider + 24.0 * u, 14.0 * u, INK, 0.0);
    for (k, b) in &l.lists {
        text.draw(pm, k.label(), b.x, b.y - 7.0 * u, 12.0 * u, INK_MUTED, 0.0);
        let open = p.open == Some(*k);
        fill(pm, b, 6.0 * u, MUTED, 255);
        outline(pm, b, 6.0 * u, if open { BRAND } else { LINE });
        let label = fit(text, &p.label(*k), 13.0 * u, b.w - 40.0 * u);
        text.draw(pm, &label, b.x + 10.0 * u, b.y + b.h / 2.0 + 4.5 * u, 13.0 * u, INK, 0.0);
        let s = 16.0 * u;
        crate::gui::icon::draw(pm, icons::CHEVRON_DOWN, b.x + b.w - s - 8.0 * u, b.y + (b.h - s) / 2.0, s, INK_MUTED);
    }

    // low-data mode: a switch spanning the card
    text.draw(pm, "Low data mode", l.low.x, l.low.y - 7.0 * u, 12.0 * u, INK_MUTED, 0.0);
    let on = st.low_data_user;
    fill(pm, &l.low, 6.0 * u, MUTED, 255);
    outline(pm, &l.low, 6.0 * u, if on { BRAND } else { LINE });
    let s = 16.0 * u;
    crate::gui::icon::draw(pm, if on { icons::WIFI_OFF } else { icons::WIFI }, l.low.x + 10.0 * u, l.low.y + (l.low.h - s) / 2.0, s, if on { BRAND } else { INK_MUTED });
    let label = fit(text, if on { "On: no camera sent or received" } else { "Off" }, 13.0 * u, l.low.w - 40.0 * u);
    text.draw(pm, &label, l.low.x + 16.0 * u + s, l.low.y + l.low.h / 2.0 + 4.5 * u, 13.0 * u, INK, 0.0);

    // noise cancellation: a slider (dimmed while it is off) and the test beside it
    if !panels::empty(&l.noise) {
        let s = st.noise_strength;
        let label = if st.noise_cancel { format!("Noise cancellation · {}", crate::calls::noise_strength_label(s)) } else { "Noise cancellation · off".into() };
        text.draw(pm, &label, l.noise.x, l.noise.y - 7.0 * u, 12.0 * u, INK_MUTED, 0.0);
        let (tx, tw) = track(&l, u);
        let (ty, th) = (l.noise.y + l.noise.h / 2.0 - 2.0 * u, 4.0 * u);
        let on = if st.noise_cancel { BRAND } else { INK_FAINT };
        fill(pm, &R { x: tx, y: ty, w: tw, h: th }, 2.0 * u, MUTED, 255);
        fill(pm, &R { x: tx, y: ty, w: tw * s, h: th }, 2.0 * u, on, 255);
        let (kx, ky, kr) = (tx + tw * s, l.noise.y + l.noise.h / 2.0, 8.0 * u);
        if let Some(c) = PathBuilder::from_circle(kx, ky, kr) {
            pm.fill_path(&c, &paint(SURFACE[0], SURFACE[1], SURFACE[2], 255), FillRule::Winding, id, None);
            let stroke = tiny_skia::Stroke { width: 2.0 * u, ..Default::default() };
            pm.stroke_path(&c, &paint(on[0], on[1], on[2], 255), &stroke, id, None);
        }

        use crate::calls::MicTest;
        let (busy, label) = match p.mic_test() {
            Some(MicTest::Preparing) => (true, "Starting…".to_string()),
            Some(MicTest::Recording(n)) => (true, format!("Talk… {n}")),
            Some(MicTest::Playing) => (true, "Playing…".to_string()),
            Some(MicTest::Failed) => (false, "Test failed".to_string()),
            _ => (false, "Test mic".to_string()),
        };
        outline(pm, &l.mic_test, 6.0 * u, if busy { BRAND } else { LINE });
        let ic = 15.0 * u;
        let lw = text.width(&label, 12.0 * u);
        let x0 = l.mic_test.x + (l.mic_test.w - ic - 6.0 * u - lw) / 2.0;
        let col = if busy { BRAND } else { INK_MUTED };
        crate::gui::icon::draw(pm, icons::MIC, x0, l.mic_test.y + (l.mic_test.h - ic) / 2.0, ic, col);
        text.draw(pm, &label, x0 + ic + 6.0 * u, l.mic_test.y + l.mic_test.h / 2.0 + 4.5 * u, 12.0 * u, col, 0.0);
    }

    // Play sound
    outline(pm, &l.test, 6.0 * u, LINE);
    let s = 15.0 * u;
    let tw = text.width("Play sound", 12.0 * u);
    let x0 = l.test.x + (l.test.w - s - 6.0 * u - tw) / 2.0;
    crate::gui::icon::draw(pm, icons::VOLUME, x0, l.test.y + (l.test.h - s) / 2.0, s, INK_MUTED);
    text.draw(pm, "Play sound", x0 + s + 6.0 * u, l.test.y + l.test.h / 2.0 + 4.5 * u, 12.0 * u, INK_MUTED, 0.0);

    // the microphone level: lit segments, green, then amber and red at the top (none on a phone)
    if !panels::empty(&l.meter) {
        outline(pm, &l.meter, 6.0 * u, LINE);
        match p.level() {
            Some(level) => {
                const N: usize = 14;
                let (x, w) = (l.meter.x + 10.0 * u, l.meter.w - 20.0 * u);
                let (gap, sh) = (2.0 * u, 12.0 * u);
                let sw = (w - gap * (N - 1) as f32) / N as f32;
                for i in 0..N {
                    let lit = (i as f32 + 0.5) / N as f32 <= level;
                    let col = if !lit { MUTED } else if i >= N - 2 { DANGER } else if i >= N - 4 { [240, 170, 60] } else { SPEAKING };
                    fill(pm, &R { x: x + i as f32 * (sw + gap), y: l.meter.y + (l.meter.h - sh) / 2.0, w: sw, h: sh }, 1.5 * u, col, 255);
                }
            }
            None => centred(text, pm, if p.meter.is_some() { "No signal" } else { "No meter" }, &l.meter, 11.0 * u, INK_FAINT),
        }
    }

    // the open list, over everything else
    if let Some((list, opts)) = &l.options {
        fill(pm, &R { x: list.x + 2.0 * u, y: list.y + 4.0 * u, ..*list }, 8.0 * u, [0, 0, 0], 90);
        fill(pm, list, 8.0 * u, SURFACE, 255);
        outline(pm, list, 8.0 * u, LINE);
        let cur = p.open.and_then(|k| p.chosen.get(k).map(String::from));
        for (id, name, r) in opts {
            let on = *id == cur;
            if on {
                fill(pm, &R { x: r.x + 4.0 * u, w: r.w - 8.0 * u, ..*r }, 5.0 * u, MUTED, 255);
            }
            let name = fit(text, name, 13.0 * u, r.w - 24.0 * u);
            text.draw(pm, &name, r.x + 12.0 * u, r.y + r.h / 2.0 + 4.5 * u, 13.0 * u, if on { BRAND } else { INK }, 0.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::{Device, Devices};

    fn state() -> State {
        let mut st = State::new("me", true);
        st.ui = 1.0;
        let devices = Devices {
            outputs: vec![Device { id: "sink.a".into(), name: "Speakers".into() }, Device { id: "sink.b".into(), name: "Headset".into() }],
            inputs: vec![Device { id: "src.a".into(), name: "Mic".into() }],
            cameras: vec![],
            screens: vec![],
        };
        st.profile = Some(Profile { sel: 0, ..Profile::with(devices) });
        st
    }

    #[test]
    fn clicks_in_the_panel() {
        let mut st = state();
        let l = layout(&st, st.profile.as_ref().unwrap(), 1000.0, 700.0);
        assert!(l.card.y + l.card.h <= 700.0 - panels::NAV, "the card sits above the nav");
        assert_eq!(hit(&st, l.logout.x + 2.0, l.logout.y + 2.0, 1000.0, 700.0), Some(Act::Logout));
        assert_eq!(hit(&st, l.test.x + 2.0, l.test.y + 2.0, 1000.0, 700.0), Some(Act::TestSound));
        let a = l.avatar;
        assert_eq!(hit(&st, a.x + a.w / 2.0, a.y + a.h / 2.0, 1000.0, 700.0), Some(Act::ChangePhoto));
        assert_eq!(hit(&st, a.x + 1.0, a.y + 1.0, 1000.0, 700.0), Some(Act::Panel), "the corner of the square is outside the circle");
        let (_, out) = l.lists[0];
        assert_eq!(hit(&st, out.x + 2.0, out.y + 2.0, 1000.0, 700.0), Some(Act::Toggle(Kind::Output)));
        assert_eq!(hit(&st, 900.0, 100.0, 1000.0, 700.0), None, "outside closes");
        st.profile.as_mut().unwrap().open = Some(Kind::Output);
        let l = layout(&st, st.profile.as_ref().unwrap(), 1000.0, 700.0);
        let (_, opts) = l.options.as_ref().unwrap();
        assert_eq!(opts.len(), 3, "the default and two speakers");
        let r = opts[2].2;
        assert_eq!(hit(&st, r.x + 5.0, r.y + 5.0, 1000.0, 700.0), Some(Act::Pick(Kind::Output, Some("sink.b".into()))));
    }
}
