package com.rustcomputers.computer;

import net.minecraft.ChatFormatting;
import net.minecraft.network.chat.Component;
import net.minecraft.world.item.BlockItem;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.TooltipFlag;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.Block;

import javax.annotation.Nullable;
import java.util.List;

/**
 * コンピューターアイテム — ブロックアイテムに ComputerId を NBT として保持する。
 * Computer item — stores ComputerId as NBT on the item stack.
 *
 * <p>CC:Tweaked の {@code IComputerItem} パターンに倣い、アイテムに ID を持たせることで
 * ホイールクリック（クリエイティブコピー）で同一 ID のコンピューターを複製できる。</p>
 *
 * <p>Following CC:Tweaked's {@code IComputerItem} pattern, storing the ID on the item
 * allows middle-click (creative pick) to duplicate computers with the same ID.</p>
 */
public class ComputerItem extends BlockItem {

    /** NBT キー / NBT key — CC:Tweaked と同じ値を使用 */
    public static final String NBT_ID = "ComputerId";

    public ComputerItem(Block block, Properties properties) {
        super(block, properties);
    }

    // ------------------------------------------------------------------
    // ID アクセサ / ID accessors
    // ------------------------------------------------------------------

    /**
     * アイテムスタックからコンピューター ID を取得する。
     * Get the computer ID from the item stack.
     *
     * @param stack アイテムスタック / the item stack
     * @return コンピューター ID、未設定なら {@code -1} / computer ID, or {@code -1} if unset
     */
    public static int getComputerId(ItemStack stack) {
        var nbt = stack.getTag();
        return nbt != null && nbt.contains(NBT_ID) ? nbt.getInt(NBT_ID) : -1;
    }

    /**
     * アイテムスタックにコンピューター ID を設定する。
     * Set the computer ID on the item stack.
     *
     * @param stack アイテムスタック / the item stack
     * @param id    設定する ID / the ID to set
     */
    public static void setComputerId(ItemStack stack, int id) {
        stack.getOrCreateTag().putInt(NBT_ID, id);
    }

    /**
     * 指定 ID が設定されたアイテムスタックを生成する。
     * Create an item stack with the specified computer ID.
     *
     * @param item アイテム / the item
     * @param id   コンピューター ID / the computer ID
     * @return ID 付きスタック / stack with ID
     */
    public static ItemStack withComputerId(ComputerItem item, int id) {
        var stack = new ItemStack(item);
        if (id >= 0) {
            setComputerId(stack, id);
        }
        return stack;
    }

    // ------------------------------------------------------------------
    // ツールチップ / Tooltip
    // ------------------------------------------------------------------

    @Override
    public void appendHoverText(ItemStack stack, @Nullable Level level,
                                List<Component> tooltip, TooltipFlag flag) {
        super.appendHoverText(stack, level, tooltip, flag);
        int id = getComputerId(stack);
        if (id >= 0) {
            // 通常モード: "Computer ID: 42"
            // 高度モード (F3+H): 常に表示
            tooltip.add(Component.literal("Computer ID: " + id)
                    .withStyle(ChatFormatting.GRAY));
        }
    }
}
