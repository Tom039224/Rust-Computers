# RedstoneRequester (Create_RedstoneRequester)

レッドストーンリクエスター。特定のアイテムを指定アドレスへ要求する。

## Functions

### request()
- `mainThread = true`
- 現在設定されているリクエストを送信する。

### setRequest(item1?, item2?, ... item9?)
- `mainThread = true`
- リクエスト内容を設定する。各引数はアイテム名の文字列、またはテーブル `{ name: string, count: number }`。

### setCraftingRequest(count: number, item1?, ... item9?)
- `mainThread = true`
- クラフトリクエストを設定する。count はクラフト回数。

### getRequest() -> table
- `mainThread = true`
- 現在のリクエスト内容を返す（`{ [slot]: { name, count } }` 形式）。

### getConfiguration() -> string
- `mainThread = true`
- 設定を返す（"allow_partial" または "strict"）。

### setConfiguration(config: string)
- `mainThread = true`
- 設定を変更する。

### setAddress(address: string)
- `mainThread = true`
- 送信先アドレスを設定する。

### getAddress() -> string
- `mainThread = true`
- 現在の送信先アドレスを返す。

## Events
なし
