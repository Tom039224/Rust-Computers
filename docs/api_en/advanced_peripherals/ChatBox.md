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

```lua
-- Method 1: book_next / read_last pattern
chat.book_next_send_message("Hello!")
wait_for_next_tick()
local ok = chat.read_last_send_message()

-- Method 2: async pattern (recommended)
local ok = chat.async_send_message("Hello!")
```

## Methods

### `sendMessage(message, prefix?, brackets?, color?)` / `book_next_send_message(...)` / `read_last_send_message()` / `async_send_message(...)`

Send a plain text message to all players on the server.

**Lua Signature:**
```lua
function sendMessage(message: string, prefix?: string, brackets?: string, color?: string) -> boolean
```

**Rust Signatures:**
```rust
pub fn book_next_send_message(&mut self, message: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>)
pub fn read_last_send_message(&self) -> Result<bool, PeripheralError>
pub async fn async_send_message(&self, message: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>) -> Result<bool, PeripheralError>
```

**Parameters:**
- `message: string` — Message text to send
- `prefix?: string` — Optional prefix (e.g., `"[Server]"`)
- `brackets?: string` — Optional bracket style (e.g., `"<>"`, `"[]"`)
- `color?: string` — Optional color code (e.g., `"red"`, `"#FF0000"`)

**Returns:** `boolean` — `true` if message was sent successfully

**Example:**
```lua
local chat = peripheral.find("advancedPeripherals:chat_box")
local ok = chat.async_send_message("Server is restarting!", "[Server]", "[]", "red")
print("Message sent: " .. tostring(ok))
```

---

### `sendFormattedMessage(json, prefix?, brackets?, color?)` / `book_next_send_formatted_message(...)` / `read_last_send_formatted_message()` / `async_send_formatted_message(...)`

Send a JSON-formatted text component message to all players.

**Lua Signature:**
```lua
function sendFormattedMessage(json: string, prefix?: string, brackets?: string, color?: string) -> boolean
```

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
```lua
local chat = peripheral.find("advancedPeripherals:chat_box")
local json = '{"text":"Important!","color":"red","bold":true}'
local ok = chat.async_send_formatted_message(json, "[Alert]")
```

---

### `sendMessageToPlayer(message, player, prefix?, brackets?, color?)` / `book_next_send_message_to_player(...)` / `read_last_send_message_to_player()` / `async_send_message_to_player(...)`

Send a plain text message to a specific player.

**Lua Signature:**
```lua
function sendMessageToPlayer(message: string, player: string, prefix?: string, brackets?: string, color?: string) -> boolean
```

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
```lua
local chat = peripheral.find("advancedPeripherals:chat_box")
local ok = chat.async_send_message_to_player("Welcome!", "Steve", "[Server]")
```

---

### `sendFormattedMessageToPlayer(json, player, prefix?, brackets?, color?)` / `book_next_send_formatted_message_to_player(...)` / `read_last_send_formatted_message_to_player()` / `async_send_formatted_message_to_player(...)`

Send a JSON-formatted text component message to a specific player.

**Lua Signature:**
```lua
function sendFormattedMessageToPlayer(json: string, player: string, prefix?: string, brackets?: string, color?: string) -> boolean
```

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

**Lua Signature:**
```lua
function sendToastToPlayer(title: string, subtitle: string, player: string, prefix?: string, brackets?: string, color?: string) -> boolean
```

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
```lua
local chat = peripheral.find("advancedPeripherals:chat_box")
local ok = chat.async_send_toast_to_player("Achievement!", "You found diamonds!", "Steve")
```

---

### `sendFormattedToastToPlayer(jsonTitle, jsonSubtitle, player, prefix?, brackets?, color?)` / `book_next_send_formatted_toast_to_player(...)` / `read_last_send_formatted_toast_to_player()` / `async_send_formatted_toast_to_player(...)`

Send a JSON-formatted toast notification to a specific player.

**Lua Signature:**
```lua
function sendFormattedToastToPlayer(jsonTitle: string, jsonSubtitle: string, player: string, prefix?: string, brackets?: string, color?: string) -> boolean
```

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

```lua
local chat = peripheral.find("advancedPeripherals:chat_box")

chat.async_send_message("Server maintenance in 5 minutes!", "[Server]", "[]", "red")
sleep(5)
chat.async_send_message("Server is restarting now!", "[Server]", "[]", "red")
```

### Example 2: Welcome Message

```lua
local chat = peripheral.find("advancedPeripherals:chat_box")

local function welcome_player(name)
  chat.async_send_message_to_player("Welcome to the server!", name, "[Server]", "[]", "green")
  chat.async_send_toast_to_player("Welcome!", "Enjoy your stay!", name)
end

welcome_player("Steve")
```

### Example 3: Formatted Message with Hover Text

```lua
local chat = peripheral.find("advancedPeripherals:chat_box")

local json = {
  text = "Click for info",
  color = "blue",
  underlined = true,
  hoverEvent = {
    action = "show_text",
    contents = "This is a tooltip"
  }
}

local json_str = textutils.serialiseJSON(json)
chat.async_send_formatted_message(json_str, "[Info]")
```

### Example 4: Batch Notifications

```lua
local chat = peripheral.find("advancedPeripherals:chat_box")

local players = {"Steve", "Alex", "Notch"}
for _, player in ipairs(players) do
  chat.async_send_message_to_player("Hello " .. player .. "!", player, "[Server]")
end
```

### Example 5: Status Updates

```lua
local chat = peripheral.find("advancedPeripherals:chat_box")

local function send_status(status, color)
  local json = {
    text = "Status: " .. status,
    color = color,
    bold = true
  }
  local json_str = textutils.serialiseJSON(json)
  chat.async_send_formatted_message(json_str, "[System]")
end

send_status("Online", "green")
sleep(2)
send_status("Busy", "yellow")
sleep(2)
send_status("Offline", "red")
```

---

## Error Handling

All methods may throw errors in the following cases:

- **Player not found**: Target player is not online
- **Peripheral disconnected**: The ChatBox is no longer accessible
- **Invalid JSON**: JSON format is malformed
- **Message too long**: Message exceeds character limit

**Example Error Handling:**
```lua
local chat = peripheral.find("advancedPeripherals:chat_box")
if not chat then
  error("ChatBox not found")
end

local success, result = pcall(function()
  return chat.async_send_message("Hello!")
end)

if not success then
  print("Error: " .. result)
else
  print("Message sent: " .. tostring(result))
end
```

---

## Type Definitions

### TextComponent (JSON)
```lua
{
  text: string,                    -- Display text
  color?: string,                  -- Color name or hex (#RRGGBB)
  bold?: boolean,                  -- Bold text
  italic?: boolean,                -- Italic text
  underlined?: boolean,            -- Underlined text
  strikethrough?: boolean,         -- Strikethrough text
  obfuscated?: boolean,            -- Obfuscated text
  clickEvent?: {
    action: string,                -- "open_url", "run_command", "suggest_command", "copy_to_clipboard"
    value: string                  -- Action value
  },
  hoverEvent?: {
    action: string,                -- "show_text", "show_item", "show_entity"
    contents: string | table       -- Event contents
  }
}
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
