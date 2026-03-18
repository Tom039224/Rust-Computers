//! Toms-Peripherals GPU。

use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::error::PeripheralError;
use crate::msgpack;
use crate::peripheral::{self, PeriphAddr, Peripheral};

// ---------------------------------------------------------------------------
// Color
// ---------------------------------------------------------------------------

/// GPU 描画用 RGBA カラー。各チャンネルは 0–255 の `u32`。
///
/// # 例
/// ```rust
/// let red   = GpuColor::new(0xFF, 0x00, 0x00, 0xFF);
/// let white = GpuColor::WHITE;
/// let semi  = GpuColor::new(0x00, 0xFF, 0x00, 0x80); // 半透明の緑
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GpuColor {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

impl GpuColor {
    /// 任意の RGBA 値からカラーを作成する。各値は 0–255 にクランプされる。
    #[inline]
    pub const fn new(r: u32, g: u32, b: u32, a: u32) -> Self {
        Self { r: clamp255(r), g: clamp255(g), b: clamp255(b), a: clamp255(a) }
    }

    /// 不透明な RGB カラーを作成する (alpha = 0xFF)。
    #[inline]
    pub const fn rgb(r: u32, g: u32, b: u32) -> Self {
        Self::new(r, g, b, 0xFF)
    }

    pub const WHITE:       Self = Self::rgb(0xFF, 0xFF, 0xFF);
    pub const BLACK:       Self = Self::rgb(0x00, 0x00, 0x00);
    pub const RED:         Self = Self::rgb(0xFF, 0x00, 0x00);
    pub const GREEN:       Self = Self::rgb(0x00, 0xFF, 0x00);
    pub const BLUE:        Self = Self::rgb(0x00, 0x00, 0xFF);
    pub const YELLOW:      Self = Self::rgb(0xFF, 0xFF, 0x00);
    pub const CYAN:        Self = Self::rgb(0x00, 0xFF, 0xFF);
    pub const MAGENTA:     Self = Self::rgb(0xFF, 0x00, 0xFF);
    pub const ORANGE:      Self = Self::rgb(0xFF, 0xA5, 0x00);
    pub const GRAY:        Self = Self::rgb(0x80, 0x80, 0x80);
    pub const LIGHT_GRAY:  Self = Self::rgb(0xC0, 0xC0, 0xC0);
    pub const DARK_GRAY:   Self = Self::rgb(0x40, 0x40, 0x40);
    pub const TRANSPARENT: Self = Self::new(0x00, 0x00, 0x00, 0x00);

    /// 0.0–1.0 の f32 タプルに変換する (GPU API 内部用)。
    #[inline]
    pub(crate) fn to_f32(self) -> (f32, f32, f32, f32) {
        (self.r as f32 / 255.0, self.g as f32 / 255.0,
         self.b as f32 / 255.0, self.a as f32 / 255.0)
    }
}

const fn clamp255(v: u32) -> u32 {
    if v > 255 { 255 } else { v }
}

/// GPU 画像データ。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TMImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u32>,
}

/// GPU ペリフェラル。
pub struct GPU {
    addr: PeriphAddr,
}

impl Peripheral for GPU {
    const NAME: &'static str = "tm_gpu";

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

