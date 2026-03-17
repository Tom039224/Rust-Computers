package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashSet;
import java.util.Set;

/**
 * CC:Tweaked Inventory ペリフェラル実装。
 * CC:Tweaked Inventory peripheral implementation.
 *
 * <p>CC:Tweaked の GenericPeripheral として実装されており、
 * 任意のインベントリブロック（チェスト、バレル等）に動的に付与される。
 * 有線ネットワーク上のインベントリ間でアイテムを直接移動できる。</p>
 *
 * <p>Implemented as CC:Tweaked's GenericPeripheral, dynamically attached to
 * any inventory block (chest, barrel, etc.). Allows direct item transfer
 * between inventories on a wired network.</p>
 *
 * <h3>Methods:</h3>
 * <ul>
 *   <li><b>size()</b> - インベントリのスロット数を返す / Returns the number of slots</li>
 *   <li><b>list()</b> - 全スロットのアイテム情報テーブルを返す / Returns item info for all slots</li>
 *   <li><b>getItemDetail(slot)</b> - 指定スロットのアイテム詳細情報を返す / Returns detailed item info for a slot</li>
 *   <li><b>getItemLimit(slot)</b> - 指定スロットの最大収納数を返す / Returns max stack size for a slot</li>
 *   <li><b>pushItems(toName, fromSlot, limit?, toSlot?)</b> - 別インベントリにアイテムを移動 / Moves items to another inventory</li>
 *   <li><b>pullItems(fromName, fromSlot, limit?, toSlot?)</b> - 別インベントリからアイテムを引き出す / Pulls items from another inventory</li>
 * </ul>
 *
 * <h3>Three-Function Pair Pattern:</h3>
 * <p>各メソッドは Rust 側で以下の3つの形式で提供される:</p>
 * <ul>
 *   <li><b>book_next_*(args)</b> - リクエストを予約 / Book a request</li>
 *   <li><b>read_last_*()</b> - 前tickの結果を読み取り / Read result from previous tick</li>
 *   <li><b>async_*(args)</b> - .await で結果を取得 / Get result with .await</li>
 * </ul>
 *
 * <h3>Query vs Action Methods:</h3>
 * <ul>
 *   <li><b>Query methods</b> (size, list, getItemDetail, getItemLimit):
 *       情報取得系。最後のリクエストのみ有効（上書き）。
 *       callImmediate 対応。</li>
 *   <li><b>Action methods</b> (pushItems, pullItems):
 *       ワールド干渉系。全リクエストを保存（追記）。
 *       複数の結果を返す。callImmediate 非対応。</li>
 * </ul>
 */
public class CcInventoryPeripheral implements PeripheralType {
    private static final Logger LOGGER = LoggerFactory.getLogger(CcInventoryPeripheral.class);

    private static final String TYPE_NAME = "inventory";
    
    /**
     * サポートされる全メソッド。
     * All supported methods.
     */
    private static final String[] METHODS = {
            "size",           // Query: スロット数取得 / Get slot count
            "list",           // Query: 全アイテムリスト / List all items
            "getItemDetail",  // Query: アイテム詳細取得 / Get item details
            "getItemLimit",   // Query: スロット上限取得 / Get slot limit
            "pushItems",      // Action: アイテム移動 / Push items
            "pullItems"       // Action: アイテム引き出し / Pull items
    };

    /**
     * callImmediate で安全に呼び出せるメソッド（Query メソッドのみ）。
     * Methods safe for callImmediate (Query methods only).
     */
    private static final Set<String> IMMEDIATE_METHODS = new HashSet<>();
    static {
        IMMEDIATE_METHODS.add("size");
        IMMEDIATE_METHODS.add("list");
        IMMEDIATE_METHODS.add("getItemDetail");
        IMMEDIATE_METHODS.add("getItemLimit");
        // pushItems/pullItems は Action なので immediate 非対応
        // pushItems/pullItems are Actions, not supported for immediate
    }

    @Override
    public String getTypeName() {
        return TYPE_NAME;
    }

    @Override
    public String[] getMethodNames() {
        return METHODS.clone();
    }

    @Override
    public byte[] callMethod(String methodName, byte[] args,
                             ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        // Inventory は CC:Tweaked の GenericPeripheral として実装されているため、
        // CcGenericPeripheral に委譲する
        // Inventory is implemented as CC:Tweaked's GenericPeripheral,
        // so delegate to CcGenericPeripheral
        CcGenericPeripheral delegate = new CcGenericPeripheral(TYPE_NAME, METHODS, IMMEDIATE_METHODS);
        return delegate.callMethod(methodName, args, level, peripheralPos);
    }

    @Override
    public byte[] callImmediate(String methodName, byte[] args,
                                ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        // pushItems/pullItems はアクション系なので immediate 非対応
        // pushItems/pullItems are action methods, not supported for immediate
        if (!IMMEDIATE_METHODS.contains(methodName)) {
            LOGGER.debug("Method '{}' is not supported for immediate call", methodName);
            return null;
        }

        // Query メソッドは immediate 対応
        // Query methods support immediate
        CcGenericPeripheral delegate = new CcGenericPeripheral(TYPE_NAME, METHODS, IMMEDIATE_METHODS);
        return delegate.callImmediate(methodName, args, level, peripheralPos);
    }
}
