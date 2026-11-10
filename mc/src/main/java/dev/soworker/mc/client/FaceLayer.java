package dev.soworker.mc.client;

import com.mojang.blaze3d.vertex.PoseStack;
import net.minecraft.client.model.player.PlayerModel;
import net.minecraft.client.renderer.SubmitNodeCollector;
import net.minecraft.client.renderer.entity.RenderLayerParent;
import net.minecraft.client.renderer.entity.layers.RenderLayer;
import net.minecraft.client.renderer.entity.state.AvatarRenderState;
import net.minecraft.client.renderer.rendertype.RenderTypes;
import net.minecraft.client.renderer.texture.OverlayTexture;
import net.minecraft.resources.Identifier;

/**
 * The camera picture over the front of the head. The head cube spans x -4..4, y -8..0, z -4..4 (pixels of 1/16
 * block) with the face at z = -4; the hat layer stands 0.5 out, so the picture goes just in front of that. Model x is
 * the viewer's left to right and model y grows downwards, so the image's top-left corner is (-4, -8).
 */
public final class FaceLayer extends RenderLayer<AvatarRenderState, PlayerModel> {
    static final float Z = -4.6f / 16, S = 4f / 16;

    public FaceLayer(RenderLayerParent<AvatarRenderState, PlayerModel> parent) {
        super(parent);
    }

    @Override
    public void submit(PoseStack pose, SubmitNodeCollector c, int light, AvatarRenderState s, float yRot, float xRot) {
        if (s.isInvisible) return;
        Identifier tex = Faces.texture(s.id);
        if (tex == null) return;
        pose.pushPose();
        getParentModel().getHead().translateAndRotate(pose);
        c.submitCustomGeometry(pose, RenderTypes.entityCutout(tex), (p, v) -> {
            vertex(v, p, -S, -2 * S, 0, 0, light);
            vertex(v, p, -S, 0, 0, 1, light);
            vertex(v, p, S, 0, 1, 1, light);
            vertex(v, p, S, -2 * S, 1, 0, light);
        });
        pose.popPose();
    }

    private static void vertex(com.mojang.blaze3d.vertex.VertexConsumer v, PoseStack.Pose p, float x, float y, float u, float w, int light) {
        v.addVertex(p, x, y, Z).setColor(-1).setUv(u, w).setOverlay(OverlayTexture.NO_OVERLAY).setLight(light).setNormal(p, 0, 0, -1);
    }
}
