# ElectricMotor

**Module:** `createaddition`  
**Peripheral Type:** `createaddition:electric_motor`

Create Additions Electric Motor peripheral. Controls rotation speed and provides energy/stress information.

## Methods

### Async Methods (with imm variants)

#### `get_type` / `get_type_imm`

Get the peripheral type string.

```rust
pub async fn get_type(&self) -> Result<String, PeripheralError>
pub fn get_type_imm(&self) -> Result<String, PeripheralError>
```

### Async Action Methods

#### `set_speed`

Set the RPM (sign controls direction).

```rust
pub async fn set_speed(&self, speed: f64) -> Result<(), PeripheralError>
```

#### `stop`

Stop the motor.

```rust
pub async fn stop(&self) -> Result<(), PeripheralError>
```

#### `rotate`

Rotate by a specified number of degrees. Optionally specify RPM. Returns the time in seconds.

```rust
pub async fn rotate(&self, degrees: f64, rpm: Option<f64>) -> Result<f64, PeripheralError>
```

#### `translate`

Move a specified distance. Optionally specify RPM. Returns the time in seconds.

```rust
pub async fn translate(&self, distance: f64, rpm: Option<f64>) -> Result<f64, PeripheralError>
```

### Async Read Methods

#### `get_speed`

Get the current speed.

```rust
pub async fn get_speed(&self) -> Result<f64, PeripheralError>
```

#### `get_stress_capacity`

Get the stress capacity.

```rust
pub async fn get_stress_capacity(&self) -> Result<f64, PeripheralError>
```

#### `get_energy_consumption`

Get the energy consumption.

```rust
pub async fn get_energy_consumption(&self) -> Result<f64, PeripheralError>
```

#### `get_max_insert`

Get the maximum energy insertion rate.

```rust
pub async fn get_max_insert(&self) -> Result<f64, PeripheralError>
```

#### `get_max_extract`

Get the maximum energy extraction rate.

```rust
pub async fn get_max_extract(&self) -> Result<f64, PeripheralError>
```

## Example

```rust
use rust_computers_api::createaddition::electric_motor::ElectricMotor;
use rust_computers_api::peripheral::Peripheral;

let motor = ElectricMotor::wrap(addr);

// Set speed to 128 RPM
motor.set_speed(128.0).await?;

// Get current status
let speed = motor.get_speed().await?;
let consumption = motor.get_energy_consumption().await?;

// Rotate 360 degrees
let time = motor.rotate(360.0, Some(64.0)).await?;

// Stop
motor.stop().await?;
```
