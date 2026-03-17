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

```lua
-- Method 1: book_next / read_last pattern
monitor.book_next_clear()
wait_for_next_tick()
monitor.read_last_clear()

-- Method 2: async pattern (recommended)
monitor.async_clear()
```

## Monitor Types

- **Normal Monitor** — Monochrome display
- **Advanced Monitor** — Color display with touch support

## Methods

### `setTextScale(scale)` / `book_next_set_text_scale(scale)` / `read_last_set_text_scale()` / `async_set_text_scale(scale)`

Set the text scale of the monitor.

**Lua Signature:**
```lua
function setTextScale(scale: number) -> nil
```

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
```lua
local monitor = peripheral.find("monitor")
monitor.async_set_text_scale(2.0)
```

---

### `getTextScale()` / `book_next_get_text_scale()` / `read_last_get_text_scale()` / `async_get_text_scale()`

Get the current text scale.

**Lua Signature:**
```lua
function getTextScale() -> number
```

**Rust Signatures:**
```rust
pub fn book_next_get_text_scale(&mut self)
pub fn read_last_get_text_scale(&self) -> Result<f32, PeripheralError>
pub async fn async_get_text_scale(&self) -> Result<f32, PeripheralError>
```

**Returns:** `number` — Current text scale

**Example:**
```lua
local monitor = peripheral.find("monitor")
local scale = monitor.async_get_text_scale()
print("Current scale: " .. scale)
```

---

### `write(text)` / `book_next_write(text)` / `read_last_write()` / `async_write(text)`

Write text at the current cursor position.

**Lua Signature:**
```lua
function write(text: string) -> nil
```

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
```lua
local monitor = peripheral.find("monitor")
monitor.async_write("Hello, World!")
```

---

### `clear()` / `book_next_clear()` / `read_last_clear()` / `async_clear()`

Clear the entire monitor screen.

**Lua Signature:**
```lua
function clear() -> nil
```

**Rust Signatures:**
```rust
pub fn book_next_clear(&mut self)
pub fn read_last_clear(&self) -> Result<(), PeripheralError>
pub async fn async_clear(&self) -> Result<(), PeripheralError>
```

**Returns:** `nil`

**Example:**
```lua
local monitor = peripheral.find("monitor")
monitor.async_clear()
```

---

### `clearLine()` / `book_next_clear_line()` / `read_last_clear_line()` / `async_clear_line()`

Clear the current line.

**Lua Signature:**
```lua
function clearLine() -> nil
```

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

**Lua Signature:**
```lua
function scroll(lines: number) -> nil
```

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

**Lua Signature:**
```lua
function getCursorPos() -> number, number
```

**Rust Signatures:**
```rust
pub fn book_next_get_cursor_pos(&mut self)
pub fn read_last_get_cursor_pos(&self) -> Result<(u32, u32), PeripheralError>
pub async fn async_get_cursor_pos(&self) -> Result<(u32, u32), PeripheralError>
```

**Returns:** `number, number` — X and Y coordinates (1-based)

**Example:**
```lua
local monitor = peripheral.find("monitor")
local x, y = monitor.async_get_cursor_pos()
print(("Cursor at (%d, %d)"):format(x, y))
```

---

### `setCursorPos(x, y)` / `book_next_set_cursor_pos(x, y)` / `read_last_set_cursor_pos()` / `async_set_cursor_pos(x, y)`

Set the cursor position.

**Lua Signature:**
```lua
function setCursorPos(x: number, y: number) -> nil
```

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
```lua
local monitor = peripheral.find("monitor")
monitor.async_set_cursor_pos(1, 1)
```

---

### `getCursorBlink()` / `book_next_get_cursor_blink()` / `read_last_get_cursor_blink()` / `async_get_cursor_blink()`

Get the cursor blink state.

**Lua Signature:**
```lua
function getCursorBlink() -> boolean
```

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

**Lua Signature:**
```lua
function setCursorBlink(blink: boolean) -> nil
```

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

**Lua Signature:**
```lua
function getSize() -> number, number
```

**Rust Signatures:**
```rust
pub fn book_next_get_size(&mut self)
pub fn read_last_get_size(&self) -> Result<(u32, u32), PeripheralError>
pub async fn async_get_size(&self) -> Result<(u32, u32), PeripheralError>
```

**Returns:** `number, number` — Width and height in characters

