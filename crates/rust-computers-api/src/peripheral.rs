//! ペリフェラル操作 API。
//! Peripheral API for interacting with adjacent or wired-modem-connected Minecraft blocks.
//!
//! コンピュータの隣接6方向（直結）または有線モデム経由で接続されたブロックと通信する。
//! Communicates with blocks placed in the 6 adjacent directions or connected via wired modem.
//!
//! ## ペリフェラルアドレス / Peripheral address
//!
//! ペリフェラルは [`PeriphAddr`] という `u32` で識別される。
//! Each peripheral is identified by a [`PeriphAddr`] (`u32`).
//!
//! | periph_id | 意味 / Meaning                        |
//! |-----------|---------------------------------------|
//! | 0         | DOWN (下)  直結 / direct               |
//! | 1         | UP (上)    直結 / direct               |
//! | 2         | NORTH (北) 直結 / direct               |
//! | 3         | SOUTH (南) 直結 / direct               |
//! | 4         | WEST (西)  直結 / direct               |
//! | 5         | EAST (東)  直結 / direct               |
//! | 6+        | 有線モデム経由接続 / wired modem        |
//!
//! ## 1tick 遅れ原則 / 1-tick delay principle
//!
//! すべての情報取得およびアクション API は **1 Game Tick (GT) 遅れ**で結果が返る。
//! All info and action APIs return results with a **1 Game Tick (GT) delay**.
//!
//! ```text
//! GT:N   Rust → リクエスト発行
//! GT:N+1 Java → 結果を収集し Rust に渡す
//! GT:N+1 Rust → .await の先に進む
//! ```
//!
//! # 使い方 / Usage
//!
//! ```rust,no_run
//! use rust_computers_api::peripheral::{self, PeriphAddr, Direction};
//!
//! // 情報取得 (1tick 遅れ) / Info request (1-tick delay)
//! let data = peripheral::request_info(Direction::Up.into(), "getItems", &[]).await?;
//!
//! // アクション (1tick 遅れ) / Action (1-tick delay)
//! peripheral::do_action(Direction::South.into(), "pushItem", &args).await?;
//!
//! // 有線モデム含む全検索 / Find all (including wired modem)
//! let radars = peripheral::find_imm::<Radar>();
//! ```

use alloc::vec;
use alloc::vec::Vec;

use crate::error::BridgeError;
use crate::ffi;
use crate::future::RequestFuture;

// ==================================================================
// 方向列挙 / Direction enum
// ==================================================================

/// コンピュータから見た方向 (直結ペリフェラルの periph_id に対応)。
/// Direction relative to the computer (corresponds to directly-connected periph_id).
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

impl From<Direction> for PeriphAddr {
    fn from(d: Direction) -> Self {
        PeriphAddr(d as u32)
    }
}

// ==================================================================
// ペリフェラルアドレス / Peripheral address
// ==================================================================

/// ペリフェラルを識別する汎用 ID。
/// Generic peripheral identifier.
///
/// - 0–5: 直結ペリフェラル (DOWN/UP/NORTH/SOUTH/WEST/EAST) / direct connection
/// - 6+:  有線モデム経由ペリフェラル / wired modem connection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PeriphAddr(pub u32);

impl PeriphAddr {
    /// raw periph_id から PeriphAddr を作成する。
    /// Create a PeriphAddr from a raw periph_id.
    pub const fn from_raw(id: u32) -> Self {
        Self(id)
    }

    /// Direction から PeriphAddr を作成する (const)。
    /// Create a PeriphAddr from a Direction (const).
    pub const fn from_dir(dir: Direction) -> Self {
        Self(dir as u32)
    }

    /// raw periph_id 値を返す。
    /// Return the raw periph_id value.
    pub fn raw(self) -> u32 {
        self.0
    }

    /// Direction に変換できれば Some(dir) を返す。
    /// Convert to Direction if this is a directly-connected peripheral.
    pub fn as_direction(self) -> Option<Direction> {
        Direction::from_id(self.0)
    }

