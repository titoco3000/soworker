# SoWork media stack (LiveKit): reference for our own client

Source: one recorded session (about 4 min), Chrome 150 on Linux, SoWork web client 11.202.0.
Everything below is either a **Fact** (evidence in the capture or in the captured JS bundles,
with timestamps in seconds since recording start) or marked **Inference** / **Unknown**.
No tokens, JWTs, join_request blobs, TURN credentials or user identities appear in this file;
placeholders are used (`<JWT>`, `<uid>`, `<conversationId>`, ...).

Helper scripts (read the raw logs, print redacted output): `analysis/scratch/pb.py` (protobuf
wire decoder + log loader), `dump.py` (LiveKit signal walk-through), `jr.py` (JWT claims and
join_request decode), `mp.py` + `gaia.py` (msgpack/Socket.IO decode of gaia events).

## 0. TL;DR

- Media is **LiveKit Cloud**, used through the **unmodified official `livekit-client` 2.20.0**
  JS SDK (signal protocol 17, server 1.13.7). Project host is `sowork-production-7a7l3zs5`.
- **One LiveKit room per gaia "conversation"** (a proximity cluster or meeting-zone group). Room
  name is a random UUID. The two connections in the capture are two **sequential** conversations
  (same people, same cluster, the user left the zone and came back), never concurrent.
- The client does **not** call a REST endpoint to get a token in the normal path. The **gaia
  Socket.IO server pushes a `cluster` event** that carries `{provider:"livekit", roomId:"RM_...",
  token:<JWT>, apiHost}`. The client connects about 80-160 ms later. A REST fallback
  `POST /api/v1/conversation/token {conversationId}` exists in the code but was not exercised.
- Token: HS256 LiveKit JWT, TTL 24 h, grants `roomJoin` + `canUpdateOwnMetadata` for exactly
  one room; participant identity = Firebase uid = gaia userId.
- `nearest-media-region.l.chime.aws` is a leftover from the Amazon Chime provider path: it
  only yields a `mediaRegion` string (`eu-central-1`) that is sent to gaia. It is not part of the
  LiveKit connection.
- For our client: `livekit-client` + a token that gaia hands us is the whole job. The hard part
  is not media, it is behaving enough like a normal gaia client that gaia emits `cluster`
  events (position, `set_meeting_zone`, `cluster_mode`). Linux WebKitGTK WebRTC remains the
  risk (section 8).

## 1. Stack identification

| Item | Value | Evidence |
|---|---|---|
| Media server | LiveKit Cloud (SFU) | WS path `/rtc/v1`, protobuf `SignalResponse`, `_lossy`/`_reliable` data channels, `PA_`/`TR_`/`RM_` sid prefixes, `*.livekit.cloud` hosts |
| Server version | 1.13.7 (JoinResponse `server_version` and `server_info.version`), edition 1 (cloud) | ws 08 frame 154.347 |
| Server node/region | node `NM_OFRANKFURT1B_*`, region `ofrankfurt1b`, label "Germany 2" | JoinResponse `server_info`, `server_region` |
| Client SDK | `sdk=JS (1)`, `version="2.20.0"`, `protocol=17`, `client_protocol=1` (ClientInfo field 12) | decoded `join_request` (both connections); constant `Uu="2.20.0",am=17` in `livekit-client.esm-BqW1jPQq.js` |
| Join request shape | `join_request` query param = base64url of `WrappedJoinRequest{compression=GZIP, join_request=<gzip bytes>}`. Inner `JoinRequest`: `client_info`, `connection_settings{auto_subscribe=true, adaptive_stream=true}`, `publisher_offer{type:"offer", sdp, id:1}` | `jr.py` |
| Signalling URL | `wss://<host>/rtc/v1?access_token=<JWT>&join_request=<b64url>` (no other query args) | ws.open events |
| Peer connection | **Single PeerConnection** per room (publisher and subscriber on the same PC). Client always sends the offer; server only answers. JoinResponse has `fast_publish=1`, `subscriber_primary` absent (false). | No server-initiated `offer` in either WS. Offers by client ids 1..5/6, answers echo ids |
| ICE servers | Not in `RTCPeerConnection` config (`pc.new` shows only `sdpSemantics`, `gather_continually`). They arrive in JoinResponse `ice_servers` (one entry, three URLs: `turn:ip-<ip>.host.livekit.cloud:3478?transport=udp`, `turns:<region>.turn.livekit.cloud:443?transport=tcp`, `turns:<project>.turn.livekit.cloud:443?transport=tcp`, with short-lived username/credential). No STUN URL. | JoinResponse field 5 (credentials not recorded here) |
| Keepalive | JoinResponse `ping_interval=5`, `ping_timeout=15`; client sends `ping` (field 14) and `ping_req` (16, with `rtt`) every 5 s; server answers `pong`/`pong_resp` | 159.348, 164.350, 169.348 ... |
| Room defaults | `empty_timeout=300` s, `departure_timeout=20` s | JoinResponse room |
| Codecs enabled by the server (`enabled_publish_codecs`) | video: VP8, H264, VP9, AV1, H265; audio: red, opus, PCMU, PCMA | JoinResponse field 14 |

### Is the SDK unmodified? Yes (very likely)

