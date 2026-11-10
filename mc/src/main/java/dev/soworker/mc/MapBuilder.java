package dev.soworker.mc;

import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import java.util.ArrayDeque;
import java.util.HashMap;
import java.util.Map;
import net.minecraft.core.BlockPos;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.resources.Identifier;
import net.minecraft.tags.FluidTags;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockState;

/**
 * Builds the office from the {@code world} message: grass floor at y = 64, meeting rooms floored with the material the
 * bridge chose for each, oak fences at y = 65 and a barrier ring (y = 65 and 66). In a sea the office is an island: the
 * floor stands on dirt down to the sea bed, with a sandy beach shelving into the water round it. Only blocks that
 * differ from the last build change; blocks we placed that are no longer wanted become water below the sea surface and
 * air above it. What we placed is remembered in the world's office file across restarts.
 */
public final class MapBuilder {
    public static final int FLOOR_Y = 64;
    /** How far the beach reaches out from the island, and how deep we build down at most. */
    static final int BEACH = 6, MAX_DEPTH = 64;
    /** Blocks we placed: packed BlockPos -> block. */
    final Map<Long, Block> placed = new HashMap<>();
    private final ArrayDeque<Map.Entry<Long, Block>> pending = new ArrayDeque<>();
    /** Zone boxes from the latest map, by zone id: {i0, j0, i1, j1} plus name. */
    public final Map<String, int[]> zones = new HashMap<>();
    public final Map<String, String> zoneNames = new HashMap<>();
    public int minX, minZ, maxX, maxZ;
    public boolean hasMap;
    /** The sea round the office: its top water block and the lowest water block (no sea: top < bottom). */
    int seaTop = Integer.MIN_VALUE, seaBottom = Integer.MAX_VALUE;

    private static void put(Map<Long, Block> m, JsonArray tiles, int y, Block b) {
        for (JsonElement e : tiles) {
            JsonArray p = e.getAsJsonArray();
            m.put(BlockPos.asLong(p.get(0).getAsInt(), y, p.get(1).getAsInt()), b);
        }
    }

    static Block block(String name, Block fallback) {
        Block b = BuiltInRegistries.BLOCK.getValue(Identifier.withDefaultNamespace(name));
        return b == null || b == Blocks.AIR ? fallback : b;
    }

    /** Find the sea a little way off the map: scan down from above the floor for water. */
    void findSea(ServerLevel level, int x, int z) {
        seaTop = Integer.MIN_VALUE;
        seaBottom = Integer.MAX_VALUE;
        BlockPos.MutableBlockPos p = new BlockPos.MutableBlockPos();
        int y = FLOOR_Y + 8;
        while (y > level.getMinY() && level.getBlockState(p.set(x, y, z)).isAir()) y--;
        if (!level.getFluidState(p.set(x, y, z)).is(FluidTags.WATER)) return;
        seaTop = y;
        while (y > level.getMinY() && level.getFluidState(p.set(x, y - 1, z)).is(FluidTags.WATER)) y--;
        seaBottom = y;
    }

    boolean inSea(int y) {
        return y <= seaTop && y >= seaBottom;
    }

