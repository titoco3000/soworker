# SoWorker app (prototype)

Plain top-down client for our SoWork office: floors, walls, meeting zones, people, and walking.
No calls, chat or avatars yet.

```
node app/import-token.mjs recorder/logs/<run>   # once: saves the Firebase refresh token (app/.session.json, mode 600)
node app/server.mjs --dry-run                    # watch only: connects, shows the office, never sends anything
node app/server.mjs                              # live: your avatar walks for real, colleagues see it
```

Needs Node >= 22 and Chrome (opens a `--app` window). WASD or arrow keys to walk, mouse wheel to zoom.

How it works: `server.mjs` is a small bridge that signs in with the refresh token, finds the room
(`gaia/find`), opens the gaia socket (`lib/gaia.mjs`) and streams the world and people to
`public/client.js` over Server-Sent Events. The page does movement (800 units/s, 5 `move`/s while
walking) and collision (floor tiles are walkable, walls are tile-edge segments) locally, like the
official client, and posts positions back to the bridge.

Behaviour to know about:
- Connecting uses `joinType=force`, which kicks any other open SoWork session of the same account.
- Live mode sets your status to away (`cluster_mode none-away`) because there is no audio or video.
- Not implemented: furniture collision, meeting zones (`set_meeting_zone`), LiveKit calls, chat,
  avatar art, token re-import without a recorder run (first login still needs the official page).
- Tauri was the plan but this machine lacks the WebKitGTK dev packages; the page is plain
  HTML/JS and ports over unchanged once the bridge logic moves into Rust.
