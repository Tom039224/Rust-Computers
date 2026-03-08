//! CC:Tweaked Monitor 書き込みテスト / Monitor write test.
//!
//! ## Build
//! ```sh
//! cargo build --example test_monitor --target wasm32-unknown-unknown --release
//! ```
//!
//! ## Setup
//! Place a CC:Tweaked Monitor adjacent to this computer, then run
//! `/rc load test_monitor` in game.
//!
//! ## What is tested
//! - `peripheral::is_mod_available("computercraft")` — CC availability check
//! - Auto-detect monitor via `peripheral::find_imm::<Monitor>()` (v0.2.0+ API)
//! - Immediate queries: `get_size_imm()` / `get_text_scale_imm()`
//! - Drawing via `book_next_*()` + `rc::wait_for_next_tick().await` + `read_last_*()`
//! - Color, scroll, and cursor tests
//!
//! ## v0.2.0+ API の基本パターン / Basic pattern of v0.2.0+ API
//!
//! v0.1.x とは違い、Monitor のメソッドは `async fn` ではありません。
//! Unlike v0.1.x, Monitor methods are NOT async functions.
//! 代わりに以下のパターンを使いましょう:
//! Instead, use this pattern:
//!
//! ```ignore
//! // 1. 1つ以上のアクションをキューに入れる / Book one or more actions
//! mon.book_next_write("text");
//! mon.book_next_set_cursor_pos(MonitorPosition { x: 1, y: 2 });
//!
//! // 2. 1ティックで全部フラッシュ / Flush all in ONE tick
//! rc::wait_for_next_tick().await;
//!
//! // 3. 結果を読む / Read the results
//! for r in mon.read_last_write() { r.unwrap(); }
//! ```
//!
//! `wait_for_next_tick().await` を1回呼ぶだけで、その前の全 `book_next_*()` 呼び出しが
//! まとめて送信されます（何回呼んでも1ティック分の遅延のみ）。
//! One `wait_for_next_tick().await` flushes ALL preceding `book_next_*()` calls
//! together (only one game-tick of latency regardless of how many were booked).

#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;

use rust_computers_api as rc;
use rc::computer_craft::monitor::{Monitor, MonitorColor, MonitorPosition, MonitorTextScale};
use rc::peripheral;

rc::entry!(main);

