# GPU

**mod**: Toms-Peripherals  
**peripheral type**: `tm_gpu`  
**source**: `GPUPeripheral.java`, `GPUImpl`（クローズドソース）

## 概要

GPU ペリフェラル。Toms-Peripherals 独自の高解像度グラフィックス描画システム。ピクセル単位の描画・3D レンダリング・イメージ処理・ウィンドウ管理が可能。

## 基本メソッド（`GPUPeripheral` 固有）

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `setSize` | `pixels: number` | — | ✗ | ディスプレイのピクセルサイズを設定する |
| `refreshSize` | — | — | ✗ | ディスプレイサイズを再計算・更新する |
| `getSize` | — | `w, h, monX, monY, pixelSize` | ✓ | ピクセル幅, 高さ, モニター列数, 行数, ピクセルサイズを返す（多値返却） |

## 描画メソッド（`GPUImpl` 基底）

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `fill` | `r, g, b, a: number` | — | ✗ | 画面全体を指定 RGBA で塗りつぶす |
| `sync` | — | — | ✗ | 描画バッファをモニターに反映する |
| `filledRectangle` | `x, y, w, h: number`, `r, g, b, a: number` | — | ✗ | 矩形を塗りつぶす |
| `drawImage` | `image: Image`, `x, y: number` | — | ✗ | イメージオブジェクトを描画する |
| `drawText` | `text: string`, `x, y: number`, `r, g, b, a: number` | — | ✗ | テキストを描画する |
| `drawChar` | `char: string`, `x, y: number`, `r, g, b, a: number` | — | ✗ | 1 文字を描画する |
| `getTextLength` | `text: string` | `number` | ✓ | テキストのピクセル幅を返す |
| `setFont` | `fontName: string` | — | ✗ | フォントを設定する |
| `clearChars` | — | — | ✗ | カスタム文字データをクリアする |
| `addNewChar` | `codepoint: number`, `data: table` | — | ✗ | カスタム文字を追加する |

## オブジェクト生成メソッド

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `createWindow` | `x, y, w, h: number` | `Window` | ✓ | 2D ウィンドウオブジェクトを作成する |
| `createWindow3D` | `x, y, w, h: number` | `GL` | ✓ | 3D レンダリングオブジェクトを作成する |
| `decodeImage` | `data: string` | `Image` | ✗ | Base64/バイナリデータからイメージを生成する |
| `newImage` | `w, h: number` | `Image` | ✓ | 空のイメージオブジェクトを生成する |

---

## Window オブジェクト

`createWindow` で取得するオブジェクト。2D 描画のサブキャンバス。

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `filledRectangle` | `x, y, w, h, r, g, b, a: number` | — | ✗ | ウィンドウ内に矩形を描く |
| `sync` | — | — | ✗ | ウィンドウの内容を GPU バッファへ反映する |
| （その他） | — | — | ✗ | GPU と同様の描画メソッド群 |

---

## GL オブジェクト

`createWindow3D` で取得するオブジェクト。OpenGL 風の 3D レンダリング API。

| メソッド名 | 引数 | 説明 |
|---|---|---|
| `clear` | — | バッファをクリアする |
| `glFrustum` | `left, right, bottom, top, near, far: number` | 透視投影行列を設定する |
| `glDirLight` | `x, y, z, r, g, b: number` | 方向ライトを設定する |
| `glTranslate` | `x, y, z: number` | 平行移動を適用する |
| `glRotate` | `angle, x, y, z: number` | 回転を適用する |
| `glBegin` | `mode: string` | プリミティブ描画を開始する（`"triangles"`, `"quads"` 等） |
| `glEnd` | — | プリミティブ描画を終了する |
| `glVertex` | `x, y, z: number` | 頂点を定義する |
| `glColor` | `r, g, b, a: number` | 現在のカラーを設定する |
| `glTexCoord` | `u, v: number` | テクスチャ座標を設定する |
| `glGenTextures` | — | テクスチャ ID を生成する |
| `glBindTexture` | `id: number` | テクスチャをバインドする |
| `glTexImage` | `id: number`, `image: Image` | テクスチャにイメージデータを割り当てる |
| `glEnable` | `feature: string` | GL 機能を有効にする |
| `glDisable` | `feature: string` | GL 機能を無効にする |
| `render` | — | 3D シーンをレンダリングする |
| `sync` | — | レンダリング結果を GPU バッファへ反映する |

---

## Image オブジェクト

`decodeImage` / `newImage` で取得するオブジェクト。

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `ref` | — | `number` | ✓ | 内部参照 ID を返す |
| `getRGB` | `x, y: number` | `r, g, b, a` | ✓ | 指定ピクセルの RGBA 値を返す |
| `gpuDraw` | `x, y: number` | — | ✗ | このイメージを GPU に直接描画する |
| `saveImage` | `filename: string` | — | ✗ | イメージをファイルに保存する |

---

## イベント

| イベント名 | パラメーター | 説明 |
|---|---|---|
| `tm_monitor_touch` | `side, x, y` | モニターをタッチしたとき |
| `tm_monitor_mouse_click` | `side, button, x, y` | マウスクリックしたとき |
| `tm_monitor_mouse_drag` | `side, button, x, y` | マウスドラッグしたとき |
| `tm_monitor_mouse_scroll` | `side, dir, x, y` | マウスホイールを動かしたとき |
| `tm_monitor_mouse_move` | `side, x, y` | マウスを動かしたとき |
