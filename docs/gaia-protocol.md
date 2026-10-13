# SoWork "Gaia" realtime protocol (client side)

Source: one recorded session (about 4 min, app client `11.202.0`, server `11.202.0`, min client `11.185.0`).
Evidence = frame timestamps (seconds since recording start, `+t`) from `analysis/work/ws/05-api.sowork.com.txt`
(socket A, 1520 frames) and `07-api.sowork.com.txt` (socket B, 311 frames), decoded raw from
`output/raw-logs/2026-10-06T12-48-44-206Z/events.jsonl`, plus the JS bundles. No secrets are reproduced here:
`<JWT>`, `<me>`, `<userN>`, `<zone>` are placeholders.

Legend: **[F]** = fact seen in traffic or code, **[G]** = guess/inference, **[?]** = unknown.

Helper scripts (all under `analysis/scratch/`): `mp_schema.py` (minimal msgpack decoder + per-event schema census,
output in `schema_census.txt`), `pb_decode.py` (protobuf wire decoder + `PbMapObject` decoder for `b_set_obj`),
`walk_stats.py` (rates/speeds of `move`), `zone_check.py` (zone rectangles vs avatar position).

---------------------------------------------------------------------------------------------------

## 1. Connection

### 1.1 Endpoint and query

Server URL comes from the REST call `POST /api/v1/gaia/find` (and `/gaia/rooms`): `room.server.url = https://api.sowork.com/gaia/g5`,
`room.serverId = g5`. The client appends `/v2` (path `/gaia/g5/v2`, Socket.IO `path` option) and connects with
`socket.io-client`, `transports: ["websocket","polling"]`, `forceNew: true`, `parser = socket.io-msgpack-parser`
(`main-DD7sSLvE.js`, class `ul` = `GaiaRoomConnection`, `connectTo()` near offset 2661000; `getSocketQuery()` near 2665700).

```
wss://api.sowork.com/gaia/g5/v2/?roomId=<roomId>&mediaRegion=eu-central-1&password=&clientVersion=11.202.0
   &autoForceId=<nanoid21>&clusterMode=none-connecting&meetingZoneId=&joinType=force&EIO=4&transport=websocket     (socket A)
wss://api.sowork.com/gaia/g5/v2/?roomId=<roomId>&mediaRegion=eu-central-1&password=&clientVersion=11.202.0
   &autoForceId=<same>&clusterMode=cluster&meetingZoneId=<zone>&x=3747.45263671875&y=3563.123046875&EIO=4&transport=websocket (socket B)
```

| param | meaning | evidence |
|---|---|---|
| `roomId` | gaia room instance id (from `gaia/find` `room.id`, 20 char Firestore-style id) | [F] code + URL |
| `mediaRegion` | AWS region picked by client for media (`awsRegion.mediaRegion`), here `eu-central-1` | [F] |
| `password` | room password (empty) | [F] |
| `clientVersion` | app build, constant `kk` = `11.202.0`; server answers `server_version` and may reject versions below `minimumClientVersion` | [F] |
| `autoForceId` | random 21 char id generated once per page load (`ug()`); identical on both sockets | [F] values equal in A and B; [G] used by server to identify "this tab" for the `joinType=force` takeover |
| `clusterMode` | the user's current voice/video availability mode (see 3.8) at connect time | [F] |
| `meetingZoneId` | id of the meeting zone the avatar currently stands in, empty if none | [F] |
| `joinType=force` | only on the first connect (`connectTo(..., force=true)`); absent on the reconnect | [F]; [G] "kick any other session of this user" |
| `x`, `y` | current avatar position, only present if the client already has a position (reconnect). Sent as stringified floats | [F] code (`getSocketQuery`) + B URL |
| `EIO=4&transport=websocket` | Engine.IO v4, direct websocket (no polling handshake; `upgrades: []`) | [F] |

On reconnect the client refreshes `socket.io.opts.query` (`refreshSocketOptions()`) so a Socket.IO automatic
reconnect always carries the latest clusterMode / meetingZoneId / x / y; it also does this whenever `clusterMode` or `meetingZoneId` changes
while connected. `joinType` is only set by the explicit `connectTo`.

### 1.2 Engine.IO / Socket.IO framing

All frames after the first are **binary** WebSocket frames holding one msgpack-encoded Socket.IO packet
(`socket.io-msgpack-parser`, notepack flavour). Text frames are Engine.IO control only.

| layer | packet | direction | example |
|---|---|---|---|
| EIO open | text `0{...}` | IN, first frame | `0{"sid":"<eio-sid>","upgrades":[],"pingInterval":35000,"pingTimeout":30000,"maxPayload":5000000}` |
| EIO ping | text `2` | IN, every 35 s | A: +111.318; B: +166.390, +201.520, +236.639 |
| EIO pong | text `3` | OUT, immediately (same ms) | |
| SIO CONNECT (type 0) | msgpack `{type:0, data:{token:"<JWT>"}, nsp:"/"}` | OUT, right after the EIO open (A: +75.548, same ms as open; B: +131.412) | |
| SIO CONNECT ack | msgpack `{type:0, data:{sid:"<sio-sid>", pid:undefined}, nsp:"/"}` | IN ~100-150 ms later | |
| SIO EVENT (type 2) | `{type:2, data:[eventName, payload], nsp:"/", options:{compress:true}, id?}` | both | |
| SIO ACK (type 3) | `{type:3, id:n, data:[{serverTime, clientTime}], nsp:"/"}` | IN | only for `sync_time` |

