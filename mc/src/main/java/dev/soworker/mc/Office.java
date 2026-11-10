package dev.soworker.mc;

import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import net.minecraft.ChatFormatting;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.server.MinecraftServer;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.entity.player.Input;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.gamerules.GameRules;
import net.minecraft.world.level.storage.LevelResource;

/**
 * The office in this world: what soworker last told us and the parts that show it. Runs on the server thread only.
 * The bridge only acts in a world that was switched on with {@code /sw enable} (it rebuilds the ground around block
 * (0, 64, 0) and puts the player in adventure mode), so an ordinary world is never touched.
 */
public final class Office {
    static final String FILE = "soworker-office.json";
    final MinecraftServer server;
    final Bridge bridge;
    final MapBuilder map = new MapBuilder();
    final Villagers villagers = new Villagers();
    final Tools tools = new Tools();
    final Screens screens = new Screens();
    final Panel panel = new Panel();
    final ZoneLabels zoneLabels = new ZoneLabels();
    boolean enabled;
    long tick;
    JsonObject hello, call, me, lastScreens;
    /** Where soworker carries the player during its own walk (null = the player walks). */
    double[] follow;
    /** The minecart ride to the desk, while there is one. */
    final Rail rail = new Rail();
    boolean toolsDirty = true;
    boolean swept;
    int saveIn = -1;

    Office(MinecraftServer server, Bridge bridge) {
        this.server = server;
        this.bridge = bridge;
        load();
    }

    ServerLevel level() {
        return server.overworld();
    }

    Path file() {
        return server.getWorldPath(LevelResource.ROOT).resolve(FILE);
    }

    void load() {
        Path f = file();
        if (!Files.exists(f)) return;
        try {
            JsonObject o = JsonParser.parseString(Files.readString(f)).getAsJsonObject();
            enabled = o.has("enabled") && o.get("enabled").getAsBoolean();
            if (o.has("placed")) {
                for (JsonElement e : o.getAsJsonArray("placed")) {
                    JsonArray a = e.getAsJsonArray();
                    Block b = switch (a.get(1).getAsString()) { // the short names of earlier versions
                        case "grass" -> Blocks.GRASS_BLOCK;
                        case "cobble" -> Blocks.COBBLESTONE;
                        case "fence" -> Blocks.OAK_FENCE;
                        default -> MapBuilder.block(a.get(1).getAsString(), Blocks.BARRIER);
                    };
                    map.placed.put(a.get(0).getAsLong(), b);
                }
            }
        } catch (Exception e) {
            SoworkerMod.LOG.warn("soworker: cannot read {}: {}", f, e.toString());
        }
    }

    void save() {
        JsonObject o = new JsonObject();
        o.addProperty("enabled", enabled);
        JsonArray placed = new JsonArray();
        map.placed.forEach((p, b) -> {
            JsonArray a = new JsonArray();
            a.add(p);
            a.add(BuiltInRegistries.BLOCK.getKey(b).getPath());
            placed.add(a);
        });
        o.add("placed", placed);
        try {
            Files.writeString(file(), o.toString());
        } catch (Exception e) {
            SoworkerMod.LOG.warn("soworker: cannot write {}: {}", file(), e.toString());
        }
    }

    ServerPlayer player() {
        List<ServerPlayer> ps = server.getPlayerList().getPlayers();
        return ps.isEmpty() ? null : ps.get(0);
    }

    void chat(String text, ChatFormatting color) {
        ServerPlayer p = player();
        if (p != null) p.sendSystemMessage(Component.literal("[SoWork] ").withStyle(ChatFormatting.DARK_AQUA).append(Component.literal(text).withStyle(color)));
    }

    // ------------------------------------------------------------ switching on and off

    /** Ask soworker for its whole state again (after switching on, or when the player joins). */
    void resync() {
        JsonObject o = new JsonObject();
        o.addProperty("t", "resync");
        bridge.send(o);
    }

