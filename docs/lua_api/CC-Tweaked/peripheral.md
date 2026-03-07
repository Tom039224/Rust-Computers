# peripheral

**mod**: CC:Tweaked  
**api**: `peripheral`  
**source**: `peripheral.lua`（Lua）, `PeripheralAPI.java`（native レイヤー）

## 概要

周辺機器（ペリフェラル）の検索・ラップ・メソッド呼び出しを行う標準 API。

- `peripheral.wrap` / `peripheral.find` は **Lua で実装されている純粋な Lua 関数**。
- 内部で呼ばれる native レイヤー（`isPresent`, `getType`, `hasType`, `getMethods`, `call`）は全て `@LuaFunction`（`mainThread` なし）。
- `peripheral.call()` はペリフェラルメソッドをディスパッチするため、imm 対応は**呼び出し先メソッド依存**。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | flags | imm | 説明 |
|---|---|---|---|---|---|
| `getNames` | — | `string[]` | — | ✓ | 接続中の全ペリフェラル名のリストを返す |
| `isPresent` | `name: string` | `boolean` | — | ✓ | 指定名のペリフェラルが存在するか返す |
| `getType` | `peripheral: string\|table` | `string...` | — | ✓ | ペリフェラルの型文字列を返す（複数の場合あり） |
| `hasType` | `peripheral: string\|table`, `type: string` | `boolean\|nil` | — | ✓ | ペリフェラルが指定型を持つか返す |
| `getMethods` | `name: string` | `string[]\|nil` | — | ✓ | ペリフェラルが提供するメソッド名の一覧を返す |
| `getName` | `peripheral: table` | `string` | — | ✓ | `wrap` で取得したペリフェラルオブジェクトの名前を返す |
| `call` | `name: string`, `method: string`, `...: any` | `any` | — | ✗ | ペリフェラルのメソッドを直接呼び出す（imm は呼び出し先依存） |
| `wrap` | `name: string` | `table\|nil` | — | ✓ | ペリフェラルのメソッドテーブルを返す（存在しない場合 `nil`） |
| `find` | `ty: string`, `filter?: function` | `table...` | — | ✓ | 指定型の全ペリフェラルを `wrap` して返す（filter なし時） |

## 備考

- `wrap(name)` は Lua 側でメタテーブルを構築するだけで Java への mainThread コールは発生しない → **imm: ✓**
- `find(ty)` は内部で `wrap()` を連鎖呼び出しするため同様に **imm: ✓**
- `find(ty, filter)` は filter 関数内でペリフェラルメソッドを呼ぶ場合がある。filter の内容が `mainThread` メソッドを呼ぶなら imm 不可。
- `call(name, method, ...)` はペリフェラルメソッドに委譲するため、対象メソッドが `mainThread = true` であれば imm: ✗ になる。

## イベント

| イベント名 | パラメーター | 説明 |
|---|---|---|
| `peripheral` | `side: string` | 新しいペリフェラルが接続されたとき |
| `peripheral_detach` | `side: string` | ペリフェラルが切断されたとき |
