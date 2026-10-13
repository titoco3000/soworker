# SoWork client, map format and object model

Scope: client application structure, office map/world/object data, avatars, client-side movement.
Companion docs: `gaia-protocol.md` (socket wire format), `auth-and-rest.md`, `media-livekit.md`.
Source: recording `output/raw-logs/2026-10-06T12-48-44-206Z` (client version 11.202.0, server `g5`).
No secret values are reproduced here. Bundle names are the hashed Vite chunk names from `app.sowork.com/static/js/`.

Legend: **[F]** fact seen in code or capture (bundle named), **[G]** guess or inference, **[?]** unknown.

Helper scripts (all under `analysis/scratch/`):

| script | purpose |
|---|---|
| `ws_msgpack.py` | decodes the full binary socket.io (notepack msgpack) frames from `events.jsonl` (the `ws/*.txt` dumps truncate bin fields to 48 bytes, so they are NOT enough to decode objects) |
| `decode_obj.py` | protobuf decoder for `PbSetMapObject` (the `b_set_obj` payload); the schema is in its docstring |
| `index_bodies.py` | url -> body file index for `bodies/` |

---------------------------------------------------------------------------------------------------

## 1. Client architecture

### 1.1 Stack summary

| layer | finding | evidence |
|---|---|---|
| UI framework | React 18.3.1, react-router, MUI (`material-*.js`, 500 KB), MobX 6 for all state (`es-CTj06141.js` is mobx; stores use `@observable`/`makeObservable` decorators via `decorate-*.js`) | `react-dom-BvUoaieB.js`: `version:"18.3.1"`; stores like `class ... extends Be{... y([j],X.prototype,"field")}` in `main-DD7sSLvE.js` **[F]** |
| Build | Vite (Rolldown-style naming `chunk-D8_bUsfh.js`, `__vite__mapDeps`, `preload-helper`), ~690 first-party chunks, ESM, sourcemap comments present but `.map` files not captured | **[F]** |
| Realtime to server | socket.io-client v4 (EIO=4) with a msgpack parser (notepack style) to `api.sowork.com/gaia/<serverId>/v2/`; ext type 0 with 1 byte = `undefined` | `main-DD7sSLvE.js` `parser:Y4e.default`, `transports:["websocket","polling"]`; decoded frames **[F]** |
| Object wire format | protobuf-es (`@bufbuild/protobuf`, proto3) messages `PbSetMapObject`/`PbMapObject`/`PbColorVariation` | `httpClientFactory-CXSNSIW-.js` module `Yu` **[F]** |
| REST/auth | Firebase Auth + Firestore (project `sage-learning-prod`, plus a second Firestore project for calendar), custom REST at `api.sowork.com/api/v1` | see `auth-and-rest.md` |
| A/V | LiveKit (`livekit-BeJ_jwe9.js`, `livekit-client.esm-*.js`), loaded lazily on entering the office | `media-livekit.md` |
| Telemetry/3rd party | Sentry, Amplitude (+session replay), Intercom, GA/GTM, Facebook pixel, Sanity (only a "releases" banner query, `uy7tlymu.apicdn.sanity.io`) | http.tsv **[F]** |
| **World renderer** | **Not PixiJS/Three/Phaser.** A custom Rust engine compiled to WebAssembly, internal name **"Hephaestus"** (`hephaestus-Km_jPPAu.js` = wasm-bindgen glue, `hephaestus_bg-BHyKr5n5.wasm`), drawing to a WebGL2 canvas | see 1.3 **[F]** |
| Engine facade | `isInputActionMovement-DiROaRkY.js` ("gameEngine broker"): maps app commands (`character:createOrUpdate`, `map:createOrUpdateObject`, ...) to wasm calls and wasm events back to app events | **[F]** |

The strings `THREE` (1x in main) and `PIXI` (0) are incidental (a Three-looking constant in an unrelated lib); no 3D engine is bundled. `Phaser` absent.

### 1.2 Bundles by size and purpose (first-party, uncompressed bytes)

Total first-party JS observed: **~24.5 MB** across 689 chunk files; transfer size is smaller (gzip/br) but is not in the log. The recorder had the HTTP cache disabled.

| file | bytes | purpose (identified by content) |
|---|---|---|
| `livekit-BeJ_jwe9.js` | 6,112,631 | LiveKit core + bundled processors (WASM/OffscreenCanvas strings, protobuf); **lazy**, loaded at office entry (t=73.6s) |
| `main-DD7sSLvE.js` | 4,155,312 | App core: socket.io client, GaiaRoomConnection, room/user stores, MobX stores, Firebase glue, commands (23k lines when split) |
| `lib-dakVzArj.js` | 983,284 | shared lib (firebase/livekit/util mix) |
| `httpClientFactory-CXSNSIW-.js` | 764,342 | REST clients, **protobuf-es + `PbMapObject` schema**, character config lib (`toAssetLayerPath`, `parseConfigString`), hue/saturation GLSL |
| `Logo100px-Cjj0mjh9.js` | 725,531 | inlined image/SVG assets |
| `UserBio-JYfR6TjN.js` | 571,429 | `CharacterMovement` controller (keyboard/click-to-move), profile UI |
| `en-D6zeki-W.js` | 569,432 | English i18n strings |
| `chunk-K2UTITRG-BUSmNyu8.js` | 542,932 | misc vendor (canvas/text/pdf helpers) |
| `crypto-browserify-DwyMmfky.js` | 509,905 | node polyfills |
| `livekit-client.esm-BqW1jPQq.js` | 508,646 | livekit-client (lazy) |
| `material-BSCm59Q5.js` | 501,724 | MUI |
| `dist-CAUp2EqB.js` | 493,418 | vendor (dnd/selection etc.) |
| `BlockingComponentLock-*.js` | 416,981 | UI |
| `hephaestus-Km_jPPAu.js` | 251,617 | **engine wasm-bindgen glue** (lazy, t=75.3s) |
| `inWorldMetadata-X7YHqDMr.js` | 217,840 | in-world interaction handlers (door, gong, portal, embed, whiteboard, ...) |
| `isInputActionMovement-DiROaRkY.js` | 137,364 | engine broker (`dispatchGameCommand`, `toGameEvent`) |
| `WASM-CErjq-cP.js` | 4,384 | **engine bootstrap**: builds `MainOptions` and calls `init` |
| `AssetDownloadManager-C5N7JjwB.js` | ~40 K | throttled fetch queue + `maps/psd/map_asset_config.json` + `characters/v5/*` loaders |
| `DeskThumbnail-Cx2o-Aiq.js` (also `0508c535c9a73427`) | ~27 K | `MapAssetsStore` ("Eyekia" client): floors/walls/assets catalog fetchers |