- The WebSocket opens from `static/js/livekit-client.esm-BqW1jPQq.js` (508 KB, a regular ESM
  build of livekit-client with `Uu="2.20.0"`). SoWork's own code
  (`static/js/livekit-BeJ_jwe9.js`, class `LivekitSession`) imports `Room` etc. from it.
- The wire messages are stock livekit-client behaviour (ordering of offers, `sync_state`-less
  fast join, `subscription_permission{all_participants}`, ping/ping_req).
- Note: `livekit-BeJ_jwe9.js` also carries an older embedded copy of the livekit protobuf
  definitions (SignalRequest up to field 16). It is not what ran (**Unknown** what it is for;
  probably a bundled helper library). Irrelevant for us.

### `new Room(...)` options used by SoWork (from `LivekitSession` constructor, JS bundle)

| Option | Value |
|---|---|
| `adaptiveStream` | `true` |
| `dynacast` | `true` |
| `audioCaptureDefaults` | `autoGainControl`, `echoCancellation`, `noiseSuppression` all `true` (the `getUserMedia` call also sets `voiceIsolation:true`) |
| `videoCaptureDefaults` | `facingMode:"user"`, resolution 960x540 (`h540`; the actual call asks 960x540 @ 25 fps, 16:9, exact deviceId) |
| `publishDefaults` (VP9 path, default) | `videoCodec:"vp9"`, `videoEncoding {maxBitrate:1_200_000, maxFramerate:24}`, `videoSimulcastLayers [h180,h360]`, `screenShareEncoding {maxBitrate:4_000_000, maxFramerate:15}`, `screenShareSimulcastLayers []`, `backupCodec {codec:"vp8"}`, `backupCodecPolicy REGRESSION` |
| `publishDefaults` (VP8 path) | used when group flag `gaia.forceVp8Codec` is set or a browser check `XY()` is true: same but `videoCodec:"vp8"`, no backup codec |
| `e2ee` | not used (no E2EE option, no `encryption` field in TrackInfo, `lk-e2ee` logger only) |
| `stopLocalTrackOnUnpublish`, `webAudioMix`, `disconnectOnPageLeave`, reconnect policy | not set (SDK defaults; default reconnect delays visible in bundle `[0,300,1200,2700,4800,7000...]` ms) |
| Extra calls | `room.prepareConnection(serverUrl, token)` right after construction (this is what causes the `/settings/regions` and HEAD requests, see section 2.3); `localParticipant.setAudioContext(...)` for Krisp (optional noise cancellation, not active in the capture) |

Device behaviour: `micPublisher` switches the active input device, `micMutePublisher` calls
`localParticipant.setMicrophoneEnabled(bool)`, `cameraPublisher` publishes the camera (codec +
resolution reconcilers), a `screenshareWatchdog` watches screen-share start/stop signals. Remote
video subscriptions are managed by the app (`videoTileLimiter`: `publication.setSubscribed()` and
`setVideoQuality()` per tile); audio stays on auto-subscribe.

## 2. Room model

### 2.1 What a LiveKit room is here

| Fact | Evidence |
|---|---|
| Room **name** is a random UUID (36 chars), different for every conversation. It is the JWT `video.room` claim and `Room.name` in JoinResponse. | JWT claims; JoinResponse field 1.2 |
| Room **sid** (`RM_...`) is also given to the client by gaia (`token.roomId`) and equals the sid in `room_update`. | cluster event 153.011 vs room_update 154.365 |
| Room **metadata** is JSON `{"groupId":"<org id>","roomId":"<gaia office room id>"}`: it maps the LiveKit room back to the SoWork org and office. | room_update 154.365 |
| Conversation id (`conversationId` in the `cluster` event) is a gaia/Firestore-style id, **not** the LiveKit room name. The mapping `conversationId -> room uuid` is server side and **not visible** to the client (the client never needs it, it gets `token` + `apiHost`). | cluster events |
| Participant **identity** = gaia `userId` = Firebase uid (28 chars, same string appears in Firebase `accounts:lookup`/`signInWithIdp` responses and in gaia `update_info`). JWT `sub` = identity, JWT `name` = display name. | `jr.py` compare, grep |
| Participant **sid** `PA_...` is per connection (changes on re-join). Track sid `TR_...` per publish. | |

### 2.2 Why two connections? (timeline)

Two **sequential** rooms for **one gaia `clusterId`**, with two different `conversationId`s.

