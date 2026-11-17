//! Android glue (docs/android-plan.md): logging to logcat, the JNI bridge to the Kotlin side (`Native.kt` in
//! android/app), and what Kotlin reports (back, media buttons, screen lock, insets, permissions, sign-in).
//! Every decision stays in Rust: Kotlin only forwards events and runs what Android needs a Java class for (the
//! foreground service, the media session, the permission dialogs, the sign-in WebView).
use crate::backend::Event;
use jni::objects::{GlobalRef, JClass, JObject, JValue};
use jni::JavaVM;
use std::sync::{Mutex, OnceLock};
use winit::event_loop::EventLoopProxy;

pub use winit::platform::android::activity::AndroidApp;

/// What Kotlin reports, posted to the event loop as `Event::Android`.
#[derive(Debug)]
pub enum Msg {
    /// The back button or gesture.
    Back,
    /// A media button or a notification action: 0 play/pause (toggle the mic), 1 play (unmute), 2 pause (mute),
    /// 3 leave the call.
    Media(i32),
    /// The screen was locked or unlocked.
    Screen { locked: bool },
    /// The system bars or the keyboard moved (read them with `insets`).
    Insets,
    /// A permission dialog was answered.
    Permission { name: String, granted: bool },
    /// The sign-in WebView captured a session.
    Login { refresh_token: String, api_key: String, group: String, alias: String },
    /// A QR code was read (phone pairing).
    Qr(String),
    /// The scan has a new camera picture to show.
    ScanFrame,
    /// The scan could not run (no camera, or it was taken away).
    ScanFailed(String),
}

/// Where the next picked photo goes (`photo::pick_bytes` waits on it).
static PHOTO: Mutex<Option<std::sync::mpsc::Sender<Vec<u8>>>> = Mutex::new(None);

pub fn photo_wait() -> std::sync::mpsc::Receiver<Vec<u8>> {
    let (tx, rx) = std::sync::mpsc::channel();
    if let Ok(mut p) = PHOTO.lock() {
        *p = Some(tx);
    }
    rx
}

/// The photo picker answered: the picture's bytes, empty when cancelled.
pub fn photo_picked(bytes: Vec<u8>) {
    if let Some(tx) = PHOTO.lock().ok().and_then(|mut p| p.take()) {
        let _ = tx.send(bytes);
    }
}

pub const RECORD_AUDIO: &str = "android.permission.RECORD_AUDIO";
pub const CAMERA: &str = "android.permission.CAMERA";
pub const POST_NOTIFICATIONS: &str = "android.permission.POST_NOTIFICATIONS";

static PROXY: OnceLock<EventLoopProxy<Event>> = OnceLock::new();
static VM: OnceLock<JavaVM> = OnceLock::new();
/// The `dev.soworker.app.Native` class, found through the app's class loader (a native thread's FindClass only sees
/// the system classes).
static NATIVE: OnceLock<GlobalRef> = OnceLock::new();
/// top, bottom, left, right, keyboard (window pixels)
static INSETS: Mutex<[f32; 5]> = Mutex::new([0.0; 5]);

/// Hand a message to the event loop (dropped before it runs).
pub fn post(m: Msg) {
    if let Some(p) = PROXY.get() {
        let _ = p.send_event(Event::Android(m));
    }
}

pub fn set_insets(v: [f32; 5]) {
    if let Ok(mut i) = INSETS.lock() {
        *i = v;
    }
    post(Msg::Insets);
}

pub fn insets() -> crate::app::Insets {
    let i = INSETS.lock().map(|i| *i).unwrap_or_default();
    crate::app::Insets { top: i[0], bottom: i[1], left: i[2], right: i[3], ime: i[4] }
}

pub(crate) fn set_proxy(p: EventLoopProxy<Event>) {
    let _ = PROXY.set(p);
}

