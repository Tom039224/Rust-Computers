package com.rustcomputers.peripheral.impl;

import com.rustcomputers.computer.ComputerBlockEntity;
import dan200.computercraft.api.ForgeComputerCraftAPI;
import dan200.computercraft.api.peripheral.IComputerAccess;
import dan200.computercraft.api.peripheral.IPeripheral;
import net.minecraft.world.level.block.entity.BlockEntity;
import net.minecraftforge.common.util.LazyOptional;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nullable;
import java.util.Collections;
import java.util.Map;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;

/**
 * RustComputers のコンピューターブロックを CC:Tweaked の IPeripheral として公開する。
 * Exposes RustComputers computer blocks as CC:Tweaked IPeripherals.
 *
 * <p>
 * これにより、有線モデムをコンピューターに向けて右クリックした際の
 * 「ローカルペリフェラル接続」が正しく成立し、
 * CC の有線ネットワークに RustComputers コンピューターを参加させられる。
 * </p>
 *
 * <p>
 * This allows wired modem right-click "local peripheral attach" to work on
 * RustComputers computers, so they can properly participate in CC wired networks.
 * </p>
 */
public final class CcRustComputerPeripheralProvider {

    private static final Logger LOGGER = LoggerFactory.getLogger(CcRustComputerPeripheralProvider.class);
    private static volatile boolean registered = false;

        /**
         * コンピュータ座標ごとの接続中 IComputerAccess 集合。
         * Attached IComputerAccess set for each Rust computer block position.
         */
        private static final ConcurrentHashMap<net.minecraft.core.BlockPos, Set<IComputerAccess>> ATTACHED_COMPUTERS =
            new ConcurrentHashMap<>();

    private CcRustComputerPeripheralProvider() {
    }

    /**
     * CC:Tweaked に peripheral provider を 1 回だけ登録する。
     * Register the peripheral provider into CC:Tweaked exactly once.
     */
    public static synchronized void register() {
        if (registered) return;

        ForgeComputerCraftAPI.registerPeripheralProvider((world, pos, side) -> {
            BlockEntity be = world.getBlockEntity(pos);
            if (!(be instanceof ComputerBlockEntity computer)) {
                return LazyOptional.empty();
            }
            return LazyOptional.of(() -> new RustComputerPeripheral(computer));
        });

        registered = true;
        LOGGER.info("Registered CC:Tweaked peripheral provider for RustComputers computer blocks");
    }

    /**
     * 指定 RustComputers コンピュータに接続している CC ネットワーク上の
     * 到達可能ペリフェラルを返す。
     *
     * <p>戻り値には自身（rust_computer）は含めない。</p>
     */
    public static Map<String, IPeripheral> getAvailablePeripherals(net.minecraft.core.BlockPos computerPos) {
        Set<IComputerAccess> accesses = ATTACHED_COMPUTERS.get(computerPos);
        if (accesses == null || accesses.isEmpty()) return Map.of();

        Map<String, IPeripheral> merged = new java.util.HashMap<>();
        for (IComputerAccess access : accesses) {
            try {
                Map<String, IPeripheral> available = access.getAvailablePeripherals();
                if (available == null || available.isEmpty()) continue;

                for (Map.Entry<String, IPeripheral> entry : available.entrySet()) {
                    IPeripheral peripheral = entry.getValue();
                    if (peripheral == null) continue;
                    // 自分自身の公開名は除外
                    if ("rust_computer".equals(peripheral.getType())) continue;
                    merged.putIfAbsent(entry.getKey(), peripheral);
                }
            } catch (RuntimeException ignored) {
                // detach 後などで例外化するケースは無視
            }
        }
        return merged;
    }

    /**
     * RustComputers コンピューターを表す最小 IPeripheral 実装。
     * Minimal IPeripheral implementation representing a RustComputers computer.
     */
    public static final class RustComputerPeripheral implements IPeripheral {
        private final ComputerBlockEntity computer;

        private RustComputerPeripheral(ComputerBlockEntity computer) {
            this.computer = computer;
        }

        @Override
        public String getType() {
            return "rust_computer";
        }

        @Override
        public @Nullable Object getTarget() {
            return computer;
        }

        @Override
        public void attach(IComputerAccess computerAccess) {
            ATTACHED_COMPUTERS
                    .computeIfAbsent(computer.getBlockPos(), p -> Collections.newSetFromMap(new ConcurrentHashMap<>()))
                    .add(computerAccess);
        }

        @Override
        public void detach(IComputerAccess computerAccess) {
            var set = ATTACHED_COMPUTERS.get(computer.getBlockPos());
            if (set == null) return;
            set.remove(computerAccess);
            if (set.isEmpty()) {
                ATTACHED_COMPUTERS.remove(computer.getBlockPos(), set);
            }
        }

        @Override
        public boolean equals(@Nullable IPeripheral other) {
            return other instanceof RustComputerPeripheral rp && rp.computer == computer;
        }
    }
}
