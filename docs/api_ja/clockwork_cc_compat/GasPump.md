# GasPump

**モジュール:** `clockwork_cc_compat::gas_pump`  
**ペリフェラルタイプ:** `clockwork:gas_pump`

Clockwork CC Compat ガスポンプ ペリフェラル。ポンプ圧力、速度、向き、共有ガスネットワーク情報へのアクセスを提供します。

## ブックリードメソッド

以下のすべてのメソッドには `_imm` イミディエイトバリアントがあります。

### `book_next_get_pump_pressure` / `read_last_get_pump_pressure`
ポンプ圧力を取得します。
```rust
pub fn book_next_get_pump_pressure(&mut self)
pub fn read_last_get_pump_pressure(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_speed` / `read_last_get_speed`
現在の速度を取得します。
```rust
pub fn book_next_get_speed(&mut self)
pub fn read_last_get_speed(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

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

## イミディエイトメソッド

- `get_pump_pressure_imm(&self) -> Result<f64, PeripheralError>`
- `get_speed_imm(&self) -> Result<f64, PeripheralError>`
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
use rust_computers_api::clockwork_cc_compat::GasPump;
use rust_computers_api::peripheral::Peripheral;

let mut pump = GasPump::wrap(addr);

loop {
    let pressure = pump.read_last_get_pump_pressure();
    let speed = pump.read_last_get_speed();

    pump.book_next_get_pump_pressure();
    pump.book_next_get_speed();
    wait_for_next_tick().await;
}
```
