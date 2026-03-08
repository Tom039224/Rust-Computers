# Transmitter

**Module:** `control_craft::transmitter`  
**Peripheral Type:** `controlcraft:transmitter_peripheral`

Control-Craft Transmitter peripheral for remote method invocation (synchronous and asynchronous) and protocol configuration across linked devices.

## Book-Read Methods

### Remote Calls

#### `book_next_call_remote` / `read_last_call_remote`
Synchronously call a remote method with the specified access key and context.
```rust
pub fn book_next_call_remote(&mut self, access: &str, ctx: &str, extra_args: &[Vec<u8>]) { ... }
pub fn read_last_call_remote(&self) -> Result<Value, PeripheralError> { ... }
```
**Parameters:**
- `access: &str` — Access key for the remote target
- `ctx: &str` — Context string
- `extra_args: &[Vec<u8>]` — Additional variadic arguments as msgpack-encoded byte arrays

**Returns:** `Value` — The remote method's return value

#### `book_next_call_remote_async` / `read_last_call_remote_async`
Asynchronously call a remote method. The result is delivered via a named event (slot).
```rust
pub fn book_next_call_remote_async(
    &mut self,
    access: &str,
    ctx: &str,
    slot_name: &str,
    remote_name: &str,
    method: &str,
    extra_args: &[Vec<u8>],
) { ... }
pub fn read_last_call_remote_async(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `access: &str` — Access key for the remote target
- `ctx: &str` — Context string
- `slot_name: &str` — Event slot name to receive the result
- `remote_name: &str` — Remote peripheral name
- `method: &str` — Remote method name
- `extra_args: &[Vec<u8>]` — Additional variadic arguments as msgpack-encoded byte arrays

---

### Configuration

#### `book_next_set_protocol` / `read_last_set_protocol`
Set the communication protocol number.
```rust
pub fn book_next_set_protocol(&mut self, protocol: i64) { ... }
pub fn read_last_set_protocol(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `protocol: i64` — Protocol number

## Usage Example

```rust
use rust_computers_api::control_craft::transmitter::*;
use rust_computers_api::peripheral::Peripheral;

let mut tx = Transmitter::find().unwrap();

// Set protocol
tx.book_next_set_protocol(1);
wait_for_next_tick().await;
let _ = tx.read_last_set_protocol();

// Synchronous remote call
tx.book_next_call_remote("my_access_key", "my_context", &[]);
wait_for_next_tick().await;
let result = tx.read_last_call_remote().unwrap();

// Asynchronous remote call
tx.book_next_call_remote_async(
    "my_access_key",
    "my_context",
    "on_result",
    "remote_device",
    "getStatus",
    &[],
);
wait_for_next_tick().await;
let _ = tx.read_last_call_remote_async();
```
