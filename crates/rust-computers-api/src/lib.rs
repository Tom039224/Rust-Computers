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

extern crate alloc;

// モジュール宣言 / Module declarations
pub mod allocator;
pub mod error;
pub mod executor;
pub mod ffi;
pub mod future;
pub mod io;
pub mod msgpack;
pub mod panic;
pub mod peripheral;

// 再エクスポート / Re-exports
pub use error::BridgeError;
pub use future::{JoinAll, RequestFuture};
pub use io::read_line;

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

/// 複数の Future を並行して待機するマクロ。
/// Macro to await multiple futures concurrently.
///
/// # タプル構文 / Tuple syntax
///
/// 2〜4 個の Future をタプル形式で渡し、結果をタプルで受け取る。
/// Pass 2–4 futures in tuple form and receive results as a tuple.
///
/// ```rust,no_run
/// let (a, b) = rc::parallel!((
///     radar.scan(64.0),
///     sensor.get_temp(),
/// ));
/// ```
///
/// # Vec 構文 / Vec syntax
///
/// 同一型の Future の Vec を渡し、結果を Vec で受け取る。
/// Pass a Vec of same-typed futures and receive results as a Vec.
///
/// ```rust,no_run
/// let results = rc::parallel!(futures_vec);
/// ```
#[macro_export]
macro_rules! parallel {
    // --- タプル構文 (2〜4 要素) / Tuple syntax (2–4 elements) ---
    (($a:expr, $b:expr $(,)?)) => {{
        $crate::future::Join2::new($a, $b).await
    }};
    (($a:expr, $b:expr, $c:expr $(,)?)) => {{
        $crate::future::Join3::new($a, $b, $c).await
    }};
    (($a:expr, $b:expr, $c:expr, $d:expr $(,)?)) => {{
        $crate::future::Join4::new($a, $b, $c, $d).await
    }};
    // --- Vec 構文 / Vec syntax ---
    ($vec:expr) => {{
        $crate::future::JoinAll::new($vec).await
    }};
}
