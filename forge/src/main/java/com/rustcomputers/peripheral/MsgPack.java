package com.rustcomputers.peripheral;

import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;
import java.util.List;
import java.util.Map;

/**
 * ペリフェラル結果用の最小 MessagePack エンコーダ / デコーダ。
 * Minimal MessagePack encoder / decoder for peripheral results.
 *
 * <p>サポートする型 / Supported types:
 * <ul>
 *   <li>nil (0xC0)</li>
 *   <li>bool (0xC2 / 0xC3)</li>
 *   <li>positive fixint (0–127), negative fixint (−32〜−1)</li>
 *   <li>uint8 (0xCC), uint16 (0xCD), int32 (0xD2)</li>
 *   <li>fixstr (0xA0–0xBF), str8 (0xD9), str16 (0xDA)</li>
 *   <li>fixarray (0x90–0x9F), array16 (0xDC)</li>
 *   <li>fixmap (0x80–0x8F) — 文字列キーのみ / string keys only</li>
 * </ul>
 * </p>
 */
public final class MsgPack {

    private MsgPack() {}

    // ==================================================================
    // エンコード / Encoding
    // ==================================================================

    /** nil を返す / Return nil. */
    public static byte[] nil() {
        return new byte[]{(byte) 0xC0};
    }

    /** bool 値をエンコードする / Encode a boolean. */
    public static byte[] bool(boolean v) {
        return new byte[]{(byte) (v ? 0xC3 : 0xC2)};
    }

    /**
     * int 値をエンコードする（最小バイト数） / Encode an int (minimal bytes).
     */
    public static byte[] int32(int v) {
        // positive fixint: 0x00–0x7F
        if (v >= 0 && v <= 0x7F)     return new byte[]{(byte) v};
        // negative fixint: 0xE0–0xFF (−32 〜 −1)
        if (v >= -32 && v < 0)       return new byte[]{(byte) v};
        // uint 8
        if (v >= 0 && v <= 0xFF)     return new byte[]{(byte) 0xCC, (byte) v};
        // uint 16
        if (v >= 0 && v <= 0xFFFF)   return new byte[]{(byte) 0xCD, (byte) (v >> 8), (byte) v};
        // int 32
        return new byte[]{
                (byte) 0xD2,
                (byte) (v >> 24), (byte) (v >> 16), (byte) (v >> 8), (byte) v,
        };
    }

    /**
     * 文字列をエンコードする（fixstr / str8 / str16） / Encode a string.
     */
    public static byte[] str(String s) {
        byte[] b = s.getBytes(StandardCharsets.UTF_8);
        if (b.length <= 31) {
            byte[] out = new byte[1 + b.length];
            out[0] = (byte) (0xA0 | b.length);
            System.arraycopy(b, 0, out, 1, b.length);
            return out;
        }
        if (b.length <= 0xFF) {
            byte[] out = new byte[2 + b.length];
            out[0] = (byte) 0xD9;
            out[1] = (byte) b.length;
            System.arraycopy(b, 0, out, 2, b.length);
            return out;
        }
        byte[] out = new byte[3 + b.length];
        out[0] = (byte) 0xDA;
        out[1] = (byte) (b.length >> 8);
        out[2] = (byte) b.length;
        System.arraycopy(b, 0, out, 3, b.length);
        return out;
    }

    /**
     * fixarray / array16 をエンコードする / Encode as fixarray or array16.
     *
     * @param items すでにエンコード済みの要素 / already-encoded elements
     */
    public static byte[] array(byte[]... items) {
        return array(List.of(items));
    }

    /**
     * fixarray / array16 をエンコードする / Encode as fixarray or array16.
     */
    public static byte[] array(List<byte[]> items) {
        int n = items.size();
        int dataLen = items.stream().mapToInt(b -> b.length).sum();
        byte[] out;
        int offset;
        if (n <= 15) {
            out = new byte[1 + dataLen];
            out[0] = (byte) (0x90 | n);
            offset = 1;
        } else {
            out = new byte[3 + dataLen];
            out[0] = (byte) 0xDC;
            out[1] = (byte) (n >> 8);
            out[2] = (byte) n;
            offset = 3;
        }
        for (byte[] item : items) {
            System.arraycopy(item, 0, out, offset, item.length);
            offset += item.length;
        }
        return out;
    }

