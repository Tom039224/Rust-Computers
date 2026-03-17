# Spinalyzer

**Module:** `control_craft::spinalyzer`  
**Peripheral Type:** `controlcraft:spinalyzer_peripheral`

Control-Craft Spinalyzer peripheral for reading ship orientation, position, velocity, angular velocity, and physics data, as well as applying forces and torques in both world-space and ship-local coordinate systems.

## Book-Read Methods

### Property Getters (with imm support)

All of the following use the `book_read_imm!` macro pattern, providing `book_next_*`, `read_last_*`, and `*_imm` variants.

| Book Method | Read Method | Imm Method | Returns |
|---|---|---|---|
| `book_next_get_quaternion` | `read_last_get_quaternion` | `get_quaternion_imm` | `CTLQuaternion` |
| `book_next_get_quaternion_j` | `read_last_get_quaternion_j` | `get_quaternion_j_imm` | `CTLQuaternion` |
| `book_next_get_rotation_matrix` | `read_last_get_rotation_matrix` | `get_rotation_matrix_imm` | `[[f64; 3]; 3]` |
| `book_next_get_rotation_matrix_t` | `read_last_get_rotation_matrix_t` | `get_rotation_matrix_t_imm` | `[[f64; 3]; 3]` |
| `book_next_get_velocity` | `read_last_get_velocity` | `get_velocity_imm` | `CTLVec3` |
| `book_next_get_angular_velocity` | `read_last_get_angular_velocity` | `get_angular_velocity_imm` | `CTLVec3` |
| `book_next_get_position` | `read_last_get_position` | `get_position_imm` | `CTLVec3` |
| `book_next_get_spinalyzer_position` | `read_last_get_spinalyzer_position` | `get_spinalyzer_position_imm` | `CTLVec3` |
| `book_next_get_spinalyzer_velocity` | `read_last_get_spinalyzer_velocity` | `get_spinalyzer_velocity_imm` | `CTLVec3` |
| `book_next_get_physics` | `read_last_get_physics` | `get_physics_imm` | `Value` |

---

### Force & Torque Application

#### `book_next_apply_invariant_force` / `read_last_apply_invariant_force`
Apply a world-space fixed-direction force to the ship's center of mass.
```rust
pub fn book_next_apply_invariant_force(&mut self, x: f64, y: f64, z: f64) { ... }
pub fn read_last_apply_invariant_force(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `x: f64`, `y: f64`, `z: f64` — Force vector in world space

#### `book_next_apply_invariant_torque` / `read_last_apply_invariant_torque`
Apply a world-space fixed-direction torque.
```rust
pub fn book_next_apply_invariant_torque(&mut self, x: f64, y: f64, z: f64) { ... }
pub fn read_last_apply_invariant_torque(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `x: f64`, `y: f64`, `z: f64` — Torque vector in world space

#### `book_next_apply_rot_dependent_force` / `read_last_apply_rot_dependent_force`
Apply a force in ship-local coordinates (follows ship rotation).
```rust
pub fn book_next_apply_rot_dependent_force(&mut self, x: f64, y: f64, z: f64) { ... }
pub fn read_last_apply_rot_dependent_force(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `x: f64`, `y: f64`, `z: f64` — Force vector in ship-local space

#### `book_next_apply_rot_dependent_torque` / `read_last_apply_rot_dependent_torque`
Apply a torque in ship-local coordinates (follows ship rotation).
```rust
pub fn book_next_apply_rot_dependent_torque(&mut self, x: f64, y: f64, z: f64) { ... }
pub fn read_last_apply_rot_dependent_torque(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `x: f64`, `y: f64`, `z: f64` — Torque vector in ship-local space

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Types

### `CTLQuaternion`
Quaternion representing rotation.
```rust
pub struct CTLQuaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
```

### `CTLVec3`
3D vector.
```rust
pub struct CTLVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```

## Usage Example

```rust
use rust_computers_api::control_craft::spinalyzer::*;
use rust_computers_api::peripheral::Peripheral;

let mut spin = Spinalyzer::find().unwrap();

// Read ship orientation immediately
let quat = spin.get_quaternion_imm().unwrap();
let pos = spin.get_position_imm().unwrap();
let vel = spin.get_velocity_imm().unwrap();

// Apply upward force in world space
spin.book_next_apply_invariant_force(0.0, 100.0, 0.0);
wait_for_next_tick().await;
let _ = spin.read_last_apply_invariant_force();

// Apply forward force in ship-local space
spin.book_next_apply_rot_dependent_force(0.0, 0.0, 50.0);
wait_for_next_tick().await;
let _ = spin.read_last_apply_rot_dependent_force();
```
