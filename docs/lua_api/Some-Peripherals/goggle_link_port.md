# GoggleLinkPort

**mod**: Some-Peripherals  
**peripheral type**: `goggle_link_port`  
**source**: `GoggleLinkPortPeripheral.kt`

## 概要

接続中の Goggles（ゴーグル系デバイス）へのアクセスを提供するペリフェラル。  
`getConnected()` が返すテーブルに、各 Goggle の操作関数が格納されている。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getConnected` | — | `table` (Goggle マップ) | ✗ | 接続中の Goggles テーブルを返す。各エントリは Goggle 識別文字列をキーとした関数テーブル |

## `getConnected` 返値テーブル構造

返値は `{ [goggle_id: string]: GoggleFunctions }` 形式のテーブル。  
各 `GoggleFunctions` には以下の関数が含まれる:

### 全 Goggle 共通

| 関数名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getInfo` | `im_execute?: boolean` | 情報テーブル | ✗ | Goggle の情報を取得する |
| `type` | — | `string` | ✗ | Goggle の種類を返す |
| `terminateAll` | — | `nil` | ✗ | 進行中の全操作を終了する |
| `getConfigInfo` | — | `table` | ✗ | 設定情報テーブルを返す |

### Range Goggles 追加関数

| 関数名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `raycast` | `distance: number, variables?, euler_mode?, im_execute?, check_for_blocks?, only_distance?` | ヒット情報テーブルまたは制御オブジェクト | ✗ | Goggle からレイキャストを実行する |
| `queueRaycasts` | `distance: number, rays_table: table, euler_mode?, check_for_blocks?, only_distance?` | `nil` | ✗ | 複数レイキャストをキューに追加する |
| `getQueuedData` | — | `table` | ✗ | キューに積まれたレイキャスト結果を取得する |

## 備考

- `raycast` / `queueRaycasts` は `MethodResult.pullEvent` を使ったコルーチン継続方式で非同期実行される（`im_execute=false` 時）。
- 内部的に `FunToLuaWrapper` で関数テーブルが動的に構築される。
