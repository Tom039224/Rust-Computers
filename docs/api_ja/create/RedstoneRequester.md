# RedstoneRequester

**モジュール:** `create`  
**ペリフェラルタイプ:** `create:redstone_requester`

Create Redstone Requester ペリフェラル。アイテムリクエストの送信、リクエストスロットの管理、クラフティングリクエスト、アドレス・構成情報の管理を行います。

## Book-Read メソッド

book-read パターンを使用します。`book_next_*` でリクエストを予約し、
`wait_for_next_tick().await` でフラッシュした後、`read_last_*` で結果を取得します。

### `book_next_request` / `read_last_request`

アイテムの一括リクエストを送信します。

```rust
pub fn book_next_request(&mut self, items: &[CROrderItem]) -> Result<(), PeripheralError>
pub fn read_last_request(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `items` | `&[CROrderItem]` | リクエストするアイテムの配列 |

### `book_next_set_request` / `read_last_set_request`

指定スロットにリクエストを設定します。

```rust
pub fn book_next_set_request(&mut self, slot: u32, item: &CROrderItem) -> Result<(), PeripheralError>
pub fn read_last_set_request(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `slot` | `u32` | 設定するスロット番号 |
| `item` | `&CROrderItem` | リクエストするアイテムと個数 |

### `book_next_set_crafting_request` / `read_last_set_crafting_request`

指定スロットにクラフティングリクエストを設定します。

```rust
pub fn book_next_set_crafting_request(&mut self, slot: u32, item: &CROrderItem) -> Result<(), PeripheralError>
pub fn read_last_set_crafting_request(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `slot` | `u32` | 設定するスロット番号 |
| `item` | `&CROrderItem` | クラフティングリクエストするアイテムと個数 |

### `book_next_get_request` / `read_last_get_request`

指定スロットのリクエスト情報を取得します。

```rust
pub fn book_next_get_request(&mut self, slot: u32)
pub fn read_last_get_request(&self) -> Result<Option<CRItemFilter>, PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `slot` | `u32` | 取得するスロット番号 |

**戻り値:** `Option<CRItemFilter>` — リクエストフィルタ情報。スロットが空の場合は `None`。

### `book_next_get_configuration` / `read_last_get_configuration`

構成情報（パッケージルーティング情報）を取得します。

```rust
pub fn book_next_get_configuration(&mut self)
pub fn read_last_get_configuration(&self) -> Result<CRPackage, PeripheralError>
```

**戻り値:** `CRPackage` — 現在の構成情報。

### `book_next_set_configuration` / `read_last_set_configuration`

構成情報を設定します。

```rust
pub fn book_next_set_configuration(&mut self, config: &CRPackage) -> Result<(), PeripheralError>
pub fn read_last_set_configuration(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `config` | `&CRPackage` | 設定する構成情報 |

### `book_next_set_address` / `read_last_set_address`

アドレスを設定します。

```rust
pub fn book_next_set_address(&mut self, address: &str)
pub fn read_last_set_address(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `address` | `&str` | 設定するアドレス |

### `book_next_get_address` / `read_last_get_address`

アドレスを取得します。

```rust
pub fn book_next_get_address(&mut self)
pub fn read_last_get_address(&self) -> Result<String, PeripheralError>
```

**戻り値:** `String` — 現在のアドレス。

## 型定義

### `CROrderItem`

```rust
pub struct CROrderItem {
    pub name: String,
    pub count: u32,
}
```

### `CRItemFilter`

```rust
pub struct CRItemFilter {
    pub name: Option<String>,
    pub request_count: Option<u32>,
}
```

### `CRPackage`

```rust
pub struct CRPackage {
    pub address: String,
}
```

## 使用例

```rust
use rust_computers_api::create::redstone_requester::RedstoneRequester;
use rust_computers_api::create::common::CROrderItem;
use rust_computers_api::peripheral::Peripheral;

let mut requester = RedstoneRequester::wrap(addr);

// アドレスを設定
requester.book_next_set_address("storage");
wait_for_next_tick().await;
requester.read_last_set_address()?;

// スロット 0 にリクエストを設定
let item = CROrderItem { name: "minecraft:iron_ingot".into(), count: 64 };
requester.book_next_set_request(0, &item)?;
wait_for_next_tick().await;
requester.read_last_set_request()?;

// 一括リクエストを送信
let items = vec![
    CROrderItem { name: "minecraft:iron_ingot".into(), count: 64 },
    CROrderItem { name: "minecraft:gold_ingot".into(), count: 32 },
];
requester.book_next_request(&items)?;
wait_for_next_tick().await;
requester.read_last_request()?;
```
