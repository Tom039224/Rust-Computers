# LuaPhysShip（物理ティックオブジェクト）

**mod**: CC:Valkyrien Skies (CC-VS)  
**peripheral type**: — (オブジェクト型)  
**source**: `LuaPhysShip.kt`

## 概要

`ship.pullPhysicsTicks()` または `physics_ticks` イベントで返されるデータオブジェクト。  
物理ティック（サーバーティックとは独立して動作）ごとに生成される一時的なオブジェクト。

## 取得方法

```lua
ship.pullPhysicsTicks()  -- physics_ticks イベントを待機
local physShip = ...     -- イベント引数として渡される
```

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getBuoyantFactor` | — | `number` | ✓ | 物理ティック時点の浮力係数を返す |
| `isStatic` | — | `boolean` | ✓ | 物理ティック時点でシップが静的かどうかを返す |
| `doFluidDrag` | — | `boolean` | ✓ | 流体抗力が有効かどうかを返す |
| `getInertia` | — | `{momentOfInertia: {x,y,z}, mass: number}` | ✓ | 慣性情報（慣性モーメントと質量）を返す |
| `getPoseVel` | — | `{vel, omega, pos, rot}` | ✓ | 姿勢・速度情報を返す |
| `getForcesInducers` | — | `table` (string list) | ✓ | 力の発生源リストを返す（現在常に空） |

## `getPoseVel` 返値テーブル

```lua
{
  vel:   { x, y, z },           -- 速度 (m/tick)
  omega: { x, y, z },           -- 角速度 (rad/tick)
  pos:   { x, y, z },           -- 重心位置 (ワールド座標)
  rot:   { x, y, z, w },        -- 回転 (クォータニオン)
}
```

## `getInertia` 返値テーブル

```lua
{
  momentOfInertia: { x, y, z },  -- 慣性モーメント (対角成分)
  mass: number,                   -- 質量
}
```
