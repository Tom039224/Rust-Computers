# BallisticAccelerator

**Module:** `some_peripherals`
**Peripheral Type:** `sp:ballistic_accelerator` (the NAME constant)

Some-Peripherals BallisticAccelerator peripheral. Provides ballistic trajectory computation methods for calculating pitch angles, flight times, and drag coefficients. All computation methods have both async and immediate (`_imm`) variants.

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Methods

### Async Methods

These methods are async and internally await a response from the game server.

#### `time_in_air`

Calculate the flight time for a projectile given vertical positions and velocity.

```rust
pub async fn time_in_air(
    &self,
    y_proj: f64,
    y_tgt: f64,
    y_vel: f64,
    gravity: Option<f64>,
    drag: Option<f64>,
    max_steps: Option<u32>,
) -> Result<SPTimeResult, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| y_proj | `f64` | Projectile Y position |
| y_tgt | `f64` | Target Y position |
| y_vel | `f64` | Initial Y velocity |
| gravity | `Option<f64>` | Custom gravity value (optional) |
| drag | `Option<f64>` | Custom drag coefficient (optional) |
| max_steps | `Option<u32>` | Maximum simulation steps (optional) |

#### `try_pitch`

Try a specific pitch angle and return the simulated result.

```rust
pub async fn try_pitch(
    &self,
    pitch: f64,
    speed: f64,
    length: f64,
    dist: f64,
    cannon: SPCoordinate,
    target: SPCoordinate,
    gravity: Option<f64>,
    drag: Option<f64>,
    max_steps: Option<u32>,
) -> Result<(f64, f64, f64), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| pitch | `f64` | The pitch angle to try |
| speed | `f64` | Projectile speed |
| length | `f64` | Barrel length |
| dist | `f64` | Distance to target |
| cannon | `SPCoordinate` | Cannon position |
| target | `SPCoordinate` | Target position |
| gravity | `Option<f64>` | Custom gravity (optional) |
| drag | `Option<f64>` | Custom drag (optional) |
| max_steps | `Option<u32>` | Maximum simulation steps (optional) |

**Returns:** `(f64, f64, f64)` — A tuple of three result values from the pitch simulation.

#### `calculate_pitch`

Calculate the optimal pitch angle for hitting a target.

```rust
pub async fn calculate_pitch(
    &self,
    cannon: SPCoordinate,
    target: SPCoordinate,
    speed: f64,
    length: f64,
    amin: Option<f64>,
    amax: Option<f64>,
    gravity: Option<f64>,
    drag: Option<f64>,
    max_delta_t_error: Option<f64>,
    max_steps: Option<u32>,
    num_iterations: Option<u32>,
    num_elements: Option<u32>,
    check_impossible: Option<bool>,
) -> Result<SPPitchResult, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| cannon | `SPCoordinate` | Cannon position |
| target | `SPCoordinate` | Target position |
| speed | `f64` | Projectile speed |
| length | `f64` | Barrel length |
| amin | `Option<f64>` | Minimum angle bound (optional) |
| amax | `Option<f64>` | Maximum angle bound (optional) |
| gravity | `Option<f64>` | Custom gravity (optional) |
| drag | `Option<f64>` | Custom drag (optional) |
| max_delta_t_error | `Option<f64>` | Max allowed time error (optional) |
| max_steps | `Option<u32>` | Maximum simulation steps (optional) |
| num_iterations | `Option<u32>` | Number of solver iterations (optional) |
| num_elements | `Option<u32>` | Number of elements for solver (optional) |
| check_impossible | `Option<bool>` | Whether to check for impossible shots (optional) |

#### `batch_calculate_pitches`

Calculate pitch angles for multiple targets in a single batch call.

```rust
pub async fn batch_calculate_pitches(
    &self,
    cannon: SPCoordinate,
    targets: &[SPCoordinate],
    speed: f64,
    length: f64,
    amin: Option<f64>,
    amax: Option<f64>,
    gravity: Option<f64>,
    drag: Option<f64>,
    max_delta_t_error: Option<f64>,
    max_steps: Option<u32>,
    num_iterations: Option<u32>,
    num_elements: Option<u32>,
    check_impossible: Option<bool>,
) -> Result<Vec<SPPitchResult>, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| cannon | `SPCoordinate` | Cannon position |
| targets | `&[SPCoordinate]` | Array of target positions |
| speed | `f64` | Projectile speed |
| length | `f64` | Barrel length |
| amin | `Option<f64>` | Minimum angle bound (optional) |
| amax | `Option<f64>` | Maximum angle bound (optional) |
| gravity | `Option<f64>` | Custom gravity (optional) |
| drag | `Option<f64>` | Custom drag (optional) |
| max_delta_t_error | `Option<f64>` | Max allowed time error (optional) |
| max_steps | `Option<u32>` | Maximum simulation steps (optional) |
| num_iterations | `Option<u32>` | Number of solver iterations (optional) |
| num_elements | `Option<u32>` | Number of elements for solver (optional) |
| check_impossible | `Option<bool>` | Whether to check for impossible shots (optional) |

