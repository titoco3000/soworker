//! Inline video through the kitty graphics protocol (kitty, ghostty, WezTerm, Konsole...). Detection asks the terminal
//! once, before the input thread starts; frames are sent as raw RGB, replacing the same image id each time.
use crate::app::Frame;
use base64::Engine;
use ratatui::layout::Rect;
use std::io::Write;

const ID: u32 = 7;

/// Does this terminal draw kitty graphics? Sends a 1x1 query plus a device-attributes request (every terminal answers
/// the latter, so we never wait out a timeout on terminals that ignore the former). Needs raw mode.
pub fn probe() -> bool {
    if std::env::var_os("TMUX").is_some() || std::env::var_os("STY").is_some() || matches!(std::env::var("TERM").as_deref(), Ok("linux" | "dumb" | "")) {
        return false; // multiplexers swallow the escape codes
    }
    let mut out = std::io::stdout();
    if out.write_all(b"\x1b_Gi=31,s=1,v=1,a=q,t=d,f=24;AAAA\x1b\\\x1b[c").and_then(|_| out.flush()).is_err() {
        return false;
    }
    let mut buf = Vec::new();
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(500);
    while std::time::Instant::now() < deadline {
        let left = deadline.saturating_duration_since(std::time::Instant::now()).as_millis() as i32;
        let mut p = libc::pollfd { fd: 0, events: libc::POLLIN, revents: 0 };
        // SAFETY: one valid pollfd, and `read` gets a buffer that is at least as long as the length passed.
        let ready = unsafe { libc::poll(&mut p, 1, left) };
        if ready <= 0 {
            break;
        }
        let mut chunk = [0u8; 256];
        let n = unsafe { libc::read(0, chunk.as_mut_ptr().cast(), chunk.len()) };
        if n <= 0 {
            break;
        }
        buf.extend_from_slice(&chunk[..n as usize]);
        if let Some(i) = buf.windows(3).position(|w| w == b"\x1b[?") {
            if buf[i..].contains(&b'c') {
                break; // the device attributes came back: everything the terminal had to say is in
            }
        }
    }
    buf.windows(9).any(|w| w == b"_Gi=31;OK")
}

/// Pixels per terminal cell, if the terminal tells.
pub fn cell_px() -> Option<(f32, f32)> {
    let ws = ratatui::crossterm::terminal::window_size().ok()?;
    (ws.width > 0 && ws.height > 0 && ws.columns > 0 && ws.rows > 0).then(|| (ws.width as f32 / ws.columns as f32, ws.height as f32 / ws.rows as f32))
}

#[derive(Default)]
pub struct Kitty {
    shown: bool,
}

impl Kitty {
    /// Draw `f` inside `area` (cells), keeping its proportions and centring it.
    pub fn show(&mut self, f: &Frame, area: Rect) {
        if f.w == 0 || f.h == 0 || area.width == 0 || area.height == 0 || f.rgba.len() != (f.w * f.h * 4) as usize {
            return self.hide();
        }
        let (cw, ch) = cell_px().unwrap_or((8.0, 16.0));
        let ar = f.w as f32 / f.h as f32;
        let (bw, bh) = (area.width as f32 * cw, area.height as f32 * ch);
        let wpx = bw.min(bh * ar);
        let cols = ((wpx / cw).round() as u16).clamp(1, area.width);
        let rows = ((cols as f32 * cw / ar / ch).round() as u16).clamp(1, area.height);
        let (x, y) = (area.x + (area.width - cols) / 2, area.y + (area.height - rows) / 2);
        let rgb: Vec<u8> = f.rgba.chunks_exact(4).flat_map(|p| [p[0], p[1], p[2]]).collect();
        let b64 = base64::engine::general_purpose::STANDARD.encode(rgb);
        let mut out = Vec::with_capacity(b64.len() + b64.len() / 512 * 16 + 128);
        let _ = write!(out, "\x1b7\x1b[{};{}H", y + 1, x + 1);
        let chunks: Vec<&[u8]> = b64.as_bytes().chunks(4096).collect();
        for (i, c) in chunks.iter().enumerate() {
            let more = (i + 1 < chunks.len()) as u8;
            if i == 0 {
                let _ = write!(out, "\x1b_Ga=T,f=24,s={},v={},i={ID},p=1,c={cols},r={rows},C=1,q=2,m={more};", f.w, f.h);
            } else {
                let _ = write!(out, "\x1b_Gm={more},q=2;");
            }
            out.extend_from_slice(c);
            out.extend_from_slice(b"\x1b\\");
        }
        out.extend_from_slice(b"\x1b8");
        let mut so = std::io::stdout().lock();
        let _ = so.write_all(&out).and_then(|_| so.flush());
        self.shown = true;
    }

    pub fn hide(&mut self) {
        if std::mem::take(&mut self.shown) {
            let mut so = std::io::stdout().lock();
            let _ = write!(so, "\x1b_Ga=d,d=I,i={ID},q=2\x1b\\").and_then(|_| so.flush());
        }
    }
}
