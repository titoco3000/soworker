//! Transcripts of meeting recordings, made on this machine with whisper.cpp's `whisper-cli`. The program is found on
//! the PATH (or set as `whisperCli` in settings.json); on Windows it is downloaded from whisper.cpp's releases the first
//! time, elsewhere `install_help` says how to get it. The model (`whisperModel`, `base` by default) is downloaded on
//! first use. The transcript is written as `.srt` subtitles next to the recording and, for a video, also added to it as
//! a subtitle track (players such as COSMIC's do not look for a separate file). Also `soworker --transcribe <file>`.
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const MODEL_URL: &str = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main";
/// whisper.cpp's releases; not every one has Windows binaries (the newest with `WINDOWS_ASSET` is used), else `WINDOWS_ZIP`.
#[cfg(windows)]
const RELEASES: &str = "https://api.github.com/repos/ggml-org/whisper.cpp/releases?per_page=30";
#[cfg(windows)]
const WINDOWS_ASSET: &str = "whisper-bin-x64.zip";
#[cfg(windows)]
const WINDOWS_ZIP: &str = "https://github.com/ggml-org/whisper.cpp/releases/download/v1.9.2/whisper-bin-x64.zip";
/// Names whisper.cpp's command-line program goes by: its own build, and what distributions package it as.
const NAMES: &[&str] = &["whisper-cli", "whisper-cpp", "whisper.cpp"];

/// How a transcript is coming along, for the UI (it stays on screen until done, and a failure until dismissed).
#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Download { what: String, pct: Option<u8> },
    Running { pct: Option<u8> },
    Done(PathBuf),
    Failed(String),
}

impl Status {
    /// One line about it (a failure's first line only).
    pub fn line(&self) -> String {
        let pct = |p: &Option<u8>| p.map_or_else(String::new, |p| format!(" {p}%"));
        match self {
            Status::Download { what, pct: p } => format!("Downloading {what}…{}", pct(p)),
            Status::Running { pct: p } => format!("Transcribing the recording…{}", pct(p)),
            Status::Done(path) if path.extension().is_some_and(|e| e == "srt") => format!("Transcript saved: {}", path.display()),
            Status::Done(path) => format!("Subtitles added to {}", path.display()),
            Status::Failed(e) => format!("No transcript: {}", e.lines().next().unwrap_or_default()),
        }
    }
}

/// Where downloaded models (and, on Windows, whisper.cpp itself) are kept.
pub fn data_dir() -> PathBuf {
    #[cfg(windows)]
    let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from);
    #[cfg(not(windows))]
    let base = std::env::var_os("XDG_DATA_HOME").filter(|v| !v.is_empty()).map(PathBuf::from).or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")));
    base.unwrap_or_else(std::env::temp_dir).join("soworker")
}

/// `path` with `ext` appended (`ext` includes its dot). Unlike `with_extension`, a dot in the name is kept.
pub fn sibling(base: &Path, ext: &str) -> PathBuf {
    let mut s = base.as_os_str().to_owned();
    s.push(ext);
    s.into()
}

fn model_name() -> String {
    crate::calls::setting("whisperModel").as_str().filter(|s| !s.is_empty()).unwrap_or("base").to_string()
}

fn model_path() -> PathBuf {
    data_dir().join("models").join(format!("ggml-{}.bin", model_name()))
}

fn on_path(name: &str) -> Option<PathBuf> {
    let file = format!("{name}{}", std::env::consts::EXE_SUFFIX);
    std::env::split_paths(&std::env::var_os("PATH")?).map(|d| d.join(&file)).find(|p| p.is_file())
}

/// Folders (under the home folder) where people usually clone whisper.cpp to build it.
const CLONES: &[&str] = &["src", "", "dev", "dev/repos", "code", "projects", "git", "build", "Downloads"];

/// `whisper-cli`: the `whisperCli` setting, else the PATH, else a build in a whisper.cpp clone in one of the usual
/// places (`~/src/whisper.cpp/build/bin/whisper-cli`), else the copy downloaded on Windows.
pub fn find_whisper() -> Option<PathBuf> {
    if let Some(p) = crate::calls::setting("whisperCli").as_str().filter(|s| !s.is_empty()) {
        return Some(PathBuf::from(p));
    }
    let exe = format!("whisper-cli{}", std::env::consts::EXE_SUFFIX);
    let home = std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" }).map(PathBuf::from);
    let built = || {
        let home = home.as_ref()?;
        CLONES.iter().flat_map(|d| [home.join(d).join("whisper.cpp/build/bin").join(&exe), home.join(d).join("whisper.cpp/build/bin/Release").join(&exe)]).find(|p| p.is_file())
    };
    NAMES.iter().find_map(|n| on_path(n)).or_else(built).or_else(|| find_file(&data_dir().join("whisper"), &exe))
}

