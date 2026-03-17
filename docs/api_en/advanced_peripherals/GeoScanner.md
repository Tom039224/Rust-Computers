# GeoScanner

**Mod:** AdvancedPeripherals  
**Peripheral Type:** `advancedPeripherals:geo_scanner`  
**Source:** `GeoScannerPeripheral.java`

## Overview

The GeoScanner peripheral scans surrounding blocks within a specified radius and analyzes ore distribution in the current chunk. It provides detailed information about block types, their positions, and tags. This is useful for mining automation, resource location, and terrain analysis systems.

## Three-Function Pattern

The GeoScanner API uses the three-function pattern for all methods:

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


## Methods

### Scanning Operations

#### `scan(radius)` / `book_next_scan(radius)` / `read_last_scan()` / `async_scan(radius)`

Scan blocks within a specified radius around the scanner.

**Rust Signatures:**
```rust
pub fn book_next_scan(&mut self, radius: f64)
pub fn read_last_scan(&self) -> Result<Vec<GeoBlock>, PeripheralError>
pub async fn async_scan(&self, radius: f64) -> Result<Vec<GeoBlock>, PeripheralError>
```

**Parameters:**
- `radius: number` — Scan radius in blocks (max 64)

**Returns:** `table` — Array of block entries with position and type information

**Example:**
```rust
// Rust example to be added
```
---

#### `cost(radius)` / `book_next_cost(radius)` / `read_last_cost()` / `async_cost(radius)`

Get the fuel cost for a scan at the given radius.

**Rust Signatures:**
```rust
pub fn book_next_cost(&mut self, radius: f64)
pub fn read_last_cost(&self) -> Result<f64, PeripheralError>
pub async fn async_cost(&self, radius: f64) -> Result<f64, PeripheralError>
```

**Parameters:**
- `radius: number` — Scan radius in blocks

**Returns:** `number` — Fuel cost in FE (Forge Energy)

**Example:**
```rust
// Rust example to be added
```
---

#### `chunkAnalyze()` / `book_next_chunk_analyze()` / `read_last_chunk_analyze()` / `async_chunk_analyze()`

Analyze ore distribution in the current chunk.

**Rust Signatures:**
```rust
pub fn book_next_chunk_analyze(&mut self)
pub fn read_last_chunk_analyze(&self) -> Result<ChunkAnalysis, PeripheralError>
pub async fn async_chunk_analyze(&self) -> Result<ChunkAnalysis, PeripheralError>
```

**Returns:** `table` — Chunk analysis data with ore distribution

**Example:**
```rust
// Rust example to be added
```
---

## Events

The GeoScanner peripheral does not generate events.

---

## Usage Examples

### Example 1: Find Specific Ore

```rust
// Rust example to be added
```
### Example 2: Scan and Report Ores

```rust
// Rust example to be added
```
### Example 3: Chunk Analysis

```rust
// Rust example to be added
```
### Example 4: Efficient Scanning with Cost Check

```rust
// Rust example to be added
```
### Example 5: Mining Automation

```rust
// Rust example to be added
```
---

## Error Handling

All methods may throw errors in the following cases:

- **Radius too large**: Radius exceeds maximum (64 blocks)
- **Insufficient energy**: Not enough FE to perform scan
- **Peripheral disconnected**: The GeoScanner is no longer accessible

**Example Error Handling:**
```rust
// Rust example to be added
```
---

## Type Definitions

### GeoBlock
```rust
// Rust example to be added
```
### ChunkAnalysis
```rust
// Rust example to be added
```
---

## Notes

- Scan radius is limited to 64 blocks maximum
- Scanning consumes energy (FE) based on radius
- Larger scans are more expensive
- Results include all blocks, not just ores
- Chunk analysis provides statistical data
- The three-function pattern allows for efficient batch operations
- GeoScanner requires energy to function

---

## Related

- [BlockReader](./BlockReader.md) — For reading individual block information
- [PlayerDetector](./PlayerDetector.md) — For detecting players
- [AdvancedPeripherals Documentation](https://advancedperipherals.readthedocs.io/) — Official documentation

**Returns:** `number` — Fuel cost (in AE units or similar)

**Note:** The `cost_imm` method returns the result immediately without waiting for the next tick.

**Example:**
```rust
// Rust example to be added
```
---

### `scan(radius)` / `book_next_scan(radius)` / `read_last_scan()` / `async_scan(radius)`

Scan blocks within the given radius.

**Rust Signatures:**
```rust
pub fn book_next_scan(&mut self, radius: f64)
pub fn read_last_scan(&self) -> Result<Vec<GeoBlockEntry>, PeripheralError>
pub async fn async_scan(&self, radius: f64) -> Result<Vec<GeoBlockEntry>, PeripheralError>
```

**Parameters:**
- `radius: number` — Scan radius in blocks

**Returns:** `table` — Array of block entries

**Block Entry Structure:**
```rust
// Rust example to be added
```
**Example:**
```rust
// Rust example to be added
```
---

### `chunkAnalyze()` / `book_next_chunk_analyze()` / `read_last_chunk_analyze()` / `async_chunk_analyze()`

Analyze ore distribution in the current chunk.

**Rust Signatures:**
```rust
pub fn book_next_chunk_analyze(&mut self)
pub fn read_last_chunk_analyze(&self) -> Result<Value, PeripheralError>
pub async fn async_chunk_analyze(&self) -> Result<Value, PeripheralError>
```

**Returns:** `table` — Map of ore names to counts

**Return Structure:**
```rust
// Rust example to be added
```
**Example:**
```rust
// Rust example to be added
```
---

## Events

The GeoScanner peripheral does not generate events.

---

## Usage Examples

### Example 1: Find Specific Ore

```rust
// Rust example to be added
```
### Example 2: Scan and Filter by Tag

```rust
// Rust example to be added
```
### Example 3: Chunk Analysis Report

```rust
// Rust example to be added
```
### Example 4: Cost-Aware Scanning

```rust
// Rust example to be added
```
### Example 5: Mining Automation

```rust
// Rust example to be added
```
---

## Error Handling

All methods may throw errors in the following cases:

- **Insufficient fuel**: Not enough energy to perform the scan
- **Peripheral disconnected**: The GeoScanner is no longer accessible
- **Invalid radius**: Radius is negative or exceeds maximum

**Example Error Handling:**
```rust
// Rust example to be added
```
---

## Type Definitions

### GeoBlockEntry
```rust
// Rust example to be added
```
### ChunkAnalysisResult
```rust
// Rust example to be added
```
---

## Notes

- Coordinates returned by `scan()` are relative to the scanner's position
- The `cost_imm()` method is the only immediate method (returns without waiting)
- Chunk analysis scans the entire current chunk, not just the radius
- Block tags can be used to filter results (e.g., "minecraft:ores", "minecraft:mineable/pickaxe")
- The three-function pattern allows for efficient batch operations
- Larger scan radii consume more fuel/energy

---

## Related

- [BlockReader](./BlockReader.md) — For reading detailed block information
- [PlayerDetector](./PlayerDetector.md) — For detecting players
- [AdvancedPeripherals Documentation](https://advancedperipherals.readthedocs.io/) — Official documentation
