# Camera

**モジュール:** `control_craft::camera`  
**ペリフェラルタイプ:** `controlcraft:camera`

Control-Craft の Camera ペリフェラル。レイキャスト、エンティティ検出、視点制御に使用します。豊富なクリップ/レイキャスト機能とカメラの向き制御を提供しています。

## Book-Read メソッド

### プロパティ取得（imm 対応）

以下のメソッドは全て `book_read_imm!` マクロパターンを使用しており、`book_next_*`、`read_last_*`、`*_imm` の3バリアントを提供します。

| Book メソッド | Read メソッド | Imm メソッド | 戻り値 |
|---|---|---|---|
| `book_next_get_abs_view_transform` | `read_last_get_abs_view_transform` | `get_abs_view_transform_imm` | `CTLTransform` |
| `book_next_get_pitch` | `read_last_get_pitch` | `get_pitch_imm` | `f64` |
| `book_next_get_yaw` | `read_last_get_yaw` | `get_yaw_imm` | `f64` |
| `book_next_get_transformed_pitch` | `read_last_get_transformed_pitch` | `get_transformed_pitch_imm` | `f64` |
| `book_next_get_transformed_yaw` | `read_last_get_transformed_yaw` | `get_transformed_yaw_imm` | `f64` |
| `book_next_get_clip_distance` | `read_last_get_clip_distance` | `get_clip_distance_imm` | `f64` |
| `book_next_latest_ship` | `read_last_latest_ship` | `latest_ship_imm` | `Option<Value>` |
| `book_next_latest_player` | `read_last_latest_player` | `latest_player_imm` | `Option<Value>` |
| `book_next_latest_entity` | `read_last_latest_entity` | `latest_entity_imm` | `Option<Value>` |
| `book_next_latest_block` | `read_last_latest_block` | `latest_block_imm` | `Option<Value>` |
| `book_next_get_camera_position` | `read_last_get_camera_position` | `get_camera_position_imm` | `(f64, f64, f64)` |
| `book_next_get_abs_view_forward` | `read_last_get_abs_view_forward` | `get_abs_view_forward_imm` | `(f64, f64, f64)` |
| `book_next_is_being_used` | `read_last_is_being_used` | `is_being_used_imm` | `bool` |
| `book_next_get_direction` | `read_last_get_direction` | `get_direction_imm` | `String` |

---

### クリップメソッド（レイキャスト）

#### `book_next_clip` / `read_last_clip`
カメラの視線方向に汎用クリップ（レイキャスト）を実行します。
```rust
pub fn book_next_clip(&mut self) { ... }
pub fn read_last_clip(&self) -> Result<CTLRaycastResult, PeripheralError> { ... }
```
**戻り値:** `CTLRaycastResult`

#### `book_next_clip_entity` / `read_last_clip_entity`
エンティティのみを対象にクリップします。
```rust
pub fn book_next_clip_entity(&mut self) { ... }
pub fn read_last_clip_entity(&self) -> Result<CTLRaycastResult, PeripheralError> { ... }
```

#### `book_next_clip_block` / `read_last_clip_block`
ブロックのみを対象にクリップします。
```rust
pub fn book_next_clip_block(&mut self) { ... }
pub fn read_last_clip_block(&self) -> Result<CTLRaycastResult, PeripheralError> { ... }
```

#### `book_next_clip_all_entity` / `read_last_clip_all_entity`
ヒットした全エンティティを返すクリップを実行します。
```rust
pub fn book_next_clip_all_entity(&mut self) { ... }
pub fn read_last_clip_all_entity(&self) -> Result<Vec<CTLRaycastResult>, PeripheralError> { ... }
```

#### `book_next_clip_ship` / `read_last_clip_ship`
船のみを対象にクリップします。
```rust
pub fn book_next_clip_ship(&mut self) { ... }
pub fn read_last_clip_ship(&self) -> Result<CTLRaycastResult, PeripheralError> { ... }
```

#### `book_next_clip_player` / `read_last_clip_player`
プレイヤーのみを対象にクリップします。
```rust
pub fn book_next_clip_player(&mut self) { ... }
pub fn read_last_clip_player(&self) -> Result<CTLRaycastResult, PeripheralError> { ... }
```

