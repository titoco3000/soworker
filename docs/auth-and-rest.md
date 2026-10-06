# SoWork: authentication, REST API, Firestore and chat socket

Source: one recorded browser session (about 4 min, 2026-10-06, web client version `11.202.0`), plus static reading of the captured JS bundles where marked.
Evidence tags: **[obs]** observed in the recording (timestamps are seconds since recording start), **[bundle]** read from the captured JS (not exercised in the recording), **[infer]** inference, **[unk]** unknown.
No secret values appear in this file. Placeholders: `<GROUP_ID>`, `<ROOM_ID>`, `<UID>`, `<ID_TOKEN>`, `<FIREBASE_API_KEY>`, etc. The gaia socket and LiveKit media are documented elsewhere; this file only says what is passed to them.

## 0. Summary

- Identity is **Firebase Auth** in project `sage-learning-prod`. The only credential the SoWork backends use is the **Firebase ID token** (a JWT, 3600 s lifetime). There is **no SoWork-specific session cookie or JWT** [obs]: every cookie on `.sowork.com` is analytics/Intercom, and the backends never send `Set-Cookie` for auth.
- The ID token is presented as `Authorization: Bearer <ID_TOKEN>` on `api.sowork.com` REST, as `?token=` on the chat WebSocket, in the Socket.IO CONNECT packet (`{"token": ...}`) on the gaia WebSocket, and in the WebChannel headers on Firestore.
- **LiveKit tokens are not minted by a REST call.** They arrive inside a gaia socket event (`cluster`, field `token = {provider:"livekit", roomId, token}`) [obs]. There is no REST endpoint that returns a LiveKit token.
- **The map/office definition is not returned by REST.** `POST /api/v1/gaia/find` returns the room and a `mapDefinitionId`, but the layout itself arrives over the gaia socket (`room` event and `b_set_obj` binary object events) [obs]. REST `eyekia` serves the static asset/floor/wall catalog used to render objects.
- A second Firebase project (`sophya-calendar-production`, the calendar service) is signed in through a **custom token minted by `GET /api/v1/calendar/token`** [obs]. A custom client does not need it.

## 1. Authentication flow

### 1.1 Facts, in order [obs]

| t (s) | Request | Meaning |
|---|---|---|
| 0.1 | `GET app.sowork.com/s/<GROUP_ID>/<ROOM_ALIAS>` (200, SPA HTML) | Invite link. Static HTML, no auth. |
| 3.3 | Firestore Listen `groupPublic/<GROUP_ID>` with **no Authorization** | Anonymous read of the public group card (name, logo, `allowGuest`, `allowAnonGuest`, `disableAnonOwner`). Used to render the invite landing page. |
| 3.2 / 3.9 | `POST firebaseinstallations.../installations`, `POST firebaseremoteconfig.../firebase:fetch` | Firebase SDK plumbing (feature flags, see appendix). Not needed. |
| 25.8 | `GET app.sowork.com/login?returnTo=/s/<GROUP_ID>/<ROOM_ALIAS>` | Client-side redirect because no Firebase user is persisted. |
| 39.3 | `POST api.sowork.com/api/v1/auth-providers/getByEmail` `{"email"}` | Returns `{"exists":true,"providers":[{"name":"google.com","provider_id":"google.com","custom_properties":{}}]}`. The client uses `provider_id` to choose a Firebase provider (Google, Facebook, Twitter, or generic `OAuthProvider(provider_id)` with `custom_properties` and `login_hint`; `password` means email+password form). **No Authorization header** on this call; headers sent: `content-type`, `web_client_version: 11.202.0`. |
| 43.0 | `GET app.sowork.com/__/auth/iframe`, `/__/auth/iframe.js`, `/__/firebase/init.json` | Firebase Auth helper iframe, served from the app origin (so `authDomain` is effectively `app.sowork.com`). `init.json` returns the public web config (apiKey, appId, projectId, authDomain `sage-learning-prod.firebaseapp.com`, ...). The apiKey is a public identifier; it is not reproduced here. |
| 43.1 | `GET identitytoolkit.../v1/projects`, `.../getProjectConfig` | Returns `authorizedDomains`: `localhost`, `sage-learning-prod.firebaseapp.com`, `app.sagelearning.ai`, `app.sophya.ai`, `app.sophya.world`, `sophya-assets.web.app`, `app.sowork.com`, `assets.sowork.com`. |
| 44.3 | (popup window) `POST www.googleapis.com/identitytoolkit/v3/relyingparty/createAuthUri` body `{providerId:"google.com", continueUri:"https://app.sowork.com/__/auth/handler", customParameter:{display:"popup", prompt:"consent", login_hint:<email>}, oauthScope:"{\"google.com\":\"profile\"}"}` | Firebase builds the Google authorization URL. Response body not captured; by the standard API it holds `authUri` and `sessionId`. |
| 44.7 | popup navigates to `accounts.google.com/o/oauth2/auth?response_type=code&client_id=<FIREBASE_WEB_CLIENT>&redirect_uri=https://app.sowork.com/__/auth/handler&state=...&scope=...&display=popup&prompt=consent&login_hint=...` | Authorization **code** flow. The redirect URI is on `app.sowork.com`, not a loopback. |
| 44.8 to 70.7 | Google sign-in and consent pages (about 26 s of user interaction) | |
| 70.7 | popup lands on `app.sowork.com/__/auth/handler?state=..&iss=https://accounts.google.com&code=..&scope=email profile ...userinfo.email/profile openid&authuser=0&hd=<WORKSPACE_DOMAIN>&prompt=consent` | `hd` is present: the account belongs to a Google Workspace domain. No `hd` restriction was sent in the request (not in `customParameter`), so SoWork does not restrict by domain at the Google step. |
| 71.3 | `POST identitytoolkit.../v1/accounts:signInWithIdp` body `{requestUri:<that handler URL incl. code+state>, sessionId:<from createAuthUri>, returnSecureToken:true, returnIdpCredential:true}` | Firebase exchanges the code server-side (the Google client secret never leaves Firebase). Response (keys only): `idToken, refreshToken, expiresIn:"3600", localId, federatedId, providerId, email, emailVerified, displayName, firstName, lastName, photoUrl, oauthAccessToken, oauthIdToken, oauthExpireIn:3599, rawUserInfo (contains hd), isNewUser?` |
| 71.8 | `POST accounts:lookup` `{idToken}` | Standard SDK profile fetch. |
| 72.5 | first `api.sowork.com` calls with `Authorization: Bearer <ID_TOKEN>` | Session bootstrap starts (section 5). |
| 73.9 | `POST accounts:signInWithCustomToken` `{token:<calendar custom token>, returnSecureToken:true}` | Signs a **second Firebase app** (calendar project) in, see 1.4. |

