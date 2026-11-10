package dev.soworker.mc;

import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import dev.soworker.mc.mixin.DisplayAccess;
import dev.soworker.mc.mixin.TextDisplayAccess;
import net.minecraft.ChatFormatting;
import net.minecraft.network.chat.Component;
import net.minecraft.network.chat.MutableComponent;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.Display;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;

/** The meeting board: a text display at the north edge of the zone we are in, while we are in its call. */
public final class Panel {
    private Display.TextDisplay board;
    private String zone;
    private String shown = "";

    public void clear() {
        if (board != null) board.discard();
        board = null;
        zone = null;
        shown = "";
    }

    public void update(ServerLevel level, JsonObject call, int[] box, String zoneName) {
        String joined = call != null && call.has("joined") && !call.get("joined").isJsonNull() ? call.get("joined").getAsString() : null;
        if (joined == null || box == null) {
            clear();
            return;
        }
        if (!joined.equals(zone) || board == null || board.isRemoved()) {
            clear();
            zone = joined;
            board = EntityTypes.TEXT_DISPLAY.create(level, EntitySpawnReason.COMMAND);
            if (board == null) return;
            board.snapTo((box[0] + box[2] + 1) / 2.0, MapBuilder.FLOOR_Y + 1.1, box[1] + 0.6, 0, 0);
            ((DisplayAccess) board).soworker$setBillboard(Display.BillboardConstraints.VERTICAL);
            ((TextDisplayAccess) board).soworker$setLineWidth(200);
            // half size: readable from the room without filling the view
            ((DisplayAccess) board).soworker$setTransformation(new com.mojang.math.Transformation(
                    new org.joml.Vector3f(), new org.joml.Quaternionf(), new org.joml.Vector3f(0.45f), new org.joml.Quaternionf()));
            ((TextDisplayAccess) board).soworker$setBackgroundColor(0x90000000);
            Villagers.mark(board);
            level.addFreshEntity(board);
        }
        MutableComponent t = Component.literal(zoneName != null ? zoneName : "Meeting").withStyle(ChatFormatting.GOLD, ChatFormatting.BOLD);
        if (call.get("rec").getAsBoolean()) {
            String clock = call.has("rec_clock") && !call.get("rec_clock").isJsonNull() ? call.get("rec_clock").getAsString() : "";
            t.append(Component.literal("\n⏺ REC " + clock).withStyle(ChatFormatting.RED));
        }
        for (JsonElement e : call.getAsJsonArray("participants")) {
            JsonObject p = e.getAsJsonObject();
            boolean mic = p.get("mic").getAsBoolean(), cam = p.get("cam").getAsBoolean(), sp = p.has("speaking") && p.get("speaking").getAsBoolean();
            t.append(Component.literal("\n" + (mic ? "🎙 " : "🔇 ")).withStyle(mic ? ChatFormatting.GREEN : ChatFormatting.RED));
            t.append(Component.literal(p.get("name").getAsString()).withStyle(sp ? ChatFormatting.AQUA : ChatFormatting.WHITE));
            if (cam) t.append(Component.literal(" 📷").withStyle(ChatFormatting.GRAY));
        }
        var hands = call.getAsJsonArray("hands");
        if (!hands.isEmpty()) {
            StringBuilder h = new StringBuilder("\n✋ ");
            int n = 1;
            for (JsonElement e : hands) {
                String id = e.getAsString();
                String name = id;
                for (JsonElement pe : call.getAsJsonArray("participants")) {
                    JsonObject p = pe.getAsJsonObject();
                    if (!p.get("id").isJsonNull() && p.get("id").getAsString().equals(id)) name = p.get("name").getAsString();
                }
                h.append(n++ > 1 ? ", " : "").append(name);
            }
            t.append(Component.literal(h.toString()).withStyle(ChatFormatting.YELLOW));
        }
        String key = call.toString() + zoneName;
        if (!key.equals(shown)) {
            ((TextDisplayAccess) board).soworker$setText(t);
            shown = key;
        }
    }
}
