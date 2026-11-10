package dev.soworker.mc;

import com.mojang.serialization.Codec;
import java.util.ArrayList;
import java.util.List;
import net.fabricmc.fabric.api.attachment.v1.AttachmentRegistry;
import net.fabricmc.fabric.api.attachment.v1.AttachmentTarget;
import net.fabricmc.fabric.api.attachment.v1.AttachmentType;
import net.minecraft.ChatFormatting;
import net.minecraft.core.component.DataComponents;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.network.chat.Component;
import net.minecraft.resources.Identifier;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.entity.player.Inventory;
import net.minecraft.world.item.Item;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.item.component.CustomData;
import net.minecraft.world.item.component.ItemLore;
import net.minecraft.world.phys.AABB;

/**
 * The hotbar tools. Slots 1–4 (inventory 0–3) are the meeting tools while we are in a call; slots 5–9 (4–8) are
 * always an ender pearl (SoWork's teleport where it lands), a spyglass, snowballs (thrown at someone: a wave), "walk to your desk" and "availability". The spyglass and the
 * snowballs work as in vanilla ({@link #plain}); there is always exactly one snowball, given back as soon as it is thrown but locked for 1 s (as the ender pearl). Tools only reflect soworker's state: using one sends a command and the
 * next state message changes the item. The player's own items in slots 1–4 are kept in a persistent attachment
 * during a meeting and given back after it; items found in slots 5–9 are moved elsewhere.
 */
public final class Tools {
    public static final String KEY = "soworker_tool";
    /** Kept in the hotbar like tools, but used the vanilla way rather than sending a command. */
    public static boolean plain(String tool) {
        return "spyglass".equals(tool) || "snowball".equals(tool) || "pearl".equals(tool);
    }
    static final AttachmentType<List<ItemStack>> SAVED =
            AttachmentRegistry.createPersistent(Identifier.fromNamespaceAndPath("soworker", "saved_hotbar"), ItemStack.OPTIONAL_CODEC.listOf());

    /** What the tools show; filled from the `call` and `me` messages. */
    public boolean inCall, mic, cam, share, rec, available = true;
    public String recClock;

    public static void init() {
        // touching the class registers the attachment type at start-up
    }

    public static String toolOf(ItemStack s) {
        if (s == null || s.isEmpty()) return null;
        CustomData d = s.get(DataComponents.CUSTOM_DATA);
        if (d == null) return null;
        CompoundTag t = d.copyTag();
        return t.contains(KEY) ? t.getStringOr(KEY, null) : null;
    }

    static ItemStack make(String tool, Item item, String name, boolean glint, String lore) {
        ItemStack s = new ItemStack(item);
        s.set(DataComponents.CUSTOM_NAME, Component.literal(name).withStyle(st -> st.withItalic(false)));
        s.set(DataComponents.LORE, new ItemLore(List.of(Component.literal(lore).withStyle(ChatFormatting.GRAY))));
        if (glint) s.set(DataComponents.ENCHANTMENT_GLINT_OVERRIDE, true);
        CompoundTag t = new CompoundTag();
        t.putString(KEY, tool);
        s.set(DataComponents.CUSTOM_DATA, CustomData.of(t));
        return s;
    }

    /** The wanted tool per hotbar slot (null = the player's own). */
    ItemStack[] wanted() {
        ItemStack[] w = new ItemStack[9];
        if (inCall) {
            w[0] = mic ? make("mic", DyeItems.LIME, "Microphone: on (use to mute)", false, "Right click: mute")
                       : make("mic", DyeItems.GRAY, "Microphone: muted (use to unmute)", false, "Right click: unmute");
            w[1] = share ? make("share", Items.SPYGLASS, "Stop sharing", true, "Right click: stop sharing your screen")
                         : make("share", Items.SPYGLASS, "Share screen", false, "Right click: share your screen");
            w[2] = rec ? make("rec", DyeItems.RED, "Stop recording" + (recClock != null ? " (" + recClock + ")" : ""), true, "Right click: stop recording")
                       : make("rec", DyeItems.RED, "Record meeting", false, "Right click: start recording");
            w[3] = cam ? make("cam", Items.ENDER_EYE, "Camera: on", true, "Right click: camera off")
                       : make("cam", Items.ENDER_PEARL, "Camera: off", false, "Right click: camera on");
        }
        w[4] = pearl();
        w[5] = make("spyglass", Items.SPYGLASS, "Spyglass", false, "Right click: look far");
        w[6] = snowball();
        w[7] = make("desk", Items.COMPASS, "Walk to your desk", false, "Right click: walk to your desk");
        w[8] = available ? make("away", Items.EMERALD, "Available (use to go away)", false, "Right click: set yourself away")
                         : make("away", Items.REDSTONE, "Away (use to become available)", false, "Right click: become available");
        return w;
    }

    static ItemStack pearl() {
        return make("pearl", Items.ENDER_PEARL, "Ender pearl", false, "Throw: teleport where it lands");
    }

