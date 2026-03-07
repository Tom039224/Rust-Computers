//! 最小限の MessagePack エンコーダ。
//! Minimal MessagePack encoder for peripheral argument encoding.
//!
//! # サポートする型 / Supported types
//! - `nil()` → `0xC0`
//! - `bool_val(bool)` → `0xC2` / `0xC3`
//! - `int(i32)` → positive fixint / negative fixint / uint8 / uint16 / int32
//! - `float64(f64)` → float64 (0xCB + 8 bytes)
//! - `str(&str)` → fixstr (≤31 bytes) / str8 (≤255 bytes)
//! - `array(&[Vec<u8>])` → fixarray (≤15 elements)
//!
//! # 使い方 / Usage
//!
//! ```rust,no_run
//! use rust_computers_api::msgpack as m;
//!
//! // write("Hello")
//! let args = m::array(&[m::str("Hello")]);
//!
//! // setCursorPos(3, 2)
//! let args = m::array(&[m::int(3), m::int(2)]);
//!
//! // setTextColor(colors.red = 16384)
//! let args = m::array(&[m::int(16384)]);
//!
//! // 引数なし / No args
//! let args = m::array(&[]);
//! ```

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

// ==================================================================
// エンコード / Encoding
// ==================================================================

/// `nil` をエンコードする / Encode nil.
#[inline]
pub fn nil() -> Vec<u8> {
    vec![0xC0]
}

/// bool をエンコードする / Encode a boolean.
#[inline]
pub fn bool_val(v: bool) -> Vec<u8> {
    vec![if v { 0xC3 } else { 0xC2 }]
}

/// i32 をエンコードする (最小バイト数) / Encode an i32 (minimal size).
pub fn int(v: i32) -> Vec<u8> {
    if v >= 0 && v <= 0x7F   { return vec![v as u8]; }          // positive fixint
    if v >= -32 && v < 0     { return vec![v as u8]; }          // negative fixint
    if v >= 0 && v <= 0xFF   { return vec![0xCC, v as u8]; }    // uint 8
    if v >= 0 && v <= 0xFFFF {                                   // uint 16
        return vec![0xCD, (v >> 8) as u8, v as u8];
    }
    vec![0xD2, (v >> 24) as u8, (v >> 16) as u8, (v >> 8) as u8, v as u8] // int 32
}

/// &str を fixstr / str8 としてエンコードする / Encode a &str as fixstr or str8.
pub fn str(s: &str) -> Vec<u8> {
    let b = s.as_bytes();
    if b.len() <= 31 {
        let mut out = Vec::with_capacity(1 + b.len());
        out.push(0xA0 | b.len() as u8);
        out.extend_from_slice(b);
        return out;
    }
    // str8 (≤ 255 bytes)
    let mut out = Vec::with_capacity(2 + b.len());
    out.push(0xD9);
    out.push(b.len() as u8);
    out.extend_from_slice(b);
    out
}

/// 要素リストを fixarray としてエンコードする (最大 15 要素)。
/// Encode elements as a fixarray (up to 15 elements).
///
/// # Panics
/// `items.len() > 15` の場合パニックする。
pub fn array(items: &[Vec<u8>]) -> Vec<u8> {
    assert!(items.len() <= 15, "msgpack::array: fixarray max 15 elements");
    let data_len: usize = items.iter().map(|b| b.len()).sum();
    let mut out = Vec::with_capacity(1 + data_len);
    out.push(0x90 | items.len() as u8);
    for item in items {
        out.extend_from_slice(item);
    }
    out
}

/// バイナリデータを bin 8/16 としてエンコードする。
/// Encode binary data as MessagePack bin format.
pub fn bytes(data: &[u8]) -> Vec<u8> {
    let len = data.len();
    let mut out = if len <= 0xFF {
        let mut v = Vec::with_capacity(2 + len);
        v.push(0xC4);
        v.push(len as u8);
        v
    } else {
        let mut v = Vec::with_capacity(3 + len);
        v.push(0xC5);
        v.push((len >> 8) as u8);
        v.push(len as u8);
        v
    };
    out.extend_from_slice(data);
    out
}

