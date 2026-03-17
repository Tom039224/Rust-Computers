# ChatBox

**Mod:** AdvancedPeripherals  
**Peripheral Type:** `advancedPeripherals:chat_box`  
**Source:** `ChatBoxPeripheral.java`

## Overview

The ChatBox peripheral allows you to send chat messages and toast notifications to players on the server. It supports plain text messages, JSON-formatted text components, and customizable formatting with prefixes, brackets, and colors. This is useful for creating server announcements, player notifications, and interactive systems.

## Three-Function Pattern

The ChatBox API uses the three-function pattern for all methods:

1. **`book_next_*`** - Schedule a request for the next tick
2. **`read_last_*`** - Read the result from the previous tick
3. **`async_*`** - Convenience method that books, waits, and reads in one call

### Pattern Explanation

```rust
// Rust example to be added
```
## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods
- chat event


## Methods

### Message Sending

#### `sendMessage(message, prefix?, brackets?, color?)` / `book_next_send_message(...)` / `read_last_send_message()` / `async_send_message(...)`

Send a plain text message to all players on the server.

**Rust Signatures:**
```rust
pub fn book_next_send_message(&mut self, message: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>)
pub fn read_last_send_message(&self) -> Result<bool, PeripheralError>
pub async fn async_send_message(&self, message: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>) -> Result<bool, PeripheralError>
```

**Parameters:**
- `message: string` — Message text to send
- `prefix?: string` — Optional prefix (e.g., "[Server]")
- `brackets?: string` — Optional bracket style (e.g., "[]", "<>")
- `color?: string` — Optional color code (e.g., "red", "blue")

**Returns:** `boolean` — `true` if message was sent successfully

**Example:**
```rust
// Rust example to be added
```
---

#### `sendMessageToPlayer(message, player, prefix?, brackets?, color?)` / `book_next_send_message_to_player(...)` / `read_last_send_message_to_player()` / `async_send_message_to_player(...)`

Send a message to a specific player.

**Rust Signatures:**
```rust
pub fn book_next_send_message_to_player(&mut self, message: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>)
pub fn read_last_send_message_to_player(&self) -> Result<bool, PeripheralError>
pub async fn async_send_message_to_player(&self, message: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>) -> Result<bool, PeripheralError>
```

**Parameters:**
- `message: string` — Message text
- `player: string` — Player name
- `prefix?: string` — Optional prefix
- `brackets?: string` — Optional bracket style
- `color?: string` — Optional color code

**Returns:** `boolean` — `true` if message was sent

**Example:**
```rust
// Rust example to be added
```
---

#### `sendJsonMessage(json)` / `book_next_send_json_message(json)` / `read_last_send_json_message()` / `async_send_json_message(json)`

Send a JSON-formatted message to all players.

**Rust Signatures:**
```rust
pub fn book_next_send_json_message(&mut self, json: &str)
pub fn read_last_send_json_message(&self) -> Result<bool, PeripheralError>
pub async fn async_send_json_message(&self, json: &str) -> Result<bool, PeripheralError>
```

**Parameters:**
- `json: string` — JSON text component string

**Returns:** `boolean` — `true` if message was sent

**Example:**
```rust
// Rust example to be added
```
---

#### `sendJsonMessageToPlayer(json, player)` / `book_next_send_json_message_to_player(json, player)` / `read_last_send_json_message_to_player()` / `async_send_json_message_to_player(json, player)`

Send a JSON-formatted message to a specific player.

**Rust Signatures:**
```rust
pub fn book_next_send_json_message_to_player(&mut self, json: &str, player: &str)
pub fn read_last_send_json_message_to_player(&self) -> Result<bool, PeripheralError>
pub async fn async_send_json_message_to_player(&self, json: &str, player: &str) -> Result<bool, PeripheralError>
```

**Parameters / Returns:** Same as `sendJsonMessage`

---

### Toast Notifications

#### `sendToast(title, description?, icon?)` / `book_next_send_toast(...)` / `read_last_send_toast()` / `async_send_toast(...)`

Send a toast notification to all players.

**Rust Signatures:**
```rust
pub fn book_next_send_toast(&mut self, title: &str, description: Option<&str>, icon: Option<&str>)
pub fn read_last_send_toast(&self) -> Result<bool, PeripheralError>
pub async fn async_send_toast(&self, title: &str, description: Option<&str>, icon: Option<&str>) -> Result<bool, PeripheralError>
```

