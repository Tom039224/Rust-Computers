package com.rustcomputers.command;

import com.mojang.brigadier.CommandDispatcher;
import com.mojang.brigadier.arguments.BoolArgumentType;
import com.mojang.brigadier.arguments.IntegerArgumentType;
import net.minecraft.commands.CommandSourceStack;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerPlayer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;
import java.util.UUID;

/**
 * /rc コマンド群の登録と、コンピューターログストリーミング管理。
 * Registers the /rc command tree and manages per-player computer log streaming.
 *
 * <h3>コマンド一覧</h3>
 * <pre>
 *   /rc log &lt;computerId&gt;             -- ストリーミング状態の確認
 *   /rc log &lt;computerId&gt; true|false  -- ストリーミングの有効/無効切り替え
 * </pre>
 *
 * <p>設定はプレイヤーのセッション中のみ有効（ログアウトでリセット）。</p>
 * <p>Settings persist only for the current session (reset on logout).</p>
 */
public final class RustComputersCommand {

    private static final Logger LOGGER = LoggerFactory.getLogger(RustComputersCommand.class);

    /**
     * プレイヤーごとのログストリーミング有効コンピューター ID セット。
     * Per-player set of computer IDs with log streaming enabled.
     * キーはプレイヤー UUID、値はストリーミング中のコンピューター ID セット。
     */
    private static final Map<UUID, Set<Integer>> STREAMS = Collections.synchronizedMap(new HashMap<>());

    private RustComputersCommand() {}

    // ------------------------------------------------------------------
    // コマンド登録 / Command registration
    // ------------------------------------------------------------------

    /**
     * Brigadier ディスパッチャーに /rc コマンドツリーを登録する。
     * Register the /rc command tree with the Brigadier dispatcher.
     *
     * @param dispatcher コマンドディスパッチャー / command dispatcher
     */
    public static void register(CommandDispatcher<CommandSourceStack> dispatcher) {
        dispatcher.register(
            Commands.literal("rc")
                .then(Commands.literal("log")
                    .then(Commands.argument("computerId", IntegerArgumentType.integer(0))
                        // /rc log <id>  → ステータス確認 / Check streaming status
                        .executes(ctx -> executeStatus(ctx.getSource(),
                                IntegerArgumentType.getInteger(ctx, "computerId")))
                        // /rc log <id> <true|false>  → ON/OFF 切り替え / Toggle streaming
                        .then(Commands.argument("enabled", BoolArgumentType.bool())
                            .executes(ctx -> executeSet(ctx.getSource(),
                                    IntegerArgumentType.getInteger(ctx, "computerId"),
                                    BoolArgumentType.getBool(ctx, "enabled")))
                        )
                    )
                )
        );
    }

    // ------------------------------------------------------------------
    // コマンド実行 / Command execution
    // ------------------------------------------------------------------

    /**
     * /rc log &lt;computerId&gt; — ストリーミング状態を表示。
     */
    private static int executeStatus(CommandSourceStack source, int computerId) {
        try {
            ServerPlayer player = source.getPlayerOrException();
            UUID uuid = player.getUUID();
            boolean enabled = STREAMS.getOrDefault(uuid, Set.of()).contains(computerId);
            String status = enabled ? "§aON§r" : "§7OFF§r";
            source.sendSuccess(
                    () -> Component.literal("[RC] Computer #" + computerId + " log stream: " + status),
                    false);
        } catch (com.mojang.brigadier.exceptions.CommandSyntaxException e) {
            // コンソールからの実行 / Console execution
            source.sendSuccess(
                    () -> Component.literal("[RC] /rc log コマンドはプレイヤーのみ使用できます"),
                    false);
        }
        return 1;
    }

    /**
     * /rc log &lt;computerId&gt; &lt;true|false&gt; — ストリーミングを ON/OFF に設定。
     */
    private static int executeSet(CommandSourceStack source, int computerId, boolean enable) {
        try {
            ServerPlayer player = source.getPlayerOrException();
            UUID uuid = player.getUUID();
            if (enable) {
                STREAMS.computeIfAbsent(uuid, k -> new HashSet<>()).add(computerId);
                LOGGER.info("Player {} enabled log stream for Computer #{}", player.getName().getString(), computerId);
                source.sendSuccess(
                        () -> Component.literal("[RC] Computer #" + computerId + " のログをサーバーログに転送します。(ログアウトでリセット)"),
                        false);
            } else {
                Set<Integer> set = STREAMS.get(uuid);
                if (set != null) {
                    set.remove(computerId);
                    if (set.isEmpty()) STREAMS.remove(uuid);
                }
                LOGGER.info("Player {} disabled log stream for Computer #{}", player.getName().getString(), computerId);
                source.sendSuccess(
                        () -> Component.literal("[RC] Computer #" + computerId + " のログ転送を無効にしました。"),
                        false);
            }
        } catch (com.mojang.brigadier.exceptions.CommandSyntaxException e) {
            source.sendSuccess(
                    () -> Component.literal("[RC] /rc log コマンドはプレイヤーのみ使用できます"),
                    false);
        }
        return 1;
    }

    // ------------------------------------------------------------------
    // ログストリーミング状態クエリ / Log streaming state queries
    // ------------------------------------------------------------------

    /**
     * 指定コンピューターのログを転送するプレイヤーが一人でもいるか確認。
     * Check if any player has log streaming enabled for the given computer.
     *
     * @param computerId コンピューター ID / computer ID
     * @return ストリーミング有効なら true / true if any player is streaming
     */
    public static boolean isStreamingFor(int computerId) {
        synchronized (STREAMS) {
            for (Set<Integer> set : STREAMS.values()) {
                if (set.contains(computerId)) return true;
            }
        }
        return false;
    }

    // ------------------------------------------------------------------
    // セッション管理 / Session management
    // ------------------------------------------------------------------

    /**
     * プレイヤーのログアウト時にストリーミング設定をリセットする。
     * Reset log streaming settings when a player logs out.
     *
     * @param playerId ログアウトしたプレイヤーの UUID / UUID of the logged-out player
     */
    public static void clearPlayerStreams(UUID playerId) {
        STREAMS.remove(playerId);
        LOGGER.debug("Cleared log stream settings for player {}", playerId);
    }
}
