# GeoScanner

**Module:** `advanced_peripherals::geo_scanner`  
**Peripheral Type:** `advancedPeripherals:geo_scanner`

AdvancedPeripherals Geo Scanner peripheral. Scans surrounding blocks within a radius and analyzes ore distribution in the current chunk.

## Book-Read Methods

### `book_next_cost` / `read_last_cost` / `cost_imm`
Get the fuel cost for a scan at the given radius.
```rust
pub fn book_next_cost(&mut self, radius: f64)
pub fn read_last_cost(&self) -> Result<f64, PeripheralError>
pub fn cost_imm(&self, radius: f64) -> Result<f64, PeripheralError>
```
**Parameters:**
- `radius: f64` — Scan radius

**Returns:** `f64`

---

### `book_next_scan` / `read_last_scan`
Scan blocks within the given radius.
```rust
pub fn book_next_scan(&mut self, radius: f64)
pub fn read_last_scan(&self) -> Result<Vec<GeoBlockEntry>, PeripheralError>
```
**Parameters:**
- `radius: f64` — Scan radius

**Returns:** `Vec<GeoBlockEntry>`

---

### `book_next_chunk_analyze` / `read_last_chunk_analyze`
Analyze ore distribution in the current chunk.
```rust
pub fn book_next_chunk_analyze(&mut self)
pub fn read_last_chunk_analyze(&self) -> Result<Value, PeripheralError>
```
**Returns:** `Value` — Map of ore names to counts

## Immediate Methods

- `cost_imm(&self, radius: f64) -> Result<f64, PeripheralError>`

## Types

```rust
pub struct GeoBlockEntry {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub name: String,
    pub tags: Vec<String>,
}
```

## Usage Example

```rust
use rust_computers_api::advanced_peripherals::GeoScanner;
use rust_computers_api::peripheral::Peripheral;

let mut scanner = GeoScanner::wrap(addr);

// Check cost first
let cost = scanner.cost_imm(8.0);

// Scan
scanner.book_next_scan(8.0);
wait_for_next_tick().await;
let blocks = scanner.read_last_scan();

// Chunk analysis
scanner.book_next_chunk_analyze();
wait_for_next_tick().await;
let ores = scanner.read_last_chunk_analyze();
```