| t (s) | Event | Source |
|---|---|---|
| 84.636 | client tells gaia `cluster_mode {mode:"cluster"}` (client state machine: `cluster` = free to talk; other values `none-connecting`, `none-away`, `none-yoga`, `none-private-meeting`) | gaia out |
| 142-151 | user walks into/out of meeting zones; each `set_meeting_zone` gets a `cluster` reply with `users:[]` and `token` undefined (msgpack ext 0) = no conversation | gaia in/out |
| 152.132 | client: `set_meeting_zone {zoneId:<zone A>}` | gaia out |
| **153.011** | gaia -> client `cluster {clusterId, conversationId:<conv 1>, users:[2 users], token:{provider:"livekit", roomId:"RM_..", token:<JWT>, apiHost:"<project>.livekit.cloud"}}` | gaia in (the other user is "remote A") |
| 153.124 | `GET <project>.livekit.cloud/settings/regions` (prepareConnection) | http.tsv |
| **153.171** | LiveKit WS #1 opens to `wss://<project>.livekit.cloud/rtc/v1` (about 160 ms after the cluster event) | ws 08 |
| 154.347 | JoinResponse; 154.354 server answer to the offer carried in `join_request`; PC connected 154.461 | |
| 179.881 | client: `set_meeting_zone {zoneId:null}` (left the zone) | gaia out |
| 180.246 | gaia: `cluster {clusterId:null, conversationId:null, users:[]}` = conversation ended | gaia in |
| **180.268** | client sends `leave {reason: CLIENT_INITIATED}` and closes WS #1 / PC 1 (22 ms after cluster end) | ws 08 |
| 187.173 | `set_meeting_zone {zoneId:<zone A>}` again | gaia out |
| **187.968** | `cluster` again, same `clusterId`, **new `conversationId`** (<conv 2>), new token, new `RM_` sid, new room UUID | gaia in |
| 188.023 | `HEAD https://<project>.ofrankfurt1b.production.livekit.cloud/` (region warm-up) | http.tsv |
| **188.053** | LiveKit WS #2 opens, this time **directly to the regional host** `<project>.ofrankfurt1b.production.livekit.cloud` | ws 09 |
| 214.912 / 214.931 | cluster ended / `leave`, WS #2 closed (215.006 server echoes `leave`) | |

So: the second connection is a new conversation, not a second region. The host difference is
the SDK's region selection (2.3). The room is destroyed server side soon after (empty timeout
300 s is the server config, departure 20 s).

**Inference:** a room is created per *cluster/conversation*: either by proximity (outside
meeting zones, "proximity circle" in the UI is shown when `clusterMode` is `cluster` or
`none-connecting`, hidden while in a meeting zone) or by a meeting-zone group. In the capture
both cluster events followed `set_meeting_zone` into zone A with the other user. **Unknown**:
the exact server rule for proximity clusters outside zones (radius, hysteresis). Needs the
extra recordings in section 9.

**Inference:** `cluster {users:[...]}` lists the users in the conversation (with avatar config),
so the client can render names before LiveKit participants arrive.

### 2.3 How the client learns the room and how tokens are minted

| Step | Detail | Confidence |
|---|---|---|
| Learn the room | gaia Socket.IO event `cluster` (server -> client). Client code: if `clusterId && conversationId` it stores `{conversationId, cachedToken: token}` and starts a media session; otherwise it ends the session (and tells the native app `cluster_end`). | Fact (bundle + capture) |
| Token source (normal) | Embedded in that event: `token = {provider, roomId, token, apiHost}`. The JWT is byte-identical to the one in the WS `access_token`. Minted by the gaia backend with the LiveKit API key (HS256, `iss` = API key id, 15 chars). | Fact |
| Token source (fallback) | `POST https://api.sowork.com/api/v1/conversation/token` (module `conversations-*.js`, used as `getToken({conversationId})`, retried 3x with 1 s delay) when `cachedToken` is missing. Request body presumably `{conversationId}`; response is the same `{provider, ...}` object (a Chime variant has `session{sessionId, meeting, attendee}`). **Not seen in the capture**, so body/response format is unverified. | Code only |
| Provider switch | `OEi(token, ...)` switches on `token.provider`: `"chime"` (Amazon Chime SDK, `chime-*.js` chunk), `"livekit"`, `"cloudflare"` (`cloudflare-*.js`, `CloudflareSession`). Only `livekit` was seen. The provider is chosen by the server (per group/flag), so our client must handle the case that gaia says something else. | Fact (code) |
| Region selection | `prepareConnection(serverUrl, token)` => `GET <project host>/settings/regions` with `Authorization: Bearer <access_token>` (response max-age=5, CORS allows only `Origin: https://app.sowork.com`) => nearest region by `distance` => `HEAD <regional host>/` warm-up => `connect()` uses the regional URL. On the first room (153.1) the WS opened before the HEAD finished, so it used the generic host and LiveKit routed to `ofrankfurt1b` anyway (JoinResponse `region`). Regions offered to this client: ofrankfurt1b, omarseille1b, ojerusalem1a, ojeddah1a, odubai1a, omontreal1a. `/settings/regions` was also polled about every 5 s from 153 to 185 (**Unknown** which caller; looks like the SDK/App re-validating the cache). | Fact |
| Other REST | The code also has `POST /api/v1/livekit/connection-test` and `POST /api/v1/livekit/client-logs` (client diagnostics, gated by feature flags `livekit_logs*`). Not called in the capture. | Code only |

### 2.4 Token (JWT) structure (names only)

| Part | Claim | Value / note |
|---|---|---|
| header | `alg` | `HS256` (no `typ`) |
| payload | `iss` | LiveKit API key id (15 chars) |
| | `sub` | participant identity = Firebase uid (28 chars) |
| | `name` | display name |
| | `nbf` | issue time (about 1 s before the WS opens, so token is minted per cluster event) |
| | `exp` | `nbf + 86400` s (**24 h TTL**, same for both tokens); there is no `iat`, no `jti`, no `metadata`, no `attributes` claim |
| | `video.roomJoin` | `true` |
| | `video.room` | room name (UUID, 36 chars) |
| | `video.canUpdateOwnMetadata` | `true` |
| | `video.roomList/roomRecord/roomAdmin` | `false` |
| | `video.canPublish/canSubscribe/canPublishData/canPublishSources` | **absent**, so LiveKit defaults apply (publish, subscribe and data all allowed). JoinResponse confirms effective `ParticipantPermission{can_subscribe, can_publish, can_publish_data, can_update_metadata}` = all true, no `hidden`, no `recorder`, no source restriction. |

