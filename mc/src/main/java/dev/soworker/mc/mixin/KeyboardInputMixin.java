package dev.soworker.mc.mixin;

import dev.soworker.mc.client.Faces;
import net.minecraft.client.player.ClientInput;
import net.minecraft.client.player.KeyboardInput;
import net.minecraft.world.entity.player.Input;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

/** Away in SoWork: sneak is held down for the player (and sprinting is off), like away coworkers, who crouch. */
@Mixin(KeyboardInput.class)
public abstract class KeyboardInputMixin {
    @Inject(method = "tick", at = @At("TAIL"))
    private void soworker$crouchWhenAway(CallbackInfo ci) {
        if (!Faces.away) return;
        ClientInput self = (ClientInput) (Object) this;
        Input k = self.keyPresses;
        self.keyPresses = new Input(k.forward(), k.backward(), k.left(), k.right(), k.jump(), true, false);
    }
}