fn find_file(dir: &Path, name: &str) -> Option<PathBuf> {
    for e in std::fs::read_dir(dir).ok()?.flatten() {
        let p = e.path();
        if p.is_dir() {
            if let Some(f) = find_file(&p, name) {
                return Some(f);
            }
        } else if p.file_name().is_some_and(|f| f == name) {
            return Some(p);
        }
    }
    None
}

/// How to install whisper.cpp where we do not download it ourselves; `file` is the recording to transcribe after.
pub fn install_help(file: Option<&Path>) -> String {
    let file = file.map_or_else(|| "<recording>".to_string(), |f| format!("\"{}\"", f.display()));
    let bin = std::env::var_os("HOME").map_or_else(|| "~/.local/bin".into(), |h| PathBuf::from(h).join(".local/bin").display().to_string());
    format!(
        "Transcripts need whisper.cpp's whisper-cli, which is not installed.\n\
         Install it from your package manager if it has it (often named whisper-cpp), or build it:\n\n\
         \x20 git clone --depth 1 https://github.com/ggml-org/whisper.cpp\n\
         \x20 cd whisper.cpp\n\
         \x20 cmake -B build -DBUILD_SHARED_LIBS=OFF -DCMAKE_BUILD_TYPE=Release\n\
         \x20 cmake --build build -j --config Release\n\
         \x20 install -Dm755 build/bin/whisper-cli {bin}/whisper-cli\n\n\
         (needs git, cmake and a C++ compiler). Or point \"whisperCli\" in settings.json at it.\n\
         Then transcribe the recording with: soworker --transcribe {file}"
    )
}

#[cfg(windows)]
fn download_whisper(progress: &dyn Fn(Status)) -> Result<PathBuf, String> {
    let dir = data_dir().join("whisper");
    let zip = dir.join(WINDOWS_ASSET);
    let url = ureq::get(RELEASES)
        .timeout(std::time::Duration::from_secs(15))
        .call()
        .ok()
        .and_then(|r| r.into_json::<serde_json::Value>().ok())
        .and_then(|v| {
            v.as_array()?.iter().flat_map(|r| r["assets"].as_array().into_iter().flatten()).find(|a| a["name"] == WINDOWS_ASSET)?["browser_download_url"].as_str().map(String::from)
        })
        .unwrap_or_else(|| WINDOWS_ZIP.to_string());
    download(&url, &zip, "whisper.cpp", progress)?;
    // tar has come with Windows since 10 1803 and reads zip files
    let ok = Command::new("tar").arg("-xf").arg(&zip).arg("-C").arg(&dir).stdin(Stdio::null()).stdout(Stdio::null()).status().map_err(|e| format!("cannot unpack whisper.cpp: {e}"))?;
    let _ = std::fs::remove_file(&zip);
    if !ok.success() {
        return Err("cannot unpack whisper.cpp".into());
    }
    find_file(&dir, "whisper-cli.exe").ok_or_else(|| "the whisper.cpp download has no whisper-cli.exe".into())
}

fn ensure_whisper(src: &Path, progress: &dyn Fn(Status)) -> Result<PathBuf, String> {
    if let Some(p) = find_whisper() {
        return Ok(p);
    }
    #[cfg(windows)]
    {
        let _ = src;
        download_whisper(progress)
    }
    #[cfg(not(windows))]
    {
        let _ = progress;
        Err(install_help(Some(src)))
    }
}

fn ensure_model(progress: &dyn Fn(Status)) -> Result<PathBuf, String> {
    let path = model_path();
    if !path.is_file() {
        download(&format!("{MODEL_URL}/ggml-{}.bin", model_name()), &path, &format!("the whisper model ({})", model_name()), progress)?;
    }
    Ok(path)
}

