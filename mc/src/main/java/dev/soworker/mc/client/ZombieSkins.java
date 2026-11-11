package dev.soworker.mc.client;

import com.mojang.blaze3d.platform.NativeImage;
import dev.soworker.mc.Skins;
import dev.soworker.mc.SoworkerMod;
import java.io.InputStream;
import net.minecraft.client.Minecraft;
import net.minecraft.client.renderer.texture.DynamicTexture;
import net.minecraft.core.ClientAsset;
import net.minecraft.resources.Identifier;

/**
 * Zombie and husk as player skins (see Skins): their textures leave the left arm and leg blank, as the zombie model
 * mirrors the right ones, so the right limbs are copied over mirrored, the way the game converts old 64x32 skins.
 * Made from the textures of the running game (and its resource packs), once a world is open.
 */
final class ZombieSkins {
    private static boolean made;

    private ZombieSkins() {}

    static void ensure(Minecraft mc) {
        if (made || mc.level == null) return;
        made = true;
        make(mc, "minecraft:textures/entity/zombie/zombie.png", Skins.ZOMBIE);
        make(mc, "minecraft:textures/entity/zombie/husk.png", Skins.HUSK);
    }

    private static void make(Minecraft mc, String source, String asset) {
        NativeImage img;
        try (InputStream in = mc.getResourceManager().open(Identifier.parse(source))) {
            img = NativeImage.read(in);
        } catch (Exception e) {
            SoworkerMod.LOG.warn("soworker: cannot read {}: {}", source, e.toString());
            return;
        }
        if (img.getWidth() != 64 || img.getHeight() != 64) {
            SoworkerMod.LOG.warn("soworker: {} is {}x{}, not a 64x64 skin", source, img.getWidth(), img.getHeight());
            img.close();
            return;
        }
        // right leg -> left leg (top, bottom, then the four sides), right arm -> left arm (same order)
        img.copyRect(4, 16, 16, 32, 4, 4, true, false);
        img.copyRect(8, 16, 16, 32, 4, 4, true, false);
        img.copyRect(0, 20, 24, 32, 4, 12, true, false);
        img.copyRect(4, 20, 16, 32, 4, 12, true, false);
        img.copyRect(8, 20, 8, 32, 4, 12, true, false);
        img.copyRect(12, 20, 16, 32, 4, 12, true, false);
        img.copyRect(44, 16, -8, 32, 4, 4, true, false);
        img.copyRect(48, 16, -8, 32, 4, 4, true, false);
        img.copyRect(40, 20, 0, 32, 4, 12, true, false);
        img.copyRect(44, 20, -8, 32, 4, 12, true, false);
        img.copyRect(48, 20, -16, 32, 4, 12, true, false);
        img.copyRect(52, 20, -8, 32, 4, 12, true, false);
        Identifier path = new ClientAsset.ResourceTexture(Identifier.parse(asset)).texturePath();
        mc.getTextureManager().register(path, new DynamicTexture(() -> "soworker " + asset, img));
    }
}
