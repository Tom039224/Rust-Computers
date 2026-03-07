# Inventory（インベントリ汎用 API）

**mod**: CC:Tweaked  
**peripheral type**: `inventory`（追加タイプとして付与）  
**source**: `AbstractInventoryMethods.java`

## 概要

GenericPeripheral として実装されており、任意のインベントリブロック（チェスト、バレル等）に動的に付与される API。有線ネットワーク上のインベントリ間でアイテムを直接移動できる。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | flags | imm | 説明 |
|---|---|---|---|---|---|
| `size` | — | `number` | mainThread | ✗ | スロット数を返す |
| `list` | — | `table` | mainThread | ✗ | 全スロットのアイテム情報テーブルを返す（空スロットは `nil`、スパーステーブル） |
| `getItemDetail` | `slot: number` | `table \| nil` | mainThread | ✗ | 指定スロットのアイテム詳細情報テーブルを返す。空スロットなら `nil` |
| `getItemLimit` | `slot: number` | `number` | mainThread | ✗ | 指定スロットの最大収納数を返す（通常 64） |
| `pushItems` | `toName: string, fromSlot: number, limit?: number, toSlot?: number` | `number` | mainThread | ✗ | 有線ネットワーク上の別インベントリにアイテムを移動し、実際の移動数を返す |
| `pullItems` | `fromName: string, fromSlot: number, limit?: number, toSlot?: number` | `number` | mainThread | ✗ | 有線ネットワーク上の別インベントリからアイテムを引き出し、実際の移動数を返す |

## `getItemDetail` 返値テーブル構造

```
{
  name:         string,   -- アイテムの登録名 (例: "minecraft:diamond")
  count:        number,   -- 個数
  displayName:  string,   -- 表示名
  maxCount:     number,   -- スタック上限
  damage?:      number,   -- 耐久値（ツール等）
  maxDamage?:   number,   -- 最大耐久値
  tags:         { [tag: string]: true },  -- 適用中のタグ
  ...                     -- mod によって追加フィールドあり
}
```

## `list` 返値テーブル構造

スロット番号をキーとするスパーステーブル。各値は `{ name, count }` の簡易テーブル。

```lua
{
  [1] = { name = "minecraft:iron_ingot", count = 64 },
  [3] = { name = "minecraft:gold_ingot", count = 10 },
  -- [2] は空スロットなので nil
}
```

## 備考

- `pushItems` / `pullItems` は有線ネットワーク（WiredModem 経由）でのみ使用可能。
- `toName` / `fromName` は `WiredModem.getNamesRemote()` で取得できる名前を使用する。