    pub fn read_last_set_size(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setSize")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// サイズをリフレッシュする。
    pub fn book_next_refresh_size(&mut self) {
        peripheral::book_action(self.addr, "refreshSize", &msgpack::array(&[]));
    }

    pub fn read_last_refresh_size(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "refreshSize")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
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
    pub fn book_next_fill(&mut self, color: GpuColor) {
        let (r, g, b, a) = color.to_f32();
        let args = msgpack::array(&[
            msgpack::float64(r as f64),
            msgpack::float64(g as f64),
            msgpack::float64(b as f64),
            msgpack::float64(a as f64),
        ]);
        peripheral::book_action(self.addr, "fill", &args);
    }

    pub fn read_last_fill(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "fill")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 画面を同期する。
    pub fn book_next_sync(&mut self) {
        peripheral::book_action(self.addr, "sync", &msgpack::array(&[]));
    }

    pub fn read_last_sync(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "sync")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 塗りつぶし矩形を描画する。
    pub fn book_next_filled_rectangle(&mut self, x: u32, y: u32, w: u32, h: u32, color: GpuColor) {
        let (r, g, b, a) = color.to_f32();
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

    pub fn read_last_filled_rectangle(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "filledRectangle")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 画像を描画する。image_ref は newImage/decodeImage が返す参照文字列。
    pub fn book_next_draw_image(&mut self, image_ref: &str, x: u32, y: u32) {
        let args = msgpack::array(&[
            msgpack::str(image_ref),
            msgpack::int(x as i32),
            msgpack::int(y as i32),
        ]);
        peripheral::book_action(self.addr, "drawImage", &args);
    }

    pub fn read_last_draw_image(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "drawImage")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// テキストを描画する。
    pub fn book_next_draw_text(&mut self, text: &str, x: u32, y: u32, color: GpuColor) {
        let (r, g, b, a) = color.to_f32();
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

    pub fn read_last_draw_text(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "drawText")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// 文字を描画する。
    pub fn book_next_draw_char(&mut self, ch: char, x: u32, y: u32, color: GpuColor) {
        let (r, g, b, a) = color.to_f32();
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

    pub fn read_last_draw_char(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "drawChar")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
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

    pub fn read_last_set_font(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setFont")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// カスタム文字をクリアする。
    pub fn book_next_clear_chars(&mut self) {
        peripheral::book_action(self.addr, "clearChars", &msgpack::array(&[]));
    }

    pub fn read_last_clear_chars(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "clearChars")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// カスタム文字を追加する。
    pub fn book_next_add_new_char(&mut self, codepoint: u32, data: &[u8]) {
        let args = msgpack::array(&[
            msgpack::int(codepoint as i32),
            msgpack::bytes(data),
        ]);
        peripheral::book_action(self.addr, "addNewChar", &args);
    }

    pub fn read_last_add_new_char(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "addNewChar")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    /// ウィンドウを作成する (imm 対応)。
    /// 戻り値は BaseGPU オブジェクト（TMLuaObject）。現在は未使用のため () を返す。
    pub fn book_next_create_window(&mut self, x: u32, y: u32, w: u32, h: u32) {
        let args = msgpack::array(&[
            msgpack::int(x as i32),
            msgpack::int(y as i32),
            msgpack::int(w as i32),
            msgpack::int(h as i32),
        ]);
        peripheral::book_request(self.addr, "createWindow", &args);
    }

    pub fn read_last_create_window(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "createWindow")
            .into_iter()
            .map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge))
            .collect()
    }

    pub fn create_window_imm(
        &self,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
    ) -> Result<(), PeripheralError> {
        let args = msgpack::array(&[
            msgpack::int(x as i32),
            msgpack::int(y as i32),
            msgpack::int(w as i32),
            msgpack::int(h as i32),
        ]);
        peripheral::request_info_imm(self.addr, "createWindow", &args)?;
        Ok(())
    }

    /// Base64 文字列から画像をデコードする。
    pub fn book_next_decode_image(&mut self, data: &str) {
        let args = msgpack::array(&[msgpack::str(data)]);
        peripheral::book_request(self.addr, "decodeImage", &args);
    }

    pub fn read_last_decode_image(&self) -> Result<alloc::string::String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "decodeImage")?;
        peripheral::decode(&data)
    }

    /// 空の新規画像を作成する (imm 対応)。
    /// 戻り値は画像への参照文字列。drawImage に渡して使用する。
    pub fn book_next_new_image(&mut self, w: u32, h: u32) {
        let args = msgpack::array(&[msgpack::int(w as i32), msgpack::int(h as i32)]);
        peripheral::book_request(self.addr, "newImage", &args);
    }

    pub fn read_last_new_image(&self) -> Result<alloc::string::String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "newImage")?;
        peripheral::decode(&data)
    }

    pub fn new_image_imm(&self, w: u32, h: u32) -> Result<alloc::string::String, PeripheralError> {
        let args = msgpack::array(&[msgpack::int(w as i32), msgpack::int(h as i32)]);
        let data = peripheral::request_info_imm(self.addr, "newImage", &args)?;
        peripheral::decode(&data)
    }
}

