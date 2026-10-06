# Findings from the first recording

Source: one recording of about 4 minutes (login with Google, enter the office, walk, two
zone-based calls, one screen share). Four detailed references were written from it:

| Doc | Covers |
|---|---|
| [auth-and-rest.md](auth-and-rest.md) | Firebase auth, REST catalog, Firestore, chat socket, bootstrap sequence |
| [gaia-protocol.md](gaia-protocol.md) | Realtime socket (Socket.IO + msgpack): event catalog, walking, zones, map objects |
| [media-livekit.md](media-livekit.md) | LiveKit rooms, tokens, tracks, screen share, signalling |
| [client-and-map.md](client-and-map.md) | Client architecture, WASM renderer, map object format, assets, client-side movement |

Everything is inferred from traffic and the shipped JS; nothing was labelled by the user, and
no remote avatar was seen walking. Facts and guesses are separated inside each doc.

## Architecture in one picture

```
 Google sign-in (Firebase Auth, project sage-learning-prod)
        |  Firebase ID token (1 h), the ONLY credential
        v
 REST  api.sowork.com   Bearer token   POST /api/v1/gaia/find: invite link -> roomId
 Chat  chat.sowork.com/ws?token=...    JSON, heartbeat + activity.report (+ chat events from bundle)
 Gaia  api.sowork.com/gaia/g5/v2/      Socket.IO v4 + msgpack: world state, movement, presence
        |  'cluster' event carries a LiveKit token + roomId + apiHost
        v
 Media LiveKit Cloud (livekit-client 2.20.0, unmodified), one room per conversation
 Misc  Firestore listeners (groups, profiles, notifications), Intercom/Amplitude/Sentry/Meta/GA (noise)
```

## What this means for the rewrite

The gate from PLAN.md step 2 passes. The walk mechanic is simple and client-trusted:

- **Walking:** the client sends `move {x, y}`, an absolute world position, about every 205 ms
  (~4.9 Hz) while moving, plus one immediate `move` on every walk/idle flip, and
  `character_state {walk|idle|sit}` for the animation. Speed is ~800 world units/s
  (~164 units per tick). No direction, velocity or timestamp. The server rebroadcasts
  coalesced updates every ~300 ms and never corrected a position in the log.
- **Collision, speed and pathfinding run in the client** (the WASM engine). The server relays
  positions without validating them, so our client must do its own collision using the
  `collider` rectangles of the furniture catalog and wall objects.
- **Map:** there is no tile array. The office arrives as ~1,473 protobuf map objects over the
  socket (806 floors, 276 walls, 371 furniture, 20 meeting zones; 179 KB). Floors sit on a
  210.94-unit grid. The schema (`PbSetMapObject`) was recovered and a decoder exists in
  `analysis/scratch/decode_obj.py`.
- **Calls are zone-based, decided by the server:** the client works out which meeting zone it
  stands in and sends `set_meeting_zone`; the server answers with a `cluster` event holding the
  LiveKit token. No distance-based voice outside zones was seen (an 850-unit proximity circle
  exists in the client but is not confirmed to drive audio).
- **Media is stock LiveKit.** `livekit-client` with the token from the `cluster` event is
  enough: no custom signalling. Token lifetime 24 h, identity = Firebase uid.
- **Weight explains the complaint:** ~680 JS chunks (17.5 MB) up front, then an 11 MB Rust/WASM
  WebGL2 engine and 6 MB of LiveKit when entering the office.

## Hard parts and risks, ranked

