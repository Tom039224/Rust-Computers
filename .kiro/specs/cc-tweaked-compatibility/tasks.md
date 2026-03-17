# Tasks: CC:Tweaked互換性改善プロジェクト

## Phase 1: Research & Specification

- [x] 1.1 CC:Tweaked Lua API仕様調査
- [x] 1.2 AdvancedPeripherals仕様調査
- [x] 1.3 CC-VS仕様調査
- [x] 1.4 Clockwork CC Compat仕様調査
- [x] 1.5 Control-Craft仕様調査
- [x] 1.6 Create仕様調査
- [x] 1.7 Create Additions仕様調査
- [x] 1.8 Some-Peripherals仕様調査
- [x] 1.9 Toms-Peripherals仕様調査
- [x] 1.10 CBC CC Control仕様調査
- [x] 1.11 VS-Addition仕様調査
- [x] 1.12 互換性マトリックス作成（docs/lua_api/COMPATIBILITY_MATRIX.md）
- [x] 1.13 APIの過不足リスト作成

## Phase 2: Java Side Implementation

- [x] 2.1 PeripheralProvider拡張設計
- [x] 2.2 イベント監視フレームワーク実装
- [x] 2.3 複数リクエスト・複数回答対応
- [x] 2.4 CC:Tweaked Inventory実装
- [x] 2.5 CC:Tweaked Modem実装（イベント対応）
- [x] 2.6 CC:Tweaked Monitor実装（イベント対応）
- [x] 2.7 CC:Tweaked Speaker実装
- [x] 2.8 AdvancedPeripherals BlockReader実装
- [x] 2.9 AdvancedPeripherals ChatBox実装
- [x] 2.10 AdvancedPeripherals MEBridge実装
- [x] 2.11 AdvancedPeripherals PlayerDetector実装
- [x] 2.12 AdvancedPeripherals GeoScanner実装
- [x] 2.13 Create DisplayLink実装
- [x] 2.14 Create Station実装
- [x] 2.15 その他優先度の高いペリフェラル実装

## Phase 3: Rust API Implementation

- [x] 3.1 ペリフェラルラッパー生成フレームワーク設計
- [x] 3.2 book_next/read_last/async マクロ実装
- [x] 3.3 CC:Tweaked Inventory Rust実装
- [x] 3.4 CC:Tweaked Modem Rust実装
- [x] 3.5 CC:Tweaked Monitor Rust実装
- [x] 3.6 CC:Tweaked Speaker Rust実装
- [x] 3.7 AdvancedPeripherals BlockReader Rust実装
- [x] 3.8 AdvancedPeripherals ChatBox Rust実装
- [x] 3.9 AdvancedPeripherals MEBridge Rust実装
- [x] 3.10 AdvancedPeripherals PlayerDetector Rust実装
- [x] 3.11 AdvancedPeripherals GeoScanner Rust実装
- [x] 3.12 Create DisplayLink Rust実装
- [x] 3.13 Create Station Rust実装
- [x] 3.14 その他優先度の高いペリフェラル Rust実装

## Phase 4: Documentation

- [x] 4.1 CC:Tweaked Rust-APIリファレンス作成（英語）
  終了後、下記のnoteにしたがってコミット
- [x] 4.2 CC:Tweaked Rust-APIリファレンス作成（日本語）
  終了後、下記のnoteにしたがってコミット
- [x] 4.3 AdvancedPeripherals Rust-APIリファレンス作成（英語）
  終了後、下記のnoteにしたがってコミット
- [x] 4.4 AdvancedPeripherals Rust-APIリファレンス作成（日本語）
  終了後、下記のnoteにしたがってコミット
- [x] 4.5 Create Rust-APIリファレンス作成（英語）
  終了後、下記のnoteにしたがってコミット
- [x] 4.6 Create Rust-APIリファレンス作成（日本語）
  終了後、下記のnoteにしたがってコミット
- [x] 4.7 その他modのRust-APIリファレンス作成
  終了後、下記のnoteにしたがってコミット
- [x] 4.8 Rust-book-read パターンチュートリアル作成
  終了後、下記のnoteにしたがってコミット
- [x] 4.9 Rust-async パターンチュートリアル作成
  終了後、下記のnoteにしたがってコミット
- [x] 4.10 Rust-イベント処理チュートリアル作成
  終了後、下記のnoteにしたがってコミット
- [x] 4.11 Rust-複数ペリフェラル並行操作チュートリアル作成
  終了後、下記のnoteにしたがってコミット

- [x] 4.12 ドキュメントに必要なのはRustComputers用RustAPIのみなので不要なLuaAPIのドキュメントを削除(docs/lua_apiは実装用のリファレンスなので維持)

