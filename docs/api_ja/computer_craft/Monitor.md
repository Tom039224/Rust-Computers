# Monitor

**モジュール:** `computer_craft::monitor`  
**ペリフェラルタイプ:** `monitor`

CC:Tweaked のモニターペリフェラル。テキストやグラフィックの表示に使用します。通常モニターとアドバンスドモニターの両方に対応しています。

## Book-Read メソッド

### `book_next_set_text_scale` / `read_last_set_text_scale`
モニターのテキストスケールを設定します。
```rust
pub fn book_next_set_text_scale(&mut self, scale: MonitorTextScale) { ... }
pub fn read_last_set_text_scale(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `scale: MonitorTextScale` — テキストスケール値（0.5〜5.0、0.5刻み）

**戻り値:** `()`

---

### `book_next_get_text_scale` / `read_last_get_text_scale`
現在のテキストスケールを取得します。
```rust
pub fn book_next_get_text_scale(&mut self) { ... }
pub fn read_last_get_text_scale(&self) -> Result<MonitorTextScale, PeripheralError> { ... }
```
**戻り値:** `MonitorTextScale`

---

### `book_next_write` / `read_last_write`
現在のカーソル位置にテキストを書き込みます。
```rust
pub fn book_next_write(&mut self, text: &str) { ... }
pub fn read_last_write(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `text: &str` — 書き込むテキスト

**戻り値:** `()`

---

### `book_next_scroll` / `read_last_scroll`
画面内容を縦方向にスクロールします。
```rust
pub fn book_next_scroll(&mut self, y: u32) { ... }
pub fn read_last_scroll(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `y: u32` — スクロールする行数

**戻り値:** `()`

---

### `book_next_get_cursor_pos` / `read_last_get_cursor_pos`
現在のカーソル位置を取得します。
```rust
pub fn book_next_get_cursor_pos(&mut self) { ... }
pub fn read_last_get_cursor_pos(&self) -> Result<MonitorPosition, PeripheralError> { ... }
```
**戻り値:** `MonitorPosition { x, y }`

---

### `book_next_set_cursor_pos` / `read_last_set_cursor_pos`
カーソル位置を設定します。
```rust
pub fn book_next_set_cursor_pos(&mut self, pos: MonitorPosition) { ... }
pub fn read_last_set_cursor_pos(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `pos: MonitorPosition` — 移動先の位置 `{ x, y }`

**戻り値:** `()`

---

### `book_next_get_cursor_blink` / `read_last_get_cursor_blink`
カーソルの点滅状態を取得します。
```rust
pub fn book_next_get_cursor_blink(&mut self) { ... }
pub fn read_last_get_cursor_blink(&self) -> Result<bool, PeripheralError> { ... }
```
**戻り値:** `bool`

---

### `book_next_set_cursor_blink` / `read_last_set_cursor_blink`
カーソルの点滅状態を設定します。
```rust
pub fn book_next_set_cursor_blink(&mut self, blink: bool) { ... }
pub fn read_last_set_cursor_blink(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `blink: bool` — カーソルを点滅させるかどうか

**戻り値:** `()`

---

### `book_next_get_size` / `read_last_get_size`
モニターのサイズ（文字数）を取得します。
```rust
pub fn book_next_get_size(&mut self) { ... }
pub fn read_last_get_size(&self) -> Result<MonitorSize, PeripheralError> { ... }
```
**戻り値:** `MonitorSize { x, y }`

---

### `book_next_clear` / `read_last_clear`
画面全体をクリアします。
```rust
pub fn book_next_clear(&mut self) { ... }
pub fn read_last_clear(&self) -> Result<(), PeripheralError> { ... }
```
**戻り値:** `()`

---

### `book_next_clear_line` / `read_last_clear_line`
現在の行をクリアします。
```rust
pub fn book_next_clear_line(&mut self) { ... }
pub fn read_last_clear_line(&self) -> Result<(), PeripheralError> { ... }
```
**戻り値:** `()`

---

### `book_next_get_text_color` / `read_last_get_text_color`
現在のテキスト色を取得します。
```rust
pub fn book_next_get_text_color(&mut self) { ... }
pub fn read_last_get_text_color(&self) -> Result<MonitorColor, PeripheralError> { ... }
```
**戻り値:** `MonitorColor`

---

### `book_next_set_text_color` / `read_last_set_text_color`
テキスト色を設定します。
```rust
pub fn book_next_set_text_color(&mut self, color: MonitorColor) { ... }
pub fn read_last_set_text_color(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `color: MonitorColor` — テキスト色（RRGGBB形式）

**戻り値:** `()`

---

### `book_next_get_background_color` / `read_last_get_background_color`
現在の背景色を取得します。
```rust
pub fn book_next_get_background_color(&mut self) { ... }
pub fn read_last_get_background_color(&self) -> Result<MonitorColor, PeripheralError> { ... }
```
**戻り値:** `MonitorColor`

---

### `book_next_set_background_color` / `read_last_set_background_color`
背景色を設定します。
```rust
pub fn book_next_set_background_color(&mut self, color: MonitorColor) { ... }
pub fn read_last_set_background_color(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `color: MonitorColor` — 背景色（RRGGBB形式）

**戻り値:** `()`

---

### `book_next_blit` / `read_last_blit`
blit を使用して、テキスト色と背景色を指定して文字列を描画します。
```rust
pub fn book_next_blit(&mut self, text: &str, text_color: MonitorColor, background_color: MonitorColor) { ... }
pub fn read_last_blit(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `text: &str` — 描画するテキスト
- `text_color: MonitorColor` — テキスト色
- `background_color: MonitorColor` — 背景色

**戻り値:** `()`

## 即時メソッド

### `get_text_scale_imm`
テキストスケールを即時取得します。
```rust
pub fn get_text_scale_imm(&self) -> Result<MonitorTextScale, PeripheralError> { ... }
```

### `get_cursor_pos_imm`
カーソル位置を即時取得します。
```rust
pub fn get_cursor_pos_imm(&self) -> Result<MonitorPosition, PeripheralError> { ... }
```

### `get_cursor_blink_imm`
カーソル点滅状態を即時取得します。
```rust
pub fn get_cursor_blink_imm(&self) -> Result<bool, PeripheralError> { ... }
```

### `get_size_imm`
モニターサイズを即時取得します。
```rust
pub fn get_size_imm(&self) -> Result<MonitorSize, PeripheralError> { ... }
```

### `get_text_color_imm`
テキスト色を即時取得します。
```rust
pub fn get_text_color_imm(&self) -> Result<MonitorColor, PeripheralError> { ... }
```

### `get_background_color_imm`
背景色を即時取得します。
```rust
pub fn get_background_color_imm(&self) -> Result<MonitorColor, PeripheralError> { ... }
```

## 型定義

### `MonitorColor`
RRGGBB 形式のモニター色。
```rust
pub struct MonitorColor(pub u32);
```
**定義済み定数:** `WHITE`, `ORANGE`, `MAGENTA`, `LIGHT_BLUE`, `YELLOW`, `LIME`, `PINK`, `GRAY`, `LIGHT_GRAY`, `CYAN`, `PURPLE`, `BLUE`, `BROWN`, `GREEN`, `RED`, `BLACK`

**コンストラクタ:** `MonitorColor::rgb(r: u8, g: u8, b: u8) -> Self`

### `MonitorTextScale`
モニターのテキストスケール（0.5〜5.0、0.5刻み）。
```rust
pub struct MonitorTextScale(pub f32);
```
**定数:** `SIZE_0_5` 〜 `SIZE_5_0`

### `MonitorPosition`
```rust
pub struct MonitorPosition { pub x: u32, pub y: u32 }
```

### `MonitorSize`
```rust
pub struct MonitorSize { pub x: u32, pub y: u32 }
```

## 使用例

```rust
use rust_computers_api::computer_craft::monitor::*;
use rust_computers_api::peripheral::Peripheral;

let mut monitor = Monitor::find().unwrap();

// 画面をクリアしてテキストを書き込む
monitor.book_next_clear();
wait_for_next_tick().await;
let _ = monitor.read_last_clear();

monitor.book_next_set_cursor_pos(MonitorPosition { x: 1, y: 1 });
wait_for_next_tick().await;
let _ = monitor.read_last_set_cursor_pos();

monitor.book_next_set_text_color(MonitorColor::GREEN);
wait_for_next_tick().await;
let _ = monitor.read_last_set_text_color();

monitor.book_next_write("Hello, World!");
wait_for_next_tick().await;
let _ = monitor.read_last_write();

// モニターサイズを即時取得
let size = monitor.get_size_imm().unwrap();
```