    /**
     * fixmap をエンコードする（文字列キーのみ、最大 15 エントリ） / Encode as fixmap.
     *
     * @param keysAndValues 交互に String キー・{@code byte[]} 値を指定する
     *                      / alternating String keys and {@code byte[]} values
     */
    public static byte[] map(Object... keysAndValues) {
        int n = keysAndValues.length / 2;
        if (n > 15) throw new IllegalArgumentException("fixmap: max 15 pairs");
        byte[][] encoded = new byte[keysAndValues.length][];
        int dataLen = 0;
        for (int i = 0; i < keysAndValues.length; i++) {
            if (i % 2 == 0) {
                encoded[i] = str((String) keysAndValues[i]);
            } else {
                encoded[i] = (byte[]) keysAndValues[i];
            }
            dataLen += encoded[i].length;
        }
        byte[] out = new byte[1 + dataLen];
        out[0] = (byte) (0x80 | n);
        int offset = 1;
        for (byte[] e : encoded) {
            System.arraycopy(e, 0, out, offset, e.length);
            offset += e.length;
        }
        return out;
    }

    /**
     * float64 (IEEE 754 double) をエンコードする / Encode a double as float64.
     */
    public static byte[] float64(double v) {
        long bits = Double.doubleToRawLongBits(v);
        return new byte[]{
            (byte) 0xCB,
            (byte)(bits >> 56), (byte)(bits >> 48), (byte)(bits >> 40), (byte)(bits >> 32),
            (byte)(bits >> 24), (byte)(bits >> 16), (byte)(bits >> 8),  (byte)(bits)
        };
    }

    /**
     * int64 をエンコードする（最小バイト数） / Encode a long (minimal bytes).
     */
    public static byte[] int64(long v) {
        // int32 範囲に収まれば int32 で / use int32 if it fits
        if (v >= Integer.MIN_VALUE && v <= Integer.MAX_VALUE) return int32((int) v);
        return new byte[]{
            (byte) 0xD3,
            (byte)(v >> 56), (byte)(v >> 48), (byte)(v >> 40), (byte)(v >> 32),
            (byte)(v >> 24), (byte)(v >> 16), (byte)(v >> 8),  (byte)(v)
        };
    }

    /**
     * Map&lt;String, byte[]&gt; を fixmap / map16 でエンコードする。
     * Encode a Map&lt;String, byte[]&gt; as fixmap or map16.
     */
    public static byte[] packMap(Map<String, byte[]> m) {
        int n = m.size();
        int dataLen = 0;
        List<Map.Entry<String, byte[]>> entries = new java.util.ArrayList<>(m.entrySet());
        // compute data length
        for (Map.Entry<String, byte[]> e : entries) {
            dataLen += str(e.getKey()).length + e.getValue().length;
        }
        byte[] out;
        int pos;
        if (n <= 15) {
            out = new byte[1 + dataLen];
            out[0] = (byte)(0x80 | n);
            pos = 1;
        } else {
            out = new byte[3 + dataLen];
            out[0] = (byte) 0xDE;
            out[1] = (byte)(n >> 8);
            out[2] = (byte)(n);
            pos = 3;
        }
        for (Map.Entry<String, byte[]> e : entries) {
            byte[] k = str(e.getKey());
            System.arraycopy(k, 0, out, pos, k.length); pos += k.length;
            byte[] v = e.getValue();
            System.arraycopy(v, 0, out, pos, v.length); pos += v.length;
        }
        return out;
    }

    /**
     * Kotlin / Java の任意値を再帰的に MessagePack にエンコードする。
     * Recursively encode any Kotlin / Java value to MessagePack.
     *
     * <p>対応型 / Supported types:
     * {@code null}, {@link Boolean}, {@link Integer}, {@link Long}, {@link Short},
     * {@link Byte}, {@link Double}, {@link Float}, {@link String},
     * {@link List}, {@link Map} (String keys)</p>
     */
    @SuppressWarnings("unchecked")
    public static byte[] packAny(Object value) {
        if (value == null)              return nil();
        if (value instanceof Boolean)   return bool((Boolean) value);
        if (value instanceof Byte)      return int32((Byte) value);
        if (value instanceof Short)     return int32((Short) value);
        if (value instanceof Integer)   return int32((Integer) value);
        if (value instanceof Long)      return int64((Long) value);
        if (value instanceof Float)     return float64((Float) value);
        if (value instanceof Double)    return float64((Double) value);
        if (value instanceof String)    return str((String) value);
        if (value instanceof List) {
            List<?> list = (List<?>) value;
            List<byte[]> encoded = new java.util.ArrayList<>(list.size());
            for (Object item : list) encoded.add(packAny(item));
            return array(encoded);
        }
        if (value instanceof Map) {
            Map<?, ?> raw = (Map<?, ?>) value;
            Map<String, byte[]> encoded = new java.util.LinkedHashMap<>();
            for (Map.Entry<?, ?> e : raw.entrySet()) {
                encoded.put(String.valueOf(e.getKey()), packAny(e.getValue()));
            }
            return packMap(encoded);
        }
        // フォールバック: toString でエンコード / fallback: encode as string
        return str(value.toString());
    }

