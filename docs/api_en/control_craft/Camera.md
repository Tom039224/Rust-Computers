# Camera

**Module:** `control_craft::camera`  
**Peripheral Type:** `controlcraft:camera`

Control-Craft Camera peripheral for raycasting, entity detection, and view control. Provides extensive clip/raycast functionality and camera orientation control.

## Book-Read Methods

### Property Getters (with imm support)

All of the following use the `book_read_imm!` macro pattern, providing `book_next_*`, `read_last_*`, and `*_imm` variants.

| Book Method | Read Method | Imm Method | Returns |
|---|---|---|---|
| `book_next_get_abs_view_transform` | `read_last_get_abs_view_transform` | `get_abs_view_transform_imm` | `CTLTransform` |
| `book_next_get_pitch` | `read_last_get_pitch` | `get_pitch_imm` | `f64` |
| `book_next_get_yaw` | `read_last_get_yaw` | `get_yaw_imm` | `f64` |
| `book_next_get_transformed_pitch` | `read_last_get_transformed_pitch` | `get_transformed_pitch_imm` | `f64` |
| `book_next_get_transformed_yaw` | `read_last_get_transformed_yaw` | `get_transformed_yaw_imm` | `f64` |
| `book_next_get_clip_distance` | `read_last_get_clip_distance` | `get_clip_distance_imm` | `f64` |
| `book_next_latest_ship` | `read_last_latest_ship` | `latest_ship_imm` | `Option<Value>` |
| `book_next_latest_player` | `read_last_latest_player` | `latest_player_imm` | `Option<Value>` |
| `book_next_latest_entity` | `read_last_latest_entity` | `latest_entity_imm` | `Option<Value>` |
| `book_next_latest_block` | `read_last_latest_block` | `latest_block_imm` | `Option<Value>` |
| `book_next_get_camera_position` | `read_last_get_camera_position` | `get_camera_position_imm` | `(f64, f64, f64)` |
| `book_next_get_abs_view_forward` | `read_last_get_abs_view_forward` | `get_abs_view_forward_imm` | `(f64, f64, f64)` |
| `book_next_is_being_used` | `read_last_is_being_used` | `is_being_used_imm` | `bool` |
| `book_next_get_direction` | `read_last_get_direction` | `get_direction_imm` | `String` |

---

### Clip Methods (Raycasting)

#### `book_next_clip` / `read_last_clip`
Perform a general clip (raycast) in the camera's view direction.
```rust
pub fn book_next_clip(&mut self) { ... }
pub fn read_last_clip(&self) -> Result<CTLRaycastResult, PeripheralError> { ... }
```
**Returns:** `CTLRaycastResult`

#### `book_next_clip_entity` / `read_last_clip_entity`
Clip targeting entities only.
```rust
pub fn book_next_clip_entity(&mut self) { ... }
pub fn read_last_clip_entity(&self) -> Result<CTLRaycastResult, PeripheralError> { ... }
```

#### `book_next_clip_block` / `read_last_clip_block`
Clip targeting blocks only.
```rust
pub fn book_next_clip_block(&mut self) { ... }
pub fn read_last_clip_block(&self) -> Result<CTLRaycastResult, PeripheralError> { ... }
```

#### `book_next_clip_all_entity` / `read_last_clip_all_entity`
Clip returning all entities hit.
```rust
pub fn book_next_clip_all_entity(&mut self) { ... }
pub fn read_last_clip_all_entity(&self) -> Result<Vec<CTLRaycastResult>, PeripheralError> { ... }
```

#### `book_next_clip_ship` / `read_last_clip_ship`
Clip targeting ships only.
```rust
pub fn book_next_clip_ship(&mut self) { ... }
pub fn read_last_clip_ship(&self) -> Result<CTLRaycastResult, PeripheralError> { ... }
```

#### `book_next_clip_player` / `read_last_clip_player`
Clip targeting players only.
```rust
pub fn book_next_clip_player(&mut self) { ... }
pub fn read_last_clip_player(&self) -> Result<CTLRaycastResult, PeripheralError> { ... }
```

