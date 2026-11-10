package dev.soworker.mc;

import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import dev.soworker.mc.mixin.DisplayAccess;
import dev.soworker.mc.mixin.ItemFrameAccess;
import dev.soworker.mc.mixin.TextDisplayAccess;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.charset.StandardCharsets;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.component.DataComponents;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.Packet;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.entity.Display;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.decoration.GlowItemFrame;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.saveddata.maps.MapId;
import net.minecraft.world.level.saveddata.maps.MapItemSavedData;

/**
 * Video on map art: each screen in our call is a grid of glow item frames holding locked maps, on a black concrete
 * wall above the meeting zone's north edge, facing into the room, with a name label above. A camera is 2×2 maps, a
 * screen share 4×3. Frames arrive as palette indices and are written straight into the maps.
 */
public final class Screens {
    static final int BASE_Y = MapBuilder.FLOOR_Y + 3;

    public static final class Screen {
        public String key, name, kind;
        int cols, rows, x0, top, z;
        MapId[] maps;
        List<GlowItemFrame> frames = new ArrayList<>();
        Display.TextDisplay label;
    }

    private final Map<String, Screen> shown = new LinkedHashMap<>();
    private final Map<Long, Boolean> wall = new HashMap<>();
    private final ArrayDeque<MapId> spareMaps = new ArrayDeque<>();
    private String layoutKey = "";

    public void clear(ServerLevel level) {
        for (Screen s : shown.values()) remove(level, s);
        shown.clear();
        for (long p : wall.keySet()) level.setBlock(BlockPos.of(p), Blocks.AIR.defaultBlockState(), Block.UPDATE_ALL);
        wall.clear();
        layoutKey = "";
    }

    private void remove(ServerLevel level, Screen s) {
        for (GlowItemFrame f : s.frames) f.discard();
        if (s.label != null) s.label.discard();
        if (s.maps != null) for (MapId m : s.maps) spareMaps.add(m);
    }

    /** The share shown by this item frame, or null. */
    public Screen screenOf(net.minecraft.world.entity.Entity e) {
        for (Screen s : shown.values()) if (s.frames.contains(e)) return s;
        return null;
    }

    /** The share whose wall (the black backing or a frame's block) is at this block, or null. */
    public Screen screenAt(BlockPos p) {
        for (Screen s : shown.values()) {
            boolean inX = p.getX() >= s.x0 && p.getX() < s.x0 + s.cols, inY = p.getY() <= s.top && p.getY() > s.top - s.rows;
            if (inX && inY && (p.getZ() == s.z || p.getZ() == s.z - 1)) return s;
        }
        return null;
    }

    /** The `screens` list, for the zone of the call we are in (null = none). */
    public void apply(ServerLevel level, JsonObject msg, String zoneId, int[] zoneBox) {
        List<JsonObject> list = new ArrayList<>();
        if (zoneId != null && zoneBox != null) for (JsonElement e : msg.getAsJsonArray("list")) list.add(e.getAsJsonObject());
        StringBuilder k = new StringBuilder(zoneId + "|");
        for (JsonObject o : list) k.append(o.get("key").getAsString()).append(',');
        if (k.toString().equals(layoutKey)) {
            for (JsonObject o : list) {
                Screen s = shown.get(o.get("key").getAsString());
                if (s != null && !s.name.equals(o.get("name").getAsString())) {
                    s.name = o.get("name").getAsString();
                    if (s.label != null) ((TextDisplayAccess) s.label).soworker$setText(Component.literal(s.name));
                }
            }
            return;
        }
        clear(level);
        layoutKey = k.toString();
        if (list.isEmpty()) return;
        int[] box = zoneBox;
        int width = box[2] - box[0] + 1;
        // share first, then cameras by name (soworker sends them in that order); keep what fits side by side
        List<Screen> plan = new ArrayList<>();
        int used = 0;
        for (JsonObject o : list) {
            Screen s = new Screen();
            s.key = o.get("key").getAsString();
            s.name = o.get("name").getAsString();
            s.kind = o.get("kind").getAsString();
            s.cols = o.get("w").getAsInt() / 128;
            s.rows = o.get("h").getAsInt() / 128;
            int need = (plan.isEmpty() ? 0 : 1) + s.cols;
            if (used + need > Math.max(width, 2)) continue;
            used += need;
            plan.add(s);
        }
        int maxRows = plan.stream().mapToInt(s -> s.rows).max().orElse(0);
        int x = box[0] + Math.max(0, (width - used) / 2);
        int z = box[1];
        for (Screen s : plan) {
            s.x0 = x;
            s.z = z;
            s.top = BASE_Y + maxRows - 1;
            build(level, s);
            shown.put(s.key, s);
            x += s.cols + 1;
        }
    }

