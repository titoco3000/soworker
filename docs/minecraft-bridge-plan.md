# Minecraft bridge: implementation plan

Status: phases 1–6 built and tested in dry run and in the `--mc-demo` office (see "As built" at the end); the first
live test (phase 4, movement visible to coworkers) is still to do, by the user. This document was the hand-off for the
build; where the implementation settled a detail differently, "Protocol" and "As built" say what is implemented.

## Goal

Show the SoWork office inside Minecraft and let the user take part in it from there:

- The office map is built from blocks: ordinary floor tiles are **grass**, meeting-zone tiles are **cobblestone**, and a
  tile with a wall on its **right (east) or bottom (south)** edge gets a **fence**.
- Every other user is a **named villager that walks** (real pathfinding) to the user's live SoWork position. No armor
  stands, no teleporting except as a fallback.
- The local Minecraft player **is** the SoWork user: walking in Minecraft moves the SoWork avatar, and walking onto
  cobblestone joins that meeting.
- **Voice, camera and screen share all work.** Audio, the microphone, the camera and screen share stay in soworker's
  existing LiveKit code (`src/media.rs`, `src/calls.rs`); Minecraft shows the call's video on map-art screens and
  offers the call controls.
- In a meeting, the **first hotbar slots become special tools**: mute/unmute, screen share, record, camera. Two slots
  are **always** tools: return to your desk, and toggle availability (see "Hotbar tools").

Decisions already made with the user (do not reopen):

- Media stays in soworker (speakers, mic, camera, share through LiveKit); Minecraft only displays video and sends
  control commands. No Simple Voice Chat integration.
- The Minecraft player drives the SoWork avatar (not a spectator).

## Environment

