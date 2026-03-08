# GeoScanner

**モジュール:** `advanced_peripherals::geo_scanner`  
**ペリフェラルタイプ:** `advancedPeripherals:geo_scanner`

AdvancedPeripherals ジオスキャナー ペリフェラル。指定半径内の周囲ブロックをスキャンし、現在のチャンクの鉱石分布を分析します。

## ブックリードメソッド

### `book_next_cost` / `read_last_cost` / `cost_imm`
指定半径でのスキャンの燃料コストを取得します。
```rust
pub fn book_next_cost(&mut self, radius: f64)
pub fn read_last_cost(&self) -> Result<f64, PeripheralError>
pub fn cost_imm(&self, radius: f64) -> Result<f64, PeripheralError>
```
**パラメータ:**
- `radius: f64` — スキャン半径

**戻り値:** `f64`

---

### `book_next_scan` / `read_last_scan`
指定半径内のブロックをスキャンします。
```rust
pub fn book_next_scan(&mut self, radius: f64)
pub fn read_last_scan(&self) -> Result<Vec<GeoBlockEntry>, PeripheralError>
```
**パラメータ:**
- `radius: f64` — スキャン半径

**戻り値:** `Vec<GeoBlockEntry>`

---

### `book_next_chunk_analyze` / `read_last_chunk_analyze`
現在のチャンクの鉱石分布を分析します。
```rust
pub fn book_next_chunk_analyze(&mut self)
pub fn read_last_chunk_analyze(&self) -> Result<Value, PeripheralError>
```
**戻り値:** `Value` — 鉱石名と個数のマップ

## イミディエイトメソッド

- `cost_imm(&self, radius: f64) -> Result<f64, PeripheralError>`

## 型定義

```rust
pub struct GeoBlockEntry {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub name: String,
    pub tags: Vec<String>,
}
```

## 使用例

```rust
use rust_computers_api::advanced_peripherals::GeoScanner;
use rust_computers_api::peripheral::Peripheral;

let mut scanner = GeoScanner::wrap(addr);

// まずコストを確認
let cost = scanner.cost_imm(8.0);

// スキャン実行
scanner.book_next_scan(8.0);
wait_for_next_tick().await;
let blocks = scanner.read_last_scan();

// チャンク分析
scanner.book_next_chunk_analyze();
wait_for_next_tick().await;
let ores = scanner.read_last_chunk_analyze();
```
