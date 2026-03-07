//! エラー型定義。
//! Error type definitions.
//!
//! Java 側の `ErrorCodes` と対応するエラー列挙型。
//! Error enum corresponding to Java-side `ErrorCodes`.

use core::fmt;

/// ブリッジエラー — ホスト関数呼び出しで発生しうるエラー。
/// Bridge error — errors that can occur during host function calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeError {
    /// 無効な request_id / Invalid request ID
    InvalidRequestId,
    /// 無効なペリフェラル ID / Invalid peripheral ID
    InvalidPeripheral,
    /// メソッドが見つからない / Method not found
    MethodNotFound,
    /// Java 側で例外が発生 / Java-side exception
    JavaException,
    /// リクエストがタイムアウト / Request timed out
    Timeout,
    /// Fuel 切れ / Fuel exhausted
    FuelExhausted,
    /// result バッファが小さすぎる / Result buffer too small
    ResultBufTooSmall,
    /// Mod が利用不可 / Mod not available
    ModNotAvailable,
    /// 結果が失われた / Result lost
    ResultLost,
    /// 不明なエラー / Unknown error
    Unknown(i32),
}

impl BridgeError {
    /// Java 側のエラーコードからエラーを生成する。
    /// Create an error from a Java-side error code.
    ///
    /// # 引数 / Arguments
    /// - `code`: 負のエラーコード / negative error code
    pub fn from_code(code: i32) -> Self {
        match code {
            -1 => Self::InvalidRequestId,
            -2 => Self::InvalidPeripheral,
            -3 => Self::MethodNotFound,
            -4 => Self::JavaException,
            -5 => Self::Timeout,
            -6 => Self::FuelExhausted,
            -7 => Self::ResultBufTooSmall,
            -8 => Self::ModNotAvailable,
            -9 => Self::ResultLost,
            other => Self::Unknown(other),
        }
    }
}

impl fmt::Display for BridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequestId  => write!(f, "invalid request ID"),
            Self::InvalidPeripheral => write!(f, "invalid peripheral"),
            Self::MethodNotFound    => write!(f, "method not found"),
            Self::JavaException     => write!(f, "Java exception"),
            Self::Timeout           => write!(f, "request timed out"),
            Self::FuelExhausted     => write!(f, "fuel exhausted"),
            Self::ResultBufTooSmall => write!(f, "result buffer too small"),
            Self::ModNotAvailable   => write!(f, "mod not available"),
            Self::ResultLost        => write!(f, "result lost"),
            Self::Unknown(code)     => write!(f, "unknown error ({})", code),
        }
    }
}

// ==================================================================
// PeripheralError
// ==================================================================

/// ペリフェラル操作エラー。
/// Peripheral operation error.
#[derive(Debug, Clone)]
pub enum PeripheralError {
    /// ブリッジ通信エラー / Bridge communication error
    Bridge(BridgeError),
    /// ペリフェラルが見つからない / Peripheral not found
    NotFound,
    /// レスポンスのデコード失敗 / Response decode failure
    DecodeFailed,
    /// 予期しないエラー (メッセージ付き) / Unexpected error (with message)
    Unexpected(alloc::string::String),
}

impl From<BridgeError> for PeripheralError {
    fn from(e: BridgeError) -> Self {
        Self::Bridge(e)
    }
}

impl From<crate::serde_msgpack::Error> for PeripheralError {
    fn from(_e: crate::serde_msgpack::Error) -> Self {
        Self::DecodeFailed
    }
}

impl fmt::Display for PeripheralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bridge(e)      => write!(f, "bridge error: {}", e),
            Self::NotFound       => write!(f, "peripheral not found"),
            Self::DecodeFailed   => write!(f, "response decode failed"),
            Self::Unexpected(s)  => write!(f, "unexpected error: {}", s),
        }
    }
}
