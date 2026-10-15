//! The profile panel: a card that opens above the user icon in the sidebar footer. A big user icon and the name, a
//! log out button, then the settings: speaker (with a test sound), microphone (with a level meter) and camera, each a
//! drop-down list of the devices that are plugged in.
use crate::app::State;
use crate::devices::Kind;
use crate::gui::icons;
use crate::gui::meeting_ui::R;
use crate::gui::render::{paint, Text};
use crate::gui::sidebar::{fill, fit, outline, FOOTER};
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
const OPT_H: f32 = 28.0;

pub struct Layout {
    pub card: R,
    pub avatar: R,
    pub logout: R,
    /// The line between the account and the settings.
    pub divider: f32,
    pub lists: [(Kind, R); 3],
    pub test: R,
    pub meter: R,
    /// The open list: its box and each choice in it.
    pub options: Option<(R, Vec<(Option<String>, String, R)>)>,
}

pub fn layout(st: &State, p: &Profile, h: f32) -> Layout {
    let u = st.ui;
    let ch = (PAD + AVATAR + 16.0 + CTRL_H + 16.0 + 36.0 + 3.0 * ROW - 10.0 + PAD) * u;
    let card = R { x: 8.0 * u, y: (h - FOOTER * u - 8.0 * u - ch).max(8.0 * u), w: W * u, h: ch };
    let (x, inner) = (card.x + PAD * u, card.w - 2.0 * PAD * u);
    let avatar = R { x, y: card.y + PAD * u, w: AVATAR * u, h: AVATAR * u };
    let logout = R { x, y: avatar.y + avatar.h + 16.0 * u, w: SIDE * u, h: CTRL_H * u };
    let divider = logout.y + logout.h + 16.0 * u;
    let row = |i: usize| divider + 36.0 * u + i as f32 * ROW * u + 18.0 * u;
    let narrow = inner - (SIDE + 8.0) * u;
    let lists = [
        (Kind::Output, R { x, y: row(0), w: narrow, h: CTRL_H * u }),
        (Kind::Input, R { x, y: row(1), w: narrow, h: CTRL_H * u }),
        (Kind::Camera, R { x, y: row(2), w: inner, h: CTRL_H * u }),
    ];
    let side = |i: usize| R { x: x + narrow + 8.0 * u, y: row(i), w: SIDE * u, h: CTRL_H * u };
    let options = p.open.and_then(|k| lists.iter().find(|(l, _)| *l == k)).map(|&(k, b)| {
        let opts = p.options(k);
        let total = opts.len() as f32 * OPT_H * u + 8.0 * u;
        let below = b.y + b.h + 4.0 * u;
        let y0 = if below + total > h - 4.0 * u { (b.y - 4.0 * u - total).max(4.0 * u) } else { below };
        let rects = opts.into_iter().enumerate().map(|(i, (id, name))| (id, name, R { x: b.x, y: y0 + 4.0 * u + i as f32 * OPT_H * u, w: b.w, h: OPT_H * u })).collect();
        (R { x: b.x, y: y0, w: b.w, h: total }, rects)
    });
    Layout { card, avatar, logout, divider, lists, test: side(0), meter: side(1), options }
}

/// What a click at (px, py) does; None = it fell outside the panel (which closes it).
pub fn hit(st: &State, px: f32, py: f32, h: f32) -> Option<Act> {
    let p = st.profile.as_ref()?;
    let l = layout(st, p, h);
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
    if l.test.contains(px, py) {
        return Some(Act::TestSound);
    }
    Some(l.lists.iter().find(|(_, r)| r.contains(px, py)).map_or(Act::Panel, |(k, _)| Act::Toggle(*k)))
}

fn centred(text: &mut Text, pm: &mut PixmapMut<'_>, s: &str, r: &R, px: f32, col: Rgb) {
    text.draw(pm, s, r.x + r.w / 2.0, r.y + r.h / 2.0 + px * 0.36, px, col, 0.5);
}

pub fn draw(st: &State, pm: &mut PixmapMut<'_>, text: &mut Text) {
    let Some(p) = st.profile.as_ref() else { return };
    let (u, h) = (st.ui, pm.height() as f32);
    let l = layout(st, p, h);
    let id = Transform::identity();
    fill(pm, &R { x: l.card.x + 2.0 * u, y: l.card.y + 4.0 * u, ..l.card }, RADIUS_CARD * u, [0, 0, 0], 90); // shadow
    fill(pm, &l.card, RADIUS_CARD * u, SURFACE, 255);
    outline(pm, &l.card, RADIUS_CARD * u, LINE);

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

    // Play sound
    outline(pm, &l.test, 6.0 * u, LINE);
    let s = 15.0 * u;
    let tw = text.width("Play sound", 12.0 * u);
    let x0 = l.test.x + (l.test.w - s - 6.0 * u - tw) / 2.0;
    crate::gui::icon::draw(pm, icons::VOLUME, x0, l.test.y + (l.test.h - s) / 2.0, s, INK_MUTED);
    text.draw(pm, "Play sound", x0 + s + 6.0 * u, l.test.y + l.test.h / 2.0 + 4.5 * u, 12.0 * u, INK_MUTED, 0.0);

    // the microphone level: lit segments, green, then amber and red at the top
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
    use crate::devices::{Choice, Device, Devices};

    fn state() -> State {
        let mut st = State::new("me", true);
        st.ui = 1.0;
        let devices = Devices {
            outputs: vec![Device { id: "sink.a".into(), name: "Speakers".into() }, Device { id: "sink.b".into(), name: "Headset".into() }],
            inputs: vec![Device { id: "src.a".into(), name: "Mic".into() }],
            cameras: vec![],
        };
        st.profile = Some(Profile { devices, chosen: Choice::default(), open: None, meter: None, sel: 0 });
        st
    }

    #[test]
    fn clicks_in_the_panel() {
        let mut st = state();
        let l = layout(&st, st.profile.as_ref().unwrap(), 700.0);
        assert!(l.card.y + l.card.h <= 700.0 - FOOTER, "the card sits above the footer");
        assert_eq!(hit(&st, l.logout.x + 2.0, l.logout.y + 2.0, 700.0), Some(Act::Logout));
        assert_eq!(hit(&st, l.test.x + 2.0, l.test.y + 2.0, 700.0), Some(Act::TestSound));
        let (_, out) = l.lists[0];
        assert_eq!(hit(&st, out.x + 2.0, out.y + 2.0, 700.0), Some(Act::Toggle(Kind::Output)));
        assert_eq!(hit(&st, 900.0, 100.0, 700.0), None, "outside closes");
        st.profile.as_mut().unwrap().open = Some(Kind::Output);
        let l = layout(&st, st.profile.as_ref().unwrap(), 700.0);
        let (_, opts) = l.options.as_ref().unwrap();
        assert_eq!(opts.len(), 3, "the default and two speakers");
        let r = opts[2].2;
        assert_eq!(hit(&st, r.x + 5.0, r.y + 5.0, 700.0), Some(Act::Pick(Kind::Output, Some("sink.b".into()))));
    }
}
