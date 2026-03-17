# KinematicMotor

**モジュール:** `control_craft::kinematic_motor`  
**ペリフェラルタイプ:** `controlcraft:kinematic_motor_peripheral`

Control-Craft の KinematicMotor ペリフェラル。目標角度の管理、制御ターゲットの選択、強制角度モードを備えた位置ベースのモーター制御を行います。

## Book-Read メソッド

### プロパティ取得（imm 対応）

以下のメソッドは全て `book_read_imm!` マクロパターンを使用しており、`book_next_*`、`read_last_*`、`*_imm` の3バリアントを提供します。

| Book メソッド | Read メソッド | Imm メソッド | 戻り値 |
|---|---|---|---|
| `book_next_get_target_angle` | `read_last_get_target_angle` | `get_target_angle_imm` | `f64` |
| `book_next_get_control_target` | `read_last_get_control_target` | `get_control_target_imm` | `String` |
| `book_next_get_physics` | `read_last_get_physics` | `get_physics_imm` | `Value` |
| `book_next_get_angle` | `read_last_get_angle` | `get_angle_imm` | `f64` |
| `book_next_get_relative` | `read_last_get_relative` | `get_relative_imm` | `[[f64; 3]; 3]` |

---

### セッター

#### `book_next_set_target_angle` / `read_last_set_target_angle`
目標角度（度）を設定します。
```rust
pub fn book_next_set_target_angle(&mut self, value: f64) { ... }
pub fn read_last_set_target_angle(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `value: f64` — 目標角度（度）

#### `book_next_set_control_target` / `read_last_set_control_target`
制御ターゲットを設定します。
```rust
pub fn book_next_set_control_target(&mut self, target: &str) { ... }
pub fn read_last_set_control_target(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `target: &str` — 制御ターゲット名

#### `book_next_set_is_forcing_angle` / `read_last_set_is_forcing_angle`
強制角度モードの有効/無効を切り替えます。
```rust
pub fn book_next_set_is_forcing_angle(&mut self, enabled: bool) { ... }
pub fn read_last_set_is_forcing_angle(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `enabled: bool` — 強制角度モードを有効にするか

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* / async_* methods

## 使用例

```rust
use rust_computers_api::control_craft::kinematic_motor::*;
use rust_computers_api::peripheral::Peripheral;

let mut motor = KinematicMotor::find().unwrap();

// 目標角度を設定
motor.book_next_set_target_angle(45.0);
wait_for_next_tick().await;
let _ = motor.read_last_set_target_angle();

// 現在の角度を即時取得
let angle = motor.get_angle_imm().unwrap();

// 現在の制御ターゲットを確認
let target = motor.get_control_target_imm().unwrap();
```