    private MapId newMap(ServerLevel level) {
        MapId id = spareMaps.poll();
        if (id == null) id = level.getFreeMapId();
        MapItemSavedData d = MapItemSavedData.createFresh(0, 0, (byte) 0, false, false, Level.OVERWORLD).locked();
        level.setMapData(id, d);
        return id;
    }

    private void build(ServerLevel level, Screen s) {
        s.maps = new MapId[s.cols * s.rows];
        for (int r = 0; r < s.rows; r++) {
            for (int c = 0; c < s.cols; c++) {
                int x = s.x0 + c, y = s.top - r;
                BlockPos back = new BlockPos(x, y, s.z - 1);
                level.setBlock(back, Blocks.CONCRETE.black().defaultBlockState(), Block.UPDATE_ALL);
                wall.put(back.asLong(), true);
                MapId id = newMap(level);
                s.maps[r * s.cols + c] = id;
                ItemStack map = new ItemStack(Items.FILLED_MAP);
                map.set(DataComponents.MAP_ID, id);
                GlowItemFrame f = new GlowItemFrame(level, new BlockPos(x, y, s.z), Direction.SOUTH);
                f.setItem(map, false);
                ((ItemFrameAccess) f).soworker$setFixed(true);
                f.setInvisible(true);
                Villagers.mark(f);
                level.addFreshEntity(f);
                s.frames.add(f);
            }
        }
        Display.TextDisplay label = EntityTypes.TEXT_DISPLAY.create(level, EntitySpawnReason.COMMAND);
        if (label != null) {
            label.snapTo(s.x0 + s.cols / 2.0, s.top + 1.15, s.z + 0.05, 0, 0);
            ((TextDisplayAccess) label).soworker$setText(Component.literal(s.name));
            ((DisplayAccess) label).soworker$setBillboard(Display.BillboardConstraints.FIXED);
            Villagers.mark(label);
            level.addFreshEntity(label);
            s.label = label;
        }
    }

    /** A binary frame: [u8 1][u16 key length][key][u16 w][u16 h][w*h indices], little-endian. */
    public void frame(ServerLevel level, byte[] msg) {
        ByteBuffer b = ByteBuffer.wrap(msg).order(ByteOrder.LITTLE_ENDIAN);
        if (b.get() != 1) return;
        int kl = Short.toUnsignedInt(b.getShort());
        byte[] kb = new byte[kl];
        b.get(kb);
        String key = new String(kb, StandardCharsets.UTF_8);
        int w = Short.toUnsignedInt(b.getShort()), h = Short.toUnsignedInt(b.getShort());
        int off = b.position();
        Screen s = shown.get(key);
        if (s == null || msg.length - off < w * h) return;
        List<ServerPlayer> near = new ArrayList<>();
        for (ServerPlayer p : level.players()) {
            if (p.distanceToSqr(s.x0, s.top, s.z) < 96 * 96) near.add(p);
        }
        for (int r = 0; r < s.rows; r++) {
            for (int c = 0; c < s.cols; c++) {
                MapId id = s.maps[r * s.cols + c];
                MapItemSavedData d = level.getMapData(id);
                if (d == null) continue;
                for (int y = 0; y < 128; y++) {
                    int sy = r * 128 + y;
                    if (sy >= h) break;
                    for (int x = 0; x < 128; x++) {
                        int sx = c * 128 + x;
                        if (sx >= w) break;
                        byte v = msg[off + sy * w + sx];
                        if (d.colors[x + y * 128] != v) d.setColor(x, y, v);
                    }
                }
                for (ServerPlayer p : near) {
                    d.getHoldingPlayer(p);
                    Packet<?> pk = d.getUpdatePacket(id, p);
                    if (pk != null) p.connection.send(pk);
                }
            }
        }
    }

    public void gone(ServerLevel level, String key) {
        // the next `screens` message lays the wall out again; until then the picture stays as it was
    }
}
