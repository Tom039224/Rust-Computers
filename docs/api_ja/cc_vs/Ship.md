# Ship

**モジュール:** `cc_vs::ship`  
**ペリフェラルタイプ:** `ship`

CC-VS の Ship ペリフェラル。Valkyrien Skies の船を制御するために使用します。船のプロパティ取得、座標変換、力・トルクの印加、物理ティックイベントの受信が可能です。

## Book-Read メソッド

### プロパティ取得（imm 対応）

以下のメソッドは全て `book_read_imm!` マクロパターンを使用しており、`book_next_*`、`read_last_*`、`*_imm` の3バリアントを提供します。

| Book メソッド | Read メソッド | Imm メソッド | 戻り値 |
|---|---|---|---|
| `book_next_get_id` | `read_last_get_id` | `get_id_imm` | `i64` |
| `book_next_get_mass` | `read_last_get_mass` | `get_mass_imm` | `f64` |
| `book_next_get_moment_of_inertia_tensor` | `read_last_get_moment_of_inertia_tensor` | `get_moment_of_inertia_tensor_imm` | `[[f64; 3]; 3]` |
| `book_next_get_slug` | `read_last_get_slug` | `get_slug_imm` | `String` |
| `book_next_get_angular_velocity` | `read_last_get_angular_velocity` | `get_angular_velocity_imm` | `VSVector3` |
| `book_next_get_quaternion` | `read_last_get_quaternion` | `get_quaternion_imm` | `VSQuaternion` |
| `book_next_get_scale` | `read_last_get_scale` | `get_scale_imm` | `VSVector3` |
| `book_next_get_shipyard_position` | `read_last_get_shipyard_position` | `get_shipyard_position_imm` | `VSVector3` |
| `book_next_get_size` | `read_last_get_size` | `get_size_imm` | `VSVector3` |
| `book_next_get_velocity` | `read_last_get_velocity` | `get_velocity_imm` | `VSVector3` |
| `book_next_get_worldspace_position` | `read_last_get_worldspace_position` | `get_worldspace_position_imm` | `VSVector3` |
| `book_next_is_static` | `read_last_is_static` | `is_static_imm` | `bool` |
| `book_next_get_transformation_matrix` | `read_last_get_transformation_matrix` | `get_transformation_matrix_imm` | `VSTransformMatrix` |
| `book_next_get_joints` | `read_last_get_joints` | `get_joints_imm` | `Vec<VSJoint>` |

---

### `book_next_transform_position_to_world` / `read_last_transform_position_to_world`
ローカル座標（シップヤード座標）をワールド座標に変換します。
```rust
pub fn book_next_transform_position_to_world(&mut self, pos: VSVector3) { ... }
pub fn read_last_transform_position_to_world(&self) -> Result<VSVector3, PeripheralError> { ... }
```
**パラメータ:**
- `pos: VSVector3` — ローカル座標

**戻り値:** `VSVector3` — ワールド座標

---

