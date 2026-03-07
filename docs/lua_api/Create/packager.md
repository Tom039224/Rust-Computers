# Packager (Create_Packager)

パッケージャー。アイテムをパッケージに梱包して送り出す。

## Functions

### makePackage() -> boolean
- `mainThread = true`
- パッケージを生成する。既にパッケージが保持されている場合は false。

### list() -> table
- `mainThread = true`
- インベントリの内容一覧を返す。

### getItemDetail(slot: number) -> table
- `mainThread = true`
- 指定スロットのアイテム詳細を返す。

### getAddress() -> string
- `mainThread = true`
- 現在の送付先アドレスを返す（サインベース）。

### setAddress(address?: string)
- `mainThread = true`
- 送付先アドレスを設定する。nil/省略でリセット（サインベースに戻る）。

### getPackage() -> PackageObject | nil
- `mainThread = true`
- 保持中のパッケージオブジェクトを返す。なければ nil。

## Events

### package_received
- 引数: `package: PackageObject`

### package_sent
- 引数: `package: PackageObject`

## PackageObject
- `address: string`
- `contents: table`
- `isEditable() -> boolean`
- `getAddress() -> string`
- `setAddress(address: string)`
- `list() -> table`
- `getItemDetail(slot: number) -> table`
- `getOrderData() -> PackageOrderObject | nil`
