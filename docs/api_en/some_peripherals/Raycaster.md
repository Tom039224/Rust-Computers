# Raycaster

**Module:** `some_peripherals`
**Peripheral Type:** `sp:raycaster` (the NAME constant)

Some-Peripherals Raycaster peripheral. Casts rays into the world to detect blocks, entities, and Valkyrien Skies ships. Also provides configuration and facing direction queries.

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Methods

### Async Methods

These methods are async and internally await a response from the game server.

#### `raycast`

Perform a raycast from the block's position.

```rust
pub async fn raycast(
    &self,
    distance: f64,
    variables: Option<(f64, f64, Option<f64>)>,
    euler_mode: Option<bool>,
    im_execute: Option<bool>,
    check_for_blocks: Option<bool>,
    only_distance: Option<bool>,
) -> Result<SPRaycastResult, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| distance | `f64` | Maximum ray distance |
| variables | `Option<(f64, f64, Option<f64>)>` | Direction variables — yaw, pitch, and optional roll (optional) |
| euler_mode | `Option<bool>` | Whether to use Euler angle mode (optional) |
| im_execute | `Option<bool>` | Whether to execute immediately (optional) |
| check_for_blocks | `Option<bool>` | Whether to check for block hits (optional) |
| only_distance | `Option<bool>` | Whether to only return distance info (optional) |

#### `add_stickers`

Set the powered state of the raycaster's stickers.

```rust
pub async fn add_stickers(&self, state: bool) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| state | `bool` | The powered state to set |

#### `get_config_info`

Get raycaster configuration information.

```rust
pub async fn get_config_info(&self) -> Result<BTreeMap<String, String>, PeripheralError>
```

#### `get_facing_direction`

Get the facing direction of the raycaster block.

```rust
pub async fn get_facing_direction(&self) -> Result<String, PeripheralError>
```

### Immediate Methods

These methods execute synchronously without needing async.

#### `get_config_info_imm`

Get raycaster configuration information immediately.

```rust
pub fn get_config_info_imm(&self) -> Result<BTreeMap<String, String>, PeripheralError>
```

#### `get_facing_direction_imm`

Get the facing direction immediately.

```rust
pub fn get_facing_direction_imm(&self) -> Result<String, PeripheralError>
```

## Example

```rust
// Simple raycast forward 100 blocks
let result = raycaster.raycast(100.0, None, None, None, None, None).await.unwrap();

if let Some(true) = result.is_block {
    if let Some(ref block_type) = result.block_type {
        // Hit a block
    }
}

// Get facing direction immediately
let direction = raycaster.get_facing_direction_imm().unwrap();
```

## Types

### SPRaycastResult

Raycast hit result. Fields are optional depending on what was hit and the raycast parameters.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPRaycastResult {
    pub is_block: Option<bool>,
    pub is_entity: Option<bool>,
    pub abs_pos: Option<SPCoordinate>,
    pub hit_pos: Option<SPCoordinate>,
    pub distance: Option<f64>,
    pub block_type: Option<String>,
    pub rel_hit_pos: Option<SPCoordinate>,
    pub id: Option<String>,
    pub description_id: Option<String>,
    pub ship_id: Option<i64>,
    pub hit_pos_ship: Option<SPCoordinate>,
    pub error: Option<String>,
}
```

| Field | Type | Description |
|-------|------|-------------|
| is_block | `Option<bool>` | Whether a block was hit |
| is_entity | `Option<bool>` | Whether an entity was hit |
| abs_pos | `Option<SPCoordinate>` | Absolute world position of the hit |
| hit_pos | `Option<SPCoordinate>` | Hit position |
| distance | `Option<f64>` | Distance to the hit point |
| block_type | `Option<String>` | Block registry name (if block hit) |
| rel_hit_pos | `Option<SPCoordinate>` | Relative hit position within the block |
| id | `Option<String>` | Entity ID (if entity hit) |
| description_id | `Option<String>` | Entity description ID |
| ship_id | `Option<i64>` | VS ship ID (if ship hit) |
| hit_pos_ship | `Option<SPCoordinate>` | Hit position in ship coordinates |
| error | `Option<String>` | Error message (if raycast failed) |

### SPCoordinate

3D coordinate (defined in `some_peripherals::ballistic_accelerator`).

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SPCoordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```