### 1.2 Tokens

| Token | Where obtained | Format / claims (names only) | Lifetime | Sent where |
|---|---|---|---|---|
| **Firebase ID token** (main) | `signInWithIdp` response `idToken` (71.3), refreshed by SDK | RS256 JWT. `iss=https://securetoken.google.com/sage-learning-prod`, `aud=sage-learning-prod`, claims `name, picture, iss, aud, auth_time, user_id, sub, iat, exp, email, email_verified, firebase{identities{google.com[], email[]}, sign_in_provider:"google.com"}`. **No custom claims, no roles.** `sub == user_id ==` Firebase UID. | `exp - iat = 3600 s` | REST `Authorization: Bearer`; chat WS `?token=`; gaia Socket.IO CONNECT `{token}`; Firestore WebChannel `Authorization: Bearer` header field |
| **Firebase refresh token** | `signInWithIdp` response `refreshToken` | opaque, starts `AMf-`... | until revoked (password change, account disabled/deleted, explicit revoke, `validSince` bump). No expiry observed. | only to `securetoken.googleapis.com/v1/token` (not seen in this 4-minute recording, because no ID token expired) |
| Google OAuth access token / ID token | `oauthAccessToken`, `oauthIdToken` in the `signInWithIdp` response | Google tokens | 3599 s | not used by SoWork backends in the recording [obs]; irrelevant for us |
| **Calendar custom token** | `GET /api/v1/calendar/token` `{token}` | JWT (Firebase Admin custom token) issued by the calendar project's Firebase Admin service account; `aud=https://identitytoolkit.googleapis.com/google.identity.identitytoolkit.v1.IdentityToolkit`, claims `iss, sub, uid, claims{}, iat, exp` | 3600 s | only to `accounts:signInWithCustomToken` of the calendar Firebase project |
| **LiveKit access token** | gaia socket `cluster` event | HS256 JWT. claims `name, video{roomJoin:true, room:<uuid>, canUpdateOwnMetadata:true, roomList/roomRecord/roomAdmin:false}, iss:<LiveKit API key id>, sub, nbf, exp` | `exp - nbf = 86400 s` | LiveKit `wss://<project>.livekit.cloud/rtc/v1?access_token=...&join_request=...` and `Authorization: Bearer` on `/settings/regions` |
| Firebase Installations authToken | installations call | irrelevant | 604800 s | Remote Config only |
| Cookies | none for auth | `.sowork.com`: `AMP_*`, `_ga*`, `_fbp`, `_gcl_au`, `intercom-*`. `chat.sowork.com`: `chat_ws_sticky` (load balancer stickiness, session cookie, not httpOnly). | | |

Browser persistence [obs]: Firebase user and tokens live in IndexedDB `firebaseLocalStorageDb` (plus `firebase-installations-database`, `firebase-heartbeat-database`, `firebase_remote_config`). localStorage holds only theme/i18n/analytics keys. **No tokens in localStorage or cookies.**

Refresh mechanism [infer, standard Firebase JS SDK 9.23.0, not exercised in the recording]: the SDK refreshes the ID token about 5 minutes before expiry by `POST https://securetoken.googleapis.com/v1/token?key=<FIREBASE_API_KEY>` with `grant_type=refresh_token&refresh_token=<REFRESH_TOKEN>`; the response contains `id_token`, `refresh_token`, `expires_in:"3600"`, `user_id`. The chat client takes the token from a `getToken()` callback on every REST call and on every WS (re)connect, so the token passed is always the current one [bundle].

### 1.3 Required headers and CORS per host [obs]

| Host | Auth | Headers the web client sends | CORS answer | Notes |
|---|---|---|---|---|
| `api.sowork.com` `/api/v1/*` | `Authorization: Bearer <ID_TOKEN>` (except `auth-providers/getByEmail`) | `content-type: application/json`, `source: web`, `version: 11.202.0` (chat-v2 calls send only `authorization`), browser `Origin: https://app.sowork.com` | `Access-Control-Allow-Origin: https://app.sowork.com`, `Allow-Credentials: true`, `Allow-Headers` is exactly `authorization,content-type,source,version` (and `content-type,web_client_version` for getByEmail), methods `POST, GET, OPTIONS, DELETE, PATCH, PUT`, `max-age 86400` | Cookies are sent by the browser (analytics only) and are not needed [infer; not tested without them]. Whether the server rejects missing `source`/`version`/`Origin` is **[unk]**. A native client is not subject to CORS, but should send `Origin: https://app.sowork.com`, `source: web`, `version: <current>` to look identical. |
| `api.sowork.com/eyekia/*`, `app.sowork.com/eyekia/*` | none | only Origin/Referer | `ACAO: https://app.sowork.com` | public asset catalog; the same service is reachable on both hosts |
| `api.sowork.com/gaia/<serverId>/v2/` (WS) | ID token in the Socket.IO CONNECT packet | `Origin: https://app.sowork.com` on the upgrade | n/a | see section 5 |
| `chat.sowork.com/ws` (WS) | ID token in the query string | `Origin: https://app.sowork.com`; server is nginx, sets `chat_ws_sticky` | n/a | |
| `*.livekit.cloud` | LiveKit JWT | | | LiveKit's own protocol |
| `identitytoolkit.googleapis.com`, `securetoken.googleapis.com` | Firebase web `key=<FIREBASE_API_KEY>` query param | | Firebase allows browser origins; the key may carry HTTP-referrer restrictions [unk] | A native client may need to send a `Referer`/`Origin` of an authorized domain if the key is referrer-restricted. Not tested. |

