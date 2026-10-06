# SoWorker native

Fully native client (Rust, no browser, no Node). One process, one 5 MB binary, software rendering.

Build (calls use the official LiveKit Rust SDK, which needs `clang`/`gcc-c++` and the libwebrtc prebuilt):

```
# first build downloads libwebrtc (156 MB, sha256 b167adad5291cea0e4d66a0454d9d52d2ad714e6b0ed70f4410317d3ebde70c5)
# from github.com/livekit/rust-sdks (tag webrtc-89d790b); to reuse an extracted copy set LK_CUSTOM_WEBRTC=<dir>
cd native && CXX=clang++ CC=clang cargo build --release
cargo build --release --no-default-features    # tiny build without calls (~5 MB)
```

```
./target/release/soworker --dry-run          # connect and watch, sends nothing
./target/release/soworker                    # live: your avatar walks for real
./target/release/soworker --demo <run dir>   # offline window fed from a recorder run
./target/release/soworker --replay <run dir> --shot out.png [--click X,Y] [--script "r:1500,d:600"]
```

`cargo run` (debug) is fine for development: the dev profile optimizes dependencies, so a frame costs
~8 ms instead of ~52 ms (release: ~1.5 ms, measured with `--replay <run> --shot x.png --bench 60`).

Session: reads `native/.session.json`, `.session.json`, `app/.session.json` (same file the Node prototype
uses, created by `node app/import-token.mjs <run>`) or `--session <file>`.

Keys: WASD/arrows walk, left click walks there (routes around walls), Shift+click teleports, R respawns,
B toggles available/away, buttons bottom-right (mic, cam, share, available/away; they enable once you are in a call), M mic (calls start muted), C camera, F3 debug panel, wheel zooms.

Modules: `session` (Firebase refresh), `net` (gaia socket thread), `proto` (Socket.IO/msgpack events),
`mapobj` (protobuf map objects), `world` (collision, A*, zones), `app` (state/logic), `render` (tiny-skia),
`window` (winit + softbuffer), `replay` (recorded runs, for tests and offline demo).

Status: world, walking, click-to-move, zones, presence, speaking rings, HiDPI, calls: join from the gaia
`cluster` token, audio in/out through the SDK's platform audio (echo cancellation included), remote camera and
screen-share tiles, mic toggle. Camera publish (V4L2 /dev/video0, YUYV 640x480 -> VP8) and screen share (libwebrtc desktop capturer; on Wayland the portal
picker dialog appears) are implemented but untested on real hardware. Not yet: chat.
Memory: ~33 MB RSS in the offline window with the SDK linked; call memory not measured yet
(docs/native-media-feasibility.md has estimates). Binary: ~28 MB with calls.
