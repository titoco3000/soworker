# Native LiveKit media: feasibility on this machine

Spike date 2026-10-06, scratch dirs `/tmp/claude-1000/lkspike` (official SDK) and `/tmp/claude-1000/lkspike2`
(pure-Rust audio path). Fedora 44, gcc 16.1.1, cargo 1.96, no sudo. No server was contacted.
Every claim is tagged **Measured** (done here), **Documented** (crate source/README in the cargo registry,
version given) or **Estimated** (memory/judgement, treat as +-50%).

## 0. Summary

- The official SDK (`livekit` 0.9.3) has everything we need API-wise (audio, camera, screen share via PipeWire,
  data, metadata, active speakers, single-PC mode = what SoWork's JS client uses). It cannot be built on this
  machine yet: **`clang++` is missing** (hard requirement, see 1.3) and no `g++` either. That is the blocker, not
  a design problem. Install `clang` and retry; nothing else I could check was a blocker.
- It is heavy by nature: a prebuilt **156 MB download / 692 MB extracted / 85 MB static `libwebrtc.a`**, and an
  estimated 25-40 MB binary and 120-250 MB RSS in a call with video.
- A pure-Rust audio-only path (str0m + livekit-protocol + libopus) is **measured at 5.2 MB stripped binary and
  6.5 MB idle RSS** for the dependency set alone; estimated 30-50 MB RSS in a 2-person audio call. It costs us a
  bespoke LiveKit signalling client, TURN, and no video decode for VP8/VP9.
- Recommendation: stage 1 = pure-Rust audio (str0m); keep the official SDK as the fallback and as the route to
  video/screen share, behind a Cargo feature so the lightweight build stays lightweight. See section 5.

## 1. Official SDK: build feasibility

### 1.1 Versions and compatibility

| Item | Value | Tag |
|---|---|---|
| Crate | `livekit` **0.9.3** (repo livekit/rust-sdks), license **Apache-2.0** | Documented (Cargo.toml) |
| Sub-crates | `libwebrtc` 0.3.50, `webrtc-sys` 0.3.47, `webrtc-sys-build` 0.3.19, `livekit-protocol` 0.8.0, `livekit-signaling` 0.1.4 | Measured (cargo fetch) |
| Signal protocol | `PROTOCOL_VERSION = 17` in `livekit-signaling/src/lib.rs:62`, identical to the official JS client's 17 | Documented |
| Server 1.13.7 | The SDK sends the same wrapped `join_request` (gzip `WrappedJoinRequest`) and defaults to **single peer connection** (`RoomOptions::single_peer_connection = true`), same shape as SoWork's JS client. Not tested against a server. | Documented / unverified live |
| Features | default = `native`; TLS choice: `rustls-tls-native-roots` (used in spike), `rustls-tls-webpki-roots`, `native-tls`; `glib-main-loop` (needed for Wayland desktop capture, see 2.7) | Documented |
| Dependency count | about 230 crates in the tree | Measured (`cargo tree`) |

### 1.2 Result of `cargo fetch` and `cargo build --release`

- `cargo fetch`: **succeeds**, a few seconds (crates.io only).
- `cargo build --release`: **fails after 10 s** in `link-cplusplus` ("failed to find tool c++"). Measured.
  `webrtc-sys` itself never got to run, so the following was read from its `build.rs`, not observed building.
- `webrtc-sys/build.rs` (`configure_hermetic_libcxx`) panics unless **`clang++`** exists (or `CXX` points to a
  clang): libwebrtc.a is built with Chromium's custom libc++ and GCC silently miscompiles `unique_ptr` calls
  against it. So gcc-c++ alone is not enough. **Documented**, `webrtc-sys-0.3.47/build.rs` lines ~509-545.
- Installed here: `gcc`, `cmake`, `clang-libs`, `clang-resource-filesystem`. Missing: `clang` (the driver, gives
  `clang++`), `gcc-c++`.

### 1.3 libwebrtc prebuilt download (I performed the same download by hand)

| Item | Value | Tag |
|---|---|---|
| URL | `https://github.com/livekit/rust-sdks/releases/download/webrtc-89d790b/webrtc-linux-x64-release.zip` (GitHub redirects to objects/release-assets.githubusercontent.com) | Measured |
| Size | zip **156,139,058 bytes**; extracted **692 MB**; `lib/libwebrtc.a` **85.5 MB**; rest is headers | Measured |
| sha256 (observed, for pinning) | `b167adad5291cea0e4d66a0454d9d52d2ad714e6b0ed70f4410317d3ebde70c5` | Measured |
| Checksum behaviour | **None.** `webrtc-sys-build` downloads over HTTPS with reqwest and unzips; no hash verification in the source. We should pre-download, verify the hash above ourselves, and point `LK_CUSTOM_WEBRTC` at the extracted dir. | Documented (grep: no sha/checksum) |
| Cache location | `scratch::path("livekit_webrtc")` under the cargo target dir, shared across builds | Documented |
| Build flags of the prebuilt | `rtc_use_pipewire=true`, `rtc_use_x11=true`, `rtc_use_h264=true`, `rtc_use_h265=true`, `rtc_libvpx_build_vp9=true`, `enable_libaom=true`, `use_custom_libcxx=true` | Measured (args.gn) |

### 1.4 Native deps and -dev packages

From `webrtc-sys/build.rs` (Documented; not exercised because the build stops earlier):
- `glib-2.0`, `gobject-2.0`, `gio-2.0` via pkg-config, headers only (installed here: `glib2-devel`).
- X11 family and drm/gbm are **lazy-loaded** via implib stubs for `drm gbm X11 Xfixes Xdamage Xrandr Xcomposite Xext`
  (runtime dlopen, so the .so's, not necessarily -devel, are needed; I could not verify whether headers are
  needed at compile time, install them to be safe).
- `libva` optional: if `libva.pc` is found, VAAPI hardware H.264 encode is compiled in (dlopened at runtime).
- Links `rt dl pthread m` only; **libc++ is inside libwebrtc.a**, no libstdc++ link.

Package state on this machine (Measured with `rpm -q`):

| Package | State |
|---|---|
| glib2-devel, libX11-devel, libXext-devel, mesa-libGL-devel, libdrm-devel, openssl-devel, cmake | installed |
| **clang**, **gcc-c++** | missing, required |
| libXrandr-devel, libXcomposite-devel, libXdamage-devel, libXfixes-devel, libXtst-devel, mesa-libgbm-devel | missing (likely needed for the desktop-capture C++ file) |
| pipewire-devel, alsa-lib-devel, pulseaudio-libs-devel | missing (see 2.x; only needed if we use cpal or pipewire crates ourselves) |

### 1.5 Binary size and RSS

**Not measurable here** (build blocked). Estimates, from experience with similar libwebrtc-linked binaries:
release binary **25-40 MB unstripped-with-no-debug, about 20-30 MB stripped**; idle process RSS right after
`Room`/factory creation about **40-70 MB** (libwebrtc spawns worker/signaling/network threads, ADM, codec factories).
Label: **Estimated**. Also: first clean build compiles ~230 crates plus the C++ glue; expect 5-10 min (Estimated).

My test program (`/tmp/claude-1000/lkspike/src/main.rs`: `RoomOptions::default()`, `NativeAudioSource::new(..AEC/NS/AGC..)`,
`TrackPublishOptions::default()`) is ready; once `clang` is installed run
`CARGO_HOME=/tmp/claude-1000/cargo CARGO_TARGET_DIR=/tmp/claude-1000/lkspike/target CXX=clang++ cargo build --release`
then `strip` and `/usr/bin/time -v`.

## 2. What the official SDK offers

| Capability | API (livekit 0.9.3 / libwebrtc 0.3.50) | Notes | Tag |
|---|---|---|---|
| Connect url+token | `Room::connect(url, token, RoomOptions) -> (Room, UnboundedReceiver<RoomEvent>)` (`room/mod.rs:567`); `room.subscribe()` for extra receivers | Works with a plain LiveKit JWT, which is what gaia gives us | Documented |
| RoomOptions | `auto_subscribe`, `adaptive_stream` (default **false**), `dynacast` (default false), `single_peer_connection` (default **true**), `join_retries`, `rtc_config`, `encryption` | Set `adaptive_stream=true` to match SoWork | Documented |
| Mic publish, option A | `PlatformAudio::new()` (`livekit::platform_audio`): libwebrtc's own Audio Device Module captures mic AND plays remote audio to speakers; `recording_devices()`, `set_recording_device(id)`; `LocalAudioTrack::create_audio_track("mic", audio.rtc_source())` | Linux backend is **PulseAudio or ALSA** (via PipeWire's pulse shim on Fedora). No cpal needed. Simplest path | Documented |
| Mic publish, option B | `NativeAudioSource::new(AudioSourceOptions{echo_cancellation,noise_suppression,auto_gain_control}, 48000, 1, queue_ms)` then `capture_frame(&AudioFrame)`; we feed from cpal | The options struct exists, but with manual push libwebrtc has no render reference, so **AEC cannot work unless we also run APM ourselves** | Documented |
| AEC / NS / AGC | `AudioProcessingModule::new(..)` with `process_stream` (capture) and `process_reverse_stream` (playout reference), `set_stream_delay_ms` (`native/apm.rs`); `PlatformAudio` has `AudioProcessingOptions{echo_cancellation,noise_suppression,auto_gain_control,prefer_hardware_processing}` all default true | Software WebRTC AEC3 is available in both routes; PlatformAudio does the wiring for us (best for headset-less use) | Documented |
| Remote audio playback | PlatformAudio plays all subscribed tracks automatically (mixing done inside libwebrtc). Manual: `NativeAudioStream::new(track, sample_rate, channels, queue)` yields `AudioFrame` (i16); `AudioMixer::add_source/mix` exists for mixing in Rust | Per-speaker volume only with the manual route | Documented |
| Receive video | `NativeVideoStream::new(RtcVideoTrack, queue_frames)` yields `VideoFrame<BoxVideoBuffer>`; buffer is I420 normally; `to_i420()` and `to_argb(format, dst, stride, w, h)` do libyuv conversion into our own buffer | Convert straight into the tiny-skia/softbuffer pixel buffer (ARGB/ABGR variant matching softbuffer's XRGB) at the displayed size; `I420Buffer::scale()` can pre-shrink | Documented |
| Limit decoder resolution | No automatic adaptive-stream observer in the Rust SDK (the flag only goes into the join request). Manual: `RemoteTrackPublication::update_video_dimensions(TrackDimension)`, `set_video_quality(VideoQuality)`, `set_enabled(false)`, `set_subscribed(false)` | We call these from our UI layout (tile size) ourselves. Server then forwards a lower simulcast layer, so decode cost drops. | Documented |
| Camera publish | Capture is NOT in the SDK. `NativeVideoSource::new(VideoResolution, is_screencast)` + `capture_frame(&VideoFrame<I420Buffer>)`; encode (VP8/VP9/H264/AV1/H265, simulcast, `scalability_mode`) is libwebrtc. Capture ourselves with `nokhwa` 0.10.11 (Apache-2.0) or `v4l` 0.14.0 (MIT) producing MJPEG/YUYV, convert to I420 with libyuv helpers in `libwebrtc::native::yuv_helper` | v4l is the thinner dependency (needs only kernel headers/libv4l not required for raw V4L2) | Documented / Estimated for crates |
| Screen share | `libwebrtc::desktop_capturer::{DesktopCapturer, DesktopCapturerOptions(Screen/Window/Generic), CaptureSource, DesktopFrame}` backed by libwebrtc's desktop capturer; prebuilt has `WEBRTC_USE_X11` and `WEBRTC_USE_PIPEWIRE`. Frames are pushed into a `NativeVideoSource(is_screencast=true)` | Wayland: **PipeWire via xdg-desktop-portal** (user-visible picker dialog each time; GNOME/KDE portal must be running) and the SDK needs the `glib-main-loop` feature + `WAYLAND_DISPLAY` set so the portal DBus loop runs (`native/desktop_capturer.rs:119`). X11: direct XShm capture. Not tested here | Documented / unverified live |
| Data channels | `local_participant().publish_data(DataPacket{payload, topic: Option<String>, reliable: bool, destination_identities})`; receive `RoomEvent::DataReceived{payload, topic, kind: Lossy/Reliable, participant}`. Also text/byte streams, RPC | Maps 1:1 to SoWork's `_reliable`/`_lossy` channels | Documented |
| Participant metadata | `local_participant().set_metadata(String)`, `set_attributes(HashMap)`, `set_name`; read via `.metadata()` and `RoomEvent::ParticipantMetadataChanged` / `ParticipantAttributesChanged` | Token has `canUpdateOwnMetadata` | Documented |
| Active speakers | `RoomEvent::ActiveSpeakersChanged{speakers}`; `Participant::is_speaking()`, `audio_level()` | | Documented |
| Codecs / simulcast | `TrackPublishOptions{video_codec: VP8/H264/VP9/AV1/H265, simulcast, simulcast_layers, scalability_mode, dtx, red, degradation_preference, video_encoder (backend)}`; `RoomOptions::dynacast` | Decode of VP8/VP9/H264/AV1 is libwebrtc's (libvpx, libaom, openh264/ffmpeg "Chrome" branding), so remote video works regardless of what the sender picked | Documented |
| Disconnect reasons | `RoomEvent::Disconnected{reason}` with `DisconnectReason` incl. `ClientInitiated`, `DuplicateIdentity`, ...; `room.close()`, `close_with_reason` | | Documented |
| Reconnect | `Reconnecting`/`Reconnected` events, resume handled inside SDK | | Documented |

## 3. Alternatives to the official SDK

### 3.1 Pure-Rust WebRTC stacks

| Option | State | Verdict |
|---|---|---|
| `str0m` **0.24.1**, MIT/Apache-2.0, sans-IO, builds with `aws-lc-rs` default (rustls-style crypto, **no OpenSSL, no C++**) | Measured: compiled fine here in 73 s together with tokio, tokio-tungstenite, livekit-protocol and libopus. Supports data channels, RTP-mode and frame-mode media, simulcast, TWCC/BWE. | Best fit. We own the socket loop (UDP socket + `poll_output` timeout loop), which is also how we keep it small |
| `webrtc` (webrtc-rs) 0.21.0 on crates.io (the old 0.8/0.9 line is the long-stale one) | Async, bigger dependency tree, historically slower release cadence. Not evaluated deeply. | Second choice; no advantage over str0m for a single client |
| Gaps in both | **No TURN client** (str0m README: TURN is "outside scope", you obtain a relayed socket yourself). LiveKit Cloud gives `turns:` on 443 as fallback; on typical home/office networks direct UDP to the SFU works, but locked-down networks need TURN/TCP. | Risk, Estimated frequency: small fraction of users |

### 3.2 What we must write ourselves

Documented in `docs/media-livekit.md` (our capture of the exact JS flow). Work items, Estimated effort for one dev:

| Piece | Effort | Risk |
|---|---|---|
| WebSocket `/rtc/v1?access_token=..&join_request=..` (wrapped, gzip join request; `livekit-protocol` 0.8.0 Apache-2.0 gives all prost types) | 1-2 days | low, we have the capture |
| SDP offer/answer plumbing for single-PC mode (publisher_offer in join request, server answer, later server offers when subscribing, trickle ICE, `SyncState`, `Leave`, ping/pong) | 3-5 days | medium: LiveKit-specific mid/transceiver conventions, "negotiation" via `UpdateSubscription`, `AddTrackRequest` before publishing |
| Subscribe audio: str0m receive, Opus decode, jitter handled by str0m, mix + cpal playback | 2-3 days | low |
| Publish mic: cpal capture, Opus encode, `AddTrackRequest`, send via str0m `Writer::write` | 2-3 days | low |
| AEC/NS: **no pure-Rust AEC3** of quality. Options: `webrtc-audio-processing` crate (C++ dep, brings back a C++ build, but small ~2-3 MB), or require headphones in stage 1 | 1-3 days | medium: echo on speakers is a real UX problem |
| Data channels `_reliable/_lossy` (str0m SCTP) | 1-2 days | low |
| Reconnect/resume, quality, mute, DUPLICATE_IDENTITY handling | 3-5 days | medium |

### 3.3 Video in the pure-Rust path

- **Receive VP8**: needs a decoder crate. `vpx-rs` 0.2.1 (BSD-2-Clause) wraps libvpx (C, needs libvpx-devel or a bundled build). Pure-Rust VP8 decoders exist only as image-oriented (WebP) code, not a streaming RTP decoder. Practical: libvpx binding.
- **Receive VP9**: **no pure-Rust decoder in practice**; libvpx via `vpx-rs` again. SoWork's JS client may publish VP9 or VP8 by browser choice (the capture would tell which; check `docs/media-livekit.md`). If any peer publishes only VP9/AV1, we would need libvpx/libaom regardless. Estimated.
- **Receive H.264**: openh264 crate (Cisco, BSD-2, downloads a binary). Browsers publish H.264 only if configured.
- **Publish camera**: libvpx VP8 encode via same binding + simulcast RTP handling ourselves. Doable but a real project (Estimated 2+ weeks incl. keyframe/PLI, BWE).
- **Screen share**: capture via `pipewire` crate 0.10.1 + portal (`ashpd`) then encode; same encode cost. Estimated 2+ weeks.
- Loss if we stay audio-only: no camera tiles, no screen share, no seeing others' video. Proximity audio, presence, chat/data still work. That is acceptable as a stage 1.

## 4. Memory expectations (whole app, 2-person call, 1 received camera stream)

Baseline for the rest of the app (window, softbuffer, world state, gaia socket) is outside this spike; assume the other thread's number and add the media delta below.

| Scenario | Media-stack RSS delta | Tag |
|---|---|---|
| (a) Official SDK, platform audio + 1 remote 640x480 video (decode + libyuv convert) | **120-250 MB** total process (libwebrtc threads, codec contexts, jitter/NetEq buffers, AEC3, ~10-30 MB frame queues). Without video about 70-120 MB. | Estimated |
| (a') same but camera publishing (VP8 simulcast encode) | +40-100 MB | Estimated |
| (b) Pure-Rust audio-only (str0m + libopus + cpal) | **25-50 MB** total for the media stack; **6.5 MB** is the measured floor of just the linked deps idle | Measured floor (6528 kB max RSS of `/tmp/claude-1000/lkspike2/target/release/rspike`, which constructs an `Rtc`, one Opus encoder and a protobuf message), rest Estimated |

Binary size (b) measured: **5,985,128 bytes unstripped, 5,196,512 bytes stripped** (str0m, aws-lc-rs, rustls via tungstenite
feature, libopus static, livekit-protocol). Official SDK (a): see 1.5, Estimated 20-40 MB.

## 5. Recommended staged plan

Gate everything behind Cargo features: `media-audio` (default) and `media-livekit-sdk` (optional heavy).

| Stage | Scope | Crates | Biggest risk |
|---|---|---|---|
| 0 (now) | Unblock the official SDK build only to validate it against a real token (optional, short): install clang, build the spike, measure real RSS/size and replace Estimated numbers in this doc | `livekit` 0.9.3 (feature `rustls-tls-native-roots`) | libwebrtc 156 MB download without checksum; pin by sha256 above, set `LK_CUSTOM_WEBRTC` |
| 1 | Audio-only, pure Rust: LiveKit signalling client, single-PC subscribe + publish mic, active speakers, metadata, data channels. Headphones assumed, or add APM later | `str0m` 0.24.1, `livekit-protocol` 0.8.0, `tokio-tungstenite` 0.26 (rustls), `opus` 0.4.0, `cpal` 0.18.2 (needs `alsa-lib-devel`; ALSA talks to PipeWire's alsa plugin on Fedora), `prost` 0.14 | Our hand-written signalling vs LiveKit 1.13.7 quirks; no TURN; echo |
| 2 | Echo cancellation/NS | `webrtc-audio-processing` crate (C++ dep) or APM from official SDK | adds C++ toolchain again |
| 3 | Receive video | Either official SDK (`NativeVideoStream`, easiest) or `vpx-rs` 0.2.1 + our RTP depacketization in str0m | VP9/AV1 senders; decode CPU; sizing tiles via `update_video_dimensions` |
| 4 | Camera publish | official SDK `NativeVideoSource` + `v4l` 0.14.0 (or `nokhwa` 0.10.11) | simulcast/BWE tuning if done in pure Rust |
| 5 | Screen share | official SDK `DesktopCapturer` with `glib-main-loop` + portal | Wayland portal picker UX, untested here |

Decision rule: if video matters to the product, stages 3-5 are cheapest on the official SDK, so a hybrid
(pure-Rust audio by default, official SDK as optional feature for video) is a sound plan. If we only ever do audio,
the official SDK is not worth its weight.

### Fedora packages (dnf) to install with sudo

Minimum to build the **official SDK** spike and the full path:

```
sudo dnf install clang gcc-c++ libXrandr-devel libXcomposite-devel libXdamage-devel libXfixes-devel libXtst-devel \
  mesa-libgbm-devel pipewire-devel xdg-desktop-portal alsa-lib-devel pulseaudio-libs-devel libva-devel
```

(`clang` is the one hard blocker. Already present: glib2-devel, libX11-devel, libXext-devel, mesa-libGL-devel,
libdrm-devel, openssl-devel, cmake.) Which of the others are strictly required at build time is unverified;
they are needed at runtime or for the desktop-capture headers. `libva-devel` is optional (hardware H.264 encode).
For the **pure-Rust audio path** only `alsa-lib-devel` (for cpal) is missing; str0m + opus built here without
anything extra (libopus is built from bundled source with the existing gcc/cmake).

## 6. Open items I could not settle here

- Real binary size/RSS of the official SDK (blocked by clang).
- Whether LiveKit Cloud's server ICE mode and candidate types work cleanly with str0m (no live test, forbidden).
- Whether SoWork peers publish VP8, VP9 or H.264 (check the capture; decides the video decode requirement).
- Wayland screen share through libwebrtc's portal path on this session.
