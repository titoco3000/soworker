package dev.soworker.mc;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.charset.StandardCharsets;
import net.minecraft.network.protocol.Packet;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.material.MapColor;
import net.minecraft.world.level.saveddata.maps.MapId;
import net.minecraft.world.level.saveddata.maps.MapItemSavedData;

/**
 * Our own camera on one locked map, the one Tools puts in the off-hand while the camera is on. soworker sends it as
 * a screen frame (type 1) with the key {@link #KEY}, 128x128, already mirrored.
 */
final class Preview {
    static final String KEY = "me#preview";
    private MapId id;

    /** The map, made the first time it is needed (one per session). */
    MapId map(ServerLevel level) {
        if (id == null) {
            id = level.getFreeMapId();
            level.setMapData(id, MapItemSavedData.createFresh(0, 0, (byte) 0, false, false, Level.OVERWORLD).locked());
        }
        return id;
    }

    /** True if the frame is the preview's (and it was drawn). */
    boolean frame(ServerLevel level, byte[] msg, ServerPlayer p) {
        ByteBuffer b = ByteBuffer.wrap(msg).order(ByteOrder.LITTLE_ENDIAN);
        if (msg.length < 3 || b.get() != 1) return false;
        int kl = Short.toUnsignedInt(b.getShort());
        if (kl != KEY.length() || msg.length < 3 + kl + 4) return false;
        byte[] kb = new byte[kl];
        b.get(kb);
        if (!KEY.equals(new String(kb, StandardCharsets.UTF_8))) return false;
        int w = Short.toUnsignedInt(b.getShort()), h = Short.toUnsignedInt(b.getShort());
        int off = b.position();
        if (w != 128 || h != 128 || msg.length - off < w * h) return true;
        MapItemSavedData d = level.getMapData(map(level));
        if (d == null) return true;
        for (int y = 0; y < 128; y++) {
            for (int x = 0; x < 128; x++) {
                byte v = msg[off + y * 128 + x];
                if (d.colors[x + y * 128] != v) d.setColor(x, y, v);
            }
        }
        send(d, p);
        return true;
    }

    /** The camera went off: a black map, so the next time it starts dark rather than with an old picture. */
    void clear(ServerLevel level, ServerPlayer p) {
        if (id == null) return;
        MapItemSavedData d = level.getMapData(id);
        if (d == null) return;
        byte black = MapColor.COLOR_BLACK.getPackedId(MapColor.Brightness.NORMAL);
        for (int y = 0; y < 128; y++) {
            for (int x = 0; x < 128; x++) {
                if (d.colors[x + y * 128] != black) d.setColor(x, y, black);
            }
        }
        send(d, p);
    }

    private void send(MapItemSavedData d, ServerPlayer p) {
        if (p == null) return;
        d.getHoldingPlayer(p);
        Packet<?> pk = d.getUpdatePacket(id, p);
        if (pk != null) p.connection.send(pk);
    }
}
