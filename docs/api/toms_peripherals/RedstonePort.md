# RedstonePort

**Module:** `toms_peripherals`  
**Peripheral Type:** `tm:redstone_port`

Toms-Peripherals Redstone Port peripheral providing per-side redstone I/O including analog and bundled signals.

## Methods

### Immediate Methods

#### `get_sides_imm`

Get the list of available sides.

```rust
pub fn get_sides_imm(&self) -> Result<Vec<String>, PeripheralError>
```

### Async Read Methods

#### `get_input`

Get the redstone input state of a side.

```rust
pub async fn get_input(&self, side: &str) -> Result<bool, PeripheralError>
```

#### `get_analog_input`

Get the analog input level of a side (0–15).

```rust
pub async fn get_analog_input(&self, side: &str) -> Result<u8, PeripheralError>
```

#### `get_bundled_input`

Get the bundled input bitmask of a side.

```rust
pub async fn get_bundled_input(&self, side: &str) -> Result<u16, PeripheralError>
```

#### `test_bundled_input`

Test if a bundled input bitmask matches.

```rust
pub async fn test_bundled_input(&self, side: &str, mask: u16) -> Result<bool, PeripheralError>
```

### Async Methods (with imm variants)

#### `get_output` / `get_output_imm`

Get the redstone output state of a side.

```rust
pub async fn get_output(&self, side: &str) -> Result<bool, PeripheralError>
pub fn get_output_imm(&self, side: &str) -> Result<bool, PeripheralError>
```

#### `get_analog_output` / `get_analog_output_imm`

Get the analog output level of a side.

```rust
pub async fn get_analog_output(&self, side: &str) -> Result<u8, PeripheralError>
pub fn get_analog_output_imm(&self, side: &str) -> Result<u8, PeripheralError>
```

#### `get_bundled_output` / `get_bundled_output_imm`

Get the bundled output bitmask of a side.

```rust
pub async fn get_bundled_output(&self, side: &str) -> Result<u16, PeripheralError>
pub fn get_bundled_output_imm(&self, side: &str) -> Result<u16, PeripheralError>
```

### Async Action Methods

#### `set_output`

Set the redstone output of a side.

```rust
pub async fn set_output(&self, side: &str, value: bool) -> Result<(), PeripheralError>
```

#### `set_analog_output`

Set the analog output level of a side.

```rust
pub async fn set_analog_output(&self, side: &str, value: u8) -> Result<(), PeripheralError>
```

#### `set_bundled_output`

Set the bundled output bitmask of a side.

```rust
pub async fn set_bundled_output(&self, side: &str, mask: u16) -> Result<(), PeripheralError>
```

## Example

```rust
use rust_computers_api::toms_peripherals::redstone_port::RedstonePort;
use rust_computers_api::peripheral::Peripheral;

let port = RedstonePort::wrap(addr);

// List available sides
let sides = port.get_sides_imm()?;

// Read input
let powered = port.get_input("front").await?;
let level = port.get_analog_input("front").await?;

// Set output
port.set_output("back", true).await?;
port.set_analog_output("back", 15).await?;
```