## Phase 5: Evaluating
- [x] 5.1 現在のドキュメントと実際のRustComputersの実装の整合性を評価し、必要に応じてフェーズ6にタスクを再度追加
- [x] 5.2 現在のドキュメントと実際の互換先(refs/compatible_mods)の整合性を評価し、必要に応じてフェーズ6にタスクを再度追加 (元𝐭𝐨𝐦)

## Phase 6: ReCompleting

### 6.1 ドキュメント修正（Documentation Fixes）
- [x] 6.1.1 全ドキュメントに実装状況セクションを追加（英語・日本語）
- [x] 6.1.2 未実装機能に「🚧 未実装」マークを追加
- [x] 6.1.3 QUICK_REFERENCE.mdを更新し、実装済み機能のみを記載
- [x] 6.1.4 各ペリフェラルドキュメントに「実装状況」テーブルを追加

### 6.2 CC:Tweaked コアペリフェラル - async_*メソッド追加
- [ ] 6.2.1 Modem: 全メソッドにasync_*バリアントを追加
  - [x] 6.2.1.1 async_open()
  - [x] 6.2.1.2 async_is_open()
  - [x] 6.2.1.3 async_close()
  - [x] 6.2.1.4 async_close_all()
  - [x] 6.2.1.5 async_transmit()
  - [x] 6.2.1.6 async_transmit_raw()
  - [x] 6.2.1.7 async_try_receive_raw()
- [ ] 6.2.2 Modem: 欠落メソッドの実装
  - [x] 6.2.2.1 is_wireless() (book_next/read_last/async)
  - [x] 6.2.2.2 get_names_remote() (book_next/read_last/async)
- [ ] 6.2.3 Inventory: 欠落メソッドの実装
  - [x] 6.2.3.1 get_item_limit() (book_next/read_last/async)
- [x] 6.2.4 Monitor: 全メソッドにasync_*バリアントを追加
  - [x] 6.2.4.1 async_set_text_scale()
  - [x] 6.2.4.2 async_get_text_scale()
  - [x] 6.2.4.3 async_write()
  - [x] 6.2.4.4 async_clear()
  - [x] 6.2.4.5 async_clear_line()
  - [x] 6.2.4.6 async_scroll()
  - [x] 6.2.4.7 async_get_cursor_pos()
  - [x] 6.2.4.8 async_set_cursor_pos()
  - [x] 6.2.4.9 async_get_cursor_blink()
  - [x] 6.2.4.10 async_set_cursor_blink()
  - [x] 6.2.4.11 async_get_size()
  - [x] 6.2.4.12 async_set_text_color()
  - [x] 6.2.4.13 async_get_text_color()
  - [x] 6.2.4.14 async_set_background_color()
  - [x] 6.2.4.15 async_get_background_color()
  - [x] 6.2.4.16 async_blit()
- [x] 6.2.5 Speaker: 全メソッドにasync_*バリアントを追加
  - [x] 6.2.5.1 async_play_note()
  - [x] 6.2.5.2 async_play_sound()
  - [x] 6.2.5.3 async_stop()

### 6.3 イベントシステム実装（Event System Implementation）
- [x] 6.3.1 Java側: イベント監視フレームワーク実装
  - [ ] 6.3.1.1 EventMonitorクラスの実装
  - [ ] 6.3.1.2 イベントキューの実装
  - [ ] 6.3.1.3 Tick-basedイベントポーリング
- [ ] 6.3.2 Rust側: イベントポーリングAPI実装
  - [ ] 6.3.2.1 book_next_*_event() メソッドパターン
  - [ ] 6.3.2.2 read_last_*_event() メソッドパターン
  - [ ] 6.3.2.3 async_*_event() メソッドパターン
- [ ] 6.3.3 Modem: modem_messageイベント実装
  - [ ] 6.3.3.1 Java側: ModemMessageEventの監視
  - [ ] 6.3.3.2 Rust側: book_next_receive_event()
  - [ ] 6.3.3.3 Rust側: read_last_receive_event()
  - [ ] 6.3.3.4 Rust側: async_receive_event()
- [ ] 6.3.4 Monitor: monitor_resizeイベント実装
  - [ ] 6.3.4.1 Java側: MonitorResizeEventの監視
  - [ ] 6.3.4.2 Rust側: イベントポーリングメソッド
- [ ] 6.3.5 ChatBox: chatイベント実装
  - [ ] 6.3.5.1 Java側: ChatEventの監視
  - [ ] 6.3.5.2 Rust側: イベントポーリングメソッド