Msgpack details [F, from raw bytes]:
- Encoding is plain msgpack maps with string keys. Client packets include the extra key `options:{compress:true}` inside the packet map (a quirk of this parser; harmless, you can send it or omit it, [G] omitting is fine).
- `undefined` is encoded as **fixext1 type 0, payload 0x00** (`d4 00 00`). Our prep decoder printed this as `{"$ext":1}` (1 = payload size, not ext type). Therefore `"token":{"$ext":1}` in an empty `cluster` event means `token: undefined`, `"target":{"$ext":1}` in `characterState` means no target, `mapBounds:{"$ext":1}` means undefined, `pid` undefined (no connection-state recovery).
- Floats are float32 or float64 as the JS encoder chose; coordinates arrive with 4 decimals (e.g. `3747.4526`), integers if exactly integral (e.g. `x:3747`).
- Event arguments are exactly one object (or one binary for `b_set_obj`).
- No packet ids except ack-requesting `sync_time` (`id` counter 0,1,2,... per page; ack also carries the same `id`).

### 1.3 Authentication

[F] The CONNECT `data.token` is the **Firebase ID token** (RS256 JWT, issuer `securetoken.google.com/sage-learning-prod`, claims `aud, auth_time, email, email_verified, exp, firebase{identities,sign_in_provider}, iat, iss, name, picture, sub, user_id`, lifetime 3600 s).
I verified byte-for-byte that the token in both CONNECT packets is identical to the `Authorization: Bearer` token used for `https://api.sowork.com/api/v1/*` REST calls in the same minute.
Code: `connectTo(async () => await vme())` with `auth: cb => getToken().then(t => cb({token:t}))` (socket.io `auth` option is a function, so the token is re-fetched on every (re)connect). `vme` = `tC()` in `httpClientFactory-CXSNSIW-.js` = `auth.currentUser.getIdTokenResult(false)`; reused if it has more than a threshold of validity left, else `getIdTokenResult(true)` (forced refresh).
Before connecting, the app logged in with Google SSO -> `signInWithIdp`, later `signInWithCustomToken` at +73.856 (REST, not covered here); the ID token used for gaia is whatever Firebase currently holds.
There is no separate gaia cookie/session; room access is gated by the server verifying the JWT + group membership (the user object comes back with `memberRole`).
[?] What the server does on an invalid/expired token (probably `connect_error`/`error` event with `code`; client reads `error.code` into `errorCode`).

### 1.4 Ping/pong and timing

`pingInterval=35000`, `pingTimeout=30000` (server sends `2`, client must answer `3`). Observed pings every 35.1 s. Client also sends application-level `sync_time` on connect and every 60 s (`i6e=6e4`, `setInterval` in `connectTo`), see 3.1.

### 1.5 Socket A vs socket B (why two connections)

| | A (`05`) | B (`07`) |
|---|---|---|
| open | +75.220 | +130.837 (EIO open frame at +131.411) |
| query | `clusterMode=none-connecting`, `meetingZoneId=` empty, `joinType=force`, no x/y | `clusterMode=cluster`, `meetingZoneId=<zone>`, `x,y` = last known position (3747.45, 3563.12), no joinType |
| first client event | `sync_time` | `user_reconnected {clusterMode, zoneId, shouldUpdate}` then `sync_time` x2 |
| end | last frame +128.646, WS close recorded +131.101 | until end of recording |

[F] B is **not a new `connectTo`**; it is a Socket.IO manager *reconnect* of the same client object: the code registers `socket.io.on("reconnect", ...)` which emits `user_reconnected {clusterMode: this.clusterMode, zoneId: this.meetingZoneId, shouldUpdate: this.shouldUpdate}` and re-runs `syncTimeOffset()`. B emits exactly that event first, and `autoForceId` is unchanged. The refreshed query is why B shows `clusterMode=cluster`, the zone and x/y (the state when A died; `cluster_mode:"cluster"` had been sent at +84.636).
Note the ordering: the recorder logged B's `ws.open` at +130.837, 283 ms *before* A's `ws.close` (+131.120), so A was already being torn down (client-side close or network drop) when the manager scheduled the reconnect.
[?] The trigger: no `disconnect` reason is recorded. Candidates: a network blip, or the page being throttled (no application frames from the server on A between +128.65 and +131.1 but pings are only every 35 s, so a ping timeout is excluded). Next ping would have been due at +146.3, so it was not a timeout. [G] network/proxy/browser-initiated close. Treat reconnect as a normal event your client must handle: re-fetch token (auth callback), reconnect with updated query, emit `user_reconnected`, re-send state.
After a reconnect the server re-sends the full handshake: `init`, `cluster`, `presence`, `server_version`, `room` (B: +133.231 .. +133.389), and the client re-emits its own state (`user_update`, `character_state`, `speech`, `typing`, `camera_on`, `set_meeting_zone`, `info_update`, `move`) as a burst at +133.389 (`mergeNewRoomState` -> `handleRoomUserUpdate`).

### 1.6 Connection sequence (A)

```
75.220 WS open (A)                         75.548 IN  EIO open {sid, pingInterval:35000, ...}
75.548 OUT SIO CONNECT {token:<JWT>}       75.694 IN  CONNECT ack {sid}
75.694 OUT sync_time {clientTime} id=0     (no ack was ever received for id 0, nor for B's id 1 and 2 -> acks only answered after "init")
78.012 IN  init                            78.012 OUT should_update {v:true,t:0,lastObjectId:""}   OUT active_app {activeApp:null}
78.013 IN  cluster {clusterId:null,conversationId:null,users:[],token:undefined}
78.013 IN  presence {userId:<me>, presence:{...}}          78.013 IN server_version {serverVersion, minimumClientVersion}
78.153 IN  room {...full room snapshot...}
78.252-79.394 IN 1473x b_set_obj (map objects, see 3.9)
79.417 IN  s [[10 clusterMode],[9 meetingZone],[3 join <me>]]      79.418 IN s [[6 speaking]]
79.860 OUT move {x,y} (from the room snapshot position)   79.860 OUT character_state idle   79.914 OUT set_meeting_zone {zoneId:<zone>}
80.050 IN  s [[1 move <me>],[2 state <me>],[9 zone <me>]]   (server echo of own state)
```
Server sends `init` ~2.3 s after CONNECT in A (+78.0) and ~1.7 s in B (+133.2) (slow server-side join).

