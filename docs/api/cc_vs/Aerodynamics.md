# Aerodynamics

**Module:** `cc_vs`  
**Peripheral Type:** `vs_aerodynamics`

CC-VS Aerodynamics global API. Provides atmospheric and aerodynamic property queries for Valkyrien Skies integration.

## Types

### `VSAtmosphericParameters`

```rust
pub struct VSAtmosphericParameters {
    pub max_y: f64,
    pub sea_level: f64,
    pub gravity: f64,
}
```

## Methods

### Immediate Methods

These methods are available only as immediate (`_imm`) calls and do not use the book/read pattern.

#### `default_max_imm`

Get the default maximum Y value.

```rust
pub fn default_max_imm(&self) -> Result<f64, PeripheralError>
```

#### `default_sea_level_imm`

Get the default sea level.

```rust
pub fn default_sea_level_imm(&self) -> Result<f64, PeripheralError>
```

#### `drag_coefficient_imm`

Get the drag coefficient.

```rust
pub fn drag_coefficient_imm(&self) -> Result<f64, PeripheralError>
```

#### `gravitational_acceleration_imm`

Get the gravitational acceleration constant.

```rust
pub fn gravitational_acceleration_imm(&self) -> Result<f64, PeripheralError>
```

#### `universal_gas_constant_imm`

Get the universal gas constant.

```rust
pub fn universal_gas_constant_imm(&self) -> Result<f64, PeripheralError>
```

#### `air_molar_mass_imm`

Get the air molar mass.

```rust
pub fn air_molar_mass_imm(&self) -> Result<f64, PeripheralError>
```

### Async Methods (with imm variants)

#### `get_atmospheric_parameters` / `get_atmospheric_parameters_imm`

Get atmospheric parameters.

```rust
pub async fn get_atmospheric_parameters(&self) -> Result<Option<VSAtmosphericParameters>, PeripheralError>
pub fn get_atmospheric_parameters_imm(&self) -> Result<Option<VSAtmosphericParameters>, PeripheralError>
```

#### `get_air_density` / `get_air_density_imm`

Get the air density at a given Y coordinate. If `y` is `None`, uses the default.

```rust
pub async fn get_air_density(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError>
pub fn get_air_density_imm(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError>
```

#### `get_air_pressure` / `get_air_pressure_imm`

Get the air pressure at a given Y coordinate.

```rust
pub async fn get_air_pressure(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError>
pub fn get_air_pressure_imm(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError>
```

#### `get_air_temperature` / `get_air_temperature_imm`

Get the air temperature at a given Y coordinate.

```rust
pub async fn get_air_temperature(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError>
pub fn get_air_temperature_imm(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError>
```

## Example

```rust
use rust_computers_api::cc_vs::aerodynamics::Aerodynamics;
use rust_computers_api::peripheral::Peripheral;

let aero = Aerodynamics::wrap(addr);

// Immediate property access
let drag_coeff = aero.drag_coefficient_imm()?;
let gravity = aero.gravitational_acceleration_imm()?;

// Async query
let params = aero.get_atmospheric_parameters().await?;
let density = aero.get_air_density(Some(64.0)).await?;
```
