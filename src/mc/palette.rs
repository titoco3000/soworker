//! Video frames as Minecraft map art: the map colour palette, a 32×32×32 RGB lookup table, scaling into a screen's
//! size (letterboxed) and the binary frame message the mod writes into its maps.
use std::sync::OnceLock;

/// The base map colours of Minecraft 26.3 (`MapColor`, ids 0..=61; 0 is transparent). Checked against the 26.3 jar.
const BASE: [u32; 62] = [
    0, 8368696, 16247203, 13092807, 16711680, 10526975, 10987431, 31744, 16777215, 10791096, 9923917, 7368816, 4210943, 9402184,
    16776437, 14188339, 11685080, 6724056, 15066419, 8375321, 15892389, 5000268, 10066329, 5013401, 8339378, 3361970, 6704179,
    6717235, 10040115, 1644825, 16445005, 6085589, 4882687, 55610, 8476209, 7340544, 13742497, 10441252, 9787244, 7367818,
    12223780, 6780213, 10505550, 3746083, 8874850, 5725276, 8014168, 4996700, 4993571, 5001770, 9321518, 2430480, 12398641,
    9715553, 6035741, 1474182, 3837580, 5647422, 1356933, 6579300, 14200723, 8365974,
];
/// `MapColor.Brightness` modifiers by id: LOW, NORMAL, HIGH, LOWEST. The packed index is `base << 2 | brightness`.
const SHADE: [u32; 4] = [180, 220, 255, 135];

/// The RGB of a packed map colour index (`None` for the transparent ones, 0..=3, and past the table).
pub fn rgb(index: u8) -> Option<[u8; 3]> {
    let (base, shade) = ((index >> 2) as usize, (index & 3) as usize);
    if base == 0 || base >= BASE.len() {
        return None;
    }
    let c = BASE[base];
    let m = SHADE[shade];
    let ch = |s: u32| ((c >> s & 0xff) * m / 255) as u8;
    Some([ch(16), ch(8), ch(0)])
}

/// The nearest map colour for each of 32×32×32 RGB cells (5 bits per channel, cell centres).
fn lut() -> &'static [u8; 32768] {
    static LUT: OnceLock<Box<[u8; 32768]>> = OnceLock::new();
    LUT.get_or_init(|| {
        let colours: Vec<(u8, [i32; 3])> = (4..=255u8).filter_map(|i| rgb(i).map(|c| (i, c.map(i32::from)))).collect();
        let mut t = Box::new([0u8; 32768]);
        for (k, slot) in t.iter_mut().enumerate() {
            let q = [(k >> 10) as i32 * 8 + 4, (k >> 5 & 31) as i32 * 8 + 4, (k & 31) as i32 * 8 + 4];
            // weighted (red-mean) distance: closer to what the eye sees than plain RGB distance
            let best = colours.iter().min_by_key(|(_, c)| {
                let rm = (q[0] + c[0]) / 2;
                let (dr, dg, db) = (q[0] - c[0], q[1] - c[1], q[2] - c[2]);
                ((512 + rm) * dr * dr >> 8) + 4 * dg * dg + ((767 - rm) * db * db >> 8)
            });
            *slot = best.map_or(0, |b| b.0);
        }
        t
    })
}

/// The map colour index nearest to an RGB colour.
pub fn nearest(r: u8, g: u8, b: u8) -> u8 {
    lut()[(r as usize >> 3) << 10 | (g as usize >> 3) << 5 | b as usize >> 3]
}

/// 4×4 Bayer matrix, centred on zero, in RGB steps.
const BAYER: [[i16; 4]; 4] = [[-15, 1, -11, 5], [9, -7, 13, -3], [-9, 7, -13, 3], [15, -1, 11, -5]];

/// Scale an RGBA picture into a `w`×`h` screen, keeping its aspect ratio (black bars), and convert it to map colour
/// indices. Each screen pixel averages the source pixels it covers. `dither` adds ordered (Bayer) dithering.
pub fn encode_pixels(src_w: u32, src_h: u32, rgba: &[u8], w: u32, h: u32, dither: bool) -> Vec<u8> {
    let black = nearest(0, 0, 0);
    let mut out = vec![black; (w * h) as usize];
    if src_w == 0 || src_h == 0 || w == 0 || h == 0 || rgba.len() < (src_w * src_h * 4) as usize {
        return out;
    }
    // the largest box of the source's shape that fits the screen
    let s = (w as f64 / src_w as f64).min(h as f64 / src_h as f64);
    let (dw, dh) = (((src_w as f64 * s).round() as u32).clamp(1, w), ((src_h as f64 * s).round() as u32).clamp(1, h));
    let (ox, oy) = ((w - dw) / 2, (h - dh) / 2);
    let span = |d: u32, n: u32, sn: u32| -> (usize, usize) {
        let a = (d as u64 * sn as u64 / n as u64) as usize;
        let b = (((d + 1) as u64 * sn as u64).div_ceil(n as u64) as usize).max(a + 1).min(sn as usize);
        (a, b)
    };
    for y in 0..dh {
        let (y0, y1) = span(y, dh, src_h);
        for x in 0..dw {
            let (x0, x1) = span(x, dw, src_w);
            let mut acc = [0u32; 3];
            for sy in y0..y1 {
                let row = &rgba[(sy * src_w as usize + x0) * 4..(sy * src_w as usize + x1) * 4];
                for p in row.chunks_exact(4) {
                    acc[0] += p[0] as u32;
                    acc[1] += p[1] as u32;
                    acc[2] += p[2] as u32;
                }
            }
            let n = ((y1 - y0) * (x1 - x0)) as u32;
            let mut c = acc.map(|v| (v / n) as i16);
            if dither {
                let d = BAYER[(y & 3) as usize][(x & 3) as usize] / 2;
                c = c.map(|v| v + d);
            }
            let c = c.map(|v| v.clamp(0, 255) as u8);
            out[((oy + y) * w + ox + x) as usize] = nearest(c[0], c[1], c[2]);
        }
    }
    out
}

