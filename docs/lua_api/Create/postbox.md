# Postbox (Create_Postbox)

ポストボックス。パッケージの受信ポイント。Frogport に似た機能を持つ。

## Functions

### setAddress(address: string)
- `mainThread = true`
- アドレスフィルタを設定する。

### getAddress() -> string
- `mainThread = true`
- 現在のアドレスフィルタを返す。

### list() -> table
- `mainThread = true`
- インベントリの内容一覧を返す。

### getItemDetail(slot: number) -> table
- `mainThread = true`
- 指定スロットのアイテム詳細を返す。

### getConfiguration() -> string | nil
- `mainThread = true`
- 設定を返す（"send_recieve" または "send"）。接続先がなければ nil。

### setConfiguration(config: string) -> boolean
- `mainThread = true`
- 設定を変更する。

## Events

### package_received
- 引数: `package: PackageObject`

### package_sent
- 引数: `package: PackageObject`
