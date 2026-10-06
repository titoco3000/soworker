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

Sign-in: with no saved session the app shows a sign-in screen. The button opens Chrome (app-style window, throwaway
profile) on SoWork's own login page, watches it over the DevTools protocol, takes the Firebase refresh token from the
page once you are signed in (and the office from the address), then closes Chrome and deletes its profile. The token
is saved as `~/.config/soworker/session.json` (folder 700, file 600, outside the repository); `.space.json` beside it
remembers the office so the next sign-in goes straight to it. A session an older version left in `native/` or `app/` is
moved there on first start. Log out forgets the token and returns to the sign-in screen. Use `--session <file>` for another path. (The Node prototype in `app/` still keeps its own `app/.session.json`, git-ignored.)

Meeting dock: ported from omoffice (frontend/src/features/meetings): draggable card snapping to 8 slots or full window,
gallery grid or pinned view with a scrolling sidebar, hover buttons per tile (mute / camera off / pin), controls row
(leave, lock, camera, mic, share screen) and the chat toggle. Dark palette from omoffice. Mute button sends SoWork's `ask-to-mute` request and we honour it when asked; available/away also sets SoWork's presence
status (docs/presence-and-mute.md). Stubs: switching off someone else's camera (SoWork has no such thing), lock, chat (layout only). Avatars: the user's uploaded photo, else SoWork's character portrait
(`app.sowork.com/portrait/v1/meeting!idle/...`), else the first letter; used on the map and in tiles.

Keys: WASD/arrows walk, left click walks there (routes around walls), Shift+click teleports, R respawns,
B toggles available/away, the meeting dock buttons (mic, cam, share screen, leave) once you are in a call, the available/away button under the top-left text, M mic (calls start muted), C camera, F3 debug panel, wheel zooms.

Modules: `session` (Firebase refresh), `net` (gaia socket thread), `proto` (Socket.IO/msgpack events),
`mapobj` (protobuf map objects), `world` (collision, A*, zones), `app` (state/logic), `render` (tiny-skia),
`window` (winit + softbuffer), `replay` (recorded runs, for tests and offline demo).

Status: world, walking, click-to-move, zones, presence, speaking rings, HiDPI, calls: join from the gaia
`cluster` token, audio in/out through the SDK's platform audio (echo cancellation included), remote camera and
screen-share tiles, mic toggle. Camera publish (V4L2 /dev/video0, YUYV 640x480 -> VP8) and screen share (libwebrtc desktop capturer; on Wayland the portal
picker dialog appears) are implemented but untested on real hardware. Not yet: chat.
Memory: ~33 MB RSS in the offline window with the SDK linked; call memory not measured yet
(docs/native-media-feasibility.md has estimates). Binary: ~28 MB with calls.
