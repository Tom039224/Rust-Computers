# Ship

**Module:** `cc_vs`  
**Peripheral Type:** `ship`

CC-VS Ship peripheral for Valkyrien Skies integration. Provides read access to ship properties and control over forces, movement, and state.

## Types

```rust
pub struct VSVector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct VSQuaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub struct VSTransformMatrix {
    pub matrix: [[f64; 4]; 4],
}

pub struct VSJoint {
    pub id: u64,
    pub name: String,
}

pub struct VSInertiaInfo {
    pub moment_of_inertia: VSVector3,
    pub mass: f64,
}

pub struct VSPoseVelInfo {
    pub vel: VSVector3,
    pub omega: VSVector3,
    pub pos: VSVector3,
    pub rot: VSQuaternion,
}

pub struct VSPhysicsTickData {
    pub buoyant_factor: f64,
    pub is_static: bool,
    pub do_fluid_drag: bool,
    pub inertia: VSInertiaInfo,
    pub pose_vel: VSPoseVelInfo,
    pub forces_inducers: Vec<String>,
}

pub struct VSTeleportData {
    pub pos: Option<VSVector3>,
    pub rot: Option<VSQuaternion>,
    pub vel: Option<VSVector3>,
    pub omega: Option<VSVector3>,
    pub dimension: Option<String>,
    pub scale: Option<f64>,
}
```

## Methods

### Async Methods (with imm variants)

All getter methods below have both `async` and `_imm` (immediate) variants.

#### `get_id` / `get_id_imm`

```rust
pub async fn get_id(&self) -> Result<i64, PeripheralError>
pub fn get_id_imm(&self) -> Result<i64, PeripheralError>
```

#### `get_mass` / `get_mass_imm`

```rust
pub async fn get_mass(&self) -> Result<f64, PeripheralError>
pub fn get_mass_imm(&self) -> Result<f64, PeripheralError>
```

#### `get_moment_of_inertia_tensor` / `get_moment_of_inertia_tensor_imm`

```rust
pub async fn get_moment_of_inertia_tensor(&self) -> Result<[[f64; 3]; 3], PeripheralError>
pub fn get_moment_of_inertia_tensor_imm(&self) -> Result<[[f64; 3]; 3], PeripheralError>
```

#### `get_slug` / `get_slug_imm`

```rust
pub async fn get_slug(&self) -> Result<String, PeripheralError>
pub fn get_slug_imm(&self) -> Result<String, PeripheralError>
```

#### `get_angular_velocity` / `get_angular_velocity_imm`

```rust
pub async fn get_angular_velocity(&self) -> Result<VSVector3, PeripheralError>
pub fn get_angular_velocity_imm(&self) -> Result<VSVector3, PeripheralError>
```

#### `get_quaternion` / `get_quaternion_imm`

```rust
pub async fn get_quaternion(&self) -> Result<VSQuaternion, PeripheralError>
pub fn get_quaternion_imm(&self) -> Result<VSQuaternion, PeripheralError>
```

#### `get_scale` / `get_scale_imm`

```rust
pub async fn get_scale(&self) -> Result<VSVector3, PeripheralError>
pub fn get_scale_imm(&self) -> Result<VSVector3, PeripheralError>
```

#### `get_shipyard_position` / `get_shipyard_position_imm`

```rust
pub async fn get_shipyard_position(&self) -> Result<VSVector3, PeripheralError>
pub fn get_shipyard_position_imm(&self) -> Result<VSVector3, PeripheralError>
```

#### `get_size` / `get_size_imm`

```rust
pub async fn get_size(&self) -> Result<VSVector3, PeripheralError>
pub fn get_size_imm(&self) -> Result<VSVector3, PeripheralError>
```

#### `get_velocity` / `get_velocity_imm`

```rust
pub async fn get_velocity(&self) -> Result<VSVector3, PeripheralError>
pub fn get_velocity_imm(&self) -> Result<VSVector3, PeripheralError>
```

#### `get_worldspace_position` / `get_worldspace_position_imm`

```rust
pub async fn get_worldspace_position(&self) -> Result<VSVector3, PeripheralError>
pub fn get_worldspace_position_imm(&self) -> Result<VSVector3, PeripheralError>
```

#### `is_static` / `is_static_imm`

```rust
pub async fn is_static(&self) -> Result<bool, PeripheralError>
pub fn is_static_imm(&self) -> Result<bool, PeripheralError>
```

#### `get_transformation_matrix` / `get_transformation_matrix_imm`

```rust
pub async fn get_transformation_matrix(&self) -> Result<VSTransformMatrix, PeripheralError>
pub fn get_transformation_matrix_imm(&self) -> Result<VSTransformMatrix, PeripheralError>
```

#### `get_joints` / `get_joints_imm`

```rust
pub async fn get_joints(&self) -> Result<Vec<VSJoint>, PeripheralError>
pub fn get_joints_imm(&self) -> Result<Vec<VSJoint>, PeripheralError>
```

#### `transform_position_to_world` / `transform_position_to_world_imm`

Convert a local (shipyard) position to world coordinates.

```rust
pub async fn transform_position_to_world(&self, pos: VSVector3) -> Result<VSVector3, PeripheralError>
pub fn transform_position_to_world_imm(&self, pos: VSVector3) -> Result<VSVector3, PeripheralError>
```

### Async Action Methods

#### `set_slug`

Set the ship's slug name.

```rust
pub async fn set_slug(&self, name: &str) -> Result<(), PeripheralError>
```

#### `set_static`

Set the ship's static state.

```rust
pub async fn set_static(&self, is_static: bool) -> Result<(), PeripheralError>
```

#### `set_scale_value`

Set the ship's scale.

```rust
pub async fn set_scale_value(&self, scale: f64) -> Result<(), PeripheralError>
```

#### `teleport`

Teleport the ship.

```rust
pub async fn teleport(&self, data: &VSTeleportData) -> Result<(), PeripheralError>
```

#### `apply_world_force`

Apply a force in world coordinates. Optionally specify a position.

```rust
pub async fn apply_world_force(&self, fx: f64, fy: f64, fz: f64, pos: Option<VSVector3>) -> Result<(), PeripheralError>
```

#### `apply_world_torque`

Apply a torque in world coordinates.

```rust
pub async fn apply_world_torque(&self, tx: f64, ty: f64, tz: f64) -> Result<(), PeripheralError>
```

#### `apply_model_force`

Apply a force in model (local) coordinates.

```rust
pub async fn apply_model_force(&self, fx: f64, fy: f64, fz: f64, pos: Option<VSVector3>) -> Result<(), PeripheralError>
```

#### `apply_model_torque`

Apply a torque in model (local) coordinates.

```rust
pub async fn apply_model_torque(&self, tx: f64, ty: f64, tz: f64) -> Result<(), PeripheralError>
```

## Example

```rust
use rust_computers_api::cc_vs::ship::{Ship, VSVector3};
use rust_computers_api::peripheral::Peripheral;

let ship = Ship::wrap(addr);

// Read ship state (immediate)
let pos = ship.get_worldspace_position_imm()?;
let vel = ship.get_velocity_imm()?;
let mass = ship.get_mass_imm()?;

// Apply force
ship.apply_world_force(0.0, 1000.0, 0.0, None).await?;

// Transform position
let local = VSVector3 { x: 0.0, y: 0.0, z: 0.0 };
let world = ship.transform_position_to_world(local).await?;
```
