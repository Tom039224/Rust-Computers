//! パニックハンドラ。
//! Panic handler for `no_std` environment.
//!
//! パニック発生時にホスト関数 `host_log` でメッセージを出力し、
//! WASM 実行を中断する（unreachable 命令）。
//!
//! On panic, outputs a message via `host_log` and aborts
//! WASM execution (unreachable instruction).

use core::panic::PanicInfo;

use alloc::format;

/// パニックハンドラ — `no_std` 環境で必須。
/// Panic handler — required in `no_std` environment.
///
/// Java 側は `wasm_tick()` の例外キャッチで CRASHED 状態へ遷移する。
/// The Java side catches the exception from `wasm_tick()` and transitions to CRASHED state.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // パニックメッセージをログに出力 / Log the panic message
    let msg = format!("[PANIC] {}", info);
    let bytes = msg.as_bytes();
    unsafe {
        crate::ffi::host_log(bytes.as_ptr() as i32, bytes.len() as i32);
    }

    // WASM の unreachable 命令で中断（Java 側で例外としてキャッチされる）
    // Abort via WASM unreachable instruction (caught as exception on Java side)
    core::arch::wasm32::unreachable()
}
