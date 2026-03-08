# Raycaster

**モジュール:** `some_peripherals`  
**ペリフェラルタイプ:** `sp:raycaster` (Peripheral::NAME)

Some-Peripherals Mod の Raycaster ペリフェラル。ワールド内でレイキャストを実行し、ブロック、エンティティ、Valkyrien Skies シップのヒット情報を提供します。

## Book-Read メソッド

### `book_next_raycast` / `read_last_raycast`

ワールド内でレイキャストを実行します。

```rust
pub fn book_next_raycast(
    &mut self,
    distance: f64,
    variables: Option<(f64, f64, Option<f64>)>,
    euler_mode: Option<bool>,
    im_execute: Option<bool>,
    check_for_blocks: Option<bool>,
    only_distance: Option<bool>,
) { ... }
pub fn read_last_raycast(&self) -> Result<SPRaycastResult, PeripheralError> { ... }
```

**パラメータ:**
- `distance`: 最大レイキャスト距離
- `variables` *(省略可)*: 方向角度タプル `(a, b, c)` — `euler_mode` によって解釈が異なる
- `euler_mode` *(省略可)*: 方向変数をオイラー角として解釈するかどうか
- `im_execute` *(省略可)*: 即座に実行するかどうか
- `check_for_blocks` *(省略可)*: ブロックヒットをチェックするかどうか
- `only_distance` *(省略可)*: 距離のみを返すかどうか（パフォーマンス向上のため）

**戻り値:** `Result<SPRaycastResult, PeripheralError>`

---

### `book_next_add_stickers` / `read_last_add_stickers`

ステッカーの powered 状態を設定します。

```rust
pub fn book_next_add_stickers(&mut self, state: bool) { ... }
pub fn read_last_add_stickers(&self) -> Result<(), PeripheralError> { ... }
```

**パラメータ:**
- `state`: ステッカーを powered にするか (`true`) unpowered にするか (`false`)

**戻り値:** `Result<(), PeripheralError>`

> **注意:** これは book-action メソッド（`book_action`）であり、ワールドに副作用を及ぼします。

---

### `book_next_get_config_info` / `read_last_get_config_info`

Raycaster の設定情報を取得します。

```rust
pub fn book_next_get_config_info(&mut self) { ... }
pub fn read_last_get_config_info(&self) -> Result<BTreeMap<String, String>, PeripheralError> { ... }
```

**パラメータ:** なし

**戻り値:** `Result<BTreeMap<String, String>, PeripheralError>` — 設定のキー・値ペア

---

### `book_next_get_facing_direction` / `read_last_get_facing_direction`

Raycaster ブロックの向きを取得します。

```rust
pub fn book_next_get_facing_direction(&mut self) { ... }
pub fn read_last_get_facing_direction(&self) -> Result<String, PeripheralError> { ... }
```

**パラメータ:** なし

**戻り値:** `Result<String, PeripheralError>` — 方向文字列（例: `"north"`、`"up"`）

## Immediate メソッド

### `get_config_info_imm`

次のティックを待たずに、即座に Raycaster の設定情報を取得します。

```rust
pub fn get_config_info_imm(&self) -> Result<BTreeMap<String, String>, PeripheralError> { ... }
```

**戻り値:** `Result<BTreeMap<String, String>, PeripheralError>`

### `get_facing_direction_imm`

次のティックを待たずに、即座にブロックの向きを取得します。

```rust
pub fn get_facing_direction_imm(&self) -> Result<String, PeripheralError> { ... }
```

**戻り値:** `Result<String, PeripheralError>`

## 型定義

### `SPRaycastResult`

レイキャストヒット結果。ヒットタイプによって異なるフィールドが設定されるため、すべてのフィールドはオプションです。

| フィールド         | 型                     | 説明                                         |
|-------------------|------------------------|----------------------------------------------|
| `is_block`        | `Option<bool>`         | ブロックにヒットしたかどうか                    |
| `is_entity`       | `Option<bool>`         | エンティティにヒットしたかどうか                 |
| `abs_pos`         | `Option<SPCoordinate>` | ヒットの絶対位置                               |
| `hit_pos`         | `Option<SPCoordinate>` | ヒット位置                                     |
| `distance`        | `Option<f64>`          | ヒットまでの距離                               |
| `block_type`      | `Option<String>`       | ブロックタイプ ID（ブロックヒットの場合）         |
| `rel_hit_pos`     | `Option<SPCoordinate>` | ブロック内の相対ヒット位置                      |
| `id`              | `Option<String>`       | エンティティ ID（エンティティヒットの場合）       |
| `description_id`  | `Option<String>`       | エンティティの説明 ID                           |
| `ship_id`         | `Option<i64>`          | VS シップ ID（シップヒットの場合）               |
| `hit_pos_ship`    | `Option<SPCoordinate>` | シップ座標系でのヒット位置                       |
| `error`           | `Option<String>`       | エラーメッセージ（レイキャスト失敗時）            |

### `SPCoordinate`

3D 座標（`ballistic_accelerator` モジュールで定義）。

| フィールド | 型    | 説明    |
|-----------|-------|---------|
| `x`       | `f64` | X 座標  |
| `y`       | `f64` | Y 座標  |
| `z`       | `f64` | Z 座標  |

## 使用例

```rust
// 単純なレイキャスト — 前方64ブロック
peripheral.book_next_raycast(64.0, None, None, None, None, None);
wait_for_next_tick().await;
let result = peripheral.read_last_raycast()?;

if let Some(true) = result.is_block {
    log!("ブロックにヒット: {:?} 距離: {:?}", result.block_type, result.distance);
}

// 角度を指定したレイキャスト
peripheral.book_next_raycast(
    128.0,
    Some((45.0, 0.0, None)),  // 方向角度
    Some(true),                 // オイラーモード
    None, None, None,
);
wait_for_next_tick().await;
let result = peripheral.read_last_raycast()?;

// 向きを即座に取得
let direction = peripheral.get_facing_direction_imm()?;
log!("向き: {}", direction);
```