/// stdout and stderr go to logcat (tag `soworker`): everything the app prints with eprintln! shows in `adb logcat`.
fn log_to_logcat() {
    #[link(name = "log")]
    extern "C" {
        fn __android_log_write(prio: i32, tag: *const std::ffi::c_char, text: *const std::ffi::c_char) -> i32;
    }
    let mut fds = [0; 2];
    // SAFETY: plain libc calls on fds we own.
    unsafe {
        if libc::pipe(fds.as_mut_ptr()) != 0 {
            return;
        }
        libc::dup2(fds[1], 1);
        libc::dup2(fds[1], 2);
    }
    let read = fds[0];
    std::thread::Builder::new()
        .name("logcat".into())
        .spawn(move || {
            use std::io::BufRead;
            use std::os::fd::FromRawFd;
            // SAFETY: the read end of the pipe made above, owned by this thread from now on.
            let f = unsafe { std::fs::File::from_raw_fd(read) };
            let tag = c"soworker";
            for line in std::io::BufReader::new(f).lines().map_while(Result::ok) {
                if let Ok(c) = std::ffi::CString::new(line) {
                    unsafe { __android_log_write(4, tag.as_ptr(), c.as_ptr()) };
                }
            }
        })
        .ok();
}

/// Run `f` with a JNI environment for this thread and the `Native` class. None when the bridge is not up, or a Java
/// exception was thrown (it is printed and cleared).
fn with_native<T>(f: impl FnOnce(&mut jni::JNIEnv, &JClass) -> jni::errors::Result<T>) -> Option<T> {
    let (vm, class) = (VM.get()?, NATIVE.get()?);
    let mut env = vm.attach_current_thread_permanently().ok()?;
    let class: &JClass = class.as_obj().into();
    match f(&mut env, class) {
        Ok(v) => Some(v),
        Err(e) => {
            if env.exception_check().unwrap_or(false) {
                let _ = env.exception_describe();
                let _ = env.exception_clear();
            }
            eprintln!("android: JNI call failed: {e}");
            None
        }
    }
}

fn call_void(name: &str, sig: &str, args: &[JValue]) {
    with_native(|env, class| env.call_static_method(class, name, sig, args).map(drop));
}

fn call_str(name: &str, s: &str) {
    with_native(|env, class| {
        let js = env.new_string(s)?;
        env.call_static_method(class, name, "(Ljava/lang/String;)V", &[JValue::Object(&js)]).map(drop)
    });
}

/// The call started or changed: the foreground service keeps it running with the screen locked, its notification
/// shows the meeting with Mute and Leave.
pub fn start_call(title: &str, mic: bool, camera: bool) {
    with_native(|env, class| {
        let js = env.new_string(title)?;
        env.call_static_method(class, "startCall", "(Ljava/lang/String;ZZ)V", &[JValue::Object(&js), JValue::Bool(mic.into()), JValue::Bool(camera.into())]).map(drop)
    });
}

pub fn stop_call() {
    call_void("stopCall", "()V", &[]);
}

/// The mic changed: the media session's play state and the notification's Mute / Unmute follow.
pub fn set_mic_state(on: bool) {
    call_void("setMicState", "(Z)V", &[JValue::Bool(on.into())]);
}

/// Nothing left to close: send the app to the background without finishing it (calls and the socket go on).
pub fn move_task_to_back() {
    call_void("moveTaskToBack", "()V", &[]);
}

pub fn has_permission(name: &str) -> bool {
    with_native(|env, class| {
        let js = env.new_string(name)?;
        env.call_static_method(class, "hasPermission", "(Ljava/lang/String;)Z", &[JValue::Object(&js)])?.z()
    })
    .unwrap_or(false)
}

/// Ask for permissions (comma-separated names); each answer comes back as `Msg::Permission`.
pub fn request_permissions(names: &[&str]) {
    call_str("requestPermissions", &names.join(","));
}

/// Where the call's sound goes: "speaker", "earpiece", "headset", "bluetooth" or "auto" (headset when there is one,
/// else the speaker).
pub fn set_audio_route(route: &str) {
    call_str("setAudioRoute", route);
}

/// The audio outputs there are now (ids of `set_audio_route`).
pub fn audio_routes() -> Vec<String> {
    with_native(|env, class| {
        let v = env.call_static_method(class, "audioRoutes", "()Ljava/lang/String;", &[])?.l()?;
        let s: String = env.get_string(&v.into())?.into();
        Ok(s.split(',').filter(|s| !s.is_empty()).map(String::from).collect())
    })
    .unwrap_or_default()
}

/// Open the sign-in page (WebView); the session comes back as `Msg::Login`.
pub fn open_login() {
    call_void("openLogin", "()V", &[]);
}

/// Open the system photo picker; the picture comes back as `Msg::Photo`.
pub fn pick_photo() {
    call_void("pickPhoto", "()V", &[]);
}

/// Open the QR scanner (phone pairing); the text comes back as `Msg::Qr`.
pub fn scan_qr() {
    call_void("scanQr", "()V", &[]);
}

