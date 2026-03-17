# Spinalyzer

**モジュール:** `control_craft::spinalyzer`  
**ペリフェラルタイプ:** `controlcraft:spinalyzer_peripheral`

Control-Craft の Spinalyzer ペリフェラル。船の姿勢、位置、速度、角速度、物理データの読み取り、およびワールド空間・船ローカル座標系での力とトルクの印加を行います。

## Book-Read メソッド

### プロパティ取得（imm 対応）

以下のメソッドは全て `book_read_imm!` マクロパターンを使用しており、`book_next_*`、`read_last_*`、`*_imm` の3バリアントを提供します。

| Book メソッド | Read メソッド | Imm メソッド | 戻り値 |
|---|---|---|---|
| `book_next_get_quaternion` | `read_last_get_quaternion` | `get_quaternion_imm` | `CTLQuaternion` |
| `book_next_get_quaternion_j` | `read_last_get_quaternion_j` | `get_quaternion_j_imm` | `CTLQuaternion` |
| `book_next_get_rotation_matrix` | `read_last_get_rotation_matrix` | `get_rotation_matrix_imm` | `[[f64; 3]; 3]` |
| `book_next_get_rotation_matrix_t` | `read_last_get_rotation_matrix_t` | `get_rotation_matrix_t_imm` | `[[f64; 3]; 3]` |
| `book_next_get_velocity` | `read_last_get_velocity` | `get_velocity_imm` | `CTLVec3` |
| `book_next_get_angular_velocity` | `read_last_get_angular_velocity` | `get_angular_velocity_imm` | `CTLVec3` |
| `book_next_get_position` | `read_last_get_position` | `get_position_imm` | `CTLVec3` |
| `book_next_get_spinalyzer_position` | `read_last_get_spinalyzer_position` | `get_spinalyzer_position_imm` | `CTLVec3` |
| `book_next_get_spinalyzer_velocity` | `read_last_get_spinalyzer_velocity` | `get_spinalyzer_velocity_imm` | `CTLVec3` |
| `book_next_get_physics` | `read_last_get_physics` | `get_physics_imm` | `Value` |

---

### 力・トルク印加

#### `book_next_apply_invariant_force` / `read_last_apply_invariant_force`
ワールド空間固定方向の力を船の重心に印加します。
```rust
pub fn book_next_apply_invariant_force(&mut self, x: f64, y: f64, z: f64) { ... }
pub fn read_last_apply_invariant_force(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `x: f64`, `y: f64`, `z: f64` — ワールド空間での力ベクトル

#### `book_next_apply_invariant_torque` / `read_last_apply_invariant_torque`
ワールド空間固定方向のトルクを印加します。
```rust
pub fn book_next_apply_invariant_torque(&mut self, x: f64, y: f64, z: f64) { ... }
pub fn read_last_apply_invariant_torque(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `x: f64`, `y: f64`, `z: f64` — ワールド空間でのトルクベクトル

#### `book_next_apply_rot_dependent_force` / `read_last_apply_rot_dependent_force`
船ローカル座標系の力を印加します（船の回転に追従）。
```rust
pub fn book_next_apply_rot_dependent_force(&mut self, x: f64, y: f64, z: f64) { ... }
pub fn read_last_apply_rot_dependent_force(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `x: f64`, `y: f64`, `z: f64` — 船ローカル空間での力ベクトル

#### `book_next_apply_rot_dependent_torque` / `read_last_apply_rot_dependent_torque`
船ローカル座標系のトルクを印加します（船の回転に追従）。
```rust
pub fn book_next_apply_rot_dependent_torque(&mut self, x: f64, y: f64, z: f64) { ... }
pub fn read_last_apply_rot_dependent_torque(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `x: f64`, `y: f64`, `z: f64` — 船ローカル空間でのトルクベクトル

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* / async_* methods

## 型定義

### `CTLQuaternion`
回転を表すクォータニオン。
```rust
pub struct CTLQuaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
```

### `CTLVec3`
3次元ベクトル。
```rust
pub struct CTLVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```

## 使用例

```rust
use rust_computers_api::control_craft::spinalyzer::*;
use rust_computers_api::peripheral::Peripheral;

let mut spin = Spinalyzer::find().unwrap();

// 船の姿勢を即時取得
let quat = spin.get_quaternion_imm().unwrap();
let pos = spin.get_position_imm().unwrap();
let vel = spin.get_velocity_imm().unwrap();

// ワールド空間で上方向の力を印加
spin.book_next_apply_invariant_force(0.0, 100.0, 0.0);
wait_for_next_tick().await;
let _ = spin.read_last_apply_invariant_force();

// 船ローカル空間で前方向の力を印加
spin.book_next_apply_rot_dependent_force(0.0, 0.0, 50.0);
wait_for_next_tick().await;
let _ = spin.read_last_apply_rot_dependent_force();
```
