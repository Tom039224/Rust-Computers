# NBTStorage

**mod**: AdvancedPeripherals  
**peripheral type**: `nbt_storage`  
**source**: `NBTStoragePeripheral.java`

## 概要

NBT ストレージ。任意の NBT データを永続的に読み書きできるキーバリューストア的なペリフェラル。JSON/SNBT 形式または Lua テーブルで書き込める。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `read` | — | `table` | ✗ | 保存されている NBT データを Lua テーブルで返す |
| `writeJson` | `snbt: string` | `boolean` | ✗ | SNBT（Stringified NBT）文字列を解析して保存する |
| `writeTable` | `data: table` | `boolean` | ✗ | Lua テーブルを NBT に変換して保存する |

## 備考

- `read` は空の場合に空テーブルを返す。
- `writeJson` は MC の SNBT 形式（例: `{Count:64b,id:"minecraft:stone"}`）を受け付ける。
- `writeTable` は Lua テーブルをそのまま NBT にマッピングして保存する。