    void enable() {
        enabled = true;
        save();
        prepareWorld();
        toolsDirty = true;
        resync();
        if (bridge.connected()) {
            chat("Office switched on: building it", ChatFormatting.GREEN);
        } else {
            chat("Office switched on. Start soworker --mc to connect.", ChatFormatting.YELLOW);
        }
    }

    void disable() {
        enabled = false;
        save();
        villagers.clear();
        rail.finish(level(), player());
        screens.clear(level());
        panel.clear();
        zoneLabels.clear();
        sendAway(false);
        ServerPlayer p = player();
        if (p != null) {
            Tools.removeAll(p);
            setWalkSpeed(p, 0.1f);
        }
        chat("Office switched off (the blocks stay; /sw enable switches it on again)", ChatFormatting.YELLOW);
    }

    /** Walking speed (abilities, so the field of view stays as it is): Minecraft's own. At two blocks per tile the
     *  avatar walks 7.6 blocks/s, faster than the player even sprinting, so it always keeps up. Set explicitly to undo
     *  the slower speed an earlier version left on the player. */
    static final float WALK = 0.1f;

    static void setWalkSpeed(ServerPlayer p, float speed) {
        if (p.getAbilities().getWalkingSpeed() != speed) {
            p.getAbilities().setWalkingSpeed(speed);
            p.onUpdateAbilities();
        }
    }

    void prepareWorld() {
        ServerLevel l = level();
        GameRules r = l.getGameRules();
        r.set(GameRules.ADVANCE_TIME, false, server);
        r.set(GameRules.ADVANCE_WEATHER, false, server);
        r.set(GameRules.SPAWN_MOBS, false, server);
        r.set(GameRules.SPAWN_MONSTERS, false, server);
        r.set(GameRules.SPAWN_PATROLS, false, server);
        r.set(GameRules.SPAWN_WANDERING_TRADERS, false, server);
        r.set(GameRules.RAIDS, false, server);
        r.set(GameRules.KEEP_INVENTORY, true, server);
        ServerPlayer p = player();
        if (p != null) {
            if (p.gameMode() == GameType.SURVIVAL) p.setGameMode(GameType.ADVENTURE);
            setWalkSpeed(p, WALK);
        }
        if (!swept) {
            for (var e : l.getAllEntities()) if (e != null && Villagers.leftover(e)) e.discard(); // already loaded ones
            swept = true;
        }
    }

    // ------------------------------------------------------------ messages

    void handle(Bridge.In in) {
        ServerLevel l = level();
        if (in.connected()) {
            if (enabled) {
                prepareWorld();
                chat("Connected to soworker", ChatFormatting.GREEN);
            } else {
                chat("soworker is running. Type /sw enable to build the office in this world (use a new void world: it rebuilds the ground around 0 64 0).", ChatFormatting.YELLOW);
            }
            return;
        }
        if (in.disconnected()) {
            follow = null;
            sendAway(false);
            if (enabled) chat("Lost the connection to soworker; trying again", ChatFormatting.RED);
            return;
        }
        if (in.binary() != null) {
            byte[] b = in.binary();
            if (enabled && b.length > 0 && b[0] == 2) face(b);
            else if (enabled && b.length > 0 && b[0] == 3) watchFrame(b);
            else if (enabled) screens.frame(l, b);
            return;
        }
        JsonObject m = in.json();
        String t = m.get("t").getAsString();
        if (t.equals("say")) {
            chat(m.get("text").getAsString(), ChatFormatting.WHITE);
            return;
        }
        if (t.equals("hello")) {
            hello = m;
            return;
        }
        if (!enabled) return;
        switch (t) {
            case "world" -> {
                map.apply(m, l);
                map.forceChunks(l);
                if (lastScreens != null) screens.apply(l, lastScreens, joinedZone(), zoneBox());
            }
            case "users" -> villagers.apply(l, m);
            case "me" -> {
                me = m;
                tools.available = m.get("available").getAsBoolean();
                sendAway(!tools.available);
                toolsDirty = true;
            }
            case "call" -> {
                call = m;
                tools.inCall = m.get("in_call").getAsBoolean();
                tools.mic = m.get("mic").getAsBoolean();
                tools.cam = m.get("cam").getAsBoolean();
                tools.share = m.get("share").getAsBoolean();
                tools.rec = m.get("rec").getAsBoolean();
                tools.recClock = m.has("rec_clock") && !m.get("rec_clock").isJsonNull() ? m.get("rec_clock").getAsString() : null;
                toolsDirty = true;
                if (lastScreens != null) screens.apply(l, lastScreens, joinedZone(), zoneBox());
            }
            case "screens" -> {
                lastScreens = m;
                screens.apply(l, m, joinedZone(), zoneBox());
            }
            case "watch_end" -> sendWatch(new WatchPayload(WatchPayload.CLOSE, "", new byte[0]));
            case "video_gone" -> {
                String key = m.get("key").getAsString();
                if (key.endsWith(":screen")) screens.gone(l, key);
                else sendFace(key, 0, 0, new byte[0]);
            }
            case "teleport" -> {
                ServerPlayer p = player();
                follow = null;
                rail.finish(l, p);
                if (p != null) p.connection.teleport(m.get("x").getAsDouble(), MapBuilder.FLOOR_Y + 1, m.get("z").getAsDouble(), p.getYRot(), p.getXRot());
            }
            case "follow" -> follow = new double[] {m.get("x").getAsDouble(), m.get("z").getAsDouble()};
            case "rail" -> {
                ServerPlayer p = player();
                if (p != null) rail.start(l, p, m.getAsJsonArray("path"));
            }
            case "follow_end" -> follow = null;
            case "emote" -> {
                String to = m.get("to").getAsString();
                boolean toMe = hello != null && hello.has("me") && to.equals(hello.get("me").getAsString());
                villagers.wave(l, m.get("from").getAsString(), to, toMe ? player() : null);
            }
            default -> {}
        }
    }