/// Downloads to `dest` through a `.part` file, so an interrupted download is not mistaken for a finished one.
/// `what` names it in the progress reports.
fn download(url: &str, dest: &Path, what: &str, progress: &dyn Fn(Status)) -> Result<(), String> {
    if let Some(d) = dest.parent() {
        std::fs::create_dir_all(d).map_err(|e| format!("{}: {e}", d.display()))?;
    }
    let part = sibling(dest, ".part");
    let resp = ureq::AgentBuilder::new().timeout_connect(std::time::Duration::from_secs(15)).build().get(url).call().map_err(|e| format!("download {url}: {e}"))?;
    let total: Option<u64> = resp.header("content-length").and_then(|l| l.parse().ok());
    let mut f = std::fs::File::create(&part).map_err(|e| format!("{}: {e}", part.display()))?;
    let (mut input, mut buf, mut got, mut shown) = (resp.into_reader(), vec![0u8; 1 << 16], 0u64, None);
    progress(Status::Download { what: what.into(), pct: total.map(|_| 0) });
    loop {
        let n = input.read(&mut buf).map_err(|e| format!("download {url}: {e}"))?;
        if n == 0 {
            break;
        }
        f.write_all(&buf[..n]).map_err(|e| format!("{}: {e}", part.display()))?;
        got += n as u64;
        let pct = total.map(|t| (got * 100 / t.max(1)).min(100) as u8);
        if pct != shown {
            shown = pct;
            progress(Status::Download { what: what.into(), pct });
        }
    }
    drop(f);
    std::fs::rename(&part, dest).map_err(|e| format!("{}: {e}", dest.display()))
}

/// A 16-bit mono WAV written as it goes; the sizes in the header are filled in by `finish`.
pub struct Wav {
    f: std::io::BufWriter<std::fs::File>,
    path: PathBuf,
    bytes: u32,
}

impl Wav {
    pub fn create(path: &Path, rate: u32) -> Result<Wav, String> {
        let f = std::fs::File::create(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut w = Wav { f: std::io::BufWriter::new(f), path: path.to_path_buf(), bytes: 0 };
        let mut h = Vec::with_capacity(44);
        h.extend_from_slice(b"RIFF\0\0\0\0WAVEfmt ");
        h.extend_from_slice(&16u32.to_le_bytes());
        h.extend_from_slice(&1u16.to_le_bytes()); // PCM
        h.extend_from_slice(&1u16.to_le_bytes()); // mono
        h.extend_from_slice(&rate.to_le_bytes());
        h.extend_from_slice(&(rate * 2).to_le_bytes());
        h.extend_from_slice(&2u16.to_le_bytes());
        h.extend_from_slice(&16u16.to_le_bytes());
        h.extend_from_slice(b"data\0\0\0\0");
        w.f.write_all(&h).map_err(|e| e.to_string())?;
        Ok(w)
    }

    pub fn write(&mut self, samples: &[i16]) -> Result<(), String> {
        let bytes: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
        self.bytes = self.bytes.saturating_add(bytes.len() as u32);
        self.f.write_all(&bytes).map_err(|e| format!("{}: {e}", self.path.display()))
    }

    pub fn finish(mut self) -> Result<PathBuf, String> {
        let patch = |f: &mut std::io::BufWriter<std::fs::File>, bytes: u32| -> std::io::Result<()> {
            f.flush()?;
            let f = f.get_mut();
            f.seek(SeekFrom::Start(4))?;
            f.write_all(&(36 + bytes).to_le_bytes())?;
            f.seek(SeekFrom::Start(40))?;
            f.write_all(&bytes.to_le_bytes())
        };
        patch(&mut self.f, self.bytes).map_err(|e| format!("{}: {e}", self.path.display()))?;
        Ok(self.path)
    }
}

/// The sample rate of a 16-bit mono PCM WAV with the plain 44-byte header (what `Wav` writes), else None.
fn our_wav(path: &Path) -> Option<u32> {
    let mut h = [0u8; 44];
    std::fs::File::open(path).ok()?.read_exact(&mut h).ok()?;
    let u16_at = |i: usize| u16::from_le_bytes([h[i], h[i + 1]]);
    let rate = u32::from_le_bytes([h[24], h[25], h[26], h[27]]);
    (&h[0..4] == b"RIFF" && &h[8..16] == b"WAVEfmt " && u16_at(20) == 1 && u16_at(22) == 1 && u16_at(34) == 16 && &h[36..40] == b"data").then_some(rate)
}

/// The 16 kHz mono WAV whisper wants: our own 48 kHz recordings are brought down by averaging every three samples,
/// anything else goes through ffmpeg.
fn prepare_audio(src: &Path, dest: &Path) -> Result<(), String> {
    if let Some(rate @ (16_000 | 48_000)) = our_wav(src) {
        let step = (rate / 16_000) as usize;
        let mut input = std::io::BufReader::new(std::fs::File::open(src).map_err(|e| e.to_string())?);
        input.seek(SeekFrom::Start(44)).map_err(|e| e.to_string())?;
        let mut out = Wav::create(dest, 16_000)?;
        let mut buf = vec![0u8; 2 * step * 4096];
        loop {
            let n = input.read(&mut buf).map_err(|e| e.to_string())?;
            if n == 0 {
                break;
            }
            let n = n - n % (2 * step); // a partial group at the very end is dropped
            let samples: Vec<i16> = buf[..n].chunks_exact(2 * step).map(|g| (g.chunks_exact(2).map(|s| i16::from_le_bytes([s[0], s[1]]) as i32).sum::<i32>() / step as i32) as i16).collect();
            out.write(&samples)?;
        }
        return out.finish().map(drop);
    }
    let out = Command::new("ffmpeg")
        .args(["-hide_banner", "-loglevel", "error", "-y", "-i"])
        .arg(src)
        .args(["-vn", "-ac", "1", "-ar", "16000", "-c:a", "pcm_s16le"])
        .arg(dest)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("cannot run ffmpeg to read {}: {e}", src.display()))?;
    if !out.status.success() {
        return Err(format!("ffmpeg could not read {}: {}", src.display(), String::from_utf8_lossy(&out.stderr).trim()));
    }
    Ok(())
}

