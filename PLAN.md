# SoWorker: lightweight SoWork client

Goal: a fast, plain-looking client for our company's SoWork space that keeps the core
mechanic (avatar walks around the office map, proximity starts audio/video).

Known constraints and risks (decided, not re-litigated here):
- SoWork's ToS forbids reverse engineering and third-party clients. Their account-termination
  clause is broad. We use our own account, keep captured logs local, and never share tokens.
- No public API exists. Everything below is inferred from traffic and will break when they ship.

## Step 1: Recorder (done)

`recorder/record.mjs` launches Chrome with a dedicated profile (`recorder/profile/`, stays logged
in between runs) and attaches over the DevTools protocol. Zero dependencies, Node >= 22.

```
node recorder/record.mjs https://<your-sowork-url>      # default https://www.sowork.com/
node recorder/record.mjs --chrome /usr/bin/chromium     # other binary
```

Use SoWork in the window that opens. Ctrl+C in the terminal to stop (or close the window; the
last periodic snapshot is at most 60 s old).

Output in `recorder/logs/<timestamp>/`:
- `events.jsonl`, one JSON object per line. Common fields: `t` (ms epoch), `k` (kind), `tg`
  (target number: tab, iframe, worker; mapped to a URL by the `target` events).
- `bodies/` response bodies above 16 KB, named by content hash (JS bundles, JSON, wasm, binary).
  Smaller bodies are inline in `res.body` events (`text`, or `b64` for binary).
- `meta.json` run info and Chrome version.

Event kinds:

| kind | content |
|---|---|
| `req`, `req.extra`, `req.postData` | URL, method, headers (extra = real wire headers incl. cookies), body |
| `res`, `res.extra`, `res.body` | status, headers (extra = incl. set-cookie), body or file |
| `fail` | failed request |
| `ws.open`, `ws.hs.req`, `ws.hs.res`, `ws.in`, `ws.out`, `ws.close` | WebSocket; `op` 1 = text, 2 = binary (base64 in `data`) |
| `sse` | Server-Sent Events |
| `pc.*` | RTCPeerConnection: config (ICE/TURN servers), SDP offer/answer, ICE candidates, state changes, tracks, 15 s stats |
| `dc.new/open/in/out/close` | RTCDataChannel lifecycle and every message (capped at 64 KB each) |
| `media.getUserMedia/getDisplayMedia` | requested constraints |
| `nav`, `target`, `target.gone` | navigation and target tree |
| `storage`, `cookies` | localStorage, sessionStorage, IndexedDB names, all cookies (every 60 s and at exit) |

Notes:
- The HTTP cache is disabled in the recorded tabs so every JS bundle is captured on each load.
- Logs contain live session tokens and cookies. Directory is mode 0700 and git-ignored.
  Do not paste raw logs anywhere, including into a chat. Redact first.
- Expect tens to a few hundred MB for a couple of hours, mostly position updates.

### Recording script (do all of these, in this order, note wall-clock time of each)

The point is to isolate each message type. Pause about 10 s between actions so they separate
cleanly in the log.

1. Fresh start: log out, clear nothing else, log in (captures the whole auth flow, SSO included).
2. Stand still for 30 s (baseline traffic: heartbeats, presence).
3. Walk one tile in each direction, one key press at a time. Then hold a direction for 5 s.
4. Walk into a wall and into furniture or another avatar.
5. Click-to-move, if the map supports it.
6. Walk next to a teammate until audio connects; walk away until it drops. Repeat twice.
7. Mute, unmute, camera on, camera off, screen share start and stop.
8. Join a private room or meeting area, leave.
9. Send chat message (room chat and DM), emoji reaction, status change, wave or emote.
10. Open the map list or teleport to another floor, if any.
11. Idle 5 min with the tab in the background, then foreground again.
12. Reload the tab mid-session (shows reconnect and state resync), then kill the network for
    20 s and restore it (shows reconnect and backoff).
13. Log out.

Tell me the wall-clock time of each action when you hand over the logs. Even rough minutes help.

## Step 2: Study logs and write the protocol spec (done: see docs/FINDINGS.md)

Result: gate passed. Walking is client-trusted absolute `move {x, y}` over Socket.IO + msgpack,
media is stock LiveKit, auth is a Firebase ID token. Step 3 order is revised in
docs/FINDINGS.md (auth spike first). Spike done: refresh token -> ID token -> gaia/find works from plain
Node, no Origin needed.

Done after the recording, not before. Planned work:

