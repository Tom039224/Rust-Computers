//! CC:Tweaked Monitor ペリフェラル。
//! CC:Tweaked Monitor peripheral.

use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// モニター色 (RRGGBB)。
/// Monitor color in RRGGBB format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonitorColor(pub u32);

impl MonitorColor {
    // CC:Tweaked カラービットマスク定数 (colors.* Lua 値と同じ)。
    // CC:Tweaked color bitmask constants (same as `colors.*` in Lua).
    //
    // 各値は `1 << カラーインデックス(0-15)` であり、Java 側の
    // `colourFromBitmask()` が期待する形式です。
    // Each value is `1 << colorIndex(0-15)`, which is what Java's
    // `colourFromBitmask()` expects.
    pub const WHITE:      Self = Self(1);        // colors.white      (index  0)
    pub const ORANGE:     Self = Self(2);        // colors.orange     (index  1)
    pub const MAGENTA:    Self = Self(4);        // colors.magenta    (index  2)
    pub const LIGHT_BLUE: Self = Self(8);        // colors.lightBlue  (index  3)
    pub const YELLOW:     Self = Self(16);       // colors.yellow     (index  4)
    pub const LIME:       Self = Self(32);       // colors.lime       (index  5)
    pub const PINK:       Self = Self(64);       // colors.pink       (index  6)
    pub const GRAY:       Self = Self(128);      // colors.gray       (index  7)
    pub const LIGHT_GRAY: Self = Self(256);      // colors.lightGray  (index  8)
    pub const CYAN:       Self = Self(512);      // colors.cyan       (index  9)
    pub const PURPLE:     Self = Self(1024);     // colors.purple     (index 10)
    pub const BLUE:       Self = Self(2048);     // colors.blue       (index 11)
    pub const BROWN:      Self = Self(4096);     // colors.brown      (index 12)
    pub const GREEN:      Self = Self(8192);     // colors.green      (index 13)
    pub const RED:        Self = Self(16384);    // colors.red        (index 14)
    pub const BLACK:      Self = Self(32768);    // colors.black      (index 15)
}

/// モニターテキストスケール (0.5–5.0, 0.5 刻み)。
/// Monitor text scale (0.5–5.0, in 0.5 increments).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MonitorTextScale(pub f32);

impl MonitorTextScale {
    pub const SIZE_0_5: Self = Self(0.5);
    pub const SIZE_1_0: Self = Self(1.0);
    pub const SIZE_1_5: Self = Self(1.5);
    pub const SIZE_2_0: Self = Self(2.0);
    pub const SIZE_2_5: Self = Self(2.5);
    pub const SIZE_3_0: Self = Self(3.0);
    pub const SIZE_3_5: Self = Self(3.5);
    pub const SIZE_4_0: Self = Self(4.0);
    pub const SIZE_4_5: Self = Self(4.5);
    pub const SIZE_5_0: Self = Self(5.0);
}

/// モニター座標。
/// Monitor position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonitorPosition {
    pub x: u32,
    pub y: u32,
}

/// モニターサイズ。
/// Monitor size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonitorSize {
    pub x: u32,
    pub y: u32,
}

/// モニターペリフェラル（通常 / アドバンスド共通）。
/// Monitor peripheral (unified for normal and advanced).
pub struct Monitor {
    addr: PeriphAddr,
}

impl Peripheral for Monitor {
    const NAME: &'static str = "monitor";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl Monitor {
    /// テキストスケールを設定する。
    pub fn book_next_set_text_scale(&mut self, scale: MonitorTextScale) {
        let args = msgpack::array(&[msgpack::float64(scale.0 as f64)]);
        peripheral::book_action(self.addr, "setTextScale", &args);
    }
    pub fn read_last_set_text_scale(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setTextScale")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// テキストスケールを取得する。
    pub fn book_next_get_text_scale(&mut self) {
        peripheral::book_request(self.addr, "getTextScale", &msgpack::array(&[]));
    }
    pub fn read_last_get_text_scale(&self) -> Result<MonitorTextScale, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getTextScale")?;
        let v: f32 = peripheral::decode(&data)?;
        Ok(MonitorTextScale(v))
    }