Non-JS: `hephaestus_bg-BHyKr5n5.wasm` **11,119,200 bytes** (gzip transfer size from `content-length`, body not captured because Chrome evicted it), `main-lfiWnj-T.css`, 5 woff2 fonts, ~70 short mp3 UI sounds (`/static/media/*.mp3`, fetched eagerly at start).

### 1.3 Startup vs lazy

| phase | JS loaded |
|---|---|
| landing/login page (t=0..3s) | **679 first-party chunks, 17.5 MB uncompressed** (everything is preloaded, including chat PDF preview, i18n, date pickers, ...) |
| second load after OAuth redirect (t=26s) | the same set again (cache disabled in recording) |
| entering the office (t=73..75s) | 9 more files, 7.0 MB: livekit (6.1 MB + 0.5 MB), `hephaestus` glue (0.25 MB), `WASM-*` bootstrap, QualityMetricsCollector; plus the 11.1 MB wasm |
| data JSON at start | `characters/v5/customizer_config.json` 1.04 MB, `character_art_database.json` 1.52 MB, `customizer_product_prices.json` 19 KB, `eyekia/assets-version` 147 KB, asset category list 1.56 MB (seating category); the character JSONs are fetched 3 times in the recording |

### 1.4 The engine ("Hephaestus"): why it is heavy

**[F]** facts (glue `hephaestus-Km_jPPAu.js` and `WASM-CErjq-cP.js`):

* wasm-bindgen module (`__wbindgen_free`, `__externref_table_alloc`), Rust. A control-flow exception message `"Using exceptions for control flow, don't mind me. This isn't actually an error!"` is swallowed in `WASM-*.js`; that is the winit-on-web idiom for `EventLoop::run`.
* Renders with **WebGL2 only** (`instanceof WebGL2RenderingContext`, ~100 `gl.*` imports; no `navigator.gpu`). It owns a canvas inside `#canvasParent` (`WebOptions.canvas_parent_id`), plus `ui_containers` (DOM overlay roots).
* Event loop frame-limited by `setFPS`: the app presets `pause:0, low:15, default:30, high:60` (`WorldFpsLimiter-CQw9kKXU.js`). When paused the client also sends `should_update {v:false}` so the server stops streaming `s` batches (`main`, `connection.shouldUpdate`).
* Rendering layers (RenderingOptions): background, floors, walls, zones, interactive and non-interactive furniture, characters, shadows, lights; `RenderQuality` Low/Medium/High; downscaling per asset class (MapBackground, Furniture, Animation, Wall, Floor) driven by `assetQuality` (`noGraphics`, `lite`, `m3`, `m2`, `m1`, `m0`).
* Physics/collision, **pathfinding (nav map)** and movement simulation all run inside the wasm (`PhysicsOptions`, `PathfindingEvent`, `NavMapChanged`, `CreateOrUpdateCharacter.ccd_enabled/collider_multiplier`).
* Images are fetched by JS on behalf of wasm: `js_callbacks.fetchBitmap(url)` -> `fetch` -> `createImageBitmap`; `fetchData(url)` for JSON; `getAssetDetailsWasm(id,type)` for asset metadata. A `new Worker(...)` import exists in the glue (likely wasm task pool).

**[G]** the engine is probably Bevy/wgpu(GL backend) + a Rust physics crate (rapier-style names `ccd_enabled`, `collider`): nothing in the JS names the crates. Every sprite is composited on the GPU; the heaviness comes from (a) ~1,400 map objects each made of several layered images (furniture) plus floor/wall tile images, (b) up to ~13 layered sprite atlases per avatar (per slot per animation), recolored by shader, (c) shadows/lights passes, (d) 11 MB wasm to compile.

---------------------------------------------------------------------------------------------------

## 2. World / map format

### 2.1 Big picture

The office is **not a tile array and not a skybox-only scene**. It is:

1. A **map background** identified by `mapId` (`skyboxSunset01`): only a backdrop image/skybox. The client sends it to the engine with `map:setBackground {mapBackgroundId: mapId}` (`main`, `handleRoomInit`). Its pixel bounds come from `mapBounds` or, if undefined (as in this capture), from a built-in table by mapId (2.4).
2. A flat dictionary of **map objects** (`b_set_obj` stream, 1,473 objects here): floors, walls, assets (furniture), zones. Each references a catalog entry by `assetId` in the **Eyekia asset service** (REST) which holds the images and the collision shapes.
3. The `room` event: room identity, spawns, spawnAreas (meeting zone anchors), users.