1. `analysis/summarize.mjs`: hosts and endpoint table (method, path pattern, count, status
   codes), WebSocket endpoints, message-type histogram per socket and per direction, data channel
   labels, ICE servers, binary vs text split.
2. Auth: reconstruct login (SSO redirects, which token ends up where, lifetimes, refresh).
   Decide what our client needs: cookie, bearer, or a token minted by a cheaper endpoint.
3. Transport map: which channel carries what. Candidates: WebSocket for world state and chat,
   data channel for positions, SFU WebRTC for media. Identify the media stack from SDP and
   signaling shape (LiveKit, mediasoup, Agora, own), since that decides how hard media is.
4. Message formats: JSON (read directly), or protobuf/msgpack/custom binary. For binary, extract
   schemas from the JS bundles in `bodies/` (search for message names, `.proto` descriptors,
   enum tables). `protoc --decode_raw` as a fallback.
5. Walk mechanic, the critical path. Answer from the movement window of the log:
   - what does a key press send (direction or absolute target), at what rate;
   - is the server authoritative (does it echo or correct positions), or is it client-trusted;
   - tick rate, interpolation hints, speed units;
   - how other avatars' positions arrive (full snapshots vs deltas) and how often;
   - collision: client-side (map data) or server-side;
   - proximity: who decides who is in range (server pushes join/leave audio events, or the
     client computes distance and asks for connections).
6. Map data: where tiles, collision and furniture come from (JSON, tilemap, image atlas), and
   asset URLs and auth on them.
7. Media connection lifecycle: signaling messages, token minting, which SDP and codecs, how
   tracks map to users, how mute state is signaled.
8. Output: `docs/protocol.md` (per-message reference with real examples), `docs/auth.md`,
   and a list of unknowns with the extra recordings needed to close them.

Gate before step 3: if the walk mechanic is server-authoritative with an opaque signed or
encrypted payload we cannot reproduce, stop and reassess (restyle-only fallback in the old
discussion).

## Step 3: Minimal client

Priority: small, fast, plain. Walk mechanic must work; everything else is optional.

Stack (revisit after step 2):
- Tauri v2 shell. Frontend: vanilla JavaScript and Canvas 2D, no UI framework, no game engine.
  Flat avatars (colored circle or sprite), tile map drawn once to an offscreen canvas, redraw
  only on change or at a capped 30 fps; pause rendering when the window is hidden.
- Network from Rust, not from the webview, to avoid CORS and control `Origin` and headers:
  `tauri-plugin-http` for REST, `tauri-plugin-websocket` for sockets (check it accepts custom
  headers; otherwise a small `tokio-tungstenite` command).
- Media from the webview's own WebRTC.

Risk to settle first (milestone 0): Tauri on Linux uses WebKitGTK, whose WebRTC and
`getUserMedia` support depends on the distro build and is often broken or partial. Spike this
before anything else. If it fails, fall back to the same frontend in Chrome `--app` mode served
from a tiny local Rust or Node server that proxies and rewrites requests (same CORS and Origin
benefits). Windows (WebView2) and macOS (WKWebView) are fine.

Milestones:
- M0 Spike: Tauri window with `getUserMedia` and a loopback `RTCPeerConnection`; a Rust-side
  fetch with a custom `Origin` header.
- M1 Auth: log in, hold and refresh the session, persist it in the OS keychain.
- M2 World: join the office, fetch the map, render it, show other avatars where they are
  (read-only, no walking). Proves the realtime channel and the protocol decoding.
- M3 Walk: keyboard input, send movement as the real client does, local prediction,
  interpolate remote avatars, handle server corrections. This is the "minimum".
- M4 Proximity audio and video: connect and disconnect on range, mute and camera toggles,
  simple video tiles. Largest unknown, depends on the media stack found in step 2.
- M5 Extras if wanted: chat, status, emotes, screen share view.

Definition of done for the minimum (M0 to M3): walk around the real office, see colleagues move
in real time, and be seen walking by them, using under 150 MB RAM and idle CPU near 0.

## Native port (decided: fully native Rust)

`native/` replaces `app/` (Node + Chrome). World view, walking, click-to-move, zones and presence are done and
tested offline against recorded runs (~30 MB RSS in a real window vs ~270 MB). Live networking is written but
untested. Calls: see docs/native-media-feasibility.md (official LiveKit Rust SDK needs clang++/g++ and ~85 MB
libwebrtc; a pure-Rust audio path was built as a spike: 5 MB binary, 6.5 MB idle RSS).
