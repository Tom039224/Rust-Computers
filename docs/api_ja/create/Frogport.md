# Frogport

**モジュール:** `create`  
**ペリフェラルタイプ:** `create:frogport`

Create Frogport ペリフェラル。アドレス管理、構成設定、インベントリ操作、パッケージ送受信イベントを扱います。すべてのメソッドは mainThread=true のため imm バリアントはありません。

## Book-Read メソッド

book-read パターンを使用します。`book_next_*` でリクエストを予約し、
`wait_for_next_tick().await` でフラッシュした後、`read_last_*` で結果を取得します。

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

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


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
use rust_computers_api::create::frogport::Frogport;
use rust_computers_api::peripheral::Peripheral;

let mut frogport = Frogport::wrap(addr);

// アドレスを設定
frogport.book_next_set_address("warehouse");
wait_for_next_tick().await;
frogport.read_last_set_address()?;

// インベントリ一覧を取得
frogport.book_next_list();
wait_for_next_tick().await;
let slots = frogport.read_last_list()?;

// パッケージ受信を待機
let package = frogport.pull_package_received().await?;
```
