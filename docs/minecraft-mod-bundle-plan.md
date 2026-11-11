# Minecraft mod with soworker built in: plan

Status: built (2026-11-10) except where noted. Checked: `make mc-test` (port line, token, stdin EOF, `no_browser`,
`failed` + retry), the jar's `Child`/`Bridge` outside Minecraft (extract, start, connect, stop; child gone after
`kill -9` of the JVM). Open: the Windows binary (not built yet: `make mc-windows`), the old-glibc Linux build (the
binary built on Fedora 44 needs glibc 2.43; `make mc-linux` prints it), confirming the "replaced" signal from a
reproduction log (implemented as gaia's Socket.IO DISCONNECT packet, logged as `gaia: disconnected by the server`),
and the manual tests. Builds on `docs/minecraft-bridge-plan.md` (the bridge itself, its protocol, the
office in the world), which stays valid except where this document replaces it.

## Goal

A user with no soworker installed copies one jar into `mods/`, opens Minecraft, types `/sw enable` in their office
world, and it works: if they have no saved sign-in, Chrome/Chromium/Edge opens on SoWork's login page; once they sign
in, the office builds.

Minecraft mode stops being a soworker execution mode. The only way into it is the mod.

Requirements the jar cannot carry (say so on the mod page):

- Minecraft 26.3, Fabric Loader ≥ 0.20, Fabric API.
- Chrome, Chromium or Edge, for the first sign-in.
- Linux: PulseAudio or PipeWire (`pactl`, `parec`/`pw-record`); `ffmpeg` for recording with video. Windows: nothing.
- Linux x86_64 and Windows x86_64 only (no macOS build exists).

## Architecture

```
 Minecraft (JVM)                                   child process (extracted from the jar)
 Fabric mod ── spawns, owns stdin ──────────────▶  soworker --mc-child
            ◀── ws://127.0.0.1:<random port> ───   Backend + State, LiveKit, login via Chrome
                 token from the environment
```

Same Rust code and the same WebSocket protocol as today, in a separate process so a crash in libwebrtc cannot take
Minecraft down. Loading Rust into the JVM (JNI/FFM) was considered and rejected: crash isolation, LiveKit's glib main
loop and threads inside the JVM, and no real gain.

## 1. Rust: the child mode

`src/mc/mod.rs`, `src/mc/ws.rs`, `src/main.rs`.

- Replace `--mc` with `--mc-child` (internal, not documented for users). `--mc-demo` and `--dry-run` keep working
  under it for development.
- Port: bind `127.0.0.1:0` and print `port <n>` as the first stdout line; the mod reads it. Removes the fixed 25599 and
  any clash with another copy.
- Token: read from the `SOWORKER_MC_TOKEN` environment variable (the mod generates it per launch). Not on the command
  line (visible in `ps`). Remove `ws::token`, `ws::token_path` and the `mc-token` file.
- Lifetime: a thread reads stdin and exits the process on EOF. The mod holds the pipe, so the child dies with
  Minecraft, crash included, on Linux and Windows alike. Keep `--exit-after` for tests.
- Sign-in reported to the mod as a new message `{"t":"login","state":…}`:
  - `browser` — no usable session, Chrome opened (`begin_login`);
  - `no_browser` — no Chrome/Chromium/Edge found;
  - `failed` + `error` — capture failed or was cancelled;
  - `offline` — retrying every 5 s (existing `retry_login_at`);
  - `signed_in` + `name`.
- A failed sign-in no longer ends the process (today `Event::Login(Err)` returns `Err`, `mc/mod.rs:604`): send
  `failed`, stay up, and start the login again on a new `{"t":"cmd","name":"login"}` from the mod.
- New command `logout`: delete the saved session, leave the call/space, send `login` state `browser` after opening
  Chrome again (switching accounts).
- Session: keep `session::default_path()` (`~/.config/soworker/session.json`, `%APPDATA%\soworker` on Windows). A user
  who also has desktop soworker signed in gets no browser at all.
- Same user signed in twice: SoWork lets one instance per user; starting a second one makes the server disconnect the
  first ("disconnected by server" in the original). So the mod's child kicks a running desktop soworker, and opening
  desktop soworker kicks the child. Handle it in the child:
  - first find which layer reports it (the SoWork socket in `net.rs`, or LiveKit's `RoomEvent::Disconnected` in
    `media.rs`) from the log of a reproduction, and the exact reason it carries;
  - on that reason, do not reconnect (`net.rs` retries every 2 s today, which would kick the other instance back in an
    endless swap): send `{"t":"login","state":"replaced"}` and wait;
  - the mod says "SoWork was opened somewhere else. `/sw login` to use it here again", and `/sw login` reconnects
    (which in turn disconnects the other one, as expected).
  The desktop clients should get the same "do not reconnect after being replaced" fix (same code path).
- Protocol version: `hello` (already sent on connect) gains `"proto": N`; the mod logs a warning on mismatch (only
  possible with a development binary override).
- Logging: keep `log_to_file` (state directory). Remove the "waiting for Minecraft" banner aimed at a terminal user.

## 2. Rust: the build for the mod

- Feature set: `--no-default-features --features mc,calls` (checked: compiles today). No `tui`, `gui`, `window`,
  `portal`.
- Remove `mc` from `default` features in `Cargo.toml`: the desktop binary no longer contains the bridge.
- Makefile: `make mc` builds both binaries and the jar:
  - Linux: `cargo build --release --no-default-features --features mc,calls --target-dir target/mc`.
  - Windows: same through the `build-windows` recipe (cargo-xwin), with the same features. Verify the `mc` code
    compiles for Windows (`talk::listen` has a Windows variant; `ws.rs` is plain std).
- glibc: a binary built on Fedora 44 needs Fedora 44's glibc and will not start on older distros
  (`GLIBC_2.xx not found`). Build the Linux binary in an old-glibc container (Debian 12 / Ubuntu 22.04 class), or at
  least check `objdump -T | grep GLIBC_` and state the minimum. Same applies to the existing desktop release.
- Strip the binaries; measure the final jar size.

## 3. Java: the mod starts soworker

New `mc/src/main/java/dev/soworker/mc/Child.java`; changes in `Bridge.java`, `Office.java`, `SoworkerMod.java`.

- Jar layout: `natives/linux-x86_64/soworker`, `natives/windows-x86_64/soworker.exe`, each with a `.sha256` next to it.
- Extract on first start to `<soworker config dir>/mc-bin/<sha256 prefix>/` (same base dir as `Bridge.tokenPath()`
  uses today), `setExecutable(true)` on Linux; skip when the file is there with the right hash. Delete other
  `mc-bin/*` dirs that are not in use.
- Unsupported OS/arch: chat "SoWork in Minecraft runs on Linux and Windows x86_64 only" and do nothing.
- Start: `ProcessBuilder(bin, "--mc-child")`, env `SOWORKER_MC_TOKEN=<random 24 bytes hex>`, stdin piped and kept open,
  stdout read on a thread (first line `port <n>`, the rest to the game log), stderr to the game log.
- `Bridge` takes port and token from `Child` instead of the fixed port and the `mc-token` file; reconnect logic stays.
- When it starts:
  - `/sw enable` (and the world's saved `enabled` flag on `SERVER_STARTED`): start the child if not running;
  - `/sw disable`, `SERVER_STOPPING`, and a JVM shutdown hook: close stdin, wait 2 s, then `destroy()`.
- Child exits unexpectedly: chat "SoWork stopped (see soworker.log)", restart at most 3 times per minute, then stop
  and tell the user to run `/sw enable` again.
- Sign-in messages in chat, from the `login` message:
  - `browser`: "Sign in to SoWork in the browser window that just opened";
  - `no_browser`: "Install Chrome, Chromium or Edge to sign in";
  - `failed`: the error, plus "`/sw login` to try again";
  - `signed_in`: "Signed in as <name>: building your office";
  - `replaced`: see "Same user signed in twice" in section 1.
- New commands: `/sw login` (retry), `/sw logout`.
- `Office.enable()`: replace "Start soworker --mc to connect." with "Starting SoWork…".
- `fabric.mod.json`: description no longer mentions `soworker --mc`; `environment` stays `*`.
- Development overrides: `-Dsoworker.bin=<path>` runs that binary instead of the bundled one (no extraction);
  `-Dsoworker.args="--mc-demo"` adds arguments.

## 4. Packaging

`mc/build.py`:

- `--bin linux=<path> --bin windows=<path>` (the Makefile passes both); copy them into `natives/…` with their hashes.
- A missing platform binary is an error unless `--allow-missing` (for quick local builds).
- Output `mc/build/soworker-mc-<version>.jar`; `--install` unchanged. Take the version from `Cargo.toml` so the
  mod and the binary inside it carry one number.
- Distribution: the GitHub release only. `make release` builds the mod's two binaries and the jar along with the
  desktop builds and attaches `soworker-mc-<version>.jar` to the release (next to the Linux tarball and Windows zip).
  If the mod build fails, the release fails (same undo as today); the Windows half follows the existing "release
  without Windows?" prompt, and a jar without the Windows binary is then named as such in the release notes.
- The binaries are bundled in the jar, not downloaded on first start.

## 5. Cleanup of the old Minecraft mode

Remove:

- `/minecraft` chat command: `src/chatstate.rs:121` and its test `minecraft_in_the_chat_box_…` (`chatstate.rs:223`).
- `UiMode::Mc` and everything for it in `src/mode.rs`: the variant, `built`/`flag`/`back` arms, the `--back` option,
  the `--mc`/`--mc-port`/`--mc-demo`/`--back` filtering in `client_args`, the `Mc` branch in `relaunch` (both unix and
  windows), the notification in `started`, and their tests (`mode.rs:372–383`).
- `/sw exit` (`SoworkerMod.java:153`), the `exit` command and the `exit` field / relaunch in `src/mc/mod.rs`. There is
  no soworker client to go back to; `/sw disable` is the way out.
- `--mc` dispatch in `src/main.rs:132` (replaced by `--mc-child`).
- `mc-token` file and fixed port: `ws::token`, `ws::token_path`, `Bridge.tokenPath()` token reading,
  `SoworkerMod.PORT` / `soworker.port`. Existing `~/.config/soworker/mc-token` files: the mod deletes one if found.
- `mc/__pycache__/build.cpython-314.pyc`: tracked by mistake; delete and add `__pycache__/` to `.gitignore` (also
  `analysis/scratch/__pycache__/`).
- `analysis/scratch/mc_client.py`: update to the new flow (`--port` required, token from `SOWORKER_MC_TOKEN`), or
  delete if no longer used.
- Desktop `soworker` keeps no Minecraft code at all (feature `mc` off by default; the `mc` module only builds for the
  mod).

Keep: `src/mc/*` (the bridge), `--mc-demo` (under `--mc-child`), `DevShots` (off unless `-Dsoworker.shotdir`).

Docs:

- `docs/minecraft-bridge-plan.md`: replace "soworker --mc" with the child process in Architecture and Protocol
  (port, auth), drop the transition section, link here.
- `README.md`: a "Minecraft" section, for users who never installed soworker:
  1. Requirements (from "Goal": Minecraft 26.3, browser, Linux audio tools, Linux/Windows x86_64).
  2. Install Fabric Loader for 26.3 (fabricmc.net installer) and Fabric API into `mods/`.
  3. Download `soworker-mc-<version>.jar` from the latest GitHub release into `mods/`.
  4. Start Minecraft with the Fabric profile, create a superflat "void" world (singleplayer).
  5. `/sw enable`; sign in in the browser window that opens the first time.
  6. What you can do: walking moves your avatar, cobblestone is a meeting, hotbar tools, snowball to wave, ender
     pearl to teleport, the `/sw` commands (`mic share rec cam desk away hand leave wave say status login logout
     disable`).
  7. Opening desktop soworker while in Minecraft disconnects the Minecraft one (and the other way round);
     `/sw login` takes it back.
  8. Where the log is (`soworker.log` in the state directory) for bug reports.
  Remove any mention of `soworker --mc`.

## 6. Tests

Rust (`cargo test`):

- `--mc-child` prints `port <n>` first and the port accepts the token from the environment, rejects another.
- Stdin EOF ends the process (spawn the test binary, close stdin, wait ≤ 2 s).
- Login failure keeps the process up and emits `login` `failed`.
- Mode tests updated after the `UiMode::Mc` removal.

Manual, before release:

1. Fresh Linux user (no `~/.config/soworker`), Minecraft 26.3 + Fabric: jar in `mods/`, new world, `/sw enable` →
   browser opens → sign in → office builds, voice works.
2. Same on a Windows machine/VM.
3. No browser installed → clear chat message, no crash.
4. Quit the world, kill Minecraft (`kill -9`): no `soworker` process left.
5. Reopen the world: no browser (session saved), office back.
6. `/sw logout` → browser again; `/sw disable` → child gone.
7. Linux binary on the oldest distro we claim to support.
8. Desktop soworker open, then `/sw enable`: desktop is disconnected and stays so (no swap loop); `/sw login` after
   reopening desktop takes it back. Then the other way round.
9. `make release` in a scratch tag run: the jar is attached and installs as in the README.

## Order of work

1. Rust child mode (section 1) with `--mc-child`, tested with `mc_client.py` and `-Dsoworker.bin`.
2. `Child.java` and the `Bridge` changes (section 3); test with `--mc-demo` via `-Dsoworker.args`.
3. Packaging (sections 2, 4), Windows build, old-glibc Linux build.
4. Cleanup (section 5) and docs.
5. Manual test pass; first live (non-dry-run) test only when the user says so, as before.
