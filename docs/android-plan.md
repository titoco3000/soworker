# SoWorker on Android: implementation plan

Status (2026-11-16): implemented through step 9 (section 11); see "Progress" at the end for what was checked on the
phone and what still waits for a live call or the user. Decisions taken with the user: QR pairing for Google sign-in, low-data mode on
the desktop app too, sideloaded APKs only (no Play Store), no notifications in the background outside calls. Written 2026-11-16 against soworker 0.1.8 (winit 0.30.13, softbuffer 0.4.8,
livekit 0.9.3 / libwebrtc 0.3.50 / webrtc-sys 0.3.47, android-activity 0.6.1).

The implementer has an Android phone on `adb`. Every phase below ends with checks you can run on that phone
(section 10 lists the adb commands).

## 0. Can this stack do it? Yes.

The window client is a software-rendered winit app: tiny-skia draws into a pixmap and softbuffer presents it. No GPU
or toolkit is involved, and the panel layout already has a phone mode (`panels::is_mobile`, header, footer, one panel
at a time). Each layer has an Android backend in the exact versions we lock:

| Layer | Android support | Notes |
|---|---|---|
| winit 0.30.13 | yes, through `android-activity` | `android-game-activity` / `android-native-activity` features. Touch arrives as `WindowEvent::Touch`, not mouse events. All keys except volume are reported `Handled`. `set_ime_allowed` shows and hides the soft keyboard. |
| softbuffer 0.4.8 | yes (`src/backends/android.rs`) | Presents into the `ANativeWindow`. |
| tiny-skia, fontdue, image, rmpv, serde, ureq, tungstenite, rustls + webpki-roots | pure Rust | No change needed. |
| livekit 0.9.3 / libwebrtc | yes | `webrtc-sys` builds for `target_os = "android"` (links EGL, OpenSLES, libc++) and copies `libwebrtc.jar` next to the build output. Before any `PlatformAudio`, call `livekit::webrtc::android::initialize_android_context(&vm, &context)`. Audio then goes through WebRTC's Java audio device module (AudioRecord / AudioTrack, with the hardware echo canceller). |

Things that do **not** carry over and need Android versions: Chrome login, the V4L2 camera, `parec` / `pw-record` /
`pactl` (noise-cancelled mic, device lists), `pw-play` (UI sounds), `notify-send`, the xdg file picker (profile photo),
the Wayland overlay, the desktop entry, the terminal client and mode switching, push-to-talk portal and socket, and
recording / transcription (ffmpeg, whisper). Each one is either replaced (sections 3 to 8) or compiled out (section 1).

A Java/Kotlin layer is unavoidable. Android requires a declared `Service` class for a foreground service (needed for
calls with the screen locked) and a `MediaSession` for play/pause. Both are small: keep every decision in Rust and let
Kotlin only forward events.

## 1. Build and project layout

### 1.1 Crate

- Add `crate-type = ["cdylib", "rlib"]` through a `[lib]` target, or a separate `soworker-android` crate in a
  workspace (`android/rust/`) that depends on `soworker` as a library. **Recommended:** the workspace crate. The
  desktop binary stays untouched, and the Android crate holds `android_main`, the JNI exports and the Android glue
  modules. This needs `src/lib.rs` re-exporting what `main.rs` uses today (mechanical change).
- New feature `android` in soworker: `gui` (window + render), `calls`, and none of `tui`, `portal`, `overlay`, `mc`.
  Gate these modules behind `cfg(not(target_os = "android"))` or features: `desktop`, `mode`, `overlay`, `talk`
  (socket + portal), `login` (Chrome), `record`, `transcribe`, `videowin`, plus the Linux paths in `devices`, `mic`
  and `sound`.
- Target-specific dependencies:
  - `winit` with `default-features = false, features = ["rwh_06", "android-game-activity"]` for
    `cfg(target_os = "android")`.
  - `softbuffer` with `default-features = false` (its Android backend has no feature flag).
  - `livekit` with `rustls-tls-webpki-roots` instead of `rustls-tls-native-roots`: native-certs has no Android store
    reader. Drop `glib-main-loop`, which is Linux only.
  - `jni = "0.21"` (the version libwebrtc uses), `ndk-context`, `android-activity` (re-exported by winit as
    `winit::platform::android::activity`).
  - `cpal` (already used on Windows) for the noise-cancelled mic and UI sounds. It uses AAudio on Android. Optional
    in phase 1 (see 5.3).
- Fonts: `render.rs` looks in `/usr/share/fonts`. On Android, embed one TTF with `include_bytes!` (Liberation Sans or
  Roboto static, about 150 KB) instead of reading `/system/fonts`, whose Roboto is a variable font and varies by vendor.
