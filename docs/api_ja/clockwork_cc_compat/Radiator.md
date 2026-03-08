# Radiator

**モジュール:** `clockwork_cc_compat::radiator`  
**ペリフェラルタイプ:** `clockwork:radiator`

Clockwork CC Compat ラジエーター ペリフェラル。ファン情報、熱特性、ガス変換、共有ガスネットワーク情報を含むラジエーター状態の包括的なモニタリングを提供します。

## ブックリードメソッド

以下のすべてのメソッドには `_imm` イミディエイトバリアントがあります。

### `book_next_get_fan_type` / `read_last_get_fan_type`
ファンタイプ名を取得します。
```rust
pub fn book_next_get_fan_type(&mut self)
pub fn read_last_get_fan_type(&self) -> Result<String, PeripheralError>
```
**戻り値:** `String`

---

### `book_next_get_fan_rpm` / `read_last_get_fan_rpm`
ファンのRPMを取得します。
```rust
pub fn book_next_get_fan_rpm(&mut self)
pub fn read_last_get_fan_rpm(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_fan_count` / `read_last_get_fan_count`
ファンの数を取得します。
```rust
pub fn book_next_get_fan_count(&mut self)
pub fn read_last_get_fan_count(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_fans` / `read_last_get_fans`
すべてのファンの詳細情報を取得します。
```rust
pub fn book_next_get_fans(&mut self)
pub fn read_last_get_fans(&self) -> Result<Vec<FanInfo>, PeripheralError>
```
**戻り値:** `Vec<FanInfo>`

---

### `book_next_is_active` / `read_last_is_active`
ラジエーターが稼働中かを確認します。
```rust
pub fn book_next_is_active(&mut self)
pub fn read_last_is_active(&self) -> Result<bool, PeripheralError>
```
**戻り値:** `bool`

---

### `book_next_is_cooling` / `read_last_is_cooling`
ラジエーターが冷却モードかを確認します。
```rust
pub fn book_next_is_cooling(&mut self)
pub fn read_last_is_cooling(&self) -> Result<bool, PeripheralError>
```
**戻り値:** `bool`

---

### `book_next_is_heating` / `read_last_is_heating`
ラジエーターが加熱モードかを確認します。
```rust
pub fn book_next_is_heating(&mut self)
pub fn read_last_is_heating(&self) -> Result<bool, PeripheralError>
```
**戻り値:** `bool`

---

### `book_next_get_target_temp` / `read_last_get_target_temp`
目標温度を取得します。
```rust
pub fn book_next_get_target_temp(&mut self)
pub fn read_last_get_target_temp(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_input_temperature` / `read_last_get_input_temperature`
入力温度を取得します。
```rust
pub fn book_next_get_input_temperature(&mut self)
pub fn read_last_get_input_temperature(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_output_temperature` / `read_last_get_output_temperature`
出力温度を取得します。
```rust
pub fn book_next_get_output_temperature(&mut self)
pub fn read_last_get_output_temperature(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_thermal_factor` / `read_last_get_thermal_factor`
熱係数を取得します。
```rust
pub fn book_next_get_thermal_factor(&mut self)
pub fn read_last_get_thermal_factor(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_atmospheric_pressure` / `read_last_get_atmospheric_pressure`
大気圧を取得します。
```rust
pub fn book_next_get_atmospheric_pressure(&mut self)
pub fn read_last_get_atmospheric_pressure(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_pressure_scale` / `read_last_get_pressure_scale`
圧力スケールを取得します。
```rust
pub fn book_next_get_pressure_scale(&mut self)
pub fn read_last_get_pressure_scale(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_thermal_power` / `read_last_get_thermal_power`
熱出力を取得します。
```rust
pub fn book_next_get_thermal_power(&mut self)
pub fn read_last_get_thermal_power(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_status` / `read_last_get_status`
ラジエーターのステータスを取得します。
```rust
pub fn book_next_get_status(&mut self)
pub fn read_last_get_status(&self) -> Result<String, PeripheralError>
```
**戻り値:** `String`

---

### `book_next_get_conversion_rate` / `read_last_get_conversion_rate`
ガス変換率を取得します。
```rust
pub fn book_next_get_conversion_rate(&mut self)
pub fn read_last_get_conversion_rate(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_conversions` / `read_last_get_conversions`
ガス変換リストを取得します。
```rust
pub fn book_next_get_conversions(&mut self)
pub fn read_last_get_conversions(&self) -> Result<Vec<ConversionInfo>, PeripheralError>
```
**戻り値:** `Vec<ConversionInfo>`

---

### ガスネットワーク共通メソッド

[GasNetwork](GasNetwork.md) を参照: `getTemperature`、`getPressure`、`getHeatEnergy`、`getGasMass`、`getPosition`、`getNetworkInfo`。

## イミディエイトメソッド

すべてのimm_getterメソッドには `_imm` バリアントがあります：

- `get_fan_type_imm`、`get_fan_rpm_imm`、`get_fan_count_imm`、`get_fans_imm`
- `is_active_imm`、`is_cooling_imm`、`is_heating_imm`
- `get_target_temp_imm`、`get_input_temperature_imm`、`get_output_temperature_imm`
- `get_thermal_factor_imm`、`get_atmospheric_pressure_imm`、`get_pressure_scale_imm`
- `get_thermal_power_imm`、`get_status_imm`
- `get_conversion_rate_imm`、`get_conversions_imm`
- その他すべてのGasNetwork `_imm` バリアント

## 型定義

```rust
pub struct FanInfo {
    pub r#type: String,
    pub rpm: f64,
    pub dir: String,
    pub dist: f64,
}

pub struct ConversionInfo {
    pub from: String,
    pub to: String,
    pub amount: f64,
}

pub struct CLPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
```

## 使用例

```rust
use rust_computers_api::clockwork_cc_compat::Radiator;
use rust_computers_api::peripheral::Peripheral;

let mut radiator = Radiator::wrap(addr);

loop {
    let active = radiator.read_last_is_active();
    let status = radiator.read_last_get_status();
    let fans = radiator.read_last_get_fans();

    radiator.book_next_is_active();
    radiator.book_next_get_status();
    radiator.book_next_get_fans();
    wait_for_next_tick().await;
}
```
