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

// ==================================================================
// WaitForNextTickFuture — book-read パターンの tick 境界
// WaitForNextTickFuture — tick boundary for the book-read pattern
// ==================================================================

/// 次の Game Tick まで待機する Future。
/// A future that waits until the next game tick.
///
/// ## 動作 / Behavior
///
/// 1. 初回 poll: 全予約リクエストを FFI 経由で発行 → `Poll::Pending`
/// 2. 以降の poll: 全 in-flight をポーリング → 全完了なら `Poll::Ready(())`
///
/// 1. First poll: flush all booked requests via FFI → `Poll::Pending`
/// 2. Subsequent polls: poll all in-flight → `Poll::Ready(())` when all done
pub struct WaitForNextTickFuture {
    first_poll: bool,
}

impl WaitForNextTickFuture {
    /// 新しい WaitForNextTickFuture を生成する。
    /// Create a new WaitForNextTickFuture.
    pub fn new() -> Self {
        Self { first_poll: true }
    }
}

impl Future for WaitForNextTickFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        let this = unsafe { self.get_unchecked_mut() };

        if this.first_poll {
            this.first_poll = false;
            // 全予約リクエストを FFI 経由で発行
            // Flush all booked requests via FFI
            crate::book_store::flush();
            return Poll::Pending;
        }

        // 全 in-flight リクエストをポーリング
        // Poll all in-flight requests
        if crate::book_store::poll_all() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

/// ホスト関数の非同期結果を表す Future。
/// A future representing an asynchronous host function result.
///
/// ## 1tick 遅れ強制 / 1-tick delay enforcement
///
/// `poll` の初回呼び出しでは必ず `Poll::Pending` を返す。
/// これにより、リクエスト発行 tick と結果取得 tick が必ず 1 tick 以上離れる。
/// (`spec.md` §1.1 「1tick 遅れ原則」の実装)
///
/// The first call to `poll` always returns `Poll::Pending`.
/// This guarantees at least one full tick between request issuance and result delivery.
/// (Implements the "1-tick delay principle" from spec.md §1.1)
pub struct RequestFuture {
    /// リクエスト ID / Request ID
    request_id: i64,

    /// 1tick 遅れ強制フラグ: true の内は必ず Pending を返す。
    /// 1-tick delay flag: returns Pending while true.
    first_poll: bool,
}

impl RequestFuture {
    /// 新しい RequestFuture を生成する。
    /// Create a new RequestFuture.
    ///
    /// # 引数 / Arguments
    /// - `request_id`: ホスト関数が返した request_id / request ID from host function
    pub fn new(request_id: i64) -> Self {
        Self {
            request_id,
            first_poll: true,
        }
    }
}

impl Future for RequestFuture {
    type Output = Result<Vec<u8>, BridgeError>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 安全性: WASM はシングルスレッド。Pin は構造的でないため get_unchecked_mut は安全。
        // Safety: WASM is single-threaded. Pin is not structural, so get_unchecked_mut is safe.
        let this = unsafe { self.get_unchecked_mut() };

        // 1tick 遅れ強制: リクエスト発行と同じ tick では必ず Pending を返す。
        // Java の Phase 1 (結果収集) は次 tick で行われるため、
        // 同 tick の host_poll_result 呼び出しは常に未完了になるはず。
        //
        // 1-tick delay enforcement: always return Pending in the tick the request was issued.
        // Java's Phase 1 (result collection) runs in the next tick;
        // calling host_poll_result in the same tick would always be Pending.
        if this.first_poll {
            this.first_poll = false;
            return Poll::Pending;
        }

        // フェーズ 1: 完了確認とサイズ取得
        // Phase 1: Check completion and get result size
        let status = unsafe { ffi::host_poll_result(this.request_id) };