/// Subtitles made by `transcribe`, and the language they are in (ISO 639-1, as whisper names it) when known.
pub struct Transcript {
    pub srt: PathBuf,
    pub lang: Option<String>,
}

/// Transcribes `src` (any audio or video file) into `<out>.srt`. `progress` hears about the slow steps (downloads,
/// the transcription itself).
pub fn transcribe(src: &Path, out: &Path, progress: &dyn Fn(Status)) -> Result<Transcript, String> {
    let whisper = ensure_whisper(src, progress)?;
    let model = ensure_model(progress)?;
    let wav = sibling(out, ".16k.wav");
    progress(Status::Running { pct: None });
    prepare_audio(src, &wav)?;
    let lang = crate::calls::setting("whisperLanguage").as_str().filter(|s| !s.is_empty()).unwrap_or("auto").to_string();
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get().min(8));
    let started = std::time::Instant::now();
    let res = Command::new(&whisper)
        .arg("-m")
        .arg(&model)
        .arg("-f")
        .arg(&wav)
        .args(["-l", &lang, "-t", &threads.to_string(), "-osrt", "-pp", "-of"])
        .arg(out)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("cannot run {}: {e}", whisper.display()));
    let mut found = (lang != "auto").then(|| lang.clone());
    let status = res.and_then(|mut child| {
        // `-pp` reports "progress = 45%" lines on stderr, and the language it detected; the last lines explain a failure
        let mut tail = std::collections::VecDeque::new();
        if let Some(err) = child.stderr.take() {
            for line in std::io::BufRead::lines(std::io::BufReader::new(err)).map_while(Result::ok) {
                if let Some(l) = line.split("auto-detected language:").nth(1).and_then(|l| l.split_whitespace().next()) {
                    found = Some(l.to_string());
                }
                if let Some(p) = line.split("progress =").nth(1).and_then(|p| p.trim().trim_end_matches('%').trim().parse::<u8>().ok()) {
                    progress(Status::Running { pct: Some(p) });
                } else if !line.trim().is_empty() && !line.starts_with("whisper_") && !line.starts_with("ggml_") {
                    tail.push_back(line);
                    if tail.len() > 3 {
                        tail.pop_front();
                    }
                }
            }
        }
        let st = child.wait().map_err(|e| e.to_string())?;
        if st.success() { Ok(()) } else { Err(format!("whisper failed: {}", Vec::from(tail).join(" / "))) }
    });
    let _ = std::fs::remove_file(&wav);
    status?;
    eprintln!("transcribe: {} in {:.0}s with {} (language {found:?})", src.display(), started.elapsed().as_secs_f64(), whisper.display());
    Ok(Transcript { srt: sibling(out, ".srt"), lang: found })
}

/// ISO 639-2 for the languages people most likely meet in; MP4 wants three letters. Others go untagged.
fn lang3(lang: &str) -> Option<&'static str> {
    const LANGS: &[(&str, &str)] = &[
        ("en", "eng"), ("pt", "por"), ("es", "spa"), ("fr", "fra"), ("de", "deu"), ("it", "ita"), ("nl", "nld"), ("pl", "pol"), ("sv", "swe"),
        ("da", "dan"), ("no", "nor"), ("fi", "fin"), ("ru", "rus"), ("uk", "ukr"), ("tr", "tur"), ("ar", "ara"), ("he", "heb"), ("hi", "hin"),
        ("ja", "jpn"), ("ko", "kor"), ("zh", "zho"), ("cs", "ces"), ("el", "ell"), ("ro", "ron"), ("hu", "hun"), ("ca", "cat"), ("id", "ind"),
    ];
    LANGS.iter().find(|(two, _)| *two == lang).map(|(_, three)| *three)
}

