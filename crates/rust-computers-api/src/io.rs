//! 入出力ユーティリティ — ログ出力マクロと標準入力。
//! I/O utilities — log output macros and standard input.
//!
//! `println!` / `eprintln!` マクロはホスト関数 `host_log` を呼び出して
//! GUI のログ欄に出力する。`read_line()` は `host_stdin_read_line` を使い、
//! Enter が押されるまで非同期で待機する。
//!
//! `println!` / `eprintln!` macros call `host_log` to output to the GUI log panel.
//! `read_line()` uses `host_stdin_read_line` and asynchronously waits until Enter is pressed.

use alloc::string::String;

use crate::ffi;
use crate::future::RequestFuture;

// ==================================================================
// ログ出力 / Logging
// ==================================================================

/// 内部: 文字列をホスト関数に送信する。
/// Internal: send a string to the host function.
#[inline]
pub fn log_str(s: &str) {
    let bytes = s.as_bytes();
    unsafe {
        ffi::host_log(bytes.as_ptr() as i32, bytes.len() as i32);
    }
}

/// 内部: `core::fmt::Arguments` をフォーマットして送信する。
/// Internal: format `core::fmt::Arguments` and send to host.
#[inline]
pub fn log_fmt(args: core::fmt::Arguments<'_>) {
    let s = alloc::format!("{}", args);
    log_str(&s);
}

/// ログ出力マクロ（改行付き）。GUI のログ欄に表示される。
/// Log output macro (with newline). Displayed in the GUI log panel.
///
/// # 使い方 / Usage
/// ```rust,no_run
/// rc::println!("Hello, {}!", name);
/// ```
#[macro_export]
macro_rules! println {
    () => {
        $crate::io::log_str("")
    };
    ($($arg:tt)*) => {
        $crate::io::log_fmt(format_args!($($arg)*))
    };
}

/// エラーログ出力マクロ（改行付き）。`println!` と同じ出力先。
/// Error log output macro (with newline). Same destination as `println!`.
///
/// CC:Tweaked に stderr はないが、将来の拡張用に分離しておく。
/// CC:Tweaked has no stderr, but kept separate for future extensibility.
#[macro_export]
macro_rules! eprintln {
    () => {
        $crate::io::log_str("")
    };
    ($($arg:tt)*) => {
        $crate::io::log_fmt(format_args!($($arg)*))
    };
}

// ==================================================================
// 標準入力 / Standard input
// ==================================================================
/// 1 行の入力を非同期で読み取る。
/// Asynchronously read one line of input.
///
/// GUI の入力欄で Enter が押されるまで `Pending` を返す。
/// WASM が `read_line().await` していない間の入力は破棄される。
///
/// Returns `Pending` until Enter is pressed in the GUI input field.
/// Input is discarded while WASM is not awaiting `read_line()`.
///
/// # 使い方 / Usage
/// ```rust,no_run
/// let line = rc::read_line().await;
/// rc::println!("You typed: {}", line);
/// ```
///
/// # 戻り値 / Return value
/// 入力された文字列（UTF-8）。エラー時は空文字列。
/// The input string (UTF-8). Empty string on error.
pub async fn read_line() -> String {
    // 2 フェーズ取得方式：リクエスト時にはバッファ不要。
    // 結果サイズ判明後に Rust 側で動的確保する。
    // Two-phase fetch: no buffer needed at request time.
    // Rust allocates dynamically once the result size is known.
    let request_id = unsafe { ffi::host_stdin_read_line() };

    if request_id <= 0 {
        // エラーの場合 / On error
        return String::new();
    }

    // RequestFuture で完了を待つ / Wait for completion via RequestFuture
    let future = RequestFuture::new(request_id);
    match future.await {
        Ok(data) => {
            // UTF-8 文字列としてデコード / Decode as UTF-8 string
            String::from_utf8(data).unwrap_or_default()
        }
        Err(_) => String::new(),
    }
}

// ==================================================================
// メタ情報 / Meta information
// ==================================================================

/// このコンピューターの ID を取得する。
/// Get the ID of this computer.
///
/// # 使い方 / Usage
/// ```rust,no_run
/// let id = rc::computer_id();
/// rc::println!("Computer ID: {}", id);
/// ```
pub fn computer_id() -> i32 {
    unsafe { ffi::host_get_computer_id() }
}

/// 指定 Mod が利用可能か確認する。
/// Check whether the specified mod is available.
///
/// # 使い方 / Usage
/// ```rust,no_run
/// if rc::is_mod_available(0x01) {
///     rc::println!("CC: Tweaked is available!");
/// }
/// ```
pub fn is_mod_available(mod_id: u16) -> bool {
    unsafe { ffi::host_is_mod_available(mod_id) == 1 }
}
