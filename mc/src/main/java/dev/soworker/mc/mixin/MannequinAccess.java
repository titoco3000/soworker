package dev.soworker.mc.mixin;

import net.minecraft.world.entity.decoration.Mannequin;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.gen.Invoker;

@Mixin(Mannequin.class)
public interface MannequinAccess {
    @Invoker("setHideDescription")
    void soworker$setHideDescription(boolean hide);

    @Invoker("setImmovable")
    void soworker$setImmovable(boolean immovable);
}
