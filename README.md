# SoWorker

Unofficial native client for [SoWork](https://www.sowork.com): walk around the office, see who is where, wave, chat and
join calls without a browser tab. Rust, one process, software rendering. Chrome is only used for the sign-in.

## Build and run

Needs a Rust toolchain and `clang` (calls use the LiveKit Rust SDK; the first build downloads libwebrtc, 156 MB).

```
make build                  # release binary in target/release/soworker
make install                # install to ~/.cargo/bin (registers the launcher entry on first start)
make dev                    # rebuild and restart on every source change (needs cargo-watch)
make test                   # cargo test
make release [BUMP=minor]   # bump version (patch by default), build, commit, tag, push, publish a GitHub release
```

Run with `soworker --dry-run` (connect and watch, sends nothing) or `soworker` (live).
`cargo build --release --no-default-features` gives a small build without calls.

On first start there is a sign-in screen: the button opens Chrome on SoWork's login page, takes the token once you are
signed in and closes it again. The token is stored in `~/.config/soworker/session.json` (use `--session <file>` for
another path). Log out in the sidebar forgets it.

## Using it

- **Keys:** WASD/arrows or left click walk (Shift+click teleports), R respawn, B available/away, M mic, C camera, wheel zoom.
- **Sidebar:** people grouped by meeting; click someone for Join, Wave or Message. Chat panel on the right.
- **Push to talk:** hold `Super+'`, or bind `soworker --talk` to a key when the window is hidden (see COSMIC setup below and `docs/push-to-talk.md`).
- **Useful flags:** `--roster` prints the member list, `--talk` controls a running instance.

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
- `src/`: `session` (sign-in), `net` (socket), `proto` (events), `world` (collision, pathfinding), `app` (state), `render`, `window`, `media` (calls).
