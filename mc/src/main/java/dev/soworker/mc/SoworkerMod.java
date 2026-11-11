package dev.soworker.mc;

import com.mojang.brigadier.CommandDispatcher;
import com.mojang.brigadier.arguments.StringArgumentType;
import com.mojang.brigadier.builder.LiteralArgumentBuilder;
import net.fabricmc.api.ModInitializer;
import net.fabricmc.fabric.api.command.v2.CommandRegistrationCallback;
import net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents;
import net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents;
import net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents;
import net.fabricmc.fabric.api.event.player.AttackBlockCallback;
import net.fabricmc.fabric.api.networking.v1.ServerPlayConnectionEvents;
import net.fabricmc.fabric.api.message.v1.ServerMessageEvents;
import net.fabricmc.fabric.api.networking.v1.PayloadTypeRegistry;
import net.fabricmc.fabric.api.event.player.UseBlockCallback;
import net.fabricmc.fabric.api.event.player.UseEntityCallback;
import net.fabricmc.fabric.api.event.player.UseItemCallback;
import net.minecraft.ChatFormatting;
import net.minecraft.commands.CommandSourceStack;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.InteractionResult;
import net.minecraft.world.entity.npc.villager.Villager;
import net.minecraft.world.entity.player.Player;
import net.minecraft.world.item.ItemStack;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/** Entry point: the office (one per running server), the `/sw` commands and the hotbar tool clicks. */
public final class SoworkerMod implements ModInitializer {
    public static final Logger LOG = LoggerFactory.getLogger("soworker");
    static Office office;
    private static long lastUse;

    @Override
    public void onInitialize() {
        Tools.init();
        PayloadTypeRegistry.clientboundPlay().register(FacePayload.TYPE, FacePayload.CODEC);
        PayloadTypeRegistry.clientboundPlay().register(AwayPayload.TYPE, AwayPayload.CODEC);
        PayloadTypeRegistry.clientboundPlay().registerLarge(WatchPayload.TYPE, WatchPayload.CODEC, WatchPayload.MAX + 1024);
        PayloadTypeRegistry.serverboundPlay().register(UnwatchPayload.TYPE, UnwatchPayload.CODEC);
        net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking.registerGlobalReceiver(UnwatchPayload.TYPE, (p, ctx) -> {
            if (office != null && office.bridge.connected()) office.cmd("unwatch", null);
        });
        ServerLifecycleEvents.SERVER_STARTED.register(s -> office = new Office(s));
        ServerLifecycleEvents.SERVER_STOPPING.register(s -> {
            if (office != null) {
                office.save();
                office.stopChild(true);
            }
            office = null;
        });
        ServerTickEvents.END_SERVER_TICK.register(s -> {
            if (office != null && office.server == s) office.tick();
        });
        // villagers, screens and boards saved by an earlier run are made again from soworker's state
        ServerEntityEvents.ENTITY_LOAD.register((e, level) -> {
            if (Villagers.leftover(e)) e.discard();
        });
        ServerPlayConnectionEvents.JOIN.register((h, sender, s) -> {
            if (office != null && office.enabled) {
                office.prepareWorld();
                office.resync(); // puts the player where the avatar is
            }
        });
        // plain chat (not commands) also goes to the SoWork room chat
        ServerMessageEvents.ALLOW_CHAT_MESSAGE.register((msg, sender, type) -> {
            if (office != null && office.enabled && office.bridge.connected()) office.cmd("chat", msg.signedContent());
            return true;
        });
        CommandRegistrationCallback.EVENT.register((d, ctx, sel) -> register(d));
        UseItemCallback.EVENT.register((p, level, hand) -> {
            // only a right click throws (a left click on a block also comes through use(), below)
            ItemStack s = p.getItemInHand(hand);
            if (p instanceof ServerPlayer && "snowball".equals(Tools.toolOf(s)) && !p.getCooldowns().isOnCooldown(s)) Tools.thrown = true;
            return use(p, hand);
        });
        UseBlockCallback.EVENT.register((p, level, hand, hit) -> use(p, hand));
        AttackBlockCallback.EVENT.register((p, level, hand, pos, dir) -> use(p, hand));
        // no trading with coworkers
        UseEntityCallback.EVENT.register((p, level, hand, e, hit) ->
                (e instanceof Villager || e instanceof net.minecraft.world.entity.decoration.Mannequin) && e.entityTags().contains(Villagers.TAG) ? InteractionResult.FAIL : InteractionResult.PASS);
    }

