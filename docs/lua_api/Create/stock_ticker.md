# StockTicker (Create_StockTicker)

ストックティッカー。接続された倉庫ネットワークの在庫管理と配送要求を行う。

## Functions

### stock(detailed?: boolean) -> table
- `mainThread = true`
- 倉庫の在庫一覧を返す。detailed=true でアイテム詳細情報付き。
  形式: `{ [i]: { name, count, displayName, ... } }`

### getStockItemDetail(slot: number) -> table
- `mainThread = true`
- 指定スロットのアイテム詳細を返す。

### requestFiltered(address: string, filter1: table, ...) -> number
- `mainThread = true`
- フィルタに合致するアイテムを指定アドレスへ配送要求する。
  フィルタテーブルには `_requestCount: number` を含めると数量制限を指定できる。
  戻り値: 送信したアイテム合計数。

### list() -> table
- `mainThread = true`
- 受信済み支払いインベントリの内容を返す。

### getItemDetail(slot: number) -> table
- `mainThread = true`
- 受信済み支払いの指定スロット詳細を返す。

## Events
なし
