# Raycaster

**mod**: Some-Peripherals  
**peripheral type**: `raycaster`  
**source**: `RaycasterPeripheral.kt`

## 概要

指定方向にレイキャストを行うペリフェラル。ブロック・エンティティ・VS シップへのヒット情報を返す。`im_execute=false` で非同期制御オブジェクトを返すコルーチン継続方式にも対応。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `raycast` | `distance: number, variables?: table, euler_mode?: boolean, im_execute?: boolean, check_for_blocks?: boolean, only_distance?: boolean` | ヒット情報テーブルまたは制御オブジェクト | ✗ | レイキャストを実行しヒット情報を返す |
| `addStickers` | `state: boolean` | `nil` | ✗ | レイキャスターブロックの powered ブロックステートを設定する |
| `getConfigInfo` | — | `table` | ✓ | 現在の設定情報を返す |
| `getFacingDirection` | — | `string` | ✓ | ブロックの向いている方向を返す (`"north"` / `"south"` / `"east"` / `"west"` / `"up"` / `"down"`) |

## `raycast` 引数

| 引数名 | 型 | デフォルト | 説明 |
|---|---|---|---|
| `distance` | `number` | — | レイの長さ |
| `variables` | `{pitch_or_y, yaw_or_x, planar_dist?}` | `{0.0, 0.0, 1.0}` | 方向変数（2〜3 要素テーブル） |
| `euler_mode` | `boolean` | `false` | `true` でオイラー角 (ピッチ/ヨー) 指定に切り替え |
| `im_execute` | `boolean` | `true` | `false` で非同期制御オブジェクトを返す |
| `check_for_blocks` | `boolean` | `true` | ブロックとの衝突判定を行うか |
| `only_distance` | `boolean` | `false` | `true` の場合は距離のみ返す |

## `raycast` 返値テーブル（`im_execute=true` のとき）

```lua
{
  is_block?:      boolean,                     -- ブロックにヒットしたか
  is_entity?:     boolean,                     -- エンティティにヒットしたか
  abs_pos?:       { x, y, z },                 -- ヒットしたブロックのワールド座標
  hit_pos?:       { x, y, z },                 -- ヒット点のワールド座標
  distance?:      number,                      -- ヒットまでの距離
  block_type?:    string,                      -- ヒットしたブロックの登録名
  rel_hit_pos?:   { x, y, z },                 -- ヒット点のブロック内相対座標 (0〜1)
  id?:            string,                      -- エンティティID（エンティティヒット時）
  descriptionId?: string,                      -- エンティティの説明ID
  ship_id?:       number,                      -- VS Ship ID（シップにヒット時）
  hit_pos_ship?:  { x, y, z },                 -- シップ座標系でのヒット点
  error?:         string,                      -- エラーメッセージ
}
```

## `raycast` 返値オブジェクト（`im_execute=false` のとき）

```lua
{
  begin:     function(),                  -- レイキャストの実行を開始する
  getCurI:   function() -> number,        -- 現在の反復インデックスを返す
  terminate: function(),                  -- 実行を中断する
}
```
