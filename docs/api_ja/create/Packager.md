# Packager

**モジュール:** `create`  
**ペリフェラルタイプ:** `create:packager`

Create Packager ペリフェラル。インベントリの内容からパッケージを作成し、アドレス管理、パッケージ送受信イベントの監視を行います。

## Book-Read メソッド

book-read パターンを使用します。`book_next_*` でリクエストを予約し、
`wait_for_next_tick().await` でフラッシュした後、`read_last_*` で結果を取得します。

### `book_next_make_package` / `read_last_make_package`

インベントリの内容からパッケージを作成します。

```rust
pub fn book_next_make_package(&mut self)
pub fn read_last_make_package(&self) -> Result<(), PeripheralError>
```

### `book_next_list` / `read_last_list`

インベントリ内のスロット一覧を取得します。

```rust
pub fn book_next_list(&mut self)
pub fn read_last_list(&self) -> Result<Vec<CRSlotInfo>, PeripheralError>
```

**戻り値:** `Vec<CRSlotInfo>` — アイテム名と個数を持つスロット一覧。

### `book_next_get_item_detail` / `read_last_get_item_detail`

指定スロットのアイテム詳細情報を取得します。

```rust
pub fn book_next_get_item_detail(&mut self, slot: u32)
pub fn read_last_get_item_detail(&self) -> Result<Option<CRItemDetail>, PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `slot` | `u32` | 調査するスロット番号 |

**戻り値:** `Option<CRItemDetail>` — アイテム詳細。スロットが空の場合は `None`。

### `book_next_get_address` / `read_last_get_address`

アドレスを取得します。

```rust
pub fn book_next_get_address(&mut self)
pub fn read_last_get_address(&self) -> Result<String, PeripheralError>
```

**戻り値:** `String` — 現在のアドレス。

### `book_next_set_address` / `read_last_set_address`

アドレスを設定します。

```rust
pub fn book_next_set_address(&mut self, address: &str)
pub fn read_last_set_address(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `address` | `&str` | 設定するアドレス |

### `book_next_get_package` / `read_last_get_package`

現在のパッケージ情報を取得します。

```rust
pub fn book_next_get_package(&mut self)
pub fn read_last_get_package(&self) -> Result<Option<CRPackage>, PeripheralError>
```

**戻り値:** `Option<CRPackage>` — 現在のパッケージ情報。パッケージがない場合は `None`。

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* / async_* methods

## イベントメソッド

### `book_next_try_pull_package_received` / `read_last_try_pull_package_received`

パッケージ受信イベントを 1tick 待機して取得します（ノンブロッキング）。イベントがなければ `None`。

```rust
pub fn book_next_try_pull_package_received(&mut self)
pub fn read_last_try_pull_package_received(&self) -> Result<Option<CRPackage>, PeripheralError>
```

### `pull_package_received`

パッケージ受信イベントを受信するまで待機します（ブロッキング非同期ループ）。

```rust
pub async fn pull_package_received(&self) -> Result<CRPackage, PeripheralError>
```

### `book_next_try_pull_package_sent` / `read_last_try_pull_package_sent`

パッケージ送信イベントを 1tick 待機して取得します（ノンブロッキング）。イベントがなければ `None`。

```rust
pub fn book_next_try_pull_package_sent(&mut self)
pub fn read_last_try_pull_package_sent(&self) -> Result<Option<CRPackage>, PeripheralError>
```

### `pull_package_sent`

パッケージ送信イベントを受信するまで待機します（ブロッキング非同期ループ）。

```rust
pub async fn pull_package_sent(&self) -> Result<CRPackage, PeripheralError>
```

## 型定義

### `CRPackage`

```rust
pub struct CRPackage {
    pub address: String,
}
```

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

## 使用例

```rust
use rust_computers_api::create::packager::Packager;
use rust_computers_api::peripheral::Peripheral;

let mut packager = Packager::wrap(addr);

// アドレスを設定してパッケージを作成
packager.book_next_set_address("factory");
wait_for_next_tick().await;
packager.read_last_set_address()?;

packager.book_next_make_package();
wait_for_next_tick().await;
packager.read_last_make_package()?;

// パッケージ受信を待機
let package = packager.pull_package_received().await?;
```