impl GPU {
    pub async fn async_set_size(&mut self, pixels: u32) -> Vec<Result<(), PeripheralError>> {
        self.book_next_set_size(pixels);
        crate::wait_for_next_tick().await;
        self.read_last_set_size()
    }

    pub async fn async_refresh_size(&mut self) -> Vec<Result<(), PeripheralError>> {
        self.book_next_refresh_size();
        crate::wait_for_next_tick().await;
        self.read_last_refresh_size()
    }

    pub async fn async_get_size(&mut self) -> Result<(u32, u32, u32, u32, u32), PeripheralError> {
        self.book_next_get_size();
        crate::wait_for_next_tick().await;
        self.read_last_get_size()
    }

    pub async fn async_fill(&mut self, color: GpuColor) -> Vec<Result<(), PeripheralError>> {
        self.book_next_fill(color);
        crate::wait_for_next_tick().await;
        self.read_last_fill()
    }

    pub async fn async_sync(&mut self) -> Vec<Result<(), PeripheralError>> {
        self.book_next_sync();
        crate::wait_for_next_tick().await;
        self.read_last_sync()
    }

    pub async fn async_filled_rectangle(&mut self, x: u32, y: u32, w: u32, h: u32, color: GpuColor) -> Vec<Result<(), PeripheralError>> {
        self.book_next_filled_rectangle(x, y, w, h, color);
        crate::wait_for_next_tick().await;
        self.read_last_filled_rectangle()
    }

    pub async fn async_draw_image(&mut self, image_ref: &str, x: u32, y: u32) -> Vec<Result<(), PeripheralError>> {
        self.book_next_draw_image(image_ref, x, y);
        crate::wait_for_next_tick().await;
        self.read_last_draw_image()
    }

    pub async fn async_draw_text(&mut self, text: &str, x: u32, y: u32, color: GpuColor) -> Vec<Result<(), PeripheralError>> {
        self.book_next_draw_text(text, x, y, color);
        crate::wait_for_next_tick().await;
        self.read_last_draw_text()
    }

    pub async fn async_draw_char(&mut self, ch: char, x: u32, y: u32, color: GpuColor) -> Vec<Result<(), PeripheralError>> {
        self.book_next_draw_char(ch, x, y, color);
        crate::wait_for_next_tick().await;
        self.read_last_draw_char()
    }

    pub async fn async_get_text_length(&mut self, text: &str) -> Result<u32, PeripheralError> {
        self.book_next_get_text_length(text);
        crate::wait_for_next_tick().await;
        self.read_last_get_text_length()
    }

    pub async fn async_set_font(&mut self, font_name: &str) -> Vec<Result<(), PeripheralError>> {
        self.book_next_set_font(font_name);
        crate::wait_for_next_tick().await;
        self.read_last_set_font()
    }

    pub async fn async_clear_chars(&mut self) -> Vec<Result<(), PeripheralError>> {
        self.book_next_clear_chars();
        crate::wait_for_next_tick().await;
        self.read_last_clear_chars()
    }

    pub async fn async_add_new_char(&mut self, codepoint: u32, data: &[u8]) -> Vec<Result<(), PeripheralError>> {
        self.book_next_add_new_char(codepoint, data);
        crate::wait_for_next_tick().await;
        self.read_last_add_new_char()
    }

    pub async fn async_create_window(&mut self, x: u32, y: u32, w: u32, h: u32) -> Vec<Result<(), PeripheralError>> {
        self.book_next_create_window(x, y, w, h);
        crate::wait_for_next_tick().await;
        self.read_last_create_window()
    }

    pub async fn async_decode_image(&mut self, data: &str) -> Result<alloc::string::String, PeripheralError> {
        self.book_next_decode_image(data);
        crate::wait_for_next_tick().await;
        self.read_last_decode_image()
    }

    pub async fn async_new_image(&mut self, w: u32, h: u32) -> Result<alloc::string::String, PeripheralError> {
        self.book_next_new_image(w, h);
        crate::wait_for_next_tick().await;
        self.read_last_new_image()
    }
}

