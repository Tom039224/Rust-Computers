# Camera

**Module:** `control_craft`  
**Peripheral Type:** `controlcraft:camera`

Control-Craft Camera peripheral. Provides view transforms, raycasting, entity detection, and camera control.

## Types

```rust
pub struct CTLTransform {
    pub matrix: [[f64; 4]; 4],
}

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

## Methods

### Async Methods (with imm variants)

All getter methods below have both `async` and `_imm` (immediate) variants.

#### `get_abs_view_transform` / `get_abs_view_transform_imm`

Get the absolute view transform matrix.

```rust
pub async fn get_abs_view_transform(&self) -> Result<CTLTransform, PeripheralError>
pub fn get_abs_view_transform_imm(&self) -> Result<CTLTransform, PeripheralError>
```

#### `get_pitch` / `get_pitch_imm`

```rust
pub async fn get_pitch(&self) -> Result<f64, PeripheralError>
pub fn get_pitch_imm(&self) -> Result<f64, PeripheralError>
```

#### `get_yaw` / `get_yaw_imm`

```rust
pub async fn get_yaw(&self) -> Result<f64, PeripheralError>
pub fn get_yaw_imm(&self) -> Result<f64, PeripheralError>
```

#### `get_transformed_pitch` / `get_transformed_pitch_imm`

```rust
pub async fn get_transformed_pitch(&self) -> Result<f64, PeripheralError>
pub fn get_transformed_pitch_imm(&self) -> Result<f64, PeripheralError>
```

#### `get_transformed_yaw` / `get_transformed_yaw_imm`

```rust
pub async fn get_transformed_yaw(&self) -> Result<f64, PeripheralError>
pub fn get_transformed_yaw_imm(&self) -> Result<f64, PeripheralError>
```

#### `get_clip_distance` / `get_clip_distance_imm`

```rust
pub async fn get_clip_distance(&self) -> Result<f64, PeripheralError>
pub fn get_clip_distance_imm(&self) -> Result<f64, PeripheralError>
```

#### `latest_ship` / `latest_ship_imm`

Get the latest detected ship data.

```rust
pub async fn latest_ship(&self) -> Result<Option<Value>, PeripheralError>
pub fn latest_ship_imm(&self) -> Result<Option<Value>, PeripheralError>
```

#### `latest_player` / `latest_player_imm`

Get the latest detected player data.

```rust
pub async fn latest_player(&self) -> Result<Option<Value>, PeripheralError>
pub fn latest_player_imm(&self) -> Result<Option<Value>, PeripheralError>
```

#### `latest_entity` / `latest_entity_imm`

Get the latest detected entity data.

```rust
pub async fn latest_entity(&self) -> Result<Option<Value>, PeripheralError>
pub fn latest_entity_imm(&self) -> Result<Option<Value>, PeripheralError>
```

#### `latest_block` / `latest_block_imm`

Get the latest detected block data.

```rust
pub async fn latest_block(&self) -> Result<Option<Value>, PeripheralError>
pub fn latest_block_imm(&self) -> Result<Option<Value>, PeripheralError>
```

#### `get_camera_position` / `get_camera_position_imm`

```rust
pub async fn get_camera_position(&self) -> Result<(f64, f64, f64), PeripheralError>
pub fn get_camera_position_imm(&self) -> Result<(f64, f64, f64), PeripheralError>
```

#### `get_abs_view_forward` / `get_abs_view_forward_imm`

```rust
pub async fn get_abs_view_forward(&self) -> Result<(f64, f64, f64), PeripheralError>
pub fn get_abs_view_forward_imm(&self) -> Result<(f64, f64, f64), PeripheralError>
```

#### `is_being_used` / `is_being_used_imm`

```rust
pub async fn is_being_used(&self) -> Result<bool, PeripheralError>
pub fn is_being_used_imm(&self) -> Result<bool, PeripheralError>
```

#### `get_direction` / `get_direction_imm`

```rust
pub async fn get_direction(&self) -> Result<String, PeripheralError>
pub fn get_direction_imm(&self) -> Result<String, PeripheralError>
```

### Async Clip/Raycast Methods

#### `clip`

Full raycast (blocks + entities).

```rust
pub async fn clip(&self) -> Result<CTLRaycastResult, PeripheralError>
```

#### `clip_entity`

Raycast targeting entities only.

```rust
pub async fn clip_entity(&self) -> Result<CTLRaycastResult, PeripheralError>
```

#### `clip_block`

Raycast targeting blocks only.

```rust
pub async fn clip_block(&self) -> Result<CTLRaycastResult, PeripheralError>
```

#### `clip_all_entity`

Raycast returning all entities hit.

```rust
pub async fn clip_all_entity(&self) -> Result<Vec<CTLRaycastResult>, PeripheralError>
```

#### `clip_ship`

Raycast targeting ships only.

```rust
pub async fn clip_ship(&self) -> Result<CTLRaycastResult, PeripheralError>
```

#### `clip_player`

Raycast targeting players only.

```rust
pub async fn clip_player(&self) -> Result<CTLRaycastResult, PeripheralError>
```

#### `raycast`

Raycast to a specific coordinate.

```rust
pub async fn raycast(&self, x: f64, y: f64, z: f64) -> Result<CTLRaycastResult, PeripheralError>
```

#### `get_entities`

Get all entities within a radius.

```rust
pub async fn get_entities(&self, radius: f64) -> Result<Vec<Value>, PeripheralError>
```

#### `get_mobs`

Get all mobs within a radius.

```rust
pub async fn get_mobs(&self, radius: f64) -> Result<Vec<Value>, PeripheralError>
```

### Async Action Methods

#### `set_pitch`

Set the camera pitch.

```rust
pub async fn set_pitch(&self, degrees: f64) -> Result<(), PeripheralError>
```

#### `set_yaw`

Set the camera yaw.

```rust
pub async fn set_yaw(&self, degrees: f64) -> Result<(), PeripheralError>
```

#### `outline_to_user`

Show outline to the current user.

```rust
pub async fn outline_to_user(&self) -> Result<(), PeripheralError>
```

#### `force_pitch_yaw`

Force set both pitch and yaw.

```rust
pub async fn force_pitch_yaw(&self, pitch: f64, yaw: f64) -> Result<(), PeripheralError>
```

#### `set_clip_range`

Set the clip (raycast) range.

```rust
pub async fn set_clip_range(&self, range: f64) -> Result<(), PeripheralError>
```

#### `set_cone_angle`

Set the detection cone angle.

```rust
pub async fn set_cone_angle(&self, angle: f64) -> Result<(), PeripheralError>
```

#### `reset`

Reset the camera to default state.

```rust
pub async fn reset(&self) -> Result<(), PeripheralError>
```

## Example

```rust
use rust_computers_api::control_craft::camera::Camera;
use rust_computers_api::peripheral::Peripheral;

let camera = Camera::wrap(addr);

// Get camera position
let pos = camera.get_camera_position_imm()?;

// Look at target
camera.force_pitch_yaw(-30.0, 45.0).await?;

// Raycast
let hit = camera.clip().await?;
if let Some(hit_type) = &hit.hit_type {
    // Process hit...
}

// Get nearby entities
let entities = camera.get_entities(32.0).await?;
```
