package dev.soworker.mc.mixin;

import dev.soworker.mc.Screens;
import dev.soworker.mc.SoworkerMod;
import net.minecraft.world.phys.HitResult;
import dev.soworker.mc.Villagers;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.decoration.Mannequin;
import net.minecraft.world.entity.npc.villager.Villager;
import net.minecraft.world.entity.projectile.throwableitemprojectile.Snowball;
import net.minecraft.world.phys.EntityHitResult;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

/**
 * Snowballs: waves, and opening a screen share full screen. One the player throws at a coworker's villager waves at that coworker; one a coworker
 * throws (tagged {@link Villagers#WAVE_TAG}) shows a wave and lands as hearts. Neither hurts or pushes.
 */
@Mixin(Snowball.class)
public abstract class SnowballMixin {
    /** The player's snowball on a screen share's wall (its black backing, or a map frame): watch it full screen. */
    @Inject(method = "onHit", at = @At("HEAD"), cancellable = true)
    private void soworker$watch(HitResult hit, CallbackInfo ci) {
        Snowball self = (Snowball) (Object) this;
        if (!(self.level() instanceof ServerLevel) || self.entityTags().contains(Villagers.WAVE_TAG)) return;
        if (!(self.getOwner() instanceof ServerPlayer p)) return;
        Screens.Screen s = SoworkerMod.screenHit(hit);
        if (s == null) return;
        ci.cancel();
        self.discard();
        SoworkerMod.snowballWatch(p, s);
    }

    @Inject(method = "onHitEntity", at = @At("HEAD"), cancellable = true)
    private void soworker$wave(EntityHitResult hit, CallbackInfo ci) {
        Snowball self = (Snowball) (Object) this;
        if (!(self.level() instanceof ServerLevel level)) return;
        if (self.entityTags().contains(Villagers.WAVE_TAG)) {
            ci.cancel();
            Villagers.hearts(level, hit.getEntity());
            return;
        }
        Entity e = hit.getEntity();
        if (!(e instanceof Villager || e instanceof Mannequin) || !e.entityTags().contains(Villagers.TAG)) return;
        ci.cancel();
        if (self.getOwner() instanceof ServerPlayer p) SoworkerMod.snowballWave(p, e);
    }
}