    /** A click with a tool in hand runs it (once per click; the item never acts itself). */
    private static InteractionResult use(Player p, InteractionHand hand) {
        if (!(p instanceof ServerPlayer sp) || office == null) {
            // client side: SUCCESS sends the click to the server (FAIL would drop it) without the item doing anything
            String t = Tools.toolOf(p.getItemInHand(hand));
            return t != null && !Tools.plain(t) ? InteractionResult.SUCCESS : InteractionResult.PASS;
        }
        ItemStack s = sp.getItemInHand(hand);
        String tool = Tools.toolOf(s);
        if (tool == null || Tools.plain(tool)) return InteractionResult.PASS;
        long now = office.tick;
        if (now - lastUse >= 5) {
            lastUse = now;
            office.cmd(tool, null);
        }
        return InteractionResult.FAIL;
    }

    /** The player's ender pearl landed: SoWork's teleport there. False when the office is not on (vanilla pearl). */
    public static boolean pearl(ServerPlayer p, net.minecraft.world.phys.Vec3 at) {
        if (office == null || !office.enabled || !office.bridge.connected()) return false;
        office.cmd("pearl", String.format(java.util.Locale.ROOT, "%.2f %.2f", at.x, at.z));
        return true;
    }

    /** The screen share whose wall a projectile hit, or null. */
    public static Screens.Screen screenHit(net.minecraft.world.phys.HitResult h) {
        if (office == null || !office.enabled) return null;
        if (h instanceof net.minecraft.world.phys.EntityHitResult e) return office.screens.screenOf(e.getEntity());
        if (h instanceof net.minecraft.world.phys.BlockHitResult b) return office.screens.screenAt(b.getBlockPos());
        return null;
    }

    /** A snowball from the player hit a screen share's wall: show it full screen. */
    public static void snowballWatch(ServerPlayer p, Screens.Screen s) {
        if (office != null && office.enabled) office.watch(p, s);
    }

    /** A snowball from the player hit a coworker's villager: wave at them (soworker limits how often). */
    public static void snowballWave(ServerPlayer p, net.minecraft.world.entity.Entity v) {
        if (office == null || !office.enabled) return;
        String id = office.villagers.idOf(v);
        if (id == null) return;
        office.villagers.wave((net.minecraft.server.level.ServerLevel) v.level(), id);
        office.cmd("wave_id", id);
    }

    private static int run(CommandSourceStack src, String name, String arg) {
        if (office == null) return 0;
        office.cmd(name, arg);
        return 1;
    }

    private static void register(CommandDispatcher<CommandSourceStack> d) {
        LiteralArgumentBuilder<CommandSourceStack> sw = Commands.literal("sw");
        for (String c : new String[] {"mic", "share", "rec", "cam", "desk", "away", "hand", "leave"}) {
            sw.then(Commands.literal(c).executes(ctx -> run(ctx.getSource(), c, null)));
        }
        sw.then(Commands.literal("wave").then(Commands.argument("name", StringArgumentType.greedyString())
                .executes(ctx -> run(ctx.getSource(), "wave", StringArgumentType.getString(ctx, "name")))));
        sw.then(Commands.literal("say").then(Commands.argument("text", StringArgumentType.greedyString())
                .executes(ctx -> run(ctx.getSource(), "say", StringArgumentType.getString(ctx, "text")))));
        sw.then(Commands.literal("status").executes(ctx -> {
            if (office == null) return 0;
            ctx.getSource().sendSuccess(() -> Component.literal("[SoWork] " + office.status()).withStyle(ChatFormatting.AQUA), false);
            if (office.bridge.connected()) office.cmd("status", null);
            return 1;
        }));
        // sign in again (after a failure, or to take SoWork back after it was opened elsewhere), or as someone else
        sw.then(Commands.literal("login").executes(ctx -> run(ctx.getSource(), "login", null)));
        sw.then(Commands.literal("logout").executes(ctx -> run(ctx.getSource(), "logout", null)));
        sw.then(Commands.literal("enable").executes(ctx -> {
            if (office == null) return 0;
            office.enable();
            return 1;
        }));
        sw.then(Commands.literal("disable").executes(ctx -> {
            if (office == null) return 0;
            office.disable();
            return 1;
        }));
        d.register(sw);
    }
}