    /// テキストスケールを即時取得する。
    pub fn get_text_scale_imm(&self) -> Result<MonitorTextScale, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getTextScale",
            &msgpack::array(&[]),
        )?;
        let v: f32 = peripheral::decode(&data)?;
        Ok(MonitorTextScale(v))
    }

    /// テキストを書き込む。
    pub fn book_next_write(&mut self, text: &str) {
        let args = msgpack::array(&[msgpack::str(text)]);
        peripheral::book_action(self.addr, "write", &args);
    }
    pub fn read_last_write(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "write")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 画面をスクロールする。
    pub fn book_next_scroll(&mut self, y: u32) {
        let args = msgpack::array(&[msgpack::int(y as i32)]);
        peripheral::book_action(self.addr, "scroll", &args);
    }
    pub fn read_last_scroll(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "scroll")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// カーソル位置を取得する。
    pub fn book_next_get_cursor_pos(&mut self) {
        peripheral::book_request(self.addr, "getCursorPos", &msgpack::array(&[]));
    }
    pub fn read_last_get_cursor_pos(&self) -> Result<MonitorPosition, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getCursorPos")?;
        let (x, y): (u32, u32) = peripheral::decode(&data)?;
        Ok(MonitorPosition { x, y })
    }

    /// カーソル位置を即時取得する。
    pub fn get_cursor_pos_imm(&self) -> Result<MonitorPosition, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getCursorPos",
            &msgpack::array(&[]),
        )?;
        let (x, y): (u32, u32) = peripheral::decode(&data)?;
        Ok(MonitorPosition { x, y })
    }

    /// カーソル位置を設定する。
    pub fn book_next_set_cursor_pos(&mut self, pos: MonitorPosition) {
        let args = msgpack::array(&[
            msgpack::int(pos.x as i32),
            msgpack::int(pos.y as i32),
        ]);
        peripheral::book_action(self.addr, "setCursorPos", &args);
    }
    pub fn read_last_set_cursor_pos(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setCursorPos")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// カーソル点滅状態を取得する。
    pub fn book_next_get_cursor_blink(&mut self) {
        peripheral::book_request(self.addr, "getCursorBlink", &msgpack::array(&[]));
    }
    pub fn read_last_get_cursor_blink(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getCursorBlink")?;
        peripheral::decode(&data)
    }

    /// カーソル点滅状態を即時取得する。
    pub fn get_cursor_blink_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getCursorBlink",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// カーソル点滅を設定する。
    pub fn book_next_set_cursor_blink(&mut self, blink: bool) {
        let args = msgpack::array(&[msgpack::bool_val(blink)]);
        peripheral::book_action(self.addr, "setCursorBlink", &args);
    }
    pub fn read_last_set_cursor_blink(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setCursorBlink")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// モニターサイズを取得する。
    pub fn book_next_get_size(&mut self) {
        peripheral::book_request(self.addr, "getSize", &msgpack::array(&[]));
    }
    pub fn read_last_get_size(&self) -> Result<MonitorSize, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getSize")?;
        let (x, y): (u32, u32) = peripheral::decode(&data)?;
        Ok(MonitorSize { x, y })
    }

    /// モニターサイズを即時取得する。
    pub fn get_size_imm(&self) -> Result<MonitorSize, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getSize",
            &msgpack::array(&[]),
        )?;
        let (x, y): (u32, u32) = peripheral::decode(&data)?;
        Ok(MonitorSize { x, y })
    }

    /// 画面をクリアする。
    pub fn book_next_clear(&mut self) {
        peripheral::book_action(self.addr, "clear", &msgpack::array(&[]));
    }
    pub fn read_last_clear(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "clear")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 現在行をクリアする。
    pub fn book_next_clear_line(&mut self) {
        peripheral::book_action(self.addr, "clearLine", &msgpack::array(&[]));
    }
    pub fn read_last_clear_line(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "clearLine")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// テキスト色を取得する。
    pub fn book_next_get_text_color(&mut self) {
        peripheral::book_request(self.addr, "getTextColour", &msgpack::array(&[]));
    }
    pub fn read_last_get_text_color(&self) -> Result<MonitorColor, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getTextColour")?;
        let v: u32 = peripheral::decode(&data)?;
        Ok(MonitorColor(v))
    }

    /// テキスト色を即時取得する。
    pub fn get_text_color_imm(&self) -> Result<MonitorColor, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getTextColour",
            &msgpack::array(&[]),
        )?;
        let v: u32 = peripheral::decode(&data)?;
        Ok(MonitorColor(v))
    }

    /// テキスト色を設定する。
    pub fn book_next_set_text_color(&mut self, color: MonitorColor) {
        let args = msgpack::array(&[msgpack::int(color.0 as i32)]);
        peripheral::book_action(self.addr, "setTextColour", &args);
    }
    pub fn read_last_set_text_color(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setTextColour")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 背景色を取得する。
    pub fn book_next_get_background_color(&mut self) {
        peripheral::book_request(self.addr, "getBackgroundColour", &msgpack::array(&[]));
    }
    pub fn read_last_get_background_color(&self) -> Result<MonitorColor, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getBackgroundColour")?;
        let v: u32 = peripheral::decode(&data)?;
        Ok(MonitorColor(v))
    }

    /// 背景色を即時取得する。
    pub fn get_background_color_imm(&self) -> Result<MonitorColor, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getBackgroundColour",
            &msgpack::array(&[]),
        )?;
        let v: u32 = peripheral::decode(&data)?;
        Ok(MonitorColor(v))
    }

    /// 背景色を設定する。
    pub fn book_next_set_background_color(&mut self, color: MonitorColor) {
        let args = msgpack::array(&[msgpack::int(color.0 as i32)]);
        peripheral::book_action(self.addr, "setBackgroundColour", &args);
    }
    pub fn read_last_set_background_color(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setBackgroundColour")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// blit で文字列を描画する。
    pub fn book_next_blit(&mut self, text: &str, text_color: MonitorColor, background_color: MonitorColor) {
        let args = msgpack::array(&[
            msgpack::str(text),
            msgpack::int(text_color.0 as i32),
            msgpack::int(background_color.0 as i32),
        ]);
        peripheral::book_action(self.addr, "blit", &args);
    }
    pub fn read_last_blit(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "blit")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }
}