- [ ] 6.3.6 PlayerDetector: playerJoin/playerLeaveイベント実装
  - [ ] 6.3.6.1 Java側: PlayerJoin/LeaveEventの監視
  - [ ] 6.3.6.2 Rust側: イベントポーリングメソッド
- [ ] 6.3.7 Speaker: speaker_audio_emptyイベント実装
  - [ ] 6.3.7.1 Java側: SpeakerAudioEmptyEventの監視
  - [ ] 6.3.7.2 Rust側: イベントポーリングメソッド

### 6.4 AdvancedPeripherals - async_*メソッド追加
- [ ] 6.4.1 MEBridge: 全メソッドにasync_*バリアントを追加（約60メソッド）
  - [ ] 6.4.1.1 マクロベースの自動生成を検討
  - [ ] 6.4.1.2 async_list_items(), async_get_item(), etc.
- [ ] 6.4.2 PlayerDetector: 全メソッドにasync_*バリアントを追加
- [ ] 6.4.3 ChatBox: 全メソッドにasync_*バリアントを追加
- [ ] 6.4.4 BlockReader: 全メソッドにasync_*バリアントを追加
- [ ] 6.4.5 GeoScanner: 全メソッドにasync_*バリアントを追加
- [ ] 6.4.6 EnvironmentDetector: 全メソッドにasync_*バリアントを追加
- [ ] 6.4.7 EnergyDetector: 全メソッドにasync_*バリアントを追加
- [ ] 6.4.8 その他のAdvancedPeripherals: async_*バリアントを追加

### 6.5 その他のMod - async_*メソッド追加
- [ ] 6.5.1 Create: 全ペリフェラルにasync_*バリアントを追加
- [ ] 6.5.2 Control-Craft: 全ペリフェラルにasync_*バリアントを追加
- [ ] 6.5.3 Clockwork CC Compat: 全ペリフェラルにasync_*バリアントを追加
- [ ] 6.5.4 Some-Peripherals: 全ペリフェラルにasync_*バリアントを追加
- [ ] 6.5.5 Toms-Peripherals: 全ペリフェラルにasync_*バリアントを追加
- [ ] 6.5.6 CBC CC Control: 全ペリフェラルにasync_*バリアントを追加
- [ ] 6.5.7 CC-VS: 全ペリフェラルにasync_*バリアントを追加

### 6.6 シグネチャ統一とAPI修正
- [ ] 6.6.1 Monitor.blit(): シグネチャをドキュメントと統一
- [ ] 6.6.2 Inventory.push_items()/pull_items(): シグネチャをドキュメントと統一
- [ ] 6.6.3 *_imm()メソッドのドキュメント化または削除の決定
- [ ] 6.6.4 全ペリフェラルのシグネチャ一貫性チェック

### 6.7 テストとバリデーション
- [ ] 6.7.1 async_*メソッドの単体テスト作成
- [ ] 6.7.2 イベントシステムの統合テスト作成
- [ ] 6.7.3 ドキュメントと実装の整合性チェック自動化
- [ ] 6.7.4 全ペリフェラルの動作確認

### 6.8 未実装ペリフェラルの追加
- [ ] 6.8.1 AdvancedPeripherals Chunkyペリフェラルの実装
  - [ ] 6.8.1.1 Java側: ChunkyPeripheralブリッジ実装
  - [ ] 6.8.1.2 Rust側: Chunky構造体とメソッド実装
  - [ ] 6.8.1.3 ドキュメント作成（英語・日本語）
- [ ] 6.8.2 VS-Additionペリフェラルの調査
  - [ ] 6.8.2.1 VS-Additionソースコードの調査
  - [ ] 6.8.2.2 実装すべきペリフェラルのリスト作成
  - [ ] 6.8.2.3 実装計画の策定

### 6.9 マクロベースの自動生成検討
- [ ] 6.9.1 async_*メソッド自動生成マクロの設計
- [ ] 6.9.2 マクロ実装とテスト
- [ ] 6.9.3 既存コードへのマクロ適用


## Phase 7: Completing
- [ ] 5.3 Phase1から6までのタスクを評価し、変更内容を評価し、まとめる(日本語)+v0.3.0としてコミットし、コミット名はv0.3.0: Refactoring APIs、バージョンタグをつけること

## Notes

- 各タスクは依存関係を考慮して実行すること
- Phase 1の完了後、Phase 2と3は並行実行可能
- テストは各実装フェーズと並行して実施すること
- ドキュメントは実装と同時に作成すること
- 各タスクが終わったらv0.2.xとしてコミットすること(xはコミットログを見て判断)
  コミット名は「v0.2.x: Description in English」とすること。また、コミットにバージョンタグをつけること
