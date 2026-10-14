//! Draws the Feather icons from `icons.rs` with tiny-skia: a small SVG path parser (M L H V C S Q A Z, absolute
//! and relative) plus the basic shapes. Strokes are 2 units on the 24x24 grid with round caps and joins.
use crate::gui::icons::Shape;
use tiny_skia::{LineCap, LineJoin, Paint, Path, PathBuilder, PixmapMut, Stroke, Transform};

fn nums(s: &str) -> Vec<f32> {
    let mut out = Vec::new();
    let (mut cur, bytes) = (String::new(), s.as_bytes());
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        let starts_new = c == '-' && !cur.is_empty() && !cur.ends_with('e');
        if c.is_ascii_digit() || c == '.' && !cur.contains('.') || (c == '-' && cur.is_empty()) || c == 'e' && !cur.is_empty() {
            if starts_new {
                out.push(cur.parse().unwrap_or(0.0));
                cur.clear();
            }
            cur.push(c);
        } else if c == '-' && starts_new {
            out.push(cur.parse().unwrap_or(0.0));
            cur = "-".into();
        } else {
            if !cur.is_empty() {
                out.push(cur.parse().unwrap_or(0.0));
                cur.clear();
            }
            if c == '.' {
                cur.push('.'); // a second '.' starts a new number: "1.5.5" = 1.5, .5
            }
        }
        i += 1;
    }
    if !cur.is_empty() {
        out.push(cur.parse().unwrap_or(0.0));
    }
    out
}

/// SVG elliptical arc (endpoint form) as cubic Béziers.
fn arc(pb: &mut PathBuilder, (x1, y1): (f32, f32), mut rx: f32, mut ry: f32, rot: f32, large: bool, sweep: bool, (x2, y2): (f32, f32)) {
    if rx == 0.0 || ry == 0.0 || (x1 == x2 && y1 == y2) {
        pb.line_to(x2, y2);
        return;
    }
    rx = rx.abs();
    ry = ry.abs();
    let phi = rot.to_radians();
    let (cp, sp) = (phi.cos(), phi.sin());
    let (dx, dy) = ((x1 - x2) / 2.0, (y1 - y2) / 2.0);
    let (x1p, y1p) = (cp * dx + sp * dy, -sp * dx + cp * dy);
    let lam = (x1p * x1p) / (rx * rx) + (y1p * y1p) / (ry * ry);
    if lam > 1.0 {
        rx *= lam.sqrt();
        ry *= lam.sqrt();
    }
    let num = rx * rx * ry * ry - rx * rx * y1p * y1p - ry * ry * x1p * x1p;
    let den = rx * rx * y1p * y1p + ry * ry * x1p * x1p;
    let co = (if large == sweep { -1.0 } else { 1.0 }) * (num / den).max(0.0).sqrt();
    let (cxp, cyp) = (co * rx * y1p / ry, -co * ry * x1p / rx);
    let (cx, cy) = (cp * cxp - sp * cyp + (x1 + x2) / 2.0, sp * cxp + cp * cyp + (y1 + y2) / 2.0);
    let ang = |ux: f32, uy: f32, vx: f32, vy: f32| (ux * vy - uy * vx).atan2(ux * vx + uy * vy);
    let th1 = ang(1.0, 0.0, (x1p - cxp) / rx, (y1p - cyp) / ry);
    let mut dth = ang((x1p - cxp) / rx, (y1p - cyp) / ry, (-x1p - cxp) / rx, (-y1p - cyp) / ry);
    if !sweep && dth > 0.0 {
        dth -= std::f32::consts::TAU;
    } else if sweep && dth < 0.0 {
        dth += std::f32::consts::TAU;
    }
    let segs = (dth.abs() / (std::f32::consts::FRAC_PI_2)).ceil().max(1.0) as i32;
    let step = dth / segs as f32;
    let t = 4.0 / 3.0 * (step / 4.0).tan();
    let mut a = th1;
    for _ in 0..segs {
        let (c1, s1, c2, s2) = (a.cos(), a.sin(), (a + step).cos(), (a + step).sin());
        let p = |x: f32, y: f32| (cp * rx * x - sp * ry * y + cx, sp * rx * x + cp * ry * y + cy);
        let (q1x, q1y) = p(c1 - t * s1, s1 + t * c1);
        let (q2x, q2y) = p(c2 + t * s2, s2 - t * c2);
        let (ex, ey) = p(c2, s2);
        pb.cubic_to(q1x, q1y, q2x, q2y, ex, ey);
        a += step;
    }
}

