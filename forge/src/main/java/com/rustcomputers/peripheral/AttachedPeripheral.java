package com.rustcomputers.peripheral;

import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;

/**
 * コンピュータに接続された単一のペリフェラルインスタンス。
 * A single peripheral instance attached to a computer.
 *
 * <p>方向（periph_id）、座標、PeripheralType の参照を保持する。</p>
 * <p>Holds the direction (periph_id), position, and PeripheralType reference.</p>
 *
 * @param type        ペリフェラルの型 / peripheral type implementation
 * @param direction   コンピュータから見た方向 / direction relative to the computer
 * @param peripheralPos ペリフェラルブロックの座標 / world position of the peripheral block
 */
public record AttachedPeripheral(
        PeripheralType type,
        Direction direction,
        BlockPos peripheralPos
) {

    /**
     * periph_id を取得する（Direction の ordinal 値）。
     * Get the periph_id (Direction ordinal).
     *
     * @return 0=DOWN, 1=UP, 2=NORTH, 3=SOUTH, 4=WEST, 5=EAST
     */
    public int periphId() {
        return direction.ordinal();
    }

    /**
     * ペリフェラルの型名を取得する。
     * Get the peripheral type name.
     */
    public String typeName() {
        return type.getTypeName();
    }
}
