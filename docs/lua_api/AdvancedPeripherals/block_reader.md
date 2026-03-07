# BlockReader

**mod**: AdvancedPeripherals  
**peripheral type**: `block_reader`  
**source**: `BlockReaderPeripheral.java`

## 概要

ブロックリーダー。隣接するブロックのデータ（NBT・ブロックステート・タイルエンティティ有無）を読み取るペリフェラル。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getBlockName` | — | `string` | ✗ | 対象ブロックのリソース ID（例: `"minecraft:chest"`）を返す |
| `getBlockData` | — | `table` | ✗ | 対象ブロック（タイルエンティティ）の NBT データをテーブルで返す |
| `getBlockStates` | — | `{ [property]: value }` | ✗ | 対象ブロックのブロックステートプロパティをテーブルで返す |
| `isTileEntity` | — | `boolean` | ✗ | 対象ブロックがタイルエンティティかどうかを返す |

## 返値テーブル構造

### `getBlockData`

NBT の内容に依存。例（チェスト）:

```lua
{
  Items = {
    { Slot = 0, id = "minecraft:stone", Count = 64 },
    -- ...
  }
}
```

### `getBlockStates`

```lua
{
  facing = "north",
  waterlogged = false,
  -- ブロック依存のプロパティ
}
```

## 備考

- `getBlockData` はタイルエンティティでないブロックに対しては空テーブルを返す。
- ブロックリーダーが向いている方向の隣接ブロックを対象とする。