async fn main() {
    rc::println!("╔══════════════════════════════════════════╗");
    rc::println!("║   CC:Tweaked Monitor Write Test          ║");
    rc::println!("║   (v0.2.0+ API)                          ║");
    rc::println!("╚══════════════════════════════════════════╝");

    // ---------------------------------------------------------------
    // Step 1: CC:Tweaked availability
    // ---------------------------------------------------------------
    let cc_ok = peripheral::is_mod_available("computercraft");
    rc::println!("[1] CC:Tweaked available: {}", cc_ok);
    if !cc_ok {
        rc::println!("ERROR: CC:Tweaked is not loaded. Aborting.");
        return;
    }

    // ---------------------------------------------------------------
    // Step 2: Find a monitor.
    //
    // `peripheral::find_imm::<Monitor>()` scans all 6 adjacent directions
    // synchronously (no async, no tick needed) and returns a Vec of all
    // found Monitor peripherals.
    //
    // v0.1.x の `Monitor::new(dir)` は v0.2.0+ では廃止されました。
    // `Monitor::new(dir)` from v0.1.x is removed in v0.2.0+.
    // ---------------------------------------------------------------
    rc::println!("[2] Searching for a CC Monitor (peripheral::find_imm)...");
    let monitors = peripheral::find_imm::<Monitor>();
    if monitors.is_empty() {
        rc::println!("ERROR: No CC:Tweaked Monitor found adjacent to this computer.");
        rc::println!("  -> Place a monitor next to the computer and try again.");
        return;
    }
    let mut mon = monitors.into_iter().next().unwrap();
    rc::println!("  Monitor found.");

    // ---------------------------------------------------------------
    // Step 3: Immediate queries (no tick needed).
    //
    // `_imm()` variants execute synchronously via `host_request_info_imm`.
    // Use these for read-only "getXxx" calls that don't modify the world.
    // ---------------------------------------------------------------
    rc::println!("[3] Immediate queries (get_size_imm, get_text_scale_imm)...");
    let (width, height) = match mon.get_size_imm() {
        Ok(sz) => {
            rc::println!("  Size: {}x{}", sz.x, sz.y);
            (sz.x, sz.y)
        }
        Err(e) => {
            rc::println!("  get_size_imm() failed: {:?}", e);
            (26, 10) // fallback
        }
    };
    match mon.get_text_scale_imm() {
        Ok(s)  => rc::println!("  Text scale: {}", s.0),
        Err(e) => rc::println!("  get_text_scale_imm() failed: {:?}", e),
    }

    // ---------------------------------------------------------------
    // Step 4: Set text scale to 1.0.
    //
    // Action methods MUST be followed by wait_for_next_tick().await.
    // Before that call, the action is only "booked" (queued), not sent.
    // ---------------------------------------------------------------
    rc::println!("[4] Setting text scale to 1.0 (book + tick + read)...");
    mon.book_next_set_text_scale(MonitorTextScale::SIZE_1_0);
    rc::wait_for_next_tick().await;
    for r in mon.read_last_set_text_scale() {
        if let Err(e) = r {
            rc::println!("  set_text_scale warning: {:?}", e);
        }
    }

    // ---------------------------------------------------------------
    // Step 5: Basic write test — batch multiple ops in ONE tick.
    //
    // 全部の book_next_*() を1回の wait_for_next_tick().await でフラッシュ。
    // All book_next_*() calls are flushed together in a single tick.
    // ---------------------------------------------------------------
    rc::println!("[5] Basic write test (batch in 1 tick)...");

    // 背景を黒にして画面クリア / Set background black and clear screen
    mon.book_next_set_background_color(MonitorColor::BLACK);
    mon.book_next_clear();

    // ヘッダー行 (lime) / Header row (lime)
    mon.book_next_set_text_color(MonitorColor::LIME);
    mon.book_next_set_cursor_pos(MonitorPosition { x: 1, y: 1 });
    mon.book_next_write("=== RustComputers Monitor Test ===");

    // 情報行 (light blue) / Info row (light blue)
    mon.book_next_set_text_color(MonitorColor::LIGHT_BLUE);
    mon.book_next_set_cursor_pos(MonitorPosition { x: 1, y: 2 });
    mon.book_next_write(&format!("Size: {}x{}", width, height));

    // Hello 行 (white) / Hello row (white)
    mon.book_next_set_text_color(MonitorColor::WHITE);
    mon.book_next_set_cursor_pos(MonitorPosition { x: 1, y: 3 });
    mon.book_next_write("Hello from Rust! (v0.2.0+ API)");

    // フラッシュ (1ティック) / Flush (1 tick)
    rc::wait_for_next_tick().await;

    // 結果確認 / Check results
    let write_results = mon.read_last_write();
    let ok_count  = write_results.iter().filter(|r| r.is_ok()).count();
    let err_count = write_results.iter().filter(|r| r.is_err()).count();
    rc::println!("  write() results: {} ok, {} errors", ok_count, err_count);
    if err_count > 0 {
        rc::println!("  FAIL: write calls returned errors!");
        for r in write_results.into_iter().filter(|r| r.is_err()) {
            rc::println!("    {:?}", r.unwrap_err());
        }
    } else {
        rc::println!("  PASS: all writes succeeded.");
    }

    // ---------------------------------------------------------------
    // Step 6: Color bar test
    // ---------------------------------------------------------------
    rc::println!("[6] Color bar test...");

    // MonitorColor 定数は CC bitmask 値 (1, 2, 4, ..., 32768)。
    // MonitorColor constants are CC bitmask values (1, 2, 4, ..., 32768).
    let colors: &[(MonitorColor, &str)] = &[
        (MonitorColor::WHITE,      "WH"),
        (MonitorColor::ORANGE,     "OR"),
        (MonitorColor::MAGENTA,    "MG"),
        (MonitorColor::LIGHT_BLUE, "LB"),
        (MonitorColor::YELLOW,     "YL"),
        (MonitorColor::LIME,       "LM"),
        (MonitorColor::PINK,       "PK"),
        (MonitorColor::CYAN,       "CY"),
    ];
    mon.book_next_set_background_color(MonitorColor::BLACK);
    mon.book_next_set_text_color(MonitorColor::WHITE);
    mon.book_next_set_cursor_pos(MonitorPosition { x: 1, y: 5 });
    mon.book_next_write("Colors:");
    for (i, (color, label)) in colors.iter().enumerate() {
        mon.book_next_set_text_color(*color);
        mon.book_next_set_cursor_pos(MonitorPosition { x: 9 + (i as u32) * 3, y: 5 });
        mon.book_next_write(label);
    }
    mon.book_next_set_text_color(MonitorColor::WHITE);
    rc::wait_for_next_tick().await;
    rc::println!("  Color bar draw done.");

    // ---------------------------------------------------------------
    // Step 7: Scroll test
    // ---------------------------------------------------------------
    rc::println!("[7] Scroll test (write 5 lines, then scroll up 3)...");
    for i in 0u32..5 {
        mon.book_next_set_text_color(if i % 2 == 0 { MonitorColor::YELLOW } else { MonitorColor::CYAN });
        mon.book_next_set_cursor_pos(MonitorPosition { x: 1, y: 7 + i });
        mon.book_next_write(&format!("--- scroll line {} ---", i + 1));
    }
    rc::wait_for_next_tick().await;

    mon.book_next_scroll(3);
    rc::wait_for_next_tick().await;
    rc::println!("  Scroll done.");

    // ---------------------------------------------------------------
    // Step 8: getCursorPos via book / tick / read
    // ---------------------------------------------------------------
    rc::println!("[8] getCursorPos via book/tick/read...");
    mon.book_next_get_cursor_pos();
    rc::wait_for_next_tick().await;
    match mon.read_last_get_cursor_pos() {
        Ok(pos) => rc::println!("  Cursor pos after scroll: ({}, {})", pos.x, pos.y),
        Err(e)  => rc::println!("  getCursorPos failed: {:?}", e),
    }

    // ---------------------------------------------------------------
    // Step 9: Completion line
    // ---------------------------------------------------------------
    mon.book_next_set_text_color(MonitorColor::LIME);
    mon.book_next_set_cursor_pos(MonitorPosition { x: 1, y: height });
    mon.book_next_write("TEST COMPLETE");
    mon.book_next_set_text_color(MonitorColor::WHITE);
    rc::wait_for_next_tick().await;

    rc::println!("");
    rc::println!("╔══════════════════════════════════════════╗");
    rc::println!("║   Monitor test COMPLETE                  ║");
    rc::println!("╚══════════════════════════════════════════╝");
    rc::println!("Check the in-game monitor for results.");
    rc::println!("(Looping — Ctrl+C or /rc stop to halt)");

    loop {
        rc::wait_for_next_tick().await;
    }

}
