package dev.soworker.mc.mixin;

import net.minecraft.world.entity.Display;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.gen.Invoker;

@Mixin(Display.class)
public interface DisplayAccess {
    @Invoker("setBillboardConstraints")
    void soworker$setBillboard(Display.BillboardConstraints c);

    @Invoker("setTransformation")
    void soworker$setTransformation(com.mojang.math.Transformation t);
}
