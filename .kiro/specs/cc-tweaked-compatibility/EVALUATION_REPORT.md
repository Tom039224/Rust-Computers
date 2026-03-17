# CC:Tweaked互換性改善プロジェクト - 整合性評価レポート

**評価日**: 2025-01-XX  
**評価者**: Kiro AI  
**評価対象**: Phase 1-4完了後の実装とドキュメントの整合性

## エグゼクティブサマリー

現在のRustComputers実装とドキュメントの間に**重大な不整合**が発見されました。主な問題点：

1. **async_*メソッドの欠落** - ドキュメントでは3つの関数パターン（book_next/read_last/async）を謳っているが、実装では大半のasync_*メソッドが存在しない
2. **イベントシステム未実装** - Modem、Monitor、ChatBox、PlayerDetectorのイベント機能が完全に未実装
3. **ドキュメントと実装の乖離** - ドキュメントが実装されていない機能を記載している

**推奨アクション**: Phase 6で以下を実施
- 全ペリフェラルにasync_*メソッドを追加
- イベントシステムの実装
- ドキュメントの修正（未実装機能の明示）

---

## 1. CC:Tweaked コアペリフェラル

### 1.1 Modem

**実装ファイル**: `crates/rust-computers-api/src/computer_craft/modem.rs`  
**ドキュメント**: `docs/api_en/computer_craft/Modem.md`, `docs/api_ja/computer_craft/Modem.md`

#### 問題点

| 項目 | ドキュメント | 実装 | 状態 |
|------|------------|------|------|
| `async_open()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_is_open()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_close()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_close_all()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_transmit()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_is_wireless()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_transmit_raw()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_try_receive_raw()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_get_names_remote()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `modem_message` イベント | ✅ 記載あり | ❌ 未実装 | **不整合** |

#### 実装状況

**実装済み**:
- `book_next_open()` / `read_last_open()` ✅
- `book_next_is_open()` / `read_last_is_open()` ✅
- `book_next_close()` / `read_last_close()` ✅
- `book_next_close_all()` / `read_last_close_all()` ✅
- `book_next_transmit()` / `read_last_transmit()` ✅
- `book_next_transmit_raw()` / `read_last_transmit_raw()` ✅
- `book_next_try_receive_raw()` / `read_last_try_receive_raw()` ✅
- `receive_wait_raw()` (async) ✅

**未実装**:
- 全メソッドの `async_*` バリアント（`receive_wait_raw`を除く）
- `modem_message` イベントの監視機能
- `is_wireless()` メソッド全体
- `get_names_remote()` メソッド全体

#### 必要な作業

1. 全メソッドに `async_*` バリアントを追加
2. `is_wireless()` メソッドの実装
3. `get_names_remote()` メソッドの実装
4. `modem_message` イベントシステムの実装

---

### 1.2 Inventory

**実装ファイル**: `crates/rust-computers-api/src/computer_craft/inventory.rs`  
**ドキュメント**: `docs/api_en/computer_craft/Inventory.md`, `docs/api_ja/computer_craft/Inventory.md`

#### 問題点

| 項目 | ドキュメント | 実装 | 状態 |
|------|------------|------|------|
| `async_size()` | ✅ 記載あり | ✅ 実装済み | **整合** |
| `async_list()` | ✅ 記載あり | ✅ 実装済み | **整合** |
| `async_get_item_detail()` | ✅ 記載あり | ✅ 実装済み | **整合** |
| `async_push_items()` | ✅ 記載あり | ✅ 実装済み | **整合** |
| `async_pull_items()` | ✅ 記載あり | ✅ 実装済み | **整合** |
| `async_get_item_limit()` | ✅ 記載あり | ❌ 未実装 | **不整合** |

#### 実装状況

**実装済み**:
- `book_next_size()` / `read_last_size()` / `async_size()` ✅
- `book_next_list()` / `read_last_list()` / `async_list()` ✅
- `book_next_get_item_detail()` / `read_last_get_item_detail()` / `async_get_item_detail()` ✅
- `book_next_push_items()` / `read_last_push_items()` / `async_push_items()` ✅
- `book_next_pull_items()` / `read_last_pull_items()` / `async_pull_items()` ✅

**未実装**:
- `get_item_limit()` メソッド全体

#### 必要な作業

1. `get_item_limit()` メソッドの実装（book_next/read_last/async）

#### 注意点

- Inventoryは比較的整合性が高い
- `push_items` / `pull_items` のシグネチャがドキュメントと異なる（実装では `&Inventory` を受け取るが、ドキュメントでは `&str` を期待）

---

### 1.3 Monitor

**実装ファイル**: `crates/rust-computers-api/src/computer_craft/monitor.rs`  
**ドキュメント**: `docs/api_en/computer_craft/Monitor.md`, `docs/api_ja/computer_craft/Monitor.md`

#### 問題点

| 項目 | ドキュメント | 実装 | 状態 |
|------|------------|------|------|
| `async_set_text_scale()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_get_text_scale()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_write()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_clear()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_clear_line()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_scroll()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_get_cursor_pos()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_set_cursor_pos()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_get_cursor_blink()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_set_cursor_blink()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_get_size()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_set_text_color()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_get_text_color()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_set_background_color()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_get_background_color()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_blit()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `monitor_resize` イベント | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `monitor_touch` イベント | ✅ 記載あり | 🟡 部分実装 | **部分不整合** |