/// f64 を float64 (0xCB + 8 bytes big-endian IEEE 754) としてエンコードする。
/// Encode an f64 as MessagePack float64.
pub fn float64(v: f64) -> Vec<u8> {
    let mut out = Vec::with_capacity(9);
    out.push(0xCB);
    out.extend_from_slice(&v.to_bits().to_be_bytes());
    out
}

/// i64 を int64 (0xD3 + 8 bytes big-endian) としてエンコードする。
/// Encode an i64 as MessagePack int64.
pub fn int64(v: i64) -> Vec<u8> {
    // 小さい値は最小バイト数でエンコード / Small values use minimal encoding
    if v >= 0 && v <= 0x7F       { return vec![v as u8]; }            // positive fixint
    if v >= -32 && v < 0         { return vec![v as u8]; }            // negative fixint
    if v >= 0 && v <= 0xFF       { return vec![0xCC, v as u8]; }      // uint 8
    if v >= 0 && v <= 0xFFFF     { return vec![0xCD, (v >> 8) as u8, v as u8]; } // uint 16
    if v >= i32::MIN as i64 && v <= i32::MAX as i64 {
        let w = v as i32;
        return vec![0xD2, (w >> 24) as u8, (w >> 16) as u8, (w >> 8) as u8, w as u8]; // int 32
    }
    // int 64
    let mut out = Vec::with_capacity(9);
    out.push(0xD3);
    out.extend_from_slice(&v.to_be_bytes());
    out
}

/// i64 を decode する。int8/int16/int32/int64/uint8/uint16/uint32/uint64 に対応。
/// Decode an i64. Supports int8/int16/int32/int64/uint8/uint16/uint32/uint64.
pub fn decode_int64_at(data: &[u8], index: usize) -> i64 {
    let off = arg_offset(data, index);
    if off < 0 { return 0; }
    let pos = off as usize;
    if pos >= data.len() { return 0; }
    let b = data[pos];
    match b {
        0x00..=0x7F => b as i64,                  // positive fixint
        0xE0..=0xFF => (b as i8) as i64,          // negative fixint
        0xCC => data.get(pos + 1).map_or(0, |&v| v as i64),  // uint8
        0xCD => {
            let hi = data.get(pos + 1).copied().unwrap_or(0) as i64;
            let lo = data.get(pos + 2).copied().unwrap_or(0) as i64;
            (hi << 8) | lo
        }
        0xCE => {
            let a = data.get(pos + 1).copied().unwrap_or(0) as u32;
            let b2 = data.get(pos + 2).copied().unwrap_or(0) as u32;
            let c = data.get(pos + 3).copied().unwrap_or(0) as u32;
            let d = data.get(pos + 4).copied().unwrap_or(0) as u32;
            ((a << 24) | (b2 << 16) | (c << 8) | d) as i64
        }
        0xD0 => data.get(pos + 1).map_or(0, |&v| v as i8 as i64),  // int8
        0xD1 => {
            let hi = data.get(pos + 1).copied().unwrap_or(0) as i16;
            let lo = data.get(pos + 2).copied().unwrap_or(0) as i16;
            ((hi << 8) | lo) as i64
        }
        0xD2 => {
            let a = data.get(pos + 1).copied().unwrap_or(0) as i32;
            let b2 = data.get(pos + 2).copied().unwrap_or(0) as i32;
            let c = data.get(pos + 3).copied().unwrap_or(0) as i32;
            let d = data.get(pos + 4).copied().unwrap_or(0) as i32;
            ((a << 24) | (b2 << 16) | (c << 8) | d) as i64
        }
        0xD3 => {
            if data.len() < pos + 9 { return 0; }
            let mut val: i64 = 0;
            for i in 0..8 {
                val = (val << 8) | (data.get(pos + 1 + i).copied().unwrap_or(0) as i64);
            }
            val
        }
        _ => 0,
    }
}

