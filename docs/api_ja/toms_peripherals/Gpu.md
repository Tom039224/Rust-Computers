# GPU
**モジュール:** `toms_peripherals::gpu`  
**ペリフェラルタイプ:** `tm:gpu`

Tom's Peripherals の GPU ペリフェラル。接続されたモニターに対して、ピクセルレベルの描画、テキストレンダリング、画像処理、ウィンドウ管理、カスタムフォント・文字のサポートを提供します。

## 型定義

### `TMImage`
```rust
pub struct TMImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u32>,
}
```
RGBA `u32` 値のフラット配列としてピクセルデータを保持する GPU 画像。

### `TMWindow`
```rust
pub struct TMWindow {
    pub x: f64,
    pub y: f64,
    pub width: u32,
    pub height: u32,
}
```
GPU ウィンドウ領域を表す構造体。

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* / async_* methods

## Book-Read メソッド

### `book_next_set_size` / `read_last_set_size`
GPU ディスプレイのピクセルサイズを設定する。
```rust
pub fn book_next_set_size(&mut self, pixels: u32)
pub fn read_last_set_size(&self) -> Result<(), PeripheralError>
```
**パラメータ:** `pixels` — 設定するピクセルサイズ。  
**戻り値:** 成功時 `()`。

---

### `book_next_refresh_size` / `read_last_refresh_size`
GPU ディスプレイのサイズをリフレッシュする。
```rust
pub fn book_next_refresh_size(&mut self)
pub fn read_last_refresh_size(&self) -> Result<(), PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** 成功時 `()`。

---

### `book_next_get_size` / `read_last_get_size`
ディスプレイのサイズ情報を取得する。
```rust
pub fn book_next_get_size(&mut self)
pub fn read_last_get_size(&self) -> Result<(u32, u32, u32, u32, u32), PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** `(pixel_width, height, monitor_cols, rows, pixel_size)`。

---

### `book_next_fill` / `read_last_fill`
画面全体を RGBA カラーで塗りつぶす。
```rust
pub fn book_next_fill(&mut self, r: f32, g: f32, b: f32, a: f32)
pub fn read_last_fill(&self) -> Result<(), PeripheralError>
```
**パラメータ:** `r`, `g`, `b`, `a` — 色成分 (0.0–1.0)。  
**戻り値:** 成功時 `()`。

---

### `book_next_sync` / `read_last_sync`
ディスプレイバッファを同期する（保留中の描画操作を画面にフラッシュ）。
```rust
pub fn book_next_sync(&mut self)
pub fn read_last_sync(&self) -> Result<(), PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** 成功時 `()`。

---

### `book_next_filled_rectangle` / `read_last_filled_rectangle`
塗りつぶし矩形を描画する。
```rust
pub fn book_next_filled_rectangle(
    &mut self,
    x: u32, y: u32, w: u32, h: u32,
    r: f32, g: f32, b: f32, a: f32,
)
pub fn read_last_filled_rectangle(&self) -> Result<(), PeripheralError>
```
**パラメータ:** `x`, `y` — 左上座標; `w`, `h` — 幅と高さ; `r`, `g`, `b`, `a` — RGBA カラー。  
**戻り値:** 成功時 `()`。

---

### `book_next_draw_image` / `read_last_draw_image`
指定位置に画像を描画する。
```rust
pub fn book_next_draw_image(&mut self, image: &TMImage, x: u32, y: u32)
pub fn read_last_draw_image(&self) -> Result<(), PeripheralError>
```
**パラメータ:** `image` — 描画する `TMImage`; `x`, `y` — 描画位置。  
**戻り値:** 成功時 `()`。

---

### `book_next_draw_text` / `read_last_draw_text`
指定位置に RGBA カラーでテキストを描画する。
```rust
pub fn book_next_draw_text(
    &mut self,
    text: &str,
    x: u32, y: u32,
    r: f32, g: f32, b: f32, a: f32,
)
pub fn read_last_draw_text(&self) -> Result<(), PeripheralError>
```
**パラメータ:** `text` — 描画する文字列; `x`, `y` — 位置; `r`, `g`, `b`, `a` — カラー。  
**戻り値:** 成功時 `()`。

---

### `book_next_draw_char` / `read_last_draw_char`
指定位置に RGBA カラーで1文字を描画する。
```rust
pub fn book_next_draw_char(
    &mut self,
    ch: char,
    x: u32, y: u32,
    r: f32, g: f32, b: f32, a: f32,
)
pub fn read_last_draw_char(&self) -> Result<(), PeripheralError>
```
**パラメータ:** `ch` — 描画する文字; `x`, `y` — 位置; `r`, `g`, `b`, `a` — カラー。  
**戻り値:** 成功時 `()`。

---

### `book_next_get_text_length` / `read_last_get_text_length`
テキスト文字列の描画ピクセル長を取得する。
```rust
pub fn book_next_get_text_length(&mut self, text: &str)
pub fn read_last_get_text_length(&self) -> Result<u32, PeripheralError>
```
**パラメータ:** `text` — 計測するテキスト。  
**戻り値:** ピクセル長 (`u32`)。

---

### `book_next_set_font` / `read_last_set_font`
テキスト描画に使用するフォントを設定する。
```rust
pub fn book_next_set_font(&mut self, font_name: &str)
pub fn read_last_set_font(&self) -> Result<(), PeripheralError>
```
**パラメータ:** `font_name` — 使用するフォント名。  
**戻り値:** 成功時 `()`。

---

### `book_next_clear_chars` / `read_last_clear_chars`
すべてのカスタム文字をクリアする。
```rust
pub fn book_next_clear_chars(&mut self)
pub fn read_last_clear_chars(&self) -> Result<(), PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** 成功時 `()`。

