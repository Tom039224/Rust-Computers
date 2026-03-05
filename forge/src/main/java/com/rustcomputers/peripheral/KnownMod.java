package com.rustcomputers.peripheral;

import java.nio.charset.StandardCharsets;
import java.util.HashMap;
import java.util.Map;
import java.util.zip.CRC32;

/**
 * RustComputers が認識する既知 Mod の一覧と、その u16 キーのマッピング。
 * Known mods recognized by RustComputers and their u16 key mappings.
 *
 * <p>Rust 側は {@code CRC32(mod_name.as_bytes()) as u16} を送信する。
 * Java 側は {@link java.util.zip.CRC32}（IEEE 802.3、同一アルゴリズム）で
 * 起動時に全エントリの u16 キーを計算してテーブルを構築する。</p>
 *
 * <p>The Rust side sends {@code CRC32(mod_name.as_bytes()) as u16}.
 * Java builds the lookup table at startup using {@link java.util.zip.CRC32}
 * (IEEE 802.3 — same algorithm as Rust).</p>
 */
public final class KnownMod {

    /** CC:Tweaked */
    public static final String COMPUTERCRAFT      = "computercraft";
    /** Create Mod */
    public static final String CREATE             = "create";
    /** Valkyrien Skies 2 */
    public static final String VALKYRIENSKIES     = "valkyrienskies";
    /** Clockwork (VS-Clockwork) */
    public static final String VS_CLOCKWORK       = "vs_clockwork";
    /** CC-VS (CC:Tweaked × Valkyrien Skies) */
    public static final String CC_VS              = "cc_vs";
    /** VS-Addition */
    public static final String VS_ADDITION        = "vs_addition";
    /** Some Peripherals */
    public static final String SOME_PERIPHERALS   = "some_peripherals";
    /** Advanced Peripherals */
    public static final String ADV_PERIPHERALS    = "advancedperipherals";
    /** Tom's Peripherals */
    public static final String TOMS_PERIPHERALS   = "toms_peripherals";
    /** Create Addition (Electricity) */
    public static final String CREATE_ADDITION    = "createaddition";
    /** Clockwork CC Compat */
    public static final String CLOCKWORK_CC_COMPAT = "clockwork_cc_compat";

    /** u16 key → Forge mod ID */
    private static final Map<Integer, String> BY_KEY = new HashMap<>();

    static {
        for (String modId : new String[]{
                COMPUTERCRAFT,
                CREATE,
                VALKYRIENSKIES,
                VS_CLOCKWORK,
                CC_VS,
                VS_ADDITION,
                SOME_PERIPHERALS,
                ADV_PERIPHERALS,
                TOMS_PERIPHERALS,
                CREATE_ADDITION,
                CLOCKWORK_CC_COMPAT,
        }) {
            BY_KEY.put(crc32u16(modId), modId);
        }
    }

    private KnownMod() {}

    /**
     * u16 キーに対応する Forge mod ID を返す。
     * Return the Forge mod ID for the given u16 key (from WASM).
     *
     * @param key Rust 側が送信した u16 値（{@code int & 0xFFFF} で取得） / u16 value sent by Rust
     * @return Forge mod ID, またはキー未登録なら {@code null}
     */
    public static String forKey(int key) {
        return BY_KEY.get(key & 0xFFFF);
    }

    // ------------------------------------------------------------------
    // Internal
    // ------------------------------------------------------------------

    static int crc32u16(String s) {
        CRC32 crc = new CRC32();
        crc.update(s.getBytes(StandardCharsets.UTF_8));
        return (int) (crc.getValue() & 0xFFFFL);
    }
}