/// Play one of the UI sounds (an embedded WAV) as a notification sound, so it does not duck the call.
pub fn play_sound(wav: &[u8]) {
    with_native(|env, class| {
        let arr = env.byte_array_from_slice(wav)?;
        env.call_static_method(class, "playSound", "([B)V", &[JValue::Object(&arr)]).map(drop)
    });
}

/// How far the screen is turned from the phone's natural orientation, in degrees (0, 90, 180, 270).
pub fn display_rotation() -> u32 {
    with_native(|env, class| Ok(env.call_static_method(class, "displayRotation", "()I", &[])?.i()?)).map_or(0, |r| (r.rem_euclid(4) * 90) as u32)
}

/// The intent's `soworker_args` extra (`adb shell am start -e soworker_args "--exit-after 20"`), as arguments.
fn intent_args(env: &mut jni::JNIEnv, activity: &JObject) -> jni::errors::Result<Vec<String>> {
    let intent = env.call_method(activity, "getIntent", "()Landroid/content/Intent;", &[])?.l()?;
    if intent.is_null() {
        return Ok(vec![]);
    }
    let key = env.new_string("soworker_args")?;
    let v = env.call_method(&intent, "getStringExtra", "(Ljava/lang/String;)Ljava/lang/String;", &[JValue::Object(&key)])?.l()?;
    if v.is_null() {
        return Ok(vec![]);
    }
    let s: String = env.get_string(&v.into())?.into();
    Ok(s.split_whitespace().map(String::from).collect())
}

/// `JNI_OnLoad`, on the Java thread that loads the library: WebRTC looks its Java classes up there (a native thread
/// only sees the system's).
pub fn on_load(vm: &JavaVM) {
    #[cfg(feature = "calls")]
    livekit::webrtc::android::initialize_android(vm);
}

/// `Native.initContext`, from the activity's onCreate on the main thread: WebRTC's Android context (audio).
pub fn init_context(env: &mut jni::JNIEnv, ctx: &JObject) {
    let Ok(vm) = env.get_java_vm() else { return };
    #[cfg(feature = "calls")]
    if !livekit::webrtc::android::initialize_android_context(&vm, ctx) {
        eprintln!("android: WebRTC context init failed (no call audio)");
    }
    let _ = (vm, ctx);
}

/// `android_main`: set up logging, the config folder and the JNI bridge, then run the window client.
pub fn main(app: AndroidApp) {
    log_to_logcat();
    std::panic::set_hook(Box::new(|p| eprintln!("panic: {p}")));
    if let Some(dir) = app.internal_data_path() {
        // session.json and settings.json land in the app's private storage (`files/soworker/`)
        std::env::set_var("XDG_CONFIG_HOME", &dir);
        std::env::set_var("HOME", &dir);
    }
    let mut args = vec![];
    // SAFETY: android-activity hands us the process's JavaVM and the activity, both valid for the app's lifetime.
    match unsafe { JavaVM::from_raw(app.vm_as_ptr() as *mut _) } {
        Ok(vm) => {
            let activity = unsafe { JObject::from_raw(app.activity_as_ptr() as jni::sys::jobject) };
            if let Ok(mut env) = vm.attach_current_thread_permanently() {
                let found = (|| -> jni::errors::Result<GlobalRef> {
                    let loader = env.call_method(&activity, "getClassLoader", "()Ljava/lang/ClassLoader;", &[])?.l()?;
                    let name = env.new_string("dev.soworker.app.Native")?;
                    let class = env.call_method(&loader, "loadClass", "(Ljava/lang/String;)Ljava/lang/Class;", &[JValue::Object(&name)])?.l()?;
                    env.new_global_ref(class)
                })();
                match found {
                    Ok(c) => drop(NATIVE.set(c)),
                    Err(e) => {
                        let _ = env.exception_clear();
                        eprintln!("android: no Native class: {e}");
                    }
                }
                args = intent_args(&mut env, &activity).unwrap_or_else(|_| {
                    let _ = env.exception_clear();
                    vec![]
                });
            }
            let _ = VM.set(vm);
        }
        Err(e) => eprintln!("android: no JavaVM: {e}"),
    }
    eprintln!("soworker {} on Android, args {args:?}", env!("CARGO_PKG_VERSION"));
    if let Err(e) = crate::gui::run_android(app, &args) {
        eprintln!("error: {e}");
    }
}
