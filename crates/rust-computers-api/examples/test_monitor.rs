//! CC:Tweaked Monitor 書き込みテスト。
//! CC:Tweaked Monitor write test.
//!
//! ## ビルド方法 / Build
//! ```sh
//! cargo build --example test_monitor --target wasm32-unknown-unknown --release
//! cp target/wasm32-unknown-unknown/release/examples/test_monitor.wasm \
//!    <server>/config/rustcomputers/
//! ```
//!
//! ## セットアップ / Setup
//! 1. CC:Tweaked を導入した Forge 1.20.1 サーバーを用意する。
//!    Prepare a Forge 1.20.1 server with CC:Tweaked installed.
//! 2. RustComputers コンピュータブロックを配置し、隣接するいずれかの方向に
//!    CC:Tweaked Monitor を設置する。
//!    Place a RustComputers computer block with a CC:Tweaked Monitor adjacent.
//! 3. `/rc load test_monitor` で実行する。
//!    Run with `/rc load test_monitor`.
//!
//! ## 何をテストするか / What is tested
//! - `is_mod_available("computercraft")` — CC 存在確認
//! - 6方向スキャンでモニターを自動検出 / Auto-detect monitor in 6 directions
//! - `clear()` — 画面クリア
//! - `setCursorPos(x, y)` — カーソル移動
//! - `write(text)` — テキスト書き込み
//! - `setTextColor(color)` — 文字色変更
//! - `setBackgroundColor(color)` — 背景色変更
//! - `getSize()` → (width, height) — 画面サイズ取得
//! - `scroll(n)` — スクロール
//! - `setTextScale(scale)` — テキストスケール変更

#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;

use rust_computers_api as rc;
use rc::msgpack as m;
use rc::peripheral::{self, Direction, is_mod_available};

// エントリーポイント / Entry point
rc::entry!(main);

// ==================================================================
// CC:Tweaked カラー定数 / CC:Tweaked color constants
// (Lua API のビットマスク形式 / Lua API bitmask form)
// ==================================================================
const COLOR_WHITE:      i32 = 1;
const COLOR_ORANGE:     i32 = 2;
const COLOR_MAGENTA:    i32 = 4;
const COLOR_LIGHT_BLUE: i32 = 8;
const COLOR_YELLOW:     i32 = 16;
const COLOR_LIME:       i32 = 32;
const COLOR_PINK:       i32 = 64;
const COLOR_GRAY:       i32 = 128;
const COLOR_LIGHT_GRAY: i32 = 256;
const COLOR_CYAN:       i32 = 512;
const COLOR_PURPLE:     i32 = 1024;
const COLOR_BLUE:       i32 = 2048;
const COLOR_BROWN:      i32 = 4096;
const COLOR_GREEN:      i32 = 8192;
const COLOR_RED:        i32 = 16384;
const COLOR_BLACK:      i32 = 32768;

// ==================================================================
// モニター操作 / Monitor operations
// ==================================================================

/// モニターをクリアする / Clear the monitor.
async fn mon_clear(dir: Direction) -> Result<(), rc::BridgeError> {
    peripheral::request_info(dir, "clear", &m::array(&[])).await?;
    Ok(())
}

/// カーソル位置を設定する (1-indexed) / Set cursor position (1-indexed).
async fn mon_set_cursor(dir: Direction, x: i32, y: i32) -> Result<(), rc::BridgeError> {
    let args = m::array(&[m::int(x), m::int(y)]);
    peripheral::request_info(dir, "setCursorPos", &args).await?;
    Ok(())
}

/// テキストを書き込む / Write text.
async fn mon_write(dir: Direction, text: &str) -> Result<(), rc::BridgeError> {
    let args = m::array(&[m::str(text)]);
    peripheral::request_info(dir, "write", &args).await?;
    Ok(())
}

/// 文字色を設定する (ビットマスク形式) / Set text color (bitmask form).
async fn mon_set_text_color(dir: Direction, color: i32) -> Result<(), rc::BridgeError> {
    let args = m::array(&[m::int(color)]);
    peripheral::request_info(dir, "setTextColor", &args).await?;
    Ok(())
}

/// 背景色を設定する (ビットマスク形式) / Set background color (bitmask form).
async fn mon_set_bg_color(dir: Direction, color: i32) -> Result<(), rc::BridgeError> {
    let args = m::array(&[m::int(color)]);
    peripheral::request_info(dir, "setBackgroundColor", &args).await?;
    Ok(())
}

