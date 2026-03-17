# Monitor

**Mod:** CC:Tweaked  
**Peripheral Type:** `monitor`  
**Source:** `MonitorPeripheral.java`, `TermMethods.java`

## Overview

The Monitor peripheral provides a terminal display that can be placed in the world. It supports text rendering, color manipulation, and touch input (on advanced monitors). Monitors inherit all methods from the Terminal API and add monitor-specific methods for text scaling.

## Three-Function Pattern

The Monitor API uses the three-function pattern for all methods:

1. **`book_next_*`** - Schedule a request for the next tick
2. **`read_last_*`** - Read the result from the previous tick
3. **`async_*`** - Convenience method that books, waits, and reads in one call

### Pattern Explanation

```rust
// Rust example to be added
```
## Monitor Types

- **Normal Monitor** — Monochrome display
- **Advanced Monitor** — Color display with touch support

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods
- *_imm() methods (immediate execution variants)
- poll_touch() (async) - for touch events
- book_next_try_poll_touch / read_last_try_poll_touch

### 🚧 Not Yet Implemented

- async_* variants for all methods (except poll_touch)
- monitor_resize event


## Methods

### `setTextScale(scale)` / `book_next_set_text_scale(scale)` / `read_last_set_text_scale()` / `async_set_text_scale(scale)`

Set the text scale of the monitor.

**Rust Signatures:**
```rust
pub fn book_next_set_text_scale(&mut self, scale: f32)
pub fn read_last_set_text_scale(&self) -> Result<(), PeripheralError>
pub async fn async_set_text_scale(&self, scale: f32) -> Result<(), PeripheralError>
```

**Parameters:**
- `scale: number` — Text scale (0.5–5.0 in 0.5 increments)

**Returns:** `nil`

**Notes:**
- Larger scales make text bigger but reduce resolution
- Valid values: 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0

**Example:**
```rust
// Rust example to be added
```
---

### `getTextScale()` / `book_next_get_text_scale()` / `read_last_get_text_scale()` / `async_get_text_scale()`

Get the current text scale.

**Rust Signatures:**
```rust
pub fn book_next_get_text_scale(&mut self)
pub fn read_last_get_text_scale(&self) -> Result<f32, PeripheralError>
pub async fn async_get_text_scale(&self) -> Result<f32, PeripheralError>
```

**Returns:** `number` — Current text scale

**Example:**
```rust
// Rust example to be added
```
---

### `write(text)` / `book_next_write(text)` / `read_last_write()` / `async_write(text)`

Write text at the current cursor position.

**Rust Signatures:**
```rust
pub fn book_next_write(&mut self, text: &str)
pub fn read_last_write(&self) -> Result<(), PeripheralError>
pub async fn async_write(&self, text: &str) -> Result<(), PeripheralError>
```

**Parameters:**
- `text: string` — Text to write

**Returns:** `nil`

**Example:**
```rust
// Rust example to be added
```
---

### `clear()` / `book_next_clear()` / `read_last_clear()` / `async_clear()`

Clear the entire monitor screen.

**Rust Signatures:**
```rust
pub fn book_next_clear(&mut self)
pub fn read_last_clear(&self) -> Result<(), PeripheralError>
pub async fn async_clear(&self) -> Result<(), PeripheralError>
```

**Returns:** `nil`

**Example:**
```rust
// Rust example to be added
```
---

### `clearLine()` / `book_next_clear_line()` / `read_last_clear_line()` / `async_clear_line()`

Clear the current line.

**Rust Signatures:**
```rust
pub fn book_next_clear_line(&mut self)
pub fn read_last_clear_line(&self) -> Result<(), PeripheralError>
pub async fn async_clear_line(&self) -> Result<(), PeripheralError>
```

**Returns:** `nil`

---

### `scroll(lines)` / `book_next_scroll(lines)` / `read_last_scroll()` / `async_scroll(lines)`

Scroll the screen vertically.

**Rust Signatures:**
```rust
pub fn book_next_scroll(&mut self, lines: u32)
pub fn read_last_scroll(&self) -> Result<(), PeripheralError>
pub async fn async_scroll(&self, lines: u32) -> Result<(), PeripheralError>
```

