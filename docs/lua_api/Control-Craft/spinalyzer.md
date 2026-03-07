# Spinalyzer

**mod**: Control-Craft  
**peripheral type**: `spinalyzer`  
**source**: `SpinalyzerPeripheral.java`

## 概要

スパイナライザー。VS2 ship の物理状態（位置・速度・姿勢・慣性）を詳細にモニタリングし、外力・外トルクを直接印加できる上位物理制御ペリフェラル。

## メソッド一覧

### 状態取得

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getQuaternion` | — | `{ x, y, z, w }` | ✓ | 船の現在姿勢をクォータニオンで返す |
| `getQuaternionJ` | — | `{ x, y, z, w }` | ✓ | J 系クォータニオンを返す |
| `getRotationMatrix` | — | `number[3][3]` | ✓ | 回転行列 (3×3) を返す |
| `getRotationMatrixT` | — | `number[3][3]` | ✓ | 転置回転行列 (3×3) を返す |
| `getVelocity` | — | `{ x, y, z }` | ✓ | 船の速度ベクトル (m/s) を返す |
| `getAngularVelocity` | — | `{ x, y, z }` | ✓ | 船の角速度ベクトル (rad/s) を返す |
| `getPosition` | — | `{ x, y, z }` | ✓ | 船の重心のワールド座標を返す |
| `getSpinalyzerPosition` | — | `{ x, y, z }` | ✓ | スパイナライザーブロック自体のワールド座標を返す |
| `getSpinalyzerVelocity` | — | `{ x, y, z }` | ✓ | スパイナライザーブロックの速度を返す |
| `getPhysics` | — | `table` | ✓ | 物理エンジンの全パラメーター Map を返す |

### 力・トルク印加

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `applyInvariantForce` | `x: number`, `y: number`, `z: number` | — | ✗ | ワールド空間固定方向の力を船の重心に印加する |
| `applyInvariantTorque` | `x: number`, `y: number`, `z: number` | — | ✗ | ワールド空間固定方向のトルクを印加する |
| `applyRotDependentForce` | `x: number`, `y: number`, `z: number` | — | ✗ | 船ローカル座標系の力を印加する（回転に追従） |
| `applyRotDependentTorque` | `x: number`, `y: number`, `z: number` | — | ✗ | 船ローカル座標系のトルクを印加する（回転に追従） |

## 返値テーブル構造

### `getQuaternion` / `getQuaternionJ`

```lua
{ x = number, y = number, z = number, w = number }
```

### `getRotationMatrix` / `getRotationMatrixT`

3×3 の二次元 Lua テーブル（行優先）。

### `getVelocity` / `getAngularVelocity` / `getPosition` / `getSpinalyzerPosition` / `getSpinalyzerVelocity`

```lua
{ x = number, y = number, z = number }
```

## 備考

- `applyInvariantForce` はワールド座標系の方向に力を加える（船が回転しても方向が変わらない）。
- `applyRotDependentForce` は船が回転すると力の方向も変わる（ローカル座標系）。
