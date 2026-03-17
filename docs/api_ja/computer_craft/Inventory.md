# Inventory

**モジュール:** CC:Tweaked  
**ペリフェラルタイプ:** `inventory`  
**ソース:** `AbstractInventoryMethods.java`

## 概要

Inventoryペリフェラルは、ワイヤードネットワークを通じてブロックインベントリ（チェスト、樽、かまど等）にアクセスできます。インベントリの内容を照会し、詳細なアイテム情報を取得し、接続されたインベントリ間でアイテムを転送できます。

## 3つの関数パターン

Inventory APIは、すべてのメソッドに対して3つの関数パターンを使用します：

1. **`book_next_*`** - 次のティックのリクエストをスケジュール
2. **`read_last_*`** - 前のティックの結果を読み取り
3. **`async_*`** - 便利なメソッド（book、待機、読み取りを1つの呼び出しで実行）

### パターンの説明

```rust
// Rust example to be added
```
## 実装状況

### ✅ 実装済み

- book_next_size / read_last_size / async_size
- book_next_list / read_last_list / async_list
- book_next_get_item_detail / read_last_get_item_detail / async_get_item_detail
- book_next_push_items / read_last_push_items / async_push_items
- book_next_pull_items / read_last_pull_items / async_pull_items

### 🚧 未実装

- get_item_limit() method (all variants)


## メソッド

### `size()` / `book_next_size()` / `read_last_size()` / `async_size()`

インベントリのスロット数を取得します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_size(&mut self)
pub fn read_last_size(&self) -> Result<u32, PeripheralError>
pub async fn async_size(&self) -> Result<u32, PeripheralError>
```

**戻り値:** `number` — スロットの総数

**例:**
```rust
// Rust example to be added
```
---

### `list()` / `book_next_list()` / `read_last_list()` / `async_list()`

インベントリ内のすべてのアイテムを概要情報とともにリストアップします。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_list(&mut self)
pub fn read_last_list(&self) -> Result<BTreeMap<u32, SlotInfo>, PeripheralError>
pub async fn async_list(&self) -> Result<BTreeMap<u32, SlotInfo>, PeripheralError>
```

**戻り値:** `table` — スロット番号をアイテム情報にマッピングするスパーステーブル

**アイテム情報構造:**
```rust
// Rust example to be added
```
**例:**
```rust
// Rust example to be added
```
---

### `getItemDetail(slot)` / `book_next_get_item_detail(slot)` / `read_last_get_item_detail()` / `async_get_item_detail(slot)`

特定のスロット内のアイテムの詳細情報を取得します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_get_item_detail(&mut self, slot: u32)
pub fn read_last_get_item_detail(&self) -> Result<Option<ItemDetail>, PeripheralError>
pub async fn async_get_item_detail(&self, slot: u32) -> Result<Option<ItemDetail>, PeripheralError>
```

**パラメータ:**
- `slot: number` — スロットインデックス（1ベース）

**戻り値:** `table | nil` — アイテム詳細テーブルまたはスロットが空の場合は`nil`

**アイテム詳細構造:**
```rust
// Rust example to be added
```
**例:**
```rust
// Rust example to be added
```
**エラーハンドリング:**
- スロット番号が範囲外の場合、エラーをスロー

---

### `pushItems(toName, fromSlot, limit?, toSlot?)` / `book_next_push_items(...)` / `read_last_push_items()` / `async_push_items(...)`

このインベントリから別の接続されたインベントリにアイテムを転送します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_push_items(&mut self, to_name: &str, from_slot: u32, limit: Option<u32>, to_slot: Option<u32>)
pub fn read_last_push_items(&self) -> Result<u32, PeripheralError>
pub async fn async_push_items(&self, to_name: &str, from_slot: u32, limit: Option<u32>, to_slot: Option<u32>) -> Result<u32, PeripheralError>
```

