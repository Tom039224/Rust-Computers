//! CC:Tweaked Monitor書き込みテスト / Monitor write test.
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
//! - `is_mod_available("computercraft")` — CC availability check
//! - Auto-detect monitor in 6 directions (uses `Monitor::new(dir).clear()`)
//! - `parallel!` macro: fetch size + is_advanced + text_scale in 1 tick
//! - All drawing APIs via the auto-generated `Monitor` struct
//!   (written by build.rs from `peripherals/monitor.toml`)

#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;

use rust_computers_api as rc;
use rc::monitor::Monitor;
use rc::peripheral::{Direction, is_mod_available};

rc::entry!(main);

// CC:Tweaked color constants (bitmask form, same as colors.* in Lua)
const COLOR_WHITE:      i32 = 1;
const COLOR_ORANGE:     i32 = 2;
const COLOR_MAGENTA:    i32 = 4;
const COLOR_LIGHT_BLUE: i32 = 8;
const COLOR_YELLOW:     i32 = 16;
const COLOR_LIME:       i32 = 32;
const COLOR_PINK:       i32 = 64;
#[allow(dead_code)]
const COLOR_GRAY:       i32 = 128;
#[allow(dead_code)]
const COLOR_LIGHT_GRAY: i32 = 256;
const COLOR_CYAN:       i32 = 512;
#[allow(dead_code)]
const COLOR_PURPLE:     i32 = 1024;
#[allow(dead_code)]
const COLOR_BLUE:       i32 = 2048;
#[allow(dead_code)]
const COLOR_BROWN:      i32 = 4096;
#[allow(dead_code)]
const COLOR_GREEN:      i32 = 8192;
#[allow(dead_code)]
const COLOR_RED:        i32 = 16384;
const COLOR_BLACK:      i32 = 32768;

