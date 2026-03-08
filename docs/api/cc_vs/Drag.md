# Drag

**Module:** `cc_vs`  
**Peripheral Type:** `vs_drag`

CC-VS Drag API for ship drag/lift control. Called from a computer placed on a Valkyrien Skies ship.

## Types

Uses `VSVector3` from the `cc_vs::ship` module:

```rust
pub struct VSVector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```

## Methods

### Async Methods (with imm variants)

#### `get_drag_force` / `get_drag_force_imm`

Get the current drag force vector.

```rust
pub async fn get_drag_force(&self) -> Result<Option<VSVector3>, PeripheralError>
pub fn get_drag_force_imm(&self) -> Result<Option<VSVector3>, PeripheralError>
```

#### `get_lift_force` / `get_lift_force_imm`

Get the current lift force vector.

```rust
pub async fn get_lift_force(&self) -> Result<Option<VSVector3>, PeripheralError>
pub fn get_lift_force_imm(&self) -> Result<Option<VSVector3>, PeripheralError>
```

### Async Action Methods

#### `enable_drag`

Enable drag simulation.

```rust
pub async fn enable_drag(&self) -> Result<(), PeripheralError>
```

#### `disable_drag`

Disable drag simulation.

```rust
pub async fn disable_drag(&self) -> Result<(), PeripheralError>
```

#### `enable_lift`

Enable lift simulation.

```rust
pub async fn enable_lift(&self) -> Result<(), PeripheralError>
```

#### `disable_lift`

Disable lift simulation.

```rust
pub async fn disable_lift(&self) -> Result<(), PeripheralError>
```

#### `enable_rot_drag`

Enable rotational drag.

```rust
pub async fn enable_rot_drag(&self) -> Result<(), PeripheralError>
```

#### `disable_rot_drag`

Disable rotational drag.

```rust
pub async fn disable_rot_drag(&self) -> Result<(), PeripheralError>
```

#### `set_wind_direction`

Set the wind direction vector.

```rust
pub async fn set_wind_direction(&self, x: f64, y: f64, z: f64) -> Result<(), PeripheralError>
```

#### `set_wind_speed`

Set the wind speed.

```rust
pub async fn set_wind_speed(&self, speed: f64) -> Result<(), PeripheralError>
```

#### `apply_wind_impulse`

Apply a wind impulse with direction and speed.

```rust
pub async fn apply_wind_impulse(&self, x: f64, y: f64, z: f64, speed: f64) -> Result<(), PeripheralError>
```

## Example

```rust
use rust_computers_api::cc_vs::drag::Drag;
use rust_computers_api::peripheral::Peripheral;

let drag = Drag::wrap(addr);

// Read forces (immediate)
let drag_force = drag.get_drag_force_imm()?;
let lift_force = drag.get_lift_force_imm()?;

// Control drag/lift
drag.enable_drag().await?;
drag.enable_lift().await?;
drag.set_wind_speed(10.0).await?;
drag.set_wind_direction(1.0, 0.0, 0.0).await?;
```
