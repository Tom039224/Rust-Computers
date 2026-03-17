# CoalBurner

**モジュール:** `clockwork_cc_compat::coal_burner`  
**ペリフェラルタイプ:** `clockwork:coal_burner`

Clockwork CC Compat 石炭バーナー ペリフェラル。燃料ステータス、最大燃焼時間、共有ガスネットワーク情報へのアクセスを提供します。

## ブックリードメソッド

以下のすべてのメソッドには `_imm` イミディエイトバリアントがあります。

### `book_next_get_fuel_ticks` / `read_last_get_fuel_ticks`
残りの燃料ティック数を取得します。
```rust
pub fn book_next_get_fuel_ticks(&mut self)
pub fn read_last_get_fuel_ticks(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_max_burn_time` / `read_last_get_max_burn_time`
現在の燃料の最大燃焼時間を取得します。
```rust
pub fn book_next_get_max_burn_time(&mut self)
pub fn read_last_get_max_burn_time(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_is_burning` / `read_last_is_burning`
バーナーが現在燃料を燃焼中かどうかを確認します。
```rust
pub fn book_next_is_burning(&mut self)
pub fn read_last_is_burning(&self) -> Result<bool, PeripheralError>
```
**戻り値:** `bool`

---

### ガスネットワーク共通メソッド

#### `book_next_get_temperature` / `read_last_get_temperature`
ガスネットワークの温度を取得します。
```rust
pub fn book_next_get_temperature(&mut self)
pub fn read_last_get_temperature(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_pressure` / `read_last_get_pressure`
ガスネットワークの圧力を取得します。
```rust
pub fn book_next_get_pressure(&mut self)
pub fn read_last_get_pressure(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_heat_energy` / `read_last_get_heat_energy`
ガスネットワーク内の熱エネルギーを取得します。
```rust
pub fn book_next_get_heat_energy(&mut self)
pub fn read_last_get_heat_energy(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_gas_mass` / `read_last_get_gas_mass`
ガス名とその質量（kg）のマップを取得します。
```rust
pub fn book_next_get_gas_mass(&mut self)
pub fn read_last_get_gas_mass(&self) -> Result<BTreeMap<String, f64>, PeripheralError>
```
**戻り値:** `BTreeMap<String, f64>`

---

#### `book_next_get_position` / `read_last_get_position`
ブロック位置を取得します。
```rust
pub fn book_next_get_position(&mut self)
pub fn read_last_get_position(&self) -> Result<CLPosition, PeripheralError>
```
**戻り値:** `CLPosition`

---

#### `book_next_get_network_info` / `read_last_get_network_info`
ガスネットワークの詳細情報を取得します（非imm、ブックリード専用）。
```rust
pub fn book_next_get_network_info(&mut self)
pub fn read_last_get_network_info(&self) -> Result<Value, PeripheralError>
```
**戻り値:** `Value`（動的テーブル）

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## イミディエイトメソッド

- `get_fuel_ticks_imm(&self) -> Result<f64, PeripheralError>`
- `get_max_burn_time_imm(&self) -> Result<f64, PeripheralError>`
- `is_burning_imm(&self) -> Result<bool, PeripheralError>`
- `get_temperature_imm(&self) -> Result<f64, PeripheralError>`
- `get_pressure_imm(&self) -> Result<f64, PeripheralError>`
- `get_heat_energy_imm(&self) -> Result<f64, PeripheralError>`
- `get_gas_mass_imm(&self) -> Result<BTreeMap<String, f64>, PeripheralError>`
- `get_position_imm(&self) -> Result<CLPosition, PeripheralError>`

## 型定義

```rust
pub struct CLPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
```

## 使用例

```rust
use rust_computers_api::clockwork_cc_compat::CoalBurner;
use rust_computers_api::peripheral::Peripheral;

let mut burner = CoalBurner::wrap(addr);

loop {
    let burning = burner.read_last_is_burning();
    let fuel = burner.read_last_get_fuel_ticks();

    burner.book_next_is_burning();
    burner.book_next_get_fuel_ticks();
    wait_for_next_tick().await;
}
```
