# BallisticAccelerator

**モジュール:** `some_peripherals`  
**ペリフェラルタイプ:** `sp:ballistic_accelerator` (Peripheral::NAME)

Some-Peripherals Mod の BallisticAccelerator ペリフェラル。飛行時間の計算、ピッチ角の計算、バッチ演算など、弾道計算機能を提供します。

## Book-Read メソッド

### `book_next_time_in_air` / `read_last_time_in_air`

投射体の飛行時間を計算します。

```rust
pub fn book_next_time_in_air(
    &mut self,
    y_proj: f64,
    y_tgt: f64,
    y_vel: f64,
    gravity: Option<f64>,
    drag: Option<f64>,
    max_steps: Option<u32>,
) { ... }
pub fn read_last_time_in_air(&self) -> Result<SPTimeResult, PeripheralError> { ... }
```

**パラメータ:**
- `y_proj`: 投射体の Y 位置
- `y_tgt`: ターゲットの Y 位置
- `y_vel`: 投射体の垂直速度
- `gravity` *(省略可)*: 重力値（`None` の場合はデフォルト値を使用）
- `drag` *(省略可)*: 抗力係数（`None` の場合はデフォルト値を使用）
- `max_steps` *(省略可)*: 最大シミュレーションステップ数

**戻り値:** `Result<SPTimeResult, PeripheralError>`

---

### `book_next_try_pitch` / `read_last_try_pitch`

特定のピッチ角をテストし、結果の軌道データを返します。

```rust
pub fn book_next_try_pitch(
    &mut self,
    pitch: f64,
    speed: f64,
    length: f64,
    dist: f64,
    cannon: SPCoordinate,
    target: SPCoordinate,
    gravity: Option<f64>,
    drag: Option<f64>,
    max_steps: Option<u32>,
) { ... }
pub fn read_last_try_pitch(&self) -> Result<(f64, f64, f64), PeripheralError> { ... }
```

**パラメータ:**
- `pitch`: テストするピッチ角（ラジアン）
- `speed`: 投射体の速度
- `length`: 砲身の長さ
- `dist`: ターゲットまでの距離
- `cannon`: 砲台の位置
- `target`: ターゲットの位置
- `gravity` *(省略可)*: 重力値
- `drag` *(省略可)*: 抗力係数
- `max_steps` *(省略可)*: 最大シミュレーションステップ数

**戻り値:** `Result<(f64, f64, f64), PeripheralError>` — 3つの f64 値のタプル

---

### `book_next_calculate_pitch` / `read_last_calculate_pitch`

ターゲットに命中するための最適なピッチ角を計算します。

```rust
pub fn book_next_calculate_pitch(
    &mut self,
    cannon: SPCoordinate,
    target: SPCoordinate,
    speed: f64,
    length: f64,
    amin: Option<f64>,
    amax: Option<f64>,
    gravity: Option<f64>,
    drag: Option<f64>,
    max_delta_t_error: Option<f64>,
    max_steps: Option<u32>,
    num_iterations: Option<u32>,
    num_elements: Option<u32>,
    check_impossible: Option<bool>,
) { ... }
pub fn read_last_calculate_pitch(&self) -> Result<SPPitchResult, PeripheralError> { ... }
```

**パラメータ:**
- `cannon`: 砲台の位置
- `target`: ターゲットの位置
- `speed`: 投射体の速度
- `length`: 砲身の長さ
- `amin` *(省略可)*: 最小ピッチ角
- `amax` *(省略可)*: 最大ピッチ角
- `gravity` *(省略可)*: 重力値
- `drag` *(省略可)*: 抗力係数
- `max_delta_t_error` *(省略可)*: 最大飛行時間誤差許容値
- `max_steps` *(省略可)*: 最大シミュレーションステップ数
- `num_iterations` *(省略可)*: ソルバー反復回数
- `num_elements` *(省略可)*: 解法に使用する要素数
- `check_impossible` *(省略可)*: 到達不可能な軌道をチェックするかどうか

**戻り値:** `Result<SPPitchResult, PeripheralError>`

---

### `book_next_batch_calculate_pitches` / `read_last_batch_calculate_pitches`

1つの砲台位置から複数のターゲットへのピッチ角をバッチ計算します。

```rust
pub fn book_next_batch_calculate_pitches(
    &mut self,
    cannon: SPCoordinate,
    targets: &[SPCoordinate],
    speed: f64,
    length: f64,
    amin: Option<f64>,
    amax: Option<f64>,
    gravity: Option<f64>,
    drag: Option<f64>,
    max_delta_t_error: Option<f64>,
    max_steps: Option<u32>,
    num_iterations: Option<u32>,
    num_elements: Option<u32>,
    check_impossible: Option<bool>,
) { ... }
pub fn read_last_batch_calculate_pitches(&self) -> Result<Vec<SPPitchResult>, PeripheralError> { ... }
```

