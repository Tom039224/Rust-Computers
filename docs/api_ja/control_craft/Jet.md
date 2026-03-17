# Jet

**モジュール:** `control_craft::jet`  
**ペリフェラルタイプ:** `controlcraft:jet_peripheral`

Control-Craft の Jet ペリフェラル。ジェット推進装置の推力出力とチルト角度を制御します。

## Book-Read メソッド

### セッター

#### `book_next_set_output_thrust` / `read_last_set_output_thrust`
出力推力スケールを設定します。
```rust
pub fn book_next_set_output_thrust(&mut self, thrust: f64) { ... }
pub fn read_last_set_output_thrust(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `thrust: f64` — 推力スケール係数

#### `book_next_set_horizontal_tilt` / `read_last_set_horizontal_tilt`
水平方向のチルト角度（度）を設定します。
```rust
pub fn book_next_set_horizontal_tilt(&mut self, angle: f64) { ... }
pub fn read_last_set_horizontal_tilt(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `angle: f64` — 水平チルト角（度）

#### `book_next_set_vertical_tilt` / `read_last_set_vertical_tilt`
垂直方向のチルト角度（度）を設定します。
```rust
pub fn book_next_set_vertical_tilt(&mut self, angle: f64) { ... }
pub fn read_last_set_vertical_tilt(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `angle: f64` — 垂直チルト角（度）

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* / async_* methods

## 使用例

```rust
use rust_computers_api::control_craft::jet::*;
use rust_computers_api::peripheral::Peripheral;

let mut jet = Jet::find().unwrap();

// 推力とチルトを設定
jet.book_next_set_output_thrust(1.0);
jet.book_next_set_horizontal_tilt(5.0);
jet.book_next_set_vertical_tilt(-10.0);
wait_for_next_tick().await;
let _ = jet.read_last_set_output_thrust();
let _ = jet.read_last_set_horizontal_tilt();
let _ = jet.read_last_set_vertical_tilt();
```