    /** A camera frame for a face: [u8 2][u16 key length][key][u16 w][u16 h][w*h*3 RGB], little-endian. */
    void face(byte[] b) {
        java.nio.ByteBuffer buf = java.nio.ByteBuffer.wrap(b).order(java.nio.ByteOrder.LITTLE_ENDIAN);
        buf.get();
        byte[] k = new byte[buf.getShort() & 0xFFFF];
        buf.get(k);
        int w = buf.getShort() & 0xFFFF, h = buf.getShort() & 0xFFFF;
        if (buf.remaining() < w * h * 3) return;
        byte[] rgb = new byte[w * h * 3];
        buf.get(rgb);
        sendFace(new String(k, java.nio.charset.StandardCharsets.UTF_8), w, h, rgb);
    }

    /** A snowball hit a screen share's wall: show it full screen on the player's client. */
    void watch(ServerPlayer p, Screens.Screen s) {
        if (!bridge.connected() || !net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking.canSend(p, WatchPayload.TYPE)) return;
        cmd("watch", s.key);
        sendWatch(new WatchPayload(WatchPayload.OPEN, s.name, new byte[0]));
    }

    /** A full-screen frame: [u8 3][u16 key length][key][u16 w][u16 h][JPEG], little-endian. */
    void watchFrame(byte[] b) {
        int kl = (b[1] & 0xFF) | (b[2] & 0xFF) << 8;
        int at = 3 + kl + 4;
        if (b.length <= at) return;
        sendWatch(new WatchPayload(WatchPayload.FRAME, "", java.util.Arrays.copyOfRange(b, at, b.length)));
    }

    void sendWatch(WatchPayload w) {
        ServerPlayer p = player();
        if (p != null && net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking.canSend(p, WatchPayload.TYPE)) {
            net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking.send(p, w);
        }
    }

    /** Tell the clients whether we are away (they hold sneak for the player while we are). */
    void sendAway(boolean away) {
        for (ServerPlayer sp : server.getPlayerList().getPlayers()) {
            if (net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking.canSend(sp, AwayPayload.TYPE)) {
                net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking.send(sp, new AwayPayload(away));
            }
        }
    }

