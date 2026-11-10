package dev.soworker.mc;

import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;
import net.minecraft.ChatFormatting;
import net.minecraft.core.particles.ParticleTypes;
import net.minecraft.network.chat.Component;
import net.minecraft.network.chat.MutableComponent;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.npc.villager.Villager;
import net.minecraft.world.entity.Display;
import com.mojang.math.Transformation;
import dev.soworker.mc.mixin.DisplayAccess;
import dev.soworker.mc.mixin.TextDisplayAccess;
import org.joml.Quaternionf;
import org.joml.Vector3f;
import net.minecraft.world.entity.npc.villager.VillagerProfession;

/** One villager per other user: named, invulnerable, brainless (see VillagerMixin), walking to the user's position. */
public final class Villagers {
    public static final String TAG = "soworker";
    static final double Y = MapBuilder.FLOOR_Y + 1;
    /** SoWork walks 800 units/s = 7.6 blocks/s at two blocks per tile; 0.75 here was about 3.8 blocks/s. */
    static final double SPEED = 1.5;

    static final class U {
        Villager v;
        /** The name over the head: a text display riding the villager, bigger than a vanilla name tag. */
        Display.TextDisplay label;
        String name = "";
        double x, z;
        boolean walking, away, speaking, muted;
        int hand;
        String shown = "";
        Boolean nitwit;
        double lastX, lastZ;
        int still;
        double navX = Double.NaN, navZ = Double.NaN;
    }

    final Map<String, U> users = new HashMap<>();
    /** Every entity this run made (villagers, screens, labels, the board): any other tagged one is a leftover. */
    static final Set<java.util.UUID> OWNED = new HashSet<>();

    /** Tag an entity as ours, before it is added to the level. */
    public static void mark(Entity e) {
        e.addTag(TAG);
        OWNED.add(e.getUUID());
    }

    /** An entity loaded from the world with our tag that this run did not make: left from an earlier run. */
    public static boolean leftover(Entity e) {
        return e.entityTags().contains(TAG) && !OWNED.contains(e.getUUID());
    }

    public void clear() {
        for (U u : users.values()) remove(u);
        users.clear();
    }

    public void apply(ServerLevel level, JsonObject msg) {
        Set<String> seen = new HashSet<>();
        for (JsonElement e : msg.getAsJsonArray("list")) {
            JsonObject o = e.getAsJsonObject();
            String id = o.get("id").getAsString();
            seen.add(id);
            U u = users.get(id);
            boolean fresh = u == null;
            if (fresh) {
                u = new U();
                users.put(id, u);
            }
            u.name = o.get("name").getAsString();
            u.x = o.get("x").getAsDouble();
            u.z = o.get("z").getAsDouble();
            u.walking = o.get("walking").getAsBoolean();
            u.away = o.get("away").getAsBoolean();
            u.speaking = o.get("speaking").getAsBoolean();
            u.muted = o.get("muted").getAsBoolean();
            u.hand = o.has("hand") && !o.get("hand").isJsonNull() ? o.get("hand").getAsInt() : 0;
            if (fresh) spawn(level, u);
        }
        users.entrySet().removeIf(en -> {
            if (seen.contains(en.getKey())) return false;
            Villager v = en.getValue().v;
            if (v != null) level.sendParticles(ParticleTypes.POOF, v.getX(), v.getY() + 1, v.getZ(), 20, 0.3, 0.5, 0.3, 0.02);
            remove(en.getValue());
            return true;
        });
    }

    private static void remove(U u) {
        if (u.label != null) u.label.discard();
        if (u.v != null) u.v.discard();
        u.label = null;
        u.v = null;
    }