// ==================================================================
// デコード / Decoding
// ==================================================================

/// fixarray の i 番目の要素オフセットを返す。
/// Return the byte offset of the i-th element in a fixarray.
///
/// 戻り値が `-1` の場合は要素が存在しない。
/// Returns `-1` if the element does not exist.
pub fn arg_offset(data: &[u8], index: usize) -> i32 {
    if data.is_empty() { return -1; }
    let b0 = data[0];
    let (count, mut pos) = if (b0 & 0xF0) == 0x90 {
        ((b0 & 0x0F) as usize, 1usize)
    } else {
        // 配列でない場合は単一値として扱う / treat as single value
        return if index == 0 { 0 } else { -1 };
    };
    if index >= count { return -1; }
    for _ in 0..index {
        let next = skip_element(data, pos);
        if next < 0 { return -1; }
        pos = next as usize;
    }
    pos as i32
}

/// offset 位置の int 値を読む / Read an int value at offset.
pub fn decode_int(data: &[u8], offset: usize) -> i32 {
    if data.len() <= offset { return 0; }
    let b = data[offset];
    match b {
        0x00..=0x7F => b as i32,                // positive fixint
        0xE0..=0xFF => (b as i8) as i32,        // negative fixint
        0xCC => data.get(offset + 1).map_or(0, |&v| v as i32), // uint8
        0xCD => {
            let hi = data.get(offset + 1).copied().unwrap_or(0) as i32;
            let lo = data.get(offset + 2).copied().unwrap_or(0) as i32;
            (hi << 8) | lo
        }
        0xD0 => data.get(offset + 1).map_or(0, |&v| v as i8 as i32), // int8
        0xD1 => {
            let hi = data.get(offset + 1).copied().unwrap_or(0) as i32;
            let lo = data.get(offset + 2).copied().unwrap_or(0) as i32;
            ((hi << 8) | lo) as i16 as i32
        }
        0xD2 => {
            let a = data.get(offset + 1).copied().unwrap_or(0) as i32;
            let b2 = data.get(offset + 2).copied().unwrap_or(0) as i32;
            let c = data.get(offset + 3).copied().unwrap_or(0) as i32;
            let d = data.get(offset + 4).copied().unwrap_or(0) as i32;
            (a << 24) | (b2 << 16) | (c << 8) | d
        }
        _ => 0,
    }
}

/// fixarray の i 番目の要素を int としてデコードする。
/// Decode the i-th element of a fixarray as an int.
pub fn decode_int_at(data: &[u8], index: usize) -> i32 {
    let off = arg_offset(data, index);
    if off < 0 { return 0; }
    decode_int(data, off as usize)
}

/// fixarray の i 番目の要素を bool としてデコードする。
/// Decode the i-th element of a fixarray as a bool.
pub fn decode_bool_at(data: &[u8], index: usize) -> bool {
    let off = arg_offset(data, index);
    if off < 0 { return false; }
    data[off as usize] == 0xC3
}

/// fixarray の i 番目の要素を &str としてデコードする。
/// Decode the i-th element of a fixarray as a &str.
/// 不正値の場合は空文字列を返す。Returns "" on invalid data.
pub fn decode_str_at<'a>(data: &'a [u8], index: usize) -> &'a str {
    let off = arg_offset(data, index);
    if off < 0 { return ""; }
    let pos = off as usize;
    if pos >= data.len() { return ""; }
    let tag = data[pos];
    let (start, len) = match tag {
        0xA0..=0xBF => (pos + 1, (tag & 0x1F) as usize),  // fixstr
        0xD9 => {                                           // str 8
            if data.len() <= pos + 1 { return ""; }
            (pos + 2, data[pos + 1] as usize)
        }
        _ => return "",
    };
    if start + len > data.len() { return ""; }
    core::str::from_utf8(&data[start..start + len]).unwrap_or("")
}

