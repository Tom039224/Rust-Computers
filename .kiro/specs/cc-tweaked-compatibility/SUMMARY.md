# CC:Tweaked互換性改善プロジェクト - 変更内容まとめ (v0.3.0)

## プロジェクト概要

RustComputersのCC:Tweaked互換性を大幅に改善するプロジェクト。
v0.2.x系の一連のコミット（v0.2.51〜v0.2.67）を経て、v0.3.0としてリリース。

**期間**: 2025年3月〜2025年7月  
**バージョン**: v0.3.0  
**コミット数**: 約17コミット（v0.2.51〜v0.2.67）

---

## Phase 1: Research & Specification（仕様調査）

### 実施内容

11個の互換対象modについて仕様調査を実施し、APIの過不足を明確化した。

**調査対象mod:**
- CC:Tweaked（コアペリフェラル）
- AdvancedPeripherals（12種のペリフェラル）
- CC-VS（Valkyrien Skies連携）
- Clockwork CC Compat（蒸気機関系）
- Control-Craft（飛行制御系）
- Create（工業mod連携）
- Create Additions（電力系）
- Some-Peripherals（各種センサー）
- Toms-Peripherals（環境センサー）
- CBC CC Control（キャノン制御）
- VS-Addition（VS2拡張）

### 成果物

- `docs/lua_api/COMPATIBILITY_MATRIX.md` - 互換性マトリックス
- `docs/lua_api/API_GAP_ANALYSIS.md` - APIの過不足リスト
- `docs/lua_api/{mod_name}/` - 各modの仕様ドキュメント

---

## Phase 2: Java Side Implementation（Java側実装）

### 実施内容

Java側のペリフェラルブリッジを拡張し、以下を実装した。

**フレームワーク:**
- `PeripheralProvider` 拡張設計
- イベント監視フレームワーク（`EventMonitor`クラス、イベントキュー、Tick-basedポーリング）
- 複数リクエスト・複数回答対応

**CC:Tweaked コアペリフェラル:**
- `Inventory` - インベントリ操作（`list`, `getItem`, `size`, `pushItems`, `pullItems`, `getItemLimit`）
- `Modem` - ネットワーク通信（`open`, `close`, `transmit`, `isWireless`, `getNamesRemote`）
- `Monitor` - 画面表示（`write`, `clear`, `setTextScale`, `getSize` 等）
- `Speaker` - 音声出力（`playNote`, `playSound`, `stop`）

**AdvancedPeripherals:**
- `BlockReader` - ブロック情報読取
- `ChatBox` - チャット監視・送信
- `MEBridge` - Applied Energistics連携
- `PlayerDetector` - プレイヤー検出
- `GeoScanner` - 地形スキャン

**Create:**
- `DisplayLink` - ディスプレイリンク
- `Station` - 列車ステーション

---

## Phase 3: Rust API Implementation（Rust側実装）

### 実施内容

Rust側のペリフェラルラッパーを全mod分実装した。

**設計パターン（3関数ペア）:**

```rust
// 1. book_next_* - リクエスト予約（FFI呼び出しなし）
sensor.book_next_get_data();

// 2. read_last_* - 前tickの結果読み取り
let data = sensor.read_last_get_data()?;

// 3. async_* - async/await対応（内部でbook→wait→readをループ）
let data = sensor.async_get_data().await?;
```

**実装ファイル:**

| mod | ディレクトリ | ペリフェラル数 |
|-----|------------|-------------|
| CC:Tweaked | `src/computer_craft/` | 4 |
| AdvancedPeripherals | `src/advanced_peripherals/` | 12 |
| Create | `src/create/` | 18 |
| Create Additions | `src/createaddition/` | 5 |
| Control-Craft | `src/control_craft/` | 14 |
| Clockwork CC Compat | `src/clockwork_cc_compat/` | 13 |
| Some-Peripherals | `src/some_peripherals/` | 6 |
| Toms-Peripherals | `src/toms_peripherals/` | 7 |
| CBC CC Control | `src/cbc_cc_control/` | 1 |
| CC-VS | `src/cc_vs/` | 3 |

