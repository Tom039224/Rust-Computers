# GeoScanner

**mod**: AdvancedPeripherals  
**peripheral type**: `geo_scanner`  
**source**: `GeoScannerPeripheral.java`

## 概要

ジオスキャナー。指定半径内のブロック情報や鉱石チャンク分布をスキャンするペリフェラル。操作コストがあり、`cost()` で事前確認できる。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `cost` | `radius: number` | `number` | ✓ | 指定半径スキャンの演算コストを返す |
| `scan` | `radius: number` | `BlockEntry[]` | ✗ | 指定半径内のブロック一覧を返す |
| `chunkAnalyze` | — | `{ [ore]: count }` | ✗ | 現在チャンクの鉱石分布（鉱石名→個数）を返す |

## 返値テーブル構造

### `scan` 要素 (`BlockEntry`)

```lua
{
  x    = number,   -- 相対 X 座標
  y    = number,   -- 相対 Y 座標
  z    = number,   -- 相対 Z 座標
  name = string,   -- ブロック ID（例: "minecraft:diamond_ore"）
  tags = string[], -- ブロックタグリスト
}
```

### `chunkAnalyze`

```lua
{
  ["minecraft:diamond_ore"]    = 12,
  ["minecraft:iron_ore"]       = 48,
  -- ...
}
```

## 備考

- スキャン半径が大きいほどコストが高くなる。
- `chunkAnalyze` はスキャナーが存在するチャンク全体を垂直方向に走査する。
