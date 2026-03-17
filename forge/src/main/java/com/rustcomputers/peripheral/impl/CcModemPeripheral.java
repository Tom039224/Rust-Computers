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
 * CC:Tweaked Modem ペリフェラル実装。
 * CC:Tweaked Modem peripheral implementation.
 *
 * <p>CC:Tweaked の Modem ペリフェラルで、無線・有線モデムの両方に対応。
 * チャンネルを開いてメッセージを送受信する。</p>
 *
 * <p>Implements CC:Tweaked's Modem peripheral, supporting both wireless and wired modems.
 * Opens channels and sends/receives messages.</p>
 *
 * <h3>Methods:</h3>
 * <ul>
 *   <li><b>open(channel)</b> - チャンネルを開く / Open a channel</li>
 *   <li><b>isOpen(channel)</b> - チャンネルが開いているか確認 / Check if channel is open</li>
 *   <li><b>close(channel)</b> - チャンネルを閉じる / Close a channel</li>
 *   <li><b>closeAll()</b> - 全チャンネルを閉じる / Close all channels</li>
 *   <li><b>transmit(channel, replyChannel, payload)</b> - メッセージを送信 / Transmit a message</li>
 *   <li><b>isWireless()</b> - 無線モデムかどうか / Check if wireless modem</li>
 *   <li><b>getNamesRemote()</b> - 有線ネットワーク上のペリフェラル名を取得 / Get peripheral names on wired network (wired modem only)</li>
 * </ul>
 *
 * <h3>Events:</h3>
 * <ul>
 *   <li><b>modem_message</b> - メッセージ受信イベント / Message received event
 *       <ul>
 *         <li>side: string - モデムの側面 / Modem side</li>
 *         <li>channel: number - 受信チャンネル / Received channel</li>
 *         <li>replyChannel: number - 返信チャンネル / Reply channel</li>
 *         <li>message: any - メッセージ内容 / Message payload</li>
 *         <li>distance: number | nil - 送信者との距離 / Distance to sender</li>
 *       </ul>
 *   </li>
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
 *   <li><b>Query methods</b> (isOpen, isWireless, getNamesRemote):
 *       情報取得系。最後のリクエストのみ有効（上書き）。
 *       callImmediate 対応。</li>
 *   <li><b>Action methods</b> (open, close, closeAll, transmit):
 *       ワールド干渉系。全リクエストを保存（追記）。
 *       callImmediate 非対応。</li>
 * </ul>
 */
public class CcModemPeripheral implements PeripheralType {
    private static final Logger LOGGER = LoggerFactory.getLogger(CcModemPeripheral.class);

    private static final String TYPE_NAME = "modem";
    
    /**
     * サポートされる全メソッド。
     * All supported methods.
     */
    private static final String[] METHODS = {
            "open",                     // Action: チャンネルを開く / Open channel
            "isOpen",                   // Query: チャンネルが開いているか / Check if channel is open
            "close",                    // Action: チャンネルを閉じる / Close channel
            "closeAll",                 // Action: 全チャンネルを閉じる / Close all channels
            "transmit",                 // Action: メッセージ送信 / Transmit message
            "isWireless",               // Query: 無線モデムか / Check if wireless
            "getNamesRemote",           // Query: 有線ネットワーク上のペリフェラル名 / Get remote peripheral names (wired only)
            "try_pull_modem_message"    // Event: modem_message イベント受信 / Receive modem_message event
    };

    /**
     * callImmediate で安全に呼び出せるメソッド（Query メソッドのみ）。
     * Methods safe for callImmediate (Query methods only).
     */
    private static final Set<String> IMMEDIATE_METHODS = new HashSet<>();
    static {
        IMMEDIATE_METHODS.add("isOpen");
        IMMEDIATE_METHODS.add("isWireless");
        IMMEDIATE_METHODS.add("getNamesRemote");
        // open/close/closeAll/transmit は Action なので immediate 非対応
        // open/close/closeAll/transmit are Actions, not supported for immediate
        // try_pull_modem_message はイベント系なので immediate 非対応
        // try_pull_modem_message is an event method, not supported for immediate
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
        // Modem は CC:Tweaked の IPeripheral として実装されているため、
        // CcGenericPeripheral に委譲する
        // Modem is implemented as CC:Tweaked's IPeripheral,
        // so delegate to CcGenericPeripheral
        CcGenericPeripheral delegate = new CcGenericPeripheral(TYPE_NAME, METHODS, IMMEDIATE_METHODS);
        return delegate.callMethod(methodName, args, level, peripheralPos);
    }

    @Override
    public byte[] callImmediate(String methodName, byte[] args,
                                ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        // Action メソッドとイベントメソッドは immediate 非対応
        // Action methods and event methods are not supported for immediate
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