Server stack hints [obs]: `app.sowork.com` is behind CloudFront (static SPA, `/__/auth` and `/__/firebase` are Firebase Hosting reserved paths proxied on the app domain). `chat.sowork.com` is nginx. `api.sowork.com` hosts several services by path prefix: `/api/v1` (main), `/eyekia` (assets), `/gaia/<serverId>` (realtime world server, here `g5`).

### 1.4 The second Firebase project (calendar)

At 72.5 `GET /api/v1/calendar/token` returns `{"token":<custom token>}` whose issuer is a service account of project `sophya-calendar-production`. At 73.9 the client calls `signInWithCustomToken` with it (a second Firebase app, `x-firebase-gmpid` of a different app id), then at 75.2 opens Firestore Listen against database `projects/sophya-calendar-production/databases/(default)` for collections `events` and `accounts` filtered by `userId`. This is the calendar/meeting-integration feature. **Not needed** unless we want calendar data.

### 1.5 Can a non-browser client obtain the tokens? (analysis; nothing here was tested)

Constraints from the evidence:
1. Sign-in is the Firebase authorization-code flow with `redirect_uri = https://app.sowork.com/__/auth/handler` and a Firebase-owned Google web OAuth client. Google OAuth redirect URIs must be pre-registered on the client, so a loopback `http://127.0.0.1:<port>` redirect on **that** client is almost certainly rejected (`redirect_uri_mismatch`). `localhost` is listed in `authorizedDomains`, which only means Firebase will accept it as `continueUri`; it does not make Google accept it. **[unk, one cheap try would settle it]**
2. A Google ID token obtained with **our own** Google OAuth client (loopback redirect) can be exchanged with `accounts:signInWithIdp` (`postBody=id_token=<GOOGLE_ID_TOKEN>&providerId=google.com`, `requestUri=http://localhost`, `returnSecureToken=true`) only if that client's ID is accepted by the Firebase project's Google provider (the Firebase Google provider validates the token audience against the project's web client and its "safelisted client IDs", which only a project admin can edit). We are not admins of `sage-learning-prod` (SoWork's), so this is **expected to fail** with an audience error. **[infer, untested]**
3. Google blocks OAuth inside embedded webviews (`disallowed_useragent`), so loading the SoWork login page in a Tauri webview is unreliable.

Options, most to least practical:

| Option | How | Pros / cons |
|---|---|---|
| **A. Browser-assisted code capture (recommended to try)** | Our client calls `createAuthUri` itself (`providerId:"google.com"`, `continueUri:"https://app.sowork.com/__/auth/handler"`, `customParameter.login_hint=<email>`) with the public web key, gets `authUri` + `sessionId`, opens `authUri` in the **system browser**. After consent Google redirects to `https://app.sowork.com/__/auth/handler?code=...&state=...`; the page itself will show an error (no opener) but the URL carries everything. The user pastes that URL (or a Tauri webview with a navigation hook intercepts it without loading it). The client then POSTs `accounts:signInWithIdp {requestUri:<pasted URL>, sessionId, returnSecureToken:true, returnIdpCredential:true}` and receives `idToken` + `refreshToken`. | Uses SoWork's own registered client and redirect, no secrets needed. One manual step per sign-in; sessions are then renewed with the refresh token. Needs a check that `createAuthUri` works from a non-browser (API key referrer restrictions [unk]) and that `sessionId`/`state` do not require a same-origin popup (they are plain server-side values per the standard API [infer]). |
| **B. Refresh-token transplant** | The user logs in once in a normal browser profile (the recorder profile already has it). The refresh token is at `IndexedDB firebaseLocalStorageDb` -> `firebaseLocalStorage` -> key `firebase:authUser:<apiKey>:[DEFAULT]` -> `value.stsTokenManager.refreshToken`. Our client stores it (OS keychain) and calls `securetoken.googleapis.com/v1/token`. | Simplest, fully automated afterwards; refresh tokens are long-lived. Treat it as a password. Signing out in the browser (Firebase `signOut`) does not revoke server-side by default, but password changes or account disabling do. |
| **C. Email + password** | SoWork supports a `password` provider (`signInWithEmailAndPassword` is in the login bundle) [bundle]. For this account `getByEmail` returned only `google.com`, so a password is not set. Adding one is a Firebase account change that the UI may not expose. | Ideal for headless use, but **[unk]** whether possible. |
| **D. Own OAuth client + signInWithIdp** | see constraint 2 | Probably fails. |

Workspace restriction: the Google response carried `hd=<WORKSPACE_DOMAIN>` and the access is by invitation/group membership at SoWork, not by Google domain restriction in the OAuth request. Access to a given office is enforced server-side per Firebase UID (group membership); the Firebase ID token has no roles [obs: claims].

Anonymous/guest access: the public group doc exposes `allowGuest: true`, `allowAnonGuest: false` for this group [obs]. Anonymous Firebase sign-in is therefore not a path for this space.

## 2. REST catalog

Base `https://api.sowork.com` unless stated. All calls below are authenticated with `Authorization: Bearer <ID_TOKEN>` and `Origin: https://app.sowork.com` unless the Auth column says otherwise. Bodies are JSON. "Seen" gives the first timestamp in the recording; "[bundle]" means present in the code but not observed.

### 2.1 Session bootstrap and user

| Method, path | Auth | Request | Response (observed shape) | When / purpose |
|---|---|---|---|---|
| `POST /api/v1/auth-providers/getByEmail` | none | `{"email": string}` | `{"exists": bool, "providers": [{"name","provider_id","custom_properties":{}}]}` | 39.3 on the login page after the user typed an email; decides the sign-in method. |
| `GET /api/v1/calendar/token` | Bearer | none | `{"token": <firebase custom token, calendar project>}` | 72.5 right after login; see 1.4. |
| `POST /api/v1/user/getIntercomUserHash` | Bearer | `{"platform":"web"}` | `{"userHash": <64 hex>}` | 72.5; HMAC for Intercom identity verification. Noise. |
| `POST /api/v1/reports/getWeeklyReportsMetadata` | Bearer | `{"groupId"}` | `{"count":0,"hasReports":false}` | 73.3; weekly check-up feature badge. |
| `POST /api/v1/user/updateUiSettings` | Bearer | `{"userId": <UID>, "data": {"mutedSounds": {<soundName>: bool, ... ~50 keys}}}` | `{"success":true}` | 74.5 and 85.3 (and when the user toggles sounds). Persists UI preferences. |
| `POST /api/v1/user/recordLogin` | Bearer | `{}` | `{"success":true}` | 79.9, once the office is entered. Login analytics. |
| `POST /api/v1/userSession/upsert` | Bearer | `{"sessionId":"tab_<epochms>_<rand>"}` (client-generated per tab) | `{"userId","sessionId"}` | 79.9. Registers the browser tab session (likely drives "is online"/multi-tab logic [infer]). |
| `POST /api/v1/crm/track` | Bearer | `{"userId","groupId","events":[{"eventName":"entered_office","timestamp": <epoch seconds float>}]}` | `{"success":true}` | 89.6. Product analytics. Skip in our client. |

