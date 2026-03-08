# GasNozzle

**モジュール:** `clockwork_cc_compat::gas_nozzle`  
**ペリフェラルタイプ:** `clockwork:gas_nozzle`

Clockwork CC Compat ガスノズル ペリフェラル。気球へのガス注入を制御し、浮力、圧力、漏れ、ガス内容物など、気球の広範なモニタリングを提供します。

## ブックリードメソッド

### セッターメソッド

#### `book_next_set_pointer` / `read_last_set_pointer`
ポインター値（注入率制御）を設定します。
```rust
pub fn book_next_set_pointer(&mut self, value: f64)
pub fn read_last_set_pointer(&self) -> Result<(), PeripheralError>
```
**パラメータ:**
- `value: f64` — 注入率制御用のポインター値

**戻り値:** `()`

---

### イミディエイトゲッターメソッド（`_imm` バリアント付き）

#### `book_next_get_pointer` / `read_last_get_pointer`
現在のポインター値を取得します。
```rust
pub fn book_next_get_pointer(&mut self)
pub fn read_last_get_pointer(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_pointer_speed` / `read_last_get_pointer_speed`
ポインター速度を取得します。
```rust
pub fn book_next_get_pointer_speed(&mut self)
pub fn read_last_get_pointer_speed(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_pocket_temperature` / `read_last_get_pocket_temperature`
ポケット温度を取得します。
```rust
pub fn book_next_get_pocket_temperature(&mut self)
pub fn read_last_get_pocket_temperature(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_duct_temperature` / `read_last_get_duct_temperature`
ダクト温度を取得します。
```rust
pub fn book_next_get_duct_temperature(&mut self)
pub fn read_last_get_duct_temperature(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_target_temperature` / `read_last_get_target_temperature`
目標温度を取得します。
```rust
pub fn book_next_get_target_temperature(&mut self)
pub fn read_last_get_target_temperature(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_balloon_volume` / `read_last_get_balloon_volume`
気球の体積を取得します。
```rust
pub fn book_next_get_balloon_volume(&mut self)
pub fn read_last_get_balloon_volume(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_leaks` / `read_last_get_leaks`
漏れ位置のリストを取得します。
```rust
pub fn book_next_get_leaks(&mut self)
pub fn read_last_get_leaks(&self) -> Result<Vec<LeakInfo>, PeripheralError>
```
**戻り値:** `Vec<LeakInfo>`

---

#### `book_next_get_temperature_delta` / `read_last_get_temperature_delta`
温度差を取得します。
```rust
pub fn book_next_get_temperature_delta(&mut self)
pub fn read_last_get_temperature_delta(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_has_balloon` / `read_last_has_balloon`
気球が接続されているかを確認します。
```rust
pub fn book_next_has_balloon(&mut self)
pub fn read_last_has_balloon(&self) -> Result<bool, PeripheralError>
```
**戻り値:** `bool`

---

### 非イミディエイトゲッターメソッド（ブックリード専用）

#### `book_next_get_buoyancy_force` / `read_last_get_buoyancy_force`
現在の浮力（N）を取得します。
```rust
pub fn book_next_get_buoyancy_force(&mut self)
pub fn read_last_get_buoyancy_force(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_balloon_pressure` / `read_last_get_balloon_pressure`
気球の内部圧力を取得します。
```rust
pub fn book_next_get_balloon_pressure(&mut self)
pub fn read_last_get_balloon_pressure(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_balloon_gas_contents` / `read_last_get_balloon_gas_contents`
気球のガス内容物をガス名と質量（kg）のマップで取得します。
```rust
pub fn book_next_get_balloon_gas_contents(&mut self)
pub fn read_last_get_balloon_gas_contents(&self) -> Result<BTreeMap<String, f64>, PeripheralError>
```
**戻り値:** `BTreeMap<String, f64>`

---

#### `book_next_get_loss_rate` / `read_last_get_loss_rate`
ガス損失率（kg/s）を取得します。
```rust
pub fn book_next_get_loss_rate(&mut self)
pub fn read_last_get_loss_rate(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_inflow_rate` / `read_last_get_inflow_rate`
ガス流入率（kg/s）を取得します。
```rust
pub fn book_next_get_inflow_rate(&mut self)
pub fn read_last_get_inflow_rate(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_missing_positions` / `read_last_get_missing_positions`
気球に必要な欠落ブロック位置のリストを取得します。
```rust
pub fn book_next_get_missing_positions(&mut self)
pub fn read_last_get_missing_positions(&self) -> Result<Vec<CLPosition>, PeripheralError>
```
**戻り値:** `Vec<CLPosition>`

---

#### `book_next_get_total_gas_mass` / `read_last_get_total_gas_mass`
気球内のガス総質量（kg）を取得します。
```rust
pub fn book_next_get_total_gas_mass(&mut self)
pub fn read_last_get_total_gas_mass(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_leak_integrity` / `read_last_get_leak_integrity`
漏れ整合性（0–1）を取得します。
```rust
pub fn book_next_get_leak_integrity(&mut self)
pub fn read_last_get_leak_integrity(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_max_leaks` / `read_last_get_max_leaks`
許容される最大漏れ数を取得します。
```rust
pub fn book_next_get_max_leaks(&mut self)
pub fn read_last_get_max_leaks(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_internal_density` / `read_last_get_internal_density`
気球の内部密度を取得します。
```rust
pub fn book_next_get_internal_density(&mut self)
pub fn read_last_get_internal_density(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### ガスネットワーク共通メソッド

[GasNetwork](GasNetwork.md) を参照: `getTemperature`、`getPressure`、`getHeatEnergy`、`getGasMass`、`getPosition`、`getNetworkInfo`。

## イミディエイトメソッド

- `get_pointer_imm(&self) -> Result<f64, PeripheralError>`
- `get_pointer_speed_imm(&self) -> Result<f64, PeripheralError>`
- `get_pocket_temperature_imm(&self) -> Result<f64, PeripheralError>`
- `get_duct_temperature_imm(&self) -> Result<f64, PeripheralError>`
- `get_target_temperature_imm(&self) -> Result<f64, PeripheralError>`
- `get_balloon_volume_imm(&self) -> Result<f64, PeripheralError>`
- `get_leaks_imm(&self) -> Result<Vec<LeakInfo>, PeripheralError>`
- `get_temperature_delta_imm(&self) -> Result<f64, PeripheralError>`
- `has_balloon_imm(&self) -> Result<bool, PeripheralError>`
- その他すべてのGasNetwork `_imm` バリアント

## 型定義

```rust
pub struct LeakInfo {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct CLPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
```

## 使用例

```rust
use rust_computers_api::clockwork_cc_compat::GasNozzle;
use rust_computers_api::peripheral::Peripheral;

let mut nozzle = GasNozzle::wrap(addr);

loop {
    let has = nozzle.read_last_has_balloon();
    let buoyancy = nozzle.read_last_get_buoyancy_force();

    nozzle.book_next_has_balloon();
    nozzle.book_next_get_buoyancy_force();
    nozzle.book_next_set_pointer(0.5);
    wait_for_next_tick().await;
}
```