---

### `book_next_add_new_char` / `read_last_add_new_char`
指定コードポイントとビットマップデータでカスタム文字を追加する。
```rust
pub fn book_next_add_new_char(&mut self, codepoint: u32, data: &[u8])
pub fn read_last_add_new_char(&self) -> Result<(), PeripheralError>
```
**パラメータ:** `codepoint` — Unicode コードポイント; `data` — 生のビットマップバイト列。  
**戻り値:** 成功時 `()`。

---

### `book_next_create_window` / `read_last_create_window`
GPU ウィンドウ領域を作成する。
```rust
pub fn book_next_create_window(&mut self, x: u32, y: u32, w: u32, h: u32)
pub fn read_last_create_window(&self) -> Result<TMWindow, PeripheralError>
```
**パラメータ:** `x`, `y` — 位置; `w`, `h` — 幅と高さ。  
**戻り値:** 作成されたウィンドウを表す `TMWindow`。

---

### `book_next_decode_image` / `read_last_decode_image`
Base64 エンコードされた文字列から画像をデコードする。
```rust
pub fn book_next_decode_image(&mut self, data: &str)
pub fn read_last_decode_image(&self) -> Result<TMImage, PeripheralError>
```
**パラメータ:** `data` — Base64 エンコードされた画像データ。  
**戻り値:** デコードされた `TMImage`。

---

### `book_next_new_image` / `read_last_new_image`
指定サイズの空の新規画像を作成する。
```rust
pub fn book_next_new_image(&mut self, w: u32, h: u32)
pub fn read_last_new_image(&self) -> Result<TMImage, PeripheralError>
```
**パラメータ:** `w`, `h` — 画像の幅と高さ。  
**戻り値:** 新しい空の `TMImage`。

## Immediate メソッド

以下のメソッドは book-read サイクルなしで即座に結果を返します:

### `get_size_imm`
```rust
pub fn get_size_imm(&self) -> Result<(u32, u32, u32, u32, u32), PeripheralError>
```
**戻り値:** `(pixel_width, height, monitor_cols, rows, pixel_size)`。

### `get_text_length_imm`
```rust
pub fn get_text_length_imm(&self, text: &str) -> Result<u32, PeripheralError>
```
**パラメータ:** `text` — 計測するテキスト。  
**戻り値:** ピクセル長 (`u32`)。

### `create_window_imm`
```rust
pub fn create_window_imm(&self, x: u32, y: u32, w: u32, h: u32) -> Result<TMWindow, PeripheralError>
```
**パラメータ:** `x`, `y` — 位置; `w`, `h` — 幅と高さ。  
**戻り値:** `TMWindow`。

### `new_image_imm`
```rust
pub fn new_image_imm(&self, w: u32, h: u32) -> Result<TMImage, PeripheralError>
```
**パラメータ:** `w`, `h` — 画像の幅と高さ。  
**戻り値:** 新しい空の `TMImage`。

## 使用例

```rust
use rust_computers_api::toms_peripherals::gpu::GPU;
use rust_computers_api::peripheral::Peripheral;

let mut gpu = GPU::find().expect("GPU が見つかりません");

// ピクセルサイズを設定し、背景を塗りつぶす
gpu.book_next_set_size(2);
gpu.book_next_fill(0.0, 0.0, 0.0, 1.0);

// 結果を待つ
gpu.read_last_set_size().unwrap();
gpu.read_last_fill().unwrap();

// テキストを描画
gpu.book_next_draw_text("Hello, GPU!", 10, 10, 1.0, 1.0, 1.0, 1.0);
gpu.read_last_draw_text().unwrap();

// ディスプレイを同期
gpu.book_next_sync();
gpu.read_last_sync().unwrap();

// Immediate コール — booking なしでサイズを取得
let (pw, ph, cols, rows, ps) = gpu.get_size_imm().unwrap();
```
