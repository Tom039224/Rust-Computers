package com.rustcomputers.peripheral;

import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;

import javax.annotation.Nullable;

/**
 * コンピュータに接続された単一のペリフェラルインスタンス。
 * A single peripheral instance attached to a computer.
 *
 * <p>直接接続（periph_id 0–5）の場合 direction は非 null、
 * 有線モデム経由（periph_id 6+）の場合 direction は null。</p>
 * <p>For direct connections (periph_id 0–5) direction is non-null;
 * for wired modem connections (periph_id 6+) direction is null.</p>
 *
 * @param type          ペリフェラルの型 / peripheral type implementation
 * @param direction     コンピュータから見た方向（有線モデム経由は null）
 *                      / direction relative to the computer (null for wired modem)
 * @param peripheralPos ペリフェラルブロックの座標 / world position of the peripheral block
 */
public record AttachedPeripheral(
        PeripheralType type,
        @Nullable Direction direction,
        BlockPos peripheralPos
) {

    /**
     * ペリフェラルの型名を取得する。
     * Get the peripheral type name.
     */
    public String typeName() {
        return type.getTypeName();
    }
}
