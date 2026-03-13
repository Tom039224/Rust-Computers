package com.rustcomputers.peripheral.impl;

import com.rustcomputers.computer.ComputerBlockEntity;
import dan200.computercraft.api.ForgeComputerCraftAPI;
import dan200.computercraft.api.peripheral.IPeripheral;
import net.minecraft.world.level.block.entity.BlockEntity;
import net.minecraftforge.common.util.LazyOptional;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nullable;

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
        public boolean equals(@Nullable IPeripheral other) {
            return other instanceof RustComputerPeripheral rp && rp.computer == computer;
        }
    }
}
