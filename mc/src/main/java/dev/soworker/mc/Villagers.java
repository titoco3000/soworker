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
import net.minecraft.world.entity.Pose;
import net.minecraft.world.effect.MobEffectInstance;
import net.minecraft.world.effect.MobEffects;
import net.minecraft.world.entity.decoration.Mannequin;
import dev.soworker.mc.mixin.MannequinAccess;
import com.mojang.math.Transformation;
import dev.soworker.mc.mixin.DisplayAccess;
import dev.soworker.mc.mixin.TextDisplayAccess;
import org.joml.Quaternionf;
import org.joml.Vector3f;
import net.minecraft.world.entity.projectile.throwableitemprojectile.Snowball;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;

/**
 * One figure per other user. A brainless (see VillagerMixin), invisible villager does the walking, with vanilla
 * pathfinding round the fences; a mannequin (the player model, so a camera can go on its face: see client.FaceLayer)
 * is put where the villager is every tick and is what everyone sees. Named, invulnerable.
 */
public final class Villagers {
    public static final String TAG = "soworker";
    /** A snowball a coworker threw to show a wave. */
    public static final String WAVE_TAG = "soworker_wave";
    static final double THROW = 1.5;
    static final double Y = MapBuilder.FLOOR_Y + 1;
    /** SoWork walks 800 units/s = 7.6 blocks/s at two blocks per tile; 0.75 here was about 3.8 blocks/s. */
    static final double SPEED = 1.5;

