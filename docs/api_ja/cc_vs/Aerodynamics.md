# Aerodynamics

**モジュール:** `cc_vs::aerodynamics`  
**ペリフェラルタイプ:** `vs_aerodynamics`

CC-VS の Aerodynamics グローバル API。大気や空気力学に関するプロパティを取得するために使用します。定数プロパティの即時取得と、計算値の book-read メソッドの両方を提供しています。

## 即時メソッド（定数）

以下のプロパティは読み取り専用の定数で、即時呼び出しのみで取得可能です。

### `default_max_imm`
デフォルトの最大高度を取得します。
```rust
pub fn default_max_imm(&self) -> Result<f64, PeripheralError> { ... }
```

### `default_sea_level_imm`
デフォルトの海面レベルを取得します。
```rust
pub fn default_sea_level_imm(&self) -> Result<f64, PeripheralError> { ... }
```

### `drag_coefficient_imm`
抗力係数を取得します。
```rust
pub fn drag_coefficient_imm(&self) -> Result<f64, PeripheralError> { ... }
```

### `gravitational_acceleration_imm`
重力加速度定数を取得します。
```rust
pub fn gravitational_acceleration_imm(&self) -> Result<f64, PeripheralError> { ... }
```

### `universal_gas_constant_imm`
普遍気体定数を取得します。
```rust
pub fn universal_gas_constant_imm(&self) -> Result<f64, PeripheralError> { ... }
```

### `air_molar_mass_imm`
空気のモル質量を取得します。
```rust
pub fn air_molar_mass_imm(&self) -> Result<f64, PeripheralError> { ... }
```

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* / async_* methods

## Book-Read メソッド

### `book_next_get_atmospheric_parameters` / `read_last_get_atmospheric_parameters`
大気パラメータを取得します。
```rust
pub fn book_next_get_atmospheric_parameters(&mut self) { ... }
pub fn read_last_get_atmospheric_parameters(&self) -> Result<Option<VSAtmosphericParameters>, PeripheralError> { ... }
```
**戻り値:** `Option<VSAtmosphericParameters>`

---

### `book_next_get_air_density` / `read_last_get_air_density`
指定 Y 座標の空気密度を取得します（`None` の場合は現在位置を使用）。
```rust
pub fn book_next_get_air_density(&mut self, y: Option<f64>) { ... }
pub fn read_last_get_air_density(&self) -> Result<Option<f64>, PeripheralError> { ... }
```
**パラメータ:**
- `y: Option<f64>` — Y 座標（省略時は現在位置を使用）

**戻り値:** `Option<f64>`

---

### `book_next_get_air_pressure` / `read_last_get_air_pressure`
指定 Y 座標の大気圧を取得します。
```rust
pub fn book_next_get_air_pressure(&mut self, y: Option<f64>) { ... }
pub fn read_last_get_air_pressure(&self) -> Result<Option<f64>, PeripheralError> { ... }
```
**パラメータ:**
- `y: Option<f64>` — Y 座標（省略可）

**戻り値:** `Option<f64>`

---

### `book_next_get_air_temperature` / `read_last_get_air_temperature`
指定 Y 座標の気温を取得します。
```rust
pub fn book_next_get_air_temperature(&mut self, y: Option<f64>) { ... }
pub fn read_last_get_air_temperature(&self) -> Result<Option<f64>, PeripheralError> { ... }
```
**パラメータ:**
- `y: Option<f64>` — Y 座標（省略可）

**戻り値:** `Option<f64>`

## 即時メソッド（計算値）

### `get_atmospheric_parameters_imm`
```rust
pub fn get_atmospheric_parameters_imm(&self) -> Result<Option<VSAtmosphericParameters>, PeripheralError> { ... }
```

### `get_air_density_imm`
```rust
pub fn get_air_density_imm(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError> { ... }
```

### `get_air_pressure_imm`
```rust
pub fn get_air_pressure_imm(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError> { ... }
```

### `get_air_temperature_imm`
```rust
pub fn get_air_temperature_imm(&self, y: Option<f64>) -> Result<Option<f64>, PeripheralError> { ... }
```

## 型定義

### `VSAtmosphericParameters`
```rust
pub struct VSAtmosphericParameters {
    pub max_y: f64,
    pub sea_level: f64,
    pub gravity: f64,
}
```

## 使用例

```rust
use rust_computers_api::cc_vs::aerodynamics::*;
use rust_computers_api::peripheral::Peripheral;

let mut aero = Aerodynamics::find().unwrap();

// 定数を即時取得
let gravity = aero.gravitational_acceleration_imm().unwrap();
let drag_coeff = aero.drag_coefficient_imm().unwrap();

// Y=100 の空気密度を取得
aero.book_next_get_air_density(Some(100.0));
wait_for_next_tick().await;
let density = aero.read_last_get_air_density().unwrap();

// 即時バリアントで大気圧を取得
let pressure = aero.get_air_pressure_imm(Some(64.0)).unwrap();
```
