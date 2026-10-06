//! Software renderer: draws the office into a tiny-skia pixmap. No GPU, no UI toolkit.
use crate::app::State;
use crate::world::{E, N, S, T, W, ZK};
use std::collections::HashMap;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, PixmapPaint, Rect, Stroke, StrokeDash, Transform};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Act {
    Away,
}

pub struct Btn {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub act: Act,
    pub label: &'static str,
    pub on: bool,
    pub enabled: bool,
}

/// The one remaining on-map button: available / away (SoWork specific; calls live in the meeting dock).
pub fn buttons(st: &State, _w: f32, _h: f32) -> Vec<Btn> {
    let ui = st.ui;
    let (bw, bh) = (84.0 * ui, 26.0 * ui);
    vec![Btn { x: 10.0 * ui, y: 122.0 * ui, w: bw, h: bh, act: Act::Away, label: if st.available { "available" } else { "away" }, on: st.available, enabled: true }]
}

pub fn hit(st: &State, px: f32, py: f32, w: f32, h: f32) -> Option<Act> {
    buttons(st, w, h).into_iter().find(|b| px >= b.x && px <= b.x + b.w && py >= b.y && py <= b.y + b.h).map(|b| b.act)
}

pub struct Text {
    font: Option<fontdue::Font>,
    cache: HashMap<(char, u32), (fontdue::Metrics, Vec<u8>)>,
}

impl Text {
    pub fn new() -> Self {
        const CANDIDATES: [&str; 6] = [
            "/usr/share/fonts/liberation-sans-fonts/LiberationSans-Regular.ttf",
            "/usr/share/fonts/liberation/LiberationSans-Regular.ttf",
            "/usr/share/fonts/dejavu-sans-fonts/DejaVuSans.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/usr/share/fonts/google-droid-sans-fonts/DroidSans.ttf",
            "/usr/share/fonts/TTF/DejaVuSans.ttf",
        ];
        let font = CANDIDATES
            .iter()
            .find_map(|p| std::fs::read(p).ok())
            .and_then(|b| fontdue::Font::from_bytes(b, fontdue::FontSettings::default()).ok());
        Text { font, cache: HashMap::new() }
    }

    pub fn width(&mut self, s: &str, px: f32) -> f32 {
        let Some(f) = &self.font else { return 0.0 };
        s.chars().map(|c| f.metrics(c, px).advance_width).sum()
    }

    /// Draw one line; `align` 0 = left, 0.5 = centre, 1 = right of x. `y` is the baseline.
    pub fn draw(&mut self, pm: &mut Pixmap, s: &str, x: f32, y: f32, px: f32, rgb: [u8; 3], align: f32) {
        if self.font.is_none() {
            return;
        }
        let mut cx = x - self.width(s, px) * align;
        let (w, h) = (pm.width() as i32, pm.height() as i32);
        let data = pm.data_mut();
        for c in s.chars() {
            let font = self.font.as_ref().unwrap();
            let key = (c, (px * 4.0) as u32);
            let (m, bmp) = self.cache.entry(key).or_insert_with(|| font.rasterize(c, px));
            let (gx0, gy0) = (cx as i32 + m.xmin, y as i32 - m.height as i32 - m.ymin);
            for row in 0..m.height as i32 {
                for col in 0..m.width as i32 {
                    let cov = bmp[(row * m.width as i32 + col) as usize] as u32;
                    let (gx, gy) = (gx0 + col, gy0 + row);
                    if cov == 0 || gx < 0 || gy < 0 || gx >= w || gy >= h {
                        continue;
                    }
                    let i = ((gy * w + gx) * 4) as usize;
                    for k in 0..3 {
                        data[i + k] = ((rgb[k] as u32 * cov + data[i + k] as u32 * (255 - cov)) / 255) as u8;
                    }
                }
            }
            cx += m.advance_width;
        }
    }