**Parameters:**
- `title: string` — Toast title
- `description?: string` — Optional description text
- `icon?: string` — Optional icon item (e.g., "minecraft:diamond")

**Returns:** `boolean` — `true` if toast was sent

**Example:**
```rust
// Rust example to be added
```
---

#### `sendToastToPlayer(title, player, description?, icon?)` / `book_next_send_toast_to_player(...)` / `read_last_send_toast_to_player()` / `async_send_toast_to_player(...)`

Send a toast notification to a specific player.

**Rust Signatures:**
```rust
pub fn book_next_send_toast_to_player(&mut self, title: &str, player: &str, description: Option<&str>, icon: Option<&str>)
pub fn read_last_send_toast_to_player(&self) -> Result<bool, PeripheralError>
pub async fn async_send_toast_to_player(&self, title: &str, player: &str, description: Option<&str>, icon: Option<&str>) -> Result<bool, PeripheralError>
```

**Parameters / Returns:** Same as `sendToast`

---

## Events

The ChatBox peripheral does not generate events.

---

## Usage Examples

### Example 1: Server Announcement

```rust
// Rust example to be added
```
### Example 2: Welcome New Players

```rust
// Rust example to be added
```
### Example 3: Colored Messages

```rust
// Rust example to be added
```
### Example 4: JSON Formatted Messages

```rust
// Rust example to be added
```
### Example 5: Achievement System

```rust
// Rust example to be added
```
---

## Error Handling

All methods may throw errors in the following cases:

- **Player not found**: Target player is not online
- **Invalid JSON**: JSON message is malformed
- **Peripheral disconnected**: The ChatBox is no longer accessible

**Example Error Handling:**
```rust
// Rust example to be added
```
---

## Type Definitions

### TextComponent
```rust
// Rust example to be added
```
---

## Notes

- Messages are sent to all players or specific players
- JSON messages support advanced formatting and interactions
- Toast notifications appear in the top-right corner
- Color codes include: black, dark_blue, dark_green, dark_aqua, dark_red, dark_purple, gold, gray, dark_gray, blue, green, aqua, red, light_purple, yellow, white
- The three-function pattern allows for efficient batch operations
- ChatBox requires network connectivity to reach players

---

## Related

