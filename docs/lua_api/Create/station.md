# Station (Create_Station)

列車ステーション。列車の組み立て・分解、スケジュール管理、経路探索などを行う。

## Functions

### assemble()
- `mainThread = true`
- ステーションにある列車を組み立てる。組み立てモードでない場合はエラー。

### disassemble()
- `mainThread = true`
- 列車を分解する。組み立てモードの場合はエラー。

### setAssemblyMode(assemblyMode: boolean)
- `mainThread = true`
- 組み立てモードを切り替える。

### isInAssemblyMode() -> boolean
- `mainThread = false`（アノテーションなし）
- 組み立てモード中かどうかを返す。
- imm対応

### getStationName() -> string
- `mainThread = false`（アノテーションなし）
- ステーション名を返す。
- imm対応

### setStationName(name: string)
- `mainThread = true`
- ステーション名を変更する。

### isTrainPresent() -> boolean
- `mainThread = false`（アノテーションなし）
- 列車が現在ステーションに停車中かどうかを返す。
- imm対応

### isTrainImminent() -> boolean
- `mainThread = false`（アノテーションなし）
- 列車がまもなく到着するかどうかを返す。
- imm対応

### isTrainEnroute() -> boolean
- `mainThread = false`（アノテーションなし）
- 列車がこのステーションに向かっているかどうかを返す。
- imm対応

### getTrainName() -> string
- `mainThread = false`（アノテーションなし）
- 停車中の列車名を返す。列車がいない場合はエラー。
- imm対応

### setTrainName(name: string)
- `mainThread = true`
- 停車中の列車名を変更する。

### hasSchedule() -> boolean
- `mainThread = false`（アノテーションなし）
- 停車中の列車がスケジュールを持つかどうかを返す。
- imm対応

### getSchedule() -> table
- `mainThread = false`（アノテーションなし）
- 停車中の列車のスケジュールをテーブル（NBT変換）で返す。
- imm対応

### setSchedule(schedule: table)
- `mainThread = true`
- 停車中の列車にスケジュールを設定する。

### canTrainReach(destinationFilter: string) -> boolean, string?
- `mainThread = false`（アノテーションなし）
- 現在の列車が目的地フィルタに合致するステーションへ到達できるかどうかを返す。
  到達不可の場合は 2 番目の戻り値に理由文字列（"cannot-reach" or "no-target"）。
- imm対応

### distanceTo(destinationFilter: string) -> number?, string?
- `mainThread = false`（アノテーションなし）
- 現在の列車が目的地フィルタまでの距離を返す。
  到達不可の場合は nil と理由文字列。
- imm対応

## Events

### train_arrive
- 引数: `trainName: string`
- 列車がステーションに到着したときに発火。

### train_depart
- 引数: `trainName: string`
- 列車がステーションを出発したときに発火。