---------------------------------------------------------------------------------------------------

## 2. Event catalog

Counts: A/B = socket A / socket B. Schemas are from a full census of all frames (`analysis/scratch/schema_census.txt`).
`s` is a batch container (2.3), shown separately as `s/N`.

### 2.1 Client -> server (OUT)

| event | count A/B | payload | meaning / when |
|---|---|---|---|
| `sync_time` | 1/4 (+B ids 1,2 unanswered) | `{clientTime:int ms}` + ack request (`id`) | clock sync. Ack = `{serverTime, clientTime}`. Observed B id 3: RTT 130 ms, serverTime - clientTime = 64 ms (about RTT/2, so clocks agree). Client sends it on connect, on reconnect and every 60 s (+135.184, +195.184); ignores it if RTT > 5 s. `room.timeOffset` is computed from it. |
| `should_update` | 1/3 | `{v:true, t:<ms epoch of newest local map object>, lastObjectId:<id>}` or `{v:false}` | subscribes/unsubscribes to map-object (`b_set_obj`) pushes. A: `t:0,lastObjectId:""` => server pushes ALL 1473 objects. B: `t:1790014784100,lastObjectId:"<id>"` => nothing newer, no objects pushed. `v:false` when the game engine FPS mode is `pause` (B +201.254, probably tab/window hidden or the share picker; [G]); `v:true` again at +213.781 answered by a fresh `room` snapshot at +214.254. |
| `active_app` | 1/1 | `{activeApp:null}` | shared "active app" presence (null = none). Debounced 30 s when changed. |
| `move` | 1/83 | `{x:float, y:float}` (+ `following:<userId>` when following someone) | THE walk message, see section 3. |
| `character_state` | 1/24 | `{characterState:{state:"idle"\|"walk"\|"sit"(\|"jump1"), target?:<objectId>}}` | animation/behaviour state. `target` is only set for `sit` (id of the chair map object), undefined otherwise. Seen OUT: idle x13, walk x11 (B). |
| `set_meeting_zone` | 1/13 | `{zoneId:<mapObjectId> \| null}` | the avatar entered/left a "meeting" zone rectangle. Triggers server clustering, see 3.7. |
| `cluster_mode` | 1/0 | `{mode:"cluster"\|"none-away"\|"none-connecting"\|"none-yoga"\|"none-private-meeting"}` | availability for voice/video. A: `cluster` at +84.636 (400 ms after `user_prompt welcome`), flips from `none-connecting`. Rule (`pPi` in main bundle): busy/away/yoga/private conversation/connecting => a `none-*` mode, else `cluster`. |
| `speech` | 0/20 | `{isSpeaking:bool}` | local voice activity (VAD from LiveKit); fires about every 1-5 s while talking. |
| `typing` | 0/2 | `{isTyping:bool}` | chat typing indicator (only in the post-reconnect burst here). |
| `camera_on` | 0/4 | `{cameraOn:bool}` | camera toggled (B: true +171.502, false +174.874). |
| `user_update` | 0/2 | `{characterConfig:{version:5,bodyType:"characterA",slots:{hair,shoes,eyebrows,pants,eyes,shirt,body,mouth,headAccessory,hat,beard,...:"characterA_<slot>_<item>+<variant>"}}}` | avatar appearance; only re-sent in the post-reconnect burst. |
| `info_update` | 0/2 | `{role,affiliation,name}` (all strings) | profile text; post-reconnect burst. |
| `user_prompt` | 1/0 | `{userId:<me>, type:"welcome"}` | A +84.592, when the welcome prompt fires after entering. Other types in code: `extend_visitor_pass {visitorUserId}`, `screenshare_acknowledge {userId, recipientUserIds}`. |
| `user_reconnected` | 0/1 | `{clusterMode, zoneId, shouldUpdate}` | first event after a Socket.IO reconnect (B +131.517). |

Present in the bundle but **not observed** (OUT): `spawn {characterConfig,x,y}` (sent instead of `move` when a *new* user joins the local room model), `emote {emoteId,recipientUserId,senderUserId,extras}`, `b_set_obj` (binary `PbSetMapObject`; editing the map; deletion = message without `data`), `npc_spawn/npc_update/npc_despawn`, `ring_user {targetUserId}`, `kick`, `follow_request {targetUserId,direction}`, `follow_request_response {requesterUserId,accepted,direction}`, `follow_stopped {targetUserId,reason}`, `game_join/game_start/game_leave/game_action`, `dart_upsert/dart_hit`.

### 2.2 Server -> client (IN), top-level events

| event | count A/B | payload | meaning |
|---|---|---|---|
| `init` | 1/1 | none | handshake complete; client replies `should_update` + `active_app`. |
| `room` | 1/2 | big object, see 2.4 | full room snapshot (users, spawn areas, map ids). Sent after init and again after `should_update{v:true}` following a pause (B +214.254). Client merges it as a diff (`mergeNewRoomState`). |
| `cluster` | 2/12 | `{clusterId:str\|null, conversationId:str\|null, users:[{id,name,config:{characterConfig}}], token:{provider:"livekit",roomId,token:<JWT>,apiHost}\|undefined}` | voice/video group assignment, see 3.7. |
| `presence` | 1/1 | `{userId, presence:{lastOnline,inPrivateConversation,inCalendarMeeting,isYoga,isOnline,isIdle,status:"online"\|"away"..,textStatus,mobile}}` | own presence at connect (only own seen). |
| `server_version` | 1/1 | `{serverVersion:"11.202.0", minimumClientVersion:"11.185.0"}` | |
| `s` | 26/115 | `[[code, payload], ...]` | batched user events, 2.3. |
| `update` | 0/4 | `{userId, characterConfig}` | a user's avatar config (sent for each cluster member after a `cluster` event with users, +154.4, +155.0, +188.9, +189.0). |
| `update_info` | 0/4 | `{userId, name, role, affiliation, uploadedPhotoURL}` | profile of cluster members, same moments. |
| `user_disconnect` | 1/0 | `{userId}` | user's socket dropped (A +127.725). Client marks `connected:false`. |
| `leave` | 0/1 | `{roomId,userId}` | user removed from the room (B +180.656), 53 s after that same user's `user_disconnect`. [G] grace period before removal. |
| `b_set_obj` | 1473/0 | binary `PbSetMapObject` | map object upsert/delete, 3.9. |

