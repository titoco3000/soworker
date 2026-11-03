# Raising a hand, going to your desk, changing the profile photo

Source: the recording of 2026-11-03 (`logs/2026-11-03T19-33-44-953Z`, ~240 s: join a meeting, raise and lower the hand,
walk to the own desk, upload a new profile photo). Times are seconds since the recording started. Facts are marked
**[recording]**; guesses are marked **[guess]**.

## 1. Raising and lowering the hand

There is **no gaia event and no REST call**. The hand is part of the LiveKit **participant metadata**, which the client
replaces as a whole with the signal request `update_metadata` (`SignalRequest` field 15, `UpdateParticipantMetadata
{metadata}`) **[recording]**.

The metadata is a JSON string:

```json
{"raisedHandAt": 1793734498692, "audioOnly": false, "allowsScreenDrawing": false, "desktop": false,
 "screenshare": null,
 "portrait": {"url": "https://app.sowork.com/portrait/v1/meeting!idle/characterA!backAccessory:~scroll+green!...",
              "kind": "character"}}
```

| Moment | `raisedHandAt` | |
|---|---|---|
| joining the call (69.2 s) | `null` | sent once right after the LiveKit room connects |
| raise (73.2 s) | Unix time in milliseconds | the moment of raising; [guess] used to order hands |
| lower (75.8 s) | `null` | |

- The whole object is sent every time, so a client that changes one field (the screen share does too) must keep the
  others, or it lowers its own hand by accident.
- `portrait.url` is the same server-rendered character portrait as in `avatars.rs`; `kind` is `"character"`.
- Other participants see the change as LiveKit's `ParticipantMetadataChanged`; a participant who joins later reads it
  from the participant's current metadata.
- gaia saw nothing at either moment, and nothing on the data channels either.

## 2. Going to your desk

Desks are fixed spots on the map; which one belongs to whom is stored in Firestore. "Go to my desk" is the client
walking there with ordinary `move` frames.

**Where the desks are**: the gaia `room` snapshot has `desks`, keyed by desk id **[recording]**:

```
desks: { "<deskId>": { type: "asset", x, y, assetId, orientation: "N"|"E"|"S"|"W" }, ... }
```

The same snapshot gives `roomDefinitionId`, which the ownership document is keyed by.

**Whose desk it is**: the Firestore document `group_desks/<groupId>`, read through the Firestore Listen channel at
start-up (44.0 s), and readable over REST with the user's ID token like `group_membermaps` (docs/members-roster.md):

```
GET https://firestore.googleapis.com/v1/projects/sage-learning-prod/databases/(default)/documents/group_desks/<groupId>
fields.rooms.mapValue.fields.<roomDefinitionId>.mapValue.fields.<deskId>.mapValue.fields.userId.stringValue = <uid>
```

In the recording, 12 of the room's 17 desks had an owner, each a different user.

**The walk** (119.0-121.1 s) **[recording]**: `character_state walk`, `move` about every 100 ms along a path the client
works out itself, `set_meeting_zone` for each zone it enters or leaves on the way, and finally `character_state idle`
with the last `move` exactly on the desk's `x, y`. There is no "go to desk" message. The avatar does not sit down
(`sit` with a `target` is a separate action on a chair).

## 3. Changing the profile photo

The browser uploads the picture to S3 itself, with a presigned POST that SoWork's API hands out **[recording]**.

1. `POST /api/v1/user/getUploadProfilePhotoURL` `{"type":"profile_photo","fileType":"image/webp"}` (178.4 s). Answer:

   ```
   { download: { url: "https://sowork-space.s3.us-east-2.amazonaws.com/profile_photos/<uid>/profile.webp" },
     upload:   { url: "https://s3.us-east-2.amazonaws.com/sowork-space",
                 fields: { key: "profile_photos/<uid>/profile.webp", acl: "public-read", bucket: "sowork-space",
                           X-Amz-Algorithm, X-Amz-Credential, X-Amz-Date, X-Amz-Security-Token, Policy,
                           X-Amz-Signature } } }
   ```

2. `POST https://s3.us-east-2.amazonaws.com/sowork-space`, `multipart/form-data`: `Content-Type: image/webp` first,
   then every field from `upload.fields` in order, then the file. S3 answers `204` (178.7 s).
3. `POST /api/v1/user/update`
   `{"userId":"<uid>","groupId":"<groupId>","data":{"uploadedPhotoURL":"<download url>?v=<ms now>"}}` (180.2 s).
   The `?v=` makes every client fetch the new picture instead of a cached one, since the object key never changes.
   The answer is the whole user document.
4. gaia OUT `info_update {role, affiliation, name}`; the server then pushes `update_info` (with the new
   `uploadedPhotoURL`) and `update` (character config) to everyone in the room (180.8 s).

The web client converted the chosen picture to WebP before uploading. The photo is public (`acl: public-read`).
A later `POST /api/v1/user/update` with only `workingHours` (229.7 s) shows that `data` is a partial update.

## Status in this client

- Raising the hand: implemented (`H`, or the hand button in the meeting dock). Other people's raised hands are shown in
  the call, numbered in the order they went up (1 = first) when more than one is up. The order is by when this client first saw each hand go
  up, on its own clock, not by the other clients' `raisedHandAt`. Each hand that goes up (ours too) plays SoWork's `raiseHand` chime, as its client does
  (`handleMetadataChanged`: plays when `raisedHandAt` becomes set). Speaking lowers our hand by itself, after LiveKit
  has listed us as an active speaker for a second without a break; SoWork's client has no such rule.
- Lowering someone else's hand: implemented (the hand button on their tile, beside mute, while their hand is up).
  SoWork's client sends a reliable LiveKit data message on topic `raise-hand`, `{raised, userId}` (msgpack, like
  `ask-to-mute`); the participant named in `userId` then raises or lowers its own hand. We send and obey it the same
  way. The message format comes from the client code (`raiseHandStore`), not from a recording.
- Going to the desk: implemented (`Ctrl+D`).
- Changing the photo: not implemented; this client only shows photos.
