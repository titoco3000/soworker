# SoWorker

Unofficial native client for [SoWork](https://www.sowork.com): the office, people, chat and calls, without a browser.
Runs in Linux and Windows as either in the terminal or as window, in Android as an app, and even as a Minecraft mod.

## Build and run

Needs Rust and `clang`. The first build downloads libwebrtc (156 MB).

```
make build      # target/release/soworker
make install    # installs build into ~/.cargo/bin
make test
```

`soworker` starts the terminal client from a terminal and the window otherwise (`--tui` / `--gui` to choose).
`--dry-run` connects without sending anything. The first start signs in through Chrome (Edge on Windows).

Files: `~/.config/soworker/` (session, `settings.json`), log in `~/.local/state/soworker/soworker.log`. On Windows
`%APPDATA%\soworker\` and `%LOCALAPPDATA%\soworker\soworker.log`.

## Shortcuts

`?` from the terminal lists every key. The main ones:

| | |
|---|---|
| Walk | WASD / arrows, or click the map |
| Call | `m` mic, `c` camera, `v` share, `h` hand, `l` leave
| Record | `R` (needs `ffmpeg`; transcribed with whisper.cpp if installed) |
| Noise cancellation | `N`; strength and a mic test in the profile |
| Settings | `u` |

## Performance

<!-- bench:start -->
| | Idle CPU | Idle memory | Call CPU (--join) | Call memory (--join) |
|---|---:|---:|---:|---:|
| SoWorker (window) | 0.4% | 24 MB | 8.5% | 39 MB |
| SoWorker (terminal) | 0.3% | 18 MB | 9.2% | 32 MB |
| SoWork in Chrome | 4.1% | 670 MB | 11.1% | 680 MB |

Tested in *2026-11-26* with 3 run(s) on:
- AMD Ryzen 7 7435HS
- Linux 7.2.11-200.fc44.x86_64
- SoWorker 0.1.9
- Google Chrome 156.0.8078.47
<!-- bench:end -->

CPU in % of one core; memory is the PSS of every process the client starts.

**Idle**: 120 s standing in the office. 

**Call**: then 120 s in a colleague's call with the mic on and no noise cancelling; only measured when the run
names one (`--join <name>`), "—" otherwise.

To reproduce, run with SoWorker and any SoWork tab closed:

```
make bench-login                          # once: sign the benchmark's Chrome in to SoWork
make bench [JOIN=<name>] [RUNS=3] [README=1]
```

Results land in `bench/results/`; `README=1` updates the table above.

## More

Push to talk outside the app: see [docs/push-to-talk.md](docs/push-to-talk.md) (includes COSMIC setup).

Minecraft mod: see [docs/minecraft.md](docs/minecraft.md).

`docs/` has the reverse-engineered protocol notes (start with `docs/FINDINGS.md`).
