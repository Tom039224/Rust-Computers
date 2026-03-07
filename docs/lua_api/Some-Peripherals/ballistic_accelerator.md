# BallisticAccelerator

**mod**: Some-Peripherals  
**peripheral type**: `ballistic_accelerator`  
**source**: `BallisticAcceleratorPeripheral.kt`

## 概要

弾道計算専用ペリフェラル。砲台位置・目標位置・砲弾パラメータを入力として、最適なピッチ角・飛行時間を計算する。Create: Big Cannons との連携を想定した計算ユーティリティ。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `timeInAir` | `y_proj, y_tgt, y_vel: number, gravity?: number, drag?: number, max_steps?: number` | `{ticks, aux}` | ✓ | 射出体が目標 Y 座標に到達するまでの飛行時間 (tick) を計算する |
| `tryPitch` | `pitch, speed, length, dist: number, cannon: table, target: table, gravity?: number, drag?: number, max_steps?: number` | `{result×3}` または `{-1, -1, -1}` | ✓ | 指定ピッチ角を試して弾道計算結果を返す。失敗時は `{-1,-1,-1}` |
| `calculatePitch` | `cannon: table, target: table, speed, length: number, [amin?, amax?, gravity?, drag?, max_delta_t_error?, max_steps?, num_iterations?, num_elements?, check_impossible?]` | `{pitch, aux}` | ✓ | 砲台位置と目標位置から最適ピッチ角を計算する |
| `batchCalculatePitches` | `cannon: table, targets: table[], speed, length: number, [amin?, amax?, ...]` | `{{pitch, aux}, ...}` | ✗ | 複数ターゲットに対して一括でピッチ角を計算する |
| `getDrag` | `base_drag: number, dim_drag_multiplier: number` | `number` | ✓ | 基本ドラッグとディメンション乗数から実効ドラッグを計算する |

## 引数の詳細

### `calculatePitch` / `batchCalculatePitches` の省略可能引数

| 引数名 | デフォルト値 | 説明 |
|---|---|---|
| `amin` | `-30` | 最小仰角（度） |
| `amax` | `60` | 最大仰角（度） |
| `gravity` | `0.05` | 重力加速度（tick 毎） |
| `drag` | `0.99` | 空気抵抗係数（0〜1） |
| `max_delta_t_error` | `1.0` | 許容飛行時間誤差（tick） |
| `max_steps` | `1000000` | シミュレーション最大ステップ数 |
| `num_iterations` | `5` | 二分探索の反復回数 |
| `num_elements` | `20` | 探索グリッドの分割数 |
| `check_impossible` | `true` | 不可能な射撃角を事前チェックするか |

## `cannon` / `target` テーブル形式

砲台位置・目標位置はそれぞれ `double[]` に相当する数値テーブル（`{x, y, z}` 形式）。
