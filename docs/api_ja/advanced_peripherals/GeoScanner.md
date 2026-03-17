# GeoScanner

**モジュール:** AdvancedPeripherals  
**ペリフェラルタイプ:** `advancedPeripherals:geo_scanner`  
**ソース:** `GeoScannerPeripheral.java`

## 概要

GeoScannerペリフェラルは、指定された半径内の周囲のブロックをスキャンし、現在のチャンク内の鉱石分布を分析します。ブロックタイプ、その位置、タグに関する詳細情報を提供します。これはマイニングオートメーション、リソース位置特定、地形分析システムに役立ちます。

## 3つの関数パターン

GeoScanner APIは、すべてのメソッドに対して3つの関数パターンを使用します：

1. **`book_next_*`** - 次のティックのリクエストをスケジュール
2. **`read_last_*`** - 前のティックの結果を読み取り
3. **`async_*`** - 便利なメソッド（book、待機、読み取りを1つの呼び出しで実行）

### パターンの説明

```rust
// Rust example to be added
```
## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* / async_* methods

## メソッド

### `cost(radius)` / `book_next_cost(radius)` / `read_last_cost()` / `cost_imm(radius)`

指定された半径でのスキャンの燃料コストを取得します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_cost(&mut self, radius: f64)
pub fn read_last_cost(&self) -> Result<f64, PeripheralError>
pub fn cost_imm(&self, radius: f64) -> Result<f64, PeripheralError>
```

**パラメータ:**
- `radius: number` — ブロック単位のスキャン半径

**戻り値:** `number` — 燃料コスト（AEユニットまたは同様）

**注記:** `cost_imm`メソッドは次のティックを待たずに即座に結果を返します。

**例:**
```rust
// Rust example to be added
```
---

### `scan(radius)` / `book_next_scan(radius)` / `read_last_scan()` / `async_scan(radius)`

指定された半径内のブロックをスキャンします。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_scan(&mut self, radius: f64)
pub fn read_last_scan(&self) -> Result<Vec<GeoBlockEntry>, PeripheralError>
pub async fn async_scan(&self, radius: f64) -> Result<Vec<GeoBlockEntry>, PeripheralError>
```

**パラメータ:**
- `radius: number` — ブロック単位のスキャン半径

**戻り値:** `table` — ブロックエントリの配列

**ブロックエントリ構造:**
```rust
// Rust example to be added
```
**例:**
```rust
// Rust example to be added
```
---

### `chunkAnalyze()` / `book_next_chunk_analyze()` / `read_last_chunk_analyze()` / `async_chunk_analyze()`

現在のチャンク内の鉱石分布を分析します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_chunk_analyze(&mut self)
pub fn read_last_chunk_analyze(&self) -> Result<Value, PeripheralError>
pub async fn async_chunk_analyze(&self) -> Result<Value, PeripheralError>
```

**戻り値:** `table` — 鉱石名から数へのマップ

**戻り値構造:**
```rust
// Rust example to be added
```
**例:**
```rust
// Rust example to be added
```
---

## イベント

GeoScannerペリフェラルはイベントを生成しません。

---

## 使用例

### 例1: 特定の鉱石を検索

```rust
// Rust example to be added
```
### 例2: タグでスキャンしてフィルター

```rust
// Rust example to be added
```
### 例3: チャンク分析レポート

```rust
// Rust example to be added
```
### 例4: コスト認識スキャン

```rust
// Rust example to be added
```
### 例5: マイニングオートメーション

```rust
// Rust example to be added
```
---

## エラーハンドリング

すべてのメソッドは以下の場合にエラーをスローする可能性があります：

- **燃料不足**: スキャンを実行するのに十分なエネルギーがない
- **ペリフェラル切断**: GeoScannerにアクセスできなくなった
- **無効な半径**: 半径が負またはマキシマムを超えている

**エラーハンドリングの例:**
```rust
// Rust example to be added
```
---

## 型定義

### GeoBlockEntry
```rust
// Rust example to be added
```
### ChunkAnalysisResult
```rust
// Rust example to be added
```
---

## 注記

- `scan()`で返される座標はスキャナーの位置からの相対位置です
- `cost_imm()`メソッドは唯一のイミディエイトメソッド（待機なしで返す）です
- チャンク分析はスキャン半径ではなくチャンク全体をスキャンします
- ブロックタグは結果をフィルターするために使用できます（例："minecraft:ores"、"minecraft:mineable/pickaxe"）
- 3つの関数パターンは効率的なバッチ操作を可能にします
- より大きなスキャン半径はより多くの燃料/エネルギーを消費します

---

## 関連

- [BlockReader](./BlockReader.md) — 詳細なブロック情報読取用
- [PlayerDetector](./PlayerDetector.md) — プレイヤー検出用
- [AdvancedPeripherals ドキュメント](https://advancedperipherals.readthedocs.io/) — 公式ドキュメント