    private void spawn(ServerLevel level, U u) {
        remove(u);
        Villager v = EntityTypes.VILLAGER.create(level, EntitySpawnReason.COMMAND);
        if (v == null) return;
        v.snapTo(u.x, Y, u.z, 0, 0);
        mark(v);
        v.setPermanentlyInvulnerable(true);
        v.setSilent(true);
        v.setPersistenceRequired();
        level.addFreshEntity(v);
        Display.TextDisplay t = EntityTypes.TEXT_DISPLAY.create(level, EntitySpawnReason.COMMAND);
        if (t != null) {
            t.snapTo(u.x, Y + 2, u.z, 0, 0);
            mark(t);
            ((DisplayAccess) t).soworker$setBillboard(Display.BillboardConstraints.CENTER);
            ((DisplayAccess) t).soworker$setTransformation(new Transformation(
                    new Vector3f(0, 0.2f, 0), new Quaternionf(), new Vector3f(2.6f), new Quaternionf()));
            ((TextDisplayAccess) t).soworker$setBackgroundColor(0xA0000000);
            ((TextDisplayAccess) t).soworker$setFlags(Display.TextDisplay.FLAG_SEE_THROUGH);
            level.addFreshEntity(t);
            t.startRiding(v, true, false);
            u.label = t;
        }
        level.sendParticles(ParticleTypes.POOF, u.x, Y + 1, u.z, 20, 0.3, 0.5, 0.3, 0.02);
        u.v = v;
        u.shown = "";
        u.nitwit = null;
        u.lastX = u.x;
        u.lastZ = u.z;
    }

    /** A user waved: hearts over their head. */
    public void wave(ServerLevel level, String id) {
        U u = users.get(id);
        if (u != null && u.v != null) level.sendParticles(ParticleTypes.HEART, u.v.getX(), u.v.getY() + 2.2, u.v.getZ(), 5, 0.4, 0.2, 0.4, 0);
    }

    private static Component label(U u) {
        MutableComponent c = Component.literal(u.name).withStyle(ChatFormatting.BOLD);
        if (u.away) c = c.withStyle(ChatFormatting.GRAY);
        if (u.muted) c = c.append(Component.literal(" 🔇").withStyle(ChatFormatting.RED));
        if (u.hand > 0) c = c.append(Component.literal(" ✋ #" + u.hand).withStyle(ChatFormatting.GOLD));
        return c;
    }

    public void tick(ServerLevel level, long tick) {
        for (U u : users.values()) {
            Villager v = u.v;
            if (v == null || v.isRemoved() || u.label == null || u.label.isRemoved()) {
                spawn(level, u);
                v = u.v;
                if (v == null) continue;
            }
            String shown = u.name + "|" + u.away + "|" + u.muted + "|" + u.hand;
            if (!shown.equals(u.shown)) {
                v.setCustomName(label(u));
                if (u.label != null) ((TextDisplayAccess) u.label).soworker$setText(label(u));
                u.shown = shown;
            }
            if (u.label != null && u.label.getVehicle() != v) u.label.startRiding(v, true, false);
            if (u.nitwit == null || u.nitwit != u.away) {
                v.setVillagerData(v.getVillagerData().withProfession(level.registryAccess(), u.away ? VillagerProfession.NITWIT : VillagerProfession.NONE));
                u.nitwit = u.away;
            }
            v.setGlowingTag(u.speaking);
            if (u.speaking && tick % 8 == 0) {
                level.sendParticles(ParticleTypes.NOTE, v.getX(), v.getY() + 2.3, v.getZ(), 1, 0.3, 0.1, 0.3, 1);
            }
            walk(v, u);
        }
    }

    private static void walk(Villager v, U u) {
        double dx = u.x - v.getX(), dz = u.z - v.getZ();
        double d = Math.sqrt(dx * dx + dz * dz);
        if (d > 16 || v.getY() < Y - 0.5) { // far behind, or fallen off (or buried in) the floor
            v.getNavigation().stop();
            v.snapTo(u.x, Y, u.z, v.getYRot(), 0);
            u.still = 0;
            return;
        }
        if (d < 0.35) {
            if (!v.getNavigation().isDone()) v.getNavigation().stop();
            u.still = 0;
            return;
        }
        // no progress for 3 s (target inside a fence, or no path): jump there
        double moved = Math.hypot(v.getX() - u.lastX, v.getZ() - u.lastZ);
        u.lastX = v.getX();
        u.lastZ = v.getZ();
        u.still = moved < 0.01 ? u.still + 1 : 0;
        if (u.still > 60) {
            v.getNavigation().stop();
            v.snapTo(u.x, Y, u.z, v.getYRot(), 0);
            u.still = 0;
            return;
        }
        if (v.getNavigation().isDone() || Math.hypot(u.x - u.navX, u.z - u.navZ) > 0.3) {
            v.getNavigation().moveTo(u.x, Y, u.z, SPEED);
            u.navX = u.x;
            u.navZ = u.z;
        }
    }
}
