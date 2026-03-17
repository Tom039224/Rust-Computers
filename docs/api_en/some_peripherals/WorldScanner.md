# WorldScanner

**Module:** `some_peripherals`
**Peripheral Type:** `sp:world_scanner` (the NAME constant)

Some-Peripherals WorldScanner peripheral. Scans blocks at specific world coordinates, with support for Valkyrien Skies shipyard coordinates.

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Methods

### Async Methods

These methods are async and internally await a response from the game server.

#### `get_block_at`

Get block information at the specified world coordinates.

```rust
pub async fn get_block_at(
    &self,
    x: i32,
    y: i32,
    z: i32,
    is_shipyard: bool,
) -> Result<SPBlockInfo, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| x | `i32` | X coordinate |
| y | `i32` | Y coordinate |
| z | `i32` | Z coordinate |
| is_shipyard | `bool` | Whether the coordinates are in Valkyrien Skies shipyard space |

## Example

```rust
// Get block at world coordinates
let block = world_scanner.get_block_at(100, 64, 200, false).await.unwrap();

// Get block in VS shipyard coordinates
let ship_block = world_scanner.get_block_at(50, 70, 50, true).await.unwrap();
if let Some(ship_id) = ship_block.ship_id {
    // Block belongs to a VS ship
}
```

## Types

### SPBlockInfo

Block information returned by the WorldScanner.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPBlockInfo {
    pub block_type: String,
    pub ship_id: Option<i64>,
}
```

| Field | Type | Description |
|-------|------|-------------|
| block_type | `String` | Block registry name (e.g. `"minecraft:stone"`) |
| ship_id | `Option<i64>` | VS ship ID if the block belongs to a ship |
