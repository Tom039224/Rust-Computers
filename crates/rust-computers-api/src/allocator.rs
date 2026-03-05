//! グローバルアロケータ設定。
//! Global allocator configuration.
//!
//! `dlmalloc` を WASM 用グローバルアロケータとして使用する。
//! Uses `dlmalloc` as the global allocator for WASM.
//!
//! ユーザーはこのモジュールを意識する必要がない。
//! Users do not need to be aware of this module.

use dlmalloc::GlobalDlmalloc;

/// グローバルアロケータ / Global allocator
#[global_allocator]
static ALLOC: GlobalDlmalloc = GlobalDlmalloc;
