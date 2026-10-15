# SoWorker

Unofficial native client for [SoWork](https://www.sowork.com): walk around the office, see who is where, wave, chat and
join calls without a browser tab. Rust, one process, runs in the terminal: the office is drawn with text, and the only
graphics are the camera and screen-share video. Chrome is only used for the sign-in. A software-rendered window client
is built in too (`soworker --gui`).

## Build and run

Needs a Rust toolchain and `clang` (calls use the LiveKit Rust SDK; the first build downloads libwebrtc, 156 MB).

```
make build                  # both clients: release binary in target/release/soworker
make build-tui              # the terminal client only (smaller)
make build-windows          # cross-compile soworker.exe from Linux (needs cargo-xwin, see the Makefile)
make install                # install to ~/.cargo/bin (registers the launcher entry on first start)
make dev                    # rebuild and restart on every source change (needs cargo-watch)
make test                   # cargo test
make release [BUMP=minor]   # bump version (patch by default), build, commit, tag, push, publish a GitHub release
```

Run with `soworker --dry-run` (connect and watch, sends nothing) or `soworker` (live).
`cargo build --release --no-default-features --features tui` gives a small build without calls.

Cargo features: `tui` (terminal client, default), `gui` (window client, default), `calls` (audio and video through LiveKit,
default), `portal` (global push-to-talk shortcut through the desktop portal, default). Internal ones: `window` (the
bare video window), `render` and `overlay` (software drawing, the always-on-top "you are live" pill; both come with `gui`).

**Which client starts:** `--tui` or `--gui` if given; otherwise the terminal client when run from a terminal; started from a
launcher icon (no terminal) it opens whichever you used last (the window the first time), and the terminal client is then
opened inside a terminal emulator (`$TERMINAL`, `xdg-terminal-exec`, or the usual ones; set `"terminal": "<your terminal's run-a-command syntax> {}"` in
`~/.config/soworker/settings.json` to choose). In the terminal, `G` (or the button top right) hands over to the window and
quits; in the window, the Terminal button in the sidebar footer does the opposite. The last choice is kept in `~/.config/soworker/last-ui`.

On first start there is a sign-in screen: Enter opens Chrome on SoWork's login page, takes the token once you are
signed in and closes it again. The token is stored in `~/.config/soworker/session.json` (use `--session <file>` for
another path). Log out (in the profile: `u` in the terminal, the user icon in the window's sidebar footer; `O` also
works in the terminal) forgets it. Diagnostics go to `~/.local/state/soworker/soworker.log` (set
`SOWORKER_LOG_STDERR=1` to see them on screen instead).

## Windows

`soworker.exe` is one self-contained file (no installer, no DLLs to ship). Both clients work; from Explorer or the
Start menu the window client opens (a console flashes up for a moment), from a terminal the terminal client.

- Sign-in uses Chrome if installed, else Edge. Files live in `%APPDATA%\soworker\` (session, `settings.json`,
  `last-ui`); the log (and any crash message) is `%LOCALAPPDATA%\soworker\soworker.log`.
- Calls: microphone, speakers, camera (Media Foundation) and screen share work. Screen share shares the first screen,
  there is no picker.
- Push to talk: the shortcut (`talkKey`, `Super+'` by default) works whichever window is in front, press and hold.
- Not there yet: the "you are live" pill and desktop notifications, inline video in the terminal (it opens in a window).
- Windows "N" editions need the Media Feature Pack, or the program does not start.

Build it on Windows with the MSVC build tools: `cargo build --release` (the `.cargo/config.toml` sets the static C
runtime libwebrtc needs). From Linux: `make build-windows`.

## Using it

- **Walk:** WASD or the arrows, or click the map (Shift+click teleports). `+`/`-` or the wheel zoom, `r` respawns, `b` available/away.
- **People:** `Tab` focuses the list, arrows select, Enter opens Join / Wave / Message under a person or a meeting.
  `p` hides it. A wave shows up there too: Enter walks to the person, `x` dismisses.
- **Calls:** `m` mic, `c` camera, `v` share screen, `l` leave, `Space` talk (hold it where the terminal reports key
  releases, otherwise it toggles). `t` opens the chat and starts typing, `Esc` leaves the input, `T` closes it.
- **Noise cancellation:** on by default: the microphone goes through [DeepFilterNet 3](https://github.com/Rikorose/DeepFilterNet)
  (built in) after WebRTC's echo cancellation. `N` in the terminal, or the waveform button left of the camera in the
  window, switches it (also during a call); the choice is kept as `noiseCancellation` in `settings.json`. While it is
  on, the mic is captured with `parec` (or `pw-record`) on Linux and WASAPI on Windows; off, libwebrtc captures it
  itself and uses its own noise suppression.
- **Profile and devices:** `u` in the terminal (or the `u profile` button top right), the user icon next to the memory
  use in the window. It shows who is signed in, Log out, and the settings: the speaker (with a test sound), the
  microphone (with a level meter) and the camera, each the system default or a device picked from what is plugged in
  (`pactl` and `/dev/video*` on Linux, WASAPI and Media Foundation on Windows). A change applies to a running call too;
  the choices are kept as `audioOutput`, `audioInput` and `camera` in `settings.json`.
- **Video:** where the terminal draws kitty graphics (kitty, ghostty, WezTerm, Konsole) the picture is shown inline in
  the call panel and `n` switches between camera and screen share. Anywhere else (and inside tmux) it opens in a small
  window of its own, started on demand. Force one with `--video inline|window|off`; `soworker --term-check` says what your terminal supports. COSMIC Terminal draws no inline graphics, so there the window is used. Pictures nobody looks at are not
  even converted.
- **Keys:** `?` lists them. Mouse clicks work on the map, the list and the chat.
- **Push to talk:** `Space` in the terminal, or with the terminal hidden the global shortcut (`Super+'` where the desktop has a shortcut portal) or `soworker --talk` bound to a key (see COSMIC setup below and `docs/push-to-talk.md`).
- **Useful flags:** `--roster` prints the member list, `--talk` controls a running instance, `--gui` starts the window client.

## Push to talk on COSMIC

COSMIC has no global-shortcut portal, so bind `soworker --talk` to a key yourself (once). A desktop shortcut cannot report
the key release, so press once to open the mic and again to close it. From a terminal, with `BIN` set to your built binary:

```
BIN=$PWD/target/release/soworker        # run from the repo root after `make build`
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

COSMIC reloads the file by itself (or add the same thing in Settings > Keyboard > Custom shortcuts). Change `Super` /
`apostrophe` for another key, and set `talkKey` in `~/.config/soworker/settings.json` to the same keys (e.g. `"Super+'"`)
so the in-app hint matches. Test with `soworker --talk` in a terminal while the app is running.

## More

- `docs/`: reverse-engineered protocol and behaviour notes (start with `docs/FINDINGS.md`).
- `src/`: the core has no drawing in it: `session` (sign-in), `net` (socket), `proto` (events), `world` (collision,
  pathfinding), `app` (state), `directory` / `meeting` / `chatstate` (what the people list, the call and the chat show),
  `media` (calls), `mic` (the noise-cancelled microphone), `backend` (the threads behind a session). `tui/` is the terminal client (`view` draws, `kitty` is
  the inline video), `videowin` the bare video window, `gui/` the window client.