**パラメータ:**
- `toName: string` — 宛先インベントリの名前（`peripheral.getNamesRemote()`から）
- `fromSlot: number` — ソーススロットインデックス（1ベース）
- `limit?: number` — 転送する最大アイテム数（オプション、デフォルトはスタック制限）
- `toSlot?: number` — 宛先スロットインデックス（オプション、省略時は自動選択）

**戻り値:** `number` — 実際に転送されたアイテム数

**要件:**
- 両方のインベントリはワイヤードモデムとネットワークケーブルで接続されている必要があります
- 宛先インベントリが存在し、アクセス可能である必要があります

**例:**
```rust
// Rust example to be added
```
**エラーハンドリング:**
- 宛先インベントリが存在しない場合、エラーをスロー
- スロットが範囲外の場合、エラーをスロー

---

### `pullItems(fromName, fromSlot, limit?, toSlot?)` / `book_next_pull_items(...)` / `read_last_pull_items()` / `async_pull_items(...)`

別の接続されたインベントリからこのインベントリにアイテムを転送します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_pull_items(&mut self, from_name: &str, from_slot: u32, limit: Option<u32>, to_slot: Option<u32>)
pub fn read_last_pull_items(&self) -> Result<u32, PeripheralError>
pub async fn async_pull_items(&self, from_name: &str, from_slot: u32, limit: Option<u32>, to_slot: Option<u32>) -> Result<u32, PeripheralError>
```

**パラメータ:**
- `fromName: string` — ソースインベントリの名前（`peripheral.getNamesRemote()`から）
- `fromSlot: number` — ソーススロットインデックス（1ベース）
- `limit?: number` — 転送する最大アイテム数（オプション、デフォルトはスタック制限）
- `toSlot?: number` — 宛先スロットインデックス（オプション、省略時は自動選択）

**戻り値:** `number` — 実際に転送されたアイテム数

**要件:**
- 両方のインベントリはワイヤードモデムとネットワークケーブルで接続されている必要があります
- ソースインベントリが存在し、アクセス可能である必要があります

**例:**
```rust
// Rust example to be added
```
**エラーハンドリング:**
- ソースインベントリが存在しない場合、エラーをスロー
- スロットが範囲外の場合、エラーをスロー

---

## イベント

Inventoryペリフェラルはイベントを生成しません。

---

## 使用例

### 例1: すべてのアイテムをリストアップ

```rust
// Rust example to be added
```
### 例2: 名前でアイテムを検索

```rust
// Rust example to be added
```
### 例3: チェスト間でアイテムを転送

```rust
// Rust example to be added
```
### 例4: 総容量を計算

```rust
// Rust example to be added
```
### 例5: インベントリをソート

```rust
// Rust example to be added
```
---

## エラーハンドリング

すべてのメソッドは以下の場合にエラーをスローする可能性があります：

- **インベントリが見つからない**: ペリフェラルが切断されているか、アクセスできない
- **無効なスロット**: スロット番号が範囲外（1 から size）
- **ネットワークエラー**: ワイヤードネットワーク接続が切断されている
- **宛先が見つからない**: ターゲットインベントリが存在しないか、アクセスできない

**エラーハンドリングの例:**
```rust
// Rust example to be added
```
---

## 型定義

### ItemDetail
```rust
// Rust example to be added
```
### SlotInfo
```rust
// Rust example to be added
```
---

## 注記

- すべてのスロットインデックスは1ベース（最初のスロットは0ではなく1）
- `pushItems` と `pullItems` はワイヤードネットワーク接続が必要です
- インベントリ名は `peripheral.getNamesRemote()` でワイヤードモデムから取得します
- 空のスロットは `list()` の結果に含まれません（スパーステーブル）
- 3つの関数パターンは効率的なバッチ操作を可能にします

---

## 関連

- [Modem](./Modem.md) — ワイヤードネットワーク通信に必要
- [CC:Tweaked ドキュメント](https://tweaked.cc/) — 公式ドキュメント
