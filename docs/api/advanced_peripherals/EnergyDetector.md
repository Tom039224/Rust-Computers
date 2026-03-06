# `EnergyDetector` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::energy_detector::EnergyDetector`

**ソースTOML**: `peripherals/advanced_peripherals/energy_detector.toml`


## 概要 / Overview

AdvancedPeripherals の `EnergyDetector` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `EnergyDetector` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::energy_detector::EnergyDetector;
use rc::peripheral::Direction;

let p = EnergyDetector::new(Direction::South);
```

## メソッド / Methods (2)

### `get_transfer_rate_limit(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getTransferRateLimit` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_transfer_rate_limit().await?;
```

### `get_transfer_rate(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getTransferRate` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_transfer_rate().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