    // ==================================================================
    // デコード / Decoding
    // ==================================================================

    /**
     * バイト列から offset 位置の double 値を読む（float32 / float64 / int 対応）。
     * Read a double from bytes at offset (handles float32, float64, and int types).
     *
     * @param data   入力バイト列 / input bytes
     * @param offset 読み取り開始位置 / read start offset
     * @return デコードされた double 値、データ不足なら 0.0
     */
    public static double decodeF64(byte[] data, int offset) {
        if (data == null || data.length <= offset) return 0.0;
        int b = data[offset] & 0xFF;
        // int formats — delegate to decodeInt
        if (b <= 0x7F || b >= 0xE0 ||
            b == 0xCC || b == 0xCD || b == 0xCE ||
            b == 0xD0 || b == 0xD1 || b == 0xD2) {
            return decodeInt(data, offset);
        }
        if (b == 0xCA) { // float32
            if (offset + 4 >= data.length) return 0.0;
            int bits = ((data[offset+1]&0xFF)<<24)|((data[offset+2]&0xFF)<<16)
                      |((data[offset+3]&0xFF)<<8)|(data[offset+4]&0xFF);
            return Float.intBitsToFloat(bits);
        }
        if (b == 0xCB) { // float64
            if (offset + 8 >= data.length) return 0.0;
            long bits = 0;
            for (int i = 1; i <= 8; i++) {
                bits = (bits << 8) | (data[offset + i] & 0xFF);
            }
            return Double.longBitsToDouble(bits);
        }
        return 0.0;
    }

    /**
     * バイト列から offset 位置の int 値を読む（引数デコード用）。
     * Read an int value at offset from bytes (for decoding method arguments).
     *
     * @param data   入力バイト列 / input bytes
     * @param offset 読み取り開始位置 / read start offset
     * @return デコードされた int 値、データ不足なら 0
     */
    public static int decodeInt(byte[] data, int offset) {
        if (data == null || data.length <= offset) return 0;
        int b = data[offset] & 0xFF;
        if (b <= 0x7F)  return b;                      // positive fixint
        if (b >= 0xE0)  return (byte) b;               // negative fixint
        switch (b) {
            case 0xCC:  // uint 8
                return data[offset + 1] & 0xFF;
            case 0xCD:  // uint 16
                return ((data[offset + 1] & 0xFF) << 8) | (data[offset + 2] & 0xFF);
            case 0xCE:  // uint 32
                return ((data[offset + 1] & 0xFF) << 24) | ((data[offset + 2] & 0xFF) << 16)
                        | ((data[offset + 3] & 0xFF) << 8) | (data[offset + 4] & 0xFF);
            case 0xD0:  // int 8
                return (byte) data[offset + 1];
            case 0xD1:  // int 16
                return (short) (((data[offset + 1] & 0xFF) << 8) | (data[offset + 2] & 0xFF));
            case 0xD2:  // int 32
                return ((data[offset + 1] & 0xFF) << 24) | ((data[offset + 2] & 0xFF) << 16)
                        | ((data[offset + 3] & 0xFF) << 8) | (data[offset + 4] & 0xFF);
            default:
                return 0;
        }
    }

    /**
     * msgpack 引数配列の i 番目の要素オフセットを返す（シンプル実装）。
     * Return byte offset of the i-th element inside a msgpack fixarray.
     *
     * <p>fixarray ヘッダ (1 byte) + 各要素を順にスキャンする。
     * fixarray only — for more complex use, add full reader.</p>
     *
     * @param data  引数バイト列 / argument bytes
     * @param index 目的要素インデックス / target element index (0-based)
     * @return 要素の先頭バイトオフセット、取得不能なら -1
     */
    public static int argOffset(byte[] data, int index) {
        if (data == null || data.length == 0) return -1;
        int b0 = data[0] & 0xFF;
        int count;
        int pos;
        if ((b0 & 0xF0) == 0x90) {         // fixarray
            count = b0 & 0x0F;
            pos = 1;
        } else if (b0 == 0xDC && data.length >= 3) { // array 16
            count = ((data[1] & 0xFF) << 8) | (data[2] & 0xFF);
            pos = 3;
        } else {
            // 配列でない場合は単一値として index=0 のみ許可
            return (index == 0) ? 0 : -1;
        }
        if (index >= count) return -1;
        for (int i = 0; i < index; i++) {
            pos = skipElement(data, pos);
            if (pos < 0) return -1;
        }
        return pos;
    }

