//! CC:Tweaked Monitor ペリフェラル。
//! CC:Tweaked Monitor peripheral.

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// モニター色 (RRGGBB)。
/// Monitor color in RRGGBB format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonitorColor(pub u32);

impl MonitorColor {
    pub const WHITE: Self = Self(0xF0F0F0);
    pub const ORANGE: Self = Self(0xF2B233);
    pub const MAGENTA: Self = Self(0xE57FD8);
    pub const LIGHT_BLUE: Self = Self(0x99B2F2);
    pub const YELLOW: Self = Self(0xDEDE6C);
    pub const LIME: Self = Self(0x7FCC19);
    pub const PINK: Self = Self(0xF2B2CC);
    pub const GRAY: Self = Self(0x4C4C4C);
    pub const LIGHT_GRAY: Self = Self(0x999999);
    pub const CYAN: Self = Self(0x4C99B2);
    pub const PURPLE: Self = Self(0xB266E5);
    pub const BLUE: Self = Self(0x3366CC);
    pub const BROWN: Self = Self(0x7F664C);
    pub const GREEN: Self = Self(0x57A64E);
    pub const RED: Self = Self(0xCC4C4C);
    pub const BLACK: Self = Self(0x111111);

    /// RGB 値からモニター色を作成する。
    /// Create a monitor color from RGB values.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }
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
    pub fn read_last_set_text_scale(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setTextScale")?;
        Ok(())
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
    pub fn read_last_write(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "write")?;
        Ok(())
    }

    /// 画面をスクロールする。
    pub fn book_next_scroll(&mut self, y: u32) {
        let args = msgpack::array(&[msgpack::int(y as i32)]);
        peripheral::book_action(self.addr, "scroll", &args);
    }
    pub fn read_last_scroll(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "scroll")?;
        Ok(())
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
    pub fn read_last_set_cursor_pos(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setCursorPos")?;
        Ok(())
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
    pub fn read_last_set_cursor_blink(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setCursorBlink")?;
        Ok(())
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
    pub fn read_last_clear(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "clear")?;
        Ok(())
    }

    /// 現在行をクリアする。
    pub fn book_next_clear_line(&mut self) {
        peripheral::book_action(self.addr, "clearLine", &msgpack::array(&[]));
    }
    pub fn read_last_clear_line(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "clearLine")?;
        Ok(())
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
    pub fn read_last_set_text_color(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setTextColour")?;
        Ok(())
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
    pub fn read_last_set_background_color(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setBackgroundColour")?;
        Ok(())
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
    pub fn read_last_blit(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "blit")?;
        Ok(())
    }
}
