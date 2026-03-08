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
//! book_next_*() [request] →  pending_requests (BTreeMap, 上書き)
//! book_next_*() [action]  →  pending_actions  (Vec,      追記)
//!                                ↓  flush()
//! FFI calls               →  in_flight (Vec, is_action フラグ付き)
//!                                ↓  poll_all()
//! read_last_*()  [request] ←  results        (BTreeMap<key, Result>)
//! read_last_*()  [action]  ←  action_results (BTreeMap<key, Vec<Result>>)
//! ```
//!
//! - **取得系 (request)**: 同 tick 内で同じメソッドを複数回予約すると最後が残る（上書き）。
//! - **反映系 (action)**: 同 tick 内で同じメソッドを複数回予約すると全て発行される（追記）。
//!
//! - **Request**: booking the same method multiple times per tick keeps only the last (overwrite).
//! - **Action**: booking the same method multiple times per tick issues all of them (accumulate).
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

/// 送信中リクエスト（FFI 発行済み、結果待ち）。
/// An in-flight request (FFI issued, awaiting result).
struct InFlightRequest {
    key: (u32, u32),
    request_id: i64,
    result_buf: Vec<u8>,
    written_bytes_buf: i32,
    is_action: bool,
}

/// BookStore 内部状態。
/// Internal BookStore state.
struct BookStoreInner {
    /// 未送信の情報リクエスト予約: (periph_id, method_id) → args（上書き）
    /// Pending info requests: (periph_id, method_id) → args (overwrite).
    pending_requests: BTreeMap<(u32, u32), Vec<u8>>,
    /// 未送信のアクション予約: (periph_id, method_id, args)（追記・順序保持）
    /// Pending action requests: (periph_id, method_id, args) (accumulate, ordered).
    pending_actions: Vec<(u32, u32, Vec<u8>)>,
    /// 送信済み・結果待ち
    /// In-flight requests awaiting results.
    in_flight: Vec<InFlightRequest>,
    /// 確定した情報リクエスト結果: (periph_id, method_id) → Result（最後1件のみ保持）
    /// Completed request results: (periph_id, method_id) → Result (single, newest).
    results: BTreeMap<(u32, u32), Result<Vec<u8>, BridgeError>>,
    /// 確定したアクション結果: (periph_id, method_id) → Vec<Result>（全件蓄積）
    /// Completed action results: (periph_id, method_id) → Vec<Result> (all accumulated).
    action_results: BTreeMap<(u32, u32), Vec<Result<Vec<u8>, BridgeError>>>,
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
                pending_requests: BTreeMap::new(),
                pending_actions: Vec::new(),
                in_flight: Vec::new(),
                results: BTreeMap::new(),
                action_results: BTreeMap::new(),
            })
        }
    }
}

// ==================================================================
// 公開 API / Public API
// ==================================================================

/// 情報リクエストを予約する（同じ key の既存予約は上書き）。
/// Book an info request (overwrites any existing booking with the same key).
///
/// `book_request()` から呼ばれる（取得系メソッド専用）。
/// Called from `book_request()` (info/query methods only).
pub(crate) fn book_request(periph_id: u32, method_id: u32, args: Vec<u8>) {
    let inner = BOOK_STORE.inner();
    inner.pending_requests.insert((periph_id, method_id), args);
}

/// アクションリクエストを予約する（同じ key でも上書きせず追記）。
/// Book an action request (accumulates; does NOT overwrite same key).
///
/// `book_action()` から呼ばれる（反映系メソッド専用）。
/// Called from `book_action()` (state-changing action methods only).
pub(crate) fn book_action(periph_id: u32, method_id: u32, args: Vec<u8>) {
    let inner = BOOK_STORE.inner();
    inner.pending_actions.push((periph_id, method_id, args));
}

/// 前回の情報リクエスト結果を読み取る（読み取ったら削除）。
/// Read the last info-request result (consumes it from the store).
///
/// `read_last_*()` メソッド（取得系）から呼ばれる。
/// Called from `read_last_*()` methods (info/query type).
pub(crate) fn read_result(periph_id: u32, method_id: u32) -> Option<Result<Vec<u8>, BridgeError>> {
    let inner = BOOK_STORE.inner();
    inner.results.remove(&(periph_id, method_id))
}

