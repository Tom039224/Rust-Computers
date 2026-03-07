# WorldScanner

**mod**: Some-Peripherals  
**peripheral type**: `world_scanner`  
**source**: `WorldScannerPeripheral.kt`

## 概要

ワールドスキャナーからの相対座標にあるブロックの種類を返すペリフェラル。  
Valkyrien Skies が有効な場合はシップ座標変換・ship_id 付与も行う。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getBlockAt` | `x, y, z: number, inShipyard?: boolean` | ブロック情報テーブルまたは `{error: string}` | ✗ | 指定相対座標のブロック種類を返す |

## `getBlockAt` 引数

| 引数名 | 型 | デフォルト | 説明 |
|---|---|---|---|
| `x, y, z` | `number` | — | ワールドスキャナーからの相対座標 |
| `inShipyard` | `boolean` | `false` | `true` でシップヤード（シップ内）座標系で取得する |

## `getBlockAt` 返値テーブル

```lua
{
  block_type: string,    -- ブロックの登録名 (例: "minecraft:stone")
  ship_id?:   number,    -- VS Ship ID（その座標がシップ上の場合のみ）
}
```

エラー時: `{ error: string }`