/// fixarray の i 番目の要素を f64 としてデコードする。
/// Decode the i-th element of a fixarray as an f64.
/// 不正値の場合は 0.0 を返す。Returns 0.0 on invalid data.
pub fn decode_float64_at(data: &[u8], index: usize) -> f64 {
    let off = arg_offset(data, index);
    if off < 0 { return 0.0; }
    let pos = off as usize;
    if pos >= data.len() { return 0.0; }
    match data[pos] {
        0xCA => {
            // float32
            if data.len() < pos + 5 { return 0.0; }
            let b: [u8; 4] = data[pos + 1..pos + 5].try_into().unwrap_or([0; 4]);
            f32::from_bits(u32::from_be_bytes(b)) as f64
        }
        0xCB => {
            // float64
            if data.len() < pos + 9 { return 0.0; }
            let b: [u8; 8] = data[pos + 1..pos + 9].try_into().unwrap_or([0; 8]);
            f64::from_bits(u64::from_be_bytes(b))
        }
        // int 系はキャスト変換で対応
        _ => decode_int(data, pos) as f64,
    }
}

// ------------------------------------------------------------------
// Private helper
// ------------------------------------------------------------------

/// 1 要素をスキップして次の先頭オフセットを返す。
/// Skip one element and return the offset of the next element.
fn skip_element(data: &[u8], pos: usize) -> i32 {
    if pos >= data.len() { return -1; }
    let b = data[pos];
    match b {
        0x00..=0x7F | 0xE0..=0xFF => (pos + 1) as i32,          // fixint
        0x80..=0x8F => {                                          // fixmap
            let n = (b & 0x0F) as usize;
            let mut p = pos + 1;
            for _ in 0..n * 2 {
                let next = skip_element(data, p);
                if next < 0 { return -1; }
                p = next as usize;
            }
            p as i32
        }
        0x90..=0x9F => {                                          // fixarray
            let n = (b & 0x0F) as usize;
            let mut p = pos + 1;
            for _ in 0..n {
                let next = skip_element(data, p);
                if next < 0 { return -1; }
                p = next as usize;
            }
            p as i32
        }
        0xA0..=0xBF => (pos + 1 + (b & 0x1F) as usize) as i32,  // fixstr
        0xC0 | 0xC2 | 0xC3 => (pos + 1) as i32,
        0xCA | 0xCE | 0xD2 => (pos + 5) as i32,  // float32 / uint32 / int32
        0xCB | 0xCF | 0xD3 => (pos + 9) as i32,  // float64 / uint64 / int64
        0xCC | 0xD0 => (pos + 2) as i32,
        0xCD | 0xD1 => (pos + 3) as i32,
        0xD9 => {
            let len = data.get(pos + 1).copied().unwrap_or(0) as usize;
            (pos + 2 + len) as i32
        }
        _ => -1,
    }
}

// ==================================================================
// Value 型定義 / Value type definition
// ==================================================================

