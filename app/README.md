# SoWorker app (prototype)

Plain top-down client for our SoWork office: floors, walls, meeting zones, people, and walking.
No calls, chat or avatars yet.

```
cd app && npm install                            # once: livekit-client
node app/import-token.mjs recorder/logs/<run>   # once: saves the Firebase refresh token (app/.session.json, mode 600)
node app/server.mjs --dry-run                    # watch only: connects, shows the office, never sends anything
node app/server.mjs                              # live: your avatar walks for real, colleagues see it
```

Needs Node >= 22 and Chrome (opens a `--app` window). WASD or arrow keys to walk, left click anywhere to walk there (routes around walls; any key cancels), mouse wheel to zoom, F3 toggles the debug panel (fps, page JS heap, bridge and Chrome memory).

How it works: `server.mjs` is a small bridge that signs in with the refresh token, finds the room
(`gaia/find`), opens the gaia socket (`lib/gaia.mjs`) and streams the world and people to
`public/client.js` over Server-Sent Events. The page does movement (800 units/s, 5 `move`/s while
walking) and collision (floor tiles are walkable, walls are tile-edge segments) locally, like the
official client, and posts positions back to the bridge.

Tests without touching SoWork: `node app/test/fake-bridge.mjs recorder/logs/<run>` serves an offline world, `node app/test/click.mjs <port> <dx> <dy>` clicks in headless Chrome.

Behaviour to know about:
- Connecting uses `joinType=force`, which kicks any other open SoWork session of the same account.
- Live mode sets you available (`cluster_mode cluster`) so calls can reach you; the away button switches to `none-away`.
- Calls: walking into a meeting zone (or, if the server groups by proximity, standing near someone while
  available) makes the server send a LiveKit token; the page joins with `livekit-client` (loaded on demand)
  **muted, camera off**. Buttons / keys: M mic, C camera, available/away toggle. Untested live.
- Not implemented: furniture collision, screen share, chat, avatar art, token re-import without a recorder run (first login still needs the official page).
- Tauri was the plan but this machine lacks the WebKitGTK dev packages; the page is plain
  HTML/JS and ports over unchanged once the bridge logic moves into Rust.