**Parameters:**
- `lines: number` — Number of lines to scroll (positive = down, negative = up)

**Returns:** `nil`

---

### `getCursorPos()` / `book_next_get_cursor_pos()` / `read_last_get_cursor_pos()` / `async_get_cursor_pos()`

Get the current cursor position.

**Rust Signatures:**
```rust
pub fn book_next_get_cursor_pos(&mut self)
pub fn read_last_get_cursor_pos(&self) -> Result<(u32, u32), PeripheralError>
pub async fn async_get_cursor_pos(&self) -> Result<(u32, u32), PeripheralError>
```

**Returns:** `number, number` — X and Y coordinates (1-based)

**Example:**
```rust
// Rust example to be added
```
---

### `setCursorPos(x, y)` / `book_next_set_cursor_pos(x, y)` / `read_last_set_cursor_pos()` / `async_set_cursor_pos(x, y)`

Set the cursor position.

**Rust Signatures:**
```rust
pub fn book_next_set_cursor_pos(&mut self, x: u32, y: u32)
pub fn read_last_set_cursor_pos(&self) -> Result<(), PeripheralError>
pub async fn async_set_cursor_pos(&self, x: u32, y: u32) -> Result<(), PeripheralError>
```

**Parameters:**
- `x: number` — X coordinate (1-based)
- `y: number` — Y coordinate (1-based)

**Returns:** `nil`

**Example:**
```rust
// Rust example to be added
```
---

### `getCursorBlink()` / `book_next_get_cursor_blink()` / `read_last_get_cursor_blink()` / `async_get_cursor_blink()`

Get the cursor blink state.

**Rust Signatures:**
```rust
pub fn book_next_get_cursor_blink(&mut self)
pub fn read_last_get_cursor_blink(&self) -> Result<bool, PeripheralError>
pub async fn async_get_cursor_blink(&self) -> Result<bool, PeripheralError>
```

**Returns:** `boolean` — `true` if blinking, `false` otherwise

---

### `setCursorBlink(blink)` / `book_next_set_cursor_blink(blink)` / `read_last_set_cursor_blink()` / `async_set_cursor_blink(blink)`

Set the cursor blink state.

**Rust Signatures:**
```rust
pub fn book_next_set_cursor_blink(&mut self, blink: bool)
pub fn read_last_set_cursor_blink(&self) -> Result<(), PeripheralError>
pub async fn async_set_cursor_blink(&self, blink: bool) -> Result<(), PeripheralError>
```

**Parameters:**
- `blink: boolean` — Whether cursor should blink

**Returns:** `nil`

---

### `getSize()` / `book_next_get_size()` / `read_last_get_size()` / `async_get_size()`

Get the monitor size in characters.

**Rust Signatures:**
```rust
pub fn book_next_get_size(&mut self)
pub fn read_last_get_size(&self) -> Result<(u32, u32), PeripheralError>
pub async fn async_get_size(&self) -> Result<(u32, u32), PeripheralError>
```

**Returns:** `number, number` — Width and height in characters

**Example:**
```rust
// Rust example to be added
```
---

### `setTextColor(color)` / `book_next_set_text_color(color)` / `read_last_set_text_color()` / `async_set_text_color(color)`

Set the text color (advanced monitors only).

**Rust Signatures:**
```rust
pub fn book_next_set_text_color(&mut self, color: u32)
pub fn read_last_set_text_color(&self) -> Result<(), PeripheralError>
pub async fn async_set_text_color(&self, color: u32) -> Result<(), PeripheralError>
```

**Parameters:**
- `color: number` — Color value (0x000000–0xFFFFFF)

**Returns:** `nil`

**Predefined Colors:**
- `0xFFFFFF` — White
- `0xFF0000` — Red
- `0x00FF00` — Green
- `0x0000FF` — Blue
- `0xFFFF00` — Yellow
- `0xFF00FF` — Magenta
- `0x00FFFF` — Cyan
- `0x000000` — Black

---

### `getTextColor()` / `book_next_get_text_color()` / `read_last_get_text_color()` / `async_get_text_color()`

Get the current text color.

**Rust Signatures:**
```rust
pub fn book_next_get_text_color(&mut self)
pub fn read_last_get_text_color(&self) -> Result<u32, PeripheralError>
pub async fn async_get_text_color(&self) -> Result<u32, PeripheralError>
```

