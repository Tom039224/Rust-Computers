# Modem

**Mod:** CC:Tweaked  
**Peripheral Type:** `modem`  
**Source:** `ModemPeripheral.java`

## Overview

The Modem peripheral provides wireless and wired network communication. It allows you to open channels, transmit messages, and receive data from other modems. Modems can operate in wireless mode (with range limits) or wired mode (unlimited range on connected networks).

## Three-Function Pattern

The Modem API uses the three-function pattern for all methods:

1. **`book_next_*`** - Schedule a request for the next tick
2. **`read_last_*`** - Read the result from the previous tick
3. **`async_*`** - Convenience method that books, waits, and reads in one call

### Pattern Explanation

```rust
// Method 1: book_next / read_last pattern
modem.book_next_open(15);
wait_for_next_tick().await;
modem.read_last_open()?;

// Method 2: async pattern (recommended)
modem.async_open(15).await?;
```

## Modem Types

CC:Tweaked provides three types of modems:

1. **Wireless Modem** - Communicates with other wireless modems within range (default 64 blocks, up to 384 blocks at y≥96)
2. **Ender Modem** - Wireless modem with unlimited range and cross-dimension support
3. **Wired Modem** - Connects to network cable for unlimited range on connected networks

## Implementation Status

### ✅ Implemented

- book_next_open / read_last_open
- book_next_is_open / read_last_is_open
- book_next_close / read_last_close
- book_next_close_all / read_last_close_all
- book_next_transmit / read_last_transmit
- book_next_transmit_raw / read_last_transmit_raw
- book_next_try_receive_raw / read_last_try_receive_raw
- receive_wait_raw (async)

### 🚧 Not Yet Implemented

- async_* variants for all methods (except receive_wait_raw)
- is_wireless() method (all variants)
- get_names_remote() method (all variants)
- modem_message event system


## Methods

### `open(channel)` / `book_next_open(channel)` / `read_last_open()` / `async_open(channel)`

Open a channel to listen for incoming messages.

**Rust Signatures:**
```rust
pub fn book_next_open(&mut self, channel: u32)
pub fn read_last_open(&self) -> Result<(), PeripheralError>
pub async fn async_open(&self, channel: u32) -> Result<(), PeripheralError>
```

**Parameters:**
- `channel: number` — Channel number to open (0–65535)

**Returns:** `nil`

**Notes:**
- Maximum 128 channels can be open simultaneously
- You don't need to open a channel to transmit, only to receive

**Example:**
```rust
// Rust example to be added
```
**Error Handling:**
- Throws error if channel number is out of range
- Throws error if 128 channels are already open

---

### `isOpen(channel)` / `book_next_is_open(channel)` / `read_last_is_open()` / `async_is_open(channel)`

Check if a channel is currently open.

**Rust Signatures:**
```rust
pub fn book_next_is_open(&mut self, channel: u32)
pub fn read_last_is_open(&self) -> Result<bool, PeripheralError>
pub async fn async_is_open(&self, channel: u32) -> Result<bool, PeripheralError>
```

**Parameters:**
- `channel: number` — Channel number to check (0–65535)

**Returns:** `boolean` — `true` if open, `false` if closed

**Example:**
```rust
// Rust example to be added
```
---

### `close(channel)` / `book_next_close(channel)` / `read_last_close()` / `async_close(channel)`

Close an open channel.

**Rust Signatures:**
```rust
pub fn book_next_close(&mut self, channel: u32)
pub fn read_last_close(&self) -> Result<(), PeripheralError>
pub async fn async_close(&self, channel: u32) -> Result<(), PeripheralError>
```

**Parameters:**
- `channel: number` — Channel number to close (0–65535)

**Returns:** `nil`

**Example:**
```rust
// Rust example to be added
```
---

### `closeAll()` / `book_next_close_all()` / `read_last_close_all()` / `async_close_all()`

Close all open channels.

**Rust Signatures:**
```rust
pub fn book_next_close_all(&mut self)
pub fn read_last_close_all(&self) -> Result<(), PeripheralError>
pub async fn async_close_all(&self) -> Result<(), PeripheralError>
```

**Returns:** `nil`

**Example:**
```rust
// Rust example to be added
```
---

### `transmit(channel, replyChannel, payload)` / `book_next_transmit(...)` / `read_last_transmit()` / `async_transmit(...)`

Transmit a message on a channel.

**Rust Signatures:**
```rust
pub fn book_next_transmit<T: Serialize>(&mut self, channel: u32, reply_channel: u32, payload: &T)
pub fn read_last_transmit(&self) -> Result<(), PeripheralError>
pub async fn async_transmit<T: Serialize>(&self, channel: u32, reply_channel: u32, payload: &T) -> Result<(), PeripheralError>
```

**Parameters:**
- `channel: number` — Target channel (0–65535)
- `replyChannel: number` — Reply channel for responses (0–65535)
- `payload: any` — Message data (primitives, tables, or serializable types)

**Returns:** `nil`

