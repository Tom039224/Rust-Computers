# Ship

**Module:** `cc_vs::ship`  
**Peripheral Type:** `ship`

CC-VS Ship peripheral for controlling Valkyrien Skies ships. Provides access to ship properties, coordinate transformations, force/torque application, and physics tick events.

## Book-Read Methods

### Property Getters (with imm support)

All of the following use the `book_read_imm!` macro pattern, providing `book_next_*`, `read_last_*`, and `*_imm` variants.

| Book Method | Read Method | Imm Method | Returns |
|---|---|---|---|
| `book_next_get_id` | `read_last_get_id` | `get_id_imm` | `i64` |
| `book_next_get_mass` | `read_last_get_mass` | `get_mass_imm` | `f64` |
| `book_next_get_moment_of_inertia_tensor` | `read_last_get_moment_of_inertia_tensor` | `get_moment_of_inertia_tensor_imm` | `[[f64; 3]; 3]` |
| `book_next_get_slug` | `read_last_get_slug` | `get_slug_imm` | `String` |
| `book_next_get_angular_velocity` | `read_last_get_angular_velocity` | `get_angular_velocity_imm` | `VSVector3` |
| `book_next_get_quaternion` | `read_last_get_quaternion` | `get_quaternion_imm` | `VSQuaternion` |
| `book_next_get_scale` | `read_last_get_scale` | `get_scale_imm` | `VSVector3` |
| `book_next_get_shipyard_position` | `read_last_get_shipyard_position` | `get_shipyard_position_imm` | `VSVector3` |
| `book_next_get_size` | `read_last_get_size` | `get_size_imm` | `VSVector3` |
| `book_next_get_velocity` | `read_last_get_velocity` | `get_velocity_imm` | `VSVector3` |
| `book_next_get_worldspace_position` | `read_last_get_worldspace_position` | `get_worldspace_position_imm` | `VSVector3` |
| `book_next_is_static` | `read_last_is_static` | `is_static_imm` | `bool` |
| `book_next_get_transformation_matrix` | `read_last_get_transformation_matrix` | `get_transformation_matrix_imm` | `VSTransformMatrix` |
| `book_next_get_joints` | `read_last_get_joints` | `get_joints_imm` | `Vec<VSJoint>` |

---

### `book_next_transform_position_to_world` / `read_last_transform_position_to_world`
Convert a local (shipyard) position to world coordinates.
```rust
pub fn book_next_transform_position_to_world(&mut self, pos: VSVector3) { ... }
pub fn read_last_transform_position_to_world(&self) -> Result<VSVector3, PeripheralError> { ... }
```
**Parameters:**
- `pos: VSVector3` — Local position

**Returns:** `VSVector3` — World position

---