**Returns:** `number` — Current text color

---

### `setBackgroundColor(color)` / `book_next_set_background_color(color)` / `read_last_set_background_color()` / `async_set_background_color(color)`

Set the background color (advanced monitors only).

**Rust Signatures:**
```rust
pub fn book_next_set_background_color(&mut self, color: u32)
pub fn read_last_set_background_color(&self) -> Result<(), PeripheralError>
pub async fn async_set_background_color(&self, color: u32) -> Result<(), PeripheralError>
```

**Parameters:**
- `color: number` — Color value (0x000000–0xFFFFFF)

**Returns:** `nil`

---

### `getBackgroundColor()` / `book_next_get_background_color()` / `read_last_get_background_color()` / `async_get_background_color()`

Get the current background color.

**Rust Signatures:**
```rust
pub fn book_next_get_background_color(&mut self)
pub fn read_last_get_background_color(&self) -> Result<u32, PeripheralError>
pub async fn async_get_background_color(&self) -> Result<u32, PeripheralError>
```

**Returns:** `number` — Current background color

---

### `blit(text, textColor, backgroundColor)` / `book_next_blit(...)` / `read_last_blit()` / `async_blit(...)`

Write text with specified colors using blit.

**Rust Signatures:**
```rust
pub fn book_next_blit(&mut self, text: &str, text_color: &str, bg_color: &str)
pub fn read_last_blit(&self) -> Result<(), PeripheralError>
pub async fn async_blit(&self, text: &str, text_color: &str, bg_color: &str) -> Result<(), PeripheralError>
```

**Parameters:**
- `text: string` — Text to write
- `textColor: string` — Text color codes (one per character)
- `backgroundColor: string` — Background color codes (one per character)

**Returns:** `nil`

**Color Codes:**
- `0` — Black, `1` — Blue, `2` — Green, `3` — Cyan
- `4` — Red, `5` — Magenta, `6` — Yellow, `7` — White
- `8` — Light Gray, `9` — Light Blue, `a` — Lime, `b` — Light Cyan
- `c` — Light Red, `d` — Pink, `e` — Light Yellow, `f` — Light Gray

---

## Events

### `monitor_resize` 🚧

Fired when the monitor size changes.

**Event Parameters:**
1. `string` — Event name (`"monitor_resize"`)
2. `string` — Monitor side or network ID

**Example:**
```rust
// Rust example to be added
```
---

### `monitor_touch`

Fired when an advanced monitor is right-clicked.

**Event Parameters:**
1. `string` — Event name (`"monitor_touch"`)
2. `string` — Monitor side or network ID
3. `number` — X coordinate (1-based)
4. `number` — Y coordinate (1-based)

**Example:**
```rust
// Rust example to be added
```
---

## Usage Examples

### Example 1: Basic Text Display

```rust
// Rust example to be added
```
### Example 2: Colored Text (Advanced Monitor)

```rust
// Rust example to be added
```
### Example 3: Terminal Redirection

```rust
// Rust example to be added
```
### Example 4: Touch Button

```rust
// Rust example to be added
```
### Example 5: Dynamic Scaling

```rust
// Rust example to be added
```
---

## Error Handling

All methods may throw errors in the following cases:

- **Monitor not found**: Peripheral is disconnected
- **Invalid color**: Color value is out of range
- **Invalid scale**: Text scale is not in valid range
- **Invalid position**: Cursor position is out of bounds

**Example Error Handling:**
```rust
// Rust example to be added
```
---

## Type Definitions

### Color
```rust
// Rust example to be added
```
### Position
```rust
// Rust example to be added
```
### Size
```rust
// Rust example to be added
```
---

## Notes

- All coordinates are 1-based (first position is 1, not 0)
- Text scale affects both display size and resolution
- Color support requires an advanced monitor
- Touch events only work on advanced monitors
- Multiple monitors can be combined into one large display
- The three-function pattern allows for efficient batch operations

---

## Related

- [Speaker](./Speaker.md) — Audio output peripheral
- [CC:Tweaked Documentation](https://tweaked.cc/) — Official documentation
- `term` API — Terminal manipulation API
