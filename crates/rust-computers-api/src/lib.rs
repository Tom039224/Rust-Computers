//! # rust-computers-api
//!
//! RustComputers の Rust 側コアランタイムクレート。
//! Core runtime crate for the Rust side of RustComputers.
//!
//! ## 概要 / Overview
//!
//! Minecraft の WASM コンピューターブロック上で動作する Rust プログラムのための
//! ランタイムライブラリ。`no_std` + `alloc` 環境で async/await を使用可能にする。
//!
//! Runtime library for Rust programs running on Minecraft WASM computer blocks.
//! Enables async/await in a `no_std` + `alloc` environment.
//!
//! ## 使い方 / Usage
//!
//! ```rust,no_run
//! #![no_std]
//! #![no_main]
//! extern crate alloc;
//!
//! use rust_computers_api as rc;
//!
//! rc::entry!(main);
//!
//! async fn main() {
//!     rc::println!("Hello from Rust Computer!");
//!     let line = rc::read_line().await;
//!     rc::println!("You typed: {}", line);
//! }
//! ```

#![no_std]
#![allow(async_fn_in_trait)]

extern crate alloc;

// モジュール宣言 / Module declarations
pub mod allocator;
pub mod book_store;
pub mod error;
pub mod executor;
pub mod ffi;
pub mod future;
pub mod io;
pub mod msgpack;
pub mod panic;
pub mod peripheral;

// 自動生成ペリフェラルラッパー → 手書きモジュールに変更
// Auto-generated peripheral wrappers → hand-written modules
pub mod serde_msgpack;

/// CC:Tweaked ペリフェラル群 / CC:Tweaked peripherals.
pub mod computer_craft;
/// CC-VS ペリフェラル群 / CC-VS peripherals.
pub mod cc_vs;
/// Some-Peripherals ペリフェラル群 / Some-Peripherals peripherals.
pub mod some_peripherals;
/// Toms-Peripherals ペリフェラル群 / Toms-Peripherals peripherals.
pub mod toms_peripherals;
/// Clockwork CC Compat ペリフェラル群 / Clockwork CC Compat peripherals.
pub mod clockwork_cc_compat;
/// Create ペリフェラル群 / Create peripherals.
pub mod create;
/// Create Additions ペリフェラル群 / Create Additions peripherals.
pub mod createaddition;
/// Control-Craft ペリフェラル群 / Control-Craft peripherals.
pub mod control_craft;
/// AdvancedPeripherals ペリフェラル群 / AdvancedPeripherals peripherals.
pub mod advanced_peripherals;
/// CBC CC Control ペリフェラル群 / CBC CC Control peripherals.
pub mod cbc_cc_control;

// 再エクスポート / Re-exports
pub use error::BridgeError;
pub use error::PeripheralError;
pub use future::RequestFuture;
pub use future::WaitForNextTickFuture;
pub use io::read_line;
pub use msgpack::Value;
pub use peripheral::Direction;

/// 次の Game Tick まで待機する Future を返す。
/// Returns a future that waits until the next game tick.
///
/// `book_next_*()` で予約した全リクエストが FFI 経由で一括発行され、
/// 結果が揃うまで yield する。
///
/// All requests booked via `book_next_*()` are flushed via FFI in batch,
/// and execution yields until all results are ready.
///
/// # 使い方 / Usage
/// ```rust,no_run
/// loop {
///     let data = sensor.read_last_get_data();
///     sensor.book_next_get_data();
///     wait_for_next_tick().await;
/// }
/// ```
pub fn wait_for_next_tick() -> WaitForNextTickFuture {
    WaitForNextTickFuture::new()
}

/// エントリーポイントマクロ。
/// Entry-point macro that generates `wasm_init()` and `wasm_tick()` exports.
///
/// # 使い方 / Usage
///
/// ```rust,no_run
/// rc::entry!(main);
///
/// async fn main() {
///     rc::println!("Hello!");
/// }
/// ```
///
/// # 展開後 / Expands to
///
/// ```rust,ignore
/// #[no_mangle]
/// pub extern "C" fn wasm_init() { /* spawn main */ }
/// #[no_mangle]
/// pub extern "C" fn wasm_tick() -> i32 { /* poll executor */ }
/// ```
#[macro_export]
macro_rules! entry {
    ($main_fn:ident) => {
        /// WASM エクスポート: main を executor に登録する。
        /// WASM export: register main with the executor.
        #[no_mangle]
        pub extern "C" fn wasm_init() {
            $crate::executor::init($main_fn());
        }

        /// WASM エクスポート: executor を 1 回 poll する。
        /// WASM export: poll the executor once.
        ///
        /// 戻り値 / Return value:
        /// -  `1` = 継続（Pending） / continue (Pending)
        /// -  `0` = main 正常終了 / main finished normally
        /// - `-1` = panic 発生 / panic occurred
        #[no_mangle]
        pub extern "C" fn wasm_tick() -> i32 {
            $crate::executor::tick()
        }
    };
}

/// 複数の Future を並行して poll する Future を返すマクロ。
/// Macro that returns a Future which polls multiple futures concurrently.
///
/// 2〜4 個の **異なる型** の Future をカンマ区切りで渡す。
/// 返り値は **Future** であり、`.await` で結果のタプルを取得する。
/// どの Future も同一 tick で発行されるため、1 tick 待つだけで全て取得できる。
///
/// Pass 2–4 futures of **potentially different types**, separated by commas.
/// Returns a **Future** that resolves to a tuple of results.
/// All futures are issued in the same tick, so only one tick is consumed.
///
/// ```rust,no_run
/// let (a, b) = rc::parallel!(
///     radar.scan(64.0),
///     sensor.get_temp(),
/// ).await;
/// ```
///
/// ```rust,no_run
/// let (size, is_adv, scale) = rc::parallel!(
///     mon.get_size(),
///     mon.is_advanced(),
///     mon.get_text_scale(),
/// ).await;
/// ```
#[macro_export]
macro_rules! parallel {
    ($a:expr, $b:expr $(,)?) => {
        $crate::future::Join2::new($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)?) => {
        $crate::future::Join3::new($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr $(,)?) => {
        $crate::future::Join4::new($a, $b, $c, $d)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr $(,)?) => {
        $crate::future::Join5::new($a, $b, $c, $d, $e)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr $(,)?) => {
        $crate::future::Join6::new($a, $b, $c, $d, $e, $f)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr $(,)?) => {
        $crate::future::Join7::new($a, $b, $c, $d, $e, $f, $g)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr $(,)?) => {
        $crate::future::Join8::new($a, $b, $c, $d, $e, $f, $g, $h)
    };
}