    static final class U {
        Villager v;
        Mannequin m;
        /** The name over the head: a text display riding the villager, bigger than a vanilla name tag. */
        Display.TextDisplay label;
        String name = "";
        double x, z;
        boolean walking, away, speaking, muted;
        int hand;
        String shown = "";
        Boolean crouched;
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
            Entity v = en.getValue().m != null ? en.getValue().m : en.getValue().v;
            if (v != null) level.sendParticles(ParticleTypes.POOF, v.getX(), v.getY() + 1, v.getZ(), 20, 0.3, 0.5, 0.3, 0.02);
            remove(en.getValue());
            return true;
        });
    }

    private static void remove(U u) {
        if (u.label != null) u.label.discard();
        if (u.v != null) u.v.discard();
        if (u.m != null) u.m.discard();
        u.label = null;
        u.v = null;
        u.m = null;
    }

    /**
     * The walker is never seen. A bare {@code setInvisible} does not last: a living entity resets the flag from its
     * effects, so it gets an endless invisibility effect without particles too (and is checked every tick).
     */
    private static void hide(Villager v) {
        if (!v.hasEffect(MobEffects.INVISIBILITY)) {
            v.addEffect(new MobEffectInstance(MobEffects.INVISIBILITY, MobEffectInstance.INFINITE_DURATION, 0, false, false));
        }
        if (!v.isInvisible()) v.setInvisible(true);
    }

    /** The figure people see for a user (the mannequin), or null. */
    public Entity shown(String id) {
        U u = users.get(id);
        return u == null ? null : u.m;
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
        hide(v);
        level.addFreshEntity(v);
        Mannequin m = EntityTypes.MANNEQUIN.create(level, EntitySpawnReason.COMMAND);
        if (m != null) {
            m.snapTo(u.x, Y, u.z, 0, 0);
            mark(m);
            m.setPermanentlyInvulnerable(true);
            m.setSilent(true);
            m.setNoGravity(true);
            ((MannequinAccess) m).soworker$setHideDescription(true);
            ((MannequinAccess) m).soworker$setImmovable(true);
            level.addFreshEntity(m);
            u.m = m;
        }
        Entity body = m != null ? m : v;
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
            t.startRiding(body, true, false);
            u.label = t;
        }
        level.sendParticles(ParticleTypes.POOF, u.x, Y + 1, u.z, 20, 0.3, 0.5, 0.3, 0.02);
        u.v = v;
        u.shown = "";
        u.crouched = null;
        u.lastX = u.x;
        u.lastZ = u.z;
    }

    /** The user a villager stands for, or null. */
    public String idOf(Entity e) {
        for (Map.Entry<String, U> en : users.entrySet()) if (en.getValue().v == e || en.getValue().m == e) return en.getKey();
        return null;
    }

    /** A user waved: hearts over their head. */
    public void wave(ServerLevel level, String id) {
        U u = users.get(id);
        if (u != null && u.m != null) hearts(level, u.m);
    }

    public static void hearts(ServerLevel level, Entity e) {
        level.sendParticles(ParticleTypes.HEART, e.getX(), e.getY() + e.getBbHeight() + 0.2, e.getZ(), 5, 0.4, 0.2, 0.4, 0);
    }

    /**
     * {@code from} waved at {@code to}: from's villager throws a snowball at the target (the player when the wave is
     * for us, else to's villager). It only shows: tagged snowballs never hurt or push (see SnowballMixin), the hearts
     * come where it lands. Nobody to throw from or at: just hearts over whoever is shown.
     */
    public void wave(ServerLevel level, String from, String to, Entity player) {
        U f = users.get(from);
        U t = users.get(to);
        Entity target = player != null ? player : t != null ? t.m : null;
        if (f == null || f.m == null || target == null || target == f.m || target == f.v) {
            wave(level, from);
            return;
        }
        Mannequin v = f.m;
        double dx = target.getX() - v.getX(), dz = target.getZ() - v.getZ();
        double d = Math.hypot(dx, dz);
        float yaw = (float) Math.toDegrees(Math.atan2(-dx, dz));
        v.setYRot(yaw);
        v.setYHeadRot(yaw);
        Snowball s = new Snowball(level, v, new ItemStack(Items.SNOWBALL));
        mark(s);
        s.addTag(WAVE_TAG);
        // aim a little high for the drop on the way (gravity 0.03 blocks/tick², at 1.5 blocks/tick)
        double ticks = d / THROW;
        double dy = target.getY() + target.getBbHeight() * 0.6 - s.getY() + 0.015 * ticks * ticks;
        s.shoot(dx, dy, dz, (float) THROW, 0);
        level.addFreshEntity(s);
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
            if (v == null || v.isRemoved() || u.m == null || u.m.isRemoved() || u.label == null || u.label.isRemoved()) {
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
            Mannequin m = u.m;
            hide(v);
            if (u.label != null && u.label.getVehicle() != m) u.label.startRiding(m, true, false);
            if (u.crouched == null || u.crouched != u.away) {
                m.setPose(u.away ? Pose.CROUCHING : Pose.STANDING); // away: crouching (the name is grey too)
                u.crouched = u.away;
            }
            m.setGlowingTag(u.speaking);
            if (u.speaking && tick % 8 == 0) {
                level.sendParticles(ParticleTypes.NOTE, m.getX(), m.getY() + 2.3, m.getZ(), 1, 0.3, 0.1, 0.3, 1);
            }
            walk(v, u);
            // the mannequin stands where the walker is, facing where it walks (or keeps facing when it stops)
            double mx = v.getX() - m.getX(), mz = v.getZ() - m.getZ();
            float yaw = Math.hypot(mx, mz) > 0.01 ? (float) Math.toDegrees(Math.atan2(-mx, mz)) : m.getYRot();
            m.setPos(v.getX(), v.getY(), v.getZ());
            m.setYRot(yaw);
            m.setYHeadRot(yaw);
            m.setYBodyRot(yaw);
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
        if (d < 0.05) {
            if (!v.getNavigation().isDone()) v.getNavigation().stop();
            u.still = 0;
            return;
        }
        // The last stretch, inside the target's own tile (its blocks, no fence between them): step straight there.
        // A user standing still is at a tile centre, which is a block corner, and the pathfinder only ends paths at
        // block centres, 0.7 away: left to it, the villager re-paths every tick and shuffles round the corner forever.
        if (d < 1.0) {
            if (!v.getNavigation().isDone()) v.getNavigation().stop();
            double step = Math.min(d, SPEED * 0.2);
            v.setDeltaMovement(0, v.getDeltaMovement().y, 0);
            v.setPos(v.getX() + dx / d * step, v.getY(), v.getZ() + dz / d * step);
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