### `book_next_set_slug` / `read_last_set_slug`
Set the ship's slug name.
```rust
pub fn book_next_set_slug(&mut self, name: &str) { ... }
pub fn read_last_set_slug(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `name: &str` — New slug name

**Returns:** `()`

---

### `book_next_set_static` / `read_last_set_static`
Set the ship's static state.
```rust
pub fn book_next_set_static(&mut self, is_static: bool) { ... }
pub fn read_last_set_static(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `is_static: bool` — Whether the ship should be static

**Returns:** `()`

---

### `book_next_set_scale_value` / `read_last_set_scale_value`
Set the ship's scale.
```rust
pub fn book_next_set_scale_value(&mut self, scale: f64) { ... }
pub fn read_last_set_scale_value(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `scale: f64` — Scale value

**Returns:** `()`

---

### `book_next_teleport` / `read_last_teleport`
Teleport the ship.
```rust
pub fn book_next_teleport(&mut self, data: &VSTeleportData) -> Result<(), PeripheralError> { ... }
pub fn read_last_teleport(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:**
- `data: &VSTeleportData` — Teleport parameters (all fields optional)

**Returns:** `()`

---

### Force/Torque Application

#### `book_next_apply_world_force` / `read_last_apply_world_force`
Apply a force in world coordinates, optionally at a specific position.
```rust
pub fn book_next_apply_world_force(&mut self, fx: f64, fy: f64, fz: f64, pos: Option<VSVector3>) { ... }
pub fn read_last_apply_world_force(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_apply_world_torque` / `read_last_apply_world_torque`
Apply a torque in world coordinates.
```rust
pub fn book_next_apply_world_torque(&mut self, tx: f64, ty: f64, tz: f64) { ... }
pub fn read_last_apply_world_torque(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_apply_model_force` / `read_last_apply_model_force`
Apply a force in model (shipyard) coordinates.
```rust
pub fn book_next_apply_model_force(&mut self, fx: f64, fy: f64, fz: f64, pos: Option<VSVector3>) { ... }
pub fn read_last_apply_model_force(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_apply_model_torque` / `read_last_apply_model_torque`
Apply a torque in model coordinates.
```rust
pub fn book_next_apply_model_torque(&mut self, tx: f64, ty: f64, tz: f64) { ... }
pub fn read_last_apply_model_torque(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_apply_world_force_to_model_pos` / `read_last_apply_world_force_to_model_pos`
Apply a world-coordinate force at a model-coordinate position.
```rust
pub fn book_next_apply_world_force_to_model_pos(&mut self, fx: f64, fy: f64, fz: f64, px: f64, py: f64, pz: f64) { ... }
pub fn read_last_apply_world_force_to_model_pos(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_apply_body_force` / `read_last_apply_body_force`
Apply a force in body coordinates.
```rust
pub fn book_next_apply_body_force(&mut self, fx: f64, fy: f64, fz: f64, pos: Option<VSVector3>) { ... }
pub fn read_last_apply_body_force(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_apply_body_torque` / `read_last_apply_body_torque`
Apply a torque in body coordinates.
```rust
pub fn book_next_apply_body_torque(&mut self, tx: f64, ty: f64, tz: f64) { ... }
pub fn read_last_apply_body_torque(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_apply_world_force_to_body_pos` / `read_last_apply_world_force_to_body_pos`
Apply a world-coordinate force at a body-coordinate position.
```rust
pub fn book_next_apply_world_force_to_body_pos(&mut self, fx: f64, fy: f64, fz: f64, px: f64, py: f64, pz: f64) { ... }
pub fn read_last_apply_world_force_to_body_pos(&self) -> Result<(), PeripheralError> { ... }
```

---

### `book_next_try_pull_physics_ticks` / `read_last_try_pull_physics_ticks`
Try to receive a physics tick event within 1 tick.
```rust
pub fn book_next_try_pull_physics_ticks(&mut self) { ... }
pub fn read_last_try_pull_physics_ticks(&self) -> Result<Option<VSPhysicsTickData>, PeripheralError> { ... }
```
**Returns:** `Option<VSPhysicsTickData>`

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Immediate Methods

### `transform_position_to_world_imm`
Immediately convert a local position to world coordinates.
```rust
pub fn transform_position_to_world_imm(&self, pos: VSVector3) -> Result<VSVector3, PeripheralError> { ... }
```

All property getters listed in the table above also have `*_imm` variants.

## Event-Wait Methods

### `pull_physics_ticks`
Wait for a physics tick event (async). Polls each tick until data arrives.
```rust
pub async fn pull_physics_ticks(&self) -> Result<VSPhysicsTickData, PeripheralError> { ... }
```
**Returns:** `VSPhysicsTickData`

## Types

### `VSVector3`
```rust
pub struct VSVector3 { pub x: f64, pub y: f64, pub z: f64 }
```

### `VSQuaternion`
```rust
pub struct VSQuaternion { pub x: f64, pub y: f64, pub z: f64, pub w: f64 }
```

### `VSTransformMatrix`
```rust
pub struct VSTransformMatrix { pub matrix: [[f64; 4]; 4] }
```

### `VSJoint`
```rust
pub struct VSJoint { pub id: u64, pub name: String }
```

### `VSInertiaInfo`
```rust
pub struct VSInertiaInfo { pub moment_of_inertia: VSVector3, pub mass: f64 }
```

### `VSPoseVelInfo`
```rust
pub struct VSPoseVelInfo {
    pub vel: VSVector3, pub omega: VSVector3,
    pub pos: VSVector3, pub rot: VSQuaternion,
}
```

### `VSPhysicsTickData`
```rust
pub struct VSPhysicsTickData {
    pub buoyant_factor: f64,
    pub is_static: bool,
    pub do_fluid_drag: bool,
    pub inertia: VSInertiaInfo,
    pub pose_vel: VSPoseVelInfo,
    pub forces_inducers: Vec<String>,
}
```

### `VSTeleportData`
```rust
pub struct VSTeleportData {
    pub pos: Option<VSVector3>,
    pub rot: Option<VSQuaternion>,
    pub vel: Option<VSVector3>,
    pub omega: Option<VSVector3>,
    pub dimension: Option<String>,
    pub scale: Option<f64>,
}
```

## Usage Example

```rust
use rust_computers_api::cc_vs::ship::*;
use rust_computers_api::peripheral::Peripheral;

let mut ship = Ship::find().unwrap();

// Get ship position immediately
let pos = ship.get_worldspace_position_imm().unwrap();

// Apply upward force
ship.book_next_apply_world_force(0.0, 1000.0, 0.0, None);
wait_for_next_tick().await;
let _ = ship.read_last_apply_world_force();

// Wait for physics tick
let tick_data = ship.pull_physics_ticks().await.unwrap();
```