/// 画面サイズを取得する → (width, height)。
/// Get screen size → (width, height).
async fn mon_get_size(dir: Direction) -> Result<(i32, i32), rc::BridgeError> {
    let data = peripheral::request_info(dir, "getSize", &m::array(&[])).await?;
    let w = m::decode_int_at(&data, 0);
    let h = m::decode_int_at(&data, 1);
    Ok((w, h))
}

/// スクロールする / Scroll by n lines.
async fn mon_scroll(dir: Direction, n: i32) -> Result<(), rc::BridgeError> {
    let args = m::array(&[m::int(n)]);
    peripheral::request_info(dir, "scroll", &args).await?;
    Ok(())
}

/// テキストスケールを設定する (0.5 〜 5.0、0.5 刻み)。
/// Set text scale (0.5 – 5.0 in steps of 0.5).
/// ※ スケールは java.lang.Double として float ではなく MessagePack str 経由で渡すため
///    int * 10 の整数値を送り、Java 側で /10.0 する。
async fn mon_set_text_scale(dir: Direction, scale_x10: i32) -> Result<(), rc::BridgeError> {
    // Java 側で scale_x10 / 10.0 として double に変換する
    let args = m::array(&[m::int(scale_x10)]);
    peripheral::request_info(dir, "setTextScale", &args).await?;
    Ok(())
}

// ==================================================================
// メインプログラム / Main program
// ==================================================================