Handlers present but not observed (IN): `user_reconnect {userId}` (connected:true), `member_role {userId,memberRole}`, `user_prompt`, `notify`, `room_object_update {roomId,roomObjectsDiff}`, `ring_received`, `follow_request_received`, `follow_request_response_received`, `follow_stopped_received`, `dart_upsert`, `dart_hit`, `game_upsert`, `game_action`, `game_delete`, `error` (with `code`), `connect_error`.

### 2.3 The `s` batch event and its codes

`["s", [[code, payload], [code, payload], ...]]`, handled by `handleBatchEvent` (main bundle). Counts are totals over both sockets, only event types seen.

| code | name | count | payload | client behaviour |
|---|---|---|---|---|
| 1 | move | 53 | `{userId, x, y}` | `handleMoveEvent`: **ignored if userId==me**; otherwise sets position with `characterMoveMode = Lerp` (interpolate). |
| 2 | character_state | 20 | `{userId, characterState:{state, target?}}` | ignored if me; else applied (animation). `sit` carries `target:<objectId>`. |
| 3 | join | 2 | `{roomId, user:{full user object, 2.4}}` | user (re)entered room; for me: position/zone/cluster ignored (client is authoritative on those), rest merged. |
| 4 | emote | 0 | `{senderUserId, ...}` | `handleEmoteEvent` (not observed) |
| 5 | spawn | 0 | `{userId,x,y,characterConfig,characterConfigOverride,cameraOn}` | `handleSpawnEvent`, sets position/config. Not observed. |
| 6 | speaking | 107 | `{userId, isSpeaking}` | |
| 7 | typing | 3 | `{userId, isTyping}` | |
| 9 | meeting_zone | 17 | `{userId, meetingZoneId:str\|null}` | |
| 10 | cluster_mode | 3 | `{userId, mode}` | |
| 11,12,13 | pet join/update/leave | 0 | `{ownerId,...}` | NPC pets. Not observed. |
| 14 | camera_on | 5 | `{userId, cameraOn}` | |
| 15 | active_app | 0 | `{userId, activeApp}` | |

Server batching (B, from IN frame timing): all frames containing own-user `s/1` arrive spaced 0.27-0.32 s (mean 0.30 s, ~3.3 Hz) while moving, so the server flushes `s` about every 300 ms; multiple codes can sit in one `s` array (e.g. +133.727: `[1,2,7,14,...]` for me; +178.853: one array with move+state+speaking+typing+camera for another user = a full re-announce of that user).
Unit events for self: the server **echoes the sender's own events back** to the sender (own `s/1`, `s/2`, `s/6`, `s/9`, `s/10`), which the client ignores for move/state (`shouldProcessEvent(userId) = userId !== me`).

### 2.4 Object shapes

`room` event keys: `id, name, serverId, roomDefinitionId, roomDefinitionName, roomDefinitionAlias, roomType:"room", roomObjects:{}, mapDefinitionId, mapId (skybox name e.g. "skyboxSunset01"), mapBounds (undefined here), spawns:[{x,y,objectId}], spawnAreas:{<zoneObjectId>:{x,y,tags}}, customizable, customizableStructures, projection:"isometric", ownerKey:"group_<groupId>", users:{<userId>:User}, npcs:{}, games:{}, guestAreas:{}, channels (undefined)`.
Size about 6 KB with 8 users. `spawnAreas` has one entry per meeting/spawn zone (20 in this office): the center-ish point used to place an avatar when "spawning at a zone".

User object (`room.users[id]`, and `s/3.user`):
```
{ id, name, x, y, characterConfig:{version:5, bodyType:"characterA", slots:{<slot>:"characterA_<slot>_<item>+<variant>"}},
  characterConfigOverride:null, role, affiliation, memberRole:"owner"|"contributor"|...,
  characterState:{state:"idle"|"walk"|"sit", target?:<objectId>} | null,
  isSpeaking, isTyping, cameraOn, activeApp:null,
  clusterMode:"cluster"|"none-away"|"none-connecting"|..., meetingZoneId:<id>|null,
  presence:{lastOnline,inPrivateConversation,inCalendarMeeting,isYoga,isOnline,isIdle,status,textStatus,mobile},
  uploadedPhotoURL:null|str }
```
[F] The `room.users` snapshot lists 7-8 users including people on mobile/away (positions are their last positions, e.g. a user parked at x=349,y=2407).

Examples (redacted):
```
IN  s  [[1,{"userId":"<me>","x":3864.2681,"y":3446.3096}]]                       // own echo, +141.851
IN  s  [[2,{"userId":"<me>","characterState":{"state":"walk","target":undefined}}]]
IN  s  [[9,{"userId":"<me>","meetingZoneId":"<zone>"}]]
IN  s  [[6,{"userId":"<user2>","isSpeaking":true}]]
OUT character_state {"characterState":{"state":"sit","target":"<chairObjectId>"}}   // shape only; "sit" seen IN for other users
IN  presence {"userId":"<me>","presence":{"lastOnline":null,"inPrivateConversation":false,"inCalendarMeeting":null,"isYoga":false,"isOnline":true,"isIdle":false,"status":"online","textStatus":null,"mobile":false}}
```

---------------------------------------------------------------------------------------------------

## 3. Details of the important flows

### 3.1 Time sync
`sync_time` with ack. `room.timeOffset = f(clientSend, serverTime, clientRecv)` (`z3i`), used by the app for animation sync. Not needed for walking.

