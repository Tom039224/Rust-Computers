# EnergyDetector

**モジュール:** AdvancedPeripherals  
**ペリフェラルタイプ:** `advancedPeripherals:energy_detector`  
**ソース:** `EnergyDetectorPeripheral.java`

## 概要

EnergyDetectorペリフェラルは、隣接するブロック間のエネルギー転送レートを監視し、転送レートの制限を設定できます。エネルギー流量のリアルタイム情報を提供し、エネルギーネットワークの過負荷防止や電力消費パターンの監視に使用できます。これは、エネルギー管理システムや電力配電の自動化に役立ちます。

## 3つの関数パターン

EnergyDetector APIは、すべてのメソッドに対して3つの関数パターンを使用します：

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

### `getTransferRate()` / `book_next_get_transfer_rate()` / `read_last_get_transfer_rate()` / `async_get_transfer_rate()`

現在のエネルギー転送レートをFE/t（Forge Energy per tick）で取得します。

**Rust署名:**
```rust
pub fn book_next_get_transfer_rate(&mut self)
pub fn read_last_get_transfer_rate(&self) -> Result<f64, PeripheralError>
pub async fn async_get_transfer_rate(&self) -> Result<f64, PeripheralError>
```

**戻り値:** `number` — 現在の転送レート（FE/t）

**例:**
```rust
// Rust example to be added
```
---

### `getTransferRateLimit()` / `book_next_get_transfer_rate_limit()` / `read_last_get_transfer_rate_limit()` / `async_get_transfer_rate_limit()`

現在の転送レート制限をFE/tで取得します。

**Rust署名:**
```rust
pub fn book_next_get_transfer_rate_limit(&mut self)
pub fn read_last_get_transfer_rate_limit(&self) -> Result<f64, PeripheralError>
pub async fn async_get_transfer_rate_limit(&self) -> Result<f64, PeripheralError>
```

**戻り値:** `number` — 転送レート制限（FE/t）

**例:**
```rust
// Rust example to be added
```
---

### `setTransferRateLimit(rate)` / `book_next_set_transfer_rate_limit(rate)` / `read_last_set_transfer_rate_limit()` / `async_set_transfer_rate_limit(rate)`

最大転送レート制限をFE/tで設定します。

**Rust署名:**
```rust
pub fn book_next_set_transfer_rate_limit(&mut self, rate: f64)
pub fn read_last_set_transfer_rate_limit(&self) -> Result<(), PeripheralError>
pub async fn async_set_transfer_rate_limit(&self, rate: f64) -> Result<(), PeripheralError>
```

**パラメータ:**
- `rate: number` — 最大転送レート（FE/t）

**戻り値:** `boolean` — 制限の設定が成功した場合は`true`

**例:**
```rust
// Rust example to be added
```
---

## イベント

EnergyDetectorペリフェラルはイベントを生成しません。

---

## 使用例

### 例1: エネルギー転送の監視

```rust
// Rust example to be added
```

### 例2: 動的なレート制限

```rust
// Rust example to be added
```

### 例3: エネルギーフロー監視

```rust
// Rust example to be added
```
---

## エラーハンドリング

すべてのメソッドは以下の場合にエラーをスローする可能性があります：

- **ペリフェラル切断**: EnergyDetectorにアクセスできなくなった
- **無効なレート**: 転送レート制限が負または最大値を超えている
- **隣接するエネルギーブロックがない**: 検出器に隣接するエネルギー対応ブロックがない

**エラーハンドリングの例:**
```rust
// Rust example to be added
```
---

## 型定義

このペリフェラルに固有の型定義はありません。

---

## 注記

- 転送レートはFE/t（Forge Energy per tick）で測定されます
- 検出器は隣接するブロック間のエネルギー流量を測定します
- レート制限を設定すると、エネルギーネットワークの過負荷を防止できます
- 3つの関数パターンは効率的なバッチ操作を可能にします
- エネルギー転送はリアルタイムで測定され、各ティックで更新されます

---

## 関連

- [BlockReader](./BlockReader.md) — ブロック情報の読み取り
- [EnvironmentDetector](./EnvironmentDetector.md) — 環境情報
- [AdvancedPeripherals ドキュメント](https://advancedperipherals.readthedocs.io/) — 公式ドキュメント