/// MessagePack 値の汎用表現 / Generic representation of a MessagePack value.
///
/// Lua側から返される複雑な戻り値（List/Map）をデコードするための型。
/// Used to decode complex return values (List/Map) from Lua side.
///
/// ## 型安全な値アクセス / Type-safe value access
///
/// ```rust,no_run
/// use rust_computers_api::msgpack::Value;
///
/// fn process(val: &Value) {
///     // 型変換メソッド
///     if let Some(i) = val.as_i64()    { /* 整数 */ }
///     if let Some(f) = val.as_f64()    { /* 浮動小数点 */ }
///     if let Some(b) = val.as_bool()   { /* ブール */ }
///     if let Some(s) = val.as_str()    { /* 文字列参照 */ }
///     if let Some(a) = val.as_array()  { /* 配列参照 */ }
///     if let Some(m) = val.as_map()    { /* マップ参照 */ }
///
///     // マップから直接キーで取得
///     if let Some(v) = val.get("key")  { /* &Value */ }
///
///     // 配列から直接インデックスで取得
///     if let Some(v) = val.at(0)       { /* &Value */ }
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// nil 値
    Nil,
    /// ブール値
    Bool(bool),
    /// 整数値（i64 でラップ）
    Integer(i64),
    /// 浮動小数点値
    Float(f64),
    /// 文字列値
    String(alloc::string::String),
    /// バイナリデータ
    Binary(alloc::vec::Vec<u8>),
    /// 配列
    Array(alloc::vec::Vec<Value>),
    /// マップ（キーは常に文字列）
    Map(alloc::collections::BTreeMap<alloc::string::String, Value>),
}

impl Value {
    // ==================================================================
    // 型変換メソッド / Type accessor methods
    // ==================================================================

