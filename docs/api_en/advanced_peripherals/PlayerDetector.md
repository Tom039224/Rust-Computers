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

```lua
-- Method 1: book_next / read_last pattern
detector.book_next_get_players_in_range(50)
wait_for_next_tick()
local players = detector.read_last_get_players_in_range()

-- Method 2: async pattern (recommended)
local players = detector.async_get_players_in_range(50)
```

## Methods

### Player Listing

#### `getOnlinePlayers()` / `book_next_get_online_players()` / `read_last_get_online_players()` / `async_get_online_players()`

Get a list of all online player names on the server.

**Lua Signature:**
```lua
function getOnlinePlayers() -> table
```

**Rust Signatures:**
```rust
pub fn book_next_get_online_players(&mut self)
pub fn read_last_get_online_players(&self) -> Result<Vec<String>, PeripheralError>
pub async fn async_get_online_players(&self) -> Result<Vec<String>, PeripheralError>
```

**Returns:** `table` — Array of player names

**Example:**
```lua
local detector = peripheral.find("advancedPeripherals:player_detector")
local players = detector.async_get_online_players()

for _, name in ipairs(players) do
  print("Online: " .. name)
end
```

---

### Range-Based Detection

#### `getPlayersInRange(radius)` / `book_next_get_players_in_range(radius)` / `read_last_get_players_in_range()` / `async_get_players_in_range(radius)`

Get players within a spherical radius of the detector.

**Lua Signature:**
```lua
function getPlayersInRange(radius: number) -> table
```

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
```lua
local detector = peripheral.find("advancedPeripherals:player_detector")
local nearby = detector.async_get_players_in_range(50)

print("Players within 50 blocks:")
for _, name in ipairs(nearby) do
  print("  " .. name)
end
```

---

#### `getPlayersInCoords(x1, y1, z1, x2, y2, z2)` / `book_next_get_players_in_coords(...)` / `read_last_get_players_in_coords()` / `async_get_players_in_coords(...)`

Get players within a coordinate bounding box.

**Lua Signature:**
```lua
function getPlayersInCoords(x1: number, y1: number, z1: number, x2: number, y2: number, z2: number) -> table
```

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
```lua
local detector = peripheral.find("advancedPeripherals:player_detector")
local players = detector.async_get_players_in_coords(0, 60, 0, 100, 100, 100)

print("Players in region:")
for _, name in ipairs(players) do
  print("  " .. name)
end
```

---

#### `getPlayersInCubic(dx, dy, dz)` / `book_next_get_players_in_cubic(dx, dy, dz)` / `read_last_get_players_in_cubic()` / `async_get_players_in_cubic(dx, dy, dz)`

Get players within a cubic area centered on the detector.

**Lua Signature:**
```lua
function getPlayersInCubic(dx: number, dy: number, dz: number) -> table
```

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
```lua
local detector = peripheral.find("advancedPeripherals:player_detector")
-- Get players in a 100x100x100 cube centered on detector
local players = detector.async_get_players_in_cubic(50, 50, 50)
```

---

### Presence Checking

#### `isPlayersInRange(radius)` / `book_next_is_players_in_range(radius)` / `read_last_is_players_in_range()` / `async_is_players_in_range(radius)`

Check if any players are within a radius.

**Lua Signature:**
```lua
function isPlayersInRange(radius: number) -> boolean
```

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

**Lua Signature:**
```lua
function isPlayersInCoords(x1: number, y1: number, z1: number, x2: number, y2: number, z2: number) -> boolean
```

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

**Lua Signature:**
```lua
function isPlayersInCubic(dx: number, dy: number, dz: number) -> boolean
```

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

**Lua Signature:**
```lua
function isPlayerInRange(player: string, radius: number) -> boolean
```

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
```lua
local detector = peripheral.find("advancedPeripherals:player_detector")
if detector.async_is_player_in_range("Steve", 50) then
  print("Steve is nearby!")
end
```

---

#### `isPlayerInCoords(player, x1, y1, z1, x2, y2, z2)` / `book_next_is_player_in_coords(...)` / `read_last_is_player_in_coords()` / `async_is_player_in_coords(...)`

Check if a specific player is within a coordinate bounding box.

**Lua Signature:**
```lua
function isPlayerInCoords(player: string, x1: number, y1: number, z1: number, x2: number, y2: number, z2: number) -> boolean
```

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

**Lua Signature:**
```lua
function isPlayerInCubic(player: string, dx: number, dy: number, dz: number) -> boolean
```

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

**Lua Signature:**
```lua
function getPlayerPos(player: string, decimals?: number) -> table
```

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
```lua
local detector = peripheral.find("advancedPeripherals:player_detector")
local info = detector.async_get_player_pos("Steve", 2)

print(("Steve is at: %.2f, %.2f, %.2f"):format(info.x, info.y, info.z))
```

---