Roles: the app has a concept of role change (`applyRoleChange`; `newRole !== "subscriber"`
=> expects `canPublish` to flip within 2 s). That means gaia/server can change permissions
in-session via LiveKit's UpdateParticipant (arrives as a `update` signal with a new permission).
Not triggered in the capture. **Unknown**: which roles exist (viewer/guest?).

The token is re-sent by the server in `refresh_token` (field 16) right at the start of each
session (154.351 in the raw frame list, 640 bytes). Not needed by us for a 24 h token.

## 3. Track model

### 3.1 Published tracks (local, from `add_track` / `track_published`)

| Room | t | Track | Source | Details |
|---|---|---|---|---|
| 1 | 159.490 `getUserMedia` (audio only, deviceId exact default, AGC+EC+NS+voiceIsolation) -> 159.532 `add_track` -> 159.551 `track_published` | microphone | `MICROPHONE (2)`, type audio, stream id `camera` | `audio_features [AUTO_GAIN_CONTROL, ECHO_CANCELLATION, NOISE_SUPPRESSION]`, `backup_codec_policy` field = 2, server TrackInfo `mime audio/red`, mid 7. Chrome stats: Opus 48k, `usedtx=1;useinbandfec=1`, target 48 kbps (the `red` codec is announced; stats show only Opus, **Unknown** whether RED is really applied) |
| 1 | 171.502 `getUserMedia` (video 960x540 @25, exact deviceId, `facingMode:user`) -> 171.939 `add_track` -> 171.969 `track_published` | camera | `CAMERA (1)`, video, 960x540, stream id `camera` | cid-bound codecs `vp9` (primary) + `vp8` (backup, policy REGRESSION). Layers: Q0 240x135 @133 kbps, Q1 480x270 @400 kbps, Q2 960x540 @1.2 Mbps (**Inference**: with VP9 these are SVC spatial layers on **one** SSRC, because the SDP has a single `ssrc-group FID` and no `rid/simulcast` lines; the server TrackInfo lists `ssrc` only on layer 0). Server codec: `video/VP9`, mid 8 |
| 1 | 174.874 | camera off | `mute {sid, muted:true}` | camera toggle = **LiveKit mute** (+ gaia `camera_on:false` at the same ms) |
| 2 | 191.459 gUM -> 191.482 `add_track` | microphone | as above | same shape |
| 2 | 194.728 `getDisplayMedia` (see 3.3) -> 201.003 `add_track` -> 201.121 `track_published` | screen share | `SCREEN_SHARE (3)`, video 1920x1080, stream id `screen` | single layer 1920x1080 @4 Mbps, codecs `vp9` + `vp8` backup. Stats (203.028): VP9 profile 0, `scalabilityMode L1T3`, 1920x1080 @14 fps, target about 2.55 Mbps, libvpx software encoder, quality limitation none |
| 2 | 207.115-207.145 | screen share stop | no `mute`/`unpublish` signal; the client removes the sender and sends a new offer with the m-section `inactive` (207.145) | LiveKit infers unpublish from the SDP |

