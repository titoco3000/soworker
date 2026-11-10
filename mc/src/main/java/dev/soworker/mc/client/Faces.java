package dev.soworker.mc.client;

import com.mojang.blaze3d.platform.NativeImage;
import dev.soworker.mc.AwayPayload;
import dev.soworker.mc.FacePayload;
import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;
import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents;
import net.fabricmc.fabric.api.client.networking.v1.ClientPlayNetworking;
import net.fabricmc.fabric.api.client.rendering.v1.LivingEntityRenderLayerRegistrationCallback;
import net.minecraft.client.Minecraft;
import net.minecraft.client.renderer.entity.player.AvatarRenderer;
import net.minecraft.client.renderer.entity.state.AvatarRenderState;
import net.minecraft.client.renderer.texture.DynamicTexture;
import net.minecraft.resources.Identifier;

/** Client side of the camera faces: one texture per entity with a live camera, drawn by {@link FaceLayer}. */
public final class Faces implements ClientModInitializer {
    /** A face with no new frame for this long is dropped (camera off without a "gone", entity remade, ...). */
    static final long STALE_MS = 3000;

    static final class Face {
        DynamicTexture tex;
        Identifier id;
        int w, h;
        long at;
    }

    static final Map<Integer, Face> FACES = new HashMap<>();
    /** We are away in SoWork: KeyboardInputMixin holds sneak. */
    public static volatile boolean away;

    /** The face texture for an entity, or null when it has no live camera. */
    public static Identifier texture(int entity) {
        Face f = FACES.get(entity);
        return f == null ? null : f.id;
    }

    public static boolean hasFace(int entity) {
        return FACES.containsKey(entity);
    }

    @Override
    @SuppressWarnings({"unchecked", "rawtypes"})
    public void onInitializeClient() {
        ClientPlayNetworking.registerGlobalReceiver(FacePayload.TYPE, (p, ctx) -> ctx.client().execute(() -> frame(p)));
        ClientPlayNetworking.registerGlobalReceiver(AwayPayload.TYPE, (p, ctx) -> away = p.away());
        LivingEntityRenderLayerRegistrationCallback.EVENT.register((type, renderer, helper, ctx) -> {
            if (renderer instanceof AvatarRenderer<?> r) helper.register(new FaceLayer((AvatarRenderer) r));
        });
        ClientTickEvents.END_CLIENT_TICK.register(mc -> {
            if (mc.level == null) away = false;
            long now = System.currentTimeMillis();
            for (Iterator<Face> it = FACES.values().iterator(); it.hasNext(); ) {
                Face f = it.next();
                if (now - f.at > STALE_MS || mc.level == null) {
                    release(f);
                    it.remove();
                }
            }
        });
    }

    private static void release(Face f) {
        Minecraft.getInstance().getTextureManager().release(f.id);
    }

    private static void frame(FacePayload p) {
        Face f = FACES.get(p.entity());
        if (p.w() <= 0 || p.h() <= 0 || p.rgb().length < p.w() * p.h() * 3) {
            if (f != null) {
                release(f);
                FACES.remove(p.entity());
            }
            return;
        }
        if (f == null || f.w != p.w() || f.h != p.h()) {
            if (f != null) release(f);
            f = new Face();
            f.w = p.w();
            f.h = p.h();
            f.id = Identifier.fromNamespaceAndPath("soworker", "face/" + p.entity());
            f.tex = new DynamicTexture(() -> "soworker face", p.w(), p.h(), false);
            Minecraft.getInstance().getTextureManager().register(f.id, f.tex);
            FACES.put(p.entity(), f);
        }
        NativeImage img = f.tex.getPixels();
        if (img == null) return;
        byte[] rgb = p.rgb();
        for (int y = 0, i = 0; y < p.h(); y++) {
            for (int x = 0; x < p.w(); x++, i += 3) {
                img.setPixel(x, y, 0xFF000000 | (rgb[i] & 0xFF) << 16 | (rgb[i + 1] & 0xFF) << 8 | (rgb[i + 2] & 0xFF));
            }
        }
        f.tex.upload();
        f.at = System.currentTimeMillis();
    }

    /** Turn the head (and the body, past 70°) of someone with a live camera toward the viewer. Not our own player. */
    public static void faceViewer(int entity, AvatarRenderState s) {
        Minecraft mc = Minecraft.getInstance();
        if (!hasFace(entity) || mc.player == null || mc.player.getId() == entity) return;
        var cam = mc.gameRenderer.mainCamera().position();
        double dx = cam.x - s.x, dz = cam.z - s.z, dy = cam.y - (s.y + s.eyeHeight);
        float yaw = (float) Math.toDegrees(Math.atan2(-dx, dz));
        float rel = wrap(yaw - s.bodyRot);
        if (rel > 70) {
            s.bodyRot = yaw - 70;
            rel = 70;
        } else if (rel < -70) {
            s.bodyRot = yaw + 70;
            rel = -70;
        }
        s.yRot = rel;
        s.xRot = (float) -Math.toDegrees(Math.atan2(dy, Math.hypot(dx, dz)));
    }

    static float wrap(float a) {
        a %= 360;
        if (a >= 180) a -= 360;
        if (a < -180) a += 360;
        return a;
    }
}