async fn main() {
    rc::println!("╔══════════════════════════════════════════╗");
    rc::println!("║   CC:Tweaked Monitor Write Test          ║");
    rc::println!("╚══════════════════════════════════════════╝");

    // ---------------------------------------------------------------
    // Step 1: CC:Tweaked availability
    // ---------------------------------------------------------------
    let cc_ok = is_mod_available("computercraft");
    rc::println!("[1] CC:Tweaked available: {}", cc_ok);
    if !cc_ok {
        rc::println!("ERROR: CC:Tweaked is not loaded. Aborting.");
        return;
    }

    // ---------------------------------------------------------------
    // Step 2: Scan 6 directions for a monitor.
    //
    // `clear()` is a do_action call (world-modifying) so it enforces
    // the 1-tick delay principle: if the peripheral isn't present the
    // result comes back as an error on the next tick.
    // ---------------------------------------------------------------
    rc::println!("[2] Scanning 6 directions for a CC Monitor...");
    let dirs: [(Direction, &str); 6] = [
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
        match Monitor::new(*dir).clear().await {
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
            rc::println!("ERROR: No CC:Tweaked Monitor found in any direction.");
            return;
        }
    };

    // All subsequent operations share this wrapper.
    let mon = Monitor::new(dir);

    // ---------------------------------------------------------------
    // Step 3: parallel! — fetch multiple info values in a single tick.
    //
    // All three Futures are dispatched at GT:N.
    // Results arrive at GT:N+1 — one tick total regardless of count.
    //
    // Syntax: rc::parallel!(future_a, future_b, future_c)
    //         returns (Result<A, _>, Result<B, _>, Result<C, _>)
    // ---------------------------------------------------------------
    rc::println!("[3] Parallel fetch: size + is_advanced + text_scale (1 tick)...");
    let (size_res, advanced_res, scale_res) = rc::parallel!(
        mon.get_size(),
        mon.is_advanced(),
        mon.get_text_scale(),
    );
    let (width, height) = size_res.unwrap_or((26, 10));
    let is_advanced      = advanced_res.unwrap_or(false);
    let _scale           = scale_res.unwrap_or(10);
    rc::println!("    size={}x{}  advanced={}  scale={}", width, height, is_advanced, _scale);

    // ---------------------------------------------------------------
    // Step 4: set text scale to 1.0 (internal unit: x10 = 10)
    // ---------------------------------------------------------------
    rc::println!("[4] Setting text scale to 1.0...");
    if let Err(e) = mon.set_text_scale(10).await {
        rc::println!("    setTextScale() warning: {:?}", e);
    }

    // ---------------------------------------------------------------
    // Step 5: basic write test
    // ---------------------------------------------------------------
    rc::println!("[5] Basic write test...");
    let _ = mon.set_background_color(COLOR_BLACK).await;
    let _ = mon.clear().await;

    // Header row — green
    let _ = mon.set_text_color(COLOR_LIME).await;
    let _ = mon.set_cursor_pos(1, 1).await;
    let _ = mon.write("=== RustComputers Monitor Test ===").await;

    // Info row — cyan
    let _ = mon.set_text_color(COLOR_LIGHT_BLUE).await;
    let _ = mon.set_cursor_pos(1, 2).await;
    let _ = mon.write(&format!("Size: {}x{}  Advanced: {}", width, height, is_advanced)).await;

    let _ = mon.set_cursor_pos(1, 3).await;
    let _ = mon.write("Hello from Rust!").await;

    // Computer ID row — white
    let _ = mon.set_text_color(COLOR_WHITE).await;
    let _ = mon.set_cursor_pos(1, 4).await;
    let _ = mon.write(&format!("Computer ID: {}", rc::io::computer_id())).await;

    rc::println!("    Basic write OK");

    // ---------------------------------------------------------------
    // Step 6: color bar test
    // ---------------------------------------------------------------
    rc::println!("[6] Color bar test...");
    let colors: &[(i32, &str)] = &[
        (COLOR_WHITE,      "WH"),
        (COLOR_ORANGE,     "OR"),
        (COLOR_MAGENTA,    "MG"),
        (COLOR_LIGHT_BLUE, "LB"),
        (COLOR_YELLOW,     "YL"),
        (COLOR_LIME,       "LM"),
        (COLOR_PINK,       "PK"),
        (COLOR_CYAN,       "CY"),
    ];
    let row = 6i32;
    let _ = mon.set_text_color(COLOR_WHITE).await;
    let _ = mon.set_cursor_pos(1, row).await;
    let _ = mon.write("Colors: ").await;
    for (i, (color, label)) in colors.iter().enumerate() {
        let _ = mon.set_text_color(*color).await;
        let _ = mon.set_background_color(COLOR_BLACK).await;
        let _ = mon.set_cursor_pos(9 + (i as i32) * 3, row).await;
        let _ = mon.write(label).await;
    }
    let _ = mon.set_background_color(COLOR_BLACK).await;
    let _ = mon.set_text_color(COLOR_WHITE).await;
    rc::println!("    Color test OK");

    // ---------------------------------------------------------------
    // Step 7: scroll test — write 5 lines then scroll up 3
    // ---------------------------------------------------------------
    rc::println!("[7] Scroll test (3 lines up)...");
    for i in 0..5i32 {
        let _ = mon.set_cursor_pos(1, height - i).await;
        let color = if i % 2 == 0 { COLOR_YELLOW } else { COLOR_CYAN };
        let _ = mon.set_text_color(color).await;
        let _ = mon.write(&format!("--- scroll line {} ---", i)).await;
    }
    let _ = mon.scroll(3).await;
    rc::println!("    Scroll test OK");

    // ---------------------------------------------------------------
    // Step 8: completion line
    // ---------------------------------------------------------------
    let _ = mon.set_text_color(COLOR_LIME).await;
    let _ = mon.set_cursor_pos(1, height).await;
    let _ = mon.write("TEST COMPLETE").await;
    let _ = mon.set_text_color(COLOR_WHITE).await;

    rc::println!("");
    rc::println!("╔══════════════════════════════════════════╗");
    rc::println!("║   Monitor test COMPLETE                  ║");
    rc::println!("╚══════════════════════════════════════════╝");
    rc::println!("Check the in-game monitor for results.");
}