/// アクション結果を全件読み取る（読み取ったら削除）。
/// Read all accumulated action results (consumes them from the store).
///
/// `read_last_*()` メソッド（反映系）から呼ばれる。
/// Called from `read_last_*()` methods (action type).
pub(crate) fn read_action_results(periph_id: u32, method_id: u32) -> Vec<Result<Vec<u8>, BridgeError>> {
    let inner = BOOK_STORE.inner();
    inner.action_results.remove(&(periph_id, method_id)).unwrap_or_default()
}

/// 全予約リクエストを FFI 経由で発行する（pending → in_flight）。
/// Flush all pending requests via FFI (pending → in_flight).
///
/// `WaitForNextTickFuture::poll()` の初回呼び出しで実行される。
/// Executed on the first poll of `WaitForNextTickFuture::poll()`.
pub(crate) fn flush() {
    let inner = BOOK_STORE.inner();

    const RESULT_BUF_SIZE: usize = 4096;

    // 情報リクエスト（上書き済み最終値のみ発行）
    // Info requests (issue only the final value after overwrite)
    let pending_requests = core::mem::take(&mut inner.pending_requests);
    for ((periph_id, method_id), args) in pending_requests {
        let result_buf = vec![0u8; RESULT_BUF_SIZE];
        let request_id = unsafe {
            ffi::host_request_info(
                periph_id,
                method_id,
                args.as_ptr() as i32,
                args.len() as i32,
                result_buf.as_ptr() as i32,
                result_buf.len() as i32,
            )
        };
        inner.in_flight.push(InFlightRequest {
            key: (periph_id, method_id),
            request_id,
            result_buf,
            written_bytes_buf: 0,
            is_action: false,
        });
    }

    // アクションリクエスト（追記順に全て発行）
    // Action requests (issue all in the order they were booked)
    let pending_actions = core::mem::take(&mut inner.pending_actions);
    for (periph_id, method_id, args) in pending_actions {
        let result_buf = vec![0u8; RESULT_BUF_SIZE];
        let request_id = unsafe {
            ffi::host_do_action(
                periph_id,
                method_id,
                args.as_ptr() as i32,
                args.len() as i32,
                result_buf.as_ptr() as i32,
                result_buf.len() as i32,
            )
        };
        inner.in_flight.push(InFlightRequest {
            key: (periph_id, method_id),
            request_id,
            result_buf,
            written_bytes_buf: 0,
            is_action: true,
        });
    }
}

/// 全 in-flight リクエストをポーリングする。
/// Poll all in-flight requests.
///
/// 全て完了していれば結果を results / action_results に移動して true を返す。
/// Returns true when all requests are complete (results moved to the respective maps).
pub(crate) fn poll_all() -> bool {
    let inner = BOOK_STORE.inner();

    if inner.in_flight.is_empty() {
        return true;
    }

    let mut remaining: Vec<InFlightRequest> = Vec::new();

    // drain() が使えないため take して処理
    // Use take since we can't drain a Vec of non-Copy items directly
    let in_flight = core::mem::take(&mut inner.in_flight);

    for mut req in in_flight {
        let written_ptr = &mut req.written_bytes_buf as *mut i32 as i32;
        let status = unsafe { ffi::host_poll_result(req.request_id, written_ptr) };

        match status {
            0 => {
                // まだ未完了 / Still pending
                remaining.push(req);
            }
            1 => {
                // 完了 / Ready
                let written = req.written_bytes_buf as usize;
                let data = if written > 0 && written <= req.result_buf.len() {
                    req.result_buf[..written].to_vec()
                } else {
                    Vec::new()
                };
                if req.is_action {
                    inner.action_results
                        .entry(req.key)
                        .or_insert_with(Vec::new)
                        .push(Ok(data));
                } else {
                    inner.results.insert(req.key, Ok(data));
                }
            }
            err_code => {
                // エラー / Error
                let err = Err(BridgeError::from_code(err_code));
                if req.is_action {
                    inner.action_results
                        .entry(req.key)
                        .or_insert_with(Vec::new)
                        .push(err);
                } else {
                    inner.results.insert(req.key, err);
                }
            }
        }
    }

    inner.in_flight = remaining;
    inner.in_flight.is_empty()
}
