package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.MsgPack;
import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.Container;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.level.block.entity.BlockEntity;

import java.util.ArrayList;
import java.util.List;

/**
 * バニラ Minecraft のインベントリ系ブロック向けペリフェラル。
 * Peripheral for vanilla Minecraft inventory blocks.
 *
 * <h3>サポートするブロック / Supported blocks</h3>
 * <ul>
 *   <li>Chest / Trapped Chest</li>
 *   <li>Barrel</li>
 *   <li>Furnace / Blast Furnace / Smoker</li>
 *   <li>Hopper</li>
 *   <li>Dropper / Dispenser</li>
 *   <li>Shulker Box (全色) / all colors</li>
 * </ul>
 *
 * <h3>メソッド一覧 / Methods</h3>
 * <pre>
 * getSize()         → int        インベントリのスロット数
 * getItem(slot)     → map|nil    スロットのアイテム情報、空なら nil
 * listItems()       → array      空でないスロットの一覧
 * getItemCount(slot)→ int        スロットのアイテム数（空なら 0）
 *
 * getSize()         → int        number of inventory slots
 * getItem(slot)     → map|nil    item info for slot, nil if empty
 * listItems()       → array      non-empty slot list
 * getItemCount(slot)→ int        item count in slot (0 if empty)
 * </pre>
 *
 * <h3>アイテムマップのキー / Item map keys</h3>
 * <pre>
 * "id"    → string   "minecraft:cobblestone" 形式のアイテム ID
 * "count" → int      スタック数
 * "maxCount" → int   最大スタック数
 * </pre>
 */
public class VanillaInventoryPeripheral implements PeripheralType {

    private static final String[] METHODS = {
            "getSize", "getItem", "listItems", "getItemCount",
    };

    @Override
    public String getTypeName() {
        return "inventory";
    }

    @Override
    public String[] getMethodNames() {
        return METHODS.clone();
    }

    @Override
    public byte[] callMethod(String methodName, byte[] args,
                             ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {

        Container container = getContainer(level, peripheralPos);

        switch (methodName) {
            case "getSize":
                return MsgPack.int32(container.getContainerSize());

            case "getItem": {
                int slot = getIntArg(args, 0, "slot");
                if (slot < 0 || slot >= container.getContainerSize()) {
                    throw new PeripheralException(
                            "slot " + slot + " out of range (0–" + (container.getContainerSize() - 1) + ")");
                }
                ItemStack stack = container.getItem(slot);
                if (stack.isEmpty()) {
                    return MsgPack.nil();
                }
                return encodeItemStack(stack);
            }

            case "listItems": {
                List<byte[]> items = new ArrayList<>();
                for (int i = 0; i < container.getContainerSize(); i++) {
                    ItemStack stack = container.getItem(i);
                    if (!stack.isEmpty()) {
                        items.add(MsgPack.map(
                                "slot",     MsgPack.int32(i),
                                "id",       MsgPack.str(itemId(stack)),
                                "count",    MsgPack.int32(stack.getCount()),
                                "maxCount", MsgPack.int32(stack.getMaxStackSize())
                        ));
                    }
                }
                return MsgPack.array(items);
            }

            case "getItemCount": {
                int slot = getIntArg(args, 0, "slot");
                if (slot < 0 || slot >= container.getContainerSize()) {
                    throw new PeripheralException(
                            "slot " + slot + " out of range (0–" + (container.getContainerSize() - 1) + ")");
                }
                return MsgPack.int32(container.getItem(slot).getCount());
            }

            default:
                throw new PeripheralException("Unknown method: " + methodName);
        }
    }

    // 読み取り専用なので callImmediate でも利用可能
    // All methods are read-only, so immediate calls are safe.
    @Override
    public byte[] callImmediate(String methodName, byte[] args,
                                ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        return callMethod(methodName, args, level, peripheralPos);
    }

    // ------------------------------------------------------------------
    // Helpers
    // ------------------------------------------------------------------

    private static Container getContainer(ServerLevel level, BlockPos pos)
            throws PeripheralException {
        BlockEntity be = level.getBlockEntity(pos);
        if (be instanceof Container c) {
            return c;
        }
        throw new PeripheralException("No inventory found at the peripheral position");
    }

    private static byte[] encodeItemStack(ItemStack stack) {
        return MsgPack.map(
                "id",       MsgPack.str(itemId(stack)),
                "count",    MsgPack.int32(stack.getCount()),
                "maxCount", MsgPack.int32(stack.getMaxStackSize())
        );
    }

    /** アイテムの名前空間付き ID を返す / Return the namespaced item ID. */
    @SuppressWarnings("deprecation")
    private static String itemId(ItemStack stack) {
        return stack.getItem().builtInRegistryHolder().key().location().toString();
    }

    /** args の index 番目から int を読む / Read the int at args[index]. */
    private static int getIntArg(byte[] args, int index, String argName)
            throws PeripheralException {
        int offset = MsgPack.argOffset(args, index);
        if (offset < 0) {
            throw new PeripheralException("Missing argument: " + argName);
        }
        return MsgPack.decodeInt(args, offset);
    }
}
