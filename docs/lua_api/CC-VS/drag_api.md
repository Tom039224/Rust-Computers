# DragAPI

**mod**: CC:Valkyrien Skies (CC-VS)  
**peripheral type**: グローバル API (`drag.*`)  
**source**: `DragAPI.kt`

## 概要

CC:Tweaked グローバル API。`drag.xxx()` の形で呼び出す。  
コンピューターが Valkyrien Skies のシップ上に配置されている必要がある。  
enable/disable 系および風設定系は管理者権限が必要。

## メソッド一覧

### 読み取り系

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getDragForce` | — | `{x, y, z} \| nil` | ✓ | シップにかかっている抗力ベクトルを返す |
| `getLiftForce` | — | `{x, y, z} \| nil` | ✓ | シップにかかっている揚力ベクトルを返す |

### 状態変更系（管理者権限必要）

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `enableDrag` | — | `nil` | ✗ | シップの抗力を有効にする |
| `disableDrag` | — | `nil` | ✗ | シップの抗力を無効にする |
| `enableLift` | — | `nil` | ✗ | シップの揚力を有効にする |
| `disableLift` | — | `nil` | ✗ | シップの揚力を無効にする |
| `enableRotDrag` | — | `nil` | ✗ | 回転抗力を有効にする |
| `disableRotDrag` | — | `nil` | ✗ | 回転抗力を無効にする |
| `setWindDirection` | `x, y, z: number` | `nil` | ✗ | 風向きベクトルを設定する |
| `setWindSpeed` | `speed: number` | `nil` | ✗ | 風速を設定する |
| `applyWindImpulse` | `x, y, z: number, (idx3 skip), speed: number` | `nil` | ✗ | 指定方向・速度の風衝撃をシップに与える |

## 注意事項

- `applyWindImpulse` の引数インデックスに**バグ**がある: 引数インデックス 0〜2 が x/y/z、インデックス 3 がスキップされ、インデックス 4 が `speed`。Lua から呼ぶ際は `applyWindImpulse(x, y, z, nil, speed)` のように nil でパディングが必要。
