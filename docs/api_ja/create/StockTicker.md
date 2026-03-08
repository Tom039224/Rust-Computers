```markdown
# StockTicker

**モジュール:** `create`  
**ペリフェラルタイプ:** `create:stock_ticker`

Create Stock Ticker ペリフェラル。在庫の監視、アイテム詳細の取得、フィルタ付きリクエストをサポートします。

## Book-Read メソッド

book-read パターンを使用します。`book_next_*` でリクエストを予約し、
`wait_for_next_tick().await` でフラッシュした後、`read_last_*` で結果を取得します。

### `book_next_stock` / `read_last_stock`

在庫情報を取得します。

```rust
pub fn book_next_stock(&mut self)
pub fn read_last_stock(&self) -> Result<Vec<CRSlotInfo>, PeripheralError>
```

**戻り値:** `Vec<CRSlotInfo>` — 在庫スロット情報のリスト。

### `book_next_get_stock_item_detail` / `read_last_get_stock_item_detail`

指定スロットの在庫アイテム詳細情報を取得します。

```rust
pub fn book_next_get_stock_item_detail(&mut self, slot: u32)
pub fn read_last_get_stock_item_detail(&self) -> Result<Option<CRItemDetail>, PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `slot` | `u32` | スロットインデックス |

**戻り値:** `Option<CRItemDetail>` — アイテム詳細。スロットが空の場合は `None`。

### `book_next_request_filtered` / `read_last_request_filtered`

フィルタ付きアイテムリクエストを送信します。

```rust
pub fn book_next_request_filtered(&mut self, filters: &[CRItemFilter]) -> Result<(), PeripheralError>
pub fn read_last_request_filtered(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `filters` | `&[CRItemFilter]` | アイテムフィルタの配列 |

### `book_next_list` / `read_last_list`

インベントリ内の全スロット情報を一覧取得します。

```rust
pub fn book_next_list(&mut self)
pub fn read_last_list(&self) -> Result<Vec<CRSlotInfo>, PeripheralError>
```

**戻り値:** `Vec<CRSlotInfo>` — スロット情報のリスト。

### `book_next_get_item_detail` / `read_last_get_item_detail`

指定スロットのアイテム詳細情報を取得します。

```rust
pub fn book_next_get_item_detail(&mut self, slot: u32)
pub fn read_last_get_item_detail(&self) -> Result<Option<CRItemDetail>, PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `slot` | `u32` | スロットインデックス |

**戻り値:** `Option<CRItemDetail>` — アイテム詳細。スロットが空の場合は `None`。

## 型定義

### `CRSlotInfo`

```rust
pub struct CRSlotInfo {
    pub name: String,
    pub count: u32,
}
```

### `CRItemDetail`

```rust
pub struct CRItemDetail {
    pub name: String,
    pub count: u32,
    pub display_name: String,
    pub tags: BTreeMap<String, bool>,
}
```

### `CRItemFilter`

```rust
pub struct CRItemFilter {
    pub name: Option<String>,
    pub request_count: Option<u32>,
}
```

## 使用例

```rust
use rust_computers_api::create::stock_ticker::StockTicker;
use rust_computers_api::peripheral::Peripheral;

let mut ticker = StockTicker::wrap(addr);

// 在庫リストを取得
ticker.book_next_stock();
wait_for_next_tick().await;
let stock = ticker.read_last_stock()?;

// スロット 0 の詳細を取得
ticker.book_next_get_stock_item_detail(0);
wait_for_next_tick().await;
let detail = ticker.read_last_get_stock_item_detail()?;

// インベントリ一覧を取得
ticker.book_next_list();
wait_for_next_tick().await;
let items = ticker.read_last_list()?;
```

```
