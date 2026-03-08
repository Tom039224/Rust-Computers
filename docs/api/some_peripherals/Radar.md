# Radar

**Module:** `some_peripherals`
**Peripheral Type:** `sp_radar` (the NAME constant)

Some-Peripherals Radar peripheral. Scans for entities, players, and Valkyrien Skies ships within a given radius.

## Methods

### Async Methods

These methods are async and internally await a response from the game server.

#### `scan_for_entities`

Scan for entities within a radius.

```rust
pub async fn scan_for_entities(
    &self,
    radius: f64,
) -> Result<Vec<SPEntityInfo>, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| radius | `f64` | The scan radius |

#### `scan_for_ships`

Scan for Valkyrien Skies ships within a radius.

```rust
pub async fn scan_for_ships(
    &self,
    radius: f64,
) -> Result<Vec<SPShipInfo>, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| radius | `f64` | The scan radius |

#### `scan_for_players`

Scan for players within a radius.

```rust
pub async fn scan_for_players(
    &self,
    radius: f64,
) -> Result<Vec<SPEntityInfo>, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| radius | `f64` | The scan radius |

#### `get_config_info`

Get radar configuration information.

```rust
pub async fn get_config_info(&self) -> Result<BTreeMap<String, String>, PeripheralError>
```

### Immediate Methods

These methods execute synchronously without needing async.

#### `get_config_info_imm`

Get radar configuration information immediately.

```rust
pub fn get_config_info_imm(&self) -> Result<BTreeMap<String, String>, PeripheralError>
```

## Example

```rust
// Scan for entities within 64 blocks
let entities = radar.scan_for_entities(64.0).await.unwrap();
for entity in &entities {
    // entity.name, entity.pos.x, entity.pos.y, entity.pos.z, entity.entity_type
}

// Scan for VS ships
let ships = radar.scan_for_ships(128.0).await.unwrap();
for ship in &ships {
    // ship.ship_id, ship.pos, ship.mass, ship.velocity
}
```

## Types

### SPPosition

3D position struct.

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SPPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```

| Field | Type | Description |
|-------|------|-------------|
| x | `f64` | X coordinate |
| y | `f64` | Y coordinate |
| z | `f64` | Z coordinate |

### SPEntityInfo

Entity information returned by scan methods.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPEntityInfo {
    pub pos: SPPosition,
    pub name: String,
    pub entity_type: String,
    pub is_entity: bool,
    pub is_player: bool,
    pub nickname: Option<String>,
    pub health: Option<f64>,
    pub max_health: Option<f64>,
    pub armor_value: Option<i32>,
    pub is_baby: Option<bool>,
    pub is_blocking: Option<bool>,
    pub is_sleeping: Option<bool>,
    pub is_fall_flying: Option<bool>,
    pub speed: Option<f64>,
}
```

| Field | Type | Description |
|-------|------|-------------|
| pos | `SPPosition` | Entity 3D position |
| name | `String` | Entity display name (nickname for players, entity_type for others) |
| entity_type | `String` | Entity type string (e.g. `"minecraft:skeleton"`) |
| is_entity | `bool` | Always `true` |
| is_player | `bool` | Whether this is a player |
| nickname | `Option<String>` | Player nickname (only for players) |
| health | `Option<f64>` | Current health (LivingEntity only) |
| max_health | `Option<f64>` | Maximum health (LivingEntity only) |
| armor_value | `Option<i32>` | Armor value (LivingEntity only) |
| is_baby | `Option<bool>` | Whether entity is baby-sized (LivingEntity only) |
| is_blocking | `Option<bool>` | Whether currently blocking (LivingEntity only) |
| is_sleeping | `Option<bool>` | Whether entity is sleeping (LivingEntity only) |
| is_fall_flying | `Option<bool>` | Whether entity is gliding/flying (LivingEntity only) |
| speed | `Option<f64>` | Entity movement speed (LivingEntity only) |

### SPShipInfo

Valkyrien Skies ship information returned by `scan_for_ships`.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPShipInfo {
    pub is_ship: bool,
    pub ship_id: i64,
    pub pos: SPCoordinate,
    pub mass: f64,
    pub rotation: VSQuaternion,
    pub velocity: SPCoordinate,
    pub size: SPCoordinate,
    pub scale: SPCoordinate,
    pub moment_of_inertia_tensor: [[f64; 3]; 3],
    pub center_of_mass_in_ship: SPCoordinate,
}
```

| Field | Type | Description |
|-------|------|-------------|
| is_ship | `bool` | Always `true` for ships |
| ship_id | `i64` | The VS ship ID (Lua key: `"id"`) |
| pos | `SPCoordinate` | World position |
| mass | `f64` | Ship mass |
| rotation | `VSQuaternion` | Ship rotation quaternion |
| velocity | `SPCoordinate` | Velocity vector |
| size | `SPCoordinate` | Ship bounding box size |
| scale | `SPCoordinate` | Ship scale |
| moment_of_inertia_tensor | `[[f64; 3]; 3]` | 3x3 inertia tensor matrix |
| center_of_mass_in_ship | `SPCoordinate` | Center of mass in ship coordinates (Lua key: `"center_of_mass_in_a_ship"`) |

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
