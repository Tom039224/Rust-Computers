# DynamicMotor

**モジュール:** `control_craft::dynamic_motor`  
**ペリフェラルタイプ:** `controlcraft:dynamic_motor_peripheral`

Control-Craft の DynamicMotor ペリフェラル。PID制御、角度調整、トルク出力、ロック/アンロック機能を備えた回転モーターを制御します。

## Book-Read メソッド

### プロパティ取得（imm 対応）

以下のメソッドは全て `book_read_imm!` マクロパターンを使用しており、`book_next_*`、`read_last_*`、`*_imm` の3バリアントを提供します。

| Book メソッド | Read メソッド | Imm メソッド | 戻り値 |
|---|---|---|---|
| `book_next_get_target_value` | `read_last_get_target_value` | `get_target_value_imm` | `f64` |
| `book_next_get_physics` | `read_last_get_physics` | `get_physics_imm` | `Value` |
| `book_next_get_angle` | `read_last_get_angle` | `get_angle_imm` | `f64` |
| `book_next_get_angular_velocity` | `read_last_get_angular_velocity` | `get_angular_velocity_imm` | `f64` |
| `book_next_get_current_value` | `read_last_get_current_value` | `get_current_value_imm` | `f64` |
| `book_next_get_relative` | `read_last_get_relative` | `get_relative_imm` | `[[f64; 3]; 3]` |
| `book_next_is_locked` | `read_last_is_locked` | `is_locked_imm` | `bool` |

---

### セッター

#### `book_next_set_pid` / `read_last_set_pid`
モーター制御用の PID ゲインを設定します。
```rust
pub fn book_next_set_pid(&mut self, p: f64, i: f64, d: f64) { ... }
pub fn read_last_set_pid(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `p: f64` — 比例ゲイン、`i: f64` — 積分ゲイン、`d: f64` — 微分ゲイン

#### `book_next_set_target_value` / `read_last_set_target_value`
目標値（角度・度）を設定します。
```rust
pub fn book_next_set_target_value(&mut self, value: f64) { ... }
pub fn read_last_set_target_value(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `value: f64` — 目標角度（度）

#### `book_next_set_output_torque` / `read_last_set_output_torque`
出力トルクスケールを設定します。
```rust
pub fn book_next_set_output_torque(&mut self, scale: f64) { ... }
pub fn read_last_set_output_torque(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `scale: f64` — トルクスケール係数

#### `book_next_set_is_adjusting_angle` / `read_last_set_is_adjusting_angle`
角度調整モードの有効/無効を切り替えます。
```rust
pub fn book_next_set_is_adjusting_angle(&mut self, enabled: bool) { ... }
pub fn read_last_set_is_adjusting_angle(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `enabled: bool` — 角度調整を有効にするか

---

### ロック制御

#### `book_next_lock` / `read_last_lock`
モーターをロック（固定）します。
```rust
pub fn book_next_lock(&mut self) { ... }
pub fn read_last_lock(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_unlock` / `read_last_unlock`
モーターのロックを解除します。
```rust
pub fn book_next_unlock(&mut self) { ... }
pub fn read_last_unlock(&self) -> Result<(), PeripheralError> { ... }
```

## 使用例

```rust
use rust_computers_api::control_craft::dynamic_motor::*;
use rust_computers_api::peripheral::Peripheral;

let mut motor = DynamicMotor::find().unwrap();

// PID と目標値を設定
motor.book_next_set_pid(1.0, 0.1, 0.05);
motor.book_next_set_target_value(90.0);
wait_for_next_tick().await;
let _ = motor.read_last_set_pid();
let _ = motor.read_last_set_target_value();

// 現在の角度を即時取得
let angle = motor.get_angle_imm().unwrap();

// モーターをロック
motor.book_next_lock();
wait_for_next_tick().await;
let _ = motor.read_last_lock();
```