**Notes:**
- You don't need to open the target channel to transmit
- The reply channel should be open on your modem to receive responses
- Payload can be any Lua value (boolean, number, string, table)
- Functions and metatables are not transmitted

**Example:**
```rust
// Rust example to be added
```
---

### `isWireless()` / `book_next_is_wireless()` / `read_last_is_wireless()` / `async_is_wireless()`

Check if this modem is wireless or wired.

**Rust Signatures:**
```rust
pub fn book_next_is_wireless(&mut self)
pub fn read_last_is_wireless(&self) -> Result<bool, PeripheralError>
pub async fn async_is_wireless(&self) -> Result<bool, PeripheralError>
```

**Returns:** `boolean` — `true` if wireless, `false` if wired

**Example:**
```rust
// Rust example to be added
```
---

### `transmit_raw(channel, replyChannel, payload)` / `book_next_transmit_raw(...)` / `read_last_transmit_raw()` / `async_transmit_raw(...)`

Transmit a raw string payload on a channel.

**Rust Signatures:**
```rust
pub fn book_next_transmit_raw(&mut self, channel: u32, reply_channel: u32, payload: &str)
pub fn read_last_transmit_raw(&self) -> Vec<Result<(), PeripheralError>>
pub async fn async_transmit_raw(&self, channel: u32, reply_channel: u32, payload: &str) -> Result<(), PeripheralError>
```

**Parameters:**
- `channel: number` — Target channel (0–65535)
- `replyChannel: number` — Reply channel for responses (0–65535)
- `payload: string` — String payload to transmit

**Returns:** `nil`

**Notes:**
- Transmits a raw string payload directly
- More efficient than `transmit` for string data
- No serialization overhead

**Example:**
```rust
// Rust example to be added
```
---

### `try_receive_raw()` / `book_next_try_receive_raw()` / `read_last_try_receive_raw()` / `async_try_receive_raw()`

Try to receive a message (non-blocking).

**Rust Signatures:**
```rust
pub fn book_next_try_receive_raw(&mut self)
pub fn read_last_try_receive_raw(&self) -> Result<Option<ReceiveData<String>>, PeripheralError>
pub async fn async_try_receive_raw(&self) -> Result<Option<ReceiveData<String>>, PeripheralError>
```

**Returns:** `ReceiveData | nil` — Message data if available, `nil` otherwise

**Example:**
```rust
// Rust example to be added
```
---

### `receive_wait_raw()` / `async_receive_wait_raw()`

Wait for and receive a message (blocking).

**Rust Signatures:**
```rust
pub async fn receive_wait_raw(&self) -> Result<ReceiveData<String>, PeripheralError>
```

**Returns:** `ReceiveData` — Received message data

**Example:**
```rust
// Rust example to be added
```
---

### `getNamesRemote()` 🚧 / `book_next_get_names_remote()` / `read_last_get_names_remote()` / `async_get_names_remote()`

Get names of peripherals on the wired network.

**Rust Signatures:**
```rust
pub fn book_next_get_names_remote(&mut self)
pub fn read_last_get_names_remote(&self) -> Result<Vec<String>, PeripheralError>
pub async fn async_get_names_remote(&self) -> Result<Vec<String>, PeripheralError>
```

**Returns:** `table` — List of peripheral names on the wired network

**Example:**
```rust
// Rust example to be added
```
---

## Events

### `modem_message` 🚧

Fired when a message is received on an open channel.

**Event Parameters:**
1. `string` — Event name (`"modem_message"`)
2. `string` — Side the modem is on
3. `number` — Channel the message was sent on
4. `number` — Reply channel specified by sender
5. `any` — Message payload
6. `number | nil` — Distance to sender in blocks (nil for cross-dimension)

**Example:**
```rust
// Rust example to be added
```
---

## Usage Examples

### Example 1: Basic Message Exchange

```rust
// Rust example to be added
```
### Example 2: Simple Server

```rust
// Rust example to be added
```
### Example 3: Broadcast Listener

```rust
// Rust example to be added
```
### Example 4: Request-Reply Pattern

```rust
// Rust example to be added
```
### Example 5: Wired Network Discovery

```rust
// Rust example to be added
```
---

## Error Handling

All methods may throw errors in the following cases:

- **Modem not found**: Peripheral is disconnected
- **Invalid channel**: Channel number is out of range (0–65535)
- **Too many channels**: 128 channels already open
- **Network error**: Wired network connection is broken

**Example Error Handling:**
```rust
// Rust example to be added
```
---

## Type Definitions

### ReceiveData
```rust
// Rust example to be added
```
---

## Notes

- Channel numbers range from 0 to 65535
- Maximum 128 channels can be open simultaneously
- Wireless modems have range limits; wired modems do not
- Messages are transmitted instantly but received as events
- The three-function pattern allows for efficient batch operations
- Ender modems can communicate across dimensions

---

## Related

- [Inventory](./Inventory.md) — Requires wired modem for network access
- [CC:Tweaked Documentation](https://tweaked.cc/) — Official documentation
- `rednet` API — Higher-level networking API built on modems