#### 実装状況

**実装済み**:
- 全メソッドの `book_next_*` / `read_last_*` ✅
- `*_imm()` メソッド（即時実行バリアント）✅
- `poll_touch()` (async) ✅ - タッチイベント用
- `book_next_try_poll_touch()` / `read_last_try_poll_touch()` ✅

**未実装**:
- 全メソッドの `async_*` バリアント（`poll_touch`を除く）
- `monitor_resize` イベント

#### 特記事項

- `*_imm()` メソッドが実装されているが、ドキュメントには記載なし
- `blit()` のシグネチャがドキュメントと異なる（実装では `MonitorColor` を受け取るが、ドキュメントでは `&str` を期待）

#### 必要な作業

1. 全メソッドに `async_*` バリアントを追加
2. `monitor_resize` イベントの実装
3. ドキュメントに `*_imm()` メソッドを追加するか、実装から削除するか決定
4. `blit()` のシグネチャ統一

---

### 1.4 Speaker

**実装ファイル**: `crates/rust-computers-api/src/computer_craft/speaker.rs`  
**ドキュメント**: `docs/api_en/computer_craft/Speaker.md`, `docs/api_ja/computer_craft/Speaker.md`

#### 問題点

| 項目 | ドキュメント | 実装 | 状態 |
|------|------------|------|------|
| `async_play_note()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_play_sound()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `async_stop()` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `speaker_audio_empty` イベント | ✅ 記載あり | ❌ 未実装 | **不整合** |

#### 実装状況

**実装済み**:
- `book_next_play_note()` / `read_last_play_note()` ✅
- `book_next_play_sound()` / `read_last_play_sound()` ✅
- `book_next_stop()` / `read_last_stop()` ✅

**未実装**:
- 全メソッドの `async_*` バリアント
- `speaker_audio_empty` イベント

#### 必要な作業

1. 全メソッドに `async_*` バリアントを追加
2. `speaker_audio_empty` イベントの実装

---

## 2. AdvancedPeripherals

### 2.1 MEBridge

**実装ファイル**: `crates/rust-computers-api/src/advanced_peripherals/me_bridge.rs`  
**ドキュメント**: `docs/api_en/advanced_peripherals/MEBridge.md`, `docs/api_ja/advanced_peripherals/MEBridge.md`

#### 問題点

- **全メソッドで `async_*` バリアントが未実装**
- ドキュメントでは3つの関数パターンを謳っているが、実装では `book_next_*` / `read_last_*` のみ

#### 実装状況

**実装済み**:
- 全メソッドの `book_next_*` / `read_last_*` ✅（約60メソッド）

**未実装**:
- 全メソッドの `async_*` バリアント（約60メソッド）

#### 必要な作業

1. 全メソッドに `async_*` バリアントを追加（大規模作業）

---

### 2.2 PlayerDetector

**実装ファイル**: `crates/rust-computers-api/src/advanced_peripherals/player_detector.rs`  
**ドキュメント**: `docs/api_en/advanced_peripherals/PlayerDetector.md`, `docs/api_ja/advanced_peripherals/PlayerDetector.md`

#### 問題点

| 項目 | ドキュメント | 実装 | 状態 |
|------|------------|------|------|
| 全メソッドの `async_*` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `playerJoin` イベント | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `playerLeave` イベント | ✅ 記載あり | ❌ 未実装 | **不整合** |

#### 実装状況

**実装済み**:
- 全メソッドの `book_next_*` / `read_last_*` ✅

**未実装**:
- 全メソッドの `async_*` バリアント
- `playerJoin` / `playerLeave` イベント

#### 必要な作業

1. 全メソッドに `async_*` バリアントを追加
2. `playerJoin` / `playerLeave` イベントの実装

---

### 2.3 ChatBox

**実装ファイル**: `crates/rust-computers-api/src/advanced_peripherals/chat_box.rs`  
**ドキュメント**: `docs/api_en/advanced_peripherals/ChatBox.md`, `docs/api_ja/advanced_peripherals/ChatBox.md`

#### 問題点

| 項目 | ドキュメント | 実装 | 状態 |
|------|------------|------|------|
| 全メソッドの `async_*` | ✅ 記載あり | ❌ 未実装 | **不整合** |
| `chat` イベント | ✅ 記載あり | ❌ 未実装 | **不整合** |

#### 実装状況

**実装済み**:
- 全メソッドの `book_next_*` / `read_last_*` ✅

**未実装**:
- 全メソッドの `async_*` バリアント
- `chat` イベント

#### 必要な作業

1. 全メソッドに `async_*` バリアントを追加
2. `chat` イベントの実装

---

### 2.4 その他のAdvancedPeripherals

**BlockReader**, **GeoScanner**, **EnvironmentDetector**, **EnergyDetector** なども同様の問題を抱えている：

- `book_next_*` / `read_last_*` は実装済み
- `async_*` バリアントが未実装

---

## 3. その他のMod

### 3.1 Create

**実装状況**: 部分実装  
**問題点**: 同様に `async_*` バリアントが未実装

### 3.2 Control-Craft

**実装状況**: 実装済み（ファイルは存在）  
**問題点**: 同様に `async_*` バリアントが未実装

### 3.3 Clockwork CC Compat

**実装状況**: 実装済み（ファイルは存在）  
**問題点**: 同様に `async_*` バリアントが未実装

---

## 4. 根本的な問題

### 4.1 async_*メソッドの欠落

**問題**: ドキュメントでは「3つの関数パターン」を謳っているが、実装では大半のペリフェラルで `async_*` メソッドが存在しない。

**影響**:
- ユーザーがドキュメント通りに `async_*` メソッドを使おうとするとコンパイルエラー
- `book_next` / `read_last` パターンを手動で実装する必要がある
- ユーザーエクスペリエンスの低下

**推奨対応**:
1. **Option A**: 全ペリフェラルに `async_*` メソッドを追加（大規模作業）
2. **Option B**: ドキュメントから `async_*` の記載を削除し、「将来実装予定」と明記

---

### 4.2 イベントシステムの未実装

**問題**: 以下のイベントがドキュメントに記載されているが、実装されていない：

- `modem_message` (Modem)
- `monitor_resize` (Monitor)
- `monitor_touch` (Monitor) - 部分実装のみ
- `chat` (ChatBox)
- `playerJoin` / `playerLeave` (PlayerDetector)
- `speaker_audio_empty` (Speaker)

**影響**:
- イベント駆動プログラミングができない
- Luaとの互換性が低い
- 非同期処理の実装が困難

**推奨対応**:
1. Java側のイベント監視フレームワークを実装
2. Rust側のイベントポーリングAPIを実装
3. 各ペリフェラルにイベント関連メソッドを追加

---

### 4.3 ドキュメントの過剰記載

**問題**: ドキュメントが実装されていない機能を記載している。

**例**:
- 全ての `async_*` メソッド
- イベント機能
- 一部のメソッド（`is_wireless()`, `get_names_remote()`, `get_item_limit()` など）

**推奨対応**:
1. ドキュメントに「実装状況」セクションを追加
2. 未実装機能に「🚧 未実装」マークを付ける
3. 実装済み機能のみを記載する「クイックリファレンス」を作成

---

## 5. 優先度付けされた修正タスク

### 優先度: 高（Critical）

1. **Modemイベントシステム** - ネットワーク通信に必須
2. **async_*メソッドの追加（CC:Tweaked コア）** - 基本的なペリフェラルから
3. **ドキュメント修正** - 未実装機能の明示

### 優先度: 中（High）

4. **Monitorイベントシステム** - タッチ操作に必要
5. **async_*メソッドの追加（AdvancedPeripherals）** - MEBridge, PlayerDetector, ChatBox
6. **欠落メソッドの実装** - `is_wireless()`, `get_names_remote()`, `get_item_limit()`

### 優先度: 低（Medium）

7. **async_*メソッドの追加（その他のMod）** - Create, Control-Craft, Clockwork
8. **その他のイベント** - `speaker_audio_empty`, `playerJoin`, `playerLeave`
9. **シグネチャの統一** - `blit()`, `push_items()`, `pull_items()`

---

## 6. 推奨実装戦略

### Phase 6.1: ドキュメント修正（1-2日）

1. 全ドキュメントに実装状況を明記
2. 未実装機能に警告を追加
3. クイックリファレンスの作成

### Phase 6.2: async_*メソッド追加（1-2週間）

1. マクロベースの自動生成を検討
2. CC:Tweaked コアから実装
3. AdvancedPeripherals、その他のModへ展開

### Phase 6.3: イベントシステム実装（2-3週間）

1. Java側のイベント監視フレームワーク
2. Rust側のイベントポーリングAPI
3. 各ペリフェラルへの統合

### Phase 6.4: 欠落メソッド実装（1週間）

1. `is_wireless()`, `get_names_remote()`, `get_item_limit()` など
2. シグネチャの統一

---

## 7. 結論

現在の実装とドキュメントの間には**重大な不整合**が存在します。特に：

1. **async_*メソッドの欠落** - 全ペリフェラルで未実装
2. **イベントシステムの未実装** - 重要な機能が欠けている
3. **ドキュメントの過剰記載** - 実装されていない機能を記載

これらの問題を解決するために、**Phase 6で大規模な修正作業が必要**です。

**推奨アクション**:
1. まずドキュメントを修正し、現状を正確に反映
2. async_*メソッドを段階的に追加
3. イベントシステムを実装
4. 欠落メソッドを補完

**作業量見積もり**: 4-6週間（フルタイム換算）

---

**評価完了日**: 2025-01-XX  
**次のアクション**: Phase 6タスクリストの作成
