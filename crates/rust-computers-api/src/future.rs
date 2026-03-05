//! リクエスト Future — ホスト関数の非同期結果をポーリングする。
//! Request future — polls asynchronous host function results.
//!
//! `host_request_info` / `host_do_action` / `host_stdin_read_line` が返す
//! `request_id` を使い、毎 tick `host_poll_result` で完了を待つ。
//!
//! Uses `request_id` returned by `host_request_info` / `host_do_action` /
//! `host_stdin_read_line` and polls `host_poll_result` each tick until done.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use crate::error::BridgeError;
use crate::ffi;

/// ホスト関数の非同期結果を表す Future。
/// A future representing an asynchronous host function result.
///
/// `poll` 呼び出しごとに `host_poll_result` で完了をチェックする。
/// Checks completion via `host_poll_result` on each `poll` call.
pub struct RequestFuture {
    /// リクエスト ID / Request ID
    request_id: i64,

    /// 結果バッファ（Rust 側で事前確保） / Result buffer (pre-allocated by Rust)
    result_buf: Vec<u8>,

    /// host_poll_result が書き込みバイト数を格納する場所（4 バイト、LE）
    /// Location where host_poll_result writes the byte count (4 bytes, LE)
    written_bytes_buf: i32,

    /// 初回 poll フラグ / First poll flag
    first_poll: bool,
}

impl RequestFuture {
    /// 新しい RequestFuture を生成する。
    /// Create a new RequestFuture.
    ///
    /// # 引数 / Arguments
    /// - `request_id`: ホスト関数が返した request_id / request ID from host function
    /// - `result_buf`: 結果バッファ（既に確保済み） / result buffer (already allocated)
    pub fn new(request_id: i64, result_buf: Vec<u8>) -> Self {
        Self {
            request_id,
            result_buf,
            written_bytes_buf: 0,
            first_poll: true,
        }
    }

    /// 即時リクエスト（request_id ベースではない）の結果を返す。
    /// Return an immediate result (not request_id based).
    pub fn immediate(data: Vec<u8>) -> Self {
        Self {
            request_id: 0,
            result_buf: data,
            written_bytes_buf: 0,
            first_poll: false,
        }
    }
}

impl Future for RequestFuture {
    type Output = Result<Vec<u8>, BridgeError>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 安全性: WASM はシングルスレッド。Pin は構造的でないため get_unchecked_mut は安全。
        // Safety: WASM is single-threaded. Pin is not structural, so get_unchecked_mut is safe.
        let this = unsafe { self.get_unchecked_mut() };

        // 即時結果の場合 / Immediate result case
        if this.request_id == 0 && !this.first_poll {
            let data = core::mem::take(&mut this.result_buf);
            return Poll::Ready(Ok(data));
        }
        this.first_poll = false;

        // written_bytes_buf のアドレスを取得 / Get address of written_bytes_buf
        let written_ptr = &mut this.written_bytes_buf as *mut i32 as i32;

        // ポーリング / Poll
        let status = unsafe { ffi::host_poll_result(this.request_id, written_ptr) };

        match status {
            0 => Poll::Pending, // 未完了 / Not ready
            1 => {
                // 完了 — 書き込みバイト数を読み取る / Ready — read the written byte count
                let written = this.written_bytes_buf as usize;
                let data = if written > 0 && written <= this.result_buf.len() {
                    this.result_buf[..written].to_vec()
                } else {
                    Vec::new()
                };
                Poll::Ready(Ok(data))
            }
            err_code => {
                // エラー / Error
                Poll::Ready(Err(BridgeError::from_code(err_code)))
            }
        }
    }
}

// ==================================================================
// join! マクロ — 複数の Future を同時に poll する
// join! macro — poll multiple futures simultaneously
// ==================================================================

/// 2 つの Future を並行して poll する。
/// Poll two futures concurrently.
pub struct Join2<A: Future, B: Future> {
    a: Option<Pin<Box<A>>>,
    b: Option<Pin<Box<B>>>,
    result_a: Option<A::Output>,
    result_b: Option<B::Output>,
}

impl<A, B> Join2<A, B>
where
    A: Future,
    B: Future,
{
    /// 2 つの Future をラップする。
    /// Wrap two futures.
    pub fn new(a: A, b: B) -> Self {
        Self {
            a: Some(Box::pin(a)),
            b: Some(Box::pin(b)),
            result_a: None,
            result_b: None,
        }
    }
}

impl<A, B> Future for Join2<A, B>
where
    A: Future,
    B: Future,
{
    type Output = (A::Output, B::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        // Future A を poll / Poll future A
        if this.result_a.is_none() {
            if let Some(ref mut fut) = this.a {
                if let Poll::Ready(val) = fut.as_mut().poll(cx) {
                    this.result_a = Some(val);
                    this.a = None;
                }
            }
        }

        // Future B を poll / Poll future B
        if this.result_b.is_none() {
            if let Some(ref mut fut) = this.b {
                if let Poll::Ready(val) = fut.as_mut().poll(cx) {
                    this.result_b = Some(val);
                    this.b = None;
                }
            }
        }

        // 両方完了したか確認 / Check if both are done
        if this.result_a.is_some() && this.result_b.is_some() {
            Poll::Ready((this.result_a.take().unwrap(), this.result_b.take().unwrap()))
        } else {
            Poll::Pending
        }
    }
}

/// 3 つの Future を並行して poll する。
/// Poll three futures concurrently.
pub struct Join3<A: Future, B: Future, C: Future> {
    a: Option<Pin<Box<A>>>,
    b: Option<Pin<Box<B>>>,
    c: Option<Pin<Box<C>>>,
    result_a: Option<A::Output>,
    result_b: Option<B::Output>,
    result_c: Option<C::Output>,
}

impl<A, B, C> Join3<A, B, C>
where
    A: Future,
    B: Future,
    C: Future,
{
    pub fn new(a: A, b: B, c: C) -> Self {
        Self {
            a: Some(Box::pin(a)),
            b: Some(Box::pin(b)),
            c: Some(Box::pin(c)),
            result_a: None,
            result_b: None,
            result_c: None,
        }
    }
}

impl<A, B, C> Future for Join3<A, B, C>
where
    A: Future,
    B: Future,
    C: Future,
{
    type Output = (A::Output, B::Output, C::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        if this.result_a.is_none() {
            if let Some(ref mut fut) = this.a {
                if let Poll::Ready(val) = fut.as_mut().poll(cx) {
                    this.result_a = Some(val);
                    this.a = None;
                }
            }
        }
        if this.result_b.is_none() {
            if let Some(ref mut fut) = this.b {
                if let Poll::Ready(val) = fut.as_mut().poll(cx) {
                    this.result_b = Some(val);
                    this.b = None;
                }
            }
        }
        if this.result_c.is_none() {
            if let Some(ref mut fut) = this.c {
                if let Poll::Ready(val) = fut.as_mut().poll(cx) {
                    this.result_c = Some(val);
                    this.c = None;
                }
            }
        }

        if this.result_a.is_some() && this.result_b.is_some() && this.result_c.is_some() {
            Poll::Ready((
                this.result_a.take().unwrap(),
                this.result_b.take().unwrap(),
                this.result_c.take().unwrap(),
            ))
        } else {
            Poll::Pending
        }
    }
}

/// 内部マクロ: Join を構築する。
/// Internal macro: build a Join combinator.
#[macro_export]
macro_rules! join {
    ($a:expr, $b:expr $(,)?) => {
        $crate::future::Join2::new($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)?) => {
        $crate::future::Join3::new($a, $b, $c)
    };
}
