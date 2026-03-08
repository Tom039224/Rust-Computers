# ChatBox

**Module:** `advanced_peripherals::chat_box`  
**Peripheral Type:** `advancedPeripherals:chat_box`

AdvancedPeripherals Chat Box peripheral. Sends chat messages and toast notifications to players. Supports plain text and JSON-formatted messages with optional prefix, brackets, and color customization.

## Book-Read Methods

### `book_next_send_message` / `read_last_send_message`
Send a message to all players.
```rust
pub fn book_next_send_message(
    &mut self,
    message: &str,
    prefix: Option<&str>,
    brackets: Option<&str>,
    color: Option<&str>,
)
pub fn read_last_send_message(&self) -> Result<bool, PeripheralError>
```
**Parameters:**
- `message: &str` — Message text
- `prefix: Option<&str>` — Optional prefix
- `brackets: Option<&str>` — Optional bracket style
- `color: Option<&str>` — Optional color

**Returns:** `bool` — Success status

---

### `book_next_send_formatted_message` / `read_last_send_formatted_message`
Send a JSON-formatted message to all players.
```rust
pub fn book_next_send_formatted_message(
    &mut self,
    json: &str,
    prefix: Option<&str>,
    brackets: Option<&str>,
    color: Option<&str>,
)
pub fn read_last_send_formatted_message(&self) -> Result<bool, PeripheralError>
```
**Parameters:**
- `json: &str` — JSON text component
- `prefix: Option<&str>` — Optional prefix
- `brackets: Option<&str>` — Optional bracket style
- `color: Option<&str>` — Optional color

**Returns:** `bool`

---

### `book_next_send_message_to_player` / `read_last_send_message_to_player`
Send a message to a specific player.
```rust
pub fn book_next_send_message_to_player(
    &mut self,
    message: &str,
    player: &str,
    prefix: Option<&str>,
    brackets: Option<&str>,
    color: Option<&str>,
)
pub fn read_last_send_message_to_player(&self) -> Result<bool, PeripheralError>
```
**Parameters:**
- `message: &str` — Message text
- `player: &str` — Target player name
- `prefix: Option<&str>` — Optional prefix
- `brackets: Option<&str>` — Optional bracket style
- `color: Option<&str>` — Optional color

**Returns:** `bool`

---

### `book_next_send_formatted_message_to_player` / `read_last_send_formatted_message_to_player`
Send a JSON-formatted message to a specific player.
```rust
pub fn book_next_send_formatted_message_to_player(
    &mut self,
    json: &str,
    player: &str,
    prefix: Option<&str>,
    brackets: Option<&str>,
    color: Option<&str>,
)
pub fn read_last_send_formatted_message_to_player(&self) -> Result<bool, PeripheralError>
```
**Parameters:**
- `json: &str` — JSON text component
- `player: &str` — Target player name
- `prefix / brackets / color` — Optional customization

**Returns:** `bool`

---

### `book_next_send_toast_to_player` / `read_last_send_toast_to_player`
Send a toast notification to a specific player.
```rust
pub fn book_next_send_toast_to_player(
    &mut self,
    title: &str,
    subtitle: &str,
    player: &str,
    prefix: Option<&str>,
    brackets: Option<&str>,
    color: Option<&str>,
)
pub fn read_last_send_toast_to_player(&self) -> Result<bool, PeripheralError>
```
**Parameters:**
- `title: &str` — Toast title
- `subtitle: &str` — Toast subtitle
- `player: &str` — Target player name
- `prefix / brackets / color` — Optional customization

**Returns:** `bool`

---

### `book_next_send_formatted_toast_to_player` / `read_last_send_formatted_toast_to_player`
Send a JSON-formatted toast notification to a specific player.
```rust
pub fn book_next_send_formatted_toast_to_player(
    &mut self,
    json_title: &str,
    json_subtitle: &str,
    player: &str,
    prefix: Option<&str>,
    brackets: Option<&str>,
    color: Option<&str>,
)
pub fn read_last_send_formatted_toast_to_player(&self) -> Result<bool, PeripheralError>
```
**Parameters:**
- `json_title: &str` — JSON title component
- `json_subtitle: &str` — JSON subtitle component
- `player: &str` — Target player name
- `prefix / brackets / color` — Optional customization

**Returns:** `bool`

## Immediate Methods

None.

## Types

None.

## Usage Example

```rust
use rust_computers_api::advanced_peripherals::ChatBox;
use rust_computers_api::peripheral::Peripheral;

let mut chat = ChatBox::wrap(addr);

chat.book_next_send_message("Hello, world!", Some("Server"), None, None);
wait_for_next_tick().await;
let ok = chat.read_last_send_message();

chat.book_next_send_message_to_player("Private msg", "Steve", None, None, None);
wait_for_next_tick().await;
```