### `book_next_set_slug` / `read_last_set_slug`
船のスラグ名を設定します。
```rust
pub fn book_next_set_slug(&mut self, name: &str) { ... }
pub fn read_last_set_slug(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `name: &str` — 新しいスラグ名

**戻り値:** `()`

---

### `book_next_set_static` / `read_last_set_static`
船の静的状態を設定します。
```rust
pub fn book_next_set_static(&mut self, is_static: bool) { ... }
pub fn read_last_set_static(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `is_static: bool` — 船を静的にするかどうか

**戻り値:** `()`

---

### `book_next_set_scale_value` / `read_last_set_scale_value`
船のスケールを設定します。
```rust
pub fn book_next_set_scale_value(&mut self, scale: f64) { ... }
pub fn read_last_set_scale_value(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `scale: f64` — スケール値

**戻り値:** `()`

---

### `book_next_teleport` / `read_last_teleport`
船をテレポートします。
```rust
pub fn book_next_teleport(&mut self, data: &VSTeleportData) -> Result<(), PeripheralError> { ... }
pub fn read_last_teleport(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `data: &VSTeleportData` — テレポートパラメータ（全フィールド省略可）

**戻り値:** `()`

---

### 力・トルクの印加

#### `book_next_apply_world_force` / `read_last_apply_world_force`
ワールド座標系で力を印加します。位置の指定は任意です。
```rust
pub fn book_next_apply_world_force(&mut self, fx: f64, fy: f64, fz: f64, pos: Option<VSVector3>) { ... }
pub fn read_last_apply_world_force(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_apply_world_torque` / `read_last_apply_world_torque`
ワールド座標系でトルクを印加します。
```rust
pub fn book_next_apply_world_torque(&mut self, tx: f64, ty: f64, tz: f64) { ... }
pub fn read_last_apply_world_torque(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_apply_model_force` / `read_last_apply_model_force`
モデル（シップヤード）座標系で力を印加します。
```rust
pub fn book_next_apply_model_force(&mut self, fx: f64, fy: f64, fz: f64, pos: Option<VSVector3>) { ... }
pub fn read_last_apply_model_force(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_apply_model_torque` / `read_last_apply_model_torque`
モデル座標系でトルクを印加します。
```rust
pub fn book_next_apply_model_torque(&mut self, tx: f64, ty: f64, tz: f64) { ... }
pub fn read_last_apply_model_torque(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_apply_world_force_to_model_pos` / `read_last_apply_world_force_to_model_pos`
ワールド座標の力をモデル座標の位置に対して印加します。
```rust
pub fn book_next_apply_world_force_to_model_pos(&mut self, fx: f64, fy: f64, fz: f64, px: f64, py: f64, pz: f64) { ... }
pub fn read_last_apply_world_force_to_model_pos(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_apply_body_force` / `read_last_apply_body_force`
ボディ座標系で力を印加します。
```rust
pub fn book_next_apply_body_force(&mut self, fx: f64, fy: f64, fz: f64, pos: Option<VSVector3>) { ... }
pub fn read_last_apply_body_force(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_apply_body_torque` / `read_last_apply_body_torque`
ボディ座標系でトルクを印加します。
```rust
pub fn book_next_apply_body_torque(&mut self, tx: f64, ty: f64, tz: f64) { ... }
pub fn read_last_apply_body_torque(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_apply_world_force_to_body_pos` / `read_last_apply_world_force_to_body_pos`
ワールド座標の力をボディ座標の位置に対して印加します。
```rust
pub fn book_next_apply_world_force_to_body_pos(&mut self, fx: f64, fy: f64, fz: f64, px: f64, py: f64, pz: f64) { ... }
pub fn read_last_apply_world_force_to_body_pos(&self) -> Result<(), PeripheralError> { ... }
```

---

### `book_next_try_pull_physics_ticks` / `read_last_try_pull_physics_ticks`
1tick 以内に物理ティックイベントの受信を試みます。
```rust
pub fn book_next_try_pull_physics_ticks(&mut self) { ... }
pub fn read_last_try_pull_physics_ticks(&self) -> Result<Option<VSPhysicsTickData>, PeripheralError> { ... }
```
**戻り値:** `Option<VSPhysicsTickData>`

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## 即時メソッド

### `transform_position_to_world_imm`
ローカル座標をワールド座標に即時変換します。
```rust
pub fn transform_position_to_world_imm(&self, pos: VSVector3) -> Result<VSVector3, PeripheralError> { ... }
```

上記テーブルのプロパティ取得メソッドも全て `*_imm` バリアントを持っています。

## イベント待機メソッド

### `pull_physics_ticks`
物理ティックイベントを受信するまで待機します（非同期）。毎 tick ポーリングしてデータが届くまでループします。
```rust
pub async fn pull_physics_ticks(&self) -> Result<VSPhysicsTickData, PeripheralError> { ... }
```
**戻り値:** `VSPhysicsTickData`

## 型定義

### `VSVector3`
```rust
pub struct VSVector3 { pub x: f64, pub y: f64, pub z: f64 }
```

### `VSQuaternion`
```rust
pub struct VSQuaternion { pub x: f64, pub y: f64, pub z: f64, pub w: f64 }
```

### `VSTransformMatrix`
```rust
pub struct VSTransformMatrix { pub matrix: [[f64; 4]; 4] }
```

### `VSJoint`
```rust
pub struct VSJoint { pub id: u64, pub name: String }
```

### `VSInertiaInfo`
```rust
pub struct VSInertiaInfo { pub moment_of_inertia: VSVector3, pub mass: f64 }
```

### `VSPoseVelInfo`
```rust
pub struct VSPoseVelInfo {
    pub vel: VSVector3, pub omega: VSVector3,
    pub pos: VSVector3, pub rot: VSQuaternion,
}
```

### `VSPhysicsTickData`
```rust
pub struct VSPhysicsTickData {
    pub buoyant_factor: f64,
    pub is_static: bool,
    pub do_fluid_drag: bool,
    pub inertia: VSInertiaInfo,
    pub pose_vel: VSPoseVelInfo,
    pub forces_inducers: Vec<String>,
}
```

### `VSTeleportData`
```rust
pub struct VSTeleportData {
    pub pos: Option<VSVector3>,
    pub rot: Option<VSQuaternion>,
    pub vel: Option<VSVector3>,
    pub omega: Option<VSVector3>,
    pub dimension: Option<String>,
    pub scale: Option<f64>,
}
```

## 使用例

```rust
use rust_computers_api::cc_vs::ship::*;
use rust_computers_api::peripheral::Peripheral;

let mut ship = Ship::find().unwrap();

// 船のワールド座標を即時取得
let pos = ship.get_worldspace_position_imm().unwrap();

// 上方向に力を印加
ship.book_next_apply_world_force(0.0, 1000.0, 0.0, None);
wait_for_next_tick().await;
let _ = ship.read_last_apply_world_force();

// 物理ティックイベントを待機
let tick_data = ship.pull_physics_ticks().await.unwrap();
```
