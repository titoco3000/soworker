//! Avatar pictures: SoWork's own sources, fetched once per URL on a background thread.
//! A user's uploaded photo wins; otherwise the server-rendered character portrait
//! (`app.sowork.com/portrait/v1/meeting!idle/<compact character string>`, no auth needed).
use crate::proto::Net;
use std::collections::HashSet;
use std::io::Read;
use std::sync::mpsc::{channel, Sender};

pub const SIZE: u32 = 96; // source resolution; drawn scaled to the tile or map size
const MAX_BYTES: u64 = 2 * 1024 * 1024;

pub fn source_url(photo: Option<&str>, portrait: Option<&str>) -> Option<String> {
    photo
        .map(String::from)
        .or_else(|| portrait.map(|p| format!("https://app.sowork.com/portrait/v1/meeting!idle/{p}")))
}

/// Decode and fit into a SIZE x SIZE circle (premultiplied RGBA).
/// Near-square pictures (photos) are cropped to a centred square and fill the circle. Anything clearly taller or
/// wider (character portraits are full-body) is scaled to fit entirely inside the circle with a small margin, so the
/// head and feet are never cut off; the empty parts stay transparent.
pub fn decode_circle(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let img = image::load_from_memory(bytes).map_err(|e| e.to_string())?.to_rgba8();
    let (w, h) = (img.width().max(1), img.height().max(1));
    let aspect = w.max(h) as f32 / w.min(h) as f32;
    let mut out = if aspect < 1.15 {
        let side = w.min(h);
        let sq = image::imageops::crop_imm(&img, (w - side) / 2, (h - side) / 2, side, side).to_image();
        image::imageops::resize(&sq, SIZE, SIZE, image::imageops::FilterType::Triangle)
    } else {
        let scale = 0.94 * SIZE as f32 / w.max(h) as f32;
        let (nw, nh) = (((w as f32 * scale).round() as u32).max(1), ((h as f32 * scale).round() as u32).max(1));
        let small = image::imageops::resize(&img, nw, nh, image::imageops::FilterType::Triangle);
        let mut canvas = image::RgbaImage::new(SIZE, SIZE);
        image::imageops::overlay(&mut canvas, &small, ((SIZE - nw) / 2) as i64, ((SIZE - nh) / 2) as i64);
        canvas
    }
    .into_raw();
    let r = SIZE as f32 / 2.0;
    for (i, px) in out.chunks_exact_mut(4).enumerate() {
        let (x, y) = ((i as u32 % SIZE) as f32 + 0.5, (i as u32 / SIZE) as f32 + 0.5);
        let cover = (r - ((x - r).hypot(y - r)) + 0.5).clamp(0.0, 1.0);
        let a = (px[3] as f32 / 255.0) * cover;
        for c in &mut px[..3] {
            *c = (*c as f32 * a) as u8;
        }
        px[3] = (a * 255.0) as u8;
    }
    Ok(out)
}

fn fetch(url: &str) -> Result<Vec<u8>, String> {
    let resp = ureq::get(url).timeout(std::time::Duration::from_secs(10)).call().map_err(|e| e.to_string())?;
    let mut buf = Vec::new();
    resp.into_reader().take(MAX_BYTES).read_to_end(&mut buf).map_err(|e| e.to_string())?;
    Ok(buf)
}

/// Starts the loader thread. Send `(user id, url)`; each decoded picture comes back as `Net::Avatar` through `emit`.
pub fn spawn(emit: impl Fn(Net) + Send + 'static) -> Sender<(String, String)> {
    let (tx, rx) = channel::<(String, String)>();
    std::thread::Builder::new()
        .name("avatars".into())
        .spawn(move || {
            let mut failed: HashSet<String> = HashSet::new();
            for (id, url) in rx {
                if failed.contains(&url) {
                    continue;
                }
                match fetch(&url).and_then(|b| decode_circle(&b)) {
                    Ok(rgba) => emit(Net::Avatar { id, size: SIZE, rgba }),
                    Err(e) => {
                        eprintln!("avatar {url}: {e}");
                        failed.insert(url);
                    }
                }
            }
        })
        .expect("avatar thread");
    tx
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encoded(fmt: image::ImageFormat) -> Vec<u8> {
        // 200x100 gradient: wider than tall, so the square crop path runs
        let img = image::RgbaImage::from_fn(200, 100, |x, y| image::Rgba([(x % 256) as u8, (y * 2) as u8, 90, 255]));
        let mut out = std::io::Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(img).write_to(&mut out, fmt).unwrap();
        out.into_inner()
    }

    #[test]
    fn decodes_png_jpeg_and_webp_into_a_circle() {
        for fmt in [image::ImageFormat::Png, image::ImageFormat::Jpeg, image::ImageFormat::WebP] {
            let px = decode_circle(&encoded(fmt)).unwrap_or_else(|e| panic!("{fmt:?}: {e}"));
            assert_eq!(px.len(), (SIZE * SIZE * 4) as usize);
            let alpha = |x: u32, y: u32| px[((y * SIZE + x) * 4 + 3) as usize];
            assert_eq!(alpha(0, 0), 0, "{fmt:?}: corner is outside the circle");
            assert_eq!(alpha(SIZE / 2, SIZE / 2), 255, "{fmt:?}: centre is opaque");
        }
    }

    #[test]
    fn tall_portraits_fit_inside_the_circle_without_cropping() {
        // a 100x300 opaque image: its top and bottom edges (head, feet) must both survive
        let img = image::RgbaImage::from_pixel(100, 300, image::Rgba([200, 80, 80, 255]));
        let mut enc = std::io::Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(img).write_to(&mut enc, image::ImageFormat::Png).unwrap();
        let px = decode_circle(&enc.into_inner()).unwrap();
        let alpha = |x: u32, y: u32| px[((y * SIZE + x) * 4 + 3) as usize];
        assert!(alpha(SIZE / 2, 5) > 200, "head (top centre) is kept");
        assert!(alpha(SIZE / 2, SIZE - 6) > 200, "feet (bottom centre) are kept");
        assert_eq!(alpha(5, SIZE / 2), 0, "sides are empty: the picture is narrower than the circle");
        // the whole picture is visible: its opaque rows span almost the full height
        let rows: Vec<u32> = (0..SIZE).filter(|&y| alpha(SIZE / 2, y) > 200).collect();
        assert!(rows.first().unwrap() <= &5 && rows.last().unwrap() >= &(SIZE - 6));
    }

    #[test]
    fn url_prefers_photo_then_portrait() {
        assert_eq!(source_url(Some("https://x/p.webp"), Some("characterA!body:~v1")).as_deref(), Some("https://x/p.webp"));
        assert_eq!(
            source_url(None, Some("characterA!body:~v1+typeA")).as_deref(),
            Some("https://app.sowork.com/portrait/v1/meeting!idle/characterA!body:~v1+typeA")
        );
        assert_eq!(source_url(None, None), None);
    }
}