    /** Pass a face frame (key = user id, or "me" for our own camera) to every client that has the mod. */
    void sendFace(String key, int w, int h, byte[] rgb) {
        net.minecraft.world.entity.Entity e = key.equals("me") ? player() : villagers.shown(key);
        if (e == null) return;
        FacePayload p = new FacePayload(e.getId(), w, h, rgb);
        for (ServerPlayer sp : server.getPlayerList().getPlayers()) {
            if (net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking.canSend(sp, FacePayload.TYPE)) {
                net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking.send(sp, p);
            }
        }
    }

    String joinedZone() {
        return call != null && call.has("joined") && !call.get("joined").isJsonNull() ? call.get("joined").getAsString() : null;
    }

    int[] zoneBox() {
        String z = joinedZone();
        return z == null ? null : map.zones.get(z);
    }

    void cmd(String name, String arg) {
        JsonObject o = new JsonObject();
        o.addProperty("t", "cmd");
        o.addProperty("name", name);
        if (arg != null) o.addProperty("arg", arg);
        if (!bridge.connected()) {
            chat("Not connected to soworker (start soworker --mc)", ChatFormatting.RED);
            return;
        }
        bridge.send(o);
    }

    // ------------------------------------------------------------ every tick

    void tick() {
        tick++;
        bridge.tick();
        for (Bridge.In in; (in = bridge.poll()) != null; ) {
            try {
                handle(in);
            } catch (Exception e) {
                SoworkerMod.LOG.warn("soworker: message failed: {}", e.toString());
            }
        }
        if (!enabled) return;
        ServerLevel l = level();
        if (map.tick(l, 4000)) {
            saveIn = 40;
        }
        if (saveIn > 0 && --saveIn == 0) save();
        if (!map.busy()) villagers.tick(l, tick); // only on a finished floor: they would fall in and be built over
        ServerPlayer p = player();
        if (tick % 5 == 0 && map.hasMap) zoneLabels.update(l, map, p);
        if (p == null) return;
        if (bridge.connected()) {
            panel.update(l, call, zoneBox(), joinedZone() == null ? null : map.zoneNames.get(joinedZone()));
            if (rail.active()) {
                // riding to the desk: the cart carries the player, and where they are is not a wish to go anywhere
                rail.tick(l, p);
            } else {
                moveAlong(p);
            }
            if (tick % 2 == 0 && !map.busy() && !rail.active()) {
                Input in = p.getLastClientInput();
                JsonObject o = new JsonObject();
                o.addProperty("t", "pos");
                o.addProperty("x", Math.round(p.getX() * 100) / 100.0);
                o.addProperty("z", Math.round(p.getZ() * 100) / 100.0);
                o.addProperty("input", in != null && (in.forward() || in.backward() || in.left() || in.right()));
                bridge.send(o);
            }
            if (toolsDirty || tick % 20 == 0) {
                tools.assertSlots(p);
                toolsDirty = false;
            }
            Tools.sweepDropped(p);
            Tools.refillThrowables(p);
        }
    }

    /** soworker's own walk: carry the player along in small steps, facing kept, unless they are pressing a movement key. */
    void moveAlong(ServerPlayer p) {
        if (follow == null) return;
        Input in = p.getLastClientInput();
        if (in != null && (in.forward() || in.backward() || in.left() || in.right())) return;
        double dx = follow[0] - p.getX(), dz = follow[1] - p.getZ();
        double d = Math.hypot(dx, dz);
        if (d < 0.02) return;
        double step = Math.min(d, 0.45); // 9 blocks/s: a little faster than the avatar's 7.6
        p.connection.teleport(p.getX() + dx / d * step, MapBuilder.FLOOR_Y + 1, p.getZ() + dz / d * step, p.getYRot(), p.getXRot());
    }

    String status() {
        StringBuilder s = new StringBuilder();
        s.append(enabled ? "office on" : "office off (/sw enable)");
        s.append(bridge.connected() ? ", connected to soworker" : ", soworker not connected");
        if (hello != null) s.append(", signed in as ").append(hello.get("name").getAsString()).append(hello.get("dry").getAsBoolean() ? " (dry run)" : "");
        s.append(", ").append(villagers.users.size()).append(" others shown");
        return s.toString();
    }
}