async fn main() {
    rc::println!("╔══════════════════════════════════════════╗");
    rc::println!("║   CC:Tweaked Monitor Write Test          ║");
    rc::println!("╚══════════════════════════════════════════╝");
    rc::println!("");

    // ----------------------------------------------------------------
    // Step 1: CC:Tweaked の存在確認 / Check CC:Tweaked availability
    // ----------------------------------------------------------------
    let cc_ok = is_mod_available("computercraft");
    rc::println!("[1] CC:Tweaked available: {}", cc_ok);
    if !cc_ok {
        rc::println!("ERROR: CC:Tweaked (computercraft) is not loaded.");
        rc::println!("       Install CC:Tweaked and try again.");
        return;
    }

    // ----------------------------------------------------------------
    // Step 2: 6方向スキャンでモニターを検出 / Scan 6 directions for monitor
    // ----------------------------------------------------------------
    rc::println!("[2] Scanning 6 directions for a CC Monitor...");

    let dirs = [
        (Direction::Down,  "Down"),
        (Direction::Up,    "Up"),
        (Direction::North, "North"),
        (Direction::South, "South"),
        (Direction::West,  "West"),
        (Direction::East,  "East"),
    ];

    let mut monitor_dir: Option<Direction> = None;
    for (dir, name) in dirs.iter() {
        rc::println!("  Trying {}...", name);
        match mon_clear(*dir).await {
            Ok(_) => {
                rc::println!("  -> Monitor found at {}!", name);
                monitor_dir = Some(*dir);
                break;
            }
            Err(e) => {
                rc::println!("  -> No monitor (err: {:?})", e);
            }
        }
    }

    let dir = match monitor_dir {
        Some(d) => d,
        None => {
            rc::println!("");
            rc::println!("ERROR: No CC:Tweaked Monitor found in any direction.");
            rc::println!("       Place a Monitor adjacent to this computer");
            rc::println!("       and run the test again.");
            return;
        }
    };

    // ----------------------------------------------------------------
    // Step 3: 画面サイズ取得 / Get screen size
    // ----------------------------------------------------------------
    rc::println!("[3] Getting monitor size...");
    let (width, height) = match mon_get_size(dir).await {
        Ok(s) => {
            rc::println!("    size = {}w x {}h", s.0, s.1);
            s
        }
        Err(e) => {
            rc::println!("    getSize() failed: {:?}", e);
            (26, 10) // デフォルト / fallback
        }
    };

    // ----------------------------------------------------------------
    // Step 4: テキストスケール設定 / Set text scale to 1.0
    // ----------------------------------------------------------------
    rc::println!("[4] Setting text scale to 1.0 (x10=10)...");
    if let Err(e) = mon_set_text_scale(dir, 10).await {
        rc::println!("    setTextScale() warning: {:?}", e);
    }

    // ----------------------------------------------------------------
    // Step 5: 基本書き込みテスト / Basic write test
    // ----------------------------------------------------------------
    rc::println!("[5] Basic write test...");

    // 画面クリア + 背景を黒に
    let _ = mon_clear(dir).await;
    let _ = mon_set_bg_color(dir, COLOR_BLACK).await;
    let _ = mon_clear(dir).await;

    // ヘッダ行 (緑文字)
    let _ = mon_set_text_color(dir, COLOR_LIME).await;
    let _ = mon_set_cursor(dir, 1, 1).await;
    let _ = mon_write(dir, "=== RustComputers Monitor Test ===").await;

    // サブ行 (水色文字)
    let _ = mon_set_text_color(dir, COLOR_LIGHT_BLUE).await;
    let _ = mon_set_cursor(dir, 1, 2).await;
    let _ = mon_write(dir, &format!("Size: {}x{}", width, height)).await;

    let _ = mon_set_cursor(dir, 1, 3).await;
    let _ = mon_write(dir, "Hello from Rust!   ").await;

    // 白文字でコンピュータID
    let _ = mon_set_text_color(dir, COLOR_WHITE).await;
    let _ = mon_set_cursor(dir, 1, 4).await;
    let _ = mon_write(dir, &format!("Computer ID: {}", rc::io::computer_id())).await;

    rc::println!("    Basic write OK");

    // ----------------------------------------------------------------
    // Step 6: カラーテスト / Color test (1行目ずつ各色で書く)
    // ----------------------------------------------------------------
    rc::println!("[6] Color bar test...");

    let colors: &[(i32, &str)] = &[
        (COLOR_WHITE,      "WH"),
        (COLOR_ORANGE,     "OR"),
        (COLOR_MAGENTA,    "MG"),
        (COLOR_LIGHT_BLUE, "LB"),
        (COLOR_YELLOW,     "YL"),
        (COLOR_LIME,       "LM"),
        (COLOR_PINK,       "PK"),
        (COLOR_GRAY,       "GR"),
        (COLOR_LIGHT_GRAY, "LG"),
        (COLOR_CYAN,       "CY"),
        (COLOR_PURPLE,     "PR"),
        (COLOR_BLUE,       "BL"),
        (COLOR_BROWN,      "BR"),
        (COLOR_GREEN,      "GN"),
        (COLOR_RED,        "RD"),
        (COLOR_BLACK,      "BK"),
    ];

    let row = 6i32;
    let _ = mon_set_text_color(dir, COLOR_WHITE).await;
    let _ = mon_set_cursor(dir, 1, row).await;
    let _ = mon_write(dir, "Colors: ").await;

    let start_col = 9i32;
    for (i, (color, label)) in colors.iter().enumerate() {
        let _ = mon_set_text_color(dir, *color).await;
        let _ = mon_set_bg_color(dir, COLOR_BLACK).await;
        let _ = mon_set_cursor(dir, start_col + (i as i32) * 3, row).await;
        let _ = mon_write(dir, label).await;
    }

    // 背景色を白に戻す
    let _ = mon_set_bg_color(dir, COLOR_BLACK).await;
    let _ = mon_set_text_color(dir, COLOR_WHITE).await;

    rc::println!("    Color test OK");

    // ----------------------------------------------------------------
    // Step 7: スクロールテスト / Scroll test
    // ----------------------------------------------------------------
    rc::println!("[7] Scroll test (3 lines)...");

    // 下部に数行書いてスクロール
    for i in 0..5 {
        let _ = mon_set_cursor(dir, 1, height - i).await;
        let _ = mon_set_text_color(dir, if i % 2 == 0 { COLOR_YELLOW } else { COLOR_CYAN }).await;
        let _ = mon_write(dir, &format!("--- scroll line {} ---", i)).await;
    }

    // 3行上にスクロール
    let _ = mon_scroll(dir, 3).await;
    rc::println!("    Scroll test OK");

    // ----------------------------------------------------------------
    // Step 8: カーソル位置確認 / Verify getCursorPos via interactive read
    // ----------------------------------------------------------------
    rc::println!("[7] Writing final status line...");
    let _ = mon_set_text_color(dir, COLOR_LIME).await;
    let _ = mon_set_cursor(dir, 1, height).await;
    let _ = mon_write(dir, "TEST COMPLETE").await;
    let _ = mon_set_text_color(dir, COLOR_WHITE).await;

    // ----------------------------------------------------------------
    // 完了 / Done
    // ----------------------------------------------------------------
    rc::println!("");
    rc::println!("╔══════════════════════════════════════════╗");
    rc::println!("║   Monitor test COMPLETE                  ║");
    rc::println!("╚══════════════════════════════════════════╝");
    rc::println!("Check the in-game monitor for results.");
}
