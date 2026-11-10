package dev.soworker.mc.mixin;

import dev.soworker.mc.SoworkerMod;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.entity.projectile.throwableitemprojectile.ThrownEnderpearl;
import net.minecraft.world.phys.HitResult;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

/**
 * The player's ender pearl in the office does SoWork's teleport (Shift+click) to where it lands instead of the vanilla
 * one: soworker moves the avatar and then puts the player there (no fall damage, no endermites).
 */
@Mixin(ThrownEnderpearl.class)
public abstract class EnderPearlMixin {
    @Inject(method = "onHit", at = @At("HEAD"), cancellable = true)
    private void soworker$teleport(HitResult hit, CallbackInfo ci) {
        ThrownEnderpearl self = (ThrownEnderpearl) (Object) this;
        if (self.level().isClientSide() || !(self.getOwner() instanceof ServerPlayer p)) return;
        if (!SoworkerMod.pearl(p, hit.getLocation())) return;
        ci.cancel();
        self.discard();
    }
}
