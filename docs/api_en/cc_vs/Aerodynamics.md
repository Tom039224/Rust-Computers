# Aerodynamics

**Module:** `cc_vs::aerodynamics`  
**Peripheral Type:** `vs_aerodynamics`

CC-VS Aerodynamics global API for querying atmospheric and aerodynamic properties. Provides both immediate-only constant properties and book-read methods for computed values.

## Immediate Methods (Constants)

These properties are read-only constants available only via immediate calls.

### `default_max_imm`
Get the default maximum altitude.
```rust
pub fn default_max_imm(&self) -> Result<f64, PeripheralError> { ... }
```

### `default_sea_level_imm`
Get the default sea level value.
```rust
pub fn default_sea_level_imm(&self) -> Result<f64, PeripheralError> { ... }
```

### `drag_coefficient_imm`
Get the drag coefficient.
```rust
pub fn drag_coefficient_imm(&self) -> Result<f64, PeripheralError> { ... }
```

### `gravitational_acceleration_imm`
Get the gravitational acceleration constant.
```rust
pub fn gravitational_acceleration_imm(&self) -> Result<f64, PeripheralError> { ... }
```

### `universal_gas_constant_imm`
Get the universal gas constant.
```rust
pub fn universal_gas_constant_imm(&self) -> Result<f64, PeripheralError> { ... }
```

### `air_molar_mass_imm`
Get the air molar mass.
```rust
pub fn air_molar_mass_imm(&self) -> Result<f64, PeripheralError> { ... }
```

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Book-Read Methods

### `book_next_get_atmospheric_parameters` / `read_last_get_atmospheric_parameters`
Get atmospheric parameters.
```rust
pub fn book_next_get_atmospheric_parameters(&mut self) { ... }
pub fn read_last_get_atmospheric_parameters(&self) -> Result<Option<VSAtmosphericParameters>, PeripheralError> { ... }
```
**Returns:** `Option<VSAtmosphericParameters>`

---

### `book_next_get_air_density` / `read_last_get_air_density`
Get air density at a specified Y coordinate (or current position if `None`).
```rust
pub fn book_next_get_air_density(&mut self, y: Option<f64>) { ... }
pub fn read_last_get_air_density(&self) -> Result<Option<f64>, PeripheralError> { ... }
```
**Parameters:**
- `y: Option<f64>` — Y coordinate (optional; uses current position if omitted)

**Returns:** `Option<f64>`

---

### `book_next_get_air_pressure` / `read_last_get_air_pressure`
Get air pressure at a specified Y coordinate.
```rust
pub fn book_next_get_air_pressure(&mut self, y: Option<f64>) { ... }
pub fn read_last_get_air_pressure(&self) -> Result<Option<f64>, PeripheralError> { ... }
```
**Parameters:**
- `y: Option<f64>` — Y coordinate (optional)

**Returns:** `Option<f64>`

---

### `book_next_get_air_temperature` / `read_last_get_air_temperature`
Get air temperature at a specified Y coordinate.
```rust
pub fn book_next_get_air_temperature(&mut self, y: Option<f64>) { ... }
pub fn read_last_get_air_temperature(&self) -> Result<Option<f64>, PeripheralError> { ... }
```
**Parameters:**
- `y: Option<f64>` — Y coordinate (optional)

**Returns:** `Option<f64>`

## Immediate Methods (Computed)

### `get_atmospheric_parameters_imm`
```rust
pub fn get_atmospheric_parameters_imm(&self) -> Result<Option<VSAtmosphericParameters>, PeripheralError> { ... }
```

### `get_air_density_imm`
```rust
pub fn get_air_density_imm(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError> { ... }
```

### `get_air_pressure_imm`
```rust
pub fn get_air_pressure_imm(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError> { ... }
```

### `get_air_temperature_imm`
```rust
pub fn get_air_temperature_imm(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError> { ... }
```

## Types

### `VSAtmosphericParameters`
```rust
pub struct VSAtmosphericParameters {
    pub max_y: f64,
    pub sea_level: f64,
    pub gravity: f64,
}
```

## Usage Example

```rust
use rust_computers_api::cc_vs::aerodynamics::*;
use rust_computers_api::peripheral::Peripheral;

let mut aero = Aerodynamics::find().unwrap();

// Read constants immediately
let gravity = aero.gravitational_acceleration_imm().unwrap();
let drag_coeff = aero.drag_coefficient_imm().unwrap();

// Get air density at Y=100
aero.book_next_get_air_density(Some(100.0));
wait_for_next_tick().await;
let density = aero.read_last_get_air_density().unwrap();

// Or use immediate variant
let pressure = aero.get_air_pressure_imm(Some(64.0)).unwrap();
```
