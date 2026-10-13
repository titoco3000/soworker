# Waves and chat

Source: recording `output/raw-logs/2026-10-07T12-31-41-224Z` (receive a wave and accept it, wave at someone who accepts,
area message both ways, private message both ways) plus SoWork's chat client code in that run's `bodies/42fc3e7686301255.js`.
Facts are **[recording]**, **[client code]**, or **[guess]**.

## Waves

| | |
|---|---|
| Send | gaia OUT `emote {emoteId:"wave", recipientUserId, senderUserId, extras: undefined}` **and** `POST /api/v1/waveToUser/create {"type":"wave_to_user","toUserId":"<uid>"}` -> `{"success":true}` (both at 60.7 s) **[recording]** |
| Receive, live | gaia IN `s [[4, {recipientUserId, senderUserId, emoteId:"wave", extras}]]`; the sender gets its own echo **[recording]** |
| Receive, kept | Firestore `users_sys/<me>/notifications/<id>`: `{type:"wave-to-user", unread, removed:false, createdAt, data:{from:{userId,userName}, to:{...}}}`; the web client listens with `removed == false` ordered by `createdAt` desc, limit 50 **[recording]** |
| Accept / dismiss | `POST /api/v1/notifications/markAs {"notificationIds":["<id>"],"action":"remove"}` (the document then has `removed:true`). Accepting also walks you to the sender: moves start 0.2 s later and end with `set_meeting_zone <their zone>` **[recording]** |
| "They accepted" | nothing is sent back: the other person just walks into your meeting zone (`s` code 9 + `cluster` event for the new call) **[recording]** |

Native: Wave button (person, or everyone else in a meeting) sends both halves. Incoming waves show in the sidebar under
**WAVES** with Accept (walks there like Join, then dismisses) and Dismiss. The list is the Firestore notifications
(`runQuery` on `users_sys/<me>`, polled every 30 s with the roster) plus waves seen over gaia in the last 20 s.

## Chat

Service: `https://api.sowork.com/api/v1/chat-v2/...` (same Bearer ID token, `source: web`, `version` headers) and a
WebSocket `wss://chat.sowork.com/ws?token=<ID token>` (Origin `https://app.sowork.com`) **[recording]**.

| | |
|---|---|
| Channels | `GET /?groupId=<g>` -> `{channels:[{id, type:"direct"\|"general"\|"meeting"..., member_key:"<uidA>:<uidB>", dm_member_user_ids, conversation_id, ...}], users}` **[recording]** |
| Area chat | a **meeting** channel per call (`conversation_id` = the gaia `cluster` event's `conversationId`). Send: `POST /meetings/<conversationId>/messages {content, idempotencyKey}` -> `{message, channel}`; the first message creates the channel and `member.added` follows **[recording]** |
| Private chat | a **direct** channel. Existing one: `POST /<channelId>/messages`. No channel yet: `POST /recipients/messages {groupId, userIds:[uid], content, idempotencyKey}` -> `{channel, message}` **[client code, not seen]** |
| Body | `{"content":{"type":"doc","content":[{"type":"paragraph","content":[{"type":"text","text":"oi"}]}]},"idempotencyKey":"<uuid v4>"}`; messages come back with a plain `text` field too **[recording]** |
| History | `GET /<channelId>/messages?limit=50` **[recording]** (order not checked, we sort by `created_at`) |
| Read | `POST /<channelId>/read {"lastReadMessageId"}` **[recording]** |
| Push | WS frames `message.new {channel_id, group_id, data:{message, users:{<uid>:{name, photo_url}}}}`, plus `read.updated`, `member.added`, `typing.start/stop`, `heartbeat` every 30 s. Client sends `activity.report {idle_seconds, locked, notifiable}` at start and when focus changes **[recording]** |
| Joining an existing call's chat | `POST /meetings/<conversationId>/channel` (`ensureMeetingChannel`) **[client code, not seen]**: we call it when the Area tab opens, and ignore a failure |

Native: right-hand panel (`chatui.rs`) with an **Area** tab (the call we are in) and one tab per private chat, one-line
input (Enter sends, Esc leaves it; clicking the map also leaves it), unread counts on the sidebar's **Chat** button, on
people rows and on tabs, and a toast for a message that arrives while the panel shows something else. The meeting
dock's chat button toggles the same panel. **Message** in a person's menu opens their chat (also for offline people).
Not done: typing indicators, attachments, threads, reactions, multi-line messages, editing/deleting, group chats.

## Open questions

- Whether `POST /meetings/<id>/channel` really is what shows an existing area chat's history; whether the history list
  is newest- or oldest-first.
- `POST /recipients/messages` for a first private message was not recorded.
- Chat WS token expiry: we reconnect with a fresh token on any drop; a socket that outlives its token is untested.
