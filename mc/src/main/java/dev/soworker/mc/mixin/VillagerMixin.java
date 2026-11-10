package dev.soworker.mc.mixin;

import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.npc.villager.Villager;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

/** Villagers that stand for SoWork users have no brain: no wandering, work, sleep or panic. Navigation still ticks. */
@Mixin(Villager.class)
public abstract class VillagerMixin {
    @Inject(method = "customServerAiStep", at = @At("HEAD"), cancellable = true)
    private void soworker$noBrain(ServerLevel level, CallbackInfo ci) {
        if (((Villager) (Object) this).entityTags().contains("soworker")) {
            ci.cancel();
        }
    }
}
