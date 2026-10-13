# Member list (online and offline) and the sidebar

Source: recording `output/raw-logs/2026-10-07T11-54-43-458Z` (the first one with Firestore streams captured, see
`output/record.mjs`, `res.chunk` events). Facts are **[recording]** unless marked otherwise.

## Where the list comes from

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
