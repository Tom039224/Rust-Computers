# Camera

**mod**: Control-Craft  
**peripheral type**: `camera`  
**source**: `CameraPeripheral.java`

## 概要

カメラペリフェラル。レイキャスト・クリッピング・姿勢制御など多彩な視覚系操作を提供する。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getAbsViewTransform` | — | `Transform` | ✓ | カメラのワールド空間変換行列を返す |
| `clip` | — | `ClipResult` | ✗ | 正面方向にレイキャストし最初に当たった対象を返す |
| `clipEntity` | — | `ClipResult` | ✗ | エンティティに対してレイキャストする |
| `clipBlock` | — | `ClipResult` | ✗ | ブロックに対してレイキャストする |
| `clipAllEntity` | — | `ClipResult[]` | ✗ | 全エンティティへのレイキャスト結果リストを返す |
| `clipShip` | — | `ClipResult` | ✗ | 船（VS2 ship）に対してレイキャストする |
| `clipPlayer` | — | `ClipResult` | ✗ | プレイヤーに対してレイキャストする |
| `setPitch` | `degrees: number` | — | ✗ | カメラのピッチ角度 (deg) を設定する |
| `setYaw` | `degrees: number` | — | ✗ | カメラのヨー角度 (deg) を設定する |
| `getPitch` | — | `number` | ✓ | 現在のピッチ角度 (deg) を返す |
| `getYaw` | — | `number` | ✓ | 現在のヨー角度 (deg) を返す |
| `getTransformedPitch` | — | `number` | ✓ | 変換後のピッチ角度 (deg) を返す |
| `getTransformedYaw` | — | `number` | ✓ | 変換後のヨー角度 (deg) を返す |
| `outlineToUser` | — | — | ✗ | カメラ視点をユーザーに表示する |
| `forcePitchYaw` | `pitch: number`, `yaw: number` | — | ✗ | ピッチとヨーを強制的に設定する |
| `getClipDistance` | — | `number` | ✓ | レイキャストの最大距離を返す |
| `setClipRange` | `range: number` | — | ✗ | レイキャストの最大距離を設定する |
| `setConeAngle` | `angle: number` | — | ✗ | コーン（円錐）検索角度を設定する |
| `latestShip` | — | `ShipInfo` | ✓ | 最後にレイキャストしたシップの情報を返す |
| `latestPlayer` | — | `PlayerInfo` | ✓ | 最後にレイキャストしたプレイヤーの情報を返す |
| `latestEntity` | — | `EntityInfo` | ✓ | 最後にレイキャストしたエンティティの情報を返す |
| `latestBlock` | — | `BlockInfo` | ✓ | 最後にレイキャストしたブロックの情報を返す |
| `clipNewShip` | — | `ClipResult` | ✗ | 新規シップへのレイキャストを実行する |
| `clipNewBlock` | — | `ClipResult` | ✗ | 新規ブロックへのレイキャストを実行する |
| `raycast` | `x: number`, `y: number`, `z: number`, `...` | `ClipResult` | ✗ | 指定方向ベクトルでレイキャストする |
| `getEntities` | `radius: number` | `EntityInfo[]` | ✗ | 指定半径内のエンティティ一覧を返す |
| `getMobs` | `radius: number` | `EntityInfo[]` | ✗ | 指定半径内のモブ一覧を返す |
| `getCameraPosition` | — | `{ x, y, z }` | ✓ | カメラのワールド座標を返す |
| `getAbsViewForward` | — | `{ x, y, z }` | ✓ | カメラの正面ベクトル（ワールド空間）を返す |
| `getLocViewTransform` | — | `Transform` | ✓ | カメラのローカル空間変換行列を返す |
| `getLocViewForward` | — | `{ x, y, z }` | ✓ | カメラの正面ベクトル（ローカル空間）を返す |
| `isBeingUsed` | — | `boolean` | ✓ | カメラが現在使用中かを返す |
| `getDirection` | — | `string` | ✓ | カメラが向いている方向を返す |
| `reset` | — | — | ✗ | カメラの向きをデフォルトにリセットする |
