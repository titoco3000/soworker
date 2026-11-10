package dev.soworker.mc;

import com.mojang.math.Transformation;
import dev.soworker.mc.mixin.DisplayAccess;
import dev.soworker.mc.mixin.TextDisplayAccess;
import java.util.HashMap;
import java.util.Map;
import net.minecraft.ChatFormatting;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.Display;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import org.joml.Quaternionf;
import org.joml.Vector3f;

/** Each meeting room's name floating over its middle, hidden while the player stands inside that room. */
public final class ZoneLabels {
    static final double HEIGHT = MapBuilder.FLOOR_Y + 4.5;
    private final Map<String, Display.TextDisplay> shown = new HashMap<>();
    /** What the labels were made from (zone id, box and name), so a new map with the same rooms changes nothing. */
    private final Map<String, String> made = new HashMap<>();

    public void clear() {
        for (Display.TextDisplay t : shown.values()) t.discard();
        shown.clear();
        made.clear();
    }

    static boolean inside(int[] b, Entity p) {
        return p != null && p.getX() >= b[0] && p.getX() < b[2] + 1 && p.getZ() >= b[1] && p.getZ() < b[3] + 1;
    }

    public void update(ServerLevel level, MapBuilder map, Entity player) {
        shown.entrySet().removeIf(en -> {
            if (map.zones.containsKey(en.getKey())) return false;
            en.getValue().discard();
            made.remove(en.getKey());
            return true;
        });
        for (Map.Entry<String, int[]> en : map.zones.entrySet()) {
            String id = en.getKey();
            int[] b = en.getValue();
            String name = map.zoneNames.getOrDefault(id, "");
            String key = b[0] + "," + b[1] + "," + b[2] + "," + b[3] + "|" + name;
            Display.TextDisplay t = shown.get(id);
            boolean want = !name.isEmpty() && !inside(b, player);
            if (t != null && (!want || t.isRemoved() || !key.equals(made.get(id)))) {
                t.discard();
                shown.remove(id);
                t = null;
            }
            if (want && t == null) {
                t = EntityTypes.TEXT_DISPLAY.create(level, EntitySpawnReason.COMMAND);
                if (t == null) continue;
                t.snapTo((b[0] + b[2] + 1) / 2.0, HEIGHT, (b[1] + b[3] + 1) / 2.0, 0, 0);
                Villagers.mark(t);
                ((DisplayAccess) t).soworker$setBillboard(Display.BillboardConstraints.CENTER);
                ((DisplayAccess) t).soworker$setTransformation(new Transformation(
                        new Vector3f(), new Quaternionf(), new Vector3f(3f), new Quaternionf()));
                ((TextDisplayAccess) t).soworker$setBackgroundColor(0x60000000);
                ((TextDisplayAccess) t).soworker$setText(Component.literal(name).withStyle(ChatFormatting.GOLD, ChatFormatting.BOLD));
                level.addFreshEntity(t);
                shown.put(id, t);
                made.put(id, key);
            }
        }
    }
}
