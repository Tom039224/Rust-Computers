# PlayerDetector

**Mod:** AdvancedPeripherals  
**Peripheral Type:** `advancedPeripherals:player_detector`  
**Source:** `PlayerDetectorPeripheral.java`

## Overview

The PlayerDetector peripheral detects and queries players in various range configurations. It can find players within a spherical radius, coordinate bounding boxes, or cubic areas centered on the detector. It also provides detailed player information including position, health, game mode, and more. This is useful for creating player-aware automation systems, security systems, and interactive installations.

## Three-Function Pattern

The PlayerDetector API uses the three-function pattern for all methods:

1. **`book_next_*`** - Schedule a request for the next tick
2. **`read_last_*`** - Read the result from the previous tick
3. **`async_*`** - Convenience method that books, waits, and reads in one call

### Pattern Explanation

```rust
// Rust example to be added
```
## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods
- playerJoin / playerLeave events


## Methods

### Player Listing

#### `getOnlinePlayers()` / `book_next_get_online_players()` / `read_last_get_online_players()` / `async_get_online_players()`

Get a list of all online player names on the server.

**Rust Signatures:**
```rust
pub fn book_next_get_online_players(&mut self)
pub fn read_last_get_online_players(&self) -> Result<Vec<String>, PeripheralError>
pub async fn async_get_online_players(&self) -> Result<Vec<String>, PeripheralError>
```

**Returns:** `table` — Array of player names

**Example:**
```rust
// Rust example to be added
```
---

### Range-Based Detection

#### `getPlayersInRange(radius)` / `book_next_get_players_in_range(radius)` / `read_last_get_players_in_range()` / `async_get_players_in_range(radius)`

Get players within a spherical radius of the detector.

**Rust Signatures:**
```rust
pub fn book_next_get_players_in_range(&mut self, radius: f64)
pub fn read_last_get_players_in_range(&self) -> Result<Vec<String>, PeripheralError>
pub async fn async_get_players_in_range(&self, radius: f64) -> Result<Vec<String>, PeripheralError>
```

**Parameters:**
- `radius: number` — Search radius in blocks

**Returns:** `table` — Array of player names within range

**Example:**
```rust
// Rust example to be added
```
---

#### `getPlayersInCoords(x1, y1, z1, x2, y2, z2)` / `book_next_get_players_in_coords(...)` / `read_last_get_players_in_coords()` / `async_get_players_in_coords(...)`

Get players within a coordinate bounding box.

**Rust Signatures:**
```rust
pub fn book_next_get_players_in_coords(&mut self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64)
pub fn read_last_get_players_in_coords(&self) -> Result<Vec<String>, PeripheralError>
pub async fn async_get_players_in_coords(&self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Result<Vec<String>, PeripheralError>
```

**Parameters:**
- `x1, y1, z1` — First corner coordinates
- `x2, y2, z2` — Second corner coordinates

**Returns:** `table` — Array of player names in the box

**Example:**
```rust
// Rust example to be added
```
---

#### `getPlayersInCubic(dx, dy, dz)` / `book_next_get_players_in_cubic(dx, dy, dz)` / `read_last_get_players_in_cubic()` / `async_get_players_in_cubic(dx, dy, dz)`

Get players within a cubic area centered on the detector.

**Rust Signatures:**
```rust
pub fn book_next_get_players_in_cubic(&mut self, dx: f64, dy: f64, dz: f64)
pub fn read_last_get_players_in_cubic(&self) -> Result<Vec<String>, PeripheralError>
pub async fn async_get_players_in_cubic(&self, dx: f64, dy: f64, dz: f64) -> Result<Vec<String>, PeripheralError>
```

**Parameters:**
- `dx, dy, dz: number` — Half-extents of the cubic area

**Returns:** `table` — Array of player names in the cube

**Example:**
```rust
// Rust example to be added
```
---

### Presence Checking

#### `isPlayersInRange(radius)` / `book_next_is_players_in_range(radius)` / `read_last_is_players_in_range()` / `async_is_players_in_range(radius)`

Check if any players are within a radius.

**Rust Signatures:**
```rust
pub fn book_next_is_players_in_range(&mut self, radius: f64)
pub fn read_last_is_players_in_range(&self) -> Result<bool, PeripheralError>
pub async fn async_is_players_in_range(&self, radius: f64) -> Result<bool, PeripheralError>
```

**Parameters / Returns:** Same as `getPlayersInRange` but returns boolean

---

#### `isPlayersInCoords(x1, y1, z1, x2, y2, z2)` / `book_next_is_players_in_coords(...)` / `read_last_is_players_in_coords()` / `async_is_players_in_coords(...)`

Check if any players are within a coordinate bounding box.

**Rust Signatures:**
```rust
pub fn book_next_is_players_in_coords(&mut self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64)
pub fn read_last_is_players_in_coords(&self) -> Result<bool, PeripheralError>
pub async fn async_is_players_in_coords(&self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Result<bool, PeripheralError>
```

**Parameters / Returns:** Same as `getPlayersInCoords` but returns boolean

---

#### `isPlayersInCubic(dx, dy, dz)` / `book_next_is_players_in_cubic(dx, dy, dz)` / `read_last_is_players_in_cubic()` / `async_is_players_in_cubic(dx, dy, dz)`

Check if any players are within a cubic area.

