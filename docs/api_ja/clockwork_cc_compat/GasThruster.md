# GasThruster

**モジュール:** `clockwork_cc_compat::gas_thruster`  
**ペリフェラルタイプ:** `clockwork:gas_thruster`

Clockwork CC Compat ガススラスター ペリフェラル。推力、流量、ガス質量流量、向き、共有ガスネットワーク情報へのアクセスを提供します。

## ブックリードメソッド

以下のすべてのメソッドには `_imm` イミディエイトバリアントがあります。

### `book_next_get_thrust` / `read_last_get_thrust`
現在の推力値を取得します。
```rust
pub fn book_next_get_thrust(&mut self)
pub fn read_last_get_thrust(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_flow_rate` / `read_last_get_flow_rate`
現在の流量を取得します。
```rust
pub fn book_next_get_flow_rate(&mut self)
pub fn read_last_get_flow_rate(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_gas_mass_flow` / `read_last_get_gas_mass_flow`
ガス名とその質量流量（kg/s）のマップを取得します。
```rust
pub fn book_next_get_gas_mass_flow(&mut self)
pub fn read_last_get_gas_mass_flow(&self) -> Result<BTreeMap<String, f64>, PeripheralError>
```
**戻り値:** `BTreeMap<String, f64>`

---

### `book_next_get_facing` / `read_last_get_facing`
向きを取得します。
```rust
pub fn book_next_get_facing(&mut self)
pub fn read_last_get_facing(&self) -> Result<String, PeripheralError>
```
**戻り値:** `String`

---

### ガスネットワーク共通メソッド

[GasNetwork](GasNetwork.md) を参照: `getTemperature`、`getPressure`、`getHeatEnergy`、`getGasMass`、`getPosition`、`getNetworkInfo`。

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* / async_* methods

## イミディエイトメソッド

- `get_thrust_imm(&self) -> Result<f64, PeripheralError>`
- `get_flow_rate_imm(&self) -> Result<f64, PeripheralError>`
- `get_gas_mass_flow_imm(&self) -> Result<BTreeMap<String, f64>, PeripheralError>`
- `get_facing_imm(&self) -> Result<String, PeripheralError>`
- その他すべてのGasNetwork `_imm` バリアント

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
use rust_computers_api::clockwork_cc_compat::GasThruster;
use rust_computers_api::peripheral::Peripheral;

let mut thruster = GasThruster::wrap(addr);

loop {
    let thrust = thruster.read_last_get_thrust();
    let flow = thruster.read_last_get_flow_rate();

    thruster.book_next_get_thrust();
    thruster.book_next_get_flow_rate();
    wait_for_next_tick().await;
}
```