- Minecraft **26.3** (the launcher's latest-release profile; `~/.minecraft/versions/26.3`). Java **25**; the JDK
  (`javac 25.0.4.1`) is installed.
- Mod loader: **Fabric**, installed: Fabric Loader **0.20.1** for 26.3 (`~/.minecraft/versions/fabric-loader-0.20.1-26.3`)
  and Fabric API **0.166.0+26.3** (`~/.minecraft/mods/fabric-api-0.166.0+26.3.jar`). Build the mod against these
  versions (Loom version: look up the one matching this loader).
- Minecraft 26.x ships unobfuscated, so use Mojang names. The planning agent's knowledge predates 26.3: verify every
  Minecraft/Fabric API against the actual jars (decompiled sources from Loom's `genSources`) and fix against compiler
  errors rather than trusting memory.
- Singleplayer: the mod is server-side logic running in the integrated server, installed in the client's
  `~/.minecraft/mods`. A vanilla client renders everything (blocks, villagers, text displays, maps, items).
- Use a dedicated new world (superflat, "void" preset). The mod sets the game rules for it (no day cycle, no mob
  spawning, no weather, keep inventory) when the bridge connects.

## Architecture

```
 SoWork servers  <──  soworker --mc  (Rust, this repo)  ──ws://127.0.0.1:25599──  Fabric mod (Java, mc/)
                       Backend + State as in the TUI,        JSON + binary frames      map, villagers, screens,
                       LiveKit audio/video, no screen                                  hotbar tools, /sw commands
```

- `soworker --mc` is a third frontend next to `tui` and `gui`. It runs the same `Backend` (`src/backend.rs`) and
  `State` (`src/app.rs`) loop as the TUI (`src/tui/mod.rs`, `event_loop`): handle `Event`s (`st.apply`,
  `st.apply_media`, `st.talk`), then `st.update(dt, now)`, then `be.flush(&mut st, start)`, then compare against the
  state last sent and send what changed to the mod.
- The mod is a WebSocket client (Java's built-in `java.net.http.WebSocket`, no extra dependency). Incoming messages
  are queued and applied on the server tick thread.
- The mod lives in `mc/` in this repo (Gradle wrapper, Loom) so the protocol stays in sync with the Rust side.

## Coordinates

- One SoWork tile is 2 × 2 blocks (`geom::SCALE`; first built at one block per tile, doubled after the first live
  test because the office felt cramped). Tile pitch is `world::T` (2 × `world::G` = 210.9375 units); tile index is
  `world::tile(v)`. A solid wall becomes oak fences on the last row or column of blocks of the tile whose E or S edge
  it is on, so the fence stands half a block inside SoWork's wall line; the bridge's follow slides along walls rather
  than routing round them, so a player pressed against the far side of a fence is not sent back.
- Block x = `SCALE × (x / T - min_i)`, block z = `SCALE × (y / T - min_j)`, where (`min_i`, `min_j`) is the smallest floor tile, so the map
  starts at block (0, 0). Floor surface at y = 64 (the floor block is at y = 64, entities stand at y = 65).
- SoWork E (wall bit `world::E`) is +x, S (`world::S`) is +z: "right" and "bottom" match Minecraft directly.
- Positions keep their fraction: a SoWork point maps to a fractional block position (for villager targets and the
  player), not just the tile.

## Map

Built by the mod from the `world` message. The Rust side computes everything (pure functions in `src/mc/geom.rs`, unit
tested); the mod only places blocks.

- Floor tile (`World::floors`): `grass_block` at y = 64.
- Meeting tile: a floor tile whose centre lies in a meeting zone, by the same test as `World::zone_at` (zone half-extent
  `ZK × size`, only zones with `meeting == true`): `cobblestone` instead of grass.
- Fences: start from `World::walls` (solid walls only; `World::soft` walls get nothing). Normalise every wall to the
  E or S edge of some tile: a W wall on (i, j) is an E wall on (i-1, j), an N wall on (i, j) is an S wall on (i, j-1).
  Every tile that then has an E or S wall gets an `oak_fence` at y = 65. Fences connect to each other by themselves.
- Outside the floor: air at floor level, plus a ring of `barrier` blocks at y = 65 (and y = 66) around the floor's
  outer edge so nobody walks off the map.
- Rebuild when `World::rebuild_if_dirty` reports a change (Rust sends a new `world`); the mod diffs against the last
  map and only changes blocks that differ, then clears what is no longer there.
- Known consequence: a fence fills its whole block, so a floor tile with a wall on its edge, walkable in SoWork, is not
  walkable in Minecraft. Movement handles this (see "Player movement").

## Villagers (every user except the local one)

- One villager per user in `State::users` (except `me_id`), tagged `soworker` and keyed by user id.
- Custom name = the user's name, always visible. Invulnerable, persistent (no despawn), silent, no gravity issues.
- Trading blocked: a mixin on the villager's `mobInteract` returns early for tagged villagers.
- Walking: a mixin skips the villager's brain/AI step (`customServerAiStep`, or whatever 26.3 calls it) for tagged
  villagers, so they never wander, work, sleep or panic. Every tick, if the target moved, call
  `getNavigation().moveTo(x, 65, z, speed)` toward the user's latest position (navigation still ticks without the
  brain; verify this in 26.3 and add an explicit navigation tick in the mixin if not). Speed close to SoWork's walk
  speed (`SPEED` in `src/app.rs`, converted to blocks per tick).
- Fallback teleport when the villager is more than 16 blocks from its target, or has made no progress for 3 s (target
  inside a fence block, or no path).
- State shown on the villager:
  - away (`Avatar::away()`): profession nitwit, name in grey;
  - speaking (`Avatar::speaking` / `call.speakers`): note particles over the head and glowing;
  - muted in the call (`call.peer_muted`): a "🔇" suffix on the name;
  - hand raised (`call.hands`): "✋ #n" suffix with the queue position from `meeting::hand_queue`;
  - wave (`Net::Emote` with emote `wave`): heart particles; a wave at us also shows a chat line.
- `Net::Join` / `Net::Leave`: spawn / remove with a smoke puff.

## Player movement (the local player is the SoWork user)

- The mod sends the player's position (`pos {x, z}` in SoWork units, converted back from blocks) 10 times a second.
- The bridge turns it into a walk target: `st.path = Some(vec![(x, y)])`, `st.path_goal = (x, y)`. The existing
  `State::update` then does the rest: SoWork collision (`World::step`), the `Move`/`State` messages at the normal rate,
  meeting-zone entry and leave (`Cmd::Zone`), and therefore joining and leaving calls.
- Leaving a meeting zone normally opens a confirmation (`State::confirm_leave`, set by `ask_leave`). Walking out in
  Minecraft counts as confirming: when `confirm_leave` becomes `Some`, call `st.leave_choice(true, now)`.
- Divergence: if the SoWork position stays more than 2 blocks away from the player for over 1 s (SoWork blocked a move
  Minecraft allowed), the mod teleports the player to the SoWork position.
- Walks started by soworker (the start-up walk to the desk, `walk_to_desk_at_start`; the "return to desk" tool): while
  `st.path` was set by soworker rather than by the player, send `follow {x, z}` updates and have the mod move the player
  along smoothly (small per-tick teleports with the player's facing kept) and ignore the player's own `pos` until the
  walk ends or the player presses a movement key (detect by the player moving away from the follow point).
- `--dry-run` must keep working: nothing is sent to SoWork. Develop and test in dry-run. **Live movement is visible to
  the user's coworkers: do the first live test only after the user says so.**

## Hotbar tools

Hotbar slots are numbered 1–9 (Minecraft inventory indices 0–8).

| slot | when | tool | action (existing `State` method) |
|---|---|---|---|
| 1 | in a meeting | Microphone: mute / unmute | `toggle_mic` |
| 2 | in a meeting | Screen share: start / stop | `toggle_share` |
| 3 | in a meeting | Record meeting: start / stop | `toggle_record` |
| 4 | in a meeting | Camera: on / off | `toggle_cam` |
| 8 | always | Return to desk | `go_to_desk` |
| 9 | always | Availability: available / away | `set_available(!st.available)` |

- "In a meeting" means `call.joined.is_some()` (we are in the call, not merely standing in the zone). The `call`
  message carries the flags; the mod adds and removes slots 1–4 when it changes.
- Item choice, on/off shown by item and name, with an enchantment glint when the feature is on:
  - mic: on = `lime_dye` "Microphone: on (use to mute)", off = `gray_dye` "Microphone: muted (use to unmute)";
  - share: `spyglass` "Share screen" / glint + "Stop sharing";
  - record: `red_dye` "Record meeting" / glint + "Stop recording (mm:ss)" using `State::rec_clock`;
  - camera: `ender_eye` "Camera: on" / `ender_pearl` "Camera: off";
  - return to desk: `compass` "Walk to your desk";
  - availability: `emerald` "Available (use to go away)" / `redstone` "Away (use to become available)".
  Use item components (custom name, glint override, lore with the action) and mark every tool with a custom data tag
  (`soworker_tool: "<name>"`) so the mod recognises it. Check the 26.3 component names.
- Using a tool = right click (`UseItemCallback`, also `UseBlockCallback`/`AttackBlockCallback` so it works when aiming
  at a block). The mod sends `cmd {tool}`; the bridge calls the method; the next `call`/`me` message updates the item.
  Never toggle the item locally, only reflect state from soworker.
- Tools cannot be dropped, moved, put in containers or lost on death: cancel drop and inventory clicks on tool items
  (mixins on the drop action and the inventory click handling), and re-assert the slots every second in case anything
  slipped through. Tools are removed from anywhere else in the inventory.
- The player's own items in those slots are not destroyed:
  - slots 8 and 9 are reserved from the first connection: items found there are moved to free inventory slots (or
    dropped at the player's feet if full, with a chat note);
  - slots 1–4: when a meeting starts, their items are saved and restored when it ends. Save them in the player's
    persistent mod data (an attachment / saved data, not memory only) so a crash or quit mid-meeting does not lose
    them; restore on the next join if no meeting is active.
- "Return to desk" leaves a meeting if the desk is outside it: `go_to_desk` starts a walk, `confirm_leave` opens, and
  the bridge confirms it (same rule as walking out). The player is moved along (see "follow" above). If there is no
  desk, `State::say` gives a toast; show it in chat.
- The rest of the hotbar (5–7) stays the player's.

## Calls: everything else

- Audio: soworker plays the call and sends the microphone as now. The global push-to-talk key (`src/talk.rs`, portal
  shortcut) keeps working while Minecraft has focus.
- `/sw` commands mirror the tools and add what has no slot: `/sw mic|share|rec|cam|desk|away|hand|leave`,
  `/sw wave <name>` (`Cmd::Wave`), `/sw say <text>` (room chat through `st.chat_out`), `/sw status`.
- Meeting panel, only for the meeting we are in: a `text_display` board at the zone's north edge with the zone name,
  participants with mic/camera icons, the hand queue (`meeting::hand_queue`), and the recording clock. Removed when we
  leave.
- Toasts (`State::toast`), incoming waves and chat become chat lines (`tellraw` style system messages).

### Video on map art

- One screen per video in our call (`call.tiles`, each a `Tile { key, name, img: Frame }`): glow item frames with
  filled maps on a black concrete backing wall, standing above the zone's north fence row, facing into the room. Name
  label (`text_display`) over each screen.
- Size: a camera gets 2×2 maps (256×256 px), a screen share 4×3 maps (512×384 px). Lay screens side by side; if they do
  not fit the zone's width, the share comes first, then cameras sorted by name. Our own camera and share are included.
- Rust side: on `MediaEvent::Video` with a frame (`w > 0`), scale it to the screen size (letterboxed), convert to
  Minecraft map colour indices (the map palette table in `src/mc/palette.rs`; a 32×32×32 RGB lookup table built once;
  optional ordered dithering), and send it as a binary message. Cap at 8 fps per screen; drop frames when the socket is
  busy. `MediaEvent::VideoGone` sends `video_gone`.
- Ask for video with `crate::calls::want_video(None)` (all tracks) and `crate::calls::set_video_width(640, 640)`
  (as the TUI's `sync_wanted` does), after a meeting starts.
- Mod side: one map id per 128×128 piece, allocated once per screen and reused. Write the pixels into the map's saved
  data and mark it dirty so vanilla map update packets go out. Lock the maps so nothing else redraws them.

## Protocol

`ws://127.0.0.1:25599` (`--mc-port` changes it), soworker is the server and listens on localhost only. The first
message from the mod is `{"t":"auth","token":"…"}` with the token from `mc-token` next to the session file
(`~/.config/soworker/mc-token`; random, created by soworker with `0600` permissions); anything else closes the socket.
One client at a time: a newly signed-in client replaces the old one. On (re)connect soworker sends the full state.

**All coordinates on the wire are blocks** (fractional, two decimals), already converted by `src/mc/geom.rs`; the mod
never needs `T` or the origin. Tile lists are integer block (x, z) pairs.

soworker to mod, JSON text messages (`t` = type); each kind is only sent when it differs from the last one sent:

- `hello {me, name, room, dry}`
- `world {origin:[min_i,min_j], floors:[[x,z]…], meeting:[[x,z]…], fences:[[x,z]…], barriers:[[x,z]…], zones:[{id,name,i0,j0,i1,j1}]}`:
  `floors` = grass (meeting tiles not included), `meeting` = cobblestone, `fences` = oak fence at y = 65 (may lie just
  outside the floor), `barriers` = the ring around the floor (barrier at y = 65 and 66; never on a fence tile). A zone's
  box is the inclusive block range of its meeting tiles. Sent again whenever the world is rebuilt.
- `users {list:[{id,name,x,z,walking,away,speaking,muted,hand,zone}]}`: at most 10/s. `x, z` is where the user is
  heading (their latest SoWork position), so the villager walks there by itself. `hand` = queue place or null.
- `me {x, z, available, desk}`: at most 10/s.
- `teleport {x, z}`: put the player here (on connect, and when the player and the avatar drift apart: more than 4
  blocks at once, or more than 2 blocks for over 1 s). Until a `pos` within 1.5 blocks of the avatar arrives, soworker
  ignores `pos` and repeats the teleport every 2 s. This replaces the mod-side divergence snap-back.
- `follow {x, z}` / `follow_end`: a walk soworker started (desk at start-up, `desk` tool); while it runs `pos` is
  ignored unless it carries `input: true` or is more than 1.5 blocks from the avatar.
- `call {joined: zone id or null, in_call, zone_name, mic, cam, share, rec, rec_clock, hand, hands:[ids], participants:[{id,name,mic,cam,speaking}]}`
- `screens {list:[{key, name, kind: "camera"|"share", w, h}]}` (share first, then cameras by name) and `video_gone {key}`
- `say {text}`: toasts, waves, incoming chat lines, command replies

soworker to mod, binary video frame: `[u8 1][u16 key length][key bytes][u16 width][u16 height][width × height palette indices]`,
little-endian. Indices are packed map colours (`base << 2 | brightness`, never 0–3), at most 8 frames/s per screen;
a frame not yet written is replaced by a newer one of the same screen.

mod to soworker:

- `auth {token}`
- `pos {x, z, input?}` (blocks, 10 times a second; `input: true` when the player pressed a movement key)
- `cmd {name: "mic"|"share"|"rec"|"cam"|"desk"|"away"|"hand"|"leave"|"wave"|"say"|"status", arg?}`

Testing without Minecraft or SoWork: `soworker --mc --mc-demo` serves a made-up office (meeting room with walls and
doors, three coworkers, one walking a loop, a desk to walk to) with no SoWork connection at all;
`python3 analysis/scratch/mc_client.py --map` connects, draws the block map as text and prints every message; type
`pos X Z`, `pos! X Z` or a command name on its stdin. `--frames DIR` saves video frames as PPM. `--mc-no-dither` turns
the ordered dithering off.

## Rust changes

- `Cargo.toml`: feature `mc` (add to `default`). No new crates: `tungstenite` (server handshake) and `serde_json` are
  already dependencies. Video encoding only with `calls`.
- `src/main.rs`: `mod mc;` behind the feature; handle `--mc` in `main` before `frontend` (it is not a `UiMode`); let the
  `compile_error!` frontend check accept `mc`.
- `src/mc/mod.rs`: `run(args)`: the headless loop (copy the shape of the TUI's `run` and `event_loop`: options
  `--dry-run`, `--session`, log to file unless `SOWORKER_LOG_STDERR`), sign-in with `Backend::begin_login` when there
  is no saved session, change detection and message sending, applying `pos`/`cmd` from the mod.
- `src/mc/ws.rs`: the WebSocket server thread (accept, auth, reader thread for incoming messages into the main loop's
  channel, writer with a bounded queue that drops video frames first).
- `src/mc/geom.rs`: origin, meeting tiles, fence derivation, SoWork↔block conversion. Pure, unit tested.
- `src/mc/palette.rs`: map colour table, lookup table, scaling, frame encoding. Unit tested.
- `State` gains nothing it does not need; if a method has to become `pub` (for example `leave_choice` is already public,
  `leaves_zone` is not), keep the change minimal.

## Mod layout (`mc/`)

- `build.gradle`, `settings.gradle`, `gradle.properties` (Minecraft 26.3, current Loader, Loom and Fabric API versions:
  look them up), Gradle wrapper (Gradle 9.6.1 is in `~/.gradle/wrapper/dists`).
- `SoworkerMod`: entry point, `/sw` command tree, server tick hook.
- `Bridge`: WebSocket client, reconnect every 5 s, incoming queue drained on the tick, outgoing `pos` at 10 Hz.
- `MapBuilder`: blocks from `world`, diffing, barriers, game rules.
- `Villagers` + `VillagerMixin`: spawning, names, state effects, navigation, trading block.
- `Tools` + inventory mixins: hotbar tools, use handling, drop/move protection, saved slot contents.
- `Screens`: map allocation, item frames, frame writes. `Panel`: meeting text displays.
- `make mc` in the root `Makefile`: build the jar and copy it to `~/.minecraft/mods`.

## Phases (finish and check each before the next)

1. Rust bridge and protocol: `soworker --mc --dry-run` with a small Python WebSocket client in `analysis/scratch/` that
   authenticates and prints messages. Unit tests for geometry, palette and frame encoding. `make test` passes.
2. Mod connects and builds the map: grass, cobblestone, fences, barriers. Compare with the TUI map of the same room.
3. Villagers walk to live positions with names and state effects.
4. Player movement in dry-run: zone detection, automatic leave confirmation, divergence snap-back, desk follow. Then
   one short live test, only when the user agrees.
5. Hotbar tools and `/sw` commands, meeting panel, speaking/mute/hand indicators.
6. Video screens: camera and share, fps and dithering tuning.

## Risks and notes

- Minecraft 26.3 and Fabric APIs are not known to the planning agent: decompile and check names, expect compile
  rounds.
- Fence blocks are coarser than SoWork's edge walls; wall-side tiles are lost in Minecraft (handled by divergence
  snap-back and villager fallback teleports).
- Map art is low resolution at about 8 fps: screen-share text is hard to read from a distance.
- This is an unofficial client: the bridge must not send anything the TUI would not (same `Cmd`s, same rates). Keep
  `--dry-run` honoured everywhere.
- Commits: only when the user asks; end messages with the attribution line from the session instructions.

## As built

- **Build without Gradle/Loom.** 26.x is unobfuscated, so the mod compiles with plain `javac` against the installed
  game jar, the launcher's libraries, Fabric Loader/Mixin and the Fabric API modules (unpacked from the jar in
  `~/.minecraft/mods`). `python3 mc/build.py` (offline) builds `mc/build/soworker-mc.jar`; `make mc` also copies it to
  `~/.minecraft/mods`. Sources: `mc/src/main/java/dev/soworker/mc/`.
- **Opt-in per world.** The mod does nothing in a world until `/sw enable` (stored in `<world>/soworker-office.json`,
  with the list of blocks it placed so a later map change can clear them). Enabling sets the game rules, puts a
  survival player in adventure mode, force-loads the map's chunks and asks soworker to `resync`. `/sw disable` removes
  villagers, screens, the board and the tools (blocks stay). Use a new superflat "void" world.
- **Entities are not saved state.** Villagers, map frames, labels and the board carry the `soworker` tag and are made
  from soworker's state each run; tagged entities loaded from the world that this run did not make are discarded.
- **Mixins:** only the villager brain skip (`VillagerMixin`) and accessors for text displays and item frames.
  Trading is blocked with `UseEntityCallback`. Tool protection is done without mixins: every second (and on every
  state change) the hotbar is made to match, tools anywhere else and on the cursor are removed, and dropped tool items
  near the player vanish. The player's slots 1–4 are kept in a persistent Fabric attachment during a meeting.
- **Extra messages:** mod to soworker `resync` (send the full state again; sent on `/sw enable` and when the player
  joins); soworker to mod `emote {from, to, emote}` for waves (hearts over the villager).
- **Board:** a text display at half size on the zone's north edge (zone name, recording clock, participants with
  mic/camera, raised hands). **Screens:** glow item frames with locked maps on black concrete from y = 67, at the
  zone's north edge facing south; map updates are sent to nearby players directly after each frame (vanilla only
  refreshes framed maps every 10 ticks).
- **Testing:** `soworker --mc --mc-demo` also fakes a call: walking into the demo meeting room joins it (Bo is there
  with a raised hand, a camera and a screen share as moving test pictures), mic/camera/share/record answer like the
  media thread, walking out or `leave` ends it. `DevShots` (client side) saves a screenshot for each
  `<dir>/<name>.req` file, only when the game runs with `-Dsoworker.shotdir=<dir>`; the test scripts use it.
- Checked in a real 26.3 client against a Fabric dedicated server: map, villagers walking, desk walk carrying the player,
  divergence teleport, tools and slot save/restore, meeting board, video screens, auto leave on the desk tool; and the
  real office in `--dry-run`.

### After the first live tests

- Two blocks per tile (see Coordinates); the player walks at Minecraft's own speed and the avatar (7.6 blocks/s) always
  keeps up.
- Meeting room floors: `geom::MATERIALS`, most wanted first (cobblestone, stone bricks, polished andesite, …). Zones are
  taken in id order and each gets the first material no zone touching it (tiles next to each other, corners
  included) already has. The `world` message carries `materials` and each meeting block as `[x, z, material index]`.
- Desk tool: the walk starts in SoWork as before; when the floor blocks connect (`geom::rail_route`, a four-way BFS
  that keeps off fences) the bridge also sends `{"t": "rail", "path": [[x, z], …]}`. The mod lays rails with explicit
  shapes, seats the player in a tagged minecart and pushes it along at ~7.6 blocks/s; while riding it sends no `pos`
  and does not apply `follow`. At the end (or Shift, or no progress for 3 s) the player gets off and the cart and
  rails go. No route: an ordinary carried walk.
- Island: the mod looks for water a little way off the map. When there is a sea, every map block (floor, fences,
  barrier ring) stands on dirt down to the sea bed (at most 64 blocks), with a 6-block sand beach shelving one block
  per two. Removed blocks below the sea surface turn back into water. A world for it: Superflat, custom preset
  `minecraft:bedrock,117*minecraft:stone,2*minecraft:sand,7*minecraft:water;minecraft:ocean` (water up to y = 62,
  two below the office floor).

### Waves, chat, faces, room names

- **Snowballs are waves.** A snowball the player throws at a coworker sends `cmd {name: "wave_id", arg: <user id>}`;
  soworker waves as the sidebar's Wave button does, at most once per user every 5 s (`WAVE_COOLDOWN`). Every wave
  soworker hears of that is not ours (`emote`) makes the waver throw a snowball at its target: the player when it is
  for us, else the target's figure. Those snowballs carry the `soworker_wave` tag, never hurt or push, and land as
  hearts. A wave to us always gives a "X waved at you" chat line (State only toasts the first unanswered one).
  `--mc-demo` fakes a wave every 15 s (Cy at us, Ana at Bo).
- **Chat.** Plain Minecraft chat stays in Minecraft and also goes to the SoWork room chat (`cmd chat`, no echo);
  `/sw say` sends and echoes the line.
- **Room names.** A text display with each meeting room's name floats over the room's middle (`ZoneLabels`), except
  over the room the player stands in.
- **Figures and camera faces.** Each coworker is a `Mannequin` (player model, default skin, description hidden,
  immovable) moved every tick onto an invisible, brainless villager that still does the pathfinding. Cameras are no
  longer wall screens (only screen shares are): soworker sends each camera as a 64×64 RGB square (the middle of the
  picture, up to 10 frames/s) in a binary message of kind 2 (`[u8 2][u16 key length][key][u16 w][u16 h][w*h*3]`,
  key = user id, or `me` for our own). The mod sends it to clients as a `soworker:face` payload with the entity id
  (the mannequin, or the player for `me`); the client keeps a `DynamicTexture` per entity and `FaceLayer` draws it
  over the front of the head of anything drawn by `AvatarRenderer` (mannequins and players, so our own face shows in
  third person). Someone with a live face turns head (and body past 70°) toward the viewer (`AvatarRendererMixin`).
  A face with no frame for 3 s is dropped. Needs the mod on the client; a vanilla client sees plain mannequins.
- **Away crouches.** While we are away (`me.available == false`) the mod sends a `soworker:away` payload and the client
  holds sneak for the player (`KeyboardInputMixin`, sprint off), matching away coworkers, who crouch. Cleared on
  `available`, `/sw disable`, a lost soworker connection, or leaving the world.
- **Spyglass and snowballs.** Hotbar slots 6 and 7 always hold a spyglass and one snowball (given back the tick
  after it is thrown, under a 1 s item cooldown, as the ender pearl), protected like the tools but used the vanilla way (`Tools.plain`). Items found there are moved, as
  for slots 8 and 9; only slot 5 is left to the player.
- **Screen share full screen.** A snowball on a share's wall (a map frame or the black backing) sends
  `cmd {name: "watch", arg: <key>}` and opens `client.ShareView` on the player's client (`soworker:watch` payload,
  `OPEN`). While a share is watched soworker asks for it up to 1920 wide (otherwise 640) and sends JPEGs (quality 88, at
  most 8/s) as binary kind 3 (`[u8 3][u16 key length][key][u16 w][u16 h][JPEG]`); the mod passes them on as `FRAME`
  payloads (registered large, up to 8 MB) and the client draws them letterboxed over the whole window. A movement key,
  jump, sneak or Esc closes the view and sends `soworker:unwatch`, so soworker sends `cmd unwatch` and goes back to 640.
  When the share ends soworker sends `{"t": "watch_end"}` and the view closes.
- **Ender pearls teleport.** Hotbar slot 5 always holds one ender pearl (given back at once; vanilla's one-second
  cooldown applies), so only slots 1–4 outside meetings are left to the player. Where the player's pearl lands, the mod
  sends `cmd {name: "pearl", arg: "x z"}` (blocks) instead of the vanilla teleport (`EnderPearlMixin`); soworker does
  SoWork's Shift+click teleport there (leaving a meeting without asking, joining one by landing in it) and sends
  `teleport` to put the player where the avatar is. A spot off the floor: "You cannot teleport there" and the player
  goes back. With the office off, pearls are vanilla.
