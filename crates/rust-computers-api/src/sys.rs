//! システム・時刻関連の API。
//! System and time-related APIs.
//!
//! ## UTC時刻とゲーム内時刻の取得
//!
//! CC:Tweaked の `os.epoch()` および `os.time()` に相当する機能。
//! - [`get_utc()`] - UTC による絶対時刻（Instant）
//! - [`get_time_ingame()`] - Minecraft ゲーム内時刻（Instant）
//!
//! ## Instant 型
//!
//! [`Instant`] は `no_std` 環境において時間を表現する型。
//! 内部的には以下のように構成されている：
//! - **UTC Instant**: Unix epoch (1970-01-01 00:00:00 UTC) からの経過ミリ秒
//! - **In-game Instant**: ワールド作成からの経過 ticks の倍精度浮動小数点表現
//!
//! # 使い方 / Usage
//!
//! ```rust,no_run
//! # #![no_std]
//! # extern crate alloc;
//! use rust_computers_api as rc;
//!
//! #[rc::entry]
//! async fn main() {
//!     let utc = rc::sys::get_utc();
//!     let ingame = rc::sys::get_time_ingame();
//!     rc::println!("UTC: {}", utc);
//!     rc::println!("In-game: {}", ingame);
//! }
//! ```

use crate::ffi;

// ==================================================================
// Instant 型定義
// ==================================================================

/// UTC時刻を表現する型（Unix epoch からのミリ秒）。
/// Represents UTC time in milliseconds since Unix epoch (1970-01-01 00:00:00 UTC).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    /// Unix epoch からの経過ミリ秒 / milliseconds since Unix epoch
    millis: i64,
}

impl Instant {
    /// ミリ秒値から Instant を構築する。
    /// Create an Instant from milliseconds since Unix epoch.
    pub fn from_millis(millis: i64) -> Self {
        Instant { millis }
    }

    /// Unix epoch からの経過ミリ秒を取得する。
    /// Get milliseconds since Unix epoch.
    pub fn as_millis(self) -> i64 {
        self.millis
    }

    /// Unix epoch からの経過秒（整数部）を取得する。
    /// Get whole seconds since Unix epoch.
    pub fn as_secs(self) -> i64 {
        self.millis / 1000
    }

    /// ミリ秒部分（余り）を取得する。
    /// Get the millisecond part (remainder).
    pub fn subsec_millis(self) -> u32 {
        (self.millis % 1000) as u32
    }
}

impl core::fmt::Display for Instant {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Instant({} ms)", self.millis)
    }
}

/// ゲーム内時刻を表現する型（ワールド作成からの ticks）。
/// Represents in-game time in ticks since world creation.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GameTime {
    /// ワールド作成からの経過 ticks / ticks since world creation
    ticks: i64,
}

impl GameTime {
    /// ticks値から GameTime を構築する。
    /// Create a GameTime from ticks since world creation.
    pub fn from_ticks(ticks: i64) -> Self {
        GameTime { ticks }
    }

    /// ワールド作成からの経過 ticks を取得する。
    /// Get ticks since world creation.
    pub fn as_ticks(self) -> i64 {
        self.ticks
    }

    /// days（日数）を取得する。1日 = 24000 ticks。
    /// Get the number of days (1 day = 24000 ticks).
    pub fn as_days(self) -> f64 {
        self.ticks as f64 / 24000.0
    }

    /// seconds（秒数）を取得する。1 second average = 50 ticks / 3600 = 20 ticks/second。
    /// Get approximate seconds. Minecraft average: 20 ticks per second.
    pub fn as_secs(self) -> f64 {
        self.ticks as f64 / 20.0
    }
}

impl core::fmt::Display for GameTime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "GameTime({} ticks, {} days)", self.ticks, self.as_days())
    }
}

// ==================================================================
// ホスト関数呼び出し / Host function calls
// ==================================================================

/// UTC時刻を取得する。
/// Get the current UTC time.
///
/// # Returns
///
/// Unix epoch (1970-01-01 00:00:00 UTC) からの経過ミリ秒を含む Instant。
/// An Instant representing milliseconds since Unix epoch.
///
/// # 使い方 / Usage
///
/// ```rust,no_run
/// use rust_computers_api as rc;
///
/// let utc = rc::sys::get_utc();
/// rc::println!("UTC millis: {}", utc.as_millis());
/// rc::println!("UTC secs: {}", utc.as_secs());
/// ```
pub fn get_utc() -> Instant {
    unsafe {
        let millis = ffi::host_get_time_utc_millis();
        Instant::from_millis(millis)
    }
}

/// ゲーム内時刻を取得する。
/// Get the current in-game time.
///
/// # Returns
///
/// ワールド作成からの経過 ticks を含む GameTime。
/// A GameTime representing ticks since world creation.
///
/// # 使い方 / Usage
///
/// ```rust,no_run
/// use rust_computers_api as rc;
///
/// let ingame = rc::sys::get_time_ingame();
/// rc::println!("In-game ticks: {}", ingame.as_ticks());
/// rc::println!("In-game days: {}", ingame.as_days());
/// ```
pub fn get_time_ingame() -> GameTime {
    unsafe {
        let ticks = ffi::host_get_time_ingame_ticks();
        GameTime::from_ticks(ticks)
    }
}
