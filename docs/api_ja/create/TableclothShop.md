```markdown
# TableclothShop

**モジュール:** `create`  
**ペリフェラルタイプ:** `create:tablecloth_shop`

Create Tablecloth Shop ペリフェラル。ショップブロックのアドレス、価格タグ、商品設定を管理します。

## Book-Read メソッド

book-read パターンを使用します。`book_next_*` でリクエストを予約し、
`wait_for_next_tick().await` でフラッシュした後、`read_last_*` で結果を取得します。

### `book_next_is_shop` / `read_last_is_shop`

このブロックがショップとして機能しているかどうかを取得します。

```rust
pub fn book_next_is_shop(&mut self)
pub fn read_last_is_shop(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — ショップとして機能している場合 `true`。

### `book_next_get_address` / `read_last_get_address`

ショップのアドレスを取得します。

```rust
pub fn book_next_get_address(&mut self)
pub fn read_last_get_address(&self) -> Result<String, PeripheralError>
```

**戻り値:** `String` — ショップアドレス。

### `book_next_set_address` / `read_last_set_address`

ショップのアドレスを設定します。

```rust
pub fn book_next_set_address(&mut self, address: &str)
pub fn read_last_set_address(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `address` | `&str` | 新しいショップアドレス |

### `book_next_get_price_tag_item` / `read_last_get_price_tag_item`

価格タグアイテムを取得します。

```rust
pub fn book_next_get_price_tag_item(&mut self)
pub fn read_last_get_price_tag_item(&self) -> Result<Option<CRItemDetail>, PeripheralError>
```

**戻り値:** `Option<CRItemDetail>` — 価格タグアイテムの詳細。設定されていない場合は `None`。

### `book_next_set_price_tag_item` / `read_last_set_price_tag_item`

価格タグアイテムをアイテム名で設定します。

```rust
pub fn book_next_set_price_tag_item(&mut self, item_name: &str)
pub fn read_last_set_price_tag_item(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `item_name` | `&str` | 価格タグのアイテム名 |

### `book_next_get_price_tag_count` / `read_last_get_price_tag_count`

価格タグの数量を取得します。

```rust
pub fn book_next_get_price_tag_count(&mut self)
pub fn read_last_get_price_tag_count(&self) -> Result<u32, PeripheralError>
```

**戻り値:** `u32` — 価格タグの数量。

### `book_next_set_price_tag_count` / `read_last_set_price_tag_count`

価格タグの数量を設定します。

```rust
pub fn book_next_set_price_tag_count(&mut self, count: u32)
pub fn read_last_set_price_tag_count(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `count` | `u32` | 価格タグの数量 |

### `book_next_get_wares` / `read_last_get_wares`

商品情報を取得します。

```rust
pub fn book_next_get_wares(&mut self)
pub fn read_last_get_wares(&self) -> Result<Option<CRItemDetail>, PeripheralError>
```

**戻り値:** `Option<CRItemDetail>` — 商品アイテムの詳細。設定されていない場合は `None`。

### `book_next_set_wares` / `read_last_set_wares`

商品をアイテム名で設定します。

```rust
pub fn book_next_set_wares(&mut self, item_name: &str)
pub fn read_last_set_wares(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `item_name` | `&str` | 商品のアイテム名 |

## 型定義

### `CRItemDetail`

```rust
pub struct CRItemDetail {
    pub name: String,
    pub count: u32,
    pub display_name: String,
    pub tags: BTreeMap<String, bool>,
}
```

## 使用例

```rust
use rust_computers_api::create::tablecloth_shop::TableclothShop;
use rust_computers_api::peripheral::Peripheral;

let mut shop = TableclothShop::wrap(addr);

// ショップとして機能しているか確認
shop.book_next_is_shop();
wait_for_next_tick().await;
let is_shop = shop.read_last_is_shop()?;

// アドレスを設定
shop.book_next_set_address("Market Street 1");
wait_for_next_tick().await;
shop.read_last_set_address()?;

// 価格を設定
shop.book_next_set_price_tag_item("minecraft:diamond");
wait_for_next_tick().await;
shop.read_last_set_price_tag_item()?;

shop.book_next_set_price_tag_count(5);
wait_for_next_tick().await;
shop.read_last_set_price_tag_count()?;

// 商品を取得
shop.book_next_get_wares();
wait_for_next_tick().await;
let wares = shop.read_last_get_wares()?;
```

```