// ---------------------------------------------------------------------------
// Sync-only helpers (book + read pattern for newly added methods)
// ---------------------------------------------------------------------------

impl GPU {
    // --- rectangle ---
    pub fn book_next_rectangle(&mut self, x: u32, y: u32, w: u32, h: u32, color: GpuColor) {
        let (r, g, b, a) = color.to_f32();
        let args = msgpack::array(&[
            msgpack::int(x as i32), msgpack::int(y as i32),
            msgpack::int(w as i32), msgpack::int(h as i32),
            msgpack::float64(r as f64), msgpack::float64(g as f64),
            msgpack::float64(b as f64), msgpack::float64(a as f64),
        ]);
        peripheral::book_action(self.addr, "rectangle", &args);
    }
    pub fn read_last_rectangle(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "rectangle")
            .into_iter().map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge)).collect()
    }

    // --- line ---
    pub fn book_next_line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, color: GpuColor) {
        let (r, g, b, a) = color.to_f32();
        let args = msgpack::array(&[
            msgpack::int(x1 as i32), msgpack::int(y1 as i32),
            msgpack::int(x2 as i32), msgpack::int(y2 as i32),
            msgpack::float64(r as f64), msgpack::float64(g as f64),
            msgpack::float64(b as f64), msgpack::float64(a as f64),
        ]);
        peripheral::book_action(self.addr, "line", &args);
    }
    pub fn read_last_line(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "line")
            .into_iter().map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge)).collect()
    }

    // --- lineS (smooth line) ---
    pub fn book_next_line_s(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, color: GpuColor) {
        let (r, g, b, a) = color.to_f32();
        let args = msgpack::array(&[
            msgpack::int(x1 as i32), msgpack::int(y1 as i32),
            msgpack::int(x2 as i32), msgpack::int(y2 as i32),
            msgpack::float64(r as f64), msgpack::float64(g as f64),
            msgpack::float64(b as f64), msgpack::float64(a as f64),
        ]);
        peripheral::book_action(self.addr, "lineS", &args);
    }
    pub fn read_last_line_s(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "lineS")
            .into_iter().map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge)).collect()
    }

    // --- drawTextSmart ---
    pub fn book_next_draw_text_smart(&mut self, text: &str, x: u32, y: u32, color: GpuColor) {
        let (r, g, b, a) = color.to_f32();
        let args = msgpack::array(&[
            msgpack::str(text),
            msgpack::int(x as i32), msgpack::int(y as i32),
            msgpack::float64(r as f64), msgpack::float64(g as f64),
            msgpack::float64(b as f64), msgpack::float64(a as f64),
        ]);
        peripheral::book_action(self.addr, "drawTextSmart", &args);
    }
    pub fn read_last_draw_text_smart(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "drawTextSmart")
            .into_iter().map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge)).collect()
    }

    // --- drawBuffer ---
    /// x, y, w, scale, data (raw pixel ints)
    pub fn book_next_draw_buffer(&mut self, x: u32, y: u32, w: u32, scale: u32, data: &[u32]) {
        let mut elems: alloc::vec::Vec<alloc::vec::Vec<u8>> = alloc::vec::Vec::new();
        elems.push(msgpack::int(x as i32));
        elems.push(msgpack::int(y as i32));
        elems.push(msgpack::int(w as i32));
        elems.push(msgpack::int(scale as i32));
        for &px in data {
            elems.push(msgpack::int(px as i32));
        }
        peripheral::book_action(self.addr, "drawBuffer", &msgpack::array(&elems));
    }
    pub fn read_last_draw_buffer(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "drawBuffer")
            .into_iter().map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge)).collect()
    }

    // --- imageFromBuffer ---
    pub fn book_next_image_from_buffer(&mut self, buf_ref: &str, w: u32, h: u32) {
        let args = msgpack::array(&[
            msgpack::str(buf_ref),
            msgpack::int(w as i32),
            msgpack::int(h as i32),
        ]);
        peripheral::book_request(self.addr, "imageFromBuffer", &args);
    }
    pub fn read_last_image_from_buffer(&self) -> Result<alloc::string::String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "imageFromBuffer")?;
        peripheral::decode(&data)
    }

    // --- newBuffer ---
    pub fn book_next_new_buffer(&mut self, size: u32) {
        let args = msgpack::array(&[msgpack::int(size as i32)]);
        peripheral::book_request(self.addr, "newBuffer", &args);
    }
    pub fn read_last_new_buffer(&self) -> Result<alloc::string::String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "newBuffer")?;
        peripheral::decode(&data)
    }

    // --- getFont ---
    pub fn get_font_imm(&self) -> Result<alloc::string::String, PeripheralError> {
        let data = peripheral::request_info_imm(self.addr, "getFont", &msgpack::array(&[]))?;
        peripheral::decode(&data)
    }

    // --- getFontDefaultCharID ---
    pub fn get_font_default_char_id_imm(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::request_info_imm(self.addr, "getFontDefaultCharID", &msgpack::array(&[]))?;
        peripheral::decode(&data)
    }

    // --- setFontDefaultCharID ---
    pub fn book_next_set_font_default_char_id(&mut self, id: u32) {
        let args = msgpack::array(&[msgpack::int(id as i32)]);
        peripheral::book_action(self.addr, "setFontDefaultCharID", &args);
    }
    pub fn read_last_set_font_default_char_id(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "setFontDefaultCharID")
            .into_iter().map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge)).collect()
    }

    // --- delChar ---
    pub fn book_next_del_char(&mut self, ch: char) {
        let mut buf = [0u8; 4];
        let s = ch.encode_utf8(&mut buf);
        let args = msgpack::array(&[msgpack::str(s)]);
        peripheral::book_action(self.addr, "delChar", &args);
    }
    pub fn read_last_del_char(&self) -> Vec<Result<(), PeripheralError>> {
        peripheral::read_action_results(self.addr, "delChar")
            .into_iter().map(|r| r.map(|_| ()).map_err(PeripheralError::Bridge)).collect()
    }

    // --- freeChars ---
    pub fn free_chars_imm(&self) -> Result<u32, PeripheralError> {
        let data = peripheral::request_info_imm(self.addr, "freeChars", &msgpack::array(&[]))?;
        peripheral::decode(&data)
    }

    // --- getUsedMemory / getMaxMemory ---
    pub fn get_used_memory_imm(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::request_info_imm(self.addr, "getUsedMemory", &msgpack::array(&[]))?;
        peripheral::decode(&data)
    }
    pub fn get_max_memory_imm(&self) -> Result<f64, PeripheralError> {
        let data = peripheral::request_info_imm(self.addr, "getMaxMemory", &msgpack::array(&[]))?;
        peripheral::decode(&data)
    }

    // --- getBounds ---
    pub fn get_bounds_imm(&self) -> Result<crate::msgpack::Value, PeripheralError> {
        let data = peripheral::request_info_imm(self.addr, "getBounds", &msgpack::array(&[]))?;
        peripheral::decode(&data)
    }

    // --- createWindow3D ---
    /// Returns a ref string for the GPU3D window object.
    pub fn book_next_create_window_3d(&mut self, x: u32, y: u32, w: u32, h: u32) {
        let args = msgpack::array(&[
            msgpack::int(x as i32), msgpack::int(y as i32),
            msgpack::int(w as i32), msgpack::int(h as i32),
        ]);
        peripheral::book_request(self.addr, "createWindow3D", &args);
    }
    pub fn read_last_create_window_3d(&self) -> Result<alloc::string::String, PeripheralError> {
        let data = peripheral::read_result(self.addr, "createWindow3D")?;
        peripheral::decode(&data)
    }
    pub fn create_window_3d_imm(&self, x: u32, y: u32, w: u32, h: u32) -> Result<alloc::string::String, PeripheralError> {
        let args = msgpack::array(&[
            msgpack::int(x as i32), msgpack::int(y as i32),
            msgpack::int(w as i32), msgpack::int(h as i32),
        ]);
        let data = peripheral::request_info_imm(self.addr, "createWindow3D", &args)?;
        peripheral::decode(&data)
    }
}
