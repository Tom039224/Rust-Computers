//! Create Display Link ペリフェラル。
//! Create Display Link peripheral.

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// Display Link ペリフェラル。
pub struct DisplayLink {
    addr: PeriphAddr,
}

impl Peripheral for DisplayLink {
    const NAME: &'static str = "create:display_link";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl DisplayLink {
    /// カーソル位置を設定する。
    pub fn book_next_set_cursor_pos(&mut self, x: u32, y: u32) {
        let args = msgpack::array(&[msgpack::int(x as i32), msgpack::int(y as i32)]);
        peripheral::book_action(self.addr, "setCursorPos", &args);
    }

    pub fn read_last_set_cursor_pos(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setCursorPos")?;
        Ok(())
    }

    /// カーソル位置を取得する。
    pub fn book_next_get_cursor_pos(&mut self) {
        peripheral::book_request(
            self.addr,
            "getCursorPos",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_cursor_pos(&self) -> Result<(u32, u32), PeripheralError> {
        let data = peripheral::read_result(self.addr, "getCursorPos")?;
        peripheral::decode(&data)
    }

    /// カーソル位置を即時取得する (imm 対応)。
    pub fn get_cursor_pos_imm(&self) -> Result<(u32, u32), PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getCursorPos",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// ディスプレイのサイズを取得する (mainThread=true のため imm 非対応)。
    pub fn book_next_get_size(&mut self) {
        peripheral::book_request(
            self.addr,
            "getSize",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_get_size(&self) -> Result<(u32, u32), PeripheralError> {
        let data = peripheral::read_result(self.addr, "getSize")?;
        peripheral::decode(&data)
    }

    /// カラー対応かどうかを取得する。
    pub fn book_next_is_color(&mut self) {
        peripheral::book_request(
            self.addr,
            "isColour",
            &msgpack::array(&[]),
        );
    }

    pub fn read_last_is_color(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isColour")?;
        peripheral::decode(&data)
    }

    /// カラー対応かどうかを即時取得する (imm 対応)。
    pub fn is_color_imm(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "isColour",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
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

    /// バイト列を書き込む。
    pub fn book_next_write_bytes(&mut self, data: &[u8]) -> Result<(), PeripheralError> {
        let encoded = peripheral::encode(&data)?;
        let args = msgpack::array(&[encoded]);
        peripheral::book_action(self.addr, "writeBytes", &args);
        Ok(())
    }

    pub fn read_last_write_bytes(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "writeBytes")?;
        Ok(())
    }

    /// 現在の行をクリアする。
    pub fn book_next_clear_line(&mut self) {
        peripheral::book_action(self.addr, "clearLine", &msgpack::array(&[]));
    }

    pub fn read_last_clear_line(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "clearLine")?;
        Ok(())
    }

    /// ディスプレイ全体をクリアする。
    pub fn book_next_clear(&mut self) {
        peripheral::book_action(self.addr, "clear", &msgpack::array(&[]));
    }

    pub fn read_last_clear(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "clear")?;
        Ok(())
    }

    /// ディスプレイを更新する。
    pub fn book_next_update(&mut self) {
        peripheral::book_action(self.addr, "update", &msgpack::array(&[]));
    }

    pub fn read_last_update(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "update")?;
        Ok(())
    }
}
