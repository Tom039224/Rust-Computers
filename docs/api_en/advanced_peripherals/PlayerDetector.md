# PlayerDetector

**Module:** `advanced_peripherals::player_detector`  
**Peripheral Type:** `advancedPeripherals:player_detector`

AdvancedPeripherals PlayerDetector peripheral for detecting and querying players in various range configurations (radius, coordinate box, cubic area).

## Book-Read Methods

### `book_next_get_online_players` / `read_last_get_online_players`
Get a list of all online player names.
```rust
pub fn book_next_get_online_players(&mut self) { ... }
pub fn read_last_get_online_players(&self) -> Result<Vec<String>, PeripheralError> { ... }
```
**Returns:** `Vec<String>`

---

### `book_next_get_players_in_range` / `read_last_get_players_in_range`
Get players within a spherical radius.
```rust
pub fn book_next_get_players_in_range(&mut self, radius: f64) { ... }
pub fn read_last_get_players_in_range(&self) -> Result<Vec<String>, PeripheralError> { ... }
```
**Parameters:**
- `radius: f64` — Search radius

**Returns:** `Vec<String>`

---

### `book_next_get_players_in_coords` / `read_last_get_players_in_coords`
Get players within a coordinate bounding box.
```rust
pub fn book_next_get_players_in_coords(
    &mut self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64,
) { ... }
pub fn read_last_get_players_in_coords(&self) -> Result<Vec<String>, PeripheralError> { ... }
```
**Parameters:**
- `x1, y1, z1` — First corner coordinates
- `x2, y2, z2` — Second corner coordinates

**Returns:** `Vec<String>`

---

### `book_next_get_players_in_cubic` / `read_last_get_players_in_cubic`
Get players within a cubic area centered on the detector.
```rust
pub fn book_next_get_players_in_cubic(&mut self, dx: f64, dy: f64, dz: f64) { ... }
pub fn read_last_get_players_in_cubic(&self) -> Result<Vec<String>, PeripheralError> { ... }
```
**Parameters:**
- `dx, dy, dz: f64` — Half-extents of the cubic area

**Returns:** `Vec<String>`

---

### `book_next_is_players_in_range` / `read_last_is_players_in_range`
Check if any players are within a radius.
```rust
pub fn book_next_is_players_in_range(&mut self, radius: f64) { ... }
pub fn read_last_is_players_in_range(&self) -> Result<bool, PeripheralError> { ... }
```
**Returns:** `bool`

---

### `book_next_is_players_in_coords` / `read_last_is_players_in_coords`
Check if any players are within a coordinate bounding box.
```rust
pub fn book_next_is_players_in_coords(
    &mut self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64,
) { ... }
pub fn read_last_is_players_in_coords(&self) -> Result<bool, PeripheralError> { ... }
```
**Returns:** `bool`

---

### `book_next_is_players_in_cubic` / `read_last_is_players_in_cubic`
Check if any players are within a cubic area.
```rust
pub fn book_next_is_players_in_cubic(&mut self, dx: f64, dy: f64, dz: f64) { ... }
pub fn read_last_is_players_in_cubic(&self) -> Result<bool, PeripheralError> { ... }
```
**Returns:** `bool`

---

### `book_next_is_player_in_range` / `read_last_is_player_in_range`
Check if a specific player is within a radius.
```rust
pub fn book_next_is_player_in_range(&mut self, player: &str, radius: f64) { ... }
pub fn read_last_is_player_in_range(&self) -> Result<bool, PeripheralError> { ... }
```
**Parameters:**
- `player: &str` — Player name
- `radius: f64` — Search radius

**Returns:** `bool`

---

### `book_next_is_player_in_coords` / `read_last_is_player_in_coords`
Check if a specific player is within a coordinate bounding box.
```rust
pub fn book_next_is_player_in_coords(
    &mut self, player: &str, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64,
) { ... }
pub fn read_last_is_player_in_coords(&self) -> Result<bool, PeripheralError> { ... }
```
**Returns:** `bool`

---

### `book_next_is_player_in_cubic` / `read_last_is_player_in_cubic`
Check if a specific player is within a cubic area.
```rust
pub fn book_next_is_player_in_cubic(&mut self, player: &str, dx: f64, dy: f64, dz: f64) { ... }
pub fn read_last_is_player_in_cubic(&self) -> Result<bool, PeripheralError> { ... }
```
**Returns:** `bool`

---

### `book_next_get_player_pos` / `read_last_get_player_pos`
Get a player's position.
```rust
pub fn book_next_get_player_pos(&mut self, player: &str, decimals: Option<u32>) { ... }
pub fn read_last_get_player_pos(&self) -> Result<ADPlayerInfo, PeripheralError> { ... }
```
**Parameters:**
- `player: &str` — Player name
- `decimals: Option<u32>` — Decimal precision (optional)

**Returns:** `ADPlayerInfo`

---

### `book_next_get_player` / `read_last_get_player`
Get detailed player information.
```rust
pub fn book_next_get_player(&mut self, player: &str, decimals: Option<u32>) { ... }
pub fn read_last_get_player(&self) -> Result<ADPlayerInfo, PeripheralError> { ... }
```
**Parameters:**
- `player: &str` — Player name
- `decimals: Option<u32>` — Decimal precision (optional)

**Returns:** `ADPlayerInfo`

## Types

### `ADPlayerInfo`
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

## Usage Example

```rust
use rust_computers_api::advanced_peripherals::player_detector::*;
use rust_computers_api::peripheral::Peripheral;

let mut detector = PlayerDetector::find().unwrap();

// Get online players
detector.book_next_get_online_players();
wait_for_next_tick().await;
let players = detector.read_last_get_online_players().unwrap();

// Check if anyone is nearby
detector.book_next_is_players_in_range(50.0);
wait_for_next_tick().await;
let nearby = detector.read_last_is_players_in_range().unwrap();

// Get detailed info for a specific player
if let Some(name) = players.first() {
    detector.book_next_get_player(name, Some(2));
    wait_for_next_tick().await;
    let info = detector.read_last_get_player().unwrap();
}
```
