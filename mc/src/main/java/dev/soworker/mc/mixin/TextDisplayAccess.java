package dev.soworker.mc.mixin;

import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.Display;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.gen.Invoker;

@Mixin(Display.TextDisplay.class)
public interface TextDisplayAccess {
    @Invoker("setText")
    void soworker$setText(Component text);

    @Invoker("setLineWidth")
    void soworker$setLineWidth(int width);

    @Invoker("setFlags")
    void soworker$setFlags(byte flags);

    @Invoker("setBackgroundColor")
    void soworker$setBackgroundColor(int argb);
}
