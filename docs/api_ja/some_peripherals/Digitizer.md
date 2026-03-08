# Digitizer

**モジュール:** `some_peripherals`  
**ペリフェラルタイプ:** `sp:digitizer` (Peripheral::NAME)

Some-Peripherals Mod の Digitizer ペリフェラル。物理アイテムをデジタル UUID に変換したり戻したりして、デジタル化されたアイテムスタックを操作できます。

## Book-Read メソッド

### `book_next_digitize_amount` / `read_last_digitize_amount`

スロット 0 のアイテムをデジタル化し、デジタルアイテムを表す UUID を返します。

```rust
pub fn book_next_digitize_amount(&mut self, amount: Option<u32>) { ... }
pub fn read_last_digitize_amount(&self) -> Result<String, PeripheralError> { ... }
```

**パラメータ:**
- `amount`: デジタル化するアイテム数。`None` でスタック全体をデジタル化。

**戻り値:** `Result<String, PeripheralError>` — デジタル化されたアイテムの UUID

> **注意:** これは book-action メソッド（`book_action`）であり、ワールドに副作用を及ぼします。

---

### `book_next_rematerialize_amount` / `read_last_rematerialize_amount`

デジタルアイテムを物理化してスロット 0 に戻します。

```rust
pub fn book_next_rematerialize_amount(&mut self, uuid: &str, amount: Option<u32>) { ... }
pub fn read_last_rematerialize_amount(&self) -> Result<bool, PeripheralError> { ... }
```

**パラメータ:**
- `uuid`: 物理化するデジタルアイテムの UUID
- `amount`: 物理化するアイテム数。`None` でスタック全体を物理化。

**戻り値:** `Result<bool, PeripheralError>` — 操作が成功したかどうか

> **注意:** これは book-action メソッド（`book_action`）です。

---

### `book_next_merge_digital_items` / `read_last_merge_digital_items`

2つのデジタルアイテムスタックを1つに合成します。

```rust
pub fn book_next_merge_digital_items(
    &mut self,
    into_uuid: &str,
    from_uuid: &str,
    amount: Option<u32>,
) { ... }
pub fn read_last_merge_digital_items(&self) -> Result<bool, PeripheralError> { ... }
```

**パラメータ:**
- `into_uuid`: 合成先スタックの UUID
- `from_uuid`: 合成元スタックの UUID
- `amount`: 合成するアイテム数。`None` で全て合成。

**戻り値:** `Result<bool, PeripheralError>` — 操作が成功したかどうか

> **注意:** これは book-action メソッド（`book_action`）です。

---

### `book_next_separate_digital_item` / `read_last_separate_digital_item`

デジタルアイテムスタックを分割し、指定数の新しいスタックを作成します。

```rust
pub fn book_next_separate_digital_item(&mut self, from_uuid: &str, amount: u32) { ... }
pub fn read_last_separate_digital_item(&self) -> Result<String, PeripheralError> { ... }
```

**パラメータ:**
- `from_uuid`: 分割元スタックの UUID
- `amount`: 新しいスタックに分離するアイテム数

**戻り値:** `Result<String, PeripheralError>` — 新しく作成されたスタックの UUID

> **注意:** これは book-action メソッド（`book_action`）です。

---

### `book_next_check_id` / `read_last_check_id`

UUID が存在するか確認し、関連するアイテムデータを返します。

```rust
pub fn book_next_check_id(&mut self, uuid: &str) { ... }
pub fn read_last_check_id(&self) -> Result<SPItemData, PeripheralError> { ... }
```

**パラメータ:**
- `uuid`: 確認する UUID

**戻り値:** `Result<SPItemData, PeripheralError>` — UUID に対応するアイテムデータ

---

### `book_next_get_item_in_slot` / `read_last_get_item_in_slot`

スロット 0 にあるアイテムの情報を取得します。

```rust
pub fn book_next_get_item_in_slot(&mut self) { ... }
pub fn read_last_get_item_in_slot(&self) -> Result<SPItemData, PeripheralError> { ... }
```

**パラメータ:** なし

**戻り値:** `Result<SPItemData, PeripheralError>` — アイテム情報

---

### `book_next_get_item_limit_in_slot` / `read_last_get_item_limit_in_slot`

スロット 0 のアイテム最大スタック数を取得します。

```rust
pub fn book_next_get_item_limit_in_slot(&mut self) { ... }
pub fn read_last_get_item_limit_in_slot(&self) -> Result<u32, PeripheralError> { ... }
```

**パラメータ:** なし

**戻り値:** `Result<u32, PeripheralError>` — 最大スタック数

## 型定義

### `SPItemData`

インベントリクエリで返されるアイテムデータ。

| フィールド | 型               | 説明                                            |
|-----------|------------------|------------------------------------------------|
| `id`      | `String`         | アイテム登録 ID（例: `"minecraft:diamond"`）      |
| `count`   | `u32`            | スタック内のアイテム数                            |
| `tag`     | `msgpack::Value` | NBT/タグデータ（存在しない場合はデフォルト: 空）    |

### `SPDigitizedItem`

デジタル化されたアイテムの参照。

| フィールド | 型       | 説明                         |
|-----------|----------|------------------------------|
| `uuid`    | `String` | デジタル化されたアイテムの UUID |

## 使用例

```rust
// スロット0の全アイテムをデジタル化
peripheral.book_next_digitize_amount(None);
wait_for_next_tick().await;
let uuid = peripheral.read_last_digitize_amount()?;
log!("デジタル化アイテム UUID: {}", uuid);

// デジタルアイテムを確認
peripheral.book_next_check_id(&uuid);
wait_for_next_tick().await;
let item = peripheral.read_last_check_id()?;
log!("アイテム: {} x{}", item.id, item.count);

// スロット0に物理化して戻す
peripheral.book_next_rematerialize_amount(&uuid, None);
wait_for_next_tick().await;
let success = peripheral.read_last_rematerialize_amount()?;
```