No "get my profile" REST call exists in the recording: the user profile (`users/<UID>`, `users_sys/<UID>`) comes from Firestore (section 3), and the profile photo is fetched from S3 (`sowork-space.s3.us-east-2.amazonaws.com/profile_photos/<UID>/profile.webp?v=<ts>`, no auth).

### 2.2 Group / room discovery (the calls that lead to the gaia socket)

| Method, path | Request | Response | Purpose |
|---|---|---|---|
| `POST /api/v1/gaia/find` | `{"type":"group","groupId":<GROUP_ID>,"alias":<ROOM_ALIAS>,"redirect":true}` | `{"room": Room, "roomDefinition": RoomDefinition, "roomGroupId": null}` | 73.6. **Resolves the invite link (`/s/<GROUP_ID>/<ROOM_ALIAS>`) to a concrete room instance.** `room.id` is the `roomId` passed to the gaia socket. |
| `POST /api/v1/gaia/rooms` | `{"groupId"}` | `{"roomDefinitionMap": {<roomDefinitionId>: {"roomDefinition": RoomDefinition, "rooms": [Room]}}}` | 73.8 (and again 213.7). Lists all rooms of the group. |
| `POST /api/v1/gaia/listRoomByRoomDefinition` | `{"roomDefinitionId"}` | `{"rooms": [Room]}` | 78.2. Instances of one definition (room picker). |

Schemas [obs]:

```
Room {
  id: string                      // roomId; use for gaia socket (?roomId=)
  name: string, index: number, local: bool
  serverId: string                // e.g. "g5"
  server: { url: "https://api.sowork.com/gaia/<serverId>" }   // gaia host; WS is wss://api.sowork.com/gaia/<serverId>/v2/
  userIds: string[]               // Firebase UIDs currently in the room (roster)
  userCount: number
  roomDefinitionId: string
  roomDefinition: { ownerKey:"group_<GROUP_ID>", name, groupId, zones:null, projection:null }
  isMeetingSpace: bool
  createdAt/updatedAt: { seconds, nanoseconds }   // Firestore Timestamp shape
  roomGroupId: null
}
RoomDefinition {
  id, groupId, name, alias, mapDefinitionId, ownerKey, primary: bool, isMeetingSpace: bool,
  zones: null, createdAt, updatedAt, roomObjects: {}, roomGroupId: null, pricingType: "premium"
}
```

`gaia/find` and `gaia/rooms` contain only metadata. **The map/layout is not in these responses**: `mapDefinitionId` is never fetched over REST in the recording; the gaia socket sends a `room` event (with `mapDefinitionId`, `mapId` (skybox name), `mapBounds`, `spawns`, `roomObjects`, ...) followed by dozens of `b_set_obj` events carrying protobuf-encoded objects [obs, gaia owner documents the detail].

### 2.3 Chat REST (`/api/v1/chat-v2`)

Observed (all GET, Bearer only, no `source`/`version` headers):

| Path | Query | Response | Seen |
|---|---|---|---|
| `/api/v1/chat-v2/` | `groupId=<GROUP_ID>` | `{channels: Channel[], current_user_group_role: "contributor", has_more: bool, users: {<UID>: {name, photo_url}}, scope_group_ids: [<GROUP_ID>,"__global__"]}` | 73.6 |
| `/api/v1/chat-v2/` | `groupId=__global__&actingGroupId=<GROUP_ID>` | `{channels:[], current_user_group_role:null, has_more:false}` | 73.6 |
| `/api/v1/chat-v2/me/saved-message-ids` | `groupId` | `{ids: []}` | 73.6 |
| `/api/v1/chat-v2/me/saved-messages` | `groupId, limit=50` | `{server_time, items:[], users:{}, apps:{}, channels:{}, has_more, next_cursor, scope_group_ids}` | 73.6 |
| `/api/v1/chat-v2/support/me` | | `{is_support_staff: bool}` | 73.6 |
| `POST /api/v1/conversation/searchDigests` | body `{groupId, section:{kind:"all"}, page:1, pageSize:25, readFilter:"all", grouped:true}` | `{apiVersion:5, results:[], pagination:{page,size,totalCount}}` | 73.6 (digests/meeting summaries inbox; empty here) |

Channel object keys [obs]: `id` (UUIDv7), `group_id`, `type` (`direct` or `general` seen; code also has others), `name`, `description`, `created_by`, `member_key` (for DMs `<UIDa>:<UIDb>`), `app_id`, `conversation_id`, `lifecycle`, `meeting_started_at/ended_at`, `metadata`, `is_default`, `is_locked`, `auto_translate_enabled`, `default_designated_at`, `stale_at`, `archived_at/by`, `support_outreach_pending_at`, `hidden`, `last_message_at`, `created_at`, `updated_at`, `deleted_at`, `membership{pinned_at, unread_count, unread_mention_count, unread_broadcast_count, notification_level, muted_until, role, joined_at, via_team}`, `dm_member_user_ids[]`, `last_message{id, user_id, app_id, type:"regular", parent_id, text, created_at}`.

Not exercised in the recording (no message was sent or opened in the log) but present in the chat client **[bundle]**, same base `/api/v1/chat-v2`, Bearer, JSON, `Accept: application/json`:

