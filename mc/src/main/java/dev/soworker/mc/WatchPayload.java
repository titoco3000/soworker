package dev.soworker.mc;

import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.codec.StreamCodec;
import net.minecraft.network.protocol.common.custom.CustomPacketPayload;
import net.minecraft.resources.Identifier;

/**
 * Server to client, the full-screen view of a screen share: {@link #OPEN} (show it, {@code name} is whose it is),
 * {@link #FRAME} ({@code jpeg} is the next picture), {@link #CLOSE} (the share ended).
 */
public record WatchPayload(int kind, String name, byte[] jpeg) implements CustomPacketPayload {
    public static final int OPEN = 0, FRAME = 1, CLOSE = 2;
    /** A 1080p JPEG is a few hundred KB: well over a normal custom payload's limit. */
    public static final int MAX = 8 << 20;
    public static final Type<WatchPayload> TYPE = new Type<>(Identifier.fromNamespaceAndPath("soworker", "watch"));
    public static final StreamCodec<RegistryFriendlyByteBuf, WatchPayload> CODEC = StreamCodec.of(
            (buf, p) -> {
                buf.writeVarInt(p.kind);
                buf.writeUtf(p.name);
                buf.writeByteArray(p.jpeg);
            },
            buf -> new WatchPayload(buf.readVarInt(), buf.readUtf(), buf.readByteArray(MAX)));

    @Override
    public Type<? extends CustomPacketPayload> type() {
        return TYPE;
    }
}
