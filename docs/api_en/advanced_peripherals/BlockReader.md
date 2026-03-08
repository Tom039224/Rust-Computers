# BlockReader

**Module:** `advanced_peripherals::block_reader`  
**Peripheral Type:** `advancedPeripherals:block_reader`

AdvancedPeripherals Block Reader peripheral. Reads information about the block in front of the peripheral, including block name, NBT data, block states, and tile entity detection.

## Book-Read Methods

### `book_next_get_block_name` / `read_last_get_block_name`
Get the block resource ID (e.g. `minecraft:stone`).
```rust
pub fn book_next_get_block_name(&mut self)
pub fn read_last_get_block_name(&self) -> Result<String, PeripheralError>
```
**Returns:** `String`

---

### `book_next_get_block_data` / `read_last_get_block_data`
Get the block's NBT data as a dynamic table.
```rust
pub fn book_next_get_block_data(&mut self)
pub fn read_last_get_block_data(&self) -> Result<Value, PeripheralError>
```
**Returns:** `Value` (dynamic table)

---

### `book_next_get_block_states` / `read_last_get_block_states`
Get the block state properties.
```rust
pub fn book_next_get_block_states(&mut self)
pub fn read_last_get_block_states(&self) -> Result<Value, PeripheralError>
```
**Returns:** `Value` (dynamic table)

---

### `book_next_is_tile_entity` / `read_last_is_tile_entity`
Check if the block is a tile entity (block entity).
```rust
pub fn book_next_is_tile_entity(&mut self)
pub fn read_last_is_tile_entity(&self) -> Result<bool, PeripheralError>
```
**Returns:** `bool`

## Immediate Methods

None.

## Types

None.

## Usage Example

```rust
use rust_computers_api::advanced_peripherals::BlockReader;
use rust_computers_api::peripheral::Peripheral;

let mut reader = BlockReader::wrap(addr);

loop {
    let name = reader.read_last_get_block_name();
    let is_te = reader.read_last_is_tile_entity();

    reader.book_next_get_block_name();
    reader.book_next_is_tile_entity();
    wait_for_next_tick().await;
}
```