---

### カメラ制御

#### `book_next_set_pitch` / `read_last_set_pitch`
カメラのピッチを設定します。
```rust
pub fn book_next_set_pitch(&mut self, degrees: f64) { ... }
pub fn read_last_set_pitch(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `degrees: f64` — ピッチ角（度）

#### `book_next_set_yaw` / `read_last_set_yaw`
カメラのヨーを設定します。
```rust
pub fn book_next_set_yaw(&mut self, degrees: f64) { ... }
pub fn read_last_set_yaw(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `degrees: f64` — ヨー角（度）

#### `book_next_force_pitch_yaw` / `read_last_force_pitch_yaw`
ピッチとヨーを同時に強制設定します。
```rust
pub fn book_next_force_pitch_yaw(&mut self, pitch: f64, yaw: f64) { ... }
pub fn read_last_force_pitch_yaw(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_outline_to_user` / `read_last_outline_to_user`
ユーザーにアウトラインを表示します。
```rust
pub fn book_next_outline_to_user(&mut self) { ... }
pub fn read_last_outline_to_user(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_set_clip_range` / `read_last_set_clip_range`
クリップ（レイキャスト）の範囲を設定します。
```rust
pub fn book_next_set_clip_range(&mut self, range: f64) { ... }
pub fn read_last_set_clip_range(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_set_cone_angle` / `read_last_set_cone_angle`
検出のコーン角度を設定します。
```rust
pub fn book_next_set_cone_angle(&mut self, angle: f64) { ... }
pub fn read_last_set_cone_angle(&self) -> Result<(), PeripheralError> { ... }
```

---

### 高度なクエリ

#### `book_next_raycast` / `read_last_raycast`
指定座標に向かってレイキャストを実行します。
```rust
pub fn book_next_raycast(&mut self, x: f64, y: f64, z: f64) { ... }
pub fn read_last_raycast(&self) -> Result<CTLRaycastResult, PeripheralError> { ... }
```

#### `book_next_get_entities` / `read_last_get_entities`
指定半径内のエンティティを取得します。
```rust
pub fn book_next_get_entities(&mut self, radius: f64) { ... }
pub fn read_last_get_entities(&self) -> Result<Vec<Value>, PeripheralError> { ... }
```

#### `book_next_get_mobs` / `read_last_get_mobs`
指定半径内のモブを取得します。
```rust
pub fn book_next_get_mobs(&mut self, radius: f64) { ... }
pub fn read_last_get_mobs(&self) -> Result<Vec<Value>, PeripheralError> { ... }
```

#### `book_next_reset` / `read_last_reset`
カメラの状態をリセットします。
```rust
pub fn book_next_reset(&mut self) { ... }
pub fn read_last_reset(&self) -> Result<(), PeripheralError> { ... }
```

## 型定義

### `CTLTransform`
4x4 変換行列。
```rust
pub struct CTLTransform { pub matrix: [[f64; 4]; 4] }
```

### `CTLRaycastResult`
レイキャストのヒット結果。
```rust
pub struct CTLRaycastResult {
    pub hit_type: Option<String>,
    pub pos: Option<(f64, f64, f64)>,
    pub block_pos: Option<(i32, i32, i32)>,
    pub entity_id: Option<String>,
    pub entity_type: Option<String>,
    pub ship_id: Option<i64>,
    pub player_name: Option<String>,
    pub distance: Option<f64>,
}
```

## 使用例

```rust
use rust_computers_api::control_craft::camera::*;
use rust_computers_api::peripheral::Peripheral;

let mut camera = Camera::find().unwrap();

// カメラの向きを設定
camera.book_next_force_pitch_yaw(-30.0, 90.0);
wait_for_next_tick().await;
let _ = camera.read_last_force_pitch_yaw();

// クリップを実行
camera.book_next_clip();
wait_for_next_tick().await;
let result = camera.read_last_clip().unwrap();

if let Some(hit_type) = &result.hit_type {
    // ヒット結果を処理
}

// カメラ位置を即時取得
let pos = camera.get_camera_position_imm().unwrap();
```