So floor plan = floor objects (806 tiles), walls = wall objects (276), furniture = asset objects (371), meeting rooms = zone objects (20). **There is no separate collision or navmesh file**: collision shapes come from the asset catalog (`primitives` of type `shape/collider`), and the nav map is built by the engine client-side from objects + colliders **[F for source of shapes, G for the engine building the navmesh]**. The server does not validate position (see 5).

### 2.2 Order of loading on entry (capture timestamps)

| t | step |
|---|---|
| 73.6 | `POST /api/v1/gaia/find` (resolve alias -> room, serverId `g5`, server url), `POST /api/v1/gaia/rooms` |
| 73.8 | `GET /eyekia/api/v1/category/` and `GET .../asset/?categoryId=...&pathOnlyUrls=true` (catalog of a category, 1.5 MB) |
| 75.2 | engine glue + wasm fetched; socket.io connect (`roomId`, `mediaRegion`, `clusterMode`, `x`,`y` of last position, `joinType=force`) |
| 78.0 | server `init`; client replies `should_update {v:true, t:0, lastObjectId:""}` (t = max `updatedAt` of known objects, so a reconnect only receives newer objects: second connection at 133s received **0** objects) |
| 78.15 | `room` event (map definition + users) |
| 78.2-79.4 | 1,473 `b_set_obj` frames, 178,764 payload bytes total, 568 bytes max |
| 79.27 | client fetches per-id catalog JSON: `/eyekia/api/v1/floor/<id>?pathOnlyUrls=true&v=<ver>` and `/wall/<id>`, `/asset/<id>` (versions from `/eyekia/api/v1/assets-version`, 146 KB list `{assets:[{id,v}],floors,walls}`) |

### 2.3 `room` event payload (server -> client, event name `room`)

Redacted example (users trimmed, ids shortened):

```json
{"id":"YmAf…IfV","name":"<room display name>","serverId":"g5",
 "roomDefinitionId":"yPsv…","roomDefinitionName":"<name>","roomDefinitionAlias":"mistymountainlodge",
 "roomType":"room","roomObjects":{},
 "mapDefinitionId":"V1vA…","mapId":"skyboxSunset01","mapBounds":undefined,
 "spawns":[{"x":3260.119,"y":979.618,"objectId":"mWVF…"}],
 "customizable":true,"customizableStructures":true,"projection":"isometric",
 "ownerKey":"group_<groupId>",
 "users":{"<uid>":{ …user… }},
 "channels":undefined,
 "spawnAreas":{"<zoneObjectId>":{"tags":undefined,"x":527.25,"y":3268.95}},
 "guestAreas":{},"npcs":{},"games":{}}
```

| field | type | meaning |
|---|---|---|
| `id` | string(20) | room instance id (a room definition can have several instances; `gaia/listRoomByRoomDefinition`) |
| `serverId` | string | gaia shard (`g5`); socket URL = `https://api.sowork.com/gaia/<serverId>` from `gaia/find` -> `room.server.url` |
| `roomDefinitionId/Name/Alias` | string | persistent definition; alias used in invite URL `/s/<groupId>/<alias>` |
| `roomType` | string | `"room"` (also `"tank-arena"`) |
| `roomObjects` | map | **legacy** static objects (signs); empty here. Newer objects arrive via `b_set_obj`. `room_object_update` event applies diffs `{status:created|updated|deleted, objectId, data}` |
| `mapDefinitionId` | string | id of the map definition document (server side) |
| `mapId` | string | background/skybox id, `skyboxSunset01` |
| `mapBounds` | `{center:[x,y],size:[w,h]}` or undefined | here **msgpack ext(0)=undefined**; client falls back to `getLegacyMapBoundsFromMapId(mapId)` (`lib-dakVzArj.js`) |
| `spawns[]` | `{x,y,objectId}` | spawn points; `objectId` is an asset object with `interaction.type == "spawn"` at the same x,y |
| `customizable`, `customizableStructures` | bool | map is editable; if false the client logs "legacy map" |
| `projection` | `"isometric"` (or `"orthographic"`; `isOrthographic` getter) | camera/render projection; default isometric if absent |
| `ownerKey` | string | `group_<groupId>` |
| `spawnAreas` | `{<zoneObjectId>: {x,y,tags}}` | one entry per **meeting zone** object, giving its centre; keys equal the `meetingZoneId` values |
| `guestAreas` | map | guest confinement zones (empty here) |
| `npcs`, `games` | map | pets/NPCs, mini-games (tank arena, ...) |
| `users` | map uid -> user | all connected users, see below |
| `channels` | undefined here | chat channels |

`zones` (portal zones to other rooms, `{type:"portal", toRoomAlias, toZoneId}`) is not in the socket event; the client takes `roomDefinition.zones` from the REST `gaia/find` reply (null here).

User entry (fields seen; same shape in the `s` code 3 join event):

| field | example/type | note |
|---|---|---|
| `id`, `name` | string | |
| `x`, `y` | float | world position |
| `characterConfig` | `{version:5, bodyType:"characterA", slots:{slot: "characterA_<slot>_<item>+<variation>"}}` | avatar recipe, see 4 |
| `characterConfigOverride` | null/obj | merged over config |
| `role`, `affiliation` | string | profile text |
| `characterState` | `{state:"idle"|"walk"|"sit", target: undefined|seatId}` | `target` = seat id when sitting |
| `isSpeaking`, `isTyping`, `cameraOn` | bool | |
| `activeApp` | null/string | shared active app |
| `clusterMode` | `"none-away"`, `"none-connecting"`, `"cluster"`... | media cluster mode (see gaia-protocol.md) |
| `presence` | `{status, isOnline, isIdle, textStatus, mobile, inPrivateConversation, inCalendarMeeting, isYoga, lastOnline}` | |
| `meetingZoneId` | zone object id or null | which meeting zone the user is in |
| `memberRole` | `owner|contributor|...` | |
| `uploadedPhotoURL` | string/null | S3 photo |