    pub fn shadowed(&mut self, pm: &mut Pixmap, s: &str, x: f32, y: f32, px: f32, rgb: [u8; 3], align: f32) {
        self.draw(pm, s, x + 1.0, y + 1.0, px, [0, 0, 0], align);
        self.draw(pm, s, x, y, px, rgb, align);
    }
}

fn paint(r: u8, g: u8, b: u8, a: u8) -> Paint<'static> {
    let mut p = Paint::default();
    p.set_color_rgba8(r, g, b, a);
    p.anti_alias = true;
    p
}

fn hue_rgb(id: &str) -> (u8, u8, u8) {
    let mut h: u32 = 0;
    for c in id.chars() {
        h = (h.wrapping_mul(31) + c as u32) % 360;
    }
    let (s, l) = (0.6f32, 0.55f32);
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = h as f32 / 60.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r, g, b) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    (((r + m) * 255.0) as u8, ((g + m) * 255.0) as u8, ((b + m) * 255.0) as u8)
}

/// (resident, proportional) memory of this process in bytes, from /proc.
pub fn proc_mem() -> (u64, u64) {
    let rss = std::fs::read_to_string("/proc/self/statm")
        .ok()
        .and_then(|s| s.split_whitespace().nth(1).and_then(|v| v.parse::<u64>().ok()))
        .unwrap_or(0)
        * 4096;
    let pss = std::fs::read_to_string("/proc/self/smaps_rollup")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("Pss:"))
                .and_then(|l| l.split_whitespace().nth(1)?.parse::<u64>().ok())
        })
        .unwrap_or(0)
        * 1024;
    (rss, pss)
}

