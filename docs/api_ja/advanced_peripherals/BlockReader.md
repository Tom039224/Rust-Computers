# BlockReader

**モジュール:** AdvancedPeripherals  
**ペリフェラルタイプ:** `advancedPeripherals:block_reader`  
**ソース:** `BlockReaderPeripheral.java`

## 概要

BlockReaderペリフェラルは、ペリフェラルの正面にあるブロックの詳細情報を読み取ります。ブロックのレジストリ名、NBTデータ、ブロックステートプロパティ、およびタイルエンティティ（ブロックエンティティ）であるかどうかが含まれます。これは、特定のブロックを識別して反応する必要があるオートメーションシステムに役立ちます。

## 3つの関数パターン

BlockReader APIは、すべてのメソッドに対して3つの関数パターンを使用します：

1. **`book_next_*`** - 次のティックのリクエストをスケジュール
2. **`read_last_*`** - 前のティックの結果を読み取り
3. **`async_*`** - 便利なメソッド（book、待機、読み取りを1つの呼び出しで実行）

### パターンの説明

```rust
// Rust example to be added
```
## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## メソッド

### `getBlockName()` / `book_next_get_block_name()` / `read_last_get_block_name()` / `async_get_block_name()`

ペリフェラルの正面にあるブロックのレジストリ名を取得します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_get_block_name(&mut self)
pub fn read_last_get_block_name(&self) -> Result<String, PeripheralError>
pub async fn async_get_block_name(&self) -> Result<String, PeripheralError>
```

**戻り値:** `string` — ブロックレジストリ名（例：`"minecraft:stone"`、`"minecraft:chest"`）

**例:**
```rust
// Rust example to be added
```
---

### `getBlockData()` / `book_next_get_block_data()` / `read_last_get_block_data()` / `async_get_block_data()`

ペリフェラルの正面にあるブロックのNBTデータを取得します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_get_block_data(&mut self)
pub fn read_last_get_block_data(&self) -> Result<Value, PeripheralError>
pub async fn async_get_block_data(&self) -> Result<Value, PeripheralError>
```

**戻り値:** `table` — 動的テーブルとしてのNBTデータ

戻り値はブロックタイプによって異なります。例えば：
- チェスト: `{Items = {...}, id = "minecraft:chest"}`
- かまど: `{Items = {...}, BurnTime = 0, CookTime = 0, ...}`
- カスタムブロックは追加のプロパティを持つ場合があります

**例:**
```rust
// Rust example to be added
```
---

### `getBlockStates()` / `book_next_get_block_states()` / `read_last_get_block_states()` / `async_get_block_states()`

ペリフェラルの正面にあるブロックのブロックステートプロパティを取得します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_get_block_states(&mut self)
pub fn read_last_get_block_states(&self) -> Result<Value, PeripheralError>
pub async fn async_get_block_states(&self) -> Result<Value, PeripheralError>
```

**戻り値:** `table` — ブロックステートプロパティ

ブロックステートはブロックタイプによって異なります。一般的な例：
- ログ: `{axis = "y", natural = true}`
- 階段: `{facing = "north", half = "bottom", shape = "straight"}`
- レッドストーン: `{north = "up", south = "side", east = "none", west = "up", power = 15}`
- ドア: `{facing = "north", half = "lower", hinge = "left", open = false, powered = false}`

**例:**
```rust
// Rust example to be added
```
---

### `isTileEntity()` / `book_next_is_tile_entity()` / `read_last_is_tile_entity()` / `async_is_tile_entity()`

ペリフェラルの正面にあるブロックがタイルエンティティ（ブロックエンティティ）であるかどうかを確認します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_is_tile_entity(&mut self)
pub fn read_last_is_tile_entity(&self) -> Result<bool, PeripheralError>
pub async fn async_is_tile_entity(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `boolean` — ブロックがタイルエンティティの場合は`true`、そうでない場合は`false`

タイルエンティティには以下が含まれます：
- チェスト、樽、ホッパー
- かまど、溶鉱炉、燻製器
- 醸造スタンド、大釜
- エンチャントテーブル、金床
- ビーコン、コンジット
- タイルエンティティを持つカスタムmodブロック

**例:**
```rust
// Rust example to be added
```
---

## イベント

BlockReaderペリフェラルはイベントを生成しません。

---

## 使用例

### 例1: ブロックタイプの識別

```rust
// Rust example to be added
```
### 例2: チェストの内容をチェック

```rust
// Rust example to be added
```
### 例3: ブロックステート変化の監視

```rust
// Rust example to be added
```
### 例4: レッドストーン信号の検出

```rust
// Rust example to be added
```
### 例5: インベントリ監視

```rust
// Rust example to be added
```
---

## エラーハンドリング

すべてのメソッドは以下の場合にエラーをスローする可能性があります：

- **正面にブロックがない**: ペリフェラルがブロックに面していない（例：空気に面している）
- **ペリフェラル切断**: ペリフェラルにアクセスできなくなった
- **無効なステート**: ブロックステートを読み取ることができない

**エラーハンドリングの例:**
```rust
// Rust example to be added
```
---

## 型定義

### BlockState
```rust
// Rust example to be added
```
### NBTData
```rust
// Rust example to be added
```
---

## 注記

- BlockReaderはそれが向いている方向の正面のブロックを読み取ります
- ブロックステートは常に文字列または数値です（ブール値のような値でも）
- NBTデータ構造はブロックタイプに依存します
- タイルエンティティはシンプルなブロックを超えた追加のNBTデータを持ちます
- 3つの関数パターンは効率的なバッチ操作を可能にします

---

## 関連

- [GeoScanner](./GeoScanner.md) — 領域内の複数ブロックをスキャン
- [AdvancedPeripherals ドキュメント](https://advancedperipherals.readthedocs.io/) — 公式ドキュメント