### 2.4 Coordinate system and units

* Positions are floats in a **2D world space** (the "iso position" of the engine, `IsoPosition{x,y}`); not pixels of the screen. Observed ranges in this office: floors x -527..7276, y 105..6222; assets x -3193..7597, y -2872..6574; spawns (3260,980) and zone anchors in 420..6650 / 1160..4960.
* **Floor grid [F from data]**: floor object centres are at odd multiples of `G = 105.46875` (e.g. `738.28 = 7G`, `1582.03 = 15G`), i.e. a tile pitch of `2G = 210.9375`. All 806 floor tiles satisfy this. The floor catalog says `pixelDensity: 211` (px per unit of size 1) and tile image pivot `(110,67)` ~ a 220x134 px 2:1 diamond. Walls sit at multiples of `105.45` (half tile). Furniture and zones use free float positions. `sizeX/sizeY` of floors/walls/assets are all 1.0 (zones use sizeX/sizeY in world units, e.g. 282.59 x 282.59 = 1.34 tiles).
* **All 139 avatar positions seen on the wire (own moves + others' `s` events) fall inside a floor tile cell** [F]. Walkable area therefore is, at least, the union of floor tiles minus collider footprints [G].
* **Isometric mapping [G]**: tile diamond width 211 px against pitch 210.94 units suggests `screen_x = (x - y)/2`, `screen_y = (x + y)/4` up to sign/offset at zoom 1; this also puts floors inside the legacy bounds below. Not verified against a screenshot. Observed keyboard motion is consistent with an iso world: holding one direction moves along a world diagonal `(+116,-116)` (Euclidean 164 per tick), another along a pure axis `(0,+165)`.
* **Legacy map bounds [F]** (`lib-dakVzArj.js`): `mapBounds = {center:[0, -1639.52*s], size:[7680*s, 4320*s]}` with scale `s` from mapId: `skyboxguestarea`/`emptySkyboxSmallTemplate` 2.7, `newNeighborhoodEmpty`/`tileGridSystemSmallTemplate` 2.5, `skyboxGradient02`/`skyboxDaytime02` 2.0, `presetOfficeM` 1.6, **default 1.5** (so `skyboxSunset01`: center (0,-2459.28), size 11520 x 6480). These bounds describe the background image/camera limits (screen-like space), not the walkable rectangle [G].
* Server-side clamp: none observed; the server relays `move {x,y}` (see gaia-protocol.md).

### 2.5 Map object (`PbMapObject`) schema [F, module `Yu` of `httpClientFactory-CXSNSIW-.js`]

`b_set_obj` payload (binary, protobuf proto3). Same message is emitted by the client when the user edits the map (`socket.emit("b_set_obj", bytes)`). A message without `data` means delete.

```proto
enum PbOrientation   { N=0; NE=1; E=2; SE=3; S=4; SW=5; W=6; NW=7; }
enum PbMapObjectType { Zone=0; Asset=1; Floor=2; Wall=3; }
message PbColorVariation { optional float hueRotationDegrees=1; optional float saturationMultiplier=2; optional float valueMultiplier=3; }
message PbMapObject {
  PbMapObjectType type = 1;
  float x = 2;  float y = 3;
  string assetId = 4;             // catalog id (floor/wall/asset), or "_zone" for zones
  PbOrientation orientation = 5;
  optional float sizeX = 6;  optional float sizeY = 7;
  optional google.protobuf.Struct perms = 8;
  optional google.protobuf.Struct interaction = 9;   // {type, data:{...}}
  optional google.protobuf.Struct config = 10;       // e.g. embed frames
  optional PbOrientation spawnDirection = 11;
  optional PbColorVariation colorPalette = 13;       // never present in this capture
  optional string colorKey = 14;                     // key into the catalog's presetColorsDictionary
  optional string updatedBy = 15;                    // uid
  optional int64 updatedAt = 16;                     // ms epoch
  repeated string tags = 17;
  optional string accessKey = 18;                    // "", "PREMIUM"
}
message PbSetMapObject { string id = 1; optional PbMapObject data = 2; }   // id = 20 char object id
```

(Field 12 is unused.) Decoded in-app via `fromPbMapObject` into `{type:"zone|asset|floor|wall", x, y, assetId, orientation, sizeX, sizeY, perms, interaction, config, colorPalette, colorKey, updatedAt, updatedBy, tags, accessKey}` and stored in `room.mapObjects[id]`. Wire example (hex, truncated, field-by-field): `0a14<id 20B> 12 52 | 08 02 (type=Floor) 15 <f32 x> 1d <f32 y> 22 14 <assetId 20B> ...`.

Object statistics for this office (1,473 objects):

| type | count | distinct assetIds | notes |
|---|---|---|---|
| floor | 806 | 26 | all sizeX=sizeY=1; orientations N 683, E 88, S 27, W 8 |
| wall | 276 | 15 | orientation S/E/W/N 79/70/66/61; `accessKey` set on all |
| asset (furniture) | 371 | 187 | orientations mostly N/E/S/W, a few NE/SE/SW (diagonal wall boards) |
| zone | 20 | 1 (`_zone`) | all `interaction.type == "meeting"` |

`interaction` types seen on assets: `desk` 17, `sysImageText` 7 (posters/text boards, with `config.frames`), `native_whiteboard` 2, `whiteboard` 1 (external URL in `data`), `spawn` 1, `game` 1, `broadcast` 1. The client code knows more (`inWorldMetadata`, `isInputActionMovement`): `portal`, `travel`, `link`, `embed`, `broadcast`, `game`, `food`, `whiteboard`, `file`, `door`, `native_whiteboard`, `integration_miro|gdocs|msoffice`, `npc_chat`, `sysImageText`, `sysGetOffice`, `gong`, `desk`, `meeting`, `spawn`, `guest_confinement`.

Zone example (redacted): `{type:"zone", x:6643.35, y:2636.25, assetId:"_zone", orientation:"N", sizeX:470.99, sizeY:659.39, interaction:{type:"meeting", data:{meeting:{name:"Town Hall"}}}, colorKey:"1cE2…", updatedBy:"<uid>", updatedAt:1762880041406}`. A zone is an axis-aligned rectangle centred at (x,y) [G: centred, since `Anchor.center()` is used for zones in `Oi()`].

Embedded images/text: `config.frames.<frameId>` is `{src,thumbnail,type:"image"}` (URLs on `sowork-space.s3.us-east-2.amazonaws.com/<groupId>/objects/<objId>/embed/<hash>`) or `{type:"text", text, color, bgColor, fontFamily, fontSize, hAlign, vAlign, width, height}` (text boards are rasterized in the browser with `ultimate-text-to-image` into a PNG data URL and passed to the engine, `isInputActionMovement` `ii()`).

### 2.6 Eyekia asset service (floor, wall, furniture catalog)

REST, no auth observed on the GET endpoints. Base `https://app.sowork.com` (or `https://api.sowork.com`) [F `DeskThumbnail-Cx2o-Aiq.js`, `MapAssetsStore`]:

| endpoint | returns |
|---|---|
| `GET /eyekia/api/v1/assets-version` | `{assets:[{id,v}],floors:[...],walls:[...]}` cache-busting versions; pass as `?v=<v>` |
| `GET /eyekia/api/v1/category/` | `{categories:[{id,data:{name,rank,thumbnail,defaultInteraction,admin}}]}` (desks, smartObjects, seating, officeTools, surfaces, dividers, ...) |
| `GET /eyekia/api/v1/asset/?categoryId=<id>&pathOnlyUrls=true` | all assets of a category (257 in `seating`, 1.5 MB) |
| `GET /eyekia/api/v1/asset|floor|wall/<id>?pathOnlyUrls=true&v=<ver>` | one entry; 404 for deleted ids (one floor 404 in capture) |
| image base (code) | `https://app.sowork.com/asset-images` + the `url` field (e.g. `/floor/<id>/<hash>`); thumbnails rewritten to `<dir>/m2/<name>.webp` |

Floor entry: `{orientations:{N|E|S|W:{url,pivotPosition:{x:110,y:67}}}, type:"floor", pixelDensity:211, isCorner, colors:{baseColor,presetColors,presetColorsDictionary}, accessKey, name}`. Wall entry: same with `height:"1M"`, `pixelDensity:106`, pivot (256,134) etc.; names like "Plaster Wall011 Border NoCollider" suggest a collider flag per wall variant.

Furniture (asset) entry (key parts):

```
{name, categoryId, thumbnail, accessKey, defaultInteractionType, colors:{...}, projections:{isometric,orthographic},
 orientations:{N|E|S|W|NE|SE|SW|NW:{goToPivot:{x,y}, images:{<imgId>:{url, transform:{position,scale,rotationRad,frontPoint,depthBand}, type:"image",
     primitives:{<id>:{type:"shape", shapeIntersectionType:"selector"|"collider", height, shape:{shape:"rect",position,scale,rotationRad}}
                    | {type:"seat", facingDir, shapeIntersectionType:"sensor", transform, shape}
                    | {type:"shadow", shadowType:"rect", shape} | {type:"frame", ...}}}}}},
 config:{frames:{}}}
```

Primitive counts in the 257-asset `seating` list: seat sensors 1462, selector shapes 917, shadows 846, **colliders 68**, frames 4. So **collision = rect `collider` primitives on furniture images [F]**; selectors are click hit areas; seat sensors define sitting spots and facing; `depthBand` and `frontPoint` give draw-order data for isometric depth sorting.

Furniture/floor/wall recolor: `colorKey` (preset id) or `colorPalette` => hue rotation + saturation/value multipliers applied by shader.

Meeting zones, private rooms and doors:

* **Meeting zone = zone object with `interaction.type:"meeting"`** and `spawnAreas[zoneId]` anchor. The user's current zone is reported via `set_meeting_zone {zoneId}` and echoed in `s` code 9 (`meetingZoneId`). Zone containment is computed **client-side by the engine** (`IntersectionEvent` -> `zone:overlapStart/End` with `zoneType:"mapObject"`, handled in `main` classes `wwt`, `ai.handleZoneEvent`); the server just stores it [F for events, G for "server does not recheck"].
* Doors: `interaction.type:"door"` objects (none in this office); lock state through `perms` Struct and `accessKey`; the engine has `SetAccessPolicy(object_id, granted)`, `ZoneAccessDenied` events and `access_granted` flag on objects (`Oi()` in the facade). Sounds `doorKnock`, `doorForceOpen`. [F code, no instance observed]
* Guest areas: `interaction.type:"guest_confinement"` zones + `ConfineToZone`/`ExcludeFromZones` engine commands.
* Teleport/portal: `portal` interaction and `room.zones[zoneId].type=="portal"` => `initialize(resolveRoomZoneInit)` joins another room (`ai.handleZoneEvent` in main). `travel` and `Rd.getRoom` handle teleport-to-user (REST `findRoom`).

---------------------------------------------------------------------------------------------------

## 3. Object model summary

| id space | example form | where defined |
|---|---|---|
| map object id | 20 chars `[A-Za-z0-9_-]` (Firestore auto id) | key of `b_set_obj`, `room.mapObjects`; zone ids are the `meetingZoneId` values |
| assetId | 20 chars; zones `_zone`; also `_zoneGuestConfinement` (client built-in) | Eyekia catalog |
| user id | Firebase uid (28 chars) | everywhere |
| seat id | 40 chars (`characterState.target` when `state:"sit"`) | derived from asset+seat primitive [?]; not found among loaded object ids |

Client-side the same objects are converted for the engine by `Oi()`: `CreateOrUpdateObject(type, id, IsoPosition(x,y), IsoSize(sizeX,sizeY), IsoDirection(orientation), assetId, interactive, ..., anchor, isDoor, instanceConfig(frames), colorPalette, colorKey, isMeetingZone)`. Anchor is `top_center` for furniture/interactive, `center` for zones/desks. `MapObjectType` = {Zone, Asset (Furniture), Floor, Wall}.

Decoder: `python3 analysis/scratch/decode_obj.py > objects.jsonl` decodes all 1,473 objects from `events.jsonl` (uses `ws_msgpack.frames()` + `decode_set`).

---------------------------------------------------------------------------------------------------

## 4. Avatars and sprites

| aspect | finding |
|---|---|
| What is drawn in the world | **layered 2D sprite atlases** composited in the wasm engine, not 3D, not photos, not emoji. Photos (`sowork-space.s3.us-east-2.amazonaws.com/profile_photos/<uid>/profile.webp?v=`, and Google `lh3.googleusercontent.com/...=s128-c-mo`) are used in DOM UI (people list, chat, camera-off tiles) |
| Recipe | `characterConfig = {version:5, bodyType:"characterA", slots:{<slot>:"characterA_<slot>_<item>+<variation>"}}`; slots seen: body, eyes, eyebrows, mouth, hair, hat, shirt, pants, shoes, beard, headAccessory, extraAccessory, backAccessory, bodyAccessory, outfit (also makeUp, avatarExtra in catalog). Body types in catalog: `characterA` (1,683 items), `characterWheels`, `pet`, `fx`, `humans`, `assistants`, `cats`, `robots` |
| Compact string form | `bodyType!slot:~item+var!slot:...` (`~` = relative to `<bodyType>_<slot>_`), used in portrait URLs |
| Catalog | `characters/v5/character_art_database.json` (1.5 MB): `assets[<assetId>]={preview:{ver,type:"atlas"}, layers:{<layer>:{<animation>:{ver,type:"atlas"}}}}`, 2,806 assets; `customizer_config.json` (1.0 MB) = bodyTypes -> slots -> items/variations/swatches; `customizer_product_prices.json` |
| Atlas URL (code) | layer: `characters/v5/` + `encodeURIComponent("<assetId>_<layer>_<animation>_atlas_<ver>")`; preview: `characters/v5/<assetId>_preview_atlas_<ver>.png` and `.json` (`+` -> `%2B`), relative to `assets_root = https://app.sowork.com/` (`toAssetLayerPath`, `toAssetPreviewPaths` in `httpClientFactory-*.js`). Atlas = PNG + JSON frame table (TexturePacker-like `frame`, `spriteSourceSize`, `sourceSize`, used by `pe()` in `AssetDownloadManager`). **The actual atlas PNG/JSON requests are not in the capture [?]** (extension of the layer files unverified; `.png`/`.json` assumed) |
| Animations | characterA: `idle`, `walk`, `sit`, `jump1` (registered by the client); catalog also has `idleClock`, `idleFeet`, `dance1`, `laugh1`, `angry1`, `celebrate1`, `inLove1`, `sitGround`, `sleep`; layers per item: `body`, `bodyFront`, ... (drawn back to front) |
| Directions | 8 (`IsoDirection` N,NE,E,SE,S,SW,W,NW; `LocalCharacterChange.facing_direction` is a Vec2) |
| Recolor | hue/saturation/value GLSL (fragment shader string in `httpClientFactory-*.js`) on layers; variations like `+blue` are separate atlases or swatch transforms |
| Portraits | server-rendered PNG: `GET /portrait/v1/<view>!<anim>/<configString>` e.g. `body!S` or `meeting!idle` / `meeting!clockEmojiMeeting` (used for DOM chips, 22 requests) |
| Name label / status / speaking ring / video bubble | DOM overlays and engine UI. Engine exposes `proximity_indicator_radius` (client passes **850** world units when `showProximityCircle`), `is_unavailable`/`in_pm` flags (availability circle), `CharacterClicked/Hovered` events with screen coordinates; the app positions React overlays from them (`ui_containers`, `characterMetadata`/`localUser {x,y,width,height}` store in `main`). Proximity events `character:proximity:near|far {userId}` come from the engine |
| Pets/NPCs | `pet` body type, follow logic via `start_following` |

What a minimal client **must** do: draw a marker (or a flat color circle / a portrait PNG) per user with name; **may skip**: layered atlases, shadows, lights, animations, recolor, 8 directions, emotes, pets, DOM bubbles for video (until needed). To keep real avatars: fetch `character_art_database.json`, build atlas URLs as above, draw `idle`/`walk` frames (need the atlas capture to confirm file layout).

---------------------------------------------------------------------------------------------------

## 5. Input and movement (client-side algorithm)

All movement simulation is in the wasm engine. JS passes inputs and options and reports results. **Server is a relay**: no position validation observed.

### 5.1 Inputs [F] (`UserBio-JYfR6TjN.js`, class `CharacterMovement`, key list `sl`)

```js
sl=["W","w","A","a","S","s","D","d","Control","ArrowLeft","ArrowUp","ArrowRight","ArrowDown"]; Ns=[...sl,"Shift","Space"]
handleKeyInput = e => { ... switch(e.type){ case"ArrowUp":case"W":case"w": t=InputActionType.Up; ...
   case"Space": t=Jump; case"Shift": t=TogglePan; case"Control": t=Delete ...}
   this.broker.dispatch({type:"interact:inputAction", data:{action:t, started: keydown}}) }
```

`InputActionType`: Delete0 Down1 Jump2 Left3 TogglePan4 Right5 Up6 Pan7 CameraFollow8 CameraPan9 ToggleZoom10 ZoomIn11 ZoomOut12 ZoomReset13 Rotate14 Copy15 Cancel16. The engine translates held actions into motion (8-way); key down/up are separate events (`started` true/false). Inputs are gated by locks (`inputLocks`, e.g. chat input focus) and `set_keyboard_enabled`.

Click-to-move [F]: `ClickInputMode{primary_click, secondary_click, double_primary_click}`; desktop = `secondary_click` (right click) and `double_primary_click` walk; mobile = single tap (`WASM-CErjq-cP.js`: `g.primary_click=w, g.secondary_click=!w, g.double_primary_click=!w`, w = Android/iOS/iPad). The engine resolves the click to an iso position and **pathfinds** (`PathfindingEvent started|changed|stopped`; app shows a "pathfinding in progress" toast with cancel -> `characterMoveMode: Stop`). Programmatic: `goToLocation({x,y})` = `character:createOrUpdate {isLocalPlayer:true, position, characterMoveMode: Pathfind}`.

`CharacterMoveMode` enum (`GaiaRoomState`, `da`/`Lf`): **Lerp=0, Pathfind=1, Teleport=2, Stop=3**. Used as: remote users' moves -> `Lerp`; reconnect diffs -> `Teleport` unless within 1000 units (`t6e = 1000^2` squared distance) then `Lerp`; local goToLocation -> `Pathfind`; when the world is obscured (`worldObscured`) the app uses `Lerp` instead of pathfinding.

### 5.2 Constants [F]

```js
initializeMovementSettings(){ broker.dispatch({type:"config:setMovementOptions",
   data:{movementSpeed:800, pathfindingFarMultiplier:2, pathfindingFarThreshold:1e3}}) }   // UserBio-*.js
```

| constant | value | meaning |
|---|---|---|
| `movementSpeed` | **800** world units/s | base walk speed (per-character `speed_multiplier` from `getCharacterModifiers`, and `collider_multiplier` for collider size) |
| `pathfindingFarThreshold` / `FarMultiplier` | 1000 / 2 | beyond 1000 units of path distance, speed is x2 (observed 1,600 u/s bursts right after click-to-move at t=214-215 s) |
| movement mode | Isometric (0) / Orthogonal (1) from map `projection` | `SetConfig.movement_mode` |
| proximity radius | 850 | passed as `proximity_indicator_radius` |
| teleport/lerp threshold | 1000 units | `GaiaRoomConnection.mergeNewRoomState` |
| `kg` = 100 | offset when teleporting next to a user (`addDirectionalOffset`) | |
| FPS presets | 15 / 30 / 60 (pause = 0) | `WorldFpsLimiter` |
| physics options | `enable_asset_collisions`, `enable_local_character_collisions`, `enable_local_character_furniture_collisions` (defaults true for assets) | `WASM-*.js` init |

### 5.3 Frame loop and prediction

* Frame loop is `requestAnimationFrame`-driven inside the engine, capped by `setFPS`; step size and fixed/variable timestep are inside wasm [?].
* There is **no server-authoritative prediction/reconciliation**. The local avatar is simulated locally; the client reports where it is. Remote avatars are interpolated (`Lerp` mode) toward each received position (no timestamp used; `sync_time` offsets exist but `move` has none).
* Camera: `StartFollowing`/`SetCameraMode.follow()` follows the local character; `SetCameraSmoothing`, `SetCameraZoom{absolute|relative, lerp_time}`, panning via Shift (`TogglePan`), `reset_camera_*`.

### 5.4 Network emission (the exact function)

Chain [F]:

1. wasm emits `LocalCharacterChange {character_id, position, character_state, facing_direction}` -> facade `toGameEvent` => `{type:"character:update", userId, data:{position,characterState,direction}}` (`isInputActionMovement-*.js`).
2. `main` (`RoomAndBrokerSync.handleLocalCharacterUpdate`): `this.room.handleUserUpdate({...e.data, id: this.room.userId})` -> room emits `room:userUpdate`.
3. `GaiaRoomConnection.handleRoomUserUpdate` in `main-DD7sSLvE.js`:

```js
handleRoomUserUpdate = e => { const {position:t, characterConfig:i, name:n, info:s, characterState:r, isSpeaking:o, isTyping:a, cameraOn:l, meetingZoneId:u} = e.data;
  if (e.type==="room:userJoin" && t && i) socket.emit("spawn",{characterConfig:i,x:t.x,y:t.y});
  else { if (t){ const c={x:t.x,y:t.y}; if(this.followingUserId) c.following=this.followingUserId; socket.emit("move",c) }
         i && socket.emit("user_update",{characterConfig:i}) }
  r && socket.emit("character_state",{characterState:r});
  isDefined(o) && socket.emit("speech",{isSpeaking:!!o}); isDefined(a) && socket.emit("typing",{isTyping:!!a});
  isDefined(l) && socket.emit("camera_on",{cameraOn:!!l}); u!==void 0 && socket.emit("set_meeting_zone",{zoneId:u}); ... }
```

So the emission rate is whatever the engine's `LocalCharacterChange` rate is. **Measured from the 84 outgoing `move` messages** [F]: while walking, one `move` about every **0.205 s (~5 Hz)**, each step ~165 units, i.e. **~800 u/s**; on start of walk one extra small first step (~27 units) with `character_state walk`, and on stop a final `move` plus `character_state idle`. Click-to-move bursts moved 1,300-2,000 u/s (the far multiplier, 100-ms spacing). No acknowledgement; the server broadcasts `s` code 1 `{userId,x,y}` to others (not to self).

Reconnect: socket query carries last `x`,`y`; client sends `user_reconnected {clusterMode, zoneId, shouldUpdate}`; the new `room` event gives positions and the client merges (`mergeNewRoomState`).

Order of messages when starting to walk: `move` then `character_state walk`; stop: `move` then `character_state idle`.

---------------------------------------------------------------------------------------------------

## 6. What a minimal client needs

Requirement: render the map plainly, walk, see others.

| need | source | notes |
|---|---|---|
| Auth + room resolve | `auth-and-rest.md`; `POST gaia/find {type:"group",groupId,alias,redirect:true}` -> `serverId`, server url | |
| Socket | `gaia-protocol.md` (socket.io EIO4 + msgpack, auth token in connect packet, query roomId/mediaRegion/clusterMode/x/y/joinType) | on `init` send `should_update {v:true,t:0,lastObjectId:""}` |
| Map | `room` event + `b_set_obj` stream decoded with the 10-field proto above (`protobufjs`/`@bufbuild/protobuf` or 80 lines by hand, see `decode_obj.py`) | store by id; apply later `b_set_obj` as upsert/delete |
| Plain rendering | floors: draw each floor tile as a rotated square `210.94 x 210.94` at (x,y) (no images); walls as thick segments at their (x,y) with orientation; furniture as boxes using `collider` rects; zones as labelled rectangles (sizeX x sizeY at x,y). Use top-down or the iso mapping hypothesized in 2.4 | no Eyekia image needed for a plain view |
| Optional sprites | `GET /eyekia/api/v1/floor|wall|asset/<id>?pathOnlyUrls=true&v=<ver>` + `asset-images` base | needed only for pretty rendering and for exact collider rects (below) |
| Walking | keyboard -> velocity 800 u/s (8-way) integrated locally; send `move {x,y}` at ~5 Hz plus `character_state {state:"walk"|"idle"}` on transitions; click-to-move needs your own pathfinding (A* over floor cells) | |
| Collision | **optional**: the server does not enforce. To avoid walking through walls you need asset `collider` primitives (fetch catalog for the 187 furniture ids and 15 wall ids) or approximate by blocking tiles without floor | the safest "good citizen" rule: stay within floor tiles |
| Others | handle `s` batches: code 1 move (lerp), 3 join (full user), 2 character_state, 6 speaking, 9 meeting zone, 10 cluster mode; `user_disconnect`/`leave`; `presence`; `update`/`update_info` | |
| Meeting zone | compute containment yourself (point in rect of zone objects with `interaction.type=="meeting"`); send `set_meeting_zone {zoneId|null}` | needed for media grouping |
| Proximity/media | `cluster_mode`, LiveKit per `media-livekit.md` | |
| Presence heartbeat/other | see gaia-protocol.md | |

**Can be dropped**: the wasm engine (11 MB) and all React/MUI/Firebase UI code, 3rd-party trackers, character customizer JSONs (2.6 MB), atlases, shadows/lights, background image, sounds, Eyekia images (unless you want art), emotes/darts/games/pets/whiteboards/portals/doors, chat (separate service `chat.sowork.com`), Intercom/Sentry/Amplitude. Data to cache: the 179 KB object stream (resend with `lastObjectId` and max `updatedAt` for deltas).

Estimated payload for the plain client: 179 KB map + room event (~10 KB) + tiny JS.

---------------------------------------------------------------------------------------------------

## 7. Open questions and recordings that would close them

| # | question | recording / action |
|---|---|---|
| 1 | Atlas/furniture/floor image requests and `maps/psd/map_asset_config.json`, background image URL and the 187 per-asset JSONs are absent from the capture (recorder only attached to page/iframe targets; the glue has `new Worker`, or asset fetches went through a cache) | re-record with worker/service-worker auto-attach and also save "Network.loadingFinished" for all targets; or open DevTools Network once and export a HAR of entering the office |
| 2 | The wasm body was evicted (Chrome inspector cache); real step size/timestep, navmesh construction, collision behavior, depth sorting | record with `Network.getResponseBody` immediately on the wasm response; run `wasm-objdump`/`wasm-tools strings` to confirm Bevy/wgpu/rapier |
| 3 | Exact world->screen mapping and axis orientation (section 2.4 is a guess) | take one screenshot at known zoom plus own `x,y` and a second position; also read the wasm `SetCameraZoom` default |
| 4 | Which keys map to which world direction (Up = ?) | record 4 isolated key presses (1 s each) and note the `move` deltas; we only have mixed walk data |
| 5 | Server-side position validation or speed limit | send an own test client `move` with a far jump on our account and watch for correction/kick |
| 6 | Seat id format (`characterState.target`) and how sitting is requested | record clicking a chair (`state:"sit"`) and standing up |
| 7 | Door/lock (`door`, `perms`, `accessKey`) and portal zones in a map that has them; `perms` Struct schema | build a test door and portal in our own space with the map editor and record `b_set_obj` + the knock flow |
| 8 | Map editing message flow (client emits `b_set_obj` with Struct fields) incl. permission failure | record placing/moving/deleting an object as admin |
| 9 | `mapBounds` when defined, and meaning for non-legacy maps (extension/`map:set` center and size) | open a second map (other `mapId`) |
| 10 | `characterWheels`/pets/NPC `npc_spawn`, `s` codes 11-13 | enable a pet in profile and record |
| 11 | Who sets `spawns` vs `spawnAreas` choice at join and `joinType=force` semantics | join a second time from another browser and compare the query and `room.spawns` use |
