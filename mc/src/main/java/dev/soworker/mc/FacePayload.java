package dev.soworker.mc;

import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.codec.StreamCodec;
import net.minecraft.network.protocol.common.custom.CustomPacketPayload;
import net.minecraft.resources.Identifier;

/**
 * Server to client: a camera frame for the face of entity {@code entity} (a coworker's mannequin, or the player),
 * {@code w}×{@code h} RGB. {@code w == 0}: that camera went off.
 */
public record FacePayload(int entity, int w, int h, byte[] rgb) implements CustomPacketPayload {
    public static final Type<FacePayload> TYPE = new Type<>(Identifier.fromNamespaceAndPath("soworker", "face"));
    public static final StreamCodec<RegistryFriendlyByteBuf, FacePayload> CODEC = StreamCodec.of(
            (buf, p) -> {
                buf.writeVarInt(p.entity);
                buf.writeVarInt(p.w);
                buf.writeVarInt(p.h);
                buf.writeByteArray(p.rgb);
            },
            buf -> new FacePayload(buf.readVarInt(), buf.readVarInt(), buf.readVarInt(), buf.readByteArray()));

    @Override
    public Type<? extends CustomPacketPayload> type() {
        return TYPE;
    }
}