    /**
     * msgpack 配列の最初の要素を文字列としてデコードする。
     * Decode the first element of a msgpack array as a string.
     *
     * <p>{@code wrap_imm} の {@code hasType} 引数（型名文字列）のデコードに使用する。</p>
     * <p>Used to decode the {@code hasType} argument (type name string) from {@code wrap_imm}.</p>
     *
     * @param data msgpack エンコード済み引数配列 / msgpack-encoded argument array
     * @return デコードされた文字列 / decoded string
     * @throws IllegalArgumentException 引数が不正な場合 / if the argument is invalid
     */
    public static String decodeFirstString(byte[] data) {
        int offset = argOffset(data, 0);
        if (offset < 0 || offset >= data.length) {
            throw new IllegalArgumentException("Cannot decode first string: no element at index 0");
        }
        return decodeStr(data, offset);
    }

    /**
     * バイト列の指定位置から msgpack 文字列をデコードする。
     * Decode a msgpack string from the specified position in bytes.
     *
     * @param data   入力バイト列 / input bytes
     * @param offset 読み取り開始位置 / read start offset
     * @return デコードされた文字列 / decoded string
     * @throws IllegalArgumentException 文字列フォーマットが不正な場合 / if not a valid string format
     */
    public static String decodeStr(byte[] data, int offset) {
        if (data == null || offset >= data.length) {
            throw new IllegalArgumentException("Cannot decode string: insufficient data");
        }
        int b = data[offset] & 0xFF;
        int len;
        int start;
        if ((b & 0xE0) == 0xA0) {           // fixstr (0xa0–0xbf)
            len = b & 0x1F;
            start = offset + 1;
        } else if (b == 0xD9) {              // str 8
            if (offset + 1 >= data.length) throw new IllegalArgumentException("str8 truncated");
            len = data[offset + 1] & 0xFF;
            start = offset + 2;
        } else if (b == 0xDA) {              // str 16
            if (offset + 2 >= data.length) throw new IllegalArgumentException("str16 truncated");
            len = ((data[offset + 1] & 0xFF) << 8) | (data[offset + 2] & 0xFF);
            start = offset + 3;
        } else {
            throw new IllegalArgumentException("Not a msgpack string at offset " + offset + " (byte 0x" + Integer.toHexString(b) + ")");
        }
        if (start + len > data.length) {
            throw new IllegalArgumentException("String data truncated");
        }
        return new String(data, start, len, java.nio.charset.StandardCharsets.UTF_8);
    }

    // ------------------------------------------------------------------
    // Private helpers
    // ------------------------------------------------------------------

    /** 1エレメント分のバイトをスキップして次の先頭オフセットを返す。 */
    private static int skipElement(byte[] data, int pos) {
        if (pos >= data.length) return -1;
        int b = data[pos] & 0xFF;
        if (b <= 0x7F || b >= 0xE0)             return pos + 1;  // fixint/neg fixint
        if ((b & 0xE0) == 0xA0)                  return pos + 1 + (b & 0x1F); // fixstr
        if ((b & 0xF0) == 0x90) {               // fixarray
            int n = b & 0x0F; pos++;
            for (int i = 0; i < n; i++) { pos = skipElement(data, pos); if (pos < 0) return -1; }
            return pos;
        }
        if ((b & 0xF0) == 0x80) {               // fixmap
            int n = b & 0x0F; pos++;
            for (int i = 0; i < n * 2; i++) { pos = skipElement(data, pos); if (pos < 0) return -1; }
            return pos;
        }
        switch (b) {
            case 0xC0: case 0xC2: case 0xC3:    return pos + 1;
            case 0xCC: case 0xD0:               return pos + 2;
            case 0xCD: case 0xD1:               return pos + 3;
            case 0xCE: case 0xD2:               return pos + 5;
            case 0xCF: case 0xD3:               return pos + 9;
            case 0xD9: return pos + 2 + (data[pos + 1] & 0xFF);
            case 0xDA: return pos + 3 + (((data[pos + 1] & 0xFF) << 8) | (data[pos + 2] & 0xFF));
            default:   return -1;
        }
    }
}
