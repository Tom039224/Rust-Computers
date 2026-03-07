# InventoryManager

**mod**: AdvancedPeripherals  
**peripheral type**: `inventory_manager`  
**source**: `InventoryManagerPeripheral.java`

## 概要

インベントリマネージャー。プレイヤーおよび隣接チェストのインベントリをリモート操作・読み取りができるペリフェラル。

## メソッド一覧

### オーナー情報

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getOwner` | — | `string` | ✓ | このペリフェラルに紐づいているプレイヤー名を返す |

### プレイヤーインベントリ操作

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `addItemToPlayer` | `slot: number`, `count?: number` | `number` | ✗ | チェストから指定スロットのアイテムをプレイヤーに渡す。移動した個数を返す |
| `removeItemFromPlayer` | `slot: number`, `count?: number` | `number` | ✗ | プレイヤーから指定スロットのアイテムをチェストへ移動する。移動した個数を返す |
| `list` | — | `ItemEntry[]` | ✗ | プレイヤーインベントリ全スロットのアイテム一覧を返す |
| `getItems` | — | `ItemEntry[]` | ✗ | `list` と同義（別名） |
| `getArmor` | — | `ItemEntry[]` | ✗ | プレイヤーの防具スロット (4 枠) を返す |
| `isPlayerEquipped` | — | `boolean` | ✗ | プレイヤーが装備を持っているかを返す |
| `isWearing` | `slot: number` | `boolean` | ✗ | 指定防具スロットにアイテムがあるかを返す |
| `getItemInHand` | — | `ItemEntry` | ✗ | メインハンドのアイテムを返す |
| `getItemInOffHand` | — | `ItemEntry` | ✗ | オフハンドのアイテムを返す |
| `getEmptySpace` | — | `number` | ✗ | プレイヤーインベントリの空きスロット数を返す |
| `isSpaceAvailable` | — | `boolean` | ✗ | 空きスロットがあるかを返す |
| `getFreeSlot` | — | `number` | ✗ | 最初の空きスロット番号を返す（なければ -1） |

### チェストインベントリ読み取り

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `listChest` | — | `ItemEntry[]` | ✗ | 隣接チェストのアイテム一覧を返す |
| `getItemsChest` | — | `ItemEntry[]` | ✗ | `listChest` と同義（別名） |

## 返値テーブル構造

### `ItemEntry`

```lua
{
  name         = string,   -- アイテム ID
  tags         = string[], -- タグリスト
  count        = number,   -- 個数
  displayName  = string,   -- 表示名
  maxStackSize = number,   -- 最大スタック数
  components   = table,    -- コンポーネントデータ（components NBT）
  fingerprint  = string,   -- アイテム識別ハッシュ
  slot         = number,   -- スロット番号
}
```