/// Adds the subtitles to an MP4 as its subtitle track (replacing one it has), in place.
pub fn embed(video: &Path, t: &Transcript) -> Result<(), String> {
    let tmp = sibling(video, ".subs.mp4");
    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-hide_banner", "-loglevel", "error", "-y", "-i"]).arg(video).arg("-i").arg(&t.srt);
    cmd.args(["-map", "0:v?", "-map", "0:a?", "-map", "1:0", "-c", "copy", "-c:s", "mov_text"]);
    if let Some(l) = t.lang.as_deref().and_then(lang3) {
        cmd.arg("-metadata:s:s:0").arg(format!("language={l}"));
    }
    let res = cmd.args(["-movflags", "+faststart"]).arg(&tmp).stdin(Stdio::null()).output().map_err(|e| format!("cannot run ffmpeg: {e}"))?;
    if !res.status.success() {
        let _ = std::fs::remove_file(&tmp);
        return Err(format!("could not add the subtitles: {}", String::from_utf8_lossy(&res.stderr).trim()));
    }
    std::fs::rename(&tmp, video).map_err(|e| format!("{}: {e}", video.display()))
}

/// The transcript made for a recording: added to it when it is an MP4. Returns what to show as saved.
pub fn attach(recording: &Path, t: &Transcript) -> Result<PathBuf, String> {
    if recording.extension().is_some_and(|e| e.eq_ignore_ascii_case("mp4")) {
        embed(recording, t)?;
        return Ok(recording.to_path_buf());
    }
    Ok(t.srt.clone())
}

/// `soworker --transcribe [file]`: transcribe a recording, or with no file say what is set up and what is missing.
pub fn cli(file: Option<&str>) -> Result<(), String> {
    let Some(file) = file else {
        match find_whisper() {
            Some(p) => println!("whisper-cli: {}", p.display()),
            None if cfg!(windows) => println!("whisper-cli: not found (downloaded on first use)"),
            None => println!("{}\n", install_help(None)),
        }
        let m = model_path();
        println!("model: {} ({})", m.display(), if m.is_file() { "downloaded" } else { "downloaded on first use" });
        println!("ffmpeg: {}", if Command::new("ffmpeg").arg("-version").stdout(Stdio::null()).stderr(Stdio::null()).status().is_ok_and(|s| s.success()) { "found" } else { "not found (recordings are audio only)" });
        println!("\nusage: soworker --transcribe <recording.mp4|.wav>");
        return Ok(());
    };
    let src = Path::new(file);
    if !src.is_file() {
        return Err(format!("{file}: no such file"));
    }
    let out = src.with_extension("");
    let last = std::sync::Mutex::new(String::new());
    let t = transcribe(src, &out, &|st| {
        let line = st.line();
        if let Ok(mut l) = last.lock() {
            if *l != line {
                println!("{line}");
                *l = line;
            }
        }
    })?;
    println!("subtitles: {}", t.srt.display());
    if attach(src, &t)? != t.srt {
        println!("added to {}", src.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn our_wavs_are_brought_down_to_16k() {
        let dir = std::env::temp_dir().join(format!("soworker-wav-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let src = dir.join("a.wav");
        let mut w = Wav::create(&src, 48_000).unwrap();
        w.write(&[3, 6, 9, 30, 60, 90, 7]).unwrap();
        w.finish().unwrap();
        assert_eq!(our_wav(&src), Some(48_000));
        let dest = dir.join("b.wav");
        prepare_audio(&src, &dest).unwrap();
        let b = std::fs::read(&dest).unwrap();
        assert_eq!(our_wav(&dest), Some(16_000));
        assert_eq!(u32::from_le_bytes([b[40], b[41], b[42], b[43]]), 4, "two samples, the leftover one dropped");
        assert_eq!(&b[44..], &[6, 0, 60, 0]);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn siblings_keep_dots_in_the_name() {
        assert_eq!(sibling(Path::new("/r/2026-11-03 14-05 Room 2.0"), ".mp4"), PathBuf::from("/r/2026-11-03 14-05 Room 2.0.mp4"));
    }
}
