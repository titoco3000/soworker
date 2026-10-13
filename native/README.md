# SoWorker native

Fully native client (Rust, no browser, no Node). One process, one 5 MB binary, software rendering.

Install for your user (puts `soworker` in `~/.cargo/bin`, which is on your PATH if you use cargo; the first start then
registers the launcher entry and icon pointing at that binary):

```
cd native && CXX=clang++ CC=clang cargo install --path . --locked
soworker
```

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
```

`cargo run` (debug) is fine for development: the dev profile optimizes dependencies, so a frame costs
~8 ms instead of ~52 ms (release: ~1.5 ms).

Sign-in: with no saved session the app shows a sign-in screen. The button opens Chrome (app-style window, throwaway
profile) on SoWork's own login page, watches it over the DevTools protocol, takes the Firebase refresh token from the
page once you are signed in (and the office from the address), then closes Chrome and deletes its profile. The token
is saved as `~/.config/soworker/session.json` (folder 700, file 600, outside the repository); `.space.json` beside it
remembers the office so the next sign-in goes straight to it. A session an older version left in `native/` or `app/` is
moved there on first start. Log out forgets the token and returns to the sign-in screen. Use `--session <file>` for another path. (The Node prototype in `app/` still keeps its own `app/.session.json`, git-ignored.)

Sidebar (left): availability and log out buttons, then people in meetings (grouped by meeting zone), online and offline
people; click someone or a meeting for Join (walks there), Wave or Message. The list comes from SoWork's
Firestore (docs/members-roster.md); `--roster` prints it.

Waves and chat: Wave in a person's menu (or a meeting's) waves at them; waves at you appear in the sidebar to Accept (walk
to them) or Dismiss. A right-hand chat panel has the area chat of your current meeting and private chats (Message in a
person's menu); see docs/chat-and-waves.md.

Wave sounds: SoWork's own `sendWave` / `receiveWave` clips (assets/sounds, from app.sowork.com/static/media, volume 0.2 as
in its client), played through `pw-play`, `paplay` or `aplay` (no audio library is linked).

Push to talk: `Super+'` held (configurable) opens the mic; with the window hidden bind `soworker --talk` to a key in your desktop
settings, and a "Press Super+' to talk" notification reminds you when you are muted in a meeting and the window is out of focus
(docs/push-to-talk.md). While your mic is open and the window is out of focus, an always-on-top pill in the top-right
corner shows "Mic on · N listening" (`soworker --overlay-test` to try it).

Push to talk setup on COSMIC (once): COSMIC has no global-shortcut portal, so bind `soworker --talk` to a key yourself.
Press once to open the mic, again to close it (a desktop shortcut cannot report the key release). From a terminal, with
`BIN` set to your built binary:

```
BIN=$PWD/target/release/soworker        # run from native/ after `cargo build --release`
f=~/.config/cosmic/com.system76.CosmicSettings.Shortcuts/v1/custom
cp "$f" "$f.bak"                         # undo with: mv "$f.bak" "$f"
sed -i '$ d' "$f"                        # drop the closing }
cat >> "$f" <<EOF
    (
        modifiers: [
            Super,
        ],
        key: "apostrophe",
    ): Spawn("$BIN --talk"),
}
EOF
```

COSMIC reloads the file by itself (or use Settings > Keyboard > Custom shortcuts to add the same thing by hand). Change
`Super` / `apostrophe` for another key, and set `talkKey` in `~/.config/soworker/settings.json` to the same keys (e.g.
`"Super+'"`) so the in-app hint matches. Test it with `soworker --talk` in a terminal while the app is running.

Meeting dock: ported from omoffice (frontend/src/features/meetings): draggable card snapping to 8 slots or full window,
gallery grid or pinned view with a scrolling sidebar, hover buttons per tile (mute / camera off / pin), controls row
(leave, lock, camera, mic, share screen) and the chat toggle. Dark palette from omoffice. Mute button sends SoWork's `ask-to-mute` request and we honour it when asked; available/away also sets SoWork's presence
status (docs/presence-and-mute.md). Stubs: switching off someone else's camera (SoWork has no such thing), lock, chat (layout only). Avatars: the user's uploaded photo, else SoWork's character portrait
(`app.sowork.com/portrait/v1/meeting!idle/...`), else the first letter; used on the map and in tiles.

App icon: `assets/icon.webp` (window icon on X11, embedded in the binary). Wayland and the launcher take it from a desktop
entry, which the app installs for your user on start (`~/.local/share/applications/soworker.desktop` and the icon, pointing at the
running binary; `src/desktop.rs`). If the icon does not show on the very first run, start it a second time.

Keys: WASD/arrows walk, left click walks there (routes around walls), Shift+click teleports, R respawns,
B toggles available/away, the meeting dock buttons (mic, cam, share screen, leave) once you are in a call, the available/away button in the sidebar, M mic (calls start muted), C camera, wheel zooms.

Modules: `session` (Firebase refresh), `net` (gaia socket thread), `proto` (Socket.IO/msgpack events),
`mapobj` (protobuf map objects), `world` (collision, A*, zones), `app` (state/logic), `render` (tiny-skia),
`window` (winit + softbuffer).

Status: world, walking, click-to-move, zones, presence, speaking rings, HiDPI, calls: join from the gaia
`cluster` token, audio in/out through the SDK's platform audio (echo cancellation included), remote camera and
screen-share tiles, mic toggle. Camera publish (V4L2 /dev/video0, YUYV 640x480 -> VP8) and screen share (libwebrtc desktop capturer; on Wayland the portal
picker dialog appears) are implemented but untested on real hardware. Not yet: chat.
Memory: ~30 MB RSS live (dry run, window open: 11 MB heap, 6 MB window buffers, 13 MB binary pages); call memory not measured yet
(docs/native-media-feasibility.md has estimates). Binary: ~28 MB with calls.