| Method | Path | Notes |
|---|---|---|
| `GET` | `/<channelId>/messages` | history; query passed as given by caller (cursor/limit style, exact names **[unk]**) |
| `POST` | `/<channelId>/messages` | send message; response `{message: ...}`; body schema **[unk]** (text, parent_id probable) |
| `PATCH` / `DELETE` | `/messages/<id>` | edit / delete |
| `GET` | `/messages/<id>`, `/messages/<id>/replies` | single / thread |
| `POST` / `DELETE` | `/messages/<id>/reactions`, `/messages/<id>/reactions/<emoji>` | reactions |
| `POST` | `/<channelId>/read`, `/<channelId>/unread`, `/threads/<id>/read`, `/threads/<id>/unread`, `/threads/read-all` | read state |
| `PUT` / `DELETE` | `/<channelId>/draft` | drafts |
| `POST` | `/` | create channel; `GET /browse?groupId`, `/archived`, `/mentions`, `/threads`, `/unread`, `/search/channels`, `/search/messages` |
| `POST/DELETE` | `/<id>/pin`, `/<id>/archive`, `/<id>/join`, `/<id>/members`, `/<id>/members/leave`, `/<id>/members/me`, `/<id>/close` | channel membership/state |
| `POST` | `/<channelId>/upload-url`, `GET /messages/<id>/attachments/<n>/download-url` | attachments |
| `POST` | `/recipients/messages`, `GET /recipients/resolve`, `/recipients/search` | send a DM without knowing the channel |
| `POST` | `/meetings/<conversationId>/channel`, `/meetings/<conversationId>/messages` | meeting chat |
| `PUT` | `/me/language`, `POST /<id>/messages/<mid>/translate` | translation |
| `GET/POST` | `/support/*`, `/saved-replies*` | support, saved replies |

Errors [bundle]: JSON `{error:{code,message}}` or `{code,message}`; 401/403 are treated as auth errors.

### 2.4 Asset catalog (`eyekia`, public, no auth)

| Method, path | Response | Seen |
|---|---|---|
| `GET /eyekia/api/v1/assets-version` (on `api.sowork.com`) | `{assets:[{id, v}]}` (~147 KB, every asset id with a version string) | 2.8 and 27.1 (cache-busting index) |
| `GET /eyekia/api/v1/category/` | `{categories:[{id, data:{name, rank, thumbnail, defaultInteraction, admin}}]}` (desks, seating, ...) | 73.6 |
| `GET /eyekia/api/v1/asset/?categoryId=<id>&pathOnlyUrls=true` | `{assets:[{id, data:{name, description, categoryId, thumbnail, accessKey ("PREMIUM"/""), defaultInteractionType, colors{...}, projections, orientations{N/E/S/W:{goToPivot, images{<id>:{url, transform, primitives{shape/seat/shadow}}}}}}}]}` (1.5 MB) | 73.8 |
| `GET /eyekia/api/v1/asset/<assetId>?pathOnlyUrls=true&v=<ver>` | one asset (same data) | 79.4 (on `app.sowork.com`) |
| `GET /eyekia/api/v1/floor/<id>?pathOnlyUrls=true&v=<ver>` | `{orientations:{N,E,..:{url, pivotPosition{x,y}}}, type:"floor", height, pixelDensity, isCorner, colors, accessKey, name, thumbnail, published}`; 404 `{statusCode:404, code:"not-found", message}` for unknown id | 79.3, about 20 floors |
| `GET /eyekia/api/v1/wall/<id>?pathOnlyUrls=true&v=<ver>` | same shape, `type:"wall"`, 4 orientations | 79.3, about 20 walls |

The ids come from the objects the gaia socket sends (79.3 is about 1 s after the `b_set_obj` burst at 78.25) [infer]. Image bytes behind the relative `url` fields (e.g. `/floor/<id>/<imgId>`) were not captured in the HTTP log; their host/prefix is **[unk]** (two S3 buckets are referenced elsewhere: `mm-asset-images-staging.s3.us-east-2.amazonaws.com` for category thumbnails, `sowork-space.s3.us-east-2.amazonaws.com` for profile photos and group logos `<GROUP_ID>/world_logo/<id>?v=`).
Avatar rendering: `GET app.sowork.com/portrait/v1/<pose>/<character slot string>` (PNG per avatar config), `app.sowork.com/characters/v5/{customizer_config,character_art_database,customizer_product_prices}.json`. All unauthenticated static.

### 2.5 Other first-party / adjacent hosts

| Host | Call | Purpose | Needed? |
|---|---|---|---|
| `nearest-media-region.l.chime.aws` | `GET /` -> `{"region":"eu-central-1"}` | Picks `mediaRegion` for the gaia socket (73.6) | Yes, or hardcode a region (the server may accept any known value, **[unk]**) |
| `uy7tlymu.apicdn.sanity.io` | `GET /v1/data/query/production?query=*[_type == 'releases'] \| order(version desc)[0]{version,bannerLabel,weekOfTitle}` | "What's new" banner | No |
| `sowork-production-7a7l3zs5.livekit.cloud` | `GET /settings/regions` (Bearer = LiveKit JWT, 19 times from 153 s on), `HEAD /` on the regional host | LiveKit region discovery done by the LiveKit JS SDK | handled by the LiveKit SDK |
| `sowork-space.s3.us-east-2.amazonaws.com` | `GET /profile_photos/<UID>/profile.webp?v=<ts>` | profile images | optional |
| Firebase (`firebaseinstallations`, `firebaseremoteconfig`, `firebase.googleapis.com webConfig`) | | SDK plumbing | No |

## 3. Firestore

63 requests to `firestore.googleapis.com/google.firestore.v1.Firestore/Listen/channel` (47 POST, 16 GET) [obs]. This is the Firestore Web SDK (JS 9.23.0) **WebChannel long-polling** transport, used purely as **realtime document/query listeners** (`Listen`); no writes (`Write/channel`) and no unary REST `documents:*` calls were seen.