    /// 直結ペリフェラル (periph_id 0–5) かどうか。
    /// Whether this is a directly-connected peripheral (periph_id 0–5).
    pub fn is_direct(self) -> bool {
        self.0 <= 5
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
/// - `addr`: ペリフェラルアドレス / peripheral address
/// - `method_name`: メソッド名 / method name
/// - `args`: MessagePack 引数バイト列 / MessagePack argument bytes
///
/// # 戻り値 / Returns
/// 結果バイト列、またはエラー / result bytes or error
pub fn request_info(
    addr: impl Into<PeriphAddr>,
    method_name: &str,
    args: &[u8],
) -> impl core::future::Future<Output = Result<Vec<u8>, BridgeError>> {
    issue_request(addr.into(), method_name, args, false)
}

/// ペリフェラルにアクションリクエストを送信する（非同期）。
/// Send an action request to a peripheral (async).
///
/// Java 側の `host_do_action` を呼び出し、結果を待つ。
/// Calls `host_do_action` on the Java side and awaits the result.
pub fn do_action(
    addr: impl Into<PeriphAddr>,
    method_name: &str,
    args: &[u8],
) -> impl core::future::Future<Output = Result<Vec<u8>, BridgeError>> {
    issue_request(addr.into(), method_name, args, true)
}

/// 内部: 非同期リクエストを発行する。
/// Internal: issue an async request.
fn issue_request(
    addr: PeriphAddr,
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
                addr.raw() as i32,
                mid,
                args.as_ptr() as i32,
                args.len() as i32,
                result_buf.as_ptr() as i32,
                result_buf.len() as i32,
            )
        } else {
            ffi::host_request_info(
                addr.raw() as i32,
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

/// ペリフェラルに即時情報リクエストを送信する（同 tick 内で完結）。
/// Send an immediate information request to a peripheral (completes in the same tick).
///
/// ## ⚠️ 使用条件 / Usage restriction
///
/// このAPIは **`@LuaFunction(immediate=true)`** として実装された Java メソッド専用。
/// 通常の情報取得には [`request_info`] を使用すること。
///
/// This API is **only for Java methods implemented as `@LuaFunction(immediate=true)`**.
/// For regular info retrieval, use [`request_info`] instead.
///
/// `_imm` サフィックスにより、呼び出し側が 1tick 遅れ原則の例外であることを
/// 明示的に了承していることを示す。
/// The `_imm` suffix signals that the caller explicitly acknowledges
/// this is an intentional exception to the 1-tick delay principle.
///
/// # 引数 / Arguments
/// - `addr`: ペリフェラルアドレス / peripheral address
/// - `method_name`: メソッド名 / method name
/// - `args`: MessagePack 引数バイト列 / MessagePack argument bytes
///
/// # 戻り値 / Returns
/// 結果バイト列、またはエラー / result bytes or error
pub fn request_info_imm(
    addr: impl Into<PeriphAddr>,
    method_name: &str,
    args: &[u8],
) -> Result<Vec<u8>, BridgeError> {
    const RESULT_BUF_SIZE: usize = 4096;
    let mut result_buf = vec![0u8; RESULT_BUF_SIZE];
    let mid = method_id(method_name);
    let periph_id = addr.into().raw();
    let written = unsafe {
        ffi::host_request_info_imm(
            periph_id as i32,
            mid,
            args.as_ptr() as i32,
            args.len() as i32,
            result_buf.as_mut_ptr() as i32,
            result_buf.len() as i32,
        )
    };
    if written < 0 {
        Err(BridgeError::from_code(written))
    } else {
        Ok(result_buf[..written as usize].to_vec())
    }
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

// ==================================================================
// Peripheral trait / ペリフェラル trait
// ==================================================================

use crate::error::PeripheralError;

/// 全ペリフェラル構造体が実装する trait。
/// Trait implemented by all peripheral structs.
pub trait Peripheral: Sized {
    /// CC:Tweaked 上のペリフェラル型名 (例: `"create:creative_motor"`)
    const NAME: &'static str;
    /// 指定アドレスに新しいインスタンスを作成する。
    /// Create a new instance at the specified address.
    fn new(addr: PeriphAddr) -> Self;
    /// このインスタンスが参照するアドレスを返す。
    /// Return the address this instance refers to.
    fn periph_addr(&self) -> PeriphAddr;

    /// Direction に変換できれば Some(dir) を返す (直結接続の場合)。
    /// Returns Some(dir) if this is a directly-connected peripheral.
    fn direction(&self) -> Option<Direction> {
        self.periph_addr().as_direction()
    }
}

/// 指定アドレスにある名前一致ペリフェラルを同期的に取得する。
/// Synchronously wrap a peripheral at the given address if its type matches.
pub fn wrap_imm<T: Peripheral>(addr: impl Into<PeriphAddr>) -> Result<T, PeripheralError> {
    let addr = addr.into();
    let args = crate::msgpack::array(&[crate::msgpack::str(T::NAME)]);
    let data = request_info_imm(addr, "hasType", &args)?;
    let (val, _) = crate::msgpack::Value::decode(&data).unwrap_or((crate::msgpack::Value::Nil, 0));
    match val {
        crate::msgpack::Value::Bool(true) => Ok(T::new(addr)),
        _ => Err(PeripheralError::NotFound),
    }
}

/// 有線モデム含む全接続から名前一致ペリフェラルを同期的に検索する。
/// Synchronously find all peripherals matching the given type,
/// including wired modem connections.
///
/// CC:Tweaked の `peripheral.find("<type>")` と同等の動作。
/// Equivalent to CC:Tweaked's `peripheral.find("<type>")`.
pub fn find_imm<T: Peripheral>() -> alloc::vec::Vec<T> {
    // Java 側から一致する periph_id のリストを取得する
    // Get the list of matching periph_ids from the Java side
    const BUF_SIZE: usize = 256;
    let mut buf = [0u8; BUF_SIZE];
    let name = T::NAME;
    let written = unsafe {
        ffi::host_find_peripherals_by_type_imm(
            name.as_ptr() as i32,
            name.len() as i32,
            buf.as_mut_ptr() as i32,
            BUF_SIZE as i32,
        )
    };

    if written <= 0 {
        return alloc::vec::Vec::new();
    }

    // msgpack 配列 (u32[]) をデコードして T のインスタンスを生成する
    // Decode msgpack array (u32[]) and create T instances
    let (val, _) = crate::msgpack::Value::decode(&buf[..written as usize])
        .unwrap_or((crate::msgpack::Value::Nil, 0));

    let mut result = alloc::vec::Vec::new();
    if let crate::msgpack::Value::Array(ids) = val {
        for item in ids {
            if let crate::msgpack::Value::Integer(id) = item {
                if id >= 0 {
                    result.push(T::new(PeriphAddr::from_raw(id as u32)));
                }
            }
        }
    }
    result
}

/// 指定アドレスにある名前一致ペリフェラルを非同期的に取得する。
/// Asynchronously wrap a peripheral at the given address.
pub async fn wrap<T: Peripheral>(addr: impl Into<PeriphAddr>) -> Result<T, PeripheralError> {
    let addr = addr.into();
    let args = crate::msgpack::array(&[crate::msgpack::str(T::NAME)]);
    let data = request_info(addr, "hasType", &args).await?;
    let (val, _) = crate::msgpack::Value::decode(&data).unwrap_or((crate::msgpack::Value::Nil, 0));
    match val {
        crate::msgpack::Value::Bool(true) => Ok(T::new(addr)),
        _ => Err(PeripheralError::NotFound),
    }
}

// ==================================================================
// Decode helpers
// ==================================================================

/// msgpack レスポンスバイト列からデシリアライズする。
/// Deserialize from msgpack response bytes.
pub fn decode<'de, T: serde::Deserialize<'de>>(data: &[u8]) -> Result<T, PeripheralError> {
    crate::serde_msgpack::from_bytes(data).map_err(|_| PeripheralError::DecodeFailed)
}

/// serde でペイロードを msgpack バイト列にエンコードする。
/// Encode a payload to msgpack bytes via serde.
pub fn encode<T: serde::Serialize>(val: &T) -> Result<alloc::vec::Vec<u8>, PeripheralError> {
    crate::serde_msgpack::to_bytes(val).map_err(|_| PeripheralError::DecodeFailed)
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