**パラメータ:**
- `cannon`: 砲台の位置
- `targets`: ターゲット位置のスライス
- `speed`: 投射体の速度
- `length`: 砲身の長さ
- `amin` *(省略可)*: 最小ピッチ角
- `amax` *(省略可)*: 最大ピッチ角
- `gravity` *(省略可)*: 重力値
- `drag` *(省略可)*: 抗力係数
- `max_delta_t_error` *(省略可)*: 最大飛行時間誤差許容値
- `max_steps` *(省略可)*: 最大シミュレーションステップ数
- `num_iterations` *(省略可)*: ソルバー反復回数
- `num_elements` *(省略可)*: 解法に使用する要素数
- `check_impossible` *(省略可)*: 到達不可能な軌道をチェックするかどうか

**戻り値:** `Result<Vec<SPPitchResult>, PeripheralError>` — 各ターゲットのピッチ計算結果

---

### `book_next_get_drag` / `read_last_get_drag`

計算された抗力係数を取得します。

```rust
pub fn book_next_get_drag(&mut self, base_drag: f64, dim_drag_multiplier: f64) { ... }
pub fn read_last_get_drag(&self) -> Result<f64, PeripheralError> { ... }
```

**パラメータ:**
- `base_drag`: 基本抗力値
- `dim_drag_multiplier`: 次元抗力乗数

**戻り値:** `Result<f64, PeripheralError>` — 計算された抗力係数

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## Immediate メソッド

### `time_in_air_imm`

次のティックを待たずに即座に飛行時間を計算します。

```rust
pub fn time_in_air_imm(
    &self,
    y_proj: f64, y_tgt: f64, y_vel: f64,
    gravity: Option<f64>, drag: Option<f64>, max_steps: Option<u32>,
) -> Result<SPTimeResult, PeripheralError> { ... }
```

### `try_pitch_imm`

次のティックを待たずに即座にピッチ角をテストします。

```rust
pub fn try_pitch_imm(
    &self,
    pitch: f64, speed: f64, length: f64, dist: f64,
    cannon: SPCoordinate, target: SPCoordinate,
    gravity: Option<f64>, drag: Option<f64>, max_steps: Option<u32>,
) -> Result<(f64, f64, f64), PeripheralError> { ... }
```

### `calculate_pitch_imm`

次のティックを待たずに即座に最適なピッチ角を計算します。

```rust
pub fn calculate_pitch_imm(
    &self,
    cannon: SPCoordinate, target: SPCoordinate,
    speed: f64, length: f64,
    amin: Option<f64>, amax: Option<f64>,
    gravity: Option<f64>, drag: Option<f64>,
    max_delta_t_error: Option<f64>, max_steps: Option<u32>,
    num_iterations: Option<u32>, num_elements: Option<u32>,
    check_impossible: Option<bool>,
) -> Result<SPPitchResult, PeripheralError> { ... }
```

### `get_drag_imm`

次のティックを待たずに即座に抗力係数を取得します。

```rust
pub fn get_drag_imm(
    &self,
    base_drag: f64,
    dim_drag_multiplier: f64,
) -> Result<f64, PeripheralError> { ... }
```

## 型定義

### `SPCoordinate`

3D 座標。

| フィールド | 型    | 説明    |
|-----------|-------|---------|
| `x`       | `f64` | X 座標  |
| `y`       | `f64` | Y 座標  |
| `z`       | `f64` | Z 座標  |

### `SPTimeResult`

飛行時間計算結果。

| フィールド | 型    | 説明              |
|-----------|-------|-------------------|
| `ticks`   | `f64` | ティック単位の時間  |
| `aux`     | `f64` | 補助値             |

### `SPPitchResult`

ピッチ角計算結果。

| フィールド | 型    | 説明              |
|-----------|-------|-------------------|
| `pitch`   | `f64` | 計算されたピッチ角  |
| `aux`     | `f64` | 補助値             |

### `SPDroneBlueprint`

ドローン設計図データ。

| フィールド | 型             | 説明           |
|-----------|----------------|----------------|
| `cannon`  | `SPCoordinate` | 砲台の位置      |
| `target`  | `SPCoordinate` | ターゲットの位置 |
| `speed`   | `f64`          | 投射体の速度    |
| `length`  | `f64`          | 砲身の長さ      |

## 使用例

```rust
let cannon = SPCoordinate { x: 0.0, y: 64.0, z: 0.0 };
let target = SPCoordinate { x: 100.0, y: 64.0, z: 50.0 };

// ピッチ角を計算
peripheral.book_next_calculate_pitch(
    cannon, target,
    10.0,   // 速度
    2.0,    // 砲身の長さ
    None, None, None, None, None, None, None, None, None,
);
wait_for_next_tick().await;
let result = peripheral.read_last_calculate_pitch()?;
log!("ピッチ: {} (補助: {})", result.pitch, result.aux);

// または即座バリアントを使用
let result = peripheral.calculate_pitch_imm(
    cannon, target, 10.0, 2.0,
    None, None, None, None, None, None, None, None, None,
)?;

// 複数ターゲットのバッチ計算
let targets = &[
    SPCoordinate { x: 100.0, y: 64.0, z: 50.0 },
    SPCoordinate { x: 200.0, y: 70.0, z: 0.0 },
];
peripheral.book_next_batch_calculate_pitches(
    cannon, targets, 10.0, 2.0,
    None, None, None, None, None, None, None, None, None,
);
wait_for_next_tick().await;
let results = peripheral.read_last_batch_calculate_pitches()?;
```
