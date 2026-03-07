# Radar

**mod**: Some-Peripherals  
**peripheral type**: `sp_radar`  
**source**: `RadarPeripheral.kt`

## 概要

指定半径内のエンティティや Valkyrien Skies シップをスキャンするペリフェラル。  
すべてのメソッドは `mainThread = true`。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `scan` | `radius: number` | エンティティ情報リスト | ✗ | 指定半径内の全エンティティ（＋ VS Ship）をスキャンする |
| `scanForEntities` | `radius: number` | エンティティ情報リスト | ✗ | エンティティのみスキャンする |
| `scanForShips` | `radius: number` | Ship 情報リスト | ✗ | VS Ship のみスキャンする（VS がない環境ではエラー） |
| `scanForPlayers` | `radius: number` | エンティティ情報リスト（プレイヤーのみ） | ✗ | プレイヤーのみスキャンする |
| `getConfigInfo` | — | `table` | ✓ | レーダーの現在の設定情報テーブルを返す |

## `scanForEntities` / `scanForPlayers` 返値テーブル（各エントリ）

```lua
{
  x:    number,   -- ワールド X 座標
  y:    number,   -- ワールド Y 座標
  z:    number,   -- ワールド Z 座標
  id:   string,   -- エンティティ登録 ID (例: "minecraft:skeleton")
  type: string,   -- エンティティタイプ文字列
  name: string,   -- 表示名
  ...             -- その他エンティティ固有フィールド
}
```

## `scanForShips` 返値テーブル（各エントリ）

```lua
{
  is_ship:   true,
  id:        number,        -- シップ ID
  pos:       { x, y, z },   -- ワールド位置
  mass:      number,         -- 質量
  rotation:  { x, y, z, w },-- 回転クォータニオン
  velocity:  { x, y, z },   -- 速度ベクトル
  size:      { x, y, z },   -- AABB サイズ
  scale:     { x, y, z },   -- スケール
  moment_of_inertia_tensor: table,  -- 慣性テンソル
  center_of_mass_in_a_ship: { x, y, z },  -- シップ座標系での重心位置
}
```
