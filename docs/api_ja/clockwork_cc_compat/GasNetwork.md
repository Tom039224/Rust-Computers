# GasNetwork

**モジュール:** `clockwork_cc_compat::gas_network`

すべてのClockworkガスペリフェラルが継承する、マクロベースの共有メソッド群です。スタンドアロンのペリフェラルではなく、AirCompressor、CoalBurner、DuctTank、Exhaust、GasNozzle、GasPump、GasThruster、GasValve、Radiator、RedstoneDuctにガスネットワーク共通メソッドを提供するトレイトです。

## ブックリードメソッド

すべての `imm_getter` メソッドには `_imm` イミディエイトバリアントがあります。

### `book_next_get_temperature` / `read_last_get_temperature`
ガスネットワークの温度を取得します。
```rust
pub fn book_next_get_temperature(&mut self)
pub fn read_last_get_temperature(&self) -> Result<f64, PeripheralError>
pub fn get_temperature_imm(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_pressure` / `read_last_get_pressure`
ガスネットワークの圧力を取得します。
```rust
pub fn book_next_get_pressure(&mut self)
pub fn read_last_get_pressure(&self) -> Result<f64, PeripheralError>
pub fn get_pressure_imm(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_heat_energy` / `read_last_get_heat_energy`
ガスネットワーク内の熱エネルギーを取得します。
```rust
pub fn book_next_get_heat_energy(&mut self)
pub fn read_last_get_heat_energy(&self) -> Result<f64, PeripheralError>
pub fn get_heat_energy_imm(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_gas_mass` / `read_last_get_gas_mass`
ガス名とその質量（kg）のマップを取得します。
```rust
pub fn book_next_get_gas_mass(&mut self)
pub fn read_last_get_gas_mass(&self) -> Result<BTreeMap<String, f64>, PeripheralError>
pub fn get_gas_mass_imm(&self) -> Result<BTreeMap<String, f64>, PeripheralError>
```
**戻り値:** `BTreeMap<String, f64>`

---

### `book_next_get_position` / `read_last_get_position`
ブロック位置を取得します。
```rust
pub fn book_next_get_position(&mut self)
pub fn read_last_get_position(&self) -> Result<CLPosition, PeripheralError>
pub fn get_position_imm(&self) -> Result<CLPosition, PeripheralError>
```
**戻り値:** `CLPosition`

---

### `book_next_get_network_info` / `read_last_get_network_info`
ガスネットワークの詳細情報を取得します。このメソッドは**非imm**（ブックリード専用）です。
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


## 型定義

```rust
pub struct CLPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
```