---

### Camera Control

#### `book_next_set_pitch` / `read_last_set_pitch`
Set the camera pitch.
```rust
pub fn book_next_set_pitch(&mut self, degrees: f64) { ... }
pub fn read_last_set_pitch(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `degrees: f64` — Pitch in degrees

#### `book_next_set_yaw` / `read_last_set_yaw`
Set the camera yaw.
```rust
pub fn book_next_set_yaw(&mut self, degrees: f64) { ... }
pub fn read_last_set_yaw(&self) -> Result<(), PeripheralError> { ... }
```
**Parameters:** `degrees: f64` — Yaw in degrees

#### `book_next_force_pitch_yaw` / `read_last_force_pitch_yaw`
Force set both pitch and yaw simultaneously.
```rust
pub fn book_next_force_pitch_yaw(&mut self, pitch: f64, yaw: f64) { ... }
pub fn read_last_force_pitch_yaw(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_outline_to_user` / `read_last_outline_to_user`
Display an outline to the user.
```rust
pub fn book_next_outline_to_user(&mut self) { ... }
pub fn read_last_outline_to_user(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_set_clip_range` / `read_last_set_clip_range`
Set the clip (raycast) range.
```rust
pub fn book_next_set_clip_range(&mut self, range: f64) { ... }
pub fn read_last_set_clip_range(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_set_cone_angle` / `read_last_set_cone_angle`
Set the cone angle for detection.
```rust
pub fn book_next_set_cone_angle(&mut self, angle: f64) { ... }
pub fn read_last_set_cone_angle(&self) -> Result<(), PeripheralError> { ... }
```

---

### Advanced Queries

#### `book_next_raycast` / `read_last_raycast`
Perform a raycast toward a specific coordinate.
```rust
pub fn book_next_raycast(&mut self, x: f64, y: f64, z: f64) { ... }
pub fn read_last_raycast(&self) -> Result<CTLRaycastResult, PeripheralError> { ... }
```

#### `book_next_get_entities` / `read_last_get_entities`
Get entities within a specified radius.
```rust
pub fn book_next_get_entities(&mut self, radius: f64) { ... }
pub fn read_last_get_entities(&self) -> Result<Vec<Value>, PeripheralError> { ... }
```

#### `book_next_get_mobs` / `read_last_get_mobs`
Get mobs within a specified radius.
```rust
pub fn book_next_get_mobs(&mut self, radius: f64) { ... }
pub fn read_last_get_mobs(&self) -> Result<Vec<Value>, PeripheralError> { ... }
```

#### `book_next_reset` / `read_last_reset`
Reset the camera state.
```rust
pub fn book_next_reset(&mut self) { ... }
pub fn read_last_reset(&self) -> Result<(), PeripheralError> { ... }
```

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Types

### `CTLTransform`
4x4 transformation matrix.
```rust
pub struct CTLTransform { pub matrix: [[f64; 4]; 4] }
```

### `CTLRaycastResult`
Raycast hit result.
```rust
pub struct CTLRaycastResult {
    pub hit_type: Option<String>,
    pub pos: Option<(f64, f64, f64)>,
    pub block_pos: Option<(i32, i32, i32)>,
    pub entity_id: Option<String>,
    pub entity_type: Option<String>,
    pub ship_id: Option<i64>,
    pub player_name: Option<String>,
    pub distance: Option<f64>,
}
```

## Usage Example

```rust
use rust_computers_api::control_craft::camera::*;
use rust_computers_api::peripheral::Peripheral;

let mut camera = Camera::find().unwrap();

// Set camera orientation
camera.book_next_force_pitch_yaw(-30.0, 90.0);
wait_for_next_tick().await;
let _ = camera.read_last_force_pitch_yaw();

// Perform a clip
camera.book_next_clip();
wait_for_next_tick().await;
let result = camera.read_last_clip().unwrap();

if let Some(hit_type) = &result.hit_type {
    // Process hit result
}

// Get camera position immediately
let pos = camera.get_camera_position_imm().unwrap();
```
