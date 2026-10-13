# soworker

Unofficial native client for SoWork, built from analysis of the web client's traffic.
See `PLAN.md` for the roadmap, `docs/` for findings and `native/` for the app.

## Recording SoWork traffic

Traffic is captured with [web-client-recorder](https://github.com/titoco3000/web-client-recorder)
(Node >= 22 or Bun, plus Chrome):

```
bunx github:titoco3000/web-client-recorder --website https://www.app.sowork.com/
```

Run it from the directory where `output/` should be created. It holds `output/raw-logs/`,
`output/logs/` and `output/profile/`.

1. A Chrome window opens on SoWork. Log in (the profile in `output/profile/` keeps you logged in between runs).
2. Use the app as usual: join a meeting, chat, send waves, mute and unmute, move around the map.
3. Press Ctrl+C in the terminal to stop. Redacted working files (HTTP table, API bodies, decoded
   WebSocket frames, WebRTC events) land in `output/logs/<timestamp>/`. The raw run in
   `output/raw-logs/` is deleted afterwards unless you passed `--preserve-raw-logs`.
4. If a run was cut short or you want to reapply updated redaction, rebuild its working files with
   `bunx github:titoco3000/web-client-recorder --process output/raw-logs/<timestamp>`.

Logs hold live session tokens and cookies. Keep them local and git-ignored, and never paste raw
logs anywhere.
