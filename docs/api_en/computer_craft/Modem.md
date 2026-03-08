# Modem

**Module:** `computer_craft::modem`  
**Peripheral Type:** `modem`

CC:Tweaked Modem peripheral for wireless and wired network communication. Supports opening/closing channels, transmitting messages, and receiving data.

## Book-Read Methods

### `book_next_open` / `read_last_open`
Open a channel for listening.
```rust
pub fn book_next_open(&mut self, channel: u32) { ... }
pub fn read_last_open(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `channel: u32` — Channel number to open

**Returns:** `()`

---

### `book_next_is_open` / `read_last_is_open`
Check if a channel is currently open.
```rust
pub fn book_next_is_open(&mut self, channel: u32) { ... }
pub fn read_last_is_open(&self) -> Result<bool, PeripheralError> { ... }
```
**Parameters:**
- `channel: u32` — Channel number to check

**Returns:** `bool`

---

### `book_next_close` / `read_last_close`
Close a channel.
```rust
pub fn book_next_close(&mut self, channel: u32) { ... }
pub fn read_last_close(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `channel: u32` — Channel number to close

**Returns:** `()`

---

### `book_next_close_all` / `read_last_close_all`
Close all open channels.
```rust
pub fn book_next_close_all(&mut self) { ... }
pub fn read_last_close_all(&self) -> Result<(), PeripheralError> { ... }
```
**Returns:** `()`

---

### `book_next_transmit` / `read_last_transmit`
Transmit a serde-serializable payload on a channel.
```rust
pub fn book_next_transmit<T: Serialize>(&mut self, channel: u32, reply_channel: u32, payload: &T) { ... }
pub fn read_last_transmit(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `channel: u32` — Target channel
- `reply_channel: u32` — Reply channel
- `payload: &T` — Serializable payload

**Returns:** `()`

---

### `book_next_transmit_raw` / `read_last_transmit_raw`
Transmit a raw string payload on a channel.
```rust
pub fn book_next_transmit_raw(&mut self, channel: u32, reply_channel: u32, payload: &str) { ... }
pub fn read_last_transmit_raw(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `channel: u32` — Target channel
- `reply_channel: u32` — Reply channel
- `payload: &str` — String payload

**Returns:** `()`

---

### `book_next_try_receive_raw` / `read_last_try_receive_raw`
Try to receive a message within 1 tick. Returns `None` if nothing arrives.
```rust
pub fn book_next_try_receive_raw(&mut self) { ... }
pub fn read_last_try_receive_raw(&self) -> Result<Option<ReceiveData<String>>, PeripheralError> { ... }
```
**Returns:** `Option<ReceiveData<String>>`

## Event-Wait Methods

### `receive_wait_raw`
Wait until a message is received (async). Polls each tick until data arrives.
```rust
pub async fn receive_wait_raw(&self) -> Result<ReceiveData<String>, PeripheralError> { ... }
```
**Returns:** `ReceiveData<String>`

## Types

### `ReceiveData<T>`
Received message data.
```rust
pub struct ReceiveData<T> {
    pub channel: u32,
    pub reply_channel: u32,
    pub payload: T,
    pub distance: u32,
}
```

## Usage Example

```rust
use rust_computers_api::computer_craft::modem::*;
use rust_computers_api::peripheral::Peripheral;

let mut modem = Modem::find().unwrap();

// Open channel 1
modem.book_next_open(1);
wait_for_next_tick().await;
let _ = modem.read_last_open();

// Transmit a string message
modem.book_next_transmit_raw(1, 1, "hello");
wait_for_next_tick().await;
let _ = modem.read_last_transmit_raw();

// Wait for a message (async)
let msg = modem.receive_wait_raw().await.unwrap();
println!("Received: {} from distance {}", msg.payload, msg.distance);
```