pub fn draw(st: &State, pm: &mut Pixmap, text: &mut Text, fps: u32) {
    let (w, h) = (pm.width() as f32, pm.height() as f32);
    pm.fill(Color::from_rgba8(20, 22, 27, 255));
    let me = st.me();
    let (cx, cy) = me.map_or((3000.0, 3000.0), |m| (m.x, m.y));
    let ui = st.ui;
    let sc = st.scale * ui;
    let xf = |x: f32| (x - cx) * sc + w / 2.0;
    let yf = |y: f32| (y - cy) * sc + h / 2.0;
    let px = T * sc;
    let (i0, i1) = (((cx - w / 2.0 / sc) / T).floor() as i32, ((cx + w / 2.0 / sc) / T).floor() as i32);
    let (j0, j1) = (((cy - h / 2.0 / sc) / T).floor() as i32, ((cy + h / 2.0 / sc) / T).floor() as i32);
    let id = Transform::identity();

    let floor = paint(43, 58, 82, 255);
    for j in j0..=j1 {
        for i in i0..=i1 {
            if st.world.floors.contains(&(i, j)) {
                if let Some(r) = Rect::from_xywh(xf(i as f32 * T) + 0.5, yf(j as f32 * T) + 0.5, px - 1.0, px - 1.0) {
                    pm.fill_rect(r, &floor, id, None);
                }
            }
        }
    }
    let edges = |pm: &mut Pixmap, map: &std::collections::HashMap<(i32, i32), u8>, col: [u8; 3], width: f32| {
        let mut pb = PathBuilder::new();
        for j in j0..=j1 {
            for i in i0..=i1 {
                let m = map.get(&(i, j)).copied().unwrap_or(0);
                if m == 0 {
                    continue;
                }
                let (x, y) = (xf(i as f32 * T), yf(j as f32 * T));
                let mut seg = |a: (f32, f32), b: (f32, f32)| {
                    pb.move_to(a.0, a.1);
                    pb.line_to(b.0, b.1);
                };
                if m & N != 0 { seg((x, y), (x + px, y)); }
                if m & S != 0 { seg((x, y + px), (x + px, y + px)); }
                if m & W != 0 { seg((x, y), (x, y + px)); }
                if m & E != 0 { seg((x + px, y), (x + px, y + px)); }
            }
        }
        if let Some(p) = pb.finish() {
            pm.stroke_path(&p, &paint(col[0], col[1], col[2], 255), &Stroke { width, ..Stroke::default() }, Transform::identity(), None);
        }
    };
    edges(pm, &st.world.soft, [116, 128, 156], (2.5 * sc).max(1.5 * ui)); // walk-through walls, faint
    edges(pm, &st.world.walls, [232, 195, 106], (5.0 * sc).max(2.0 * ui)); // solid walls

    let prop = paint(74, 90, 110, 255);
    for &(ax, ay) in &st.world.assets {
        let (sx, sy) = (xf(ax), yf(ay));
        if sx > -5.0 && sx < w + 5.0 && sy > -5.0 && sy < h + 5.0 {
            if let Some(r) = Rect::from_xywh(sx - ui, sy - ui, 3.0 * ui, 3.0 * ui) {
                pm.fill_rect(r, &prop, id, None);
            }
        }
    }

    for z in &st.world.zones {
        let (zx, zy) = (xf(z.x - z.sx * ZK), yf(z.y - z.sy * ZK));
        let (zw, zh) = (z.sx * ZK * 2.0 * sc, z.sy * ZK * 2.0 * sc);
        if let Some(r) = Rect::from_xywh(zx, zy, zw, zh) {
            if st.cur_zone_id.as_deref() == Some(z.id.as_str()) {
                pm.fill_rect(r, &paint(217, 87, 127, 46), id, None);
            }
            let path = PathBuilder::from_rect(r);
            let col = if z.meeting { (217, 87, 127) } else { (102, 102, 119) };
            pm.stroke_path(&path, &paint(col.0, col.1, col.2, 255), &Stroke { width: 1.0, ..Stroke::default() }, id, None);
        }
        if let Some(n) = &z.name {
            text.draw(pm, n, xf(z.x), zy - 3.0 * ui, 11.0 * ui, [217, 87, 127], 0.5);
        }
    }

    if let (Some(path), Some(m)) = (&st.path, me) {
        let mut pb = PathBuilder::new();
        pb.move_to(xf(m.x), yf(m.y));
        for &(px_, py_) in path {
            pb.line_to(xf(px_), yf(py_));
        }
        if let Some(p) = pb.finish() {
            let stroke = Stroke { width: 2.0, dash: StrokeDash::new(vec![5.0, 5.0], 0.0), ..Stroke::default() };
            pm.stroke_path(&p, &paint(127, 214, 168, 255), &stroke, id, None);
        }
        if let Some(c) = PathBuilder::from_circle(xf(st.path_goal.0), yf(st.path_goal.1), 6.0) {
            pm.stroke_path(&c, &paint(127, 214, 168, 255), &Stroke { width: 2.0, ..Stroke::default() }, id, None);
        }
    }

    let mut order: Vec<_> = st.users.values().collect();
    order.sort_by(|a, b| (a.id == st.me_id).cmp(&(b.id == st.me_id))); // draw ourselves last
    for u in order {
        let (sx, sy, r) = (xf(u.x), yf(u.y), (40.0 * sc).max(6.0 * ui));
        let (cr, cg, cb) = hue_rgb(&u.id);
        match st.pics.get(&u.id) {
            Some(p) => {
                // the picture is already cut to a circle; draw it scaled to the avatar size
                let s = (2.0 * r) / p.width() as f32;
                let tf = Transform::from_scale(s, s).post_translate(sx - r, sy - r);
                pm.draw_pixmap(0, 0, p.as_ref(), &PixmapPaint { quality: tiny_skia::FilterQuality::Bilinear, ..PixmapPaint::default() }, tf, None);
            }
            None => {
                if let Some(c) = PathBuilder::from_circle(sx, sy, r) {
                    pm.fill_path(&c, &paint(cr, cg, cb, 255), FillRule::Winding, id, None);
                }
                let ch = u.name.trim().chars().next().map_or("?".to_string(), |c| c.to_uppercase().to_string());
                text.draw(pm, &ch, sx, sy + r * 0.38, r * 1.1, [255, 255, 255], 0.5);
            }
        }
        if u.speaking {
            if let Some(c) = PathBuilder::from_circle(sx, sy, r + 4.0) {
                pm.stroke_path(&c, &paint(127, 214, 168, 255), &Stroke { width: 3.0, ..Stroke::default() }, id, None);
            }
        }
        if u.id == st.me_id {
            if let Some(c) = PathBuilder::from_circle(sx, sy, r) {
                pm.stroke_path(&c, &paint(255, 255, 255, 255), &Stroke { width: 2.0, ..Stroke::default() }, id, None);
            }
        }
        let label = if u.state == "walk" { format!("{} ·", u.name) } else { u.name.clone() };
        text.draw(pm, &label, sx, sy - r - 4.0 * ui, 11.0 * ui, [221, 221, 238], 0.5);
    }

    // HUD
    let mut y = 20.0 * ui;
    let mut line = |pm: &mut Pixmap, s: &str, col: [u8; 3]| {
        text.shadowed(pm, s, 10.0 * ui, y, 13.0 * ui, col, 0.0);
        y += 16.0 * ui;
    };
    line(pm, &format!("{}{}", st.room, if st.dry { "  [dry run]" } else { "" }), [170, 170, 187]);
    line(pm, &format!("{} people{}", st.users.len(), if st.status.is_empty() { String::new() } else { format!("  {}", st.status) }), [170, 170, 187]);
    if let Some((t, _)) = &st.toast {
        line(pm, t, [240, 200, 120]);
    }
    if let Some(m) = me {
        line(pm, &format!("{}, {}", m.x.round(), m.y.round()), [170, 170, 187]);
    }
    let zname = st.cur_zone_id.as_ref().and_then(|id| st.world.zones.iter().find(|z| &z.id == id)).map(|z| z.name.clone().unwrap_or_else(|| "(unnamed)".into()));
    line(pm, &format!("zone: {}   zone msgs sent {}   cluster events {}   mode {}", zname.unwrap_or_else(|| "-".into()), st.zone_sent, st.clusters_seen, if st.available { "available" } else { "away" }), [170, 170, 187]);
    if st.show_debug {
        let (rss, pss) = proc_mem();
        let mb = |b: u64| format!("{:.1} MB", b as f64 / 1_048_576.0);
        let lines = [
            format!("fps {fps}   {} floors, {} solid + {} soft walls, {} props, {} users", st.world.floors.len(), st.world.walls.len(), st.world.soft.len(), st.world.assets.len(), st.users.len()),
            format!("whole app (one process)  rss {}   pss {}", mb(rss), mb(pss)),
        ];
        let mut by = h - (10.0 + 16.0 * (lines.len() as f32 - 1.0)) * ui;
        for l in &lines {
            text.shadowed(pm, l, 10.0 * ui, by, 12.0 * ui, [143, 255, 187], 0.0);
            by += 16.0 * ui;
        }
    }
    for b in buttons(st, w, h) {
        let col = match (b.enabled, b.on, b.act) {
            (false, ..) => (38, 44, 56),
            (true, true, _) => (47, 111, 79),
            (true, false, Act::Away) => (106, 47, 58),
        };
        if let Some(rc) = Rect::from_xywh(b.x, b.y, b.w, b.h) {
            pm.fill_rect(rc, &paint(col.0, col.1, col.2, 255), id, None);
            pm.stroke_path(&PathBuilder::from_rect(rc), &paint(70, 84, 104, 255), &Stroke { width: 1.0, ..Stroke::default() }, id, None);
        }
        let tc = if b.enabled { [220, 230, 240] } else { [110, 120, 135] };
        text.draw(pm, b.label, b.x + b.w / 2.0, b.y + b.h / 2.0 + 4.0 * ui, 12.0 * ui, tc, 0.5);
    }
    crate::meeting_ui::draw(st, pm, text, w, h);
}
