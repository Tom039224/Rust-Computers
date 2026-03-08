# NBTStorage

**Module:** `advanced_peripherals::nbt_storage`  
**Peripheral Type:** `advancedPeripherals:nbt_storage`

AdvancedPeripherals NBT Storage peripheral. Provides persistent NBT data storage with read/write capabilities. Supports both SNBT string and table-based write operations.

## Book-Read Methods

### `book_next_read` / `read_last_read`
Read the stored NBT data.
```rust
pub fn book_next_read(&mut self)
pub fn read_last_read(&self) -> Result<Value, PeripheralError>
```
**Returns:** `Value` (dynamic table)

---

### `book_next_write_json` / `read_last_write_json`
Parse and store an SNBT string as NBT data.
```rust
pub fn book_next_write_json(&mut self, snbt: &str)
pub fn read_last_write_json(&self) -> Result<bool, PeripheralError>
```
**Parameters:**
- `snbt: &str` — SNBT (Stringified NBT) string

**Returns:** `bool` — Success status

---

### `book_next_write_table` / `read_last_write_table`
Convert a MsgPack-encoded table to NBT and store it.
```rust
pub fn book_next_write_table(&mut self, table_data: &[u8])
pub fn read_last_write_table(&self) -> Result<bool, PeripheralError>
```
**Parameters:**
- `table_data: &[u8]` — MsgPack-encoded table data

**Returns:** `bool` — Success status

## Immediate Methods

None.

## Types

None.

## Usage Example

```rust
use rust_computers_api::advanced_peripherals::NbtStorage;
use rust_computers_api::peripheral::Peripheral;

let mut storage = NbtStorage::wrap(addr);

// Write SNBT
storage.book_next_write_json(r#"{myKey: "hello"}"#);
wait_for_next_tick().await;
let ok = storage.read_last_write_json();

// Read back
storage.book_next_read();
wait_for_next_tick().await;
let data = storage.read_last_read();
```