- [PlayerDetector](./PlayerDetector.md) — For detecting players
- [MEBridge](./MEBridge.md) — For inventory management
- [AdvancedPeripherals Documentation](https://advancedperipherals.readthedocs.io/) — Official documentation
- `prefix?: string` — Optional prefix (e.g., `"[Server]"`)
- `brackets?: string` — Optional bracket style (e.g., `"<>"`, `"[]"`)
- `color?: string` — Optional color code (e.g., `"red"`, `"#FF0000"`)

**Returns:** `boolean` — `true` if message was sent successfully

**Example:**
```rust
// Rust example to be added
```
---

### `sendFormattedMessage(json, prefix?, brackets?, color?)` / `book_next_send_formatted_message(...)` / `read_last_send_formatted_message()` / `async_send_formatted_message(...)`

Send a JSON-formatted text component message to all players.

**Rust Signatures:**
```rust
pub fn book_next_send_formatted_message(&mut self, json: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>)
pub fn read_last_send_formatted_message(&self) -> Result<bool, PeripheralError>
pub async fn async_send_formatted_message(&self, json: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>) -> Result<bool, PeripheralError>
```

**Parameters:**
- `json: string` — JSON text component (Minecraft chat format)
- `prefix?: string` — Optional prefix
- `brackets?: string` — Optional bracket style
- `color?: string` — Optional color code

**Returns:** `boolean` — `true` if message was sent successfully

**JSON Format Example:**
```json
{
  "text": "Click me!",
  "color": "blue",
  "clickEvent": {
    "action": "run_command",
    "value": "/say clicked"
  }
}
```

**Example:**
```rust
// Rust example to be added
```
---

### `sendMessageToPlayer(message, player, prefix?, brackets?, color?)` / `book_next_send_message_to_player(...)` / `read_last_send_message_to_player()` / `async_send_message_to_player(...)`

Send a plain text message to a specific player.

**Rust Signatures:**
```rust
pub fn book_next_send_message_to_player(&mut self, message: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>)
pub fn read_last_send_message_to_player(&self) -> Result<bool, PeripheralError>
pub async fn async_send_message_to_player(&self, message: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>) -> Result<bool, PeripheralError>
```

**Parameters:**
- `message: string` — Message text
- `player: string` — Target player name
- `prefix?: string` — Optional prefix
- `brackets?: string` — Optional bracket style
- `color?: string` — Optional color code

**Returns:** `boolean` — `true` if message was sent successfully

**Example:**
```rust
// Rust example to be added
```
---

### `sendFormattedMessageToPlayer(json, player, prefix?, brackets?, color?)` / `book_next_send_formatted_message_to_player(...)` / `read_last_send_formatted_message_to_player()` / `async_send_formatted_message_to_player(...)`

Send a JSON-formatted text component message to a specific player.

**Rust Signatures:**
```rust
pub fn book_next_send_formatted_message_to_player(&mut self, json: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>)
pub fn read_last_send_formatted_message_to_player(&self) -> Result<bool, PeripheralError>
pub async fn async_send_formatted_message_to_player(&self, json: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>) -> Result<bool, PeripheralError>
```

**Parameters:**
- `json: string` — JSON text component
- `player: string` — Target player name
- `prefix?: string` — Optional prefix
- `brackets?: string` — Optional bracket style
- `color?: string` — Optional color code

**Returns:** `boolean` — `true` if message was sent successfully

---

### `sendToastToPlayer(title, subtitle, player, prefix?, brackets?, color?)` / `book_next_send_toast_to_player(...)` / `read_last_send_toast_to_player()` / `async_send_toast_to_player(...)`

Send a toast notification to a specific player.

**Rust Signatures:**
```rust
pub fn book_next_send_toast_to_player(&mut self, title: &str, subtitle: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>)
pub fn read_last_send_toast_to_player(&self) -> Result<bool, PeripheralError>
pub async fn async_send_toast_to_player(&self, title: &str, subtitle: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>) -> Result<bool, PeripheralError>
```

**Parameters:**
- `title: string` — Toast title text
- `subtitle: string` — Toast subtitle text
- `player: string` — Target player name
- `prefix?: string` — Optional prefix
- `brackets?: string` — Optional bracket style
- `color?: string` — Optional color code

**Returns:** `boolean` — `true` if toast was sent successfully

**Example:**
```rust
// Rust example to be added
```
---

### `sendFormattedToastToPlayer(jsonTitle, jsonSubtitle, player, prefix?, brackets?, color?)` / `book_next_send_formatted_toast_to_player(...)` / `read_last_send_formatted_toast_to_player()` / `async_send_formatted_toast_to_player(...)`

Send a JSON-formatted toast notification to a specific player.

**Rust Signatures:**
```rust
pub fn book_next_send_formatted_toast_to_player(&mut self, json_title: &str, json_subtitle: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>)
pub fn read_last_send_formatted_toast_to_player(&self) -> Result<bool, PeripheralError>
pub async fn async_send_formatted_toast_to_player(&self, json_title: &str, json_subtitle: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>) -> Result<bool, PeripheralError>
```

**Parameters:**
- `jsonTitle: string` — JSON title component
- `jsonSubtitle: string` — JSON subtitle component
- `player: string` — Target player name
- `prefix?: string` — Optional prefix
- `brackets?: string` — Optional bracket style
- `color?: string` — Optional color code

**Returns:** `boolean` — `true` if toast was sent successfully

---

## Events

The ChatBox peripheral does not generate events.

---

## Usage Examples

### Example 1: Server Announcement

```rust
// Rust example to be added
```
### Example 2: Welcome Message

```rust
// Rust example to be added
```
### Example 3: Formatted Message with Hover Text

```rust
// Rust example to be added
```
### Example 4: Batch Notifications

```rust
// Rust example to be added
```
### Example 5: Status Updates

```rust
// Rust example to be added
```
---

## Error Handling

All methods may throw errors in the following cases:

- **Player not found**: Target player is not online
- **Peripheral disconnected**: The ChatBox is no longer accessible
- **Invalid JSON**: JSON format is malformed
- **Message too long**: Message exceeds character limit

**Example Error Handling:**
```rust
// Rust example to be added
```
---

## Type Definitions

### TextComponent (JSON)
```rust
// Rust example to be added
```
---

## Notes

- Messages are sent asynchronously and may not appear immediately
- Player names are case-sensitive
- JSON formatting follows Minecraft's text component format
- Toast notifications appear in the top-right corner of the player's screen
- Color codes can be color names (e.g., "red", "blue") or hex codes (e.g., "#FF0000")
- The three-function pattern allows for efficient batch operations

---

## Related

- [Modem](../computer_craft/Modem.md) — For network communication
- [AdvancedPeripherals Documentation](https://advancedperipherals.readthedocs.io/) — Official documentation
