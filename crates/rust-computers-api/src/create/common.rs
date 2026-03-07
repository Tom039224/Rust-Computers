//! Create mod 共通型定義。
//! Shared types for Create mod peripherals.

use alloc::collections::BTreeMap;
use alloc::string::String;

use serde::{Deserialize, Serialize};

/// アイテム詳細情報。
/// Detailed item information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CRItemDetail {
    pub name: String,
    pub count: u32,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(default)]
    pub tags: BTreeMap<String, bool>,
}

/// スロット情報。
/// Slot information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CRSlotInfo {
    pub name: String,
    pub count: u32,
}

/// 注文アイテム。
/// Order item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CROrderItem {
    pub name: String,
    pub count: u32,
}

/// アイテムフィルタ。
/// Item filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CRItemFilter {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub request_count: Option<u32>,
}

/// シグナルパラメータ。
/// Signal display parameters.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CRSignalParams {
    #[serde(default)]
    pub r: Option<u8>,
    #[serde(default)]
    pub g: Option<u8>,
    #[serde(default)]
    pub b: Option<u8>,
    #[serde(default)]
    pub glow_width: Option<u8>,
    #[serde(default)]
    pub glow_height: Option<u8>,
    #[serde(default)]
    pub blink_period: Option<u8>,
    #[serde(default)]
    pub blink_off_time: Option<u8>,
}

/// パッケージ情報。
/// Package information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CRPackage {
    pub address: String,
}