Transport anatomy [obs]:
- `POST .../Listen/channel?VER=8&database=projects/<project>/databases/(default)&RID=<n>&CVER=22&X-HTTP-Session-Id=gsessionid&zx=..&t=1` with form body `headers=<urlencoded "X-Goog-Api-Client:gl-js/ fire/9.23.0\r\nContent-Type:text/plain\r\nX-Firebase-GMPID:<appId>\r\n[Authorization:Bearer <ID_TOKEN>\r\n]">&count=N&ofs=K&req0___data__=<JSON>...`. First POST returns `[[0,["c","<SID>","",8,15,30000]]]` (session id) and the `gsessionid`.
- `GET .../Listen/channel?gsessionid=..&VER=8&RID=rpc&SID=<SID>&AID=<n>&CI=0&TYPE=xmlhttp&t=1` is the long-lived downstream (server-to-client) stream; responses are length-prefixed JSON arrays of `[seq,[ListenResponse...]]` with `targetChange`, `documentChange`, `documentDelete`, `noop`. The HTTP log recorder could not capture the content of these streaming bodies except the first one, so **document contents for authenticated listeners are [unk]**; only the listener definitions (client to server) are known.
- Each `addTarget {documents|query, targetId}` / `removeTarget` is a POST with `req0___data__=`.
- Anonymous session (3.3 and 27.6, before login): no `Authorization` field; allowed to read `groupPublic/<GROUP_ID>` only. Authenticated sessions carry `Authorization: Bearer <ID_TOKEN>` inside the `headers=` form field (72.1 on, which is why the WebChannel session was re-created right after sign-in at 72.1).

Listeners opened [obs] (project `sage-learning-prod`, database `(default)`), with inferred purpose:

| t (s) | Target (path or query) | Inferred use |
|---|---|---|
| 3.3, 27.6 | doc `groupPublic/<GROUP_ID>` | public group card (name, `authLogoUrl`, `allowGuest`, `allowAnonGuest`, `disableAnonOwner`; the only content captured: 5 fields) for the invite/login page |
| 72.1 | doc `users/<UID>`, doc `users_sys/<UID>` | own profile (name, avatar config, status ...) and own server-managed fields (plan/flags) |
| 72.9 | queries on `groupJoinRequests` (4 variants: by `groupId`+`requestorUserId`, `status=="pending"`, `seenReqAnswer==false`) | "request to join" workflow state |
| 72.9 | query `external_group_access` | guest/external access |
| 73.1 | docs `groups/<GROUP_ID>`, `groupPublic/<GROUP_ID>`, `groupMembers_sys/<UID>_<GROUP_ID>`, `group_membermaps/<GROUP_ID>_0..3` (4 shards) | group settings, own membership/role, member directory (who is in the group, sharded) |
| 73.6-73.8 | docs `groupMembers_integrations/<UID>_<GROUP_ID>`, `group_desks/<GROUP_ID>`, `group_rooms_state/<GROUP_ID>`, `users/<UID>/private/digestReadStates`; queries `groups/<GROUP_ID>/digestTemplates`, `notifications` (where `removed==false`, order `createdAt desc`), `strong_connections` (where `userId==<UID>`, limit 1), `group_announce` (array-contains-any `sc`) | desks, rooms state, notifications, announcements, social graph |
| 75.1-75.3 | docs `users/<other UID>` x about 9 (people in the room) | **per-user profile/presence-ish data for people seen in the room** (names, avatars). Added/removed dynamically (66-80 added at 75, removed at 204). |
| 75.3 | query `conversations` (where `ownerGroupId==<GROUP_ID>`, `roomDefinitionId...`) (3 variants at 213-214) | active conversations (call clusters) in the office |
| 78.2-78.4 | queries `broadcasts` (`status=="started"`, `groupId`) x3, `doorbells` (`groupId`, `requestorUserId`) | broadcasts (announcements to all) and "door knock" requests |
| 80.3 | doc `user_sessions/<UID>` | own tab sessions |
| 84.6 | query `groups/<GROUP_ID>/npcs` | NPC/bot characters |
| 153.2, 185.2, 188.0, 216 | docs `conversations/<conversationId>` | the conversation the user just joined (ids equal the `conversationId` in the gaia `cluster` event) |

Calendar project listeners [obs] (project `sophya-calendar-production`, opened 75.2, signed in via custom token): queries `events` (by `userId`, `start >= ...`) and `accounts` (by `userId`). Calendar only.

What flows through Firestore vs. the other channels [infer from listener definitions]: group/member/profile/notification/doorbell/broadcast/conversation **metadata**. **Not** chat messages (chat has its own REST + WebSocket), **not** positions or the live map (gaia socket), **not** per-second presence (gaia `presence` events).

Do we need it? For a minimal client (walk around, hear/see neighbours): **no**. Needed data and its non-Firestore source: room roster `userIds` (REST `gaia/rooms`), live presence and positions (gaia socket), neighbour names and avatars (gaia `cluster.users[]` carries `name`, `id`, avatar `config`), group name (`groupPublic` is Firestore only, optional). Security rules appear to be based on the ID token (anonymous access reads only `groupPublic`). If we want group member names/roles or doorbells, we need Firestore (`users/<UID>`, `group_membermaps/*`, `doorbells`), but the plain REST variant (`GET https://firestore.googleapis.com/v1/projects/sage-learning-prod/databases/(default)/documents/<path>` with `Authorization: Bearer <ID_TOKEN>`) would be an option **[unk, untested; rules may restrict]**.

## 4. chat.sowork.com WebSocket

### 4.1 Connection [obs; reconnect behaviour from bundle]

`wss://chat.sowork.com/ws?token=<ID_TOKEN>&device_id=<uuid>&group_id=<GROUP_ID>&client=web`, opened at 73.6 (the same instant as the REST chat calls), 101 Switching Protocols, `Server: nginx`, `Origin: https://app.sowork.com`.

| Param | Meaning |
|---|---|
| `token` | Firebase ID token (the socket is **not** re-authenticated later; the client re-reads a fresh token on every reconnect) |
| `device_id` | client-generated UUID, stable per browser install (`resume=<device_id>` is appended on reconnect so the server can replay missed events) [bundle] |
| `group_id` | the group (workspace) scope |
| `client` | `web` |
| `resume` | only on reconnects [bundle] |

Client behaviours [bundle]: events with `event_id` are de-duplicated; a watchdog reconnects if no frame arrives for 75 s (`staleAfterMs` default 75000); exponential backoff capped, up to 2^5 s; close codes 1000 and 1008 do not auto-reconnect; `sync.required` (`data.reason`) tells the client to refetch REST state.