    static ItemStack snowball() {
        return make("snowball", Items.SNOWBALL, "Snowball", false, "Throw at someone to wave at them");
    }

    /** Ticks before the snowball can be thrown again (shown as the vanilla cooldown sweep over it). */
    static final int RELOAD = 20; // 1 s, the same as vanilla's ender pearl cooldown
    /** The player threw the snowball (set from the use handler, before vanilla takes it). */
    public static boolean thrown;

    /**
     * One snowball, given back the tick after it is thrown (assertSlots would take up to a second) but locked for
     * {@link #RELOAD} ticks.
     */
    public static void refillThrowables(ServerPlayer p) {
        Inventory inv = p.getInventory();
        ItemStack s = inv.getItem(6);
        if (!"snowball".equals(toolOf(s)) || s.getCount() != 1) {
            if (!s.isEmpty() && toolOf(s) == null) stash(p, s);
            inv.setItem(6, snowball());
        }
        // the pearl comes back at once too; vanilla's own one-second cooldown covers it
        ItemStack e = inv.getItem(4);
        if (!"pearl".equals(toolOf(e)) || e.getCount() != 1) {
            if (!e.isEmpty() && toolOf(e) == null) stash(p, e);
            inv.setItem(4, pearl());
        }
        if (thrown) {
            thrown = false;
            p.getCooldowns().addCooldown(inv.getItem(6), RELOAD);
        }
    }

    @SuppressWarnings("unchecked")
    static List<ItemStack> saved(ServerPlayer p) {
        return ((AttachmentTarget) (Object) p).getAttached(SAVED);
    }

    static void setSaved(ServerPlayer p, List<ItemStack> l) {
        if (l == null) ((AttachmentTarget) (Object) p).removeAttached(SAVED);
        else ((AttachmentTarget) (Object) p).setAttached(SAVED, l);
    }

    /** Put a stack somewhere other than the tool slots, or at the player's feet. */
    static void stash(ServerPlayer p, ItemStack s) {
        Inventory inv = p.getInventory();
        for (int i = 9; i < 36 && !s.isEmpty(); i++) {
            if (inv.getItem(i).isEmpty()) {
                inv.setItem(i, s);
                return;
            }
        }
        p.drop(s, false, null);
        p.sendSystemMessage(Component.literal("[SoWork] Your inventory is full: dropped " + s.getHoverName().getString() + " at your feet").withStyle(ChatFormatting.YELLOW));
    }

    /** Make the inventory match the state: called every second and whenever the state changes. */
    public void assertSlots(ServerPlayer p) {
        Inventory inv = p.getInventory();
        ItemStack[] w = wanted();
        // meeting started: keep the player's slots 1–4; ended: give them back
        List<ItemStack> keep = saved(p);
        if (inCall && keep == null) {
            List<ItemStack> l = new ArrayList<>();
            for (int i = 0; i < 4; i++) {
                ItemStack s = inv.getItem(i);
                l.add(toolOf(s) != null ? ItemStack.EMPTY : s.copy());
                inv.setItem(i, ItemStack.EMPTY);
            }
            setSaved(p, l);
        } else if (!inCall && keep != null) {
            for (int i = 0; i < 4; i++) {
                if (toolOf(inv.getItem(i)) != null) inv.setItem(i, ItemStack.EMPTY);
            }
            for (int i = 0; i < keep.size() && i < 4; i++) {
                ItemStack s = keep.get(i);
                if (s.isEmpty()) continue;
                if (inv.getItem(i).isEmpty()) inv.setItem(i, s.copy());
                else stash(p, s.copy());
            }
            setSaved(p, null);
        }
        for (int i = 0; i < inv.getContainerSize(); i++) {
            ItemStack s = inv.getItem(i);
            boolean tool = toolOf(s) != null;
            if (i < 9 && w[i] != null) {
                if (!tool && !s.isEmpty()) stash(p, s); // the player's own item in a reserved slot
                if (!ItemStack.matches(s, w[i])) inv.setItem(i, w[i]);
            } else if (tool) {
                inv.setItem(i, ItemStack.EMPTY); // a tool where it does not belong
            }
        }
        if (toolOf(p.containerMenu.getCarried()) != null) {
            p.containerMenu.setCarried(ItemStack.EMPTY);
            p.containerMenu.broadcastChanges();
        }
    }

    /** Tools thrown on the ground vanish (the slot is filled again by assertSlots). */
    public static void sweepDropped(ServerPlayer p) {
        AABB box = p.getBoundingBox().inflate(12);
        for (ItemEntity e : p.level().getEntities(EntityTypes.ITEM, box, e -> toolOf(e.getItem()) != null)) {
            e.discard();
        }
    }

    /** Remove every tool (the bridge is off). */
    public static void removeAll(ServerPlayer p) {
        Inventory inv = p.getInventory();
        for (int i = 0; i < inv.getContainerSize(); i++) {
            if (toolOf(inv.getItem(i)) != null) inv.setItem(i, ItemStack.EMPTY);
        }
    }
}
