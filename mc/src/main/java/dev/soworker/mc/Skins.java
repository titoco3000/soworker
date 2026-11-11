package dev.soworker.mc;

import com.google.gson.JsonObject;
import com.mojang.serialization.JsonOps;
import net.minecraft.world.entity.decoration.Mannequin;
import net.minecraft.world.item.component.ResolvableProfile;

/**
 * The skin a coworker's mannequin wears: one of the game's own humanoid skins, picked from the user id so the same
 * person always looks the same. Only textures in the player skin layout fit the mannequin (the player model): the
 * nine default skins, drowned, and zombie and husk once their missing left limbs are filled in (client.ZombieSkins).
 * Skeletons (64x32, two-pixel limbs) and piglins (wider head) do not fit.
 */
public final class Skins {
    record Skin(String texture, String model) {}

    static final String[] DEFAULTS = {"steve", "alex", "ari", "efe", "kai", "makena", "noor", "sunny", "zuri"};
    /** Zombie and husk as player skins, made on the client (see client.ZombieSkins). */
    public static final String ZOMBIE = "soworker:entity/skin/zombie", HUSK = "soworker:entity/skin/husk";
    static final Skin[] MOBS = {new Skin(ZOMBIE, "wide"), new Skin(HUSK, "wide"), new Skin("minecraft:entity/zombie/drowned", "wide")};

    private Skins() {}

    /** The skin for a user: each default skin and each mob equally likely; a default skin's arms wide or slim. */
    static Skin of(String userId) {
        long h = fnv(userId);
        int n = DEFAULTS.length + MOBS.length;
        int i = (int) Long.remainderUnsigned(h, n);
        if (i >= DEFAULTS.length) return MOBS[i - DEFAULTS.length];
        String model = ((h >>> 32) & 1) == 0 ? "wide" : "slim";
        return new Skin("minecraft:entity/player/" + model + "/" + DEFAULTS[i], model);
    }

    /** The mannequin profile wearing that skin (the default profile if the game refuses it). */
    static ResolvableProfile profile(String userId) {
        Skin s = of(userId);
        JsonObject o = new JsonObject();
        o.addProperty("texture", s.texture());
        o.addProperty("model", s.model());
        return ResolvableProfile.CODEC.parse(JsonOps.INSTANCE, o).resultOrPartial(e -> SoworkerMod.LOG.warn("soworker: skin {}: {}", s, e)).orElse(Mannequin.DEFAULT_PROFILE);
    }

    private static long fnv(String s) {
        long h = 0xcbf29ce484222325L;
        for (int i = 0; i < s.length(); i++) {
            h ^= s.charAt(i);
            h *= 0x100000001b3L;
        }
        return h;
    }
}
