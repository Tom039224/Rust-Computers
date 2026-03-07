# ShipAPI

**mod**: CC:Valkyrien Skies (CC-VS)  
**peripheral type**: グローバル API (`ship.*`)  
**source**: `ShipAPI.kt`

## 概要

CC:Tweaked グローバル API。`ship.xxx()` の形で呼び出す。  
コンピューターが Valkyrien Skies のシップ上に配置されている必要がある。  
力系 / 状態変更系メソッドはサーバー管理者権限が必要。

## メソッド一覧

### 読み取り系

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getId` | — | `number` (long) | ✓ | このコンピューターが乗っているシップの ID を返す |
| `getMass` | — | `number` (double) | ✓ | シップの質量を返す |
| `getMomentOfInertiaTensor` | — | `table` (3×3 行列) | ✓ | シップの慣性テンソルを 3×3 行列として返す |
| `getSlug` | — | `string` | ✓ | シップの名前（未設定なら `"no-name"`） |
| `getAngularVelocity` | — | `{x, y, z}` | ✓ | 角速度ベクトルを返す |
| `getQuaternion` | — | `{x, y, z, w}` | ✓ | シップ→ワールド回転クォータニオン（shipToWorldRotation）を返す |
| `getScale` | — | `{x, y, z}` | ✓ | シップのスケールベクトルを返す |
| `getShipyardPosition` | — | `{x, y, z}` | ✓ | シップ座標系でのシップ中心位置を返す |
| `getSize` | — | `{x, y, z}` | ✓ | シップ AABB のサイズ（max - min）を返す |
| `getVelocity` | — | `{x, y, z}` | ✓ | ワールド空間での速度ベクトルを返す |
| `getWorldspacePosition` | — | `{x, y, z}` | ✓ | ワールド空間でのシップ位置を返す |
| `transformPositionToWorld` | `pos: {x,y,z}` または `x: number, y: number, z: number` | `{x, y, z}` | ✓ | シップ座標→ワールド座標に変換する |
| `isStatic` | — | `boolean` | ✓ | シップが静的（固定）かどうかを返す |
| `getTransformationMatrix` | — | `table` (4×4 行列) | ✓ | シップ→ワールド変換行列を返す |
| `getJoints` | — | `table` (list) | ✓ | シップに接続されているジョイント一覧を返す |

### 状態変更系（管理者権限必要）

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `setSlug` | `name: string` | `nil` | ✗ | シップ名を設定する |
| `setStatic` | `b: boolean` | `nil` | ✗ | シップを静的/動的に設定する |
| `setScale` | `scale: number` | `nil` | ✗ | シップのスケールを変更する |
| `teleport` | `data: table` | `nil` | ✗ | シップをテレポートする（設定でテレポート有効時のみ） |

### 力の印加系（管理者権限必要）

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `applyWorldForce` | `fx, fy, fz: number, [px, py, pz?: number]` | `nil` | ✗ | ワールド座標系で力を印加する（作用点省略時は重心） |
| `applyWorldTorque` | `tx, ty, tz: number` | `nil` | ✗ | ワールド座標系でトルクを印加する |
| `applyModelForce` | `fx, fy, fz: number, [px, py, pz?: number]` | `nil` | ✗ | シップモデル座標系で力を印加する |
| `applyModelTorque` | `tx, ty, tz: number` | `nil` | ✗ | シップモデル座標系でトルクを印加する |
| `applyWorldForceToModelPos` | `fx, fy, fz: number, px, py, pz: number` | `nil` | ✗ | ワールド力をシップ座標上の指定点に印加する |
| `applyBodyForce` | `fx, fy, fz: number, [px, py, pz?: number]` | `nil` | ✗ | ボディ座標系で力を印加する |
| `applyBodyTorque` | `tx, ty, tz: number` | `nil` | ✗ | ボディ座標系でトルクを印加する |
| `applyWorldForceToBodyPos` | `fx, fy, fz: number, px, py, pz: number` | `nil` | ✗ | ワールド力をボディ座標の指定点に印加する |

### 物理ティックイベント

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `pullPhysicsTicks` | — | `nil` | ✗ | 物理ティックイベント待機。設定で `physics_tick` 無効時は LuaException |

## `teleport` 引数テーブル

```lua
{
  pos?:       { x, y, z },         -- 位置 (ワールド座標)
  rot?:       { x, y, z, w },      -- 回転 (クォータニオン)
  vel?:       { x, y, z },         -- 速度
  omega?:     { x, y, z },         -- 角速度
  dimension?: string,              -- ディメンション ("minecraft:overworld" など)
  scale?:     number,              -- スケール
}
```

## `getTransformationMatrix` 返値

4×4 行列（`List<List<Double>>`）:
```
[ [m00, m01, m02, m03],
  [m10, m11, m12, m13],
  [m20, m21, m22, m23],
  [m30, m31, m32, m33] ]
```
