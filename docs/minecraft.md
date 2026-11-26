# Minecraft mod

The SoWork office inside Minecraft, as a Fabric mod that carries everything it needs: you do not need to install
SoWorker for it.

1. **Requirements.** Minecraft Java 26.3 on Linux x86_64 or Windows x86_64 (no macOS build). Chrome, Chromium or Edge,
   for signing in the first time. On Linux: PulseAudio or PipeWire (`pactl`, and `parec` or `pw-record`), and
   `ffmpeg` for recordings with video. On Windows nothing else.
2. **Fabric.** Install Fabric Loader 0.20 or newer for 26.3 with the installer from
   [fabricmc.net](https://fabricmc.net/use/installer/), and put [Fabric API](https://modrinth.com/mod/fabric-api) for
   26.3 into your `mods/` folder (`~/.minecraft/mods`, or `%APPDATA%\.minecraft\mods` on Windows).
3. **The mod.** Download `soworker-mc-<version>.jar` from the
   [latest release](../../releases/latest) into the same `mods/` folder (remove an older `soworker-mc` jar).
4. **A world for the office.** Start Minecraft with the Fabric profile and create a new singleplayer world: World
   type "Superflat", preset "The Void". The office is built around block 0 64 0 and replaces the ground there, so do
   not use a world you care about.
5. **Switch it on.** Type `/sw enable`. The first time, a browser window opens on SoWork's sign-in page: sign in, and
   the office builds. Next time it remembers you (the sign-in is saved next to the desktop SoWorker's, so if you use
   that too you are already signed in).
6. **What you can do.** Walking moves your avatar; stone is a meeting room (walk in to join the call). In a
   meeting the hotbar holds mic, screen share, record and camera; always there are "walk to your desk", available /
   away, a spyglass, snowballs and an ender pearl (right click the tools to use them). Throw a snowball at a coworker to wave at them, or at a screen share to watch it full screen; an ender pearl teleports
   you. Plain chat goes to the room chat in a meeting. Commands: `/sw mic`, `share`, `rec`, `cam`, `desk`, `away`,
   `hand`, `leave`, `wave <name>`, `say <text>`, `status`, `login`, `logout` (sign in as someone else), `disable`
   (switch the office off; the blocks stay).
7. **One place at a time.** SoWork lets each account be connected once: opening the desktop SoWorker (or SoWork in a
   browser) while you are in Minecraft disconnects the Minecraft one, and the other way round. `/sw login` takes it
   back into Minecraft.
8. **Bug reports.** The mod's soworker writes its log to `~/.local/state/soworker/soworker.log` on Linux and
   `%LOCALAPPDATA%\soworker\soworker.log` on Windows; Minecraft's own log (`logs/latest.log`) has the rest.

Building it: `make mc` builds the mod's soworker for Linux and Windows (`--features mc,calls`, no window code; the
Windows half needs the `build-windows` setup) and the jar (`mc/build/`), and installs it into `~/.minecraft/mods`.
`make mc-test` runs its tests. For development, `-Dsoworker.bin=<path>` in the game's JVM arguments runs another
soworker binary, and `-Dsoworker.args="--mc-demo"` passes arguments (the demo office needs no SoWork account).