### 3.2 THE WALK MECHANIC

**Message:** `OUT move {x, y}` — absolute position, float32-ish, 4 decimals, nothing else (no direction, velocity, timestamp, speed). Optional `following:<userId>` is added while the user follows someone (code `handleRoomUserUpdate`). Animation is a separate message: `OUT character_state {characterState:{state:"walk"|"idle"|...}}`.

**Coordinate system [F/G]:** Same "world" space as the map objects (`b_set_obj` x/y, floor tiles are on a 210.94-unit grid; avatar positions seen -105..5300, objects -3193..7597 / -2871..6574). Units: world pixels (float). The projection is `isometric`; screen arrows map to world diagonals, so one arrow key changes x and y together: all observed free walking steps are exactly diagonal `(±d, ±d)` (single key) or axis-aligned `(0,±d)` (likely two keys / path-following), with `d`/vector length constant.
Code evidence: the engine's `Mi(dir)` maps 8 directions index 0..7 to `[0,-1],[1,-1],[1,0],[1,1],[0,1],[-1,1],[-1,0],[-1,-1]` (N..NW); the position that goes on the wire is `localCharacterIsoPosition` from the engine event `LocalCharacterChange` (`character:update` {position, characterState, direction}) in `isInputActionMovement-DiROaRkY.js` (`ho()` translates wasm events) and `main-DD7sSLvE.js` (`d2` class near 2651428, `handleLocalCharacterUpdate` ~2423437 -> `room.handleUserUpdate` -> `room:userUpdate` -> `ul.handleRoomUserUpdate` -> `socket.emit("move", {x,y})`).
Facing direction (`direction`) exists in the engine event but is **not sent** to the server.

**Movement itself runs in a Rust/WASM game engine** (`hephaestus-Km_jPPAu.js` = wasm-bindgen glue; wasm blob not analysed). JS only configures it (`config:setMovementOptions {movementSpeed, pathfindingFarThreshold, pathfindingFarMultiplier}`, `setPhysicsOptions {enableAssetCollisions, enableLocalCharacterCollisions, enableLocalCharacterFurnitureCollisions}`; values live inside wasm defaults and a per-character `speed`/`collider` from the character-config validator). So **speed and collision are client-side**. The server never rejected or corrected a position in this recording.

**Send rate [F]** (B, +141.4 .. +143.8, a single held key):
```
141.435 OUT character_state walk      141.435 OUT move 3748.0752,3562.5015    (first step ~19 units from rest)
141.641 OUT move 3864.2681,3446.3096  (+116.2,-116.2 in 0.206 s)
141.846 OUT move 3980.4048,3330.1729  (+116.1,-116.1 in 0.205 s)
142.050 OUT move 4095.8618,3214.7158
142.269 OUT move 4212.3291,3090.3623
142.511 OUT move 4327.8984,2974.7954
 ... same cadence ...
143.720 OUT move 5034.1602,2268.5464
143.820 OUT character_state idle      143.820 OUT move 5072.2324,2230.4775    (final partial step)
```
- Cadence while moving: **one `move` every 205 ms (mean 0.206 s, sd 0.015, n=46) = ~4.9 Hz.** Plus an extra immediate `move` on every state transition (start of walk, stop): these arrive 15-70 ms apart (e.g. +151.419/+151.625/+151.661/+151.696 when tapping keys).
- **Speed:** step length per 205 ms tick = 164.3 +/- 3 units (diag 116.2 per axis) => **~800 world units/s** (hypot(dx,dy)/dt = 797-805 for full ticks). Axis-only movement (+214.25..: dx=0, dy=+163..169 per 0.205 s) has the same 800 u/s, so speed is direction independent (diagonals are normalized).
- Faster segments (1300-1970 u/s, B +214.3-+214.5, +220-+222 e.g. steps of 160-250 units in ~0.1 s) are click-to-move/pathfinding or the `farDistanceSpeedMultiplier` boost, [G] (the engine has `pathfindingFarMultiplier`). Not fully characterised.
- **Absolute, not delta.** Every message carries the full position.
- First `move` after connecting is sent immediately after `room` with the position the server stored (A +79.860: 3747.4526,3563.123, identical to the `room`/join user position).
- `character_state` `walk` -> `idle` flips send exactly when the engine's animation state changes; they are sent on the same millisecond as the move that triggered them.

**What the server does [F]:** it coalesces and rebroadcasts. In B every own `move` echoed back in an `s/1` is bit-identical to one previously sent (51 of 51 echoes matched a sent value; no correction, no clamping). Echoes show the **latest** position at flush time, so intermediate positions are dropped when 2 `move`s fall in the same ~300 ms flush window (e.g. out 3864.27@141.641, 3980.40@141.846 -> in 3864.27@141.851, 3980.40@142.135; out 4095.86@142.050 never echoed, out 4212.33@142.269 -> in@142.441). Server->client broadcast rate for a moving user is therefore **~3.3 Hz** (one `s/1` per ~300 ms), latency 0.2-0.3 s.
`s/2` (state) is broadcast only on change.

**Remote avatars:** the recording contains no continuous movement of *another* user (the only `s/1` for another user is one re-announce at +178.853). From code: other users' `s/1` => `handleUserUpdate({position, characterMoveMode: Lerp})`; the engine interpolates between received points (Lerp mode, [G] roughly constant-speed lerp to the new point). Full-state `join` (`s/3`) and `room` snapshots are applied as diffs: position change <= 1000 units (`t6e = 1e6` squared) uses Lerp, > 1000 uses **Teleport**; a user not previously known is created with Teleport.

