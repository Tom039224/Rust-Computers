//! 最小限の MessagePack エンコーダ。
//! Minimal MessagePack encoder for peripheral argument encoding.
//!
//! # サポートする型 / Supported types
//! - `nil()` → `0xC0`
//! - `bool_val(bool)` → `0xC2` / `0xC3`
//! - `int(i32)` → positive fixint / negative fixint / uint8 / uint16 / int32
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
        0xCC | 0xD0 => (pos + 2) as i32,
        0xCD | 0xD1 => (pos + 3) as i32,
        0xCE | 0xD2 => (pos + 5) as i32,
        0xCF | 0xD3 => (pos + 9) as i32,
        0xD9 => {
            let len = data.get(pos + 1).copied().unwrap_or(0) as usize;
            (pos + 2 + len) as i32
        }
        _ => -1,
    }
}