- Paths: at startup set `XDG_CONFIG_HOME` to `AndroidApp::internal_data_path()` before anything reads
  `session::default_path()`. `session.json` and `settings.json` then land in the app's private storage, with no other
  code changes.

### 1.2 Toolchain

- `rustup target add aarch64-linux-android` (one ABI is enough: arm64-v8a. Add `x86_64-linux-android` only for the
  emulator).
- Android NDK r27+ (`ANDROID_NDK_HOME`). `webrtc_sys_build::android_ndk_toolchain` finds it. `cargo-ndk` builds the
  `.so` into `android/app/src/main/jniLibs/arm64-v8a/`.
- libwebrtc prebuilt: the Makefile exports `LK_CUSTOM_WEBRTC=third_party/libwebrtc`, which is the **linux-x64** build.
  For Android, unset it or point it at `third_party/libwebrtc-android-arm64`. That folder is
  `webrtc-android-arm64-release.zip` from the same rust-sdks release as the Linux one (`webrtc-89d790b`, see the
  Makefile comment); record its sha256 the same way. `webrtc-sys` copies `libwebrtc.jar` to
  `target/aarch64-linux-android/<profile>/`. Gradle must pick it up (`implementation files(...)`), because the Java
  audio device module lives in it.
- New Makefile targets: `android` (cargo ndk + `./gradlew assembleDebug`) and `android-run` (`adb install -r`,
  `adb shell am start`, `adb logcat` filtered by the app's pid).

### 1.3 Android project (`android/`)

Gradle project, Kotlin, `minSdk 26`, `compileSdk`/`targetSdk 36`. Sideloaded only (no Play Store), so no Play policy
reviews (FGS type justifications, data-safety forms) and no Play signing. Sign release APKs with a key of our own,
kept outside the repo (path and passwords from env vars in `gradle.properties` or `~/.gradle`). Ship the APK as a
release asset next to the desktop builds. Every update must be signed with the same key, or Android refuses to
install it over the old one. Back the key up.

- `MainActivity : GameActivity` (`androidx.games:games-activity:4.4.0`, the version android-activity 0.6.1 is built
  against). GameActivity rather than NativeActivity because it gives real IME text input (`text_input_state`). With
  NativeActivity, Gboard's committed text never reaches us.
  `meta-data android.app.lib_name = soworker_android`.
- `CallService : Service`: the foreground service, `MediaSession`, wake locks (section 6).
- `Native.kt`: the JNI surface. Kotlin to Rust: `onBack()`, `onMediaButton(kind)`, `onScreen(locked: Boolean)`,
  `onInsets(top, bottom, left, right)`, `onPermission(name, granted)`, `onLoginResult(refreshToken, apiKey, group,
  alias)`. Rust to Kotlin, as static methods called through `jni`: `startCall(title)`, `stopCall()`,
  `setMicState(on)`, `moveTaskToBack()`, `requestPermissions([...])`, `setAudioRoute(route)`, `notify(text)`,
  `openLogin()`.
- Manifest permissions: `INTERNET`, `ACCESS_NETWORK_STATE`, `RECORD_AUDIO`, `CAMERA`, `MODIFY_AUDIO_SETTINGS`,
  `BLUETOOTH_CONNECT` (API 31+, for headset routing), `POST_NOTIFICATIONS` (API 33+, only for the ongoing-call
  notification; without it the call still works, just without the notification), `WAKE_LOCK`,
  `FOREGROUND_SERVICE`, `FOREGROUND_SERVICE_MICROPHONE`, `FOREGROUND_SERVICE_CAMERA`. Declare `<service
  android:name=".CallService" android:foregroundServiceType="microphone|camera" android:exported="false"/>`.
- `android:configChanges="orientation|screenSize|screenLayout|keyboardHidden|keyboard|density|uiMode"` on the
  activity, so rotation and keyboard do not recreate it. The Rust state lives in the process, and recreation would
  tear down the event loop.

### 1.4 Entry point

```rust
#[no_mangle]
fn android_main(app: AndroidApp) {
    // logcat: route eprintln!/log to android_logger (tag "soworker")
    std::env::set_var("XDG_CONFIG_HOME", app.internal_data_path().unwrap());
    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as _) }.unwrap();
    let ctx = unsafe { jni::objects::JObject::from_raw(app.activity_as_ptr() as _) };
    livekit::webrtc::android::initialize_android_context(&vm, &ctx);
    soworker::gui::run_android(app);   // EventLoop::builder().with_android_app(app)
}
```

`gui::run` is split into `run` (desktop, parses args) and a shared `run_with(event_loop, options)`.

**Phase 1 acceptance:** the APK installs and launches. The login screen (or the map, after section 3.1's adb session
push) draws in the phone layout. `adb logcat -s soworker` shows the gaia socket connecting.

## 2. Window lifecycle, input, layout

### 2.1 Suspend / resume

Android destroys the native window whenever the app leaves the screen (home, lock, app switch) and gives a new one on
return. `App::resumed` currently returns early if a window exists, and nothing handles `suspended`.

- `suspended`: drop `self.surface` (softbuffer must not touch the dead `ANativeWindow`) and call `st.set_visible(false)`
  (section 7).
- `resumed`: if a window exists, recreate only the softbuffer context and surface, re-read the scale factor, then
  `st.set_visible(true)`, `dirty = true`.
- Throttle the event loop while there is no surface: no redraw requests, `ControlFlow::Wait`. Backend threads keep
  running.
- Process death: Android may kill the app in the background when no call is running. Nothing to save beyond
  `session.json` / `settings.json`, which are already on disk.

### 2.2 Touch

Map `WindowEvent::Touch` onto the existing mouse paths in `gui/mod.rs`:

- First finger `Started`: set `cursor`, then run the current `MouseInput Pressed` branch.
- `Moved`: past 8 logical px it becomes a drag. Scroll the people list, chat or settings under it (reuse the
  `MouseWheel` handlers with a pixel delta), or drag the split line or the meeting dock (existing drag code). A
  one-finger drag on the map does nothing: the camera is always centred on us (`render.rs` `to_world` /
  `draw_world`), so there is no pan to do. A tap still walks.
- `Ended`: run the `MouseInput Released` branch. A tap with no drag clicks: walk on the map, buttons elsewhere.
- A second finger on the map starts a pinch (section 2.6). The tap and drag of the first finger are cancelled, so
  lifting the fingers never walks. Ignore third and further fingers.
- No hover on touch: `sidebar::update_hover` and the hover card do not apply (the phone layout never collapses the
  sidebar anyway).
- Touch targets: header and footer buttons are 36 to 56 px, fine. Check that chat rows and the meeting dock's controls
  are at least 40 logical px on phones and grow them in the mobile layout where they are not.

### 2.3 Soft keyboard and text

- When the chat input gets focus (`st.chat.focus = true`), call `window.set_ime_allowed(true)`. Call it with `false`
  on blur, send or back.
- Text arrives through GameActivity's `TextInputState`. Each frame while the keyboard is up, read
  `app.text_input_state()` and diff it against the chat input line: replace the line with the state's text and move
  the caret. On Enter (`KeyboardInput` Enter, or the IME action), send the message and clear both. Keep the state in
  sync after we change the text ourselves (`set_text_input_state`).
- Fallback if that turns out unreliable: an invisible Kotlin `EditText` placed over the chat input rect (Rust passes
  the rect), forwarding `onTextChanged` and `onEditorAction` through JNI. Decide after testing with Gboard on the
  device.
- The keyboard shrinks the visible area: read the IME inset (section 2.4) and lift the chat panel's input above it.

### 2.4 Insets

`targetSdk 36` forces edge-to-edge: the status bar, navigation bar and display cutout sit over our pixels. Kotlin
listens with `ViewCompat.setOnApplyWindowInsetsListener` (`systemBars | displayCutout | ime`) and calls
`Native.onInsets`. Rust keeps them in `State.insets` (logical px), and `panels::frame` offsets the mobile header by the
top inset and the footer by the bottom inset (or the IME inset when larger). The map and the dock use `f.content` as
now.

### 2.5 Phone layout

The window is far narrower than 500 logical px on phones (around 360 to 412), so `is_mobile` is true. Landscape on
small phones (around 800 wide) gets the desktop layout. Probably acceptable. Force the mobile layout on Android when
the shorter side is under 500 so landscape keeps the phone UI, if it looks wrong on the device.

### 2.6 Pinch to zoom on the map

Required. The map zoom is `State::scale` (0.08 to 1.5, saved as `guiZoom`), changed today in 10% steps by
`State::zoom(up)` from the mouse wheel.

- Track active touches by id in `App` (`HashMap<u64, (f32, f32)>`). When two are down and the first one started inside
  `f.main` (not over the dock, a panel or the desk button), enter pinch. Remember `d0` (the distance between the
  fingers) and `s0 = st.scale`.
- On each `Moved`: `st.set_zoom(s0 * d / d0)`, a new `State` method that clamps to the same 0.08 to 1.5 range, sets
  `dirty` and does **not** save. Saving on every frame would write settings.json 60 times a second.
- On `Ended` or `Cancelled` of either finger: leave pinch and save `guiZoom` once. `zoom(up)` becomes `set_zoom`
  followed by the save, so the wheel and pinch share one path.
- Anchor: the camera is centred on us, so the zoom is around our avatar, not between the fingers. That is consistent
  with the wheel on desktop. Zooming around the fingers would need a camera offset, which the map does not have. Not
  worth adding for this.
- Performance: every scale change re-rasterizes the map (`MapCache` is invalidated by the scene changing). Measure
  `SOWORKER_PERF` draw times on the device while pinching a big office. If frames go over about 30 ms, scale the
  cached map pixmap during the gesture (one `draw_pixmap` with a scale transform), then rasterize sharp again at the
  final scale on finger up.
- Unit-test the pure part (`set_zoom` clamping, the pinch maths in a small `fn pinch_scale(s0, d0, d) -> f32`).

**Acceptance:** on the device, pinch out and in on the map: smooth zoom, bounded at both ends, no walk on release, and
the zoom is kept after an app restart. `adb shell input` cannot do two-finger gestures, so test by hand, or with
`adb shell sendevent` / a small `uiautomator` script if automated checks are needed.

## 3. Sign-in

The desktop flow (`login.rs`) drives Chrome over the DevTools protocol, which is impossible on Android. Android Chrome
exposes DevTools only to `adb`, not to other apps.

### 3.1 Development: copy a session

All development and testing, including after 3.2 is built, uses this copied session, not the QR flow. The QR flow
needs the user in front of both screens. The desktop app is already signed in on this machine, so copy its session:

```
adb push ~/.config/soworker/session.json /data/local/tmp/
adb shell run-as <applicationId> sh -c 'mkdir -p files/soworker && cp /data/local/tmp/session.json files/soworker/ && chmod 600 files/soworker/session.json'
adb shell rm /data/local/tmp/session.json
```

(`run-as` works on debug builds. The config dir is `XDG_CONFIG_HOME` = `files/`.) This unblocks everything else.
Logout only deletes the local session file (`Backend::finish_logout`), so logging out on the phone does not sign out
the desktop. Copy the file again afterwards.

The phone and the desktop then share one account, and gaia lets an account connect only once. The second connection
replaces the first (`net.rs`, `REPLACED`), and reconnecting would kick the other one in turn. So quit the desktop app
while the phone is connected. For call tests that need a second participant, run the desktop client with a different
account (`--session <other file>`). Ask the user once for a second account if there is none.

### 3.2 Users: in-app WebView capture

Same idea as `login.rs`: let SoWork's own page sign in, then take the Firebase refresh token from its IndexedDB.

- Kotlin `LoginActivity` with a `WebView` (JavaScript and DOM storage on) on `https://app.sowork.com/login`.
- When the page reaches `/s/<group>/<alias>`, run `evaluateJavascript`. The script reads the
  `firebaseLocalStorageDb` / `firebaseLocalStorage` entry (same lookup as `login.rs`'s CDP script), returns
  `{apiKey, refreshToken}` plus group and alias from the URL, and passes them to `Native.onLoginResult`.
  Rust builds the `Auth` and saves it exactly as the desktop path does. Then clear the WebView's cookies and storage
  (`CookieManager.removeAllCookies`, `WebStorage.deleteAllData`). That matches the desktop's throwaway Chrome profile.
- **Known blocker:** Google refuses OAuth inside embedded WebViews (`disallowed_useragent`). Email and password, and
  non-Google SSO providers, work in a WebView. Google accounts (likely most SoWork users) do not.
- For Google accounts: **pair from the desktop** (agreed with the user). Add "Sign in a phone" to the desktop profile panel. It shows a QR
  code holding the refresh token and API key, encrypted with a one-time key. The phone scans it with the camera from
  section 5.4 (decode with the `rqrr` crate) and saves the session.
  - The QR carries a long-lived credential, so it shows only after a click, disappears after 60 s, and says what it is.
  - Do not use Custom Tabs: their storage is unreadable to us.
  - Testing: unit-test the payload (encode on the desktop side, decrypt and parse on the phone side, expiry) and the QR
    decode against a generated image without the user. The single end-to-end scan with the camera is done **once,
    with the user**, at the end of step 9: ask them when it is ready, do not block other work on it.
- Do not spoof a Chrome user agent to get Google sign-in through the WebView. It is fragile and against Google's policy.

## 4. Back button

Goal: back closes the innermost thing open. When nothing is open, the app goes to the background **without finishing
the activity**, so a running call and the socket survive.

### 4.1 Delivery

- With `targetSdk 36`, Android 16 uses predictive back. `KEYCODE_BACK` is no longer guaranteed to reach the native
  key handler, and `onBackPressed` is not called. Register an `OnBackPressedCallback` (androidx `OnBackPressedDispatcher`,
  which covers API 26 to 36) that is **always enabled** and calls `Native.onBack()`.
- `Native.onBack` posts `UserEvent::Back` through the `EventLoopProxy`, which is kept in a `OnceLock` for JNI.
- winit also maps `KEYCODE_BACK` to `NamedKey::BrowserBack` (older Android versions, or a hardware back key). Route
  that key to the same handler. Kotlin's callback consumes the event first, so the two never both fire.

### 4.2 What back does (`State::back(&mut self) -> bool`, pure logic, unit tested)

Close in this order, first match wins, return `true`:

1. Logout or leave-meeting confirmation open: cancel it (`logout_choice(false)` / `leave_choice(false, now)`).
2. A settings dropdown open (`profile.open`): close it (`Act::Panel`).
3. Settings open: close them (`Act::Close`).
4. Soft keyboard up / chat input focused: drop focus and hide the keyboard.
5. A people-list menu open (`sidebar.open`): close it.
6. A tile fullscreen (`dock.tile_full`) or the dock maximized (`dock.full` on desktop; on mobile the dock is always
   full): leave tile fullscreen.
7. Mobile view is People or Chat: switch to Main (Office).
8. Otherwise return `false`. Rust calls `Native.moveTaskToBack()` (`activity.moveTaskToBack(true)`). The process,
   the call and the socket keep running. Section 7 takes over for video.

Do not add "press back twice to quit". Leaving the app means log out, or swipe it away from recents. A swipe-away
during a call ends the call: `CallService.onTaskRemoved` sends `MediaCmd::Leave` and stops the service.

**Acceptance:** with each of 1 to 7 open, `adb shell input keyevent KEYCODE_BACK` closes exactly that one. With
nothing open, the app goes to the background and `dumpsys activity activities` still lists the activity (not
finished). Repeat with gesture navigation (swipe from the edge) on the device.

## 5. Calls on Android

### 5.1 Audio

- `PlatformAudio` works after `initialize_android_context`. Remove the desktop-only device paths in `media.rs`
  (`pactl_bluetooth_sources`, `mic_is_bluetooth`, `apply_devices`' PulseAudio names) under
  `cfg(not(target_os = "android"))`.
- **Routing.** WebRTC puts the phone in `MODE_IN_COMMUNICATION`, which plays through the **earpiece** by default. Expected
  behavior for a meeting app: speakerphone, or a wired or Bluetooth headset when one is connected. Kotlin
  `setAudioRoute`:
  - API 31+: `AudioManager.setCommunicationDevice()`, choosing BT SCO or BLE headset > wired headset > built-in speaker.
  - Older: `setSpeakerphoneOn` / `startBluetoothSco`.
  - Listen to `AudioDeviceCallback` and re-route on plug or unplug.
  - In settings, replace the desktop speaker and microphone lists with one "Audio output" list: Speaker, Earpiece,
    Headset, Bluetooth. Keep the setting key `audioOutput`.
- Request `RECORD_AUDIO` at the first call join (`requestPermissions`). If denied, join listen-only and say so.

### 5.2 Mic mute

Unchanged: `MediaCmd::Mic(bool)` from `toggle_mic`. Section 8 adds play/pause on top.

### 5.3 Noise cancellation

DeepFilterNet in `mic.rs` captures with `parec` or WASAPI. On Android:

- Phase 1: noise cancellation off, setting hidden. The platform ADM already uses the hardware AEC and NS. The desktop
  default (`noiseCancellation` true) must not apply on Android: `noise_cancel_setting()` returns false there.
- Later: `devices::Capture` on cpal/AAudio (`InputPreset::VoiceCommunication`), feeding the existing `mic.rs` chain.
  Measure CPU on the device first: DeepFilterNet 3 on tract costs about 1 core at 48 kHz on older phones.

### 5.4 Camera

V4L2 and Media Foundation are not available. Implement `camera_thread` for Android with the NDK Camera2 API
(`libcamera2ndk` + `AImageReader`, `AIMAGE_FORMAT_YUV_420_888`, 640x480 at 30 fps, front camera by default). Copy
the Y/U/V planes (row and pixel strides) into the existing `I420Buffer` path. Use `ndk-sys` bindings for
`ACameraManager` / `ACameraDevice` / `ACaptureSession` / `AImageReader`. Or, if those are missing in the locked
`ndk-sys`, write about 30 lines of `extern "C"` declarations.

- Request `CAMERA` at the first camera-on.
- Rotate frames to match the sensor orientation (`ACAMERA_SENSOR_ORIENTATION` + display rotation) before publishing.
  Otherwise others see us sideways.
- The camera list in settings becomes Front and Back.
- Screen sharing from Android: out of scope (needs MediaProjection). Hide the share button on Android.

### 5.5 Recording and transcription

Hide on Android (no ffmpeg or whisper). Compile `record`/`transcribe` out and hide their buttons.

## 6. Calls with the screen locked

Android stops a backgrounded app's mic and network unless it has a foreground service.

- **On join** (`MediaCmd::Join`, app in front, which Android 12+ requires for starting a microphone FGS): Rust calls
  `Native.startCall(meetingName)`. `CallService` then:
  - Calls `startForeground(id, notification, FOREGROUND_SERVICE_TYPE_MICROPHONE | (camera on ? CAMERA : 0))`.
  - Builds the notification in `MediaStyle` with the `MediaSession` token. It shows the meeting name, a
    **Mute/Unmute** action and a **Leave** action, on the lock screen too (`VISIBILITY_PUBLIC`).
  - Takes a `PARTIAL_WAKE_LOCK` and a `WifiLock` (`WIFI_MODE_FULL_LOW_LATENCY`, API 29+; `FULL_HIGH_PERF` before), so
    Doze and Wi-Fi power save do not starve the call.
- **On leave** (`MediaEvent::Left`, or `Leave`): `Native.stopCall()` releases the locks, stops the service and removes
  the notification.
- The foreground service also keeps the gaia socket and chat socket alive during the call.
- **Outside calls**, no foreground service: Android may freeze the process in the background, the socket drops and
  presence goes offline. On resume, the existing reconnect logic rejoins, and availability is reaffirmed on join
  (`net.rs`). Decided: no permanent background notification, so no foreground service outside calls. The ongoing-call
  notification is the only one, and Android requires it for the call to keep running locked.
- With the screen locked, always switch to low-data mode (section 7).

**Acceptance:** join a meeting, lock the phone (`adb shell input keyevent KEYCODE_SLEEP`), wait 10 minutes. A second
participant on the desktop client still hears us and we hear them. `dumpsys activity services <pkg>` shows
`CallService` as foreground with type `microphone`. `dumpsys power | grep -i wake_lock` shows our partial lock.

## 7. Low-data mode (no camera sent, no camera received)

### 7.1 Behavior

Low-data mode is a feature of the **desktop app too**, not only Android. The setting, the State logic and the media
side are shared. Only "forced while not visible" is Android-specific. Build it on the desktop first (section 11). That
is quicker to test, and it lands before any Android work.

- **User setting:** a "Low data" toggle in the meeting dock controls (icon `wifi-off` or `activity`) and in settings,
  saved as `lowDataMode` in settings.json.
- **Forced** while the app is not visible: screen locked, or app in the background. Both arrive as winit `suspended`.
  Kotlin also reports the lock (`ACTION_SCREEN_OFF` / `ACTION_USER_PRESENT`, `KeyguardManager.isKeyguardLocked`) for
  the message. Nothing is on screen while suspended, so video is pointless whether or not the phone is locked.
- Effective mode: `low = user_low || !visible`.
- When low:
  - Our camera is off and unpublished. The capture thread stops, so no frames leave the phone.
  - Remote **camera** tracks are **unsubscribed** (`RemoteTrackPublication::set_subscribed(false)`): the SFU stops
    forwarding them, so nothing is downloaded. Hiding the tiles alone is not enough. The current `wants_video` filter
    only skips decoding, and the bytes still arrive.
  - Remote **screen shares**: unsubscribed while not visible. While visible in user-chosen low mode, keep receiving
    them (decided with the user: people usually join a meeting for the shared screen).
  - Audio is untouched.
- **Leaving low mode** (unlock, or toggle off): resubscribe remote video. Do **not** turn our own camera back on by
  itself. A camera that switches on as the phone unlocks is a privacy surprise. Show "Camera stayed off: tap to turn it
  on" instead.
- Messages, through `State::say`, like the other state changes: "Low data mode: video paused" / "Video back on" /
  "Screen locked: video paused".
- While low, the camera button is disabled and says why when pressed.

### 7.2 Media side (`media.rs`)

- Add `MediaCmd::VideoPolicy { send_camera: bool, recv_camera: bool, recv_screen: bool }`, sent by `State` whenever
  the effective mode changes, and once on join.
- Join with `opts.auto_subscribe = false`. Subscribe explicitly, using `fn wanted(source, policy) -> bool` (audio
  always true):
  - on join, for every existing publication;
  - on `RoomEvent::TrackPublished`;
  - on every policy change, for all remote publications (subscribe or unsubscribe as needed).
  This avoids subscribe-then-unsubscribe churn. Check that SoWork's own clients still see us subscribed to their audio
  (they cannot tell subscriptions apart; this is an SFU-side setting).
- `send_camera == false`: `set_camera(s, false, ...)` and remember that the policy turned it off. Rust State then
  knows not to show "camera on".
- Emit `VideoGone` for every tile whose track was unsubscribed, so no frozen last frame stays on screen.
- Desktop GUI: the same dock toggle and settings row as on Android. TUI: a key in the call view (next to the mic and
  camera keys) and a line in its status bar. All share `lowDataMode`. The forced-on-suspend part is Android only.
  Optional on the desktop: also force it while the window is minimized (winit `Occluded(true)`). Off by default.

### 7.3 Pure logic and tests

`State::set_visible`, `State::low_data()`, and the `MediaCmd`s they queue are pure State logic. Unit-test them like
`toggle_available`: invisible makes video off, visible restores the user's choice, the camera never comes back by
itself, and each transition shows its message.

**Acceptance on the desktop:** two desktop clients in one call, both cameras on. Toggle low data on one. Its camera
tile disappears on the other side, and its own view shows no remote camera tiles. The LiveKit inbound-rtp video bytes
stop growing. Toggling it off brings remote video back, while its own camera stays off.

**Acceptance on the device:** in a call with a desktop participant whose camera is on, with the phone unlocked and
normal mode, compare mobile data used before and after locking for 2 minutes (`adb shell dumpsys netstats detail |
grep -A3 <uid>`, or the LiveKit `get_stats()` inbound-rtp bytes, logged each 10 s). Locked must show only audio rates
(about 30 to 60 kbit/s each way). Unlocked must show video rates again within a few seconds. The desktop participant
sees our camera tile go away on lock and not come back on unlock.

## 8. Play/pause mutes and unmutes

- `CallService` owns a `MediaSession` (androidx `MediaSessionCompat`), active only during a call. Set its
  `PlaybackState` to `STATE_PLAYING` while unmuted and `STATE_PAUSED` while muted, with actions
  `ACTION_PLAY | ACTION_PAUSE | ACTION_PLAY_PAUSE`. Android sends media buttons to the active session that played
  last, so the session must exist and be active.
- `MediaSession.Callback`:
  - `onPlay` unmutes, `onPause` mutes. `onMediaButtonEvent` with `KEYCODE_HEADSETHOOK`, `KEYCODE_MEDIA_PLAY_PAUSE`
    or `KEYCODE_MEDIA_PLAY`/`PAUSE` toggles or sets.
  - Each one calls `Native.onMediaButton(kind)`, which posts `UserEvent::Talk(TalkAct::…)`. That reuses the existing
    push-to-talk path (`State::talk`), so the UI, the mute sound and the dock's mic button stay consistent.
- When the mic changes from the UI (`MediaEvent::MicState`), Rust calls `Native.setMicState(on)`. Kotlin updates the
  `PlaybackState` and the notification's Mute/Unmute action, so the next press does the right thing and the lock-screen
  notification shows the truth.
- While the app is in front, winit also delivers `NamedKey::MediaPlayPause` as a key. MediaSession is the only source
  of truth: ignore that key in Rust on Android to avoid a double toggle.
- Known limit: a Bluetooth headset in call (HFP/SCO) mode may send its button as an HFP "answer/hang up" command, not
  as a media key. Telecom handles that, and we do not see it without a self-managed `ConnectionService`. Test with the
  device's headset. If the button does nothing in a call, `ConnectionService` is a follow-up; ask the user first, it is
  a bigger piece.

**Acceptance:** `adb shell cmd media_session dispatch play-pause` (or `adb shell input keyevent
KEYCODE_MEDIA_PLAY_PAUSE`), unlocked and locked, toggles our mic. The desktop participant sees the mute icon change.
The notification's action label flips. `adb shell dumpsys media_session` shows our session active and
playing/paused accordingly. Then try the real buttons of a wired and a Bluetooth headset.

## 9. Smaller Android replacements

| Desktop piece | Android version |
|---|---|
| `notify.rs` (`notify-send`) | Nothing: decided, no notifications in the background. `notify` is a no-op on Android. Messages and waves that arrive in the background show in the app (unread badges, wave rows) when it is opened again. The only notification is the ongoing-call one (6). |
| `sound.rs` (`pw-play`) | cpal output stream playing the embedded WAVs, or `SoundPool` through JNI. Use `USAGE_NOTIFICATION` so they do not duck the call. |
| `photo.rs` file picker | Android Photo Picker (`ActivityResultContracts.PickVisualMedia`) through Kotlin. Returns the bytes to Rust, then the existing `prepare` + `upload`. |
| `devices.rs` lists | Android lists (5.1, 5.4). |
| `talk.rs`, `overlay.rs`, `desktop.rs`, `mode.rs`, terminal button in settings | compiled out and hidden. |
| `--exit-after`, `--logout-after`, `SOWORKER_PERF` | Read from intent extras (`am start -e soworker_args "..."`) so the same test switches work over adb. |

## 10. adb toolbox for the implementer

```
make android && adb install -r android/app/build/outputs/apk/debug/app-debug.apk
adb shell am start -n <pkg>/.MainActivity
adb logcat --pid=$(adb shell pidof -s <pkg>) -v color          # our stderr goes here via android_logger
adb shell input keyevent KEYCODE_BACK                          # back
adb shell input keyevent KEYCODE_SLEEP / KEYCODE_WAKEUP        # lock / wake (then swipe or PIN for USER_PRESENT)
adb shell input keyevent KEYCODE_MEDIA_PLAY_PAUSE              # play/pause key
adb shell cmd media_session dispatch play-pause                # media session dispatch
adb shell dumpsys media_session | grep -A20 <pkg>
adb shell dumpsys activity services <pkg>                      # foreground service and its type
adb shell dumpsys power | grep -i <pkg>                        # wake locks
adb shell dumpsys netstats detail | grep -A4 "uid=$(adb shell pm list packages -U <pkg> | sed 's/.*uid://')"
adb shell input tap X Y / input swipe X1 Y1 X2 Y2 300          # drive the UI
adb exec-out screencap -p > shot.png                           # check layouts (insets, keyboard)
adb shell cmd uimode night yes / wm size 1080x1920              # odd sizes
adb shell am kill <pkg> (in background) then relaunch          # process-death recovery
```

Run the live call tests against a desktop soworker in the same office. Per the repo's testing rule, a session or call
fix is only reported as done after a live run.

## 11. Order of work

0. Low-data mode on the desktop app (7: setting, State logic, media side, dock toggle, TUI key). Independent of
   Android, and testable now with two desktop clients.
1. Build skeleton (1.x), adb session push (3.1), lifecycle (2.1), touch (2.2), insets (2.4), pinch zoom (2.6).
   Result: browse the office, walk, zoom, read the people list, on the phone.
2. Back button (4).
3. Soft keyboard and chat (2.3).
4. Calls: audio, routing, permissions (5.1, 5.2, 5.3 phase 1).
5. Foreground service, lock screen, wake locks (6).
6. Play/pause through MediaSession, notification actions (8).
7. Low-data mode on Android (7): forced while suspended, lock messages. The shared part is done in step 0.
8. Camera (5.4).
9. Sign-in for users: WebView and QR pairing (3.2; end-to-end scan with the user), sounds, photo picker (9). Release signing and a `make android-release`
   that puts the signed APK next to the desktop builds.
10. Polish: landscape, Bluetooth HFP button, noise cancellation on cpal.

## 12. Decisions and remaining questions

Decided (2026-11-16):
- Google sign-in on the phone: QR pairing from the desktop (3.2).
- No notifications in the background, and no foreground service outside calls (6, 9).
- Low-data mode also in the desktop app (7).
- Sideloaded APKs only (1.3).
- In user-chosen low-data mode while the app is visible, screen shares keep coming through (7.1).
- Development signs in by copying the desktop session over adb (3.1). The QR scan is tested once with the user (3.2).

Nothing open.

## 13. Progress

Checked on the phone (Samsung SM-S931B, Android 17), signed in with the desktop session pushed over adb:
- Build, install, launch; the map, the people list and the chat draw in the phone layout inside the system bars,
  also sideways (a phone held sideways keeps the phone layout).
- Taps, the tabs, scrolling the settings, back (closes the keyboard, then the tab, then sends the app to the
  background without finishing it); the surface comes back after the app returns.
- Soft keyboard: typing, delete, back; with the keyboard up the tabs (and, sideways, the header) step aside so the
  message box stays visible. android-activity 0.6.1 aborted on an empty text field: patched locally
  (`android/patches/android-activity`, `[patch.crates-io]` in Cargo.toml).
- Settings on the phone: "Audio output" (Automatic, Speaker, Phone earpiece, wired, Bluetooth as connected), Camera
  (front by default, or back), low-data mode; Play sound plays through MediaPlayer.
- Sign-in screen: "Sign in with SoWork" opens SoWork's page in the WebView, back returns to the sign-in screen.
  "Scan a code from your computer" asks for the camera, starts the back camera (frames arrive) and Cancel releases it.
- Hiding the app forces low-data mode and showing it restores the choice (policy commands in the log).

Done but not yet checked live:
- Calls (audio route, foreground service, media buttons, low-data in a call, the camera picture's orientation): they
  need a second account in the call (section 3.1).
- The end-to-end QR scan (desktop profile, "Sign in a phone", then the phone's scan): to do once with the user. The
  code drawn by the desktop is decoded in a unit test (`gui::panels::tests::the_drawn_phone_code_scans`).

Deviation: the QR code carries the session base64-encoded, not encrypted. A key that decrypts it would have to be in
the same picture, so encryption adds nothing; the code only shows on a click, says what it is and disappears after
60 seconds (`pairing.rs`).