**Rust Signatures:**
```rust
pub fn book_next_is_players_in_cubic(&mut self, dx: f64, dy: f64, dz: f64)
pub fn read_last_is_players_in_cubic(&self) -> Result<bool, PeripheralError>
pub async fn async_is_players_in_cubic(&self, dx: f64, dy: f64, dz: f64) -> Result<bool, PeripheralError>
```

**Parameters / Returns:** Same as `getPlayersInCubic` but returns boolean

---

### Specific Player Detection

#### `isPlayerInRange(player, radius)` / `book_next_is_player_in_range(player, radius)` / `read_last_is_player_in_range()` / `async_is_player_in_range(player, radius)`

Check if a specific player is within a radius.

**Rust Signatures:**
```rust
pub fn book_next_is_player_in_range(&mut self, player: &str, radius: f64)
pub fn read_last_is_player_in_range(&self) -> Result<bool, PeripheralError>
pub async fn async_is_player_in_range(&self, player: &str, radius: f64) -> Result<bool, PeripheralError>
```

**Parameters:**
- `player: string` — Player name
- `radius: number` — Search radius

**Returns:** `boolean` — `true` if player is in range

**Example:**
```rust
// Rust example to be added
```
---

#### `isPlayerInCoords(player, x1, y1, z1, x2, y2, z2)` / `book_next_is_player_in_coords(...)` / `read_last_is_player_in_coords()` / `async_is_player_in_coords(...)`

Check if a specific player is within a coordinate bounding box.

**Rust Signatures:**
```rust
pub fn book_next_is_player_in_coords(&mut self, player: &str, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64)
pub fn read_last_is_player_in_coords(&self) -> Result<bool, PeripheralError>
pub async fn async_is_player_in_coords(&self, player: &str, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Result<bool, PeripheralError>
```

**Parameters / Returns:** Same as `isPlayerInRange`

---

#### `isPlayerInCubic(player, dx, dy, dz)` / `book_next_is_player_in_cubic(player, dx, dy, dz)` / `read_last_is_player_in_cubic()` / `async_is_player_in_cubic(player, dx, dy, dz)`

Check if a specific player is within a cubic area.

**Rust Signatures:**
```rust
pub fn book_next_is_player_in_cubic(&mut self, player: &str, dx: f64, dy: f64, dz: f64)
pub fn read_last_is_player_in_cubic(&self) -> Result<bool, PeripheralError>
pub async fn async_is_player_in_cubic(&self, player: &str, dx: f64, dy: f64, dz: f64) -> Result<bool, PeripheralError>
```

**Parameters / Returns:** Same as `isPlayerInRange`

---

### Player Information

#### `getPlayerPos(player, decimals?)` / `book_next_get_player_pos(player, decimals?)` / `read_last_get_player_pos()` / `async_get_player_pos(player, decimals?)`

Get a player's position.

**Rust Signatures:**
```rust
pub fn book_next_get_player_pos(&mut self, player: &str, decimals: Option<u32>)
pub fn read_last_get_player_pos(&self) -> Result<ADPlayerInfo, PeripheralError>
pub async fn async_get_player_pos(&self, player: &str, decimals: Option<u32>) -> Result<ADPlayerInfo, PeripheralError>
```

**Parameters:**
- `player: string` — Player name
- `decimals?: number` — Decimal precision (optional, default 2)

**Returns:** `table` — Player information

**Example:**
```rust
// Rust example to be added
```
---

#### `getPlayer(player, decimals?)` / `book_next_get_player(player, decimals?)` / `read_last_get_player()` / `async_get_player(player, decimals?)`

Get detailed player information.

**Rust Signatures:**
```rust
pub fn book_next_get_player(&mut self, player: &str, decimals: Option<u32>)
pub fn read_last_get_player(&self) -> Result<ADPlayerInfo, PeripheralError>
pub async fn async_get_player(&self, player: &str, decimals: Option<u32>) -> Result<ADPlayerInfo, PeripheralError>
```

**Parameters / Returns:** Same as `getPlayerPos`

**Example:**
```rust
// Rust example to be added
```
---

## Events

The PlayerDetector peripheral does not generate events. However, you can poll for player presence changes using the detection methods.

---

## Usage Examples

### Example 1: Detect Player Presence

```rust
// Rust example to be added
```
### Example 2: Track Specific Player

```rust
// Rust example to be added
```
### Example 3: Security System

```rust
// Rust example to be added
```
### Example 4: List All Online Players

```rust
// Rust example to be added
```
### Example 5: Proximity-Based Greeting

```rust
// Rust example to be added
```
---

## Error Handling

All methods may throw errors in the following cases:

- **Player not found**: Player is not online or not in range
- **Peripheral disconnected**: The PlayerDetector is no longer accessible
- **Invalid coordinates**: Coordinate values are invalid

**Example Error Handling:**
```rust
// Rust example to be added
```
---

## Type Definitions

### ADPlayerInfo
```rust
// Rust example to be added
```
---

## Notes

- Player names are case-sensitive
- Coordinates are in block units
- Decimal precision affects the accuracy of position reporting
- The detector can track players across dimensions if configured
- The three-function pattern allows for efficient batch operations
- Detection range is limited by the detector's configuration

---

## Related

- [ChatBox](./ChatBox.md) — For sending messages to players
- [BlockReader](./BlockReader.md) — For reading block information
- [AdvancedPeripherals Documentation](https://advancedperipherals.readthedocs.io/) — Official documentation
