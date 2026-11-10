package dev.soworker.mc;

import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.codec.ByteBufCodecs;
import net.minecraft.network.codec.StreamCodec;
import net.minecraft.network.protocol.common.custom.CustomPacketPayload;
import net.minecraft.resources.Identifier;

/** Server to client: we are away in SoWork (the client then holds sneak for the player), or no longer. */
public record AwayPayload(boolean away) implements CustomPacketPayload {
    public static final Type<AwayPayload> TYPE = new Type<>(Identifier.fromNamespaceAndPath("soworker", "away"));
    public static final StreamCodec<RegistryFriendlyByteBuf, AwayPayload> CODEC =
            StreamCodec.composite(ByteBufCodecs.BOOL, AwayPayload::away, AwayPayload::new);

    @Override
    public Type<? extends CustomPacketPayload> type() {
        return TYPE;
    }
}
