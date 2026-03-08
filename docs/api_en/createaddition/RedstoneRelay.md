# RedstoneRelay

**Module:** `createaddition::redstone_relay`  
**Peripheral Type:** `createaddition:redstone_relay`

Create Additions Redstone Relay peripheral. A redstone-controlled energy relay. Provides access to transfer rates, throughput, and powered status.

## Book-Read Methods

### `book_next_get_max_insert` / `read_last_get_max_insert`
Get the maximum input rate (FE/t).
```rust
pub fn book_next_get_max_insert(&mut self)
pub fn read_last_get_max_insert(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_max_extract` / `read_last_get_max_extract`
Get the maximum output rate (FE/t).
```rust
pub fn book_next_get_max_extract(&mut self)
pub fn read_last_get_max_extract(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_get_throughput` / `read_last_get_throughput`
Get the current energy throughput (FE/t).
```rust
pub fn book_next_get_throughput(&mut self)
pub fn read_last_get_throughput(&self) -> Result<f64, PeripheralError>
```
**Returns:** `f64`

---

### `book_next_is_powered` / `read_last_is_powered`
Check if the relay is receiving a redstone signal.
```rust
pub fn book_next_is_powered(&mut self)
pub fn read_last_is_powered(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

## Immediate Methods

None.

## Types

None.

## Usage Example

```rust
use rust_computers_api::createaddition::RedstoneRelay;
use rust_computers_api::peripheral::Peripheral;

let mut relay = RedstoneRelay::wrap(addr);

loop {
    let powered = relay.read_last_is_powered();
    let throughput = relay.read_last_get_throughput();

    relay.book_next_is_powered();
    relay.book_next_get_throughput();
    wait_for_next_tick().await;
}
```
