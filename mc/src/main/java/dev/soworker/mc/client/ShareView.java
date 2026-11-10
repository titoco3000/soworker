package dev.soworker.mc.client;

import com.mojang.blaze3d.platform.NativeImage;
import dev.soworker.mc.UnwatchPayload;
import dev.soworker.mc.WatchPayload;
import java.awt.image.BufferedImage;
import java.io.ByteArrayInputStream;
import javax.imageio.ImageIO;
import net.fabricmc.fabric.api.client.networking.v1.ClientPlayNetworking;
import net.minecraft.client.KeyMapping;
import net.minecraft.client.Minecraft;
import net.minecraft.client.gui.GuiGraphicsExtractor;
import net.minecraft.client.gui.screens.Screen;
import net.minecraft.client.input.KeyEvent;
import net.minecraft.client.renderer.texture.DynamicTexture;
import net.minecraft.network.chat.Component;
import net.minecraft.resources.Identifier;

/**
 * A screen share over the whole window, at the share's own resolution (JPEG frames from soworker). Opened by a
 * snowball on the share's wall; a movement key, jump, sneak or Esc closes it and tells soworker to stop.
 */
public final class ShareView extends Screen {
    static final Identifier TEX = Identifier.fromNamespaceAndPath("soworker", "share_full");
    private static ShareView open;
    private final String who;
    private DynamicTexture tex;
    private int w, h;

    private ShareView(String who) {
        super(Component.literal("Screen share"));
        this.who = who;
    }

    public static void register() {
        ClientPlayNetworking.registerGlobalReceiver(WatchPayload.TYPE, (p, ctx) -> {
            switch (p.kind()) {
                case WatchPayload.OPEN -> ctx.client().execute(() -> {
                    open = new ShareView(p.name());
                    ctx.client().setScreenAndShow(open);
                });
                case WatchPayload.FRAME -> {
                    // decode here, off the render thread; only the upload happens there
                    BufferedImage img;
                    try {
                        img = ImageIO.read(new ByteArrayInputStream(p.jpeg()));
                    } catch (Exception e) {
                        return;
                    }
                    if (img == null) return;
                    ctx.client().execute(() -> {
                        if (open != null && ctx.client().gui.screen() == open) open.show(img);
                    });
                }
                case WatchPayload.CLOSE -> ctx.client().execute(() -> {
                    if (open != null && ctx.client().gui.screen() == open) open.onClose();
                });
                default -> {}
            }
        });
    }

    private void show(BufferedImage img) {
        int iw = img.getWidth(), ih = img.getHeight();
        if (tex == null || iw != w || ih != h) {
            if (tex != null) Minecraft.getInstance().getTextureManager().release(TEX);
            w = iw;
            h = ih;
            tex = new DynamicTexture(() -> "soworker share", w, h, false);
            Minecraft.getInstance().getTextureManager().register(TEX, tex);
        }
        NativeImage px = tex.getPixels();
        if (px == null) return;
        int[] row = new int[w];
        for (int y = 0; y < h; y++) {
            img.getRGB(0, y, w, 1, row, 0, w);
            for (int x = 0; x < w; x++) px.setPixel(x, y, 0xFF000000 | row[x]);
        }
        tex.upload();
    }

    @Override
    public boolean isPauseScreen() {
        return false;
    }

    @Override
    public void extractBackground(GuiGraphicsExtractor g, int mx, int my, float pt) {
        // no blur: the picture covers everything
    }

    @Override
    public void extractRenderState(GuiGraphicsExtractor g, int mx, int my, float pt) {
        g.fill(0, 0, width, height, 0xFF000000);
        if (tex == null) {
            g.centeredText(font, "Opening " + who + "'s screen…", width / 2, height / 2 - 4, 0xFFFFFFFF);
            return;
        }
        // fit the window, keeping the share's shape (black bars)
        float s = Math.min((float) width / w, (float) height / h);
        int dw = Math.round(w * s), dh = Math.round(h * s);
        int x = (width - dw) / 2, y = (height - dh) / 2;
        g.blit(TEX, x, y, x + dw, y + dh, 0f, 1f, 0f, 1f);
    }

    @Override
    public boolean keyPressed(KeyEvent e) {
        var o = Minecraft.getInstance().options;
        for (KeyMapping k : new KeyMapping[] {o.keyUp, o.keyDown, o.keyLeft, o.keyRight, o.keyJump, o.keyShift}) {
            if (k.matches(e)) {
                onClose();
                return true;
            }
        }
        return super.keyPressed(e); // Esc closes too
    }

    @Override
    public void removed() {
        if (open == this) {
            open = null;
            if (ClientPlayNetworking.canSend(UnwatchPayload.TYPE)) ClientPlayNetworking.send(new UnwatchPayload());
        }
        if (tex != null) Minecraft.getInstance().getTextureManager().release(TEX);
        tex = null;
        super.removed();
    }
}
