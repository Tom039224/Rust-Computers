//! CC:Tweaked Monitor async methods test.
//!
//! ## Build
//! ```sh
//! cargo build --example test_monitor_async --target wasm32-unknown-unknown --release
//! ```
//!
//! ## Setup
//! Place a CC:Tweaked Monitor adjacent to this computer, then run
//! `/rc load test_monitor_async` in game.
//!
//! ## What is tested
//! - All 16 async_* methods for Monitor
//! - async_set_text_scale / async_get_text_scale
//! - async_write / async_clear / async_clear_line
//! - async_scroll
//! - async_get_cursor_pos / async_set_cursor_pos
//! - async_get_cursor_blink / async_set_cursor_blink
//! - async_get_size
//! - async_set_text_color / async_get_text_color
//! - async_set_background_color / async_get_background_color
//! - async_blit

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
    rc::println!("║   CC:Tweaked Monitor Async Test          ║");
    rc::println!("║   Testing all 16 async_* methods         ║");
    rc::println!("╚══════════════════════════════════════════╝");

    // Check CC:Tweaked availability
    let cc_ok = peripheral::is_mod_available("computercraft");
    rc::println!("[1] CC:Tweaked available: {}", cc_ok);
    if !cc_ok {
        rc::println!("ERROR: CC:Tweaked is not loaded. Aborting.");
        return;
    }

    // Find a monitor
    rc::println!("[2] Searching for a CC Monitor...");
    let monitors = peripheral::find_imm::<Monitor>();
    if monitors.is_empty() {
        rc::println!("ERROR: No CC:Tweaked Monitor found adjacent to this computer.");
        return;
    }
    let mut mon = monitors.into_iter().next().unwrap();
    rc::println!("  Monitor found.");

    // Test async_get_size
    rc::println!("[3] Testing async_get_size()...");
    match mon.async_get_size().await {
        Ok(sz) => rc::println!("  Size: {}x{}", sz.x, sz.y),
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }

    // Test async_set_text_scale and async_get_text_scale
    rc::println!("[4] Testing async_set_text_scale(1.0) and async_get_text_scale()...");
    match mon.async_set_text_scale(MonitorTextScale::SIZE_1_0).await {
        Ok(_) => rc::println!("  Set text scale: OK"),
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }
    match mon.async_get_text_scale().await {
        Ok(scale) => rc::println!("  Current text scale: {}", scale.0),
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }

    // Test async_set_background_color and async_clear
    rc::println!("[5] Testing async_set_background_color(BLACK) and async_clear()...");
    match mon.async_set_background_color(MonitorColor::BLACK).await {
        Ok(_) => rc::println!("  Set background color: OK"),
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }
    match mon.async_clear().await {
        Ok(_) => rc::println!("  Clear screen: OK"),
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }

    // Test async_set_text_color and async_get_text_color
    rc::println!("[6] Testing async_set_text_color(LIME) and async_get_text_color()...");
    match mon.async_set_text_color(MonitorColor::LIME).await {
        Ok(_) => rc::println!("  Set text color: OK"),
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }
    match mon.async_get_text_color().await {
        Ok(color) => rc::println!("  Current text color: {}", color.0),
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }

    // Test async_set_cursor_pos and async_get_cursor_pos
    rc::println!("[7] Testing async_set_cursor_pos(1,1) and async_get_cursor_pos()...");
    match mon.async_set_cursor_pos(MonitorPosition { x: 1, y: 1 }).await {
        Ok(_) => rc::println!("  Set cursor pos: OK"),
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }
    match mon.async_get_cursor_pos().await {
        Ok(pos) => rc::println!("  Current cursor pos: ({}, {})", pos.x, pos.y),
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }

    // Test async_write
    rc::println!("[8] Testing async_write()...");
    match mon.async_write("=== Async Test ===").await {
        Ok(_) => rc::println!("  Write: OK"),
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }

    // Test async_set_cursor_blink and async_get_cursor_blink
    rc::println!("[9] Testing async_set_cursor_blink(true) and async_get_cursor_blink()...");
    match mon.async_set_cursor_blink(true).await {
        Ok(_) => rc::println!("  Set cursor blink: OK"),
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }
    match mon.async_get_cursor_blink().await {
        Ok(blink) => rc::println!("  Current cursor blink: {}", blink),
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }

    // Test async_clear_line
    rc::println!("[10] Testing async_clear_line()...");
    match mon.async_set_cursor_pos(MonitorPosition { x: 1, y: 2 }).await {
        Ok(_) => {},
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }
    match mon.async_write("This line will be cleared").await {
        Ok(_) => {},
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }
    match mon.async_clear_line().await {
        Ok(_) => rc::println!("  Clear line: OK"),
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }

    // Test async_scroll
    rc::println!("[11] Testing async_scroll(1)...");
    match mon.async_scroll(1).await {
        Ok(_) => rc::println!("  Scroll: OK"),
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }

    // Test async_blit
    rc::println!("[12] Testing async_blit()...");
    match mon.async_set_cursor_pos(MonitorPosition { x: 1, y: 3 }).await {
        Ok(_) => {},
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }
    match mon.async_blit("BLIT TEST", MonitorColor::YELLOW, MonitorColor::BLUE).await {
        Ok(_) => rc::println!("  Blit: OK"),
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }

    // Test async_get_background_color
    rc::println!("[13] Testing async_get_background_color()...");
    match mon.async_get_background_color().await {
        Ok(color) => rc::println!("  Current background color: {}", color.0),
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }

    // Completion message
    match mon.async_set_cursor_pos(MonitorPosition { x: 1, y: 5 }).await {
        Ok(_) => {},
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }
    match mon.async_set_text_color(MonitorColor::LIME).await {
        Ok(_) => {},
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }
    match mon.async_write("ALL ASYNC TESTS COMPLETE").await {
        Ok(_) => {},
        Err(e) => rc::println!("  ERROR: {:?}", e),
    }

    rc::println!("");
    rc::println!("╔══════════════════════════════════════════╗");
    rc::println!("║   All 16 async_* methods tested          ║");
    rc::println!("╚══════════════════════════════════════════╝");
    rc::println!("Check the in-game monitor for results.");

    loop {
        rc::wait_for_next_tick().await;
    }
}
