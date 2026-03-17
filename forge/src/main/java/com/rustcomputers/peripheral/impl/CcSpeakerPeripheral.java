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
 * CC:Tweaked Speaker ペリフェラル実装。
 * CC:Tweaked Speaker peripheral implementation.
 *
 * <p>CC:Tweaked の Speaker ペリフェラルで、ノート音・サウンドイベント・PCMオーディオの再生が可能。</p>
 *
 * <p>Implements CC:Tweaked's Speaker peripheral, supporting note playback, sound events, and PCM audio streaming.</p>
 *
 * <h3>Methods:</h3>
 * <ul>
 *   <li><b>playNote(instrument, volume?, pitch?)</b> - ノートブロックの音を再生 / Play a note block sound</li>
 *   <li><b>playSound(name, volume?, pitch?)</b> - サウンドイベントを再生 / Play a sound event</li>
 *   <li><b>playAudio(audio, volume?)</b> - PCMオーディオデータを再生 / Play PCM audio data</li>
 *   <li><b>stop()</b> - 再生中のオーディオを停止 / Stop playing audio</li>
 * </ul>
 *
 * <h3>Events:</h3>
 * <ul>
 *   <li><b>speaker_audio_empty</b> - スピーカーのオーディオバッファが空になったときに発生 / Fired when speaker audio buffer is empty
 *       <ul>
 *         <li>name: string - スピーカーの名前 / Speaker name</li>
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
 *   <li><b>Action methods</b> (playNote, playSound, playAudio, stop):
 *       ワールド干渉系。全リクエストを保存（追記）。
 *       callImmediate 非対応。</li>
 * </ul>
 *
 * <h3>Valid Instruments:</h3>
 * <p>playNote で使用可能な楽器名:</p>
 * <ul>
 *   <li>harp, basedrum, snare, hat, bass</li>
 *   <li>flute, bell, guitar, chime, xylophone</li>
 *   <li>iron_xylophone, cow_bell, didgeridoo, bit, banjo, pling</li>
 * </ul>
 *
 * <h3>Limitations:</h3>
 * <ul>
 *   <li>playNote: 1ティックあたり最大8音まで再生可能 / Max 8 notes per tick</li>
 *   <li>playSound: 一度に1つのサウンドのみ再生可能 / Only 1 sound at a time</li>
 *   <li>playAudio: 最大128×1024サンプル、48kHz、8ビットPCM / Max 128×1024 samples, 48kHz, 8-bit PCM</li>
 * </ul>
 */
public class CcSpeakerPeripheral implements PeripheralType {
    private static final Logger LOGGER = LoggerFactory.getLogger(CcSpeakerPeripheral.class);

    private static final String TYPE_NAME = "speaker";
    
    /**
     * サポートされる全メソッド。
     * All supported methods.
     */
    private static final String[] METHODS = {
            "playNote",                     // Action: ノート音を再生 / Play note sound
            "playSound",                    // Action: サウンドイベントを再生 / Play sound event
            "playAudio",                    // Action: PCMオーディオを再生 / Play PCM audio
            "stop",                         // Action: 再生停止 / Stop playback
            "try_pull_speaker_audio_empty"  // Event: speaker_audio_empty イベント受信 / Receive speaker_audio_empty event
    };

    /**
     * callImmediate で安全に呼び出せるメソッド（Query メソッドのみ）。
     * Methods safe for callImmediate (Query methods only).
     * 
     * Speaker は全てアクション系メソッドなので immediate 対応メソッドは無し。
     * Speaker has only action methods, so no immediate methods.
     */
    private static final Set<String> IMMEDIATE_METHODS = new HashSet<>();
    static {
        // Speaker は全てアクション系なので immediate 非対応
        // All Speaker methods are actions, not supported for immediate
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
        // Speaker は CC:Tweaked の IPeripheral として実装されているため、
        // CcGenericPeripheral に委譲する
        // Speaker is implemented as CC:Tweaked's IPeripheral,
        // so delegate to CcGenericPeripheral
        CcGenericPeripheral delegate = new CcGenericPeripheral(TYPE_NAME, METHODS, IMMEDIATE_METHODS);
        return delegate.callMethod(methodName, args, level, peripheralPos);
    }

    @Override
    public byte[] callImmediate(String methodName, byte[] args,
                                ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        // Speaker は全てアクション系メソッドなので immediate 非対応
        // All Speaker methods are actions, not supported for immediate
        LOGGER.debug("Method '{}' is not supported for immediate call", methodName);
        return null;
    }
}