**合計: 83ペリフェラル実装**

---

## Phase 4: Documentation（ドキュメント作成）

### 実施内容

全modのAPIリファレンスを英語・日本語で作成し、チュートリアルも整備した。

**APIリファレンス（英語）:** `docs/api_en/{mod_name}/{Peripheral}.md`
**APIリファレンス（日本語）:** `docs/api_ja/{mod_name}/{Peripheral}.md`

**チュートリアル（日本語）:**
- `docs/TUTORIAL_RUST_BOOK_READ_PATTERN_JA.md` - book-readパターン解説
- `docs/TUTORIAL_RUST_ASYNC_PATTERN_JA.md` - asyncパターン解説
- `docs/TUTORIAL_RUST_EVENT_HANDLING_JA.md` - イベント処理解説
- `docs/TUTORIAL_RUST_CONCURRENT_PERIPHERALS_JA.md` - 複数ペリフェラル並行操作

**削除:** 不要なLuaAPIドキュメント（`docs/lua_api/`は実装用リファレンスとして維持）

---

## Phase 5: Evaluating（評価）

### 実施内容

ドキュメントと実装・互換先の整合性を評価し、Phase 6のタスクを特定した。

**評価結果:**
1. **実装との整合性**: 多くのメソッドに`async_*`バリアントが不足していることを確認
2. **互換先との整合性**: 一部のシグネチャが互換先のLua APIと異なることを確認
3. **未実装機能**: イベントシステムのRust側実装が未完了であることを確認

---

## Phase 6: ReCompleting（再完成）

### 6.1 ドキュメント修正

全ドキュメントに実装状況セクションを追加し、未実装機能を明示した。

- 全ドキュメントに「実装状況」テーブルを追加
- 未実装機能に「🚧 未実装」マークを追加
- `QUICK_REFERENCE.md`を実装済み機能のみに更新

### 6.2 CC:Tweaked コアペリフェラル - async_*メソッド追加

全コアペリフェラルに`async_*`バリアントを追加した。

**Modem（7メソッド）:**
- `async_open()`, `async_is_open()`, `async_close()`, `async_close_all()`
- `async_transmit()`, `async_transmit_raw()`, `async_try_receive_raw()`
- 欠落メソッド追加: `is_wireless()`, `get_names_remote()`

**Inventory（欠落メソッド追加）:**
- `get_item_limit()` (book_next/read_last/async)

**Monitor（16メソッド）:**
- `async_set_text_scale()`, `async_get_text_scale()`, `async_write()`, `async_clear()`
- `async_clear_line()`, `async_scroll()`, `async_get_cursor_pos()`, `async_set_cursor_pos()`
- `async_get_cursor_blink()`, `async_set_cursor_blink()`, `async_get_size()`
- `async_set_text_color()`, `async_get_text_color()`, `async_set_background_color()`
- `async_get_background_color()`, `async_blit()`

**Speaker（3メソッド）:**
- `async_play_note()`, `async_play_sound()`, `async_stop()`

### 6.3 イベントシステム実装

**Java側（完了）:**
- `EventMonitor`クラスの実装
- イベントキューの実装
- Tick-basedイベントポーリング

**Rust側（部分完了）:**
- Modemのイベントメソッド実装:
  - `book_next_receive_event()`
  - `read_last_receive_event()`
  - `async_receive_event()`
- その他ペリフェラルのイベント実装は将来対応

### 6.4 AdvancedPeripherals - async_*メソッド追加

12種全ペリフェラルに`async_*`バリアントを追加:
- `MEBridge`, `RSBridge`, `PlayerDetector`, `ChatBox`, `BlockReader`, `GeoScanner`
- `EnvironmentDetector`, `EnergyDetector`, `ColonyIntegrator`, `Compass`
- `InventoryManager`, `NbtStorage`

### 6.5 その他のMod - async_*メソッド追加

