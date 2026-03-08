# Monitor

**Module:** `computer_craft::monitor`  
**Peripheral Type:** `monitor`

CC:Tweaked Monitor peripheral for displaying text and graphics. Supports both normal and advanced monitors.

## Book-Read Methods

### `book_next_set_text_scale` / `read_last_set_text_scale`
Set the text scale of the monitor.
```rust
pub fn book_next_set_text_scale(&mut self, scale: MonitorTextScale) { ... }
pub fn read_last_set_text_scale(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `scale: MonitorTextScale` — Text scale value (0.5–5.0 in 0.5 increments)

**Returns:** `()`

---

### `book_next_get_text_scale` / `read_last_get_text_scale`
Get the current text scale.
```rust
pub fn book_next_get_text_scale(&mut self) { ... }
pub fn read_last_get_text_scale(&self) -> Result<MonitorTextScale, PeripheralError> { ... }
```
**Returns:** `MonitorTextScale`

---

### `book_next_write` / `read_last_write`
Write text at the current cursor position.
```rust
pub fn book_next_write(&mut self, text: &str) { ... }
pub fn read_last_write(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `text: &str` — Text to write

**Returns:** `()`

---

### `book_next_scroll` / `read_last_scroll`
Scroll the screen contents vertically.
```rust
pub fn book_next_scroll(&mut self, y: u32) { ... }
pub fn read_last_scroll(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `y: u32` — Number of lines to scroll

**Returns:** `()`

---

### `book_next_get_cursor_pos` / `read_last_get_cursor_pos`
Get the current cursor position.
```rust
pub fn book_next_get_cursor_pos(&mut self) { ... }
pub fn read_last_get_cursor_pos(&self) -> Result<MonitorPosition, PeripheralError> { ... }
```
**Returns:** `MonitorPosition { x, y }`

---

### `book_next_set_cursor_pos` / `read_last_set_cursor_pos`
Set the cursor position.
```rust
pub fn book_next_set_cursor_pos(&mut self, pos: MonitorPosition) { ... }
pub fn read_last_set_cursor_pos(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `pos: MonitorPosition` — Target position `{ x, y }`

**Returns:** `()`

---

### `book_next_get_cursor_blink` / `read_last_get_cursor_blink`
Get the cursor blink state.
```rust
pub fn book_next_get_cursor_blink(&mut self) { ... }
pub fn read_last_get_cursor_blink(&self) -> Result<bool, PeripheralError> { ... }
```
**Returns:** `bool`

---

### `book_next_set_cursor_blink` / `read_last_set_cursor_blink`
Set the cursor blink state.
```rust
pub fn book_next_set_cursor_blink(&mut self, blink: bool) { ... }
pub fn read_last_set_cursor_blink(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `blink: bool` — Whether the cursor should blink

**Returns:** `()`

---

### `book_next_get_size` / `read_last_get_size`
Get the monitor size in characters.
```rust
pub fn book_next_get_size(&mut self) { ... }
pub fn read_last_get_size(&self) -> Result<MonitorSize, PeripheralError> { ... }
```
**Returns:** `MonitorSize { x, y }`

---

### `book_next_clear` / `read_last_clear`
Clear the entire screen.
```rust
pub fn book_next_clear(&mut self) { ... }
pub fn read_last_clear(&self) -> Result<(), PeripheralError> { ... }
```
**Returns:** `()`

---

### `book_next_clear_line` / `read_last_clear_line`
Clear the current line.
```rust
pub fn book_next_clear_line(&mut self) { ... }
pub fn read_last_clear_line(&self) -> Result<(), PeripheralError> { ... }
```
**Returns:** `()`

---

### `book_next_get_text_color` / `read_last_get_text_color`
Get the current text color.
```rust
pub fn book_next_get_text_color(&mut self) { ... }
pub fn read_last_get_text_color(&self) -> Result<MonitorColor, PeripheralError> { ... }
```
**Returns:** `MonitorColor`

---

### `book_next_set_text_color` / `read_last_set_text_color`
Set the text color.
```rust
pub fn book_next_set_text_color(&mut self, color: MonitorColor) { ... }
pub fn read_last_set_text_color(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `color: MonitorColor` — Text color (RRGGBB)

**Returns:** `()`

---

### `book_next_get_background_color` / `read_last_get_background_color`
Get the current background color.
```rust
pub fn book_next_get_background_color(&mut self) { ... }
pub fn read_last_get_background_color(&self) -> Result<MonitorColor, PeripheralError> { ... }
```
**Returns:** `MonitorColor`

---

### `book_next_set_background_color` / `read_last_set_background_color`
Set the background color.
```rust
pub fn book_next_set_background_color(&mut self, color: MonitorColor) { ... }
pub fn read_last_set_background_color(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `color: MonitorColor` — Background color (RRGGBB)

**Returns:** `()`

---

### `book_next_blit` / `read_last_blit`
Write a string with specified text and background colors using blit.
```rust
pub fn book_next_blit(&mut self, text: &str, text_color: MonitorColor, background_color: MonitorColor) { ... }
pub fn read_last_blit(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `text: &str` — Text to draw
- `text_color: MonitorColor` — Text color
- `background_color: MonitorColor` — Background color

**Returns:** `()`

## Immediate Methods

### `get_text_scale_imm`
Immediately get the current text scale.
```rust
pub fn get_text_scale_imm(&self) -> Result<MonitorTextScale, PeripheralError> { ... }
```

### `get_cursor_pos_imm`
Immediately get the current cursor position.
```rust
pub fn get_cursor_pos_imm(&self) -> Result<MonitorPosition, PeripheralError> { ... }
```

### `get_cursor_blink_imm`
Immediately get the cursor blink state.
```rust
pub fn get_cursor_blink_imm(&self) -> Result<bool, PeripheralError> { ... }
```

### `get_size_imm`
Immediately get the monitor size.
```rust
pub fn get_size_imm(&self) -> Result<MonitorSize, PeripheralError> { ... }
```

### `get_text_color_imm`
Immediately get the text color.
```rust
pub fn get_text_color_imm(&self) -> Result<MonitorColor, PeripheralError> { ... }
```

### `get_background_color_imm`
Immediately get the background color.
```rust
pub fn get_background_color_imm(&self) -> Result<MonitorColor, PeripheralError> { ... }
```

## Types

### `MonitorColor`
Monitor color in RRGGBB format.
```rust
pub struct MonitorColor(pub u32);
```
**Predefined constants:** `WHITE`, `ORANGE`, `MAGENTA`, `LIGHT_BLUE`, `YELLOW`, `LIME`, `PINK`, `GRAY`, `LIGHT_GRAY`, `CYAN`, `PURPLE`, `BLUE`, `BROWN`, `GREEN`, `RED`, `BLACK`

**Constructor:** `MonitorColor::rgb(r: u8, g: u8, b: u8) -> Self`

### `MonitorTextScale`
Monitor text scale (0.5–5.0, in 0.5 increments).
```rust
pub struct MonitorTextScale(pub f32);
```
**Constants:** `SIZE_0_5` through `SIZE_5_0`

### `MonitorPosition`
```rust
pub struct MonitorPosition { pub x: u32, pub y: u32 }
```

### `MonitorSize`
```rust
pub struct MonitorSize { pub x: u32, pub y: u32 }
```

## Usage Example

```rust
use rust_computers_api::computer_craft::monitor::*;
use rust_computers_api::peripheral::Peripheral;

let mut monitor = Monitor::find().unwrap();

// Clear and write text
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

// Immediate read of monitor size
let size = monitor.get_size_imm().unwrap();
```