        match status {
            0 => Poll::Pending, // 未完了 / Not ready
            size if size > 0 => {
                // 完了 — offset-by-1: actual_size = size - 1
                // Ready — offset-by-1: actual_size = size - 1
                let actual_size = (size as usize) - 1;
                if actual_size == 0 {
                    // 空ペイロード / Empty payload
                    Poll::Ready(Ok(alloc::vec![]))
                } else {
                    // フェーズ 2: 動的確保したバッファにデータを転送
                    // Phase 2: Fetch data into dynamically allocated buffer
                    let mut buf = alloc::vec![0u8; actual_size];
                    let written = unsafe {
                        ffi::host_fetch_result(
                            this.request_id,
                            buf.as_ptr() as i32,
                            actual_size as i32,
                        )
                    };
                    if written < 0 {
                        Poll::Ready(Err(BridgeError::from_code(written)))
                    } else {
                        buf.truncate(written as usize);
                        Poll::Ready(Ok(buf))
                    }
                }
            }
            err_code => {
                // エラー / Error
                Poll::Ready(Err(BridgeError::from_code(err_code as i32)))
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

/// 4 つの Future を並行して poll する。
/// Poll four futures concurrently.
pub struct Join4<A: Future, B: Future, C: Future, D: Future> {
    a: Option<Pin<Box<A>>>,
    b: Option<Pin<Box<B>>>,
    c: Option<Pin<Box<C>>>,
    d: Option<Pin<Box<D>>>,
    result_a: Option<A::Output>,
    result_b: Option<B::Output>,
    result_c: Option<C::Output>,
    result_d: Option<D::Output>,
}

impl<A, B, C, D> Join4<A, B, C, D>
where
    A: Future,
    B: Future,
    C: Future,
    D: Future,
{
    pub fn new(a: A, b: B, c: C, d: D) -> Self {
        Self {
            a: Some(Box::pin(a)),
            b: Some(Box::pin(b)),
            c: Some(Box::pin(c)),
            d: Some(Box::pin(d)),
            result_a: None,
            result_b: None,
            result_c: None,
            result_d: None,
        }
    }
}

impl<A, B, C, D> Future for Join4<A, B, C, D>
where
    A: Future,
    B: Future,
    C: Future,
    D: Future,
{
    type Output = (A::Output, B::Output, C::Output, D::Output);

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
        if this.result_d.is_none() {
            if let Some(ref mut fut) = this.d {
                if let Poll::Ready(val) = fut.as_mut().poll(cx) {
                    this.result_d = Some(val);
                    this.d = None;
                }
            }
        }

        if this.result_a.is_some()
            && this.result_b.is_some()
            && this.result_c.is_some()
            && this.result_d.is_some()
        {
            Poll::Ready((
                this.result_a.take().unwrap(),
                this.result_b.take().unwrap(),
                this.result_c.take().unwrap(),
                this.result_d.take().unwrap(),
            ))
        } else {
            Poll::Pending
        }
    }
}
/// 5 つの Future を並行して poll する。
/// Poll five futures concurrently.
pub struct Join5<A: Future, B: Future, C: Future, D: Future, E: Future> {
    a: Option<Pin<Box<A>>>,
    b: Option<Pin<Box<B>>>,
    c: Option<Pin<Box<C>>>,
    d: Option<Pin<Box<D>>>,
    e: Option<Pin<Box<E>>>,
    result_a: Option<A::Output>,
    result_b: Option<B::Output>,
    result_c: Option<C::Output>,
    result_d: Option<D::Output>,
    result_e: Option<E::Output>,
}
impl<A, B, C, D, E> Join5<A, B, C, D, E>
where A: Future, B: Future, C: Future, D: Future, E: Future,
{
    pub fn new(a: A, b: B, c: C, d: D, e: E) -> Self {
        Self {
            a: Some(Box::pin(a)), b: Some(Box::pin(b)),
            c: Some(Box::pin(c)), d: Some(Box::pin(d)), e: Some(Box::pin(e)),
            result_a: None, result_b: None, result_c: None, result_d: None, result_e: None,
        }
    }
}
impl<A, B, C, D, E> Future for Join5<A, B, C, D, E>
where A: Future, B: Future, C: Future, D: Future, E: Future,
{
    type Output = (A::Output, B::Output, C::Output, D::Output, E::Output);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        macro_rules! poll_slot { ($slot:ident, $res:ident) => {
            if this.$res.is_none() { if let Some(ref mut f) = this.$slot {
                if let Poll::Ready(v) = f.as_mut().poll(cx) { this.$res = Some(v); this.$slot = None; }
            }}
        }; }
        poll_slot!(a, result_a); poll_slot!(b, result_b); poll_slot!(c, result_c);
        poll_slot!(d, result_d); poll_slot!(e, result_e);
        if this.result_a.is_some() && this.result_b.is_some() && this.result_c.is_some()
            && this.result_d.is_some() && this.result_e.is_some() {
            Poll::Ready((this.result_a.take().unwrap(), this.result_b.take().unwrap(),
                         this.result_c.take().unwrap(), this.result_d.take().unwrap(),
                         this.result_e.take().unwrap()))
        } else { Poll::Pending }
    }
}

/// 6 つの Future を並行して poll する。
/// Poll six futures concurrently.
pub struct Join6<A: Future, B: Future, C: Future, D: Future, E: Future, F: Future> {
    a: Option<Pin<Box<A>>>, b: Option<Pin<Box<B>>>, c: Option<Pin<Box<C>>>,
    d: Option<Pin<Box<D>>>, e: Option<Pin<Box<E>>>, f: Option<Pin<Box<F>>>,
    result_a: Option<A::Output>, result_b: Option<B::Output>, result_c: Option<C::Output>,
    result_d: Option<D::Output>, result_e: Option<E::Output>, result_f: Option<F::Output>,
}
impl<A, B, C, D, E, F> Join6<A, B, C, D, E, F>
where A: Future, B: Future, C: Future, D: Future, E: Future, F: Future,
{
    pub fn new(a: A, b: B, c: C, d: D, e: E, f: F) -> Self {
        Self {
            a: Some(Box::pin(a)), b: Some(Box::pin(b)), c: Some(Box::pin(c)),
            d: Some(Box::pin(d)), e: Some(Box::pin(e)), f: Some(Box::pin(f)),
            result_a: None, result_b: None, result_c: None,
            result_d: None, result_e: None, result_f: None,
        }
    }
}
impl<A, B, C, D, E, F> Future for Join6<A, B, C, D, E, F>
where A: Future, B: Future, C: Future, D: Future, E: Future, F: Future,
{
    type Output = (A::Output, B::Output, C::Output, D::Output, E::Output, F::Output);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        macro_rules! poll_slot { ($slot:ident, $res:ident) => {
            if this.$res.is_none() { if let Some(ref mut f) = this.$slot {
                if let Poll::Ready(v) = f.as_mut().poll(cx) { this.$res = Some(v); this.$slot = None; }
            }}
        }; }
        poll_slot!(a, result_a); poll_slot!(b, result_b); poll_slot!(c, result_c);
        poll_slot!(d, result_d); poll_slot!(e, result_e); poll_slot!(f, result_f);
        if this.result_a.is_some() && this.result_b.is_some() && this.result_c.is_some()
            && this.result_d.is_some() && this.result_e.is_some() && this.result_f.is_some() {
            Poll::Ready((this.result_a.take().unwrap(), this.result_b.take().unwrap(),
                         this.result_c.take().unwrap(), this.result_d.take().unwrap(),
                         this.result_e.take().unwrap(), this.result_f.take().unwrap()))
        } else { Poll::Pending }
    }
}

