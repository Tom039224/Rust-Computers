//! ペリフェラル操作 API。
//! Peripheral API for interacting with adjacent Minecraft blocks.
//!
//! コンピュータの隣接6方向に設置されたブロックと通信する。
//! Communicates with blocks placed in the 6 adjacent directions of the computer.
//!
//! # 方向 ID / Direction IDs
//!
//! | periph_id | 方向 / Direction |
//! |-----------|------------------|
//! | 0         | DOWN   (下)      |
//! | 1         | UP     (上)      |
//! | 2         | NORTH  (北)      |
//! | 3         | SOUTH  (南)      |
//! | 4         | WEST   (西)      |
//! | 5         | EAST   (東)      |
//!
//! # 使い方 / Usage
//!
//! ```rust,no_run
//! use rust_computers_api::peripheral::{self, Direction};
//!
//! // 非同期リクエスト (host_request_info)
//! let data = peripheral::request_info(Direction::UP, "getItems", &[]).await?;
//!
//! // 非同期アクション (host_do_action)
//! peripheral::do_action(Direction::SOUTH, "pushItem", &args).await?;
//!
//! // 即時リクエスト (host_request_info_imm) — 同一 tick で結果を得る
//! let temp = peripheral::request_info_imm(Direction::DOWN, "getTemperature", &[])?;
//! ```

use alloc::vec;
use alloc::vec::Vec;

use crate::error::BridgeError;
use crate::ffi;
use crate::future::RequestFuture;

// ==================================================================
// 方向列挙 / Direction enum
// ==================================================================

/// コンピュータから見た方向 (periph_id に対応)。
/// Direction relative to the computer (corresponds to periph_id).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Direction {
    Down  = 0,
    Up    = 1,
    North = 2,
    South = 3,
    West  = 4,
    East  = 5,
}

impl Direction {
    /// periph_id を Direction に変換する。無効な値は None。
    /// Convert a periph_id to a Direction. Returns None for invalid values.
    pub fn from_id(id: u32) -> Option<Self> {
        match id {
            0 => Some(Self::Down),
            1 => Some(Self::Up),
            2 => Some(Self::North),
            3 => Some(Self::South),
            4 => Some(Self::West),
            5 => Some(Self::East),
            _ => None,
        }
    }

    /// periph_id 値を返す。
    /// Return the periph_id value.
    pub fn id(self) -> u32 {
        self as u32
    }
}

// ==================================================================
// CRC32 ハッシュ / CRC32 hash
// ==================================================================

/// メソッド名を CRC32 ハッシュに変換する。
/// Convert a method name to a CRC32 hash.
///
/// Java 側の `java.util.zip.CRC32` と同じアルゴリズム。
/// Uses the same algorithm as Java's `java.util.zip.CRC32`.
pub fn method_id(name: &str) -> u32 {
    crc32(name.as_bytes())
}

/// CRC32 (IEEE 802.3) を計算する。
/// Compute CRC32 (IEEE 802.3).
fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

// ==================================================================
// 非同期リクエスト / Async requests
// ==================================================================

/// ペリフェラルに情報リクエストを送信する（非同期）。
/// Send an information request to a peripheral (async).
///
/// Java 側の `host_request_info` を呼び出し、結果を待つ。
/// Calls `host_request_info` on the Java side and awaits the result.
///
/// # 引数 / Arguments
/// - `dir`: ペリフェラルの方向 / peripheral direction
/// - `method_name`: メソッド名 / method name
/// - `args`: MessagePack 引数バイト列 / MessagePack argument bytes
///
/// # 戻り値 / Returns
/// 結果バイト列、またはエラー / result bytes or error
pub fn request_info(
    dir: Direction,
    method_name: &str,
    args: &[u8],
) -> impl core::future::Future<Output = Result<Vec<u8>, BridgeError>> {
    issue_request(dir, method_name, args, false)
}

/// ペリフェラルにアクションリクエストを送信する（非同期）。
/// Send an action request to a peripheral (async).
///
/// Java 側の `host_do_action` を呼び出し、結果を待つ。
/// Calls `host_do_action` on the Java side and awaits the result.
pub fn do_action(
    dir: Direction,
    method_name: &str,
    args: &[u8],
) -> impl core::future::Future<Output = Result<Vec<u8>, BridgeError>> {
    issue_request(dir, method_name, args, true)
}

/// 内部: 非同期リクエストを発行する。
/// Internal: issue an async request.
fn issue_request(
    dir: Direction,
    method_name: &str,
    args: &[u8],
    is_action: bool,
) -> RequestFuture {
    // 結果バッファを確保 / Allocate result buffer
    const RESULT_BUF_SIZE: usize = 4096;
    let result_buf = vec![0u8; RESULT_BUF_SIZE];

    let mid = method_id(method_name);

    let request_id = unsafe {
        if is_action {
            ffi::host_do_action(
                dir.id(),
                mid,
                args.as_ptr() as i32,
                args.len() as i32,
                result_buf.as_ptr() as i32,
                result_buf.len() as i32,
            )
        } else {
            ffi::host_request_info(
                dir.id(),
                mid,
                args.as_ptr() as i32,
                args.len() as i32,
                result_buf.as_ptr() as i32,
                result_buf.len() as i32,
            )
        }
    };

    RequestFuture::new(request_id, result_buf)
}

// ==================================================================
// 即時リクエスト / Immediate requests
// ==================================================================

/// ペリフェラルに即時情報リクエストを送信する（同期）。
/// Send an immediate information request to a peripheral (synchronous).
///
/// Java 側の `host_request_info_imm` を呼び出し、同一 tick 内で結果を得る。
/// Calls `host_request_info_imm` on the Java side, returning within the same tick.
///
/// # 注意 / Notice
/// 全てのメソッドが即時呼び出しをサポートするわけではない。
/// Not all methods support immediate invocation.
pub fn request_info_imm(
    dir: Direction,
    method_name: &str,
    args: &[u8],
) -> Result<Vec<u8>, BridgeError> {
    const RESULT_BUF_SIZE: usize = 4096;
    let mut result_buf = vec![0u8; RESULT_BUF_SIZE];

    let mid = method_id(method_name);

    let written = unsafe {
        ffi::host_request_info_imm(
            dir.id(),
            mid,
            args.as_ptr() as i32,
            args.len() as i32,
            result_buf.as_ptr() as i32,
            result_buf.len() as i32,
        )
    };

    if written < 0 {
        return Err(BridgeError::from_code(written));
    }

    result_buf.truncate(written as usize);
    Ok(result_buf)
}

// ==================================================================
// ユーティリティ / Utilities
// ==================================================================

/// 指定 Mod が利用可能か確認する。
/// Check if the specified mod is available.
///
/// Java 側の `host_is_mod_available` を呼び出す。
/// Calls `host_is_mod_available` on the Java side.
pub fn is_mod_available(mod_name: &str) -> bool {
    let mid = crc32(mod_name.as_bytes()) as u16;
    unsafe { ffi::host_is_mod_available(mid) != 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32_basic() {
        // "test" の CRC32 = 0xD87F7E0C
        assert_eq!(crc32(b"test"), 0xD87F7E0C);
    }

    #[test]
    fn test_direction_roundtrip() {
        for id in 0..=5 {
            let dir = Direction::from_id(id).unwrap();
            assert_eq!(dir.id(), id);
        }
        assert!(Direction::from_id(6).is_none());
    }
}
