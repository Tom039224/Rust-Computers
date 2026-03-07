# TableClothShop (Create_TableClothShop)

テーブルクロスショップ。アイテムを販売する自動ショップ機能を持つ。

## Functions

### isShop() -> boolean
- `mainThread = true`
- ショップモードかどうかを返す。

### getAddress() -> string
- `mainThread = true`
- 配送先アドレスを返す。

### setAddress(address: string)
- `mainThread = true`
- 配送先アドレスを設定する。

### getPriceTagItem() -> table
- `mainThread = true`
- 価格として設定されているアイテムの詳細を返す。

### setPriceTagItem(itemName?: string)
- `mainThread = true`
- 価格アイテムを設定する。省略/nil で air に設定。

### getPriceTagCount() -> number
- `mainThread = true`
- 価格アイテムの必要個数を返す（1～100）。

### setPriceTagCount(count?: number)
- `mainThread = true`
- 価格アイテムの個数を設定する（1～100）。省略/nil で 1。

### getWares() -> table
- `mainThread = true`
- 販売中のアイテム一覧を返す（`{ [i]: { name, count, ... } }`）。

### setWares(item1: table, ... item9: table)
- `mainThread = true`
- 販売するアイテムを設定する。インベントリが空でないとエラー。
  各引数はテーブル `{ name: string, count: number }`。

## Events
なし
