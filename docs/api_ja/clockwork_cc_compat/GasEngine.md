# GasEngine
**モジュール:** `clockwork_cc_compat::gas_engine`  
**ペリフェラルタイプ:** `clockwork:gas_engine`

Clockwork CC Compat のガスエンジンペリフェラル。ガスエンジンアセンブリの接続エンジン数と全体効率のモニタリングを提供します。

## Book-Read メソッド

### `book_next_get_attached_engines` / `read_last_get_attached_engines`
接続エンジン数を取得する。
```rust
pub fn book_next_get_attached_engines(&mut self)
pub fn read_last_get_attached_engines(&self) -> Result<u32, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** 接続エンジン数 (`u32`)。

---

### `book_next_get_total_efficiency` / `read_last_get_total_efficiency`
ガスエンジンアセンブリの全体効率を取得する。
```rust
pub fn book_next_get_total_efficiency(&mut self)
pub fn read_last_get_total_efficiency(&self) -> Result<f64, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** 全体効率 (`f64`)。

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## Immediate メソッド

### `get_attached_engines_imm`
```rust
pub fn get_attached_engines_imm(&self) -> Result<u32, PeripheralError>
```
**戻り値:** 接続エンジン数 (`u32`)。

### `get_total_efficiency_imm`
```rust
pub fn get_total_efficiency_imm(&self) -> Result<f64, PeripheralError>
```
**戻り値:** 全体効率 (`f64`)。

## 使用例

```rust
use rust_computers_api::clockwork_cc_compat::gas_engine::GasEngine;
use rust_computers_api::peripheral::Peripheral;

let mut engine = GasEngine::find().expect("GasEngine が見つかりません");

// Book-read パターン
engine.book_next_get_attached_engines();
engine.book_next_get_total_efficiency();

let count = engine.read_last_get_attached_engines().unwrap();
let efficiency = engine.read_last_get_total_efficiency().unwrap();

// Immediate コール
let count_imm = engine.get_attached_engines_imm().unwrap();
let eff_imm = engine.get_total_efficiency_imm().unwrap();
```
