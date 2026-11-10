package dev.soworker.mc;

import java.io.File;
import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents;
import net.minecraft.client.Minecraft;
import net.minecraft.client.Screenshot;

/**
 * Development aid only, off unless the game runs with {@code -Dsoworker.shotdir=DIR}: a file {@code DIR/NAME.req}
 * makes the client save a screenshot as {@code DIR/screenshots/NAME.png} (used by the test scripts, which cannot press
 * F2). Nothing else on the client side.
 */
public final class DevShots implements ClientModInitializer {
    @Override
    public void onInitializeClient() {
        String dir = System.getProperty("soworker.shotdir");
        if (dir == null) return;
        File d = new File(dir);
        ClientTickEvents.END_CLIENT_TICK.register(mc -> {
            File[] reqs = d.listFiles((f, n) -> n.endsWith(".req"));
            if (reqs == null) return;
            for (File r : reqs) {
                String name = r.getName().replace(".req", ".png");
                r.delete();
                Screenshot.grab(d, name, mc.gameRenderer.mainRenderTarget(), 1, m -> {});
            }
        });
    }
}