/// The screen size (pixels) for a video key: a screen share gets 4×3 maps, a camera 2×2.
pub fn screen_size(key: &str) -> (u32, u32) {
    if key.ends_with(":screen") { (512, 384) } else { (256, 256) }
}

/// The binary frame message: `[u8 1][u16 key length][key][u16 width][u16 height][width × height indices]`, little-endian.
pub fn frame_message(key: &str, w: u32, h: u32, pixels: &[u8]) -> Vec<u8> {
    let k = key.as_bytes();
    let mut m = Vec::with_capacity(7 + k.len() + pixels.len());
    m.push(1);
    m.extend_from_slice(&(k.len() as u16).to_le_bytes());
    m.extend_from_slice(k);
    m.extend_from_slice(&(w as u16).to_le_bytes());
    m.extend_from_slice(&(h as u16).to_le_bytes());
    m.extend_from_slice(pixels);
    m
}

/// A screen share frame for the full-screen view: RGBA to a JPEG at its own size.
pub fn jpeg(w: u32, h: u32, rgba: &[u8]) -> Result<Vec<u8>, String> {
    let rgb: Vec<u8> = rgba.chunks_exact(4).flat_map(|p| [p[0], p[1], p[2]]).collect();
    let mut out = Vec::new();
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 88)
        .encode(&rgb, w, h, image::ExtendedColorType::Rgb8)
        .map_err(|e| e.to_string())?;
    Ok(out)
}

/// The binary full-screen message: `[u8 3][u16 key length][key][u16 width][u16 height][JPEG]`, little-endian.
pub fn full_message(key: &str, w: u32, h: u32, jpeg: &[u8]) -> Vec<u8> {
    let mut m = frame_message(key, w, h, jpeg);
    m[0] = 3;
    m
}

/// Camera frames go on the avatar's face: a `FACE`×`FACE` RGB square, not map colours.
pub const FACE: u32 = 64;

/// The middle square of an RGBA picture, scaled to `n`×`n` RGB (each pixel averages the source pixels it covers).
pub fn face_pixels(src_w: u32, src_h: u32, rgba: &[u8], n: u32) -> Vec<u8> {
    let mut out = vec![0u8; (n * n * 3) as usize];
    if src_w == 0 || src_h == 0 || n == 0 || rgba.len() < (src_w * src_h * 4) as usize {
        return out;
    }
    let side = src_w.min(src_h);
    let (ox, oy) = ((src_w - side) / 2, (src_h - side) / 2);
    let span = |d: u32| -> (u32, u32) {
        let a = (d as u64 * side as u64 / n as u64) as u32;
        let b = (((d + 1) as u64 * side as u64).div_ceil(n as u64) as u32).max(a + 1).min(side);
        (a, b)
    };
    for y in 0..n {
        let (y0, y1) = span(y);
        for x in 0..n {
            let (x0, x1) = span(x);
            let mut acc = [0u32; 3];
            for sy in oy + y0..oy + y1 {
                let row = &rgba[((sy * src_w + ox + x0) * 4) as usize..((sy * src_w + ox + x1) * 4) as usize];
                for p in row.chunks_exact(4) {
                    acc[0] += p[0] as u32;
                    acc[1] += p[1] as u32;
                    acc[2] += p[2] as u32;
                }
            }
            let c = ((y1 - y0) * (x1 - x0)).max(1);
            let i = ((y * n + x) * 3) as usize;
            out[i..i + 3].copy_from_slice(&acc.map(|v| (v / c) as u8));
        }
    }
    out
}