**Teleports/spawns [F/G]:**
- Initial position: the server remembers the last position of the user and sends it in `room.users[me]`/`s/3`; the client then `move`s to it. No client spawn event was sent (`spawn` is only emitted for a brand new local user, `room:userJoin` with characterConfig).
- Reconnect: client puts `x`,`y` in the query. The server's own join echo (B +133.457) shows the position as integers rounded (3747, 3563) while the query had 3747.4526 (so the server stores whatever the query said, rounded or truncated, [G]), and the client moved on immediately (first B `move` at +133.333 = 3767.53,3543.04 which is one tick (+20,-20) from the query position).
- Room snapshot with `spawns:[{x,y,objectId}]` and `spawnAreas:{<zoneId>:{x,y,tags}}` = candidate spawn points (1 `spawn` interaction object in the room, 20 zone areas). Client chooses (`V3i`: atMyDesk / atUserId / atZoneId / spawnId / default) and sets its own position locally with `characterMoveMode: Teleport`, then the normal `move` goes out.
- [?] No teleport via click or portal observed (portal zones in code: `zone:overlapStart` of a `portal` zone => `resolveRoomZoneInit` => client calls `initialize` for another room, i.e. a new connection).

**Collisions [F code, G data]:** client-side in the WASM engine (asset, remote character and furniture colliders, configurable, `ccd_enabled`). The server accepts any `x,y`. Data hints: after +150.29 steps (-117,-117) x3 then (-44.7,-44.7), (+18.9,-18.9), i.e. the avatar was stopped by something and bounced (character_state idle with tiny steps). At +214.3-+214.5 and +239.6-+240 sequences like (0,+163) and (-2.2,-162) show the avatar sliding along a wall (one axis zeroed). **A custom client must replicate (or deliberately skip) collision itself; nothing server-side stops walking through walls** (unverified: server may validate in ways not triggered here).

**Throttling summary for a re-implementation [G, safe values]:** send `character_state walk` + `move` when starting; then `move` every 200 ms with absolute (x,y) advanced by 160 units per tick in the walk direction (800 u/s); on stop send final `move` + `character_state idle`. Echoes of own state come back as `s/1`, `s/2` and must be ignored.

### 3.3 Position data example (B +137.65..): single key taps
```
137.652 OUT character_state walk   OUT move 3748.6787,3561.8979       (-19.7,+19.7 from rest at ~3767.5,3543.0)
137.719 OUT character_state idle   OUT move 3729.2188,3581.3579       (another -19.5,+19.5; key released after 67 ms)
```
Two quick ticks of about 27 units each (= one engine frame at ~30 Hz*800) on a tap.

### 3.4 `set_meeting_zone` and zones
`OUT set_meeting_zone {zoneId}` is sent by the client when the engine reports `zone:overlapStart/End` (`IntersectionEvent` of type `MapmakerObject`, zone with `interaction.type == "meeting"`), see `main-DD7sSLvE.js` near 2395434 (`handleZoneEnter/Leave`, orphan watcher that logs a stuck zone after 10 s) and the `connection.meetingZoneId` setter (~2656145: updates local user, refreshes socket query).
Server broadcasts it as `s/9 {userId, meetingZoneId}` to everyone (including the sender).
Walking through zones produced quick toggles: +142.251 null, +142.511 `<zoneB>`, +143.534 null; +150.633 `<zoneB>`, +150.960 null; each followed by a `cluster` event after 100-300 ms.
Zone ids are map-object ids (Zone type, `assetId:"_zone"`, `interaction:{type:"meeting", data:{meeting:{name:"Mesa"|"Engenharia"|"Town Hall"|"Stela"...}}}`); 20 meeting zones in this office (see 3.9). Zone rectangles are in `b_set_obj` (`x,y,sizeX,sizeY`).
[?] Geometry of zone vs avatar position: I could not reproduce the engine's overlap with a simple center-anchored rectangle in the avatar coordinate space (`zone_check.py`: avatar positions at zone enter fall at dx/dy offsets from the zone x,y that are inconsistent between zones of the same size, e.g. zone `XGdh6` entered at d=(337,388), (351,452), (134,235) with size 283x283; `mohyb` entered at d=(-134,63) at spawn and (-10,-421)). The engine probably applies the isometric transform and an anchor/pivot to zone objects. A custom client should (a) get the engine geometry right or (b) simply send `set_meeting_zone` for the zone it decides to join (server trusts it: [G] untested).

### 3.5 Presence, status
`presence` is only seen for self at connect; others' presence is inside `room.users[*].presence` and `s/3.user.presence`. [?] how presence changes are pushed (the handler `d.on("presence", ...)` exists, `{userId, presence}`); no changes occurred in the recording.
`clusterMode` per user is part of the user object and is pushed via `s/10`.

### 3.6 Join/leave of other users
`s/3 join` (full user), `user_disconnect` (connection lost, shown greyed, client sets `connected:false`), `leave` (removed from room). Our own join is announced to us as `s/3` right after `room` (A +79.417).

### 3.7 Proximity / voice-video grouping (gaia side)

[F] In this recording **audio/video grouping is zone based and decided by the server**: the client reports `set_meeting_zone`; the server responds with a `cluster` event.

