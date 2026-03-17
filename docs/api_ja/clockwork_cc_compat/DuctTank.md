# DuctTank

**モジュール:** `clockwork_cc_compat::duct_tank`  
**ペリフェラルタイプ:** `clockwork:duct_tank`

Clockwork CC Compat ダクトタンク ペリフェラル。タンクの寸法と共有ガスネットワーク情報へのアクセスを提供します。

## ブックリードメソッド

以下のすべてのメソッドには `_imm` イミディエイトバリアントがあります。

### `book_next_get_height` / `read_last_get_height`
タンクの高さを取得します。
```rust
pub fn book_next_get_height(&mut self)
pub fn read_last_get_height(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_width` / `read_last_get_width`
タンクの幅を取得します。
```rust
pub fn book_next_get_width(&mut self)
pub fn read_last_get_width(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

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

- `get_height_imm(&self) -> Result<f64, PeripheralError>`
- `get_width_imm(&self) -> Result<f64, PeripheralError>`
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
use rust_computers_api::clockwork_cc_compat::DuctTank;
use rust_computers_api::peripheral::Peripheral;

let mut tank = DuctTank::wrap(addr);

loop {
    let height = tank.read_last_get_height();
    let pressure = tank.read_last_get_pressure();

    tank.book_next_get_height();
    tank.book_next_get_pressure();
    wait_for_next_tick().await;
}
```