    /// 整数値として取得する。Integer の場合のみ Some を返す。
    /// Returns the value as i64 if it is an Integer.
    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        match self { Self::Integer(v) => Some(*v), _ => None }
    }

    /// i32 として取得する。Integer の場合のみ Some を返す（切り捨て）。
    /// Returns the value as i32 if it is an Integer (truncated).
    #[inline]
    pub fn as_i32(&self) -> Option<i32> {
        match self { Self::Integer(v) => Some(*v as i32), _ => None }
    }

    /// 浮動小数点値として取得する。Float の場合のみ Some を返す。
    /// Integer の場合は f64 にキャストして返す。
    /// Returns the value as f64. Float returns directly; Integer is cast.
    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float(v) => Some(*v),
            Self::Integer(v) => Some(*v as f64),
            _ => None,
        }
    }

    /// ブール値として取得する。Bool の場合のみ Some を返す。
    /// Returns the value as bool if it is a Bool.
    #[inline]
    pub fn as_bool(&self) -> Option<bool> {
        match self { Self::Bool(v) => Some(*v), _ => None }
    }

    /// 文字列参照として取得する。String の場合のみ Some を返す。
    /// Returns a string reference if the value is a String.
    #[inline]
    pub fn as_str(&self) -> Option<&str> {
        match self { Self::String(s) => Some(s.as_str()), _ => None }
    }

    /// 文字列を所有権付きで取得する。String の場合のみ Some を返す。
    /// Returns the owned string if the value is a String.
    #[inline]
    pub fn into_string(self) -> Option<alloc::string::String> {
        match self { Self::String(s) => Some(s), _ => None }
    }

    /// 配列参照として取得する。Array の場合のみ Some を返す。
    /// Returns an array reference if the value is an Array.
    #[inline]
    pub fn as_array(&self) -> Option<&alloc::vec::Vec<Value>> {
        match self { Self::Array(a) => Some(a), _ => None }
    }

    /// 配列を所有権付きで取得する。Array の場合のみ Some を返す。
    /// Returns the owned array if the value is an Array.
    #[inline]
    pub fn into_array(self) -> Option<alloc::vec::Vec<Value>> {
        match self { Self::Array(a) => Some(a), _ => None }
    }

    /// マップ参照として取得する。Map の場合のみ Some を返す。
    /// Returns a map reference if the value is a Map.
    #[inline]
    pub fn as_map(&self) -> Option<&alloc::collections::BTreeMap<alloc::string::String, Value>> {
        match self { Self::Map(m) => Some(m), _ => None }
    }

    /// マップを所有権付きで取得する。Map の場合のみ Some を返す。
    /// Returns the owned map if the value is a Map.
    #[inline]
    pub fn into_map(self) -> Option<alloc::collections::BTreeMap<alloc::string::String, Value>> {
        match self { Self::Map(m) => Some(m), _ => None }
    }

    /// バイナリ参照として取得する。Binary の場合のみ Some を返す。
    /// Returns a binary reference if the value is Binary.
    #[inline]
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self { Self::Binary(b) => Some(b.as_slice()), _ => None }
    }

    /// nil かどうかを返す。
    /// Returns true if the value is Nil.
    #[inline]
    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }

    // ==================================================================
    // コレクションアクセス / Collection access
    // ==================================================================

    /// Map のキーから値を取得する。Map でない場合は None を返す。
    /// Get a value by key from a Map. Returns None if not a Map.
    #[inline]
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Self::Map(m) => m.get(key),
            _ => None,
        }
    }

    /// Array のインデックスから値を取得する。Array でない場合は None を返す。
    /// Get a value by index from an Array. Returns None if not an Array.
    #[inline]
    pub fn at(&self, index: usize) -> Option<&Value> {
        match self {
            Self::Array(a) => a.get(index),
            _ => None,
        }
    }

    /// Array の長さを返す。Array でない場合は 0。
    /// Returns the length of an Array. Returns 0 if not an Array.
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Self::Array(a) => a.len(),
            Self::Map(m) => m.len(),
            _ => 0,
        }
    }

    /// コレクションが空かどうかを返す。
    /// Returns true if the collection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// MessagePack バイト列を Value にデコードする。
    /// Decode MessagePack bytes to a Value.
    ///
    /// # 戻り値 / Return value
    /// `(Value, next_offset)` のタプル。offset がデータ長と同じなら完全にデコード済み。
    /// Returns `(Value, next_offset)`. If next_offset == data.len(), fully decoded.
    pub fn decode(data: &[u8]) -> Option<(Self, usize)> {
        if data.is_empty() {
            return None;
        }
        Self::decode_at(data, 0).map(|(val, off)| (val, off as usize))
    }

    fn decode_at(data: &[u8], pos: usize) -> Option<(Self, i32)> {
        if pos >= data.len() {
            return None;
        }
        let b = data[pos];
        match b {
            // nil
            0xC0 => Some((Self::Nil, (pos + 1) as i32)),
            // bool
            0xC2 => Some((Self::Bool(false), (pos + 1) as i32)),
            0xC3 => Some((Self::Bool(true), (pos + 1) as i32)),
            // positive fixint (0x00-0x7F)
            0x00..=0x7F => Some((Self::Integer(b as i64), (pos + 1) as i32)),
            // negative fixint (0xE0-0xFF)
            0xE0..=0xFF => Some((Self::Integer((b as i8) as i64), (pos + 1) as i32)),
            // uint 8
            0xCC => {
                let val = *data.get(pos + 1)?;
                Some((Self::Integer(val as i64), (pos + 2) as i32))
            }
            // uint 16
            0xCD => {
                let hi = *data.get(pos + 1)? as u16;
                let lo = *data.get(pos + 2)? as u16;
                let val = (hi << 8) | lo;
                Some((Self::Integer(val as i64), (pos + 3) as i32))
            }
            // uint 32
            0xCE => {
                let a = *data.get(pos + 1)? as u32;
                let b2 = *data.get(pos + 2)? as u32;
                let c = *data.get(pos + 3)? as u32;
                let d = *data.get(pos + 4)? as u32;
                let val = (a << 24) | (b2 << 16) | (c << 8) | d;
                Some((Self::Integer(val as i64), (pos + 5) as i32))
            }
            // int 8
            0xD0 => {
                let val = *data.get(pos + 1)? as i8 as i64;
                Some((Self::Integer(val), (pos + 2) as i32))
            }
            // int 16
            0xD1 => {
                let hi = *data.get(pos + 1)? as i16;
                let lo = *data.get(pos + 2)? as i16;
                let val = (hi << 8) | lo;
                Some((Self::Integer(val as i64), (pos + 3) as i32))
            }
            // int 32
            0xD2 => {
                let a = *data.get(pos + 1)? as i32;
                let b2 = *data.get(pos + 2)? as i32;
                let c = *data.get(pos + 3)? as i32;
                let d = *data.get(pos + 4)? as i32;
                let val = (a << 24) | (b2 << 16) | (c << 8) | d;
                Some((Self::Integer(val as i64), (pos + 5) as i32))
            }
            // int 64
            0xD3 => {
                if data.len() < pos + 9 {
                    return None;
                }
                let mut val: i64 = 0;
                for i in 0..8 {
                    val = (val << 8) | (*data.get(pos + 1 + i)? as i64);
                }
                Some((Self::Integer(val), (pos + 9) as i32))
            }
            // float 32
            0xCA => {
                if data.len() < pos + 5 {
                    return None;
                }
                let mut bits: u32 = 0;
                for i in 0..4 {
                    bits = (bits << 8) | (*data.get(pos + 1 + i)? as u32);
                }
                let f = f32::from_bits(bits);
                Some((Self::Float(f as f64), (pos + 5) as i32))
            }
            // float 64
            0xCB => {
                if data.len() < pos + 9 {
                    return None;
                }
                let mut bits: u64 = 0;
                for i in 0..8 {
                    bits = (bits << 8) | (*data.get(pos + 1 + i)? as u64);
                }
                let f = f64::from_bits(bits);
                Some((Self::Float(f), (pos + 9) as i32))
            }
            // fixstr (0xA0-0xBF)
            0xA0..=0xBF => {
                let len = (b & 0x1F) as usize;
                let start = pos + 1;
                let end = start + len;
                if end > data.len() {
                    return None;
                }
                let s = alloc::string::String::from_utf8_lossy(&data[start..end]).into_owned();
                Some((Self::String(s), end as i32))
            }
            // str 8
            0xD9 => {
                let len = *data.get(pos + 1)? as usize;
                let start = pos + 2;
                let end = start + len;
                if end > data.len() {
                    return None;
                }
                let s = alloc::string::String::from_utf8_lossy(&data[start..end]).into_owned();
                Some((Self::String(s), end as i32))
            }
            // str 16
            0xDA => {
                let hi = *data.get(pos + 1)? as usize;
                let lo = *data.get(pos + 2)? as usize;
                let len = (hi << 8) | lo;
                let start = pos + 3;
                let end = start + len;
                if end > data.len() {
                    return None;
                }
                let s = alloc::string::String::from_utf8_lossy(&data[start..end]).into_owned();
                Some((Self::String(s), end as i32))
            }
            // bin 8
            0xC4 => {
                let len = *data.get(pos + 1)? as usize;
                let start = pos + 2;
                let end = start + len;
                if end > data.len() {
                    return None;
                }
                let bin = data[start..end].to_vec();
                Some((Self::Binary(bin), end as i32))
            }
            // bin 16
            0xC5 => {
                let hi = *data.get(pos + 1)? as usize;
                let lo = *data.get(pos + 2)? as usize;
                let len = (hi << 8) | lo;
                let start = pos + 3;
                let end = start + len;
                if end > data.len() {
                    return None;
                }
                let bin = data[start..end].to_vec();
                Some((Self::Binary(bin), end as i32))
            }
            // fixarray (0x90-0x9F)
            0x90..=0x9F => {
                let count = (b & 0x0F) as usize;
                let mut arr = Vec::new();
                let mut cur = (pos + 1) as i32;
                for _ in 0..count {
                    let (val, next) = Self::decode_at(data, cur as usize)?;
                    arr.push(val);
                    cur = next;
                }
                Some((Self::Array(arr), cur))
            }
            // array 16
            0xDC => {
                let hi = *data.get(pos + 1)? as usize;
                let lo = *data.get(pos + 2)? as usize;
                let count = (hi << 8) | lo;
                let mut arr = Vec::new();
                let mut cur = (pos + 3) as i32;
                for _ in 0..count {
                    let (val, next) = Self::decode_at(data, cur as usize)?;
                    arr.push(val);
                    cur = next;
                }
                Some((Self::Array(arr), cur))
            }
            // fixmap (0x80-0x8F)
            0x80..=0x8F => {
                let count = (b & 0x0F) as usize;
                let mut map = BTreeMap::new();
                let mut cur = (pos + 1) as i32;
                for _ in 0..count {
                    // key は常に String であると仮定
                    let (key_val, next_key) = Self::decode_at(data, cur as usize)?;
                    let key = match key_val {
                        Self::String(s) => s,
                        _ => return None,
                    };
                    cur = next_key;
                    let (val, next_val) = Self::decode_at(data, cur as usize)?;
                    map.insert(key, val);
                    cur = next_val;
                }
                Some((Self::Map(map), cur))
            }
            // map 16
            0xDE => {
                let hi = *data.get(pos + 1)? as usize;
                let lo = *data.get(pos + 2)? as usize;
                let count = (hi << 8) | lo;
                let mut map = BTreeMap::new();
                let mut cur = (pos + 3) as i32;
                for _ in 0..count {
                    let (key_val, next_key) = Self::decode_at(data, cur as usize)?;
                    let key = match key_val {
                        Self::String(s) => s,
                        _ => return None,
                    };
                    cur = next_key;
                    let (val, next_val) = Self::decode_at(data, cur as usize)?;
                    map.insert(key, val);
                    cur = next_val;
                }
                Some((Self::Map(map), cur))
            }
            _ => None,
        }
    }
}

