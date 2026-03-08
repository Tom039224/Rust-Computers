//! book-read パターンの状態管理。
//! State management for the book-read pattern.
//!
//! ## 概要 / Overview
//!
//! `book_next_*()` で予約されたリクエストを一括管理し、
//! `wait_for_next_tick()` で FFI 経由で発行・結果回収する。
//!
//! Manages requests booked by `book_next_*()` in bulk and
//! flushes/collects results via FFI during `wait_for_next_tick()`.
//!
//! ## データフロー / Data Flow
//!
//! ```text
//! book_next_*()  →  pending (BTreeMap)
//!                      ↓  flush()
//! FFI calls      →  in_flight (Vec)
//!                      ↓  poll_all()
//! read_last_*()  ←  results (BTreeMap)
//! ```
//!
//! ## スレッド安全性 / Thread Safety
//!
//! WASM はシングルスレッド実行のため `UnsafeCell` で安全。
//! `UnsafeCell` is safe because WASM runs in a single thread.

use alloc::vec;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use core::cell::UnsafeCell;

use crate::error::BridgeError;
use crate::ffi;

// ==================================================================
// BookStore 構造体 / BookStore structs
// ==================================================================

/// 予約済みリクエスト。
/// A booked request waiting to be flushed.
pub(crate) struct BookedRequest {
    pub args: Vec<u8>,
    pub is_action: bool,
}

/// 送信中リクエスト（FFI 発行済み、結果待ち）。
/// An in-flight request (FFI issued, awaiting result).
struct InFlightRequest {
    request_id: i64,
    result_buf: Vec<u8>,
    written_bytes_buf: i32,
}

/// BookStore 内部状態。
/// Internal BookStore state.
struct BookStoreInner {
    /// 未送信の予約: (periph_id, method_id) → BookedRequest
    /// Pending books: (periph_id, method_id) → BookedRequest
    pending: BTreeMap<(u32, u32), BookedRequest>,
    /// 送信済み・結果待ち
    /// In-flight requests awaiting results
    in_flight: Vec<((u32, u32), InFlightRequest)>,
    /// 確定結果: (periph_id, method_id) → Result
    /// Completed results: (periph_id, method_id) → Result
    results: BTreeMap<(u32, u32), Result<Vec<u8>, BridgeError>>,
}

/// グローバル BookStore ラッパー。
/// Global BookStore wrapper.
struct BookStore {
    inner: UnsafeCell<Option<BookStoreInner>>,
}

// 安全性: WASM は常にシングルスレッドで実行される。
// Safety: WASM always runs in a single thread.
unsafe impl Sync for BookStore {}

/// グローバル BookStore インスタンス。
/// Global BookStore instance.
static BOOK_STORE: BookStore = BookStore {
    inner: UnsafeCell::new(None),
};

impl BookStore {
    /// 内部状態への参照を取得（未初期化なら自動初期化）。
    /// Get a reference to the inner state (auto-initializes if needed).
    fn inner(&self) -> &mut BookStoreInner {
        unsafe {
            let opt = &mut *self.inner.get();
            opt.get_or_insert_with(|| BookStoreInner {
                pending: BTreeMap::new(),
                in_flight: Vec::new(),
                results: BTreeMap::new(),
            })
        }
    }
}

// ==================================================================
// 公開 API / Public API
// ==================================================================

/// リクエストを予約する（同じ key の既存予約は上書き）。
/// Book a request (overwrites any existing booking with the same key).
///
/// `book_next_*()` メソッドから呼ばれる。
/// Called from `book_next_*()` methods.
pub(crate) fn book(periph_id: u32, method_id: u32, args: Vec<u8>, is_action: bool) {
    let inner = BOOK_STORE.inner();
    inner.pending.insert(
        (periph_id, method_id),
        BookedRequest { args, is_action },
    );
}

/// 前回の結果を読み取る（読み取ったら削除）。
/// Read the last result (consumes it from the store).
///
/// `read_last_*()` メソッドから呼ばれる。
/// Called from `read_last_*()` methods.
pub(crate) fn read_result(periph_id: u32, method_id: u32) -> Option<Result<Vec<u8>, BridgeError>> {
    let inner = BOOK_STORE.inner();
    inner.results.remove(&(periph_id, method_id))
}

/// 全予約リクエストを FFI 経由で発行する（pending → in_flight）。
/// Flush all pending requests via FFI (pending → in_flight).
///
/// `WaitForNextTickFuture::poll()` の初回呼び出しで実行される。
/// Executed on the first poll of `WaitForNextTickFuture::poll()`.
pub(crate) fn flush() {
    let inner = BOOK_STORE.inner();

    const RESULT_BUF_SIZE: usize = 4096;

    // BTreeMap には drain() がないので take + iterate
    // BTreeMap doesn't have drain(), so take + iterate
    let pending = core::mem::take(&mut inner.pending);

    for ((periph_id, method_id), req) in pending {
        let result_buf = vec![0u8; RESULT_BUF_SIZE];

        let request_id = unsafe {
            if req.is_action {
                ffi::host_do_action(
                    periph_id,
                    method_id,
                    req.args.as_ptr() as i32,
                    req.args.len() as i32,
                    result_buf.as_ptr() as i32,
                    result_buf.len() as i32,
                )
            } else {
                ffi::host_request_info(
                    periph_id,
                    method_id,
                    req.args.as_ptr() as i32,
                    req.args.len() as i32,
                    result_buf.as_ptr() as i32,
                    result_buf.len() as i32,
                )
            }
        };

        inner.in_flight.push((
            (periph_id, method_id),
            InFlightRequest {
                request_id,
                result_buf,
                written_bytes_buf: 0,
            },
        ));
    }
}

/// 全 in-flight リクエストをポーリングする。
/// Poll all in-flight requests.
///
/// 全て完了していれば結果を results に移動して true を返す。
/// Returns true if all requests are complete (results moved to the results map).
pub(crate) fn poll_all() -> bool {
    let inner = BOOK_STORE.inner();

    if inner.in_flight.is_empty() {
        return true;
    }

    let mut completed: Vec<((u32, u32), Result<Vec<u8>, BridgeError>)> = Vec::new();
    let mut remaining: Vec<((u32, u32), InFlightRequest)> = Vec::new();

    for (key, mut req) in inner.in_flight.drain(..) {
        let written_ptr = &mut req.written_bytes_buf as *mut i32 as i32;
        let status = unsafe { ffi::host_poll_result(req.request_id, written_ptr) };

        match status {
            0 => {
                // まだ未完了 / Still pending
                remaining.push((key, req));
            }
            1 => {
                // 完了 / Ready
                let written = req.written_bytes_buf as usize;
                let data = if written > 0 && written <= req.result_buf.len() {
                    req.result_buf[..written].to_vec()
                } else {
                    Vec::new()
                };
                completed.push((key, Ok(data)));
            }
            err_code => {
                // エラー / Error
                completed.push((key, Err(BridgeError::from_code(err_code))));
            }
        }
    }

    // 完了した結果を results マップに格納
    // Store completed results in the results map
    for (key, result) in completed {
        inner.results.insert(key, result);
    }

    // 残りを in_flight に戻す
    // Put remaining back into in_flight
    inner.in_flight = remaining;

    inner.in_flight.is_empty()
}