1. **First sign-in from a non-browser client.** Refresh is PROVEN (see "Auth spike result"
   below). What remains is obtaining the first refresh token without our recorder. Plan: a
   one-time Tauri window on `app.sowork.com/login` (Google's callback only works on that origin),
   then read the refresh token from the page's IndexedDB (`firebaseLocalStorageDb`). Own Google
   OAuth client IDs will probably fail Firebase's audience check. Untested.
2. **Zone entry geometry.** Zone `x,y,size` versus avatar position did not reconcile (isometric
   transform or anchor unknown). Joining calls depends on it. Needs a dedicated recording.
3. **Version gates.** The server publishes `minimumClientVersion` (11.185.0 at recording time);
   we must send a plausible `clientVersion` and track it. Expect breakage on their releases.
4. **One identity, one connection.** A second gaia/LiveKit connection with the same identity
   kicks the first, so the official app and ours cannot both be online.
5. **Origin checks.** `/settings/regions` allows only `https://app.sowork.com`. Skip
   `prepareConnection`; sending the right `Origin` from the Rust side is planned anyway.
   Whether other endpoints enforce Origin/`source`/`version` headers is unknown.
6. **WebRTC in Tauri on Linux (WebKitGTK):** still the M0 spike; force VP8 and keep the Chrome
   `--app` fallback.
7. **Art:** avatars are layered sprite atlases whose file layout was not captured. A minimal
   client can use plain circles; faithful avatars need another recording.
8. **ToS and detection.** Unchanged from PLAN.md: reverse engineering and third-party clients
   are prohibited, termination is discretionary.

## What to drop in our client

React/MUI/MobX UI, the 11 MB engine, Intercom, Amplitude, Sentry, Meta pixel, Google Analytics,
GTM, Firestore (not needed for walking or calls; presence arrives on gaia), chime region lookup
(only yields `mediaRegion`, which is passed to gaia and does not affect LiveKit), sprites.

## Auth spike result (2026-10-06, run by the user)

Using the refresh token from the recording (Firebase `signInWithIdp` response), a plain Node
script with no browser headers:
- `POST securetoken.googleapis.com/v1/token` -> 200, new ID token, `expires_in=3600`. No `Origin`
  or `Referer` needed, so the web API key is not referrer-restricted.
- `POST api.sowork.com/api/v1/gaia/find` with `Authorization: Bearer`, `source: web`,
  `version: 11.202.0`, no `Origin` -> 200 with `room`, `roomDefinition`, `roomGroupId`.
So a native client can stay signed in indefinitely from a refresh token. Script:
`analysis/scratch/auth_spike.mjs` (prints no secrets). Not yet tested: opening the gaia socket
from a non-browser client, and whether the refresh token survives for weeks (Firebase refresh
tokens normally do until revoked or the password changes).

## Gaia spike result (2026-10-06, run by Claude with the user's permission)

`analysis/scratch/gaia_spike.mjs`, plain Node, zero dependencies (own msgpack encode/decode, Node's
built-in WebSocket), no browser: refresh token -> ID token -> `gaia/find` -> Socket.IO connect
with `clusterMode=none-connecting`, `joinType=force`, random `autoForceId`, `clientVersion=11.202.0`
and the CONNECT token -> server accepted. Within 12 s it received `init`, `cluster`, `presence`,
`server_version` (11.202.0, minimum client 11.185.0), `room` (mapId skyboxSunset01, 1 spawn) and all
1473 `b_set_obj` frames. So login-to-world works headless, with no Origin or browser headers.
Caveats: `joinType=force` kicks any other session of the same identity, so the script briefly
appeared online in the office; the spike never sent `move`.

## Revised step 3 order

1. **M0 spike:** (a) Tauri webview with `getUserMedia` and loopback WebRTC; (b) one-time login
   in a Tauri window on `app.sowork.com/login` and extraction of the refresh token from
   IndexedDB; (c) already proven in Node: refresh, `gaia/find` and the gaia socket with the `room` event
   and all map objects. Port that script to Rust or Tauri.
2. **M1 session:** token refresh, persist refresh token in the OS keychain.
3. **M2 world:** gaia connect (CONNECT packet with token, `room`, `b_set_obj` stream), render
   floors/walls/zones as flat shapes on Canvas 2D, other avatars as circles from `presence`
   and `s` batches.
4. **M3 walk:** WASD, collision against walls and colliders, `move` at ~5 Hz, `character_state`.
5. **M4 calls:** compute zone, `set_meeting_zone`, join the LiveKit room from the `cluster`
   event with `livekit-client`, mute/camera (`camera_on` gaia event), speaking indicator.
6. **M5 extras:** chat, screen share (metadata + `screenshare` data channel topic), status.

## Recordings that would close the biggest unknowns (about 10 minutes total)

Do each with a teammate online if possible, and write down wall-clock times:

1. Walk in straight lines N, E, S, W for 10 s each, and note the direction in the log (settles
   the world-to-screen mapping and key directions).
2. Watch a teammate walk past you (remote movement frames; none were captured).
3. Walk into a meeting zone and out, three times, from different sides (zone geometry).
4. Mute and unmute the mic; camera on/off; receive a teammate's video.
5. Proximity without zones: stand next to a teammate in the open floor for 30 s.
6. Reload the page and, separately, go offline 20 s (reconnect, `user_reconnected`).
7. Open the avatar editor and the map/furniture view so atlas and tile images load.

Rerun `node analysis/prepare.mjs recorder/logs/<run>` on each new run to get redacted working
files in `analysis/work/`.

## Housekeeping

- `recorder/logs/<run>/` is the only raw data (contains live tokens; git-ignored). The failed
  first run was deleted.
- `analysis/work/` is derived and token-redacted, but still contains user ids and names. Do not
  share it.
- `analysis/scratch/` holds the agents' decoder scripts (msgpack, protobuf, walk stats).
  `ws_msgpack.py` and `decode_obj.py` are the useful ones; the rest are throwaway.

## Meetings (zone and proximity), implemented in app/

- **Zone containment, solved by fitting** (`83` position samples from the recording): a zone is a
  rectangle centred on its `x,y` with half-extent about `1.1 x sizeX/sizeY` (98.8% of samples
  classified correctly; the naive `size/2` rectangle got 69%). The page computes the zone, sends
  `set_meeting_zone`, and the server answers with `cluster` carrying the LiveKit token.
  Only 3 of 20 zones were visited, so the 1.1 factor may need a small adjustment.
- **Proximity (hypothesis, untested):** the official client only uses `shouldBeConnectedTo` (users within
  850 units, available, outside zones) to draw "connecting" placeholders; there is no client request that
  starts a proximity call. So the server most likely groups them itself from `move` positions and
  `cluster_mode`, and announces the result through the same `cluster` event. Our client therefore uses one
  path for both: any `cluster` event with a token joins its LiveKit room, a null cluster leaves.
- Calls join muted and camera off; mic/camera need an explicit click (privacy: walking past someone must not
  open the microphone).

- Presence and muting others: see [presence-and-mute.md](presence-and-mute.md).
