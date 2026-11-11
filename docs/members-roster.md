# Member list (online and offline) and the sidebar

Source: recording `output/raw-logs/2026-10-07T11-54-43-458Z` (the first one with Firestore streams captured, see
`output/record.mjs`, `res.chunk` events). Facts are **[recording]** unless marked otherwise.

## Current source (2026-11-11, web app 11.228)

The `group_membermaps` documents below are no longer kept current (shard `updateTime`s hours old, people online in the
office listed as `isOnline:false`). The web app's people list now comes from a live query on the **chat socket**
(`wss://chat.sowork.com/ws`), sent after `connection.ready`:

```
OUT {"type":"query.subscribe","data":{"sub_id":"<id>","collection":"members","group_id":"<groupId>",
     "query":{"filter":{"include_apps":true},"sort":["availability","last_seen","name"],"limit":5000},"time_zone":"UTC"}}
IN  {"type":"members.rows","data":{"sub_id","group_id","total","has_more","rows":[row...]}}
IN  {"type":"members.changes","data":{"sub_id","total","removed":[userId...],"upserted":[row...]}}
row = {user_id, name, role, is_app, last_seen_at:<ms>, team_ids, uploaded_photo_url, character_string, title,
       presence:{online, online_since, status, text_status, room_id, zone_id, mode:"room"|"simple"|null,
                 sessions:[{kind:"web"|"desktop"|"mobile", state:"active"|"locked"|"background"}],
                 mobile_online, mobile_reachable, idle, forced_away, in_private_conversation, calendar, version, ...}}
```

An upsert whose `presence.version` is older than the one held is ignored. Verified live: this list matched the browser
exactly, while `group_membermaps` had three people in the office as offline and one long-gone person as online.
soworker: `roster::Live` on the chat socket (`chat.rs`); `soworker --roster` still prints the stale Firestore data.

## Where the list came from (October 2026)

gaia's `room.users` only holds people who are connected right now (2 users in this recording, both `isOnline:true`).
The official client's people list comes from **Firestore** (project `sage-learning-prod`, same Firebase ID token as
everything else):

| Document | Content |
|---|---|
| `group_membermaps/<groupId>_<n>`, n = 0, 1, 2, ... | one shard each (4 shards for 12 members); field `users` is a map `<userId>` -> member |
| `users/<userId>` | profile (`name`, `email`, `affiliation`, `character`, ...), read once per member |
| `conversations` where `ownerGroupId`, `roomDefinitionId`, `status=active`, `onlineCount>0` | live meetings (shape not captured) |

A member inside `group_membermaps`:

```
{ userName, role:"owner"|"contributor", isOnline:bool, status:"online"|"busy"|"away", mobile:bool,
  lastOnline:<timestamp>, inPrivateConversation:bool, inCalendarMeeting:null|"external",
  meetingZoneId?, roomDefinitionId?, mode?:"room" }      // the last three only while online in a room
```

`lastOnline` is current for online users too (it is updated about once a minute). The shard documents change live in the
web client (Firestore `Listen`).

## What the native app does

- `roster.rs` reads `GET https://firestore.googleapis.com/v1/projects/sage-learning-prod/databases/(default)/documents/group_membermaps/<groupId>_<n>`
  with `Authorization: Bearer <ID token>` for n = 0.. until a 404, every 30 s, on a background thread. The group id is
  `groupId` of the saved `gaia/find` body. `soworker --roster` prints the result (uses the saved sign-in).
  **Not yet run against the live service from the development sandbox (no network there): run `--roster` once.**
  If Firestore's rules reject the REST call, the sidebar simply has no offline list.
- Online = in the gaia room (live), or `isOnline` in the roster. A person's meeting is gaia's `meetingZoneId`
  (`room.users[*]`, `s` code 9), else the roster's.
- Sidebar (`sidebar.rs`): availability and log out buttons; **In meetings** (two or more online people in the same
  meeting zone, grouped under the zone's name); **Online**; **Offline** (newest `lastOnline` first). Click an online
  person or a meeting to open a menu: **Join** walks (path-finding, like a click on the map) to the centre of the
  person's meeting zone, else to where they stand; walking into a meeting zone makes gaia start the call as usual.
  **Wave** is a stub (gaia has an `emote {emoteId, recipientUserId, senderUserId}` event, never observed).

## Open questions

- Live updates of the roster: polling every 30 s is a stand-in for the `Listen` stream.
- The `conversations` documents (who is in which meeting, including people in other rooms) were not captured.
- How the official client's "Join" works for someone in a meeting we are not near (walks vs. teleports): we walk.
