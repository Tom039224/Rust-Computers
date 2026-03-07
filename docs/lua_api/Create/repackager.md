# Repackager (Create_Repackager)

リパッケージャー。パッケージを開封して再梱包する。Packager に似た API を持つ。

## Functions

### makePackage() -> boolean
- `mainThread = true`
- パッケージを生成する。

### list() -> table
- `mainThread = true`
- インベントリの内容一覧を返す。

### getItemDetail(slot: number) -> table
- `mainThread = true`
- 指定スロットのアイテム詳細を返す。

### getAddress() -> string
- `mainThread = true`
- 現在の送付先アドレスを返す。

### setAddress(address?: string)
- `mainThread = true`
- 送付先アドレスを設定する。

### getPackage() -> PackageObject | nil
- `mainThread = true`
- 保持中のパッケージオブジェクトを返す。

## Events

### package_repackaged
- 引数: `package: PackageObject`, `count: number`（梱包数）
- パッケージが再梱包されたときに発火。

### package_received / package_sent
- 引数: `package: PackageObject`
