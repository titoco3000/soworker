package dev.soworker.mc;

import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.codec.StreamCodec;
import net.minecraft.network.protocol.common.custom.CustomPacketPayload;
import net.minecraft.resources.Identifier;

/** Client to server: the player left the full-screen view of a screen share. */
public record UnwatchPayload() implements CustomPacketPayload {
    public static final Type<UnwatchPayload> TYPE = new Type<>(Identifier.fromNamespaceAndPath("soworker", "unwatch"));
    public static final StreamCodec<RegistryFriendlyByteBuf, UnwatchPayload> CODEC = StreamCodec.unit(new UnwatchPayload());

    @Override
    public Type<? extends CustomPacketPayload> type() {
        return TYPE;
    }
}
