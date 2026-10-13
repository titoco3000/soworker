# Muting others and away / available

Source: the second recording (`recorder/raw-logs/2026-10-06T17-30-38-857Z`, ~376 s: join a meeting, be muted, mute someone,
the other person goes away and back, then the same for the recording user), cross-checked against SoWork's own client code
(`bodies/0ba23eb32e0799b6.js`, `bodies/d3674270c44d8bdb.js`). Facts are marked **[recording]** or **[client code]**.

## 1. Muting another person

There is **no REST call and no gaia event**. It is a LiveKit **data message** between the two clients, so it only works
inside a call.

| | |
|---|---|
| Transport | LiveKit data packet, reliable, topic `ask-to-mute` **[recording, client code]** |
| Payload | MessagePack map `{ "targetUserId": "<firebase uid of the person to mute>" }` **[recording]** |
| Sent by | `askToMute` -> `sendAskToMute` -> `signaler.send("ask-to-mute", {targetUserId})`; broadcast to the room, receivers filter on the target **[client code]** |
| Receiver rule | act only if `targetUserId` is me, **my mic is currently on**, and the **sender's role** has the group permission `ask_to_mute` (`isSenderAllowed`) **[client code]** |
| Receiver effect | mute own mic (`setAudioEnabled(false)`), which is an ordinary LiveKit `mute {track_sid, muted:true}` signal; show a 60 s notification "<name> asked you to mute" **[client code, recording 221.2 s and 299.7 s]** |
| Un-mute | the user does it themselves: another `mute` signal (`muted` false) **[recording 236.0 s, 321.6 s]** |
| Sender rule | button shown only if the `ask_to_mute` feature is enabled, my role has the group permission, and the target is not already muted (`canAskToMute`) **[client code]** |
| "Mute all" | one message per unmuted participant (`askToMuteAll`) **[client code]** |

Evidence in the recording: two inbound `ask-to-mute` messages from the other user (221.2 s and 299.7 s), each followed
within the same 100 ms by my outgoing LiveKit `mute` signal. **The recording contains no outgoing `ask-to-mute`**, so the
send side comes from the client code; our encoder produces byte-for-byte the payload of the recorded inbound messages
(unit test `ask_to_mute_payload_matches_the_official_clients_bytes`).

Related topics in the same client (not implemented): `ask-to-enable-camera` and `camera-request-declined`. SoWork can only
**ask someone to turn their camera on**; there is no "turn off their camera" message.

## 2. Away / available

Two systems change together: gaia's cluster mode (decides who can be put into calls) and a REST presence status (what
people see in lists).

Going **away** (359.1-361.1 s) **[recording]**:

1. `POST /api/v1/user-presence/scheduleClear` `{"type":"cancel"}` -> `{"success":true}`
2. `POST /api/v1/user/updateUiSettings` `{"userId":"<me>","data":{"presenceStatus":{"clearAfter":"never"}}}`
3. gaia OUT `cluster_mode {"mode":"none-away"}`
4. `POST /api/v1/user-presence/update` `{"status":"busy"}` -> `{"success":true}`

Back to **available** (362.6-364.2 s):

1. gaia OUT `cluster_mode {"mode":"cluster"}`
2. `POST /api/v1/user-presence/update` `{"status":"online"}`

Headers on the REST calls: `Authorization: Bearer <Firebase ID token>`, `source: web`, `version: 11.202.0`, JSON body.

What others receive **[recording]**:
- gaia `s` code 10 `{userId, mode}`: `none-away` ~0.3 s after the change, `cluster` when back.
- gaia IN `presence {userId, presence:{status, isIdle, isOnline, inPrivateConversation, inCalendarMeeting, isYoga, textStatus, mobile, lastOnline}}`
  ~2 s later (after the REST call): `status` `busy` / `online`.
- While `none-away` the user is not clustered into calls; leaving the meeting happened as a LiveKit `leave` (359.6 s).

The client applies the status locally first, then sends the REST body, which is a partial update of the presence object
(`status`, `isIdle`, `isYoga`, `inPrivateConversation`, `inCalendarMeeting`, `textStatus`) **[client code, `updateMyPresence`]**.

## 3. What the native app does

- Tile **Mute** button: sends `ask-to-mute` for that person (disabled when they already show as muted); toast "Asked <name> to mute".
- Receiving `ask-to-mute` for me while my mic is on: mutes the mic and toasts "<name> asked you to mute".
  We do **not** check the sender's role (we do not have the group's permission table), so anyone in the call can mute you.
- **available / away** button (or **B**): sends gaia `cluster_mode` and the REST sequence above. Away users (anyone with
  `none-away` or status `busy`) are drawn dimmed with an amber dot on the map.
- Camera-off for others stays a stub (SoWork has no such message).

## 4. Open questions

- The group role/permission table (`ask_to_mute`) lives in group settings (Firestore); not captured.
- Other statuses besides `busy`/`online`, the options behind `clearAfter` (scheduled status clear), `textStatus`, and
  automatic idle (`isIdle`) were not exercised.
- Whether the official client shows a different label for `busy` than "away".