/// The binary face message: `[u8 2][u16 key length][key][u16 width][u16 height][width × height × RGB]`, little-endian.
pub fn face_message(key: &str, w: u32, h: u32, rgb: &[u8]) -> Vec<u8> {
    let mut m = frame_message(key, w, h, rgb);
    m[0] = 2;
    m
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_full_screen_frame_is_a_jpeg_of_the_picture() {
        let rgba = [200u8, 10, 10, 255].repeat(16 * 8);
        let j = super::jpeg(16, 8, &rgba).unwrap();
        assert_eq!(&j[..2], &[0xFF, 0xD8]);
        let m = super::full_message("u:screen", 16, 8, &j);
        assert_eq!((m[0], &m[3..11], &m[11..15]), (3, &b"u:screen"[..], &[16, 0, 8, 0][..]));
    }

    #[test]
    fn a_face_is_the_middle_square_of_the_picture() {
        // 4x2: left and right columns red, the middle two green; the face is the green middle
        let mut rgba = vec![];
        for _ in 0..2 {
            for x in 0..4 {
                rgba.extend_from_slice(if x == 0 || x == 3 { &[255, 0, 0, 255] } else { &[0, 255, 0, 255] });
            }
        }
        assert_eq!(super::face_pixels(4, 2, &rgba, 1), vec![0, 255, 0]);
        let m = super::face_message("u1", 1, 1, &[1, 2, 3]);
        assert_eq!(m, vec![2, 2, 0, b'u', b'1', 1, 0, 1, 0, 1, 2, 3]);
    }

    use super::*;

    #[test]
    fn palette_matches_minecraft() {
        assert_eq!(rgb(0), None, "transparent");
        assert_eq!(rgb(3), None);
        // GRASS 0x7FB238 at HIGH (index 1 << 2 | 2) is the colour itself, at LOW scaled by 180/255
        assert_eq!(rgb(6), Some([0x7f, 0xb2, 0x38]));
        let sc = |c: u32, m: u32| (c * m / 255) as u8;
        assert_eq!(rgb(4), Some([sc(127, 180), sc(178, 180), sc(56, 180)]));
        assert_eq!(rgb(8 << 2 | 2), Some([255, 255, 255]), "SNOW");
        assert_eq!(rgb(61 << 2 | 3), Some([sc(127, 135), sc(167, 135), sc(150, 135)]), "GLOW_LICHEN, LOWEST");
        assert_eq!(rgb(62 << 2), None, "past the table");
    }

    #[test]
    fn nearest_finds_exact_palette_colours() {
        assert_eq!(rgb(nearest(255, 255, 255)), Some([255, 255, 255]));
        assert_eq!(rgb(nearest(255, 0, 0)), Some([255, 0, 0]), "FIRE");
        let dark = rgb(nearest(0, 0, 0)).unwrap();
        assert!(dark.iter().all(|&c| c < 30), "black maps to something near black: {dark:?}");
        assert!(nearest(0, 0, 0) >= 4, "never a transparent index");
        // every index the table hands out is a real colour
        assert!(lut().iter().all(|&i| rgb(i).is_some()));
    }

    #[test]
    fn letterboxes_a_wide_picture() {
        // a 4x1 white picture in a 4x4 screen: one white row, black above and below
        let white = vec![255u8; 4 * 1 * 4];
        let px = encode_pixels(4, 1, &white, 4, 4, false);
        let (w, b) = (nearest(255, 255, 255), nearest(0, 0, 0));
        assert_eq!(px.len(), 16);
        let rows: Vec<Vec<u8>> = px.chunks(4).map(|r| r.to_vec()).collect();
        assert_eq!(rows.iter().filter(|r| r.iter().all(|&p| p == w)).count(), 1);
        assert_eq!(rows.iter().filter(|r| r.iter().all(|&p| p == b)).count(), 3);
        assert!(rows[1].iter().all(|&p| p == w), "centred: (4 - 1) / 2 = row 1");
    }

    #[test]
    fn downscaling_averages() {
        // 2x2 checker of white and black into 1x1: grey
        let px = [255, 255, 255, 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255, 255];
        let out = encode_pixels(2, 2, &px, 1, 1, false);
        assert_eq!(out, [nearest(127, 127, 127)]);
        // a tall picture: pillarboxed, the middle column is the picture
        let red = [255u8, 0, 0, 255].repeat(1 * 8);
        let out = encode_pixels(1, 8, &red, 3, 2, false);
        assert_eq!(out[1], nearest(255, 0, 0));
        assert_eq!(out[0], nearest(0, 0, 0));
        assert_eq!(out[2], nearest(0, 0, 0));
    }

    #[test]
    fn bad_input_gives_a_black_screen() {
        let out = encode_pixels(10, 10, &[0; 3], 4, 4, true);
        assert!(out.iter().all(|&p| p == nearest(0, 0, 0)));
    }

    #[test]
    fn dithering_mixes_neighbouring_colours() {
        // a flat colour between two palette entries comes out as more than one index with dithering, one without
        let grey = [120u8, 120, 120, 255].repeat(64);
        let flat = encode_pixels(8, 8, &grey, 8, 8, false);
        let dith = encode_pixels(8, 8, &grey, 8, 8, true);
        let distinct = |v: &[u8]| v.iter().collect::<std::collections::HashSet<_>>().len();
        assert_eq!(distinct(&flat), 1);
        assert!(distinct(&dith) > 1);
    }

    #[test]
    fn frame_message_layout() {
        let m = frame_message("ana", 2, 1, &[7, 9]);
        assert_eq!(m, [1, 3, 0, b'a', b'n', b'a', 2, 0, 1, 0, 7, 9]);
        assert_eq!(screen_size("ana:screen"), (512, 384));
        assert_eq!(screen_size("me"), (256, 256));
    }
}
