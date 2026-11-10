package dev.soworker.mc;

import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import java.util.ArrayList;
import java.util.List;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.vehicle.minecart.AbstractMinecart;
import net.minecraft.world.entity.vehicle.minecart.Minecart;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.RailBlock;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.block.state.properties.RailShape;
import net.minecraft.world.phys.Vec3;

/**
 * The ride to the desk: the bridge sends a route of floor blocks (four-way steps); we lay a rail track along it, seat
 * the player in a minecart at its start and push the cart along the route each tick. At the end (or when the player
 * gets out, or the cart stops getting anywhere) the player gets off, and the cart and the track go away.
 */
public final class Rail {
    /** Blocks per tick: about the avatar's 7.6 blocks/s, under the minecart's 0.4 limit. */
    static final double SPEED = 0.38, SLOW = 0.18;
    static final int FLAGS = Block.UPDATE_CLIENTS | Block.UPDATE_KNOWN_SHAPE;

    private List<BlockPos> route;
    private final List<BlockPos> laid = new ArrayList<>();
    private AbstractMinecart cart;
    private int idx, still;

    public boolean active() {
        return route != null;
    }

    /** Which way a route step goes: the shape a rail needs to join the steps before and after it. */
    static RailShape shape(BlockPos prev, BlockPos at, BlockPos next) {
        boolean n = false, s = false, e = false, w = false;
        for (BlockPos o : new BlockPos[] {prev, next}) {
            if (o == null) continue;
            if (o.getZ() < at.getZ()) n = true;
            if (o.getZ() > at.getZ()) s = true;
            if (o.getX() > at.getX()) e = true;
            if (o.getX() < at.getX()) w = true;
        }
        if (n && e) return RailShape.NORTH_EAST;
        if (n && w) return RailShape.NORTH_WEST;
        if (s && e) return RailShape.SOUTH_EAST;
        if (s && w) return RailShape.SOUTH_WEST;
        return e || w ? RailShape.EAST_WEST : RailShape.NORTH_SOUTH;
    }

    public void start(ServerLevel level, ServerPlayer p, JsonArray path) {
        finish(level, p);
        List<BlockPos> r = new ArrayList<>();
        for (JsonElement e : path) {
            JsonArray a = e.getAsJsonArray();
            r.add(new BlockPos(a.get(0).getAsInt(), MapBuilder.FLOOR_Y + 1, a.get(1).getAsInt()));
        }
        if (r.size() < 2) return;
        for (BlockPos b : r) {
            if (!level.getBlockState(b).isAir()) return; // something in the way: walk instead
        }
        // twice: placing a rail reshapes it to its neighbours as they are then; the second pass sets the shapes we want
        for (int pass = 0; pass < 2; pass++) {
            for (int i = 0; i < r.size(); i++) {
                BlockState s = Blocks.RAIL.defaultBlockState().setValue(RailBlock.SHAPE,
                        shape(i > 0 ? r.get(i - 1) : null, r.get(i), i + 1 < r.size() ? r.get(i + 1) : null));
                level.setBlock(r.get(i), s, FLAGS);
                if (pass == 0) laid.add(r.get(i));
            }
        }
        BlockPos a = r.get(0);
        Minecart c = AbstractMinecart.createMinecart(level, a.getX() + 0.5, a.getY(), a.getZ() + 0.5, EntityTypes.MINECART,
                EntitySpawnReason.COMMAND, ItemStack.EMPTY, null);
        if (c == null) {
            removeTrack(level);
            return;
        }
        Villagers.mark(c);
        level.addFreshEntity(c);
        p.stopRiding();
        p.connection.teleport(a.getX() + 0.5, a.getY(), a.getZ() + 0.5, p.getYRot(), p.getXRot());
        if (!p.startRiding(c, true, false)) {
            c.discard();
            removeTrack(level);
            return;
        }
        route = r;
        cart = c;
        idx = 0;
        still = 0;
    }

    public void tick(ServerLevel level, ServerPlayer p) {
        if (route == null) return;
        if (cart == null || cart.isRemoved() || p.getVehicle() != cart) {
            finish(level, p); // the player got out
            return;
        }
        // how far along: the nearest route block a little ahead of where we were
        Vec3 at = cart.position();
        int best = idx;
        double bd = Double.MAX_VALUE;
        for (int i = idx; i < Math.min(route.size(), idx + 6); i++) {
            BlockPos b = route.get(i);
            double d = Math.hypot(b.getX() + 0.5 - at.x, b.getZ() + 0.5 - at.z);
            if (d < bd) {
                bd = d;
                best = i;
            }
        }
        still = best == idx ? still + 1 : 0;
        idx = best;
        int last = route.size() - 1;
        if ((idx == last && bd < 0.4) || still > 60) {
            finish(level, p);
            return;
        }
        BlockPos from = route.get(Math.min(idx, last - 1)), to = route.get(Math.min(idx, last - 1) + 1);
        double v = last - idx <= 2 ? SLOW : SPEED;
        cart.setDeltaMovement(new Vec3((to.getX() - from.getX()) * v, 0, (to.getZ() - from.getZ()) * v));
    }

    /** Off the cart where it is, cart and track gone. */
    public void finish(ServerLevel level, ServerPlayer p) {
        if (cart != null) {
            Vec3 at = cart.position();
            cart.ejectPassengers();
            cart.discard();
            if (p != null) p.connection.teleport(at.x, MapBuilder.FLOOR_Y + 1, at.z, p.getYRot(), p.getXRot());
        }
        cart = null;
        route = null;
        removeTrack(level);
    }

    private void removeTrack(ServerLevel level) {
        for (BlockPos b : laid) {
            if (level.getBlockState(b).is(Blocks.RAIL)) level.setBlock(b, Blocks.AIR.defaultBlockState(), FLAGS);
        }
        laid.clear();
    }
}
