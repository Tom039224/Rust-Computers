# PlayerDetector

**Module:** `advanced_peripherals`  
**Peripheral Type:** `advancedPeripherals:player_detector`

Advanced Peripherals Player Detector. Detects and queries player locations, presence, and detailed information.

## Types

```rust
pub struct ADPlayerInfo {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub name: Option<String>,
    pub uuid: Option<String>,
    pub health: Option<f64>,
    pub max_health: Option<f64>,
    pub is_flying: Option<bool>,
    pub is_sprinting: Option<bool>,
    pub is_sneaking: Option<bool>,
    pub game_mode: Option<String>,
    pub experience: Option<u32>,
    pub level: Option<u32>,
    pub pitch: Option<f64>,
}
```

## Methods

All methods are async.

### Player Listing

#### `get_online_players`

Get all online player names.

```rust
pub async fn get_online_players(&self) -> Result<Vec<String>, PeripheralError>
```

#### `get_players_in_range`

Get player names within a radius.

```rust
pub async fn get_players_in_range(&self, radius: f64) -> Result<Vec<String>, PeripheralError>
```

#### `get_players_in_coords`

Get player names within a coordinate bounding box.

```rust
pub async fn get_players_in_coords(&self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Result<Vec<String>, PeripheralError>
```

#### `get_players_in_cubic`

Get player names within a cubic range (relative to detector).

```rust
pub async fn get_players_in_cubic(&self, dx: f64, dy: f64, dz: f64) -> Result<Vec<String>, PeripheralError>
```

### Presence Checks (multiple players)

#### `is_players_in_range`

Check if any players are within a radius.

```rust
pub async fn is_players_in_range(&self, radius: f64) -> Result<bool, PeripheralError>
```

#### `is_players_in_coords`

Check if any players are within a coordinate bounding box.

```rust
pub async fn is_players_in_coords(&self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Result<bool, PeripheralError>
```

#### `is_players_in_cubic`

Check if any players are within a cubic range.

```rust
pub async fn is_players_in_cubic(&self, dx: f64, dy: f64, dz: f64) -> Result<bool, PeripheralError>
```

### Presence Checks (specific player)

#### `is_player_in_range`

Check if a specific player is within a radius.

```rust
pub async fn is_player_in_range(&self, player: &str, radius: f64) -> Result<bool, PeripheralError>
```

#### `is_player_in_coords`

Check if a specific player is within a coordinate bounding box.

```rust
pub async fn is_player_in_coords(&self, player: &str, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Result<bool, PeripheralError>
```

#### `is_player_in_cubic`

Check if a specific player is within a cubic range.

```rust
pub async fn is_player_in_cubic(&self, player: &str, dx: f64, dy: f64, dz: f64) -> Result<bool, PeripheralError>
```

### Player Info

#### `get_player_pos`

Get a player's position. Optionally specify decimal precision.

```rust
pub async fn get_player_pos(&self, player: &str, decimals: Option<u32>) -> Result<ADPlayerInfo, PeripheralError>
```

#### `get_player`

Get detailed player information. Optionally specify decimal precision.

```rust
pub async fn get_player(&self, player: &str, decimals: Option<u32>) -> Result<ADPlayerInfo, PeripheralError>
```

## Example

```rust
use rust_computers_api::advanced_peripherals::player_detector::PlayerDetector;
use rust_computers_api::peripheral::Peripheral;

let detector = PlayerDetector::wrap(addr);

// Get nearby players
let players = detector.get_players_in_range(50.0).await?;

// Check specific player
let nearby = detector.is_player_in_range("Steve", 100.0).await?;

// Get player details
let info = detector.get_player("Steve", Some(2)).await?;
```