Remote user A (the other participant) published a microphone only (no camera/screen in the
capture): room 1 at 165.868, room 2 at 193.308 (`update` with a track `audio/red`, then
`subscription_permission_update{allowed:true}` and a new offer from us binding it to a
recvonly mid). So **remote video subscription was not observed**; the receive side for video is
untested in this log (only LiveKit's default recvonly transceivers exist).

### 3.2 SDP / transceivers

| Fact | Detail |
|---|---|
| Initial offer | 3 audio + 3 video `recvonly` transceivers (mid 0-5) + `m=application` (mid 6, SCTP). Opus/red/G722/PCMU/PCMA audio; VP8/VP9/H264/AV1 video |
| Answer | `sendonly` for mid 0-5 (server will send into those); Opus, PCMU, PCMA; VP8/VP9/H264/AV1 |
| Publishing | each published track adds a new sendonly transceiver (audio mid 7, video mid 8), renegotiated with a new client offer (ids 3,4,5,6); the server only answers |
| Remote tracks | arrive on the pre-created recvonly mids; `pc.track` stream ids are `PA_<participant sid>\|TR_<track sid>` (the mapping `mid -> track sid` is given in the `answer` signal, field `mid_to_track_id`/`4`) |
| Header extensions | abs-send-time, transport-wide-cc, sdes:mid, rid / repaired-rid, AV1 dependency descriptor, video-layers-allocation, ssrc-audio-level |
| Not present | no simulcast `rid` lines; no FEC for audio beyond opus inband FEC |

### 3.3 Capture constraints

- Mic: `{audio:{deviceId:{exact:<id>}, autoGainControl:true, echoCancellation:true, noiseSuppression:true, voiceIsolation:true}, video:false}`.
- Camera: `{video:{deviceId:{exact:<id>}, facingMode:"user", width:960, height:540, frameRate:25, aspectRatio:1.7777}, audio:false}`.
- Screen: `getDisplayMedia({audio:{autoGainControl:false, echoCancellation:false, noiseSuppression:false, sampleRate:44100}, video:{width:{max:1920}, height:{max:1080}}})` called once, 194.728; the user chose a **window** (the participant metadata then says `screenshare:{type:"window"}`). No screen-share audio track was published (no `SCREEN_SHARE_AUDIO` add_track; **Inference**: window sharing yields no audio).
- Earlier `getUserMedia({audio:true, video:true})` calls at 79.2, 81.4, 81.5, 84.6 s (before any LiveKit room) are device/permission probing/preview in the lobby.

### 3.4 Subscriptions, quality, speakers, mute

| Topic | Fact |
|---|---|
| Auto-subscribe | `join_request.connection_settings.auto_subscribe=true`; right after join the client sends `subscription_permission{all_participants:true}` (154.461) so everyone may subscribe to our tracks. No explicit `subscription` requests in the capture (no remote video existed). |
| Adaptive stream | `adaptive_stream=true`; the app overrides with its own tile limiter for video |
| Dynacast | server tells publisher which layers to produce: `subscribed_quality_update` for camera (172.368, all 3 qualities enabled for vp9) and screen (201.532). Preceded by `track_subscribed{track_sid}` (a remote subscribed to our track): mic 159.948 / 191.896, camera 172.368, screen 201.532 |
| Active speakers | `speakers_changed [{sid, level(float), active}]` several per second whenever someone talks; an entry with no level = stopped. The app maps **our own** speaking state to a gaia event `speech {isSpeaking}` (e.g. LiveKit 160.358 -> gaia 160.358) and gaia broadcasts it as `s` type 6 to others (drives the speaking indicator on avatars) |
| Connection quality | `connection_quality {sid, quality=EXCELLENT(2), score=4.5}` |
| Mute | LiveKit `mute {track_sid, muted}` is used for **camera** (174.874). Mic mute was **not exercised** in the capture (mic stayed published and unmuted; **Inference**: `setMicrophoneEnabled(false)` => LiveKit mute, as for the camera). Gaia additionally gets `camera_on {cameraOn}` (171.502 on, 174.874 off); there is **no** gaia mic-state event in the capture |
| Participant metadata | JSON in participant `metadata`, set via `update_metadata{metadata,name}` (signal field 15) right after join (154.522) and when screen share starts/stops. Keys: `raisedHandAt`, `audioOnly`, `allowsScreenDrawing`, `desktop`, `screenshare` (`null` or `{type:"window"}`), `portrait {url, kind:"character"}`. Server answers `request_response{request_id, ok}`. `canUpdateOwnMetadata` grant is what allows it |

### 3.5 Data channels

Three DCs created by the client at PC creation (labels from the stock SDK): `_lossy`
(unordered, maxRetransmits 0, SCTP id 1), `_reliable` (ordered, id 3), `_data_track` (unordered,
maxRetransmits 0, id 5; new in livekit-client 2.20, unused here). Only `_reliable` carried
traffic: **6 messages in the whole capture**.

Format: LiveKit `DataPacket` protobuf, `user` variant: `{payload, topic, participant_identity,
destination_identities?}` plus `sequence` and `participant_sid` on inbound. Payload is
**MessagePack** of a map (SoWork `Signaler.send(topic, obj)` = msgpack-encode + `publishData(...,{reliable:true, topic})`).

| t | dir | topic | payload (decoded) | meaning |
|---|---|---|---|---|
| 154.705, 188.491 | out | `missing-av-permissions` | `{missingPermissions: []}` | announces "my mic/camera permissions are fine"; re-sent every 120 s (`300 s * 0.4`) while permissions are missing (list contains `"microphone"`/`"camera"`) |
| 155.272, 189.409 | in | same | `{missingPermissions: []}` | remote A's equivalent, sent right when DC opens |
| 201.233 | out | `screenshare` | `{userId:<uid>, start:true}` | screen share started |
| 207.115 | out | `screenshare` | `{userId:<uid>, start:false}` | screen share stopped (`trackSid` key exists in code but was undefined here, so it is omitted by msgpack). Receivers use `start:false` + `trackSid` to arm a watchdog that drops stale screen tiles |

Chat, emotes, positions, typing etc. do **not** use LiveKit data channels (gaia and
`chat.sowork.com` carry them).

### 3.6 pc.stats highlights (two snapshots)

| Item | Room 1 @168.1 | Room 2 @203.0 |
|---|---|---|
| Transport | DTLS 1.2 (`FEFD`) `TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256`, SRTP `AES_CM_128_HMAC_SHA1_80`; client is ICE controlling, DTLS role *server* (we sent `actpass`, LiveKit chose active) | same |
| Selected candidate pair | local `prflx` UDP, remote `host` UDP (SFU public IP), RTT 17 ms | same, RTT 15 ms |
| TURN | **not used**; remote ICE candidates in the answer/trickle are host UDP and host TCP :7881 | same |
| Available outgoing bitrate | 523 kbps (audio only) | 3.18 Mbps |
| Outbound audio | Opus, 395 packets / 95 KB, target 48 kbps, 0 NACK | 490 packets / 118 KB |
| Outbound video | n/a (camera not yet on) | screen: VP9 L1T3, 1920x1080, 14 fps, 514 KB in about 2 s window |
| Inbound audio | Opus stereo (`stereo=1;useinbandfec=1`), 0 lost, jitter 4 ms | 0 lost, jitter 3 ms |
| LiveKit-side RTT | 17-36 ms from the `ping_req.rtt` values seen | |

## 4. Signalling walk-through (LiveKit protobuf, decoded)

Wire: each WS binary frame = one `SignalRequest` (client) or `SignalResponse` (server). The
recorder's text dumps mis-labelled these as msgpack (`MP`); they are protobuf. Field names below
are verified against the message definitions that ship inside the captured `livekit-client`
bundle (`livekit.SignalRequest/SignalResponse`, numbers 1-21 / 1-25).

SignalRequest fields seen: `offer=1, answer=2, trickle=3, add_track=4, mute=5, subscription=6,
track_setting=7, leave=8, update_layers=10, subscription_permission=11, sync_state=12,
simulate=13, ping=14, update_metadata=15, ping_req=16, update_audio_track=17,
update_video_track=18`. SignalResponse: `join=1, answer=2, offer=3, trickle=4, update=5,
track_published=6, leave=8, mute=9, speakers_changed=10, room_update=11, connection_quality=12,
stream_state_update=13, subscribed_quality_update=14, subscription_permission_update=15,
refresh_token=16, track_unpublished=17, pong=18, reconnect=19, pong_resp=20,
subscription_response=21, request_response=22, track_subscribed=23, room_moved=24`.

Sequence for **room 1** (WS #1; times are seconds since recording start; `>` = client to server):

| t | Dir | Message | Notes |
|---|---|---|---|
| 153.171 | | WS open | `access_token` + `join_request` (publisher offer #1 inside it) |
| 154.347 | < | `join` | room (name uuid, empty_timeout 300, departure 20), participant (identity, name, permission all true, region `ofrankfurt1b`), `server_version 1.13.7`, `ice_servers`, ping 5/15, `server_info`, `enabled_publish_codecs`, `fast_publish` |
| 154.351 | < | `refresh_token` | fresh JWT |
| 154.351 | < | `update` | own participant (state JOINED) |
| 154.354 | < | `answer` (id 1) | answer to the offer from `join_request` |
| 154.357 | < x4 | `trickle` | server ICE candidates (UDP v4/v6 + TCP 7881 v4/v6), `target=PUBLISHER` |
| 154.365 | < | `room_update` | adds metadata JSON, 1 participant |
| 154.373 | > | `trickle` | client host candidate |
| 154.387 | > | `offer` (id 2) | second client offer right after join (16.6 KB vs 18 KB first one; purpose **Unknown**, stock SDK behaviour); 154.409 `< answer` id 2 |
| 154.461 | > | `subscription_permission {all_participants:true}` | |
| 154.522 | > | `update_metadata {metadata:<json>, name, request_id 1}` | portrait etc.; 154.539 `< request_response ok` |
| 154.536 | < | `update` | own participant ACTIVE with metadata |
| 154.709 | < | `update` | remote A is in the room (state ACTIVE, region `osaopaulo1b`: so participants may sit on different edge regions, the SFU mesh handles it) |
| 154.705 / 155.272 | DC | `missing-av-permissions` | see 3.5 |
| 159.348 | > | `ping` + `ping_req` | repeated every 5 s |
| 159.490-159.564 | > | `add_track` (mic), `< track_published`, `> offer` id 3, `< answer` id 3, `< update` x2 (own participant now lists the track) | |
| 159.948 | < | `track_subscribed` | someone subscribed to our mic |
| 160.358+ | < | `speakers_changed` | stream of speaking levels |
| 165.868-165.930 | < | `update` (A published mic), `subscription_permission_update`, `> offer` id 4, `< answer` id 4 (`mid_to_track_id {0 -> TR_A}`) | A's audio now plays on mid 0 |
| 171.939-172.005 | > | `add_track` (camera VP9+VP8 backup, 3 layers), `< track_published`, `> offer` id 5, `< answer` | |
| 172.368 | < | `track_subscribed` + `subscribed_quality_update` (dynacast) | |
| 174.874 | > | `mute {camera sid, true}`; 174.891 `< update` (track muted) | |
| 180.268 | > | `leave {reason: CLIENT_INITIATED}` | WS closes 180.317 |

**Room 2** (WS #2, 188.053-215.0) repeats the same pattern with differences: mic `add_track`
191.482; remote A's mic 193.308; **screen share**: `add_track` 201.003 (source 3, 1920x1080,
4 Mbps, vp9+vp8), `track_published` 201.121, `offer` id 5 201.136, `update_metadata` 201.233
(`screenshare:{type:"window"}`), DC `screenshare start` 201.233, `track_subscribed` and
`subscribed_quality_update` 201.532; stop 207.115: `update_metadata` (`screenshare:null`), DC
`screenshare stop`, `offer` id 6 with the video m-section `inactive` (207.145), `answer`
207.350, `update` showing the screen track gone (207.349); `leave` 214.931, server `leave`
215.006.

Request ids: `update_metadata.request_id` 1, 2, 3 are echoed in `request_response`.

## 5. gaia events that matter for media

Cross-reference only (gaia is documented elsewhere). Socket.IO v4 over `wss://api.sowork.com/gaia/g5/v2/?roomId=<officeRoomId>&mediaRegion=<chime region>&password=&clientVersion=<ver>&autoForceId=<id>&clusterMode=<mode>&meetingZoneId=<id>&x=&y=&EIO=4&transport=websocket`, msgpack parser, auth `{token:<Firebase ID token>}` in the connect packet.

| Direction | Event | Payload | Media relevance |
|---|---|---|---|
| out | `cluster_mode` | `{mode: "cluster" \| "none-away" \| "none-yoga" \| "none-private-meeting" \| "none-connecting"}` | opt in/out of conversations; query param `clusterMode` repeats it on connect. **A client that never sends `cluster` mode will not get conversations** (**Inference**) |
| out | `set_meeting_zone` | `{zoneId \| null}` | entering/leaving a named meeting zone; triggers `cluster` replies |
| out | `move`, `character_state` | position | drives proximity (server side) |
| in | **`cluster`** | `{clusterId, conversationId, users:[{id,name,config}], token:{provider,roomId,token,apiHost}}` or `{clusterId:null, conversationId:null, users:[], token:undefined}` | the join/leave trigger (section 2) |
| out | `speech` | `{isSpeaking}` | from LiveKit `speakers_changed` (own entry) |
| out | `camera_on` | `{cameraOn}` | camera toggle state |
| out | `user_update`, `info_update`, `active_app`, `should_update` | misc state | sent on (re)connect; not media |
| in | `s` (batched) | `[6,{userId,isSpeaking}]` etc. | speaking state of others |

## 6. Chime and other media-related hosts

| Host | Purpose | Evidence |
|---|---|---|
| `nearest-media-region.l.chime.aws` | `GET` at 73.633 (once, at app start, before any room): returns `{"region":"eu-central-1"}`. Code `getClosestChimeRegion()` with a 5 s timeout, falls back to `us-east-1`, retries every 120 s on failure. The value is called `mediaRegion`: it is sent to gaia as query param `mediaRegion` and to REST `conversation/invite` (+ metrics `userRegion`). It is a **leftover of the Amazon Chime provider** (the app still contains a Chime SDK path and `ChimeIssueModal`); it has **no effect on the LiveKit connection**. | bundle `BOe()`; ws URL |
| `<project>.livekit.cloud` (`sowork-production-7a7l3zs5`) | LiveKit signalling (global entry) and `/settings/regions` | |
| `<project>.ofrankfurt1b.production.livekit.cloud` | regional signalling (WS #2, HEAD warm-up) | |
| `turn:ip-<ip>.host.livekit.cloud:3478` (UDP), `turns:<region>.turn.livekit.cloud:443` (TCP/TLS), `turns:<project>.turn.livekit.cloud:443` | TURN, from JoinResponse. Unused here since direct UDP worked | |
| SFU media | UDP to the SFU host candidate (port 50000-range), TCP 7881 as fallback | trickle candidates |
| `chat.sowork.com` | chat WS, not media | |

No separate STUN, no Cloudflare/Chime traffic in this capture.

## 7. Recommendation for our client

**Can we use `livekit-client` unchanged with a SoWork-minted token? Yes.** The server is plain
LiveKit Cloud and SoWork's own client is stock `livekit-client`. Anything stock
`livekit-client >= 2.x` that speaks protocol 15+ can join with that JWT. Minimal shape:

```js
const room = new Room({ adaptiveStream: true, dynacast: true,
  publishDefaults: { videoCodec: 'vp9', backupCodec: { codec: 'vp8' }, /* optional */ } });
await room.connect('wss://' + token.apiHost, token.token);   // from the gaia `cluster` event
```

What we need from SoWork:

1. A gaia connection (Socket.IO, msgpack, Firebase ID token) behaving like a normal client:
   connect URL args, send `cluster_mode {mode:"cluster"}`, `move`/`set_meeting_zone`, and
   react to `cluster`. On `cluster` with `clusterId && conversationId` => `room.connect(...)`;
   on a null cluster => `room.disconnect()` (client sends `leave`). That is the full media
   lifecycle; there is no extra REST call. Mirror SoWork's gaia side effects: send `speech`
   on active-speaker change and `camera_on` on camera toggles, otherwise others do not see
   speaking/camera indicators on the avatar.
2. Optional: fallback `POST /api/v1/conversation/token` (need one more recording to learn the
   request/response, section 9).

Token/permission facts relevant to a custom client:
- Grants: join exactly one room, publish + subscribe + publish data (defaults), update own
  metadata. Cannot list/administer rooms, record, or mute others. Token valid 24 h, tied to the
  room UUID and identity; a new conversation always brings a new token. If we disconnect and
  re-join within the same conversation, the same token can be reused until `exp`.
- Identity must stay the Firebase uid; a second connection with the same identity kicks the
  first (LiveKit `DUPLICATE_IDENTITY`, the app handles this reason explicitly). So running our
  client and the official web client at the same time with the same account will fight.
- To be a good citizen toward other SoWork clients, replicate these app-level conventions:
  participant metadata JSON keys (`raisedHandAt, audioOnly, allowsScreenDrawing, desktop,
  screenshare, portrait`), data-channel topics `missing-av-permissions` and `screenshare`
  (msgpack payloads, reliable), stream ids `camera`/`screen`, and source types
  (MICROPHONE/CAMERA/SCREEN_SHARE). Without the `screenshare` topic, the official client's
  "screenshare watchdog" may treat our share as stale; **Unknown** how strictly.
- CORS: `/settings/regions` only allows `Origin: https://app.sowork.com`. In a Tauri webview
  skip `prepareConnection` and connect straight to `wss://<apiHost>` (WebSocket is not subject
  to CORS; LiveKit Cloud routes to the nearest region itself, as happened for WS #1). If we want
  region pinning, do that fetch from Rust with a spoofed Origin. **Unknown**: whether LiveKit
  Cloud checks `Origin` on the WS (nothing suggests it; verify in the spike).

**Linux WebKitGTK/WebRTC risk (PLAN.md M0).** *From my own knowledge, not from the capture, and
possibly out of date; verify in the spike:* WebKitGTK has WebRTC and `getUserMedia` behind build
flags (`ENABLE_WEB_RTC`/`ENABLE_MEDIA_STREAM`, GStreamer or libwebrtc backend); distribution
builds differ, `RTCPeerConnection` may be missing, codec support (VP9/H264 encode, Opus) depends
on GStreamer plugins, `getUserMedia` needs a permission-request handler in the embedding app
(Tauri/wry has to enable the `enable-media-stream` setting and grant the request), and
`getDisplayMedia` needs xdg-desktop-portal + PipeWire. LiveKit's own docs historically list
Safari/WebKit as supported but WebKitGTK is not a tested target. This stack needs: Opus audio,
VP8 (safest) or VP9 video, unified-plan with several transceivers, DataChannel
(ordered and unordered), and `RTCRtpSender.setParameters` (used by simulcast/dynacast). A
pragmatic mitigation that fits this capture: **force VP8** (`videoCodec:'vp8'`, no backup; the
app itself has a `forceVp8Codec` path) which avoids VP9 SVC dependence, and receive-only video
is the easiest part. Fallbacks, in order of effort: (a) Chrome/Chromium `--app` mode as in
PLAN.md; (b) Electron/CEF; (c) a native client with the LiveKit Rust SDK (libwebrtc based)
speaking to the same signalling, which avoids the webview altogether but costs the most work.
This capture proves the server accepts a plain `livekit-client`, so (a) is low risk.

## 8. Facts vs guesses, quick index

| Claim | Status |
|---|---|
| LiveKit Cloud, server 1.13.7, protocol 17, SDK JS 2.20.0, single PC, auto-subscribe, adaptive stream | Fact |
| Room options adaptiveStream/dynacast/VP9+VP8 backup/h540/4 Mbps screen | Fact (bundle) |
| Token delivered inside gaia `cluster` event, byte-equal to WS token | Fact |
| REST fallback `POST /api/v1/conversation/token` exists | Fact (code), body/response Unknown |
| Room per conversation, UUID name, metadata `{groupId, roomId}` | Fact |
| Rule that creates a conversation (proximity radius, zone semantics) | Unknown (zone entry with another user observed) |
| VP9 layers are SVC on one SSRC | Inference (SDP + stats `L1T3`; camera stats not captured) |
| Mic mute uses LiveKit `mute` | Inference (camera does; mic not toggled) |
| Chime request only feeds `mediaRegion` | Fact (code) for use; effect on gaia/server routing Unknown |
| Role/permission changes via UpdateParticipant | Inference (client code waits for `canPublish` change) |

## 9. Open questions and recordings that would close them

| Question | Extra recording |
|---|---|
| How does a conversation form outside meeting zones (radius, hysteresis, group merge/split, multi-person)? | Walk toward and away from a teammate in open floor, 3+ users; note `cluster` events and `users` list changes; check whether the same LiveKit room gets a third participant (`update`) or a new conversation is created |
| Mic mute/unmute signalling (LiveKit `mute` vs track unpublish vs gaia event) and push-to-talk | Toggle mic 3x with 10 s gaps |
| Remote video receive path: subscribe/quality requests, `track_setting`, `update_layers`, tile limit behaviour, SVC consumption | Join a room where another user has camera and screen share on; 3+ video tiles |
| REST `POST /api/v1/conversation/token` request/response; when it is used instead of `cluster` | Reload the page while inside a conversation; start a private call (`conversation/invite`, `respond`) and a meeting-link join |
| `refresh_token` / reconnect behaviour (resume, ICE restart, `reconnect` signal), full token refresh after 24 h | Kill the network for 20 s while in a room; leave the tab idle/background 5+ min |
| Permission/role changes (`subscriber` role, kicked, `DUPLICATE_IDENTITY`) | Join as guest/visitor link; open the same account in two tabs |
| Screen share with audio; screen share by a remote user; stop-share via browser UI vs app button | Share a tab with audio; have a teammate share while we watch |
| Other providers (Chime/Cloudflare): which groups get them, what does `token` look like | Capture a session from an org/flag using Chime if any; check group `gaia` flags in the `gaia/find` response |
| Whether LiveKit Cloud validates the WS `Origin`, and whether `/settings/regions` is needed | Spike with a non-browser WS client (or Tauri webview) using a freshly captured token |
| Meaning of unknown fields (`Participant` 20, `backup_codec_policy` value 2 on audio, RED actually used) | Compare against the public protocol definitions or capture `getStats` codec list with RED active |
