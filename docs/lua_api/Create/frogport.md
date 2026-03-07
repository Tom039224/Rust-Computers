# Frogport (Create_Frogport)

フロッグポート。パッケージの送受信ポート。インベントリへのアクセスとイベント受信が可能。

## Functions

### setAddress(address: string)
- `mainThread = true`
- このポートのアドレスフィルタを設定する。

### getAddress() -> string
- `mainThread = true`
- 現在のアドレスフィルタを返す。

### getConfiguration() -> string | nil
- `mainThread = true`
- 設定を返す（"send_recieve" または "send"）。接続先がなければ nil。

### setConfiguration(config: string) -> boolean
- `mainThread = true`
- 設定を変更する（"send_recieve" または "send"）。

### list() -> table
- `mainThread = true`
- インベントリの内容一覧を返す。`{ [slot]: { name, count, displayName } }`

### getItemDetail(slot: number) -> table
- `mainThread = true`
- 指定スロットのアイテム詳細を返す。

## Events

### package_received
- 引数: `package: PackageObject`
- パッケージを受信したときに発火。

### package_sent
- 引数: `package: PackageObject`
- パッケージを送信したときに発火。

## PackageObject
パッケージのハンドル。以下のフィールドを持つ：
- `address: string` — 宛先アドレス
- `contents: table` — パッケージ内のアイテム一覧