#### `getPlayer(player, decimals?)` / `book_next_get_player(player, decimals?)` / `read_last_get_player()` / `async_get_player(player, decimals?)`

Get detailed player information.

**Lua Signature:**
```lua
function getPlayer(player: string, decimals?: number) -> table
```

**Rust Signatures:**
```rust
pub fn book_next_get_player(&mut self, player: &str, decimals: Option<u32>)
pub fn read_last_get_player(&self) -> Result<ADPlayerInfo, PeripheralError>
pub async fn async_get_player(&self, player: &str, decimals: Option<u32>) -> Result<ADPlayerInfo, PeripheralError>
```

**Parameters / Returns:** Same as `getPlayerPos`

**Example:**
```lua
local detector = peripheral.find("advancedPeripherals:player_detector")
local info = detector.async_get_player("Steve")

print("Player: " .. info.name)
print("Position: " .. info.x .. ", " .. info.y .. ", " .. info.z)
print("Health: " .. info.health .. "/" .. info.max_health)
print("Game Mode: " .. info.game_mode)
print("Sneaking: " .. tostring(info.is_sneaking))
```

---

## Events

The PlayerDetector peripheral does not generate events. However, you can poll for player presence changes using the detection methods.

---

## Usage Examples

### Example 1: Detect Player Presence

```lua
local detector = peripheral.find("advancedPeripherals:player_detector")

while true do
  if detector.async_is_players_in_range(50) then
    print("Players detected nearby!")
  else
    print("No players nearby")
  end
  
  sleep(1)
end
```

### Example 2: Track Specific Player

```lua
local detector = peripheral.find("advancedPeripherals:player_detector")

local function track_player(name)
  local info = detector.async_get_player(name)
  
  if info then
    print(("Tracking %s at (%.1f, %.1f, %.1f)"):format(
      info.name, info.x, info.y, info.z
    ))
    print("Health: " .. info.health .. "/" .. info.max_health)
  else
    print("Player not found")
  end
end

track_player("Steve")
```

### Example 3: Security System

```lua
local detector = peripheral.find("advancedPeripherals:player_detector")

local protected_zone = {x1 = 0, y1 = 60, z1 = 0, x2 = 100, y2 = 100, z2 = 100}

while true do
  if detector.async_is_players_in_coords(
    protected_zone.x1, protected_zone.y1, protected_zone.z1,
    protected_zone.x2, protected_zone.y2, protected_zone.z2
  ) then
    print("ALERT: Unauthorized player in protected zone!")
  end
  
  sleep(1)
end
```

### Example 4: List All Online Players

```lua
local detector = peripheral.find("advancedPeripherals:player_detector")

local players = detector.async_get_online_players()
print("Online players: " .. #players)

for _, name in ipairs(players) do
  local info = detector.async_get_player(name)
  if info then
    print(("  %s at (%.1f, %.1f, %.1f) - Health: %.1f"):format(
      name, info.x, info.y, info.z, info.health
    ))
  end
end
```

### Example 5: Proximity-Based Greeting

```lua
local detector = peripheral.find("advancedPeripherals:player_detector")
local chat = peripheral.find("advancedPeripherals:chat_box")

local greeted = {}

while true do
  local nearby = detector.async_get_players_in_range(20)
  
  for _, name in ipairs(nearby) do
    if not greeted[name] then
      chat.async_send_message_to_player("Welcome, " .. name .. "!", name)
      greeted[name] = true
    end
  end
  
  sleep(1)
end
```

---

## Error Handling

All methods may throw errors in the following cases:

- **Player not found**: Player is not online or not in range
- **Peripheral disconnected**: The PlayerDetector is no longer accessible
- **Invalid coordinates**: Coordinate values are invalid

**Example Error Handling:**
```lua
local detector = peripheral.find("advancedPeripherals:player_detector")
if not detector then
  error("PlayerDetector not found")
end

local success, result = pcall(function()
  return detector.async_get_player("Steve")
end)

if not success then
  print("Error: " .. result)
else
  if result then
    print("Found player at: " .. result.x .. ", " .. result.y .. ", " .. result.z)
  else
    print("Player not found")
  end
end
```

---

## Type Definitions

### ADPlayerInfo
```lua
{
  x: number,                 -- X coordinate
  y: number,                 -- Y coordinate
  z: number,                 -- Z coordinate
  name?: string,             -- Player name (optional)
  uuid?: string,             -- Player UUID (optional)
  health?: number,           -- Current health (optional)
  max_health?: number,       -- Maximum health (optional)
  is_flying?: boolean,       -- Is flying (optional)
  is_sprinting?: boolean,    -- Is sprinting (optional)
  is_sneaking?: boolean,     -- Is sneaking (optional)
  game_mode?: string,        -- Game mode: "survival", "creative", "adventure", "spectator" (optional)
  experience?: number,       -- Experience points (optional)
  level?: number,            -- Experience level (optional)
  pitch?: number,            -- Look pitch angle (optional)
}
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