### 4.2 Frames observed (27 in 4 minutes)

All frames are JSON text `{"type": string, "data"?: object}`.

| Dir | type | data | Example (placeholders) | Notes |
|---|---|---|---|---|
| IN | `connection.ready` | `connection_id` (uuid), `channels` (array of channel ids the user belongs to), `scope_group_ids` (`[<GROUP_ID>, "__global__"]`), `position` (stream offset, `"<ms>-0"` Redis-stream style) | `{"type":"connection.ready","data":{"connection_id":"<uuid>","channels":["<CH1>","<CH2>"],"scope_group_ids":["<GROUP_ID>","__global__"],"position":"1791290998977-0"}}` | first frame, 0.9 s after open (74.5) |
| IN | `heartbeat` | none | `{"type":"heartbeat"}` | every 30 s (85.4, 115.5, 145.4, ...) |
| OUT | `activity.report` | `idle_seconds` (int), `locked` (bool), `notifiable` (bool) | `{"type":"activity.report","data":{"idle_seconds":1,"locked":false,"notifiable":true}}` | sent at open and on every focus/blur or activity transition (`notifiable` true when the tab is visible and focused; 22 frames). Server uses it to decide push-style notification vs. in-tab. Not heartbeats. |

No chat messages, typing events or read receipts appear in the recording (nobody sent chat in the window, or the chat panel was not opened).

### 4.3 Frame types the client understands [bundle, not observed]

IN: `message.new`, `message.updated`, `message.deleted`, `message.translated`, `reaction.new`, `reaction.deleted`, `read.updated`, `unread.updated`, `thread.read`, `thread.unread`, `threads.read_all`, `typing.start`, `typing.stop`, `draft.updated`, `draft.deleted`, `channel.updated`, `channel.deleted`, `member.added`, `member.removed`, `member.updated`, `user.updated`, `access.revoked`, `meeting_channel.archived`, `sync.required`, `connection.ready`, `heartbeat`. Payloads are `{type, data, event_id?}`; known `data` keys include `channel_id`, `parent_id` (thread), `message` objects. Exact message schema: **[unk]** (take a recording that includes sending a message).
OUT: `typing.start` `{"type":"typing.start","data":{"channel_id":<id>}}` (throttled), `typing.stop` presumably symmetric, plus `activity.report`. Sending a message, marking read, reacting etc. go through REST (section 2.3), not the socket.

## 5. Bootstrap sequence from cold start (with a valid Firebase login)

Times are from the recording; only the ordering and dependencies matter.

| # | Call | Needs | Produces / used for |
|---|---|---|---|
| 0 | Obtain a Firebase **ID token** (+ refresh token): section 1.5. Refresh via `securetoken` when `exp` is near. | | `<ID_TOKEN>`, `<UID>` (`sub`) |
| 1 | Parse the invite URL `/s/<GROUP_ID>/<ROOM_ALIAS>` | | `groupId`, `alias` |
| 2 | `POST /api/v1/gaia/find {type:"group", groupId, alias, redirect:true}` (73.6) | token | `room.id` = **`roomId`**, `room.server.url` / `serverId` (gaia host), `roomDefinition.mapDefinitionId` |
| 3 | `GET https://nearest-media-region.l.chime.aws/` (73.6) | | **`mediaRegion`** (e.g. `eu-central-1`) |
| 4 | optional, in parallel at 73.6: `POST /gaia/rooms`, `GET /chat-v2/?groupId=`, `/chat-v2/me/saved-*`, `/chat-v2/support/me`, `POST /conversation/searchDigests`, `GET /eyekia/api/v1/category/`, `GET /eyekia/api/v1/asset/?categoryId=`, `GET /calendar/token`, `POST /user/getIntercomUserHash`, `POST /reports/getWeeklyReportsMetadata` | token | UI data. None of these are prerequisites of the gaia socket in the observed order, except that the browser had them in flight; whether the gaia server needs any of them first (for example `userSession/upsert`, `recordLogin`) is **[unk]** (those two ran at 79.9, after the socket was already open at 75.2, so they are not prerequisites) |
| 5 | **Chat socket**: `wss://chat.sowork.com/ws?token=<ID_TOKEN>&device_id=<uuid>&group_id=<GROUP_ID>&client=web` (73.6) | token | `connection.ready`; then `activity.report` OUT |
| 6 | **Gaia socket** (75.2): `wss://api.sowork.com/gaia/<serverId>/v2/?roomId=<ROOM_ID>&mediaRegion=<region>&password=&clientVersion=11.202.0&autoForceId=<client nanoid, 21 chars, random per tab session>&clusterMode=none-connecting&meetingZoneId=&joinType=force&EIO=4&transport=websocket` then, after the Engine.IO open packet `0{sid,pingInterval:35000,pingTimeout:30000,maxPayload:5000000}`, send Socket.IO CONNECT (msgpack parser, packet type 0) **`{"type":0,"data":{"token":"<ID_TOKEN>"},"nsp":"/"}`**; then `sync_time {clientTime}`. | roomId, mediaRegion, token | server pushes `init`, `cluster`, `presence`, `server_version {serverVersion:"11.202.0", minimumClientVersion:"11.185.0"}`, `room`, `b_set_obj`... |
| 7 | Reconnect into a call cluster (130.8): same URL with `clusterMode=cluster&meetingZoneId=<zone id>&x=<float>&y=<float>`, then `user_reconnected {clusterMode, zoneId, shouldUpdate}` | | the app opens a new gaia connection when entering/leaving a call zone |
| 8 | **LiveKit**: the gaia `cluster` event carries `token: {provider:"livekit", roomId:"RM_...", token:<JWT>}`. Open `wss://<project>.livekit.cloud/rtc/v1?access_token=<LK_JWT>&join_request=<base64 protobuf>` with the LiveKit SDK (it also calls `/settings/regions` with `Authorization: Bearer <LK_JWT>` and may be redirected to a regional host, e.g. `...ofrankfurt1b.production.livekit.cloud`). | gaia `cluster` event | audio/video. A new LiveKit token arrives with every `cluster` event (153.0 and 188.0 for two different rooms). |
| 9 | Firestore listeners (optional): section 3 | token | metadata |
| 10 | Housekeeping the web app does and we can skip: `updateUiSettings`, `recordLogin`, `userSession/upsert`, `crm/track` | | analytics/preferences |