pub fn parse_path(d: &str) -> Option<Path> {
    let mut pb = PathBuilder::new();
    let (mut cur, mut start, mut last_c) = ((0.0f32, 0.0f32), (0.0f32, 0.0f32), None::<(f32, f32)>);
    let mut i = 0;
    let chars: Vec<char> = d.chars().collect();
    while i < chars.len() {
        let cmd = chars[i];
        if !cmd.is_ascii_alphabetic() {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < chars.len() && !chars[j].is_ascii_alphabetic() {
            j += 1;
        }
        let rel = cmd.is_ascii_lowercase();
        let args_src: String = chars[i + 1..j].iter().collect();
        // arc flags may be packed ("a2 2 0 011 2"): split them out before generic number parsing
        let a = if cmd.eq_ignore_ascii_case(&'a') { arc_args(&args_src) } else { nums(&args_src) };
        let o = |p: (f32, f32), x: f32, y: f32| if rel { (p.0 + x, p.1 + y) } else { (x, y) };
        match cmd.to_ascii_uppercase() {
            'M' => {
                for (k, c) in a.chunks_exact(2).enumerate() {
                    cur = o(cur, c[0], c[1]);
                    if k == 0 {
                        pb.move_to(cur.0, cur.1);
                        start = cur;
                    } else {
                        pb.line_to(cur.0, cur.1);
                    }
                }
                last_c = None;
            }
            'L' => {
                for c in a.chunks_exact(2) {
                    cur = o(cur, c[0], c[1]);
                    pb.line_to(cur.0, cur.1);
                }
                last_c = None;
            }
            'H' => {
                for &x in &a {
                    cur.0 = if rel { cur.0 + x } else { x };
                    pb.line_to(cur.0, cur.1);
                }
                last_c = None;
            }
            'V' => {
                for &y in &a {
                    cur.1 = if rel { cur.1 + y } else { y };
                    pb.line_to(cur.0, cur.1);
                }
                last_c = None;
            }
            'C' => {
                for c in a.chunks_exact(6) {
                    let (p1, p2, p) = (o(cur, c[0], c[1]), o(cur, c[2], c[3]), o(cur, c[4], c[5]));
                    pb.cubic_to(p1.0, p1.1, p2.0, p2.1, p.0, p.1);
                    (cur, last_c) = (p, Some(p2));
                }
            }
            'S' => {
                for c in a.chunks_exact(4) {
                    let p1 = last_c.map_or(cur, |l| (2.0 * cur.0 - l.0, 2.0 * cur.1 - l.1));
                    let (p2, p) = (o(cur, c[0], c[1]), o(cur, c[2], c[3]));
                    pb.cubic_to(p1.0, p1.1, p2.0, p2.1, p.0, p.1);
                    (cur, last_c) = (p, Some(p2));
                }
            }
            'Q' => {
                for c in a.chunks_exact(4) {
                    let (p1, p) = (o(cur, c[0], c[1]), o(cur, c[2], c[3]));
                    pb.quad_to(p1.0, p1.1, p.0, p.1);
                    cur = p;
                }
                last_c = None;
            }
            'A' => {
                for c in a.chunks_exact(7) {
                    let p = o(cur, c[5], c[6]);
                    arc(&mut pb, cur, c[0], c[1], c[2], c[3] != 0.0, c[4] != 0.0, p);
                    cur = p;
                }
                last_c = None;
            }
            'Z' => {
                pb.close();
                cur = start;
                last_c = None;
            }
            _ => {}
        }
        i = j;
    }
    pb.finish()
}

/// Arc arguments where the two flags may be written without separators.
fn arc_args(s: &str) -> Vec<f32> {
    let mut out = Vec::new();
    let mut rest = s.trim_start_matches(|c: char| c == ' ' || c == ',').to_string();
    while !rest.is_empty() {
        for k in 0..7 {
            rest = rest.trim_start_matches(|c: char| c == ' ' || c == ',').to_string();
            if k == 3 || k == 4 {
                let f = rest.chars().next().unwrap_or('0');
                out.push(if f == '1' { 1.0 } else { 0.0 });
                rest.remove(0);
            } else {
                let end = (1..=rest.len())
                    .take_while(|&e| rest.is_char_boundary(e) && (e == 1 || { let c = rest.as_bytes()[e - 1] as char; c.is_ascii_digit() || c == '.' && !rest[..e - 1].contains('.') || c == 'e' }))
                    .last()
                    .unwrap_or(1);
                out.push(rest[..end].parse().unwrap_or(0.0));
                rest = rest[end..].to_string();
            }
        }
    }
    out
}

pub fn rounded_rect(x: f32, y: f32, w: f32, h: f32, r: f32) -> Option<Path> {
    let r = r.min(w / 2.0).min(h / 2.0);
    let mut pb = PathBuilder::new();
    pb.move_to(x + r, y);
    pb.line_to(x + w - r, y);
    pb.quad_to(x + w, y, x + w, y + r);
    pb.line_to(x + w, y + h - r);
    pb.quad_to(x + w, y + h, x + w - r, y + h);
    pb.line_to(x + r, y + h);
    pb.quad_to(x, y + h, x, y + h - r);
    pb.line_to(x, y + r);
    pb.quad_to(x, y, x + r, y);
    pb.close();
    pb.finish()
}

/// Draw an icon `size` pixels square with its top-left corner at (x, y).
pub fn draw(pm: &mut PixmapMut<'_>, shapes: &[Shape], x: f32, y: f32, size: f32, rgb: [u8; 3]) {
    let mut paint = Paint::default();
    paint.set_color_rgba8(rgb[0], rgb[1], rgb[2], 255);
    paint.anti_alias = true;
    let stroke = Stroke { width: 2.0, line_cap: LineCap::Round, line_join: LineJoin::Round, ..Stroke::default() };
    let tf = Transform::from_scale(size / 24.0, size / 24.0).post_translate(x, y);
    for s in shapes {
        let path = match *s {
            Shape::Path(d) => parse_path(d),
            Shape::Line(a, b, c, d) => {
                let mut pb = PathBuilder::new();
                pb.move_to(a, b);
                pb.line_to(c, d);
                pb.finish()
            }
            Shape::Rect { x, y, w, h, rx } => rounded_rect(x, y, w, h, rx),
            Shape::Circle(cx, cy, r) => PathBuilder::from_circle(cx, cy, r),
            Shape::Poly(points) => {
                let close = points.ends_with('z');
                let n = nums(points.trim_end_matches('z'));
                let mut pb = PathBuilder::new();
                for (k, c) in n.chunks_exact(2).enumerate() {
                    if k == 0 { pb.move_to(c[0], c[1]) } else { pb.line_to(c[0], c[1]) }
                }
                if close {
                    pb.close();
                }
                pb.finish()
            }
        };
        if let Some(p) = path {
            pm.stroke_path(&p, &paint, &stroke, tf, None);
        }
    }
}
