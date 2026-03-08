# Modem

**Module:** `computer_craft`
**Peripheral Type:** `modem` (the NAME constant)

CC:Tweaked Modem peripheral (unified for wireless and wired). Provides channel-based messaging for inter-computer communication.

## Methods

### Book/Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_open` / `read_last_open`

Open a channel for listening.

```rust
pub fn book_next_open(&mut self, channel: u32)
pub fn read_last_open(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| channel | `u32` | The channel number to open |

#### `book_next_is_open` / `read_last_is_open`

Check if a channel is currently open.

```rust
pub fn book_next_is_open(&mut self, channel: u32)
pub fn read_last_is_open(&self) -> Result<bool, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| channel | `u32` | The channel number to check |

#### `book_next_close` / `read_last_close`

Close a channel.

```rust
pub fn book_next_close(&mut self, channel: u32)
pub fn read_last_close(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| channel | `u32` | The channel number to close |

#### `book_next_close_all` / `read_last_close_all`

Close all open channels.

```rust
pub fn book_next_close_all(&mut self)
pub fn read_last_close_all(&self) -> Result<(), PeripheralError>
```

#### `book_next_transmit` / `read_last_transmit`

Transmit a serde-serializable payload on a channel.

```rust
pub fn book_next_transmit<T: Serialize>(&mut self, channel: u32, reply_channel: u32, payload: &T)
pub fn read_last_transmit(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| channel | `u32` | The channel to transmit on |
| reply_channel | `u32` | The channel for replies |
| payload | `&T` | Serde-serializable payload to send |

#### `book_next_transmit_raw` / `read_last_transmit_raw`

Transmit a raw string payload on a channel.

```rust
pub fn book_next_transmit_raw(&mut self, channel: u32, reply_channel: u32, payload: &str)
pub fn read_last_transmit_raw(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| channel | `u32` | The channel to transmit on |
| reply_channel | `u32` | The channel for replies |
| payload | `&str` | Raw string payload to send |

#### `book_next_try_receive_raw` / `read_last_try_receive_raw`

Try to receive a message within 1 tick. Returns `None` if nothing arrives.

```rust
pub fn book_next_try_receive_raw(&mut self)
pub fn read_last_try_receive_raw(&self) -> Result<Option<ReceiveData<String>>, PeripheralError>
```

### Async Methods (Event)

These methods block until an event occurs.

#### `receive_wait_raw`

Wait until a message is received. Loops internally, polling each tick until a message arrives.

```rust
pub async fn receive_wait_raw(&self) -> Result<ReceiveData<String>, PeripheralError>
```

## Example

```rust
// Open a channel, transmit, and wait for a reply
modem.book_next_open(1);
wait_for_next_tick().await;
modem.read_last_open().unwrap();

modem.book_next_transmit_raw(1, 2, "hello");
wait_for_next_tick().await;
modem.read_last_transmit_raw().unwrap();

let msg = modem.receive_wait_raw().await.unwrap();
```

## Types

### ReceiveData\<T\>

Received data wrapper returned by receive methods.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiveData<T> {
    pub channel: u32,
    pub reply_channel: u32,
    pub payload: T,
    pub distance: u32,
}
```

| Field | Type | Description |
|-------|------|-------------|
| channel | `u32` | The channel the message was received on |
| reply_channel | `u32` | The reply channel specified by sender |
| payload | `T` | The message payload |
| distance | `u32` | Distance from sender |