gaia handshake parameter provenance:
- `roomId`: `gaia/find` -> `room.id` (or `gaia/rooms`). Same value on both gaia connections in the recording.
- `mediaRegion`: `nearest-media-region.l.chime.aws` response.
- `autoForceId`: not returned by any call; generated client-side (21-char URL-safe id), equal in both connections of the session, so it is a per-tab client id [infer].
- `clientVersion`: web build `11.202.0`; the server reports `minimumClientVersion: 11.185.0` in `server_version`, so old versions are refused at some point. Keep this current.
- `password=`: empty (rooms can have passwords [infer]).
- `joinType=force`: observed value on both connections.
- `clusterMode=none-connecting` first, then `cluster` with `meetingZoneId`, `x`, `y` on re-entry.

## 6. Open questions and which recordings would close them

| # | Question | Recording / test that closes it |
|---|---|---|
| 1 | Does `securetoken` refresh behave as documented here, and does the browser refresh at about 55 min? | Record 70+ minutes (or just capture one refresh). Alternatively test from our client with the refresh token (option B). |
| 2 | Do `api.sowork.com`, the chat WS and gaia accept requests without `Origin`/`source`/`version` headers or with a different `version`? Is cookie state ever required? | Replay one REST call from curl with and without those headers (own account, own token). |
| 3 | Does `createAuthUri` + system-browser + `signInWithIdp` (option A) work outside a browser? Does Google accept a loopback redirect on the Firebase web client? Is the Firebase API key referrer-restricted? | One scripted attempt with the public web key; also records the missing `createAuthUri` response body (`authUri`, `sessionId`) by repeating the login with the recorder. |
| 4 | Is there an email+password sign-in for our workspace accounts? | Open the login page, choose password/reset, record. |
| 5 | Chat message schemas (`message.new`, send body, history query params, read receipts, typing), thread and attachment flow | Record: send a message in `general`, reply in a thread, react, edit, delete, mark read, DM someone, type without sending; also receive a message from another person; open a channel to see `GET /<channel>/messages`. |
| 6 | What does Firestore actually return for `users/<UID>`, `group_membermaps/*`, `group_rooms_state`, `doorbells`, `broadcasts`, `notifications` (content of the streaming responses was not captured)? | Extend the recorder to capture streaming `Listen` GET bodies (e.g. via `Network.dataReceived`/`streamResourceContent`), or record the browser DevTools frame; or just test the Firestore REST GET with our token. |
| 7 | Where do floor/wall/asset image bytes come from (host and path prefix), and are they authenticated? | The recording only has the JSON; record entering a room with a fresh cache and include image requests (the current http log filtered images), or inspect `pathOnlyUrls=false` responses. |
| 8 | How does the app obtain a fresh LiveKit token when it expires (86400 s) or on room change, and is there a gaia event to request one? | Gaia owner: look at outgoing `cluster`/`join`-style events around 153 s and 188 s. A multi-hour session would show re-issue. |
| 9 | Other REST endpoints the recording never hit (map editing, avatar customization/wardrobe save, status/`textStatus` update, profile edit, waves, doorbell/knock, invites, calendar sync): `api/v1/accounts/*`, `api/v1/events/*`, `api/v1/integrations/*`, `api/v1/tokens/*` exist in the bundles (calendar service, **[bundle]**, host not confirmed) | Targeted recordings: change status text, change avatar, knock on a door, start/stop a broadcast, invite a teammate, toggle settings, accept a wave. |
| 10 | Server-side authorization model: can an ID token for a user who is **not** a member of the group call `gaia/find` and join (guest links, `allowGuest`)? Per-room passwords? | Test with a second account (own company) or the invite flow as a guest. |
| 11 | ToS risk is accepted per PLAN.md; account-level detection (version string, UA, fingerprints) is unknown. Keep `version`, UA and Origin identical to the web client. | n/a |

## Appendix A: noise to ignore

| Host / feature | What it is |
|---|---|
| `intercom.io` (`widget.intercom.io`, `api-iam.intercom.io`, `nexus-websocket-a.intercom.io` WebSockets #01, #02, #04, #06) | Support chat widget. `POST /api/v1/user/getIntercomUserHash` feeds it. |
| Amplitude (`AMP_*` cookies/localStorage, `api*.amplitude.com`) | Product analytics |
| Sentry | Error reporting |
| Facebook Pixel (`connect.facebook.net`, `www.facebook.com/tr`), Google Analytics (`_ga*`), Google Tag Manager, `_gcl_au` | Marketing |
| `firebaseinstallations`, `firebaseremoteconfig` (feature flags: `feat_*` with values `all`/`none`/`group_feature`/`only_sophyan`), `firebase.googleapis.com/.../webConfig`, `signaler-pa.googleapis.com/punctual/*` (Google sign-in helper) | Firebase/Google SDK plumbing |
| Google account pages (`accounts.google.com`, `accounts.youtube.com`, `SetSID` on `google.com.br`/`google.de`) | The OAuth popup |
| `uy7tlymu.apicdn.sanity.io` | Sanity CMS release banner |
| `app.sowork.com/static/js/*`, `/static/media/*.mp3`, fonts | SPA bundles and sounds (Vite build, hashed names) |
| `crm/track`, `recordLogin`, `userSession/upsert`, `updateUiSettings` | First-party analytics/preferences, not required to play |

## Appendix B: scratch scripts

Analysis helpers (read-only on the logs, print structure only, never tokens) are in `/home/tito/dev/repos/soworker/analysis/scratch/`: `hdrs.py` (per-endpoint headers and JWT claim names), `ws.py` and `find_lk.py` (WebSocket URLs, token provenance), `cal.py` (calendar custom token claims), `fs.py` and `fs2.py` (Firestore listener decoding), `cors.py` (CORS headers), `nav.py` (OAuth navigation with secrets masked), `st.py` (storage and cookies).
