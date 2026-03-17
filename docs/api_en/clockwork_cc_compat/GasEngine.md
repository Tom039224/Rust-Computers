# GasEngine

**Module:** `clockwork_cc_compat`  
**Peripheral Type:** `clockwork:gas_engine`

Clockwork CC Compat Gas Engine peripheral. Provides read access to attached engine count and efficiency.

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Methods

### book/read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.
All methods also have `_imm` immediate variants.

#### `book_next_get_attached_engines` / `read_last_get_attached_engines` / `get_attached_engines_imm`

Get the number of attached engines.

```rust
pub fn book_next_get_attached_engines(&mut self)
pub fn read_last_get_attached_engines(&self) -> Result<u32, PeripheralError>
pub fn get_attached_engines_imm(&self) -> Result<u32, PeripheralError>
```

#### `book_next_get_total_efficiency` / `read_last_get_total_efficiency` / `get_total_efficiency_imm`

Get the total efficiency.

```rust
pub fn book_next_get_total_efficiency(&mut self)
pub fn read_last_get_total_efficiency(&self) -> Result<f64, PeripheralError>
pub fn get_total_efficiency_imm(&self) -> Result<f64, PeripheralError>
```

## Example

```rust
use rust_computers_api::clockwork_cc_compat::gas_engine::GasEngine;
use rust_computers_api::peripheral::Peripheral;

let mut engine = GasEngine::wrap(addr);

loop {
    let engines = engine.read_last_get_attached_engines();
    let efficiency = engine.read_last_get_total_efficiency();

    engine.book_next_get_attached_engines();
    engine.book_next_get_total_efficiency();
    wait_for_next_tick().await;
}
```