#### `get_drag`

Get the effective drag coefficient.

```rust
pub async fn get_drag(
    &self,
    base_drag: f64,
    dim_drag_multiplier: f64,
) -> Result<f64, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| base_drag | `f64` | Base drag value |
| dim_drag_multiplier | `f64` | Dimensional drag multiplier |

### Immediate Methods

These methods execute synchronously without needing async.

#### `time_in_air_imm`

Calculate flight time immediately (same parameters as `time_in_air`).

```rust
pub fn time_in_air_imm(
    &self,
    y_proj: f64,
    y_tgt: f64,
    y_vel: f64,
    gravity: Option<f64>,
    drag: Option<f64>,
    max_steps: Option<u32>,
) -> Result<SPTimeResult, PeripheralError>
```

#### `try_pitch_imm`

Try a pitch angle immediately (same parameters as `try_pitch`).

```rust
pub fn try_pitch_imm(
    &self,
    pitch: f64,
    speed: f64,
    length: f64,
    dist: f64,
    cannon: SPCoordinate,
    target: SPCoordinate,
    gravity: Option<f64>,
    drag: Option<f64>,
    max_steps: Option<u32>,
) -> Result<(f64, f64, f64), PeripheralError>
```

#### `calculate_pitch_imm`

Calculate optimal pitch immediately (same parameters as `calculate_pitch`).

```rust
pub fn calculate_pitch_imm(
    &self,
    cannon: SPCoordinate,
    target: SPCoordinate,
    speed: f64,
    length: f64,
    amin: Option<f64>,
    amax: Option<f64>,
    gravity: Option<f64>,
    drag: Option<f64>,
    max_delta_t_error: Option<f64>,
    max_steps: Option<u32>,
    num_iterations: Option<u32>,
    num_elements: Option<u32>,
    check_impossible: Option<bool>,
) -> Result<SPPitchResult, PeripheralError>
```

#### `get_drag_imm`

Get drag coefficient immediately (same parameters as `get_drag`).

```rust
pub fn get_drag_imm(
    &self,
    base_drag: f64,
    dim_drag_multiplier: f64,
) -> Result<f64, PeripheralError>
```

## Example

```rust
let cannon = SPCoordinate { x: 0.0, y: 64.0, z: 0.0 };
let target = SPCoordinate { x: 100.0, y: 64.0, z: 50.0 };

// Calculate pitch to hit a target
let result = accelerator.calculate_pitch(
    cannon, target, 10.0, 2.0,
    None, None, None, None, None, None, None, None, None,
).await.unwrap();

// Use immediate variant for quick calculations
let drag = accelerator.get_drag_imm(0.01, 1.0).unwrap();
```

## Types

### SPCoordinate

3D coordinate used throughout Some-Peripherals.

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SPCoordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```

### SPTimeResult

Flight time calculation result.

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SPTimeResult {
    pub ticks: f64,
    pub aux: f64,
}
```

| Field | Type | Description |
|-------|------|-------------|
| ticks | `f64` | Estimated flight time in ticks |
| aux | `f64` | Auxiliary value |

### SPPitchResult

Pitch angle calculation result.

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SPPitchResult {
    pub pitch: f64,
    pub aux: f64,
}
```

| Field | Type | Description |
|-------|------|-------------|
| pitch | `f64` | Calculated pitch angle |
| aux | `f64` | Auxiliary value |

### SPDroneBlueprint

Drone blueprint structure (defined but not used in methods directly).

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPDroneBlueprint {
    pub cannon: SPCoordinate,
    pub target: SPCoordinate,
    pub speed: f64,
    pub length: f64,
}
```
