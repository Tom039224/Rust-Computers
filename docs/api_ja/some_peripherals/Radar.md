# Radar

**モジュール:** `some_peripherals`  
**ペリフェラルタイプ:** `sp_radar` (Peripheral::NAME)

Some-Peripherals Mod の Radar ペリフェラル。指定した半径内のエンティティ、プレイヤー、Valkyrien Skies シップをスキャンします。

## Book-Read メソッド

### `book_next_scan_for_entities` / `read_last_scan_for_entities`

指定した半径内のエンティティをスキャンします。

```rust
pub fn book_next_scan_for_entities(&mut self, radius: f64) { ... }
pub fn read_last_scan_for_entities(&self) -> Result<Vec<SPEntityInfo>, PeripheralError> { ... }
```

**パラメータ:**
- `radius`: スキャン半径（ブロック単位）

**戻り値:** `Result<Vec<SPEntityInfo>, PeripheralError>` — 検出されたエンティティのリスト

---

### `book_next_scan_for_ships` / `read_last_scan_for_ships`

指定した半径内の Valkyrien Skies シップをスキャンします。

```rust
pub fn book_next_scan_for_ships(&mut self, radius: f64) { ... }
pub fn read_last_scan_for_ships(&self) -> Result<Vec<SPShipInfo>, PeripheralError> { ... }
```

**パラメータ:**
- `radius`: スキャン半径（ブロック単位）

**戻り値:** `Result<Vec<SPShipInfo>, PeripheralError>` — 検出されたシップのリスト

---

### `book_next_scan_for_players` / `read_last_scan_for_players`

指定した半径内のプレイヤーをスキャンします。

```rust
pub fn book_next_scan_for_players(&mut self, radius: f64) { ... }
pub fn read_last_scan_for_players(&self) -> Result<Vec<SPEntityInfo>, PeripheralError> { ... }
```

**パラメータ:**
- `radius`: スキャン半径（ブロック単位）

**戻り値:** `Result<Vec<SPEntityInfo>, PeripheralError>` — 検出されたプレイヤーのリスト

---

### `book_next_get_config_info` / `read_last_get_config_info`

Radar の設定情報を取得します。

```rust
pub fn book_next_get_config_info(&mut self) { ... }
pub fn read_last_get_config_info(&self) -> Result<BTreeMap<String, String>, PeripheralError> { ... }
```

**パラメータ:** なし

**戻り値:** `Result<BTreeMap<String, String>, PeripheralError>` — 設定のキー・値ペア

## Immediate メソッド

### `get_config_info_imm`

次のティックを待たずに、即座に Radar の設定情報を取得します。

```rust
pub fn get_config_info_imm(&self) -> Result<BTreeMap<String, String>, PeripheralError> { ... }
```

**戻り値:** `Result<BTreeMap<String, String>, PeripheralError>`

## 型定義

### `SPEntityInfo`

エンティティ/プレイヤースキャンで返されるエンティティ情報。

| フィールド     | 型       | 説明                                              |
|---------------|----------|---------------------------------------------------|
| `x`           | `f64`    | エンティティの X 座標                               |
| `y`           | `f64`    | エンティティの Y 座標                               |
| `z`           | `f64`    | エンティティの Z 座標                               |
| `id`          | `String` | エンティティ登録 ID（例: `"minecraft:skeleton"`）    |
| `entity_type` | `String` | エンティティタイプ文字列（Lua キー: `"type"`）       |
| `name`        | `String` | エンティティの表示名                                |

### `SPShipInfo`

シップスキャンで返されるシップ情報。

| フィールド                  | 型                | 説明                                                    |
|----------------------------|-------------------|---------------------------------------------------------|
| `is_ship`                  | `bool`            | シップかどうか                                           |
| `ship_id`                  | `i64`             | シップ ID（Lua キー: `"id"`）                            |
| `pos`                      | `SPCoordinate`    | シップの位置                                             |
| `mass`                     | `f64`             | シップの質量                                             |
| `rotation`                 | `VSQuaternion`    | シップの回転クォータニオン                                |
| `velocity`                 | `SPCoordinate`    | シップの速度ベクトル                                      |
| `size`                     | `SPCoordinate`    | シップのバウンディングボックスサイズ                       |
| `scale`                    | `SPCoordinate`    | シップのスケール                                         |
| `moment_of_inertia_tensor` | `[[f64; 3]; 3]`   | 3×3 慣性モーメントテンソル                                |
| `center_of_mass_in_ship`   | `SPCoordinate`    | シップ座標系での重心位置（Lua キー: `"center_of_mass_in_a_ship"`） |

### `SPCoordinate`

3D 座標（`ballistic_accelerator` モジュールで定義）。

| フィールド | 型    | 説明     |
|-----------|-------|----------|
| `x`       | `f64` | X 座標   |
| `y`       | `f64` | Y 座標   |
| `z`       | `f64` | Z 座標   |

## 使用例

```rust
// 半径64ブロック以内のエンティティをスキャン
peripheral.book_next_scan_for_entities(64.0);
wait_for_next_tick().await;
let entities = peripheral.read_last_scan_for_entities()?;

for entity in &entities {
    log!("{} を ({}, {}, {}) で検出", entity.name, entity.x, entity.y, entity.z);
}

// シップをスキャン
peripheral.book_next_scan_for_ships(128.0);
wait_for_next_tick().await;
let ships = peripheral.read_last_scan_for_ships()?;

// 設定情報を即座に取得
let config = peripheral.get_config_info_imm()?;
```
