//! Reading the desktop's sign-in code (docs/android-plan.md, 3.2) with the back camera: a thread decodes the
//! pictures with `rqrr` and keeps a small upright preview for the sign-in screen. The answer goes to the window
//! client as `android::Msg::Qr` (or `ScanFailed`).
use crate::android::{post, Msg};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// The running scan's stop flag.
static RUN: Mutex<Option<Arc<AtomicBool>>> = Mutex::new(None);
/// The newest preview: width, height and RGBA bytes.
static PREVIEW: Mutex<Option<(u32, u32, Vec<u8>)>> = Mutex::new(None);

/// Width of the preview on the sign-in screen, in pixels of the picture.
const PREVIEW_W: usize = 360;

pub fn running() -> bool {
    RUN.lock().is_ok_and(|r| r.is_some())
}

/// Starts the camera and looks for a code (needs the CAMERA permission). Does nothing when a scan runs.
pub fn start() {
    let Ok(mut run) = RUN.lock() else { return };
    if run.is_some() {
        return;
    }
    let stop = Arc::new(AtomicBool::new(false));
    *run = Some(stop.clone());
    std::thread::Builder::new().name("qr-scan".into()).spawn(move || scan(&stop)).ok();
}

pub fn stop() {
    if let Some(s) = RUN.lock().ok().and_then(|mut r| r.take()) {
        s.store(true, Ordering::Relaxed);
    }
    if let Ok(mut p) = PREVIEW.lock() {
        *p = None;
    }
}

/// The camera picture to show while scanning.
pub fn preview() -> Option<(u32, u32, Vec<u8>)> {
    PREVIEW.lock().ok()?.clone()
}

fn scan(stop: &AtomicBool) {
    // a dense code (the session is about 1 kB) needs the sharper picture
    let mut cam = match crate::android_camera::Camera::open(false, 1280, 720) {
        Ok(c) => c,
        Err(e) => {
            finish(stop);
            return post(Msg::ScanFailed(e));
        }
    };
    let mut rot_at = Instant::now() - Duration::from_secs(1);
    let mut rot = 0;
    while !stop.load(Ordering::Relaxed) && !cam.lost() {
        if rot_at.elapsed() >= Duration::from_millis(500) {
            (rot, rot_at) = (crate::media::upright(cam.sensor, crate::android::display_rotation(), cam.front), Instant::now());
        }
        let got = cam.latest(|p| {
            // the luma plane alone is enough for both the preview and the reader
            let mut luma = vec![0u8; p.w * p.h];
            for y in 0..p.h {
                let row = &p.y[y * p.y_row..(y * p.y_row + p.w).min(p.y.len())];
                luma[y * p.w..y * p.w + row.len()].copy_from_slice(row);
            }
            set_preview(&luma, p.w, p.h, rot);
            post(Msg::ScanFrame);
            crate::pairing::read_qr(&luma, p.w, p.h)
        });
        match got {
            Some(Some(text)) => {
                finish(stop);
                return post(Msg::Qr(text));
            }
            Some(None) => {}
            None => std::thread::sleep(Duration::from_millis(10)),
        }
    }
    if cam.lost() && !stop.load(Ordering::Relaxed) {
        post(Msg::ScanFailed("The camera was taken by another app.".into()));
    }
    finish(stop);
}

/// The scan is over (found, failed or cancelled): forget it, unless a new one already started.
fn finish(stop: &AtomicBool) {
    if let Ok(mut run) = RUN.lock() {
        if run.as_ref().is_some_and(|r| std::ptr::eq(r.as_ref(), stop)) {
            *run = None;
        }
    }
}

/// A grey, upright, `PREVIEW_W`-wide copy of the picture.
fn set_preview(luma: &[u8], w: usize, h: usize, rot: u32) {
    let step = (w.max(h) / PREVIEW_W).max(1);
    let (sw, sh) = (w / step, h / step);
    let small: Vec<u8> = (0..sw * sh).map(|i| luma[(i / sw) * step * w + (i % sw) * step]).collect();
    let (pw, ph) = if rot % 180 == 90 { (sh, sw) } else { (sw, sh) };
    let mut turned = vec![0u8; pw * ph];
    crate::media::rotate_plane(&small, sw, 1, sw, sh, rot, &mut turned, pw);
    let rgba = turned.iter().flat_map(|&v| [v, v, v, 255]).collect();
    if let Ok(mut p) = PREVIEW.lock() {
        *p = Some((pw as u32, ph as u32, rgba));
    }
}
