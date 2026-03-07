# Digitizer

**mod**: Some-Peripherals  
**peripheral type**: `digitizer`  
**source**: `DigitizerPeripheral.kt`

## 概要

アイテムをデジタル化（UUID で管理）し、保存・合成・分割・再マテリアル化できるペリフェラル。  
すべてのメソッドは `mainThread = true`（サーバーメインスレッドで実行）。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `digitizeAmount` | `amount?: number` | `string` (UUID) または `{error: string}` | ✗ | スロット 0 のアイテムを指定数デジタル化し UUID を返す。スロットが空・amount ≤ 0 の場合はエラー |
| `rematerializeAmount` | `uuid: string, amount?: number` | `boolean` または `{error: string}` | ✗ | 指定 UUID のデジタルアイテムを物理化してスロット 0 に戻す |
| `mergeDigitalItems` | `into_uuid: string, from_uuid: string, amount?: number` | `boolean` または `{error: string}` | ✗ | 2 つのデジタルアイテムを結合する（同種スタック可能アイテムのみ） |
| `separateDigitalItem` | `from_uuid: string, amount: number` | `string` (新 UUID) または `{error: string}` | ✗ | デジタルアイテムスタックを分割し、分割分の新 UUID を返す |
| `checkID` | `uuid: string` | `{item: {...}}` または `{error: string}` | ✗ | UUID が存在するか確認し、対応アイテム情報を返す |
| `getItemInSlot` | — | `{id, count, tag, ...}` | ✗ | スロット 0 のアイテム情報テーブル（ItemData マップ）を返す |
| `getItemLimitInSlot` | — | `number` | ✗ | スロット 0 のアイテム上限数を返す |

## `checkID` 返値テーブル

```lua
{
  item = {
    id:    string,   -- アイテム登録名
    count: number,   -- 個数
    tag:   table,    -- NBT データ
    ...              -- その他 ItemData フィールド
  }
}
```