```
142.251 OUT set_meeting_zone null           142.412 IN cluster {clusterId:null, conversationId:null, users:[], token:undefined}
142.511 OUT set_meeting_zone <zoneB>        142.789 IN cluster {clusterId:null, ...users:[]}        (nobody else in zoneB: no group)
152.132 OUT set_meeting_zone <zoneC>        153.011 IN cluster {clusterId:"<clusterId>", conversationId:"<convId>",
                                                      users:[{id:"<user2>",name,config},{id:"<me>",name,config}],
                                                      token:{provider:"livekit", roomId:"RM_xxxxxxxxxxxx", token:"<JWT>", apiHost:"<project>.livekit.cloud"}}
153.171 client opens wss://<apiHost>/rtc/v1?access_token=<JWT>... (LiveKit signalling; +160 ms after the cluster event) ; pc.new at 153.130
154.4  IN update_info/update for each cluster user
179.881 OUT set_meeting_zone null           180.246 IN cluster {clusterId:null,...,token:undefined}     180.268 pc.close (LiveKit torn down)
187.173 OUT set_meeting_zone <zoneC>        187.968 IN cluster {clusterId:"<same clusterId>", conversationId:"<new convId>", users:[...], token:{... new roomId, new token}}
188.025 pc.new  ... 214.569 OUT set_meeting_zone null  214.912 IN cluster null    214.931 pc.close
```
Facts [F]:
- `cluster` is a full replacement state. `clusterId` stayed `U47j...` for the same group across two visits; `conversationId` and LiveKit `roomId`+`token` changed on each visit (new LiveKit room per conversation).
- `token = {provider:"livekit", roomId, token:<JWT, a LiveKit access token>, apiHost}`; the client passes it as `publicConversationSource.session = {conversationId, cachedToken: token}` and connects LiveKit with it (`cluster` handler in `connectTo`). When `clusterId&&conversationId` are both falsy the session is cleared => LiveKit disconnect. `users` lists the members of the cluster including me.
- Delay set_meeting_zone -> cluster: 0.16-0.28 s if no group results, 0.88 s (+153.011) and 0.80 s (+187.968) when a LiveKit room had to be created/joined.
- With `clusterMode != "cluster"` (e.g. `none-away`, `none-connecting`) the server does not cluster the user (`userIsAvailableToConnect` requires `clusterMode==="cluster"` and not in a private conversation, client side check on `room.users[*]`).
- **Distance-based proximity**: the engine also emits `character:proximity:near/far {userId}` (`CharacterProximity` intersection, proximity circle radius 850 world units when `showProximityCircle`, code `proximity_indicator_radius`), kept in `nearbyUserIds` and used for NPC/pet interaction and `shouldBeConnectedTo()` (users within the circle that `userShouldConnect`: `clusterMode=="cluster"`, connected, not in private convo, **and not inside a meeting zone**). In this recording there was no `cluster` event with users that was not preceded by `set_meeting_zone`, and no separate gaia message reports proximity, so [G] for non-zone ("open floor") proximity the client computes nearness itself (wasm) and the conversation join goes through a REST/other channel (`/api/v1/conversation/*`, not covered here) rather than gaia. [?] Needs a recording where two avatars stand next to each other outside any zone.
- The server decides membership of `users` (it knows each user's `meetingZoneId` and `clusterMode`). The client never sends distances.

### 3.8 clusterMode values
`cluster` (available), `none-connecting` (connecting to room), `none-away` (busy/away), `none-yoga`, `none-private-meeting`. Query param + `cluster_mode` event + user field + `s/10` broadcast.

### 3.9 `b_set_obj` binary blobs (map objects)

Each `b_set_obj` event carries **one msgpack `bin`** = protobuf `PbSetMapObject` (`@bufbuild/protobuf` generated code in `httpClientFactory-CXSNSIW-.js`, offset ~279413; also used OUT for edits):

```
message PbSetMapObject { string id = 1;  PbMapObject data = 2; /* absent => delete */ }
message PbMapObject {
  PbMapObjectType type = 1;   // 0 Zone, 1 Asset, 2 Floor, 3 Wall
  float x = 2;  float y = 3;           // world units (same space as avatars)
  string assetId = 4;                  // "_zone" for zones, else asset id
  PbOrientation orientation = 5;       // 0 N,1 NE,2 E,3 SE,4 S,5 SW,6 W,7 NW
  optional float sizeX = 6; optional float sizeY = 7;
  optional google.protobuf.Struct perms = 8;
  optional google.protobuf.Struct interaction = 9;   // {type:"meeting"|"desk"|"spawn"|"game"|"whiteboard"|"native_whiteboard"|"broadcast"|"sysImageText"|..., data:{...}}
  optional google.protobuf.Struct config = 10;
  optional PbOrientation spawnDirection = 11;
  optional PbColorVariation colorPalette = 13;       // {1:hue deg, 2:saturation mul, 3:value mul} floats
  optional string colorKey = 14;
  optional string updatedBy = 15;                    // userId
  optional int64 updatedAt = 16;                     // ms epoch
  repeated string tags = 17;
  optional string accessKey = 18;                    // e.g. "PREMIUM"
}
```
Example decoded (`pb_decode.py`), first frame of A: `id:"<objId>"`, `data{ type:2(Floor), x:738.2813, y:1582.0314, assetId:"<assetId>", sizeX:1.0 (field6), sizeY:1.0, updatedBy:"<userId>", updatedAt:1676993669820 }`; wire: `0a14<20 id bytes>12 <len> 08 02 15 <f32 x> 1d <f32 y> 22 14 <assetId> 35 <f32> 3d <f32> 7a 1c <updatedBy> 80 01 <varint updatedAt>`. (`pb_decode.py` reports field 6/7 as 1.0; those are sizeX/sizeY.)
Census of the 1473 objects (pushed within 1.14 s, +78.252 .. +79.394, 106-252 bytes each): Floor 806, Asset 371, Wall 276, Zone 20. Interactions: meeting 20, desk 17, sysImageText 7, native_whiteboard 2, spawn 1, game 1, broadcast 1, whiteboard 1. Bounding box x -3193..7597, y -2871..6574 world units.
Meeting zone example: `type:Zone, x:6643.35, y:2636.25, assetId:"_zone", sizeX:470.99, sizeY:659.39, interaction:{type:"meeting", data:{meeting:{name:"Town Hall"}}}`.
`pb_decode.py` implements `pb_map_object()` incl. `google.protobuf.Struct`. Asset ids reference the `/eyekia/api/v1/asset/` catalog (other agent).
`should_update` controls whether this is pushed; after the initial dump only changes (edits by other users) come as single `b_set_obj` events.

---------------------------------------------------------------------------------------------------

## 4. Timeline of the recording (inferred, nothing labelled by the user)

| +t | event |
|---|---|
| 75.2 | gaia socket A opens (after Google login at ~71, `gaia/find`, `gaia/rooms`) |
| 78.0-79.4 | init, room snapshot, 1473 map objects, own join |
| 79.9 | avatar already in meeting zone `mohyb...` ("Mesa" at the spawn desk); `cluster` empty |
| 79.2-84.6 | getUserMedia prompts (mic/cam permission), welcome prompt, `cluster_mode cluster` at 84.636 |
| 99-128 | other users speaking (s/6 only), user `Stela` disconnects at 127.7 |
| 130.8-131.1 | socket B opens; A closed; `user_reconnected` |
| 133-143 | burst re-sync; walks NE 141.4-143.8 across zones |
| 150-152.4 | walks NW then N; enters `<zoneC>` ("Mesa" near another user) at 152.1 |
| 153.0 | cluster with 1 other user + LiveKit join (+153.17) |
| 160-212 | speaking toggles, camera on 171.5 / off 174.9, leaves zone 179.9 (LiveKit closes 180.27), re-enters 187.2, LiveKit #2 188.0 |
| 194.7 | `getDisplayMedia` = screen share (no gaia message at all for it; `active_app` unchanged) |
| 201.3-213.8 | `should_update false` then `true` (game paused, probably window hidden/picker) -> new `room` snapshot at 214.25 and client re-announces its state |
| 214.6-216.4 | leaves zone (LiveKit closes 214.93), walks to `mohyb` zone at 216.4 |
| 239-243 | final walk |

---------------------------------------------------------------------------------------------------

## 5. Minimal client recipe (derived)

1. Obtain Firebase ID token for the user (out of scope), call `POST /api/v1/gaia/find` (`{type:"group",groupId,alias,redirect:true}`) to get `room.id` and `room.server.url`.
2. `io(origin, {path: "/gaia/g5/v2", parser: msgpackParser, transports:["websocket"], query:{roomId, mediaRegion, password:"", clientVersion:"11.202.0", autoForceId:<nanoid>, clusterMode:"none-connecting", meetingZoneId:"", joinType:"force"}, auth: cb => cb({token})})`. Use a msgpack encoder that writes `undefined` as fixext1 type 0 (`notepack.io`).
3. After `init`: emit `should_update {v:true,t:0,lastObjectId:""}` (if you want the map) and `active_app {activeApp:null}`. Handle `room`, `s`, `cluster`, `presence`, `server_version`.
4. Wait for `room` + own `s/3`; take own position from `room.users[<me>]`; emit `move` with it, `character_state {state:"idle"}`; later `cluster_mode {mode:"cluster"}`.
5. Walk: `character_state walk` + `move`, `move` every 200 ms (absolute, 160 units/tick), final `move` + `character_state idle`. Ignore own echoes.
6. When the avatar overlaps a meeting zone rect: `set_meeting_zone {zoneId}`; on leave `{zoneId:null}`. On `cluster` with `token` connect LiveKit to `wss://<apiHost>/rtc/v1` with `token.token`; with null cluster disconnect.
7. On Socket.IO reconnect re-fetch token, refresh query with latest clusterMode/zone/x/y and emit `user_reconnected {clusterMode, zoneId, shouldUpdate}` then the state burst.
8. Emit `sync_time` every 60 s (optional) and `speech {isSpeaking}` from VAD (optional).

---------------------------------------------------------------------------------------------------

## 6. Open questions and recordings that would close them

| # | Unknown | Recording / experiment |
|---|---|---|
| 1 | Why socket A closed and B reconnected at +130.8 (network? tab throttling?) and whether server sends a `disconnect` reason | Leave the tab idle 3-5 min and also toggle offline in devtools; record. |
| 2 | Pure axis moves and speed per key: which keys give (0,±d) vs (±d,±d); effect of Shift/run, click-to-move (pathfinding) speed ("far multiplier"), jump (`jump1` state) | Walk one key at a time for 10 s each (up, down, left, right, two-key diagonals), then 3 click-to-move trips of known distance, then jump, with a stopwatch log. |
| 3 | Remote avatar movement as seen by us (rate, whether `s/1` carries only the latest, interpolation delay) | Have a second account/browser walk continuously for 20 s near the recorder's user, and log both sides. |
| 4 | Server-side validation: does the server clamp speed/teleports/collisions? | In our own test, send `move` with a far position and through a wall and see if `s/1` echoes it or if a correction/`room` follows. |
| 5 | Exact zone geometry vs avatar coordinates (anchor, iso transform) | Walk slowly (1 tap steps) across the border of one known Zone from 4 sides, log `set_meeting_zone` with the avatar position, and compare with the zone's `x,y,sizeX,sizeY` from `b_set_obj`. |
| 6 | Open-floor proximity (no zone): which message starts voice, who decides | Two accounts standing 100/400/800/1200 units apart outside any zone, 10 s each; look for `cluster`, `/api/v1/conversation/*` and LiveKit signalling. |
| 7 | `presence`, `member_role`, `user_reconnect`, `ring_*`, `emote`, `follow_*`, `spawn` (`s/5`), pets (`s/11-13`), `notify` payloads | Trigger each in UI (status change, emote key, follow request, ring a user), one action per 10 s. |
| 8 | Sit/stand mechanics (`character_state sit` OUT with `target`), auto-sit | Sit on a chair and stand up, record. |
| 9 | Error and auth-failure behaviour (expired token, wrong `clientVersion`, wrong room password, kicked by `joinType=force` from a second tab) | Open a second tab on the same account; send a bad token from a test script. |
| 10 | Why `sync_time` acks are missing for ids 0,1,2 (sent before `init`) | Re-test from a script: emit `sync_time` before and after `init`. |
| 11 | WASM engine constants (movement speed 800 u/s source, tick, collider sizes) | Disassemble the wasm blob loaded by `hephaestus-*.js` / `WASM-CErjq-cP.js`; not done. |
| 12 | Screen share: no gaia event seen. Check if `active_app` or `s/15` changes when sharing an app/window | Share a window for 20 s with another user present. |
