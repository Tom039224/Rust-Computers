# Boiler

**Module:** `clockwork_cc_compat`  
**Peripheral Type:** `clockwork:boiler`

Clockwork CC Compat Boiler peripheral. Provides read access to boiler state including heat, water supply, and engine information.

## Types

```rust
pub struct CLFluidInfo {
    pub fluid: String,
    pub amount: u32,
    pub capacity: u32,
}

pub struct CLPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
```

## Methods

### book/read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.
All methods also have `_imm` immediate variants.

#### `book_next_is_active` / `read_last_is_active` / `is_active_imm`

Check if the boiler is active.

```rust
pub fn book_next_is_active(&mut self)
pub fn read_last_is_active(&self) -> Result<bool, PeripheralError>
pub fn is_active_imm(&self) -> Result<bool, PeripheralError>
```

#### `book_next_get_heat_level` / `read_last_get_heat_level` / `get_heat_level_imm`

Get the current heat level.

```rust
pub fn book_next_get_heat_level(&mut self)
pub fn read_last_get_heat_level(&self) -> Result<f64, PeripheralError>
pub fn get_heat_level_imm(&self) -> Result<f64, PeripheralError>
```

#### `book_next_get_active_heat` / `read_last_get_active_heat` / `get_active_heat_imm`

Get the active heat value.

```rust
pub fn book_next_get_active_heat(&mut self)
pub fn read_last_get_active_heat(&self) -> Result<f64, PeripheralError>
pub fn get_active_heat_imm(&self) -> Result<f64, PeripheralError>
```

#### `book_next_is_passive_heat` / `read_last_is_passive_heat` / `is_passive_heat_imm`

Check if using passive heat.

```rust
pub fn book_next_is_passive_heat(&mut self)
pub fn read_last_is_passive_heat(&self) -> Result<bool, PeripheralError>
pub fn is_passive_heat_imm(&self) -> Result<bool, PeripheralError>
```

#### `book_next_get_water_supply` / `read_last_get_water_supply` / `get_water_supply_imm`

Get the water supply level.

```rust
pub fn book_next_get_water_supply(&mut self)
pub fn read_last_get_water_supply(&self) -> Result<f64, PeripheralError>
pub fn get_water_supply_imm(&self) -> Result<f64, PeripheralError>
```

#### `book_next_get_attached_engines` / `read_last_get_attached_engines` / `get_attached_engines_imm`

Get the number of attached engines.

```rust
pub fn book_next_get_attached_engines(&mut self)
pub fn read_last_get_attached_engines(&self) -> Result<u32, PeripheralError>
pub fn get_attached_engines_imm(&self) -> Result<u32, PeripheralError>
```

#### `book_next_get_attached_whistles` / `read_last_get_attached_whistles` / `get_attached_whistles_imm`

Get the number of attached whistles.

```rust
pub fn book_next_get_attached_whistles(&mut self)
pub fn read_last_get_attached_whistles(&self) -> Result<u32, PeripheralError>
pub fn get_attached_whistles_imm(&self) -> Result<u32, PeripheralError>
```

#### `book_next_get_engine_efficiency` / `read_last_get_engine_efficiency` / `get_engine_efficiency_imm`

Get the engine efficiency.

```rust
pub fn book_next_get_engine_efficiency(&mut self)
pub fn read_last_get_engine_efficiency(&self) -> Result<f64, PeripheralError>
pub fn get_engine_efficiency_imm(&self) -> Result<f64, PeripheralError>
```

#### `book_next_get_boiler_size` / `read_last_get_boiler_size` / `get_boiler_size_imm`

Get the boiler size.

```rust
pub fn book_next_get_boiler_size(&mut self)
pub fn read_last_get_boiler_size(&self) -> Result<f64, PeripheralError>
pub fn get_boiler_size_imm(&self) -> Result<f64, PeripheralError>
```

#### `book_next_get_width` / `read_last_get_width` / `get_width_imm`

Get the boiler width.

```rust
pub fn book_next_get_width(&mut self)
pub fn read_last_get_width(&self) -> Result<u32, PeripheralError>
pub fn get_width_imm(&self) -> Result<u32, PeripheralError>
```

#### `book_next_get_height` / `read_last_get_height` / `get_height_imm`

Get the boiler height.

```rust
pub fn book_next_get_height(&mut self)
pub fn read_last_get_height(&self) -> Result<u32, PeripheralError>
pub fn get_height_imm(&self) -> Result<u32, PeripheralError>
```

#### `book_next_get_max_heat_for_size` / `read_last_get_max_heat_for_size` / `get_max_heat_for_size_imm`

Get the maximum heat for the current size.

```rust
pub fn book_next_get_max_heat_for_size(&mut self)
pub fn read_last_get_max_heat_for_size(&self) -> Result<f64, PeripheralError>
pub fn get_max_heat_for_size_imm(&self) -> Result<f64, PeripheralError>
```

#### `book_next_get_max_heat_for_water` / `read_last_get_max_heat_for_water` / `get_max_heat_for_water_imm`

Get the maximum heat for water.

```rust
pub fn book_next_get_max_heat_for_water(&mut self)
pub fn read_last_get_max_heat_for_water(&self) -> Result<f64, PeripheralError>
pub fn get_max_heat_for_water_imm(&self) -> Result<f64, PeripheralError>
```

#### `book_next_get_fill_state` / `read_last_get_fill_state` / `get_fill_state_imm`

Get the fill state.

```rust
pub fn book_next_get_fill_state(&mut self)
pub fn read_last_get_fill_state(&self) -> Result<f64, PeripheralError>
pub fn get_fill_state_imm(&self) -> Result<f64, PeripheralError>
```

#### `book_next_get_fluid_contents` / `read_last_get_fluid_contents` / `get_fluid_contents_imm`

Get the fluid contents information.

```rust
pub fn book_next_get_fluid_contents(&mut self)
pub fn read_last_get_fluid_contents(&self) -> Result<CLFluidInfo, PeripheralError>
pub fn get_fluid_contents_imm(&self) -> Result<CLFluidInfo, PeripheralError>
```

#### `book_next_get_controller_pos` / `read_last_get_controller_pos` / `get_controller_pos_imm`

Get the controller block position.

```rust
pub fn book_next_get_controller_pos(&mut self)
pub fn read_last_get_controller_pos(&self) -> Result<CLPosition, PeripheralError>
pub fn get_controller_pos_imm(&self) -> Result<CLPosition, PeripheralError>
```

## Example

```rust
use rust_computers_api::clockwork_cc_compat::boiler::Boiler;
use rust_computers_api::peripheral::Peripheral;

let mut boiler = Boiler::wrap(addr);

loop {
    let heat = boiler.read_last_get_heat_level();
    let active = boiler.read_last_is_active();

    boiler.book_next_get_heat_level();
    boiler.book_next_is_active();
    wait_for_next_tick().await;
}
```
