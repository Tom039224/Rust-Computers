# WorldScanner

**モジュール:** `some_peripherals`  
**ペリフェラルタイプ:** `sp:world_scanner` (Peripheral::NAME)

Some-Peripherals Mod の WorldScanner ペリフェラル。ワールド内の指定座標のブロック情報をスキャンし、Valkyrien Skies の shipyard 座標にも対応しています。

## Book-Read メソッド

### `book_next_get_block_at` / `read_last_get_block_at`

指定座標のブロック情報を取得します。

```rust
pub fn book_next_get_block_at(&mut self, x: i32, y: i32, z: i32, is_shipyard: bool) { ... }
pub fn read_last_get_block_at(&self) -> Result<SPBlockInfo, PeripheralError> { ... }
```

**パラメータ:**
- `x`: X 座標
- `y`: Y 座標
- `z`: Z 座標
- `is_shipyard`: Valkyrien Skies の shipyard 座標かどうか

**戻り値:** `Result<SPBlockInfo, PeripheralError>`

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## 型定義

### `SPBlockInfo`

スキャナーが返すブロック情報。

| フィールド    | 型              | 説明                                          |
|-------------|-----------------|-----------------------------------------------|
| `block_type`| `String`        | ブロック登録 ID（例: `"minecraft:stone"`）       |
| `ship_id`   | `Option<i64>`   | VS シップ ID（ブロックがシップ上にある場合）       |

## 使用例

```rust
// 座標 (10, 64, 20) のブロックを確認
peripheral.book_next_get_block_at(10, 64, 20, false);
wait_for_next_tick().await;
let block = peripheral.read_last_get_block_at()?;
log!("ブロック: {}", block.block_type);

// VS shipyard 座標のブロックを確認
peripheral.book_next_get_block_at(1000, 100, 2000, true);
wait_for_next_tick().await;
let block = peripheral.read_last_get_block_at()?;
if let Some(ship_id) = block.ship_id {
    log!("ブロック {} はシップ {} 上にあります", block.block_type, ship_id);
}
```
