//! Toms-Peripherals GPU。

use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

/// GPU 画像データ。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TMImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u32>,
}

/// GPU ウィンドウ。
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TMWindow {
    pub x: f64,
    pub y: f64,
    pub width: u32,
    pub height: u32,
}

/// GPU ペリフェラル。
pub struct GPU {
    addr: PeriphAddr,
}

impl Peripheral for GPU {
    const NAME: &'static str = "tm:gpu";

    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }

    fn periph_addr(&self) -> PeriphAddr {
        self.addr
    }
}

impl GPU {
    /// ピクセルサイズを設定する。
    pub fn book_next_set_size(&mut self, pixels: u32) {
        let args = msgpack::array(&[msgpack::int(pixels as i32)]);
        peripheral::book_action(self.addr, "setSize", &args);
    }

    pub fn read_last_set_size(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setSize")?;
        Ok(())
    }

    /// サイズをリフレッシュする。
    pub fn book_next_refresh_size(&mut self) {
        peripheral::book_action(self.addr, "refreshSize", &msgpack::array(&[]));
    }

    pub fn read_last_refresh_size(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "refreshSize")?;
        Ok(())
    }

    /// サイズ情報を取得する (imm 対応)。
    /// Returns (pixel_width, height, monitor_cols, rows, pixel_size).
    pub fn book_next_get_size(&mut self) {
        peripheral::book_request(self.addr, "getSize", &msgpack::array(&[]));
    }

    pub fn read_last_get_size(&self) -> Result<(u32, u32, u32, u32, u32), PeripheralError> {
        let data = peripheral::read_result(self.addr, "getSize")?;
        peripheral::decode(&data)
    }

    pub fn get_size_imm(&self) -> Result<(u32, u32, u32, u32, u32), PeripheralError> {
        let data = peripheral::request_info_imm(
            self.addr,
            "getSize",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// RGBA で塗りつぶす。
    pub fn book_next_fill(&mut self, r: f32, g: f32, b: f32, a: f32) {
        let args = msgpack::array(&[
            msgpack::float64(r as f64),
            msgpack::float64(g as f64),
            msgpack::float64(b as f64),
            msgpack::float64(a as f64),
        ]);
        peripheral::book_action(self.addr, "fill", &args);
    }

    pub fn read_last_fill(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "fill")?;
        Ok(())
    }

    /// 画面を同期する。
    pub fn book_next_sync(&mut self) {
        peripheral::book_action(self.addr, "sync", &msgpack::array(&[]));
    }

    pub fn read_last_sync(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "sync")?;
        Ok(())
    }

    /// 塗りつぶし矩形を描画する。
    #[allow(clippy::too_many_arguments)]
    pub fn book_next_filled_rectangle(
        &mut self,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        let args = msgpack::array(&[
            msgpack::int(x as i32),
            msgpack::int(y as i32),
            msgpack::int(w as i32),
            msgpack::int(h as i32),
            msgpack::float64(r as f64),
            msgpack::float64(g as f64),
            msgpack::float64(b as f64),
            msgpack::float64(a as f64),
        ]);
        peripheral::book_action(self.addr, "filledRectangle", &args);
    }

    pub fn read_last_filled_rectangle(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "filledRectangle")?;
        Ok(())
    }

    /// 画像を描画する。
    pub fn book_next_draw_image(&mut self, image: &TMImage, x: u32, y: u32) {
        let img_encoded = peripheral::encode(image).expect("TMImage encode failed");
        let args = msgpack::array(&[
            img_encoded,
            msgpack::int(x as i32),
            msgpack::int(y as i32),
        ]);
        peripheral::book_action(self.addr, "drawImage", &args);
    }

    pub fn read_last_draw_image(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "drawImage")?;
        Ok(())
    }

    /// テキストを描画する。
    #[allow(clippy::too_many_arguments)]
    pub fn book_next_draw_text(
        &mut self,
        text: &str,
        x: u32,
        y: u32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        let args = msgpack::array(&[
            msgpack::str(text),
            msgpack::int(x as i32),
            msgpack::int(y as i32),
            msgpack::float64(r as f64),
            msgpack::float64(g as f64),
            msgpack::float64(b as f64),
            msgpack::float64(a as f64),
        ]);
        peripheral::book_action(self.addr, "drawText", &args);
    }

    pub fn read_last_draw_text(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "drawText")?;
        Ok(())
    }

    /// 文字を描画する。
    #[allow(clippy::too_many_arguments)]
    pub fn book_next_draw_char(
        &mut self,
        ch: char,
        x: u32,
        y: u32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        let mut buf = [0u8; 4];
        let s = ch.encode_utf8(&mut buf);
        let args = msgpack::array(&[
            msgpack::str(s),
            msgpack::int(x as i32),
            msgpack::int(y as i32),
            msgpack::float64(r as f64),
            msgpack::float64(g as f64),
            msgpack::float64(b as f64),
            msgpack::float64(a as f64),
        ]);
        peripheral::book_action(self.addr, "drawChar", &args);
    }

    pub fn read_last_draw_char(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "drawChar")?;
        Ok(())
    }

    /// テキストの描画長を取得する (imm 対応)。
    pub fn book_next_get_text_length(&mut self, text: &str) {
        let args = msgpack::array(&[msgpack::str(text)]);
        peripheral::book_request(self.addr, "getTextLength", &args);
    }

    pub fn read_last_get_text_length(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::read_result(self.addr, "getTextLength")?;
        peripheral::decode(&data)
    }

    pub fn get_text_length_imm(&self, text: &str) -> Result<u32, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(text)]);
        let data = peripheral::request_info_imm(self.addr, "getTextLength", &args)?;
        peripheral::decode(&data)
    }

    /// フォントを設定する。
    pub fn book_next_set_font(&mut self, font_name: &str) {
        let args = msgpack::array(&[msgpack::str(font_name)]);
        peripheral::book_action(self.addr, "setFont", &args);
    }

    pub fn read_last_set_font(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "setFont")?;
        Ok(())
    }

    /// カスタム文字をクリアする。
    pub fn book_next_clear_chars(&mut self) {
        peripheral::book_action(self.addr, "clearChars", &msgpack::array(&[]));
    }

    pub fn read_last_clear_chars(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "clearChars")?;
        Ok(())
    }

    /// カスタム文字を追加する。
    pub fn book_next_add_new_char(&mut self, codepoint: u32, data: &[u8]) {
        let args = msgpack::array(&[
            msgpack::int(codepoint as i32),
            msgpack::bytes(data),
        ]);
        peripheral::book_action(self.addr, "addNewChar", &args);
    }

    pub fn read_last_add_new_char(&self) -> Result<(), PeripheralError> {
        let _ = peripheral::read_result(self.addr, "addNewChar")?;
        Ok(())
    }

    /// ウィンドウを作成する (imm 対応)。
    pub fn book_next_create_window(&mut self, x: u32, y: u32, w: u32, h: u32) {
        let args = msgpack::array(&[
            msgpack::int(x as i32),
            msgpack::int(y as i32),
            msgpack::int(w as i32),
            msgpack::int(h as i32),
        ]);
        peripheral::book_request(self.addr, "createWindow", &args);
    }

    pub fn read_last_create_window(&self) -> Result<TMWindow, PeripheralError> {
        let data = peripheral::read_result(self.addr, "createWindow")?;
        peripheral::decode(&data)
    }

    pub fn create_window_imm(
        &self,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
    ) -> Result<TMWindow, PeripheralError> {
        let args = msgpack::array(&[
            msgpack::int(x as i32),
            msgpack::int(y as i32),
            msgpack::int(w as i32),
            msgpack::int(h as i32),
        ]);
        let data = peripheral::request_info_imm(self.addr, "createWindow", &args)?;
        peripheral::decode(&data)
    }

    /// Base64 文字列から画像をデコードする。
    pub fn book_next_decode_image(&mut self, data: &str) {
        let args = msgpack::array(&[msgpack::str(data)]);
        peripheral::book_request(self.addr, "decodeImage", &args);
    }

    pub fn read_last_decode_image(&self) -> Result<TMImage, PeripheralError> {
        let data = peripheral::read_result(self.addr, "decodeImage")?;
        peripheral::decode(&data)
    }

    /// 空の新規画像を作成する (imm 対応)。
    pub fn book_next_new_image(&mut self, w: u32, h: u32) {
        let args = msgpack::array(&[msgpack::int(w as i32), msgpack::int(h as i32)]);
        peripheral::book_request(self.addr, "newImage", &args);
    }

    pub fn read_last_new_image(&self) -> Result<TMImage, PeripheralError> {
        let data = peripheral::read_result(self.addr, "newImage")?;
        peripheral::decode(&data)
    }

    pub fn new_image_imm(&self, w: u32, h: u32) -> Result<TMImage, PeripheralError> {
        let args = msgpack::array(&[msgpack::int(w as i32), msgpack::int(h as i32)]);
        let data = peripheral::request_info_imm(self.addr, "newImage", &args)?;
        peripheral::decode(&data)
    }
}