/// 7 つの Future を並行して poll する。
/// Poll seven futures concurrently.
pub struct Join7<A: Future, B: Future, C: Future, D: Future, E: Future, F: Future, G: Future> {
    a: Option<Pin<Box<A>>>, b: Option<Pin<Box<B>>>, c: Option<Pin<Box<C>>>,
    d: Option<Pin<Box<D>>>, e: Option<Pin<Box<E>>>, f: Option<Pin<Box<F>>>, g: Option<Pin<Box<G>>>,
    result_a: Option<A::Output>, result_b: Option<B::Output>, result_c: Option<C::Output>,
    result_d: Option<D::Output>, result_e: Option<E::Output>, result_f: Option<F::Output>,
    result_g: Option<G::Output>,
}
impl<A, B, C, D, E, F, G> Join7<A, B, C, D, E, F, G>
where A: Future, B: Future, C: Future, D: Future, E: Future, F: Future, G: Future,
{
    pub fn new(a: A, b: B, c: C, d: D, e: E, f: F, g: G) -> Self {
        Self {
            a: Some(Box::pin(a)), b: Some(Box::pin(b)), c: Some(Box::pin(c)),
            d: Some(Box::pin(d)), e: Some(Box::pin(e)), f: Some(Box::pin(f)), g: Some(Box::pin(g)),
            result_a: None, result_b: None, result_c: None, result_d: None,
            result_e: None, result_f: None, result_g: None,
        }
    }
}
impl<A, B, C, D, E, F, G> Future for Join7<A, B, C, D, E, F, G>
where A: Future, B: Future, C: Future, D: Future, E: Future, F: Future, G: Future,
{
    type Output = (A::Output, B::Output, C::Output, D::Output, E::Output, F::Output, G::Output);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        macro_rules! poll_slot { ($slot:ident, $res:ident) => {
            if this.$res.is_none() { if let Some(ref mut f) = this.$slot {
                if let Poll::Ready(v) = f.as_mut().poll(cx) { this.$res = Some(v); this.$slot = None; }
            }}
        }; }
        poll_slot!(a, result_a); poll_slot!(b, result_b); poll_slot!(c, result_c);
        poll_slot!(d, result_d); poll_slot!(e, result_e); poll_slot!(f, result_f);
        poll_slot!(g, result_g);
        if this.result_a.is_some() && this.result_b.is_some() && this.result_c.is_some()
            && this.result_d.is_some() && this.result_e.is_some() && this.result_f.is_some()
            && this.result_g.is_some() {
            Poll::Ready((this.result_a.take().unwrap(), this.result_b.take().unwrap(),
                         this.result_c.take().unwrap(), this.result_d.take().unwrap(),
                         this.result_e.take().unwrap(), this.result_f.take().unwrap(),
                         this.result_g.take().unwrap()))
        } else { Poll::Pending }
    }
}