    /** Work out the blocks for a new map and queue the changes. */
    public void apply(JsonObject w, ServerLevel level) {
        Map<Long, Block> want = new HashMap<>();
        put(want, w.getAsJsonArray("floors"), FLOOR_Y, Blocks.GRASS_BLOCK);
        JsonArray mats = w.has("materials") ? w.getAsJsonArray("materials") : new JsonArray();
        Block[] mat = new Block[Math.max(1, mats.size())];
        mat[0] = Blocks.COBBLESTONE;
        for (int i = 0; i < mats.size(); i++) mat[i] = block(mats.get(i).getAsString(), Blocks.COBBLESTONE);
        for (JsonElement e : w.getAsJsonArray("meeting")) {
            JsonArray p = e.getAsJsonArray();
            int k = p.size() > 2 ? p.get(2).getAsInt() : 0;
            want.put(BlockPos.asLong(p.get(0).getAsInt(), FLOOR_Y, p.get(1).getAsInt()), mat[k >= 0 && k < mat.length ? k : 0]);
        }
        put(want, w.getAsJsonArray("fences"), FLOOR_Y + 1, Blocks.OAK_FENCE);
        put(want, w.getAsJsonArray("barriers"), FLOOR_Y + 1, Blocks.BARRIER);
        put(want, w.getAsJsonArray("barriers"), FLOOR_Y + 2, Blocks.BARRIER);
        // the ground of the island: under the fences and the barrier ring too
        java.util.Set<Long> land = new java.util.HashSet<>();
        for (String k : new String[] {"floors", "meeting", "fences", "barriers"}) {
            for (JsonElement e : w.getAsJsonArray(k)) {
                JsonArray p = e.getAsJsonArray();
                land.add(BlockPos.asLong(p.get(0).getAsInt(), 0, p.get(1).getAsInt()));
            }
        }
        for (long c : land) want.putIfAbsent(BlockPos.asLong(BlockPos.getX(c), FLOOR_Y, BlockPos.getZ(c)), Blocks.GRASS_BLOCK);
        minX = minZ = Integer.MAX_VALUE;
        maxX = maxZ = Integer.MIN_VALUE;
        for (long c : land) {
            minX = Math.min(minX, BlockPos.getX(c));
            minZ = Math.min(minZ, BlockPos.getZ(c));
            maxX = Math.max(maxX, BlockPos.getX(c));
            maxZ = Math.max(maxZ, BlockPos.getZ(c));
        }
        hasMap = !land.isEmpty();
        if (hasMap) findSea(level, minX - BEACH - 24, minZ - BEACH - 24);
        if (seaTop >= seaBottom) {
            int bottom = Math.max(seaBottom, FLOOR_Y - MAX_DEPTH);
            for (long c : land) {
                for (int y = FLOOR_Y - 1; y >= bottom; y--) want.put(BlockPos.asLong(BlockPos.getX(c), y, BlockPos.getZ(c)), Blocks.DIRT);
            }
            // the beach: sand shelving down one block every two out from the island, into the water
            Map<Long, Integer> ring = new HashMap<>();
            for (long c : land) {
                int x = BlockPos.getX(c), z = BlockPos.getZ(c);
                for (int dx = -BEACH; dx <= BEACH; dx++) {
                    for (int dz = -BEACH; dz <= BEACH; dz++) {
                        long n = BlockPos.asLong(x + dx, 0, z + dz);
                        if (land.contains(n)) continue;
                        int d = (int) Math.ceil(Math.hypot(dx, dz));
                        if (d > BEACH) continue;
                        ring.merge(n, d, Math::min);
                    }
                }
            }
            for (Map.Entry<Long, Integer> e : ring.entrySet()) {
                int x = BlockPos.getX(e.getKey()), z = BlockPos.getZ(e.getKey());
                int top = FLOOR_Y - 1 - (e.getValue() - 1) / 2;
                for (int y = top; y >= bottom; y--) want.put(BlockPos.asLong(x, y, z), y >= top - 1 ? Blocks.SAND : Blocks.SANDSTONE);
            }
            minX -= BEACH;
            minZ -= BEACH;
            maxX += BEACH;
            maxZ += BEACH;
        }
        pending.clear();
        for (Map.Entry<Long, Block> e : placed.entrySet()) {
            if (!want.containsKey(e.getKey())) pending.add(Map.entry(e.getKey(), Blocks.AIR));
        }
        // bottom up: sand never has water under it to fall into
        want.entrySet().stream().sorted(java.util.Comparator.comparingInt(e -> BlockPos.getY(e.getKey()))).forEach(pending::add);
        zones.clear();
        zoneNames.clear();
        for (JsonElement e : w.getAsJsonArray("zones")) {
            JsonObject z = e.getAsJsonObject();
            String id = z.get("id").getAsString();
            zones.put(id, new int[] {z.get("i0").getAsInt(), z.get("j0").getAsInt(), z.get("i1").getAsInt(), z.get("j1").getAsInt()});
            zoneNames.put(id, z.get("name").getAsString());
        }
    }

    /** Chunks we keep loaded (packed x, z), so villagers walk and blocks change with the player anywhere. */
    private final java.util.Set<Long> forced = new java.util.HashSet<>();

    /** Keep the map's chunks loaded, and let go of the ones it no longer covers. */
    public void forceChunks(ServerLevel level) {
        java.util.Set<Long> want = new java.util.HashSet<>();
        if (hasMap) {
            for (int cx = (minX - 2) >> 4; cx <= (maxX + 2) >> 4; cx++)
                for (int cz = (minZ - 2) >> 4; cz <= (maxZ + 2) >> 4; cz++) want.add(net.minecraft.world.level.ChunkPos.pack(cx, cz));
        }
        for (long c : forced) if (!want.contains(c)) level.setChunkForced(net.minecraft.world.level.ChunkPos.getX(c), net.minecraft.world.level.ChunkPos.getZ(c), false);
        for (long c : want) if (!forced.contains(c)) level.setChunkForced(net.minecraft.world.level.ChunkPos.getX(c), net.minecraft.world.level.ChunkPos.getZ(c), true);
        forced.clear();
        forced.addAll(want);
    }

    /** Place up to {@code budget} queued blocks. Returns true while work remains. */
    public boolean tick(ServerLevel level, int budget) {
        BlockPos.MutableBlockPos pos = new BlockPos.MutableBlockPos();
        while (budget-- > 0 && !pending.isEmpty()) {
            Map.Entry<Long, Block> e = pending.poll();
            pos.set(e.getKey());
            // what we take away goes back to the sea it was taken from
            BlockState s = e.getValue() == Blocks.AIR && inSea(pos.getY()) ? Blocks.WATER.defaultBlockState() : e.getValue().defaultBlockState();
            if (!level.getBlockState(pos).is(s.getBlock())) {
                level.setBlock(pos, s, Block.UPDATE_ALL);
            }
            if (e.getValue() == Blocks.AIR) placed.remove(e.getKey());
            else placed.put(e.getKey(), e.getValue());
        }
        return !pending.isEmpty();
    }

    public boolean busy() {
        return !pending.isEmpty();
    }
}