// ==================================================================
// Display 実装 / Display implementation
// ==================================================================

impl core::fmt::Display for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Bool(v) => write!(f, "{}", v),
            Self::Integer(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{}", v),
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Binary(b) => write!(f, "<{} bytes>", b.len()),
            Self::Array(a) => {
                write!(f, "[")?;
                for (i, v) in a.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Self::Map(m) => {
                write!(f, "{{")?;
                for (i, (k, v)) in m.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "\"{}\": {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

// ==================================================================
// From 実装 / From implementations
// ==================================================================

impl From<i32> for Value {
    #[inline]
    fn from(v: i32) -> Self { Self::Integer(v as i64) }
}

impl From<i64> for Value {
    #[inline]
    fn from(v: i64) -> Self { Self::Integer(v) }
}

impl From<f64> for Value {
    #[inline]
    fn from(v: f64) -> Self { Self::Float(v) }
}

impl From<bool> for Value {
    #[inline]
    fn from(v: bool) -> Self { Self::Bool(v) }
}

impl From<&str> for Value {
    #[inline]
    fn from(v: &str) -> Self { Self::String(alloc::string::String::from(v)) }
}

impl From<alloc::string::String> for Value {
    #[inline]
    fn from(v: alloc::string::String) -> Self { Self::String(v) }
}

impl Default for Value {
    fn default() -> Self {
        Self::Nil
    }
}

/// Value を MessagePack バイト列にエンコードする。
/// Encode a Value to MessagePack bytes.
pub fn encode_value(val: &Value) -> Vec<u8> {
    match val {
        Value::Nil => nil(),
        Value::Bool(b) => bool_val(*b),
        Value::Integer(i) => int64(*i),
        Value::Float(f) => float64(*f),
        Value::String(s) => str(s),
        Value::Binary(b) => bytes(b),
        Value::Array(arr) => {
            let items: Vec<Vec<u8>> = arr.iter().map(encode_value).collect();
            array(&items)
        }
        Value::Map(map) => {
            let count = map.len();
            let mut out = Vec::new();
            if count <= 15 {
                out.push(0x80 | count as u8);
            } else {
                out.push(0xDE);
                out.push((count >> 8) as u8);
                out.push(count as u8);
            }
            for (k, v) in map {
                out.extend_from_slice(&str(k));
                out.extend_from_slice(&encode_value(v));
            }
            out
        }
    }
}