/// 8 つの Future を並行して poll する。
/// Poll eight futures concurrently.
pub struct Join8<A: Future, B: Future, C: Future, D: Future, E: Future, F: Future, G: Future, H: Future> {
    a: Option<Pin<Box<A>>>, b: Option<Pin<Box<B>>>, c: Option<Pin<Box<C>>>,
    d: Option<Pin<Box<D>>>, e: Option<Pin<Box<E>>>, f: Option<Pin<Box<F>>>,
    g: Option<Pin<Box<G>>>, h: Option<Pin<Box<H>>>,
    result_a: Option<A::Output>, result_b: Option<B::Output>, result_c: Option<C::Output>,
    result_d: Option<D::Output>, result_e: Option<E::Output>, result_f: Option<F::Output>,
    result_g: Option<G::Output>, result_h: Option<H::Output>,
}
impl<A, B, C, D, E, F, G, H> Join8<A, B, C, D, E, F, G, H>
where A: Future, B: Future, C: Future, D: Future, E: Future, F: Future, G: Future, H: Future,
{
    pub fn new(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H) -> Self {
        Self {
            a: Some(Box::pin(a)), b: Some(Box::pin(b)), c: Some(Box::pin(c)),
            d: Some(Box::pin(d)), e: Some(Box::pin(e)), f: Some(Box::pin(f)),
            g: Some(Box::pin(g)), h: Some(Box::pin(h)),
            result_a: None, result_b: None, result_c: None, result_d: None,
            result_e: None, result_f: None, result_g: None, result_h: None,
        }
    }
}
impl<A, B, C, D, E, F, G, H> Future for Join8<A, B, C, D, E, F, G, H>
where A: Future, B: Future, C: Future, D: Future, E: Future, F: Future, G: Future, H: Future,
{
    type Output = (A::Output, B::Output, C::Output, D::Output, E::Output, F::Output, G::Output, H::Output);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        macro_rules! poll_slot { ($slot:ident, $res:ident) => {
            if this.$res.is_none() { if let Some(ref mut f) = this.$slot {
                if let Poll::Ready(v) = f.as_mut().poll(cx) { this.$res = Some(v); this.$slot = None; }
            }}
        }; }
        poll_slot!(a, result_a); poll_slot!(b, result_b); poll_slot!(c, result_c);
        poll_slot!(d, result_d); poll_slot!(e, result_e); poll_slot!(f, result_f);
        poll_slot!(g, result_g); poll_slot!(h, result_h);
        if this.result_a.is_some() && this.result_b.is_some() && this.result_c.is_some()
            && this.result_d.is_some() && this.result_e.is_some() && this.result_f.is_some()
            && this.result_g.is_some() && this.result_h.is_some() {
            Poll::Ready((this.result_a.take().unwrap(), this.result_b.take().unwrap(),
                         this.result_c.take().unwrap(), this.result_d.take().unwrap(),
                         this.result_e.take().unwrap(), this.result_f.take().unwrap(),
                         this.result_g.take().unwrap(), this.result_h.take().unwrap()))
        } else { Poll::Pending }
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
