//! Toms-Peripherals GPU。

use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, Direction, Peripheral};

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
    dir: Direction,
}

impl Peripheral for GPU {
    const NAME: &'static str = "tm:gpu";

    fn new(dir: Direction) -> Self {
        Self { dir }
    }

    fn direction(&self) -> Direction {
        self.dir
    }
}

impl GPU {
    /// ピクセルサイズを設定する。
    pub async fn set_size(&self, pixels: u32) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::int(pixels as i32)]);
        peripheral::do_action(self.dir, "setSize", &args).await?;
        Ok(())
    }

    /// サイズをリフレッシュする。
    pub async fn refresh_size(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.dir, "refreshSize", &msgpack::array(&[])).await?;
        Ok(())
    }

    /// サイズ情報を取得する (imm 対応)。
    /// Returns (pixel_width, height, monitor_cols, rows, pixel_size).
    pub async fn get_size(&self) -> Result<(u32, u32, u32, u32, u32), PeripheralError> {
        let data = peripheral::request_info(
            self.dir,
            "getSize",
            &msgpack::array(&[]),
        )
        .await?;
        peripheral::decode(&data)
    }

    pub fn get_size_imm(&self) -> Result<(u32, u32, u32, u32, u32), PeripheralError> {
        let data = peripheral::request_info_imm(
            self.dir,
            "getSize",
            &msgpack::array(&[]),
        )?;
        peripheral::decode(&data)
    }

    /// RGBA で塗りつぶす。
    pub async fn fill(
        &self,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[
            msgpack::float64(r as f64),
            msgpack::float64(g as f64),
            msgpack::float64(b as f64),
            msgpack::float64(a as f64),
        ]);
        peripheral::do_action(self.dir, "fill", &args).await?;
        Ok(())
    }

    /// 画面を同期する。
    pub async fn sync(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.dir, "sync", &msgpack::array(&[])).await?;
        Ok(())
    }

    /// 塗りつぶし矩形を描画する。
    #[allow(clippy::too_many_arguments)]
    pub async fn filled_rectangle(
        &self,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) -> Result<(), PeripheralError> {
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
        peripheral::do_action(self.dir, "filledRectangle", &args).await?;
        Ok(())
    }

    /// 画像を描画する。
    pub async fn draw_image(
        &self,
        image: &TMImage,
        x: u32,
        y: u32,
    ) -> Result<(), PeripheralError> {
        let img_encoded = peripheral::encode(image)?;
        let args = msgpack::array(&[
            img_encoded,
            msgpack::int(x as i32),
            msgpack::int(y as i32),
        ]);
        peripheral::do_action(self.dir, "drawImage", &args).await?;
        Ok(())
    }

    /// テキストを描画する。
    #[allow(clippy::too_many_arguments)]
    pub async fn draw_text(
        &self,
        text: &str,
        x: u32,
        y: u32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[
            msgpack::str(text),
            msgpack::int(x as i32),
            msgpack::int(y as i32),
            msgpack::float64(r as f64),
            msgpack::float64(g as f64),
            msgpack::float64(b as f64),
            msgpack::float64(a as f64),
        ]);
        peripheral::do_action(self.dir, "drawText", &args).await?;
        Ok(())
    }

    /// 文字を描画する。
    #[allow(clippy::too_many_arguments)]
    pub async fn draw_char(
        &self,
        ch: char,
        x: u32,
        y: u32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) -> Result<(), PeripheralError> {
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
        peripheral::do_action(self.dir, "drawChar", &args).await?;
        Ok(())
    }

    /// テキストの描画長を取得する (imm 対応)。
    pub async fn get_text_length(&self, text: &str) -> Result<u32, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(text)]);
        let data =
            peripheral::request_info(self.dir, "getTextLength", &args).await?;
        peripheral::decode(&data)
    }

    pub fn get_text_length_imm(&self, text: &str) -> Result<u32, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(text)]);
        let data = peripheral::request_info_imm(self.dir, "getTextLength", &args)?;
        peripheral::decode(&data)
    }

    /// フォントを設定する。
    pub async fn set_font(&self, font_name: &str) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[msgpack::str(font_name)]);
        peripheral::do_action(self.dir, "setFont", &args).await?;
        Ok(())
    }

    /// カスタム文字をクリアする。
    pub async fn clear_chars(&self) -> Result<(), PeripheralError> {
        peripheral::do_action(self.dir, "clearChars", &msgpack::array(&[])).await?;
        Ok(())
    }

    /// カスタム文字を追加する。
    pub async fn add_new_char(
        &self,
        codepoint: u32,
        data: &[u8],
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[
            msgpack::int(codepoint as i32),
            msgpack::bytes(data),
        ]);
        peripheral::do_action(self.dir, "addNewChar", &args).await?;
        Ok(())
    }

    /// ウィンドウを作成する (imm 対応)。
    pub async fn create_window(
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
        let data = peripheral::request_info(self.dir, "createWindow", &args).await?;
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
        let data = peripheral::request_info_imm(self.dir, "createWindow", &args)?;
        peripheral::decode(&data)
    }

    /// Base64 文字列から画像をデコードする。
    pub async fn decode_image(&self, data: &str) -> Result<TMImage, PeripheralError> {
        let args = msgpack::array(&[msgpack::str(data)]);
        let resp =
            peripheral::request_info(self.dir, "decodeImage", &args).await?;
        peripheral::decode(&resp)
    }

    /// 空の新規画像を作成する (imm 対応)。
    pub async fn new_image(&self, w: u32, h: u32) -> Result<TMImage, PeripheralError> {
        let args = msgpack::array(&[msgpack::int(w as i32), msgpack::int(h as i32)]);
        let data = peripheral::request_info(self.dir, "newImage", &args).await?;
        peripheral::decode(&data)
    }

    pub fn new_image_imm(&self, w: u32, h: u32) -> Result<TMImage, PeripheralError> {
        let args = msgpack::array(&[msgpack::int(w as i32), msgpack::int(h as i32)]);
        let data = peripheral::request_info_imm(self.dir, "newImage", &args)?;
        peripheral::decode(&data)
    }
}