7mod全ペリフェラルに`async_*`バリアントを追加:
- Create（18ペリフェラル）
- Control-Craft（14ペリフェラル）
- Clockwork CC Compat（12ペリフェラル、gas_network.rsはimm専用のため対象外）
- Some-Peripherals（6ペリフェラル）
- Toms-Peripherals（7ペリフェラル）
- CBC CC Control（1ペリフェラル）
- CC-VS（3ペリフェラル）

### 6.6 シグネチャ統一とAPI修正

- `Monitor.blit()`: シグネチャ確認済み（`&str`引数は適切）
- `Inventory.push_items()/pull_items()`: `Peripheral` traitを使った型安全なシグネチャに統一
- 方向引数の型統一: `&str` → `Direction` enum（`redstone_port.rs`, `digital_adapter.rs`）
  - `Direction::as_str()`メソッドを`peripheral.rs`に追加

### 6.8 未実装ペリフェラルの調査

- **AdvancedPeripherals ChunkyTurtle**: `@LuaFunction`メソッドなし。Turtle upgradeのみ（チャンクロード）。Lua APIなし。ドキュメント作成済み。
- **VS-Addition**: 新規ペリフェラルなし。既存CC:Tweaked/Clockwork機能へのMixinのみ。実装対象なし。

---

## 未完了・将来対応事項

以下の項目は将来のバージョンで対応予定:

1. **Rust側イベントポーリングAPI**: `book_next_*_event()`, `read_last_*_event()`, `async_*_event()`の汎用パターン
2. **Monitor resizeイベント**: `monitor_resize`イベントのRust実装
3. **ChatBoxイベント**: `chat`イベントのRust実装
4. **PlayerDetectorイベント**: `playerJoin`/`playerLeave`イベントのRust実装
5. **Speakerイベント**: `speaker_audio_empty`イベントのRust実装
6. **`*_imm()`メソッドのドキュメント化**: 現在未文書化
7. **自動テスト**: 実機テストが必要なため自動テストは対象外

---

## 変更ファイル一覧（主要）

### Rust実装
- `crates/rust-computers-api/src/computer_craft/` - CC:Tweakedペリフェラル
- `crates/rust-computers-api/src/advanced_peripherals/` - AdvancedPeripherals
- `crates/rust-computers-api/src/create/` - Createペリフェラル
- `crates/rust-computers-api/src/createaddition/` - Create Additionsペリフェラル
- `crates/rust-computers-api/src/control_craft/` - Control-Craftペリフェラル
- `crates/rust-computers-api/src/clockwork_cc_compat/` - Clockwork CC Compatペリフェラル
- `crates/rust-computers-api/src/some_peripherals/` - Some-Peripheralsペリフェラル
- `crates/rust-computers-api/src/toms_peripherals/` - Toms-Peripheralsペリフェラル
- `crates/rust-computers-api/src/cbc_cc_control/` - CBC CC Controlペリフェラル
- `crates/rust-computers-api/src/cc_vs/` - CC-VSペリフェラル
- `crates/rust-computers-api/src/peripheral.rs` - `Direction` enum追加

### ドキュメント
- `docs/api_en/` - 英語APIリファレンス（全mod）
- `docs/api_ja/` - 日本語APIリファレンス（全mod）
- `docs/TUTORIAL_RUST_*.md` - Rustチュートリアル（4本）
- `docs/lua_api/COMPATIBILITY_MATRIX.md` - 互換性マトリックス

---

## 成果サマリー

| 項目 | 結果 |
|------|------|
| 対応mod数 | 11 |
| 実装ペリフェラル数 | 83 |
| APIメソッド数（概算） | 500+ |
| ドキュメントファイル数 | 100+ |
| チュートリアル数 | 4 |
| async_*バリアント追加 | 全ペリフェラル対応 |
| イベントシステム | Java側完了、Rust側部分完了 |

---

**バージョン**: v0.3.0  
**リリース日**: 2025年7月  
**次バージョン予定**: v0.4.0（イベントシステム完全実装）
