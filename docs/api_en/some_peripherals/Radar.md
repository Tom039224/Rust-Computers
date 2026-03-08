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
    // entity.name, entity.x, entity.y, entity.z, entity.id
}

// Scan for VS ships
let ships = radar.scan_for_ships(128.0).await.unwrap();
for ship in &ships {
    // ship.ship_id, ship.pos, ship.mass, ship.velocity
}
```

## Types

### SPEntityInfo

Entity information returned by scan methods.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPEntityInfo {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub id: String,
    pub entity_type: String,
    pub name: String,
}
```

| Field | Type | Description |
|-------|------|-------------|
| x | `f64` | Entity X coordinate |
| y | `f64` | Entity Y coordinate |
| z | `f64` | Entity Z coordinate |
| id | `String` | Entity registry ID (e.g. `"minecraft:skeleton"`) |
| entity_type | `String` | Entity type string (Lua key: `"type"`) |
| name | `String` | Entity display name |

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