**Example:**
```lua
local monitor = peripheral.find("monitor")
local width, height = monitor.async_get_size()
print(("Monitor size: %d x %d"):format(width, height))
```

---

### `setTextColor(color)` / `book_next_set_text_color(color)` / `read_last_set_text_color()` / `async_set_text_color(color)`

Set the text color (advanced monitors only).

**Lua Signature:**
```lua
function setTextColor(color: number) -> nil
```

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

**Lua Signature:**
```lua
function getTextColor() -> number
```

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

**Lua Signature:**
```lua
function setBackgroundColor(color: number) -> nil
```

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

**Lua Signature:**
```lua
function getBackgroundColor() -> number
```

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

**Lua Signature:**
```lua
function blit(text: string, textColor: string, backgroundColor: string) -> nil
```

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

### `monitor_resize`

Fired when the monitor size changes.

**Event Parameters:**
1. `string` — Event name (`"monitor_resize"`)
2. `string` — Monitor side or network ID

**Example:**
```lua
local monitor = peripheral.find("monitor")

while true do
  local event, side = os.pullEvent("monitor_resize")
  local width, height = monitor.async_get_size()
  print(("Monitor resized to %d x %d"):format(width, height))
end
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
```lua
local monitor = peripheral.find("monitor")

while true do
  local event, side, x, y = os.pullEvent("monitor_touch")
  print(("Touched at (%d, %d)"):format(x, y))
end
```

---

## Usage Examples

### Example 1: Basic Text Display

```lua
local monitor = peripheral.find("monitor")

monitor.async_clear()
monitor.async_set_cursor_pos(1, 1)
monitor.async_write("Hello, World!")
```

### Example 2: Colored Text (Advanced Monitor)

```lua
local monitor = peripheral.find("monitor")

monitor.async_clear()
monitor.async_set_cursor_pos(1, 1)

monitor.async_set_text_color(0xFF0000)  -- Red
monitor.async_write("Red Text")

monitor.async_set_cursor_pos(1, 2)
monitor.async_set_text_color(0x00FF00)  -- Green
monitor.async_write("Green Text")
```

### Example 3: Terminal Redirection

```lua
local monitor = peripheral.find("monitor")

-- Redirect all output to monitor
term.redirect(monitor)

print("This appears on the monitor")
print("And this too!")

-- Restore normal output
term.restore()
```

### Example 4: Touch Button

```lua
local monitor = peripheral.find("monitor")

local function draw_button(x, y, text, color)
  monitor.async_set_cursor_pos(x, y)
  monitor.async_set_background_color(color)
  monitor.async_write(" " .. text .. " ")
  monitor.async_set_background_color(0x000000)
end

monitor.async_clear()
draw_button(2, 2, "Button 1", 0xFF0000)
draw_button(2, 4, "Button 2", 0x00FF00)

while true do
  local event, side, x, y = os.pullEvent("monitor_touch")
  
  if y == 2 then
    print("Button 1 pressed")
  elseif y == 4 then
    print("Button 2 pressed")
  end
end
```

### Example 5: Dynamic Scaling

```lua
local monitor = peripheral.find("monitor")
local scale = 0.5

while true do
  monitor.async_set_text_scale(scale)
  monitor.async_clear()
  monitor.async_set_cursor_pos(1, 1)
  monitor.async_write("Scale: " .. scale)
  
  local w, h = monitor.async_get_size()
  monitor.async_set_cursor_pos(1, 2)
  monitor.async_write(("Size: %d x %d"):format(w, h))
  
  scale = scale + 0.5
  if scale > 5.0 then scale = 0.5 end
  
  sleep(2)
end
```

---

## Error Handling

All methods may throw errors in the following cases:

- **Monitor not found**: Peripheral is disconnected
- **Invalid color**: Color value is out of range
- **Invalid scale**: Text scale is not in valid range
- **Invalid position**: Cursor position is out of bounds

**Example Error Handling:**
```lua
local monitor = peripheral.find("monitor")
if not monitor then
  error("No monitor found")
end

local success, result = pcall(function()
  monitor.async_set_text_scale(2.0)
end)

if not success then
  print("Error: " .. result)
else
  print("Scale set successfully")
end
```

---

## Type Definitions

### Color
```lua
-- RGB color value (0x000000 to 0xFFFFFF)
type Color = number
```

### Position
```lua
-- (x, y) coordinates (1-based)
type Position = (number, number)
```

### Size
```lua
-- (width, height) in characters
type Size = (number, number)
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
