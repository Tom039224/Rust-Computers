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
- [-] 2.5 CC:Tweaked Modem実装（イベント対応）
- [ ] 2.6 CC:Tweaked Monitor実装（イベント対応）
- [ ] 2.7 CC:Tweaked Speaker実装
- [ ] 2.8 AdvancedPeripherals BlockReader実装
- [ ] 2.9 AdvancedPeripherals ChatBox実装
- [ ] 2.10 AdvancedPeripherals MEBridge実装
- [ ] 2.11 AdvancedPeripherals PlayerDetector実装
- [ ] 2.12 AdvancedPeripherals GeoScanner実装
- [ ] 2.13 Create DisplayLink実装
- [ ] 2.14 Create Station実装
- [ ] 2.15 その他優先度の高いペリフェラル実装

## Phase 3: Rust API Implementation

- [ ] 3.1 ペリフェラルラッパー生成フレームワーク設計
- [ ] 3.2 book_next/read_last/async マクロ実装
- [ ] 3.3 CC:Tweaked Inventory Rust実装
- [ ] 3.4 CC:Tweaked Modem Rust実装
- [ ] 3.5 CC:Tweaked Monitor Rust実装
- [ ] 3.6 CC:Tweaked Speaker Rust実装
- [ ] 3.7 AdvancedPeripherals BlockReader Rust実装
- [ ] 3.8 AdvancedPeripherals ChatBox Rust実装
- [ ] 3.9 AdvancedPeripherals MEBridge Rust実装
- [ ] 3.10 AdvancedPeripherals PlayerDetector Rust実装
- [ ] 3.11 AdvancedPeripherals GeoScanner Rust実装
- [ ] 3.12 Create DisplayLink Rust実装
- [ ] 3.13 Create Station Rust実装
- [ ] 3.14 その他優先度の高いペリフェラル Rust実装

## Phase 4: Documentation

- [ ] 4.1 CC:Tweaked APIリファレンス作成（英語）
- [ ] 4.2 CC:Tweaked APIリファレンス作成（日本語）
- [ ] 4.3 AdvancedPeripherals APIリファレンス作成（英語）
- [ ] 4.4 AdvancedPeripherals APIリファレンス作成（日本語）
- [ ] 4.5 Create APIリファレンス作成（英語）
- [ ] 4.6 Create APIリファレンス作成（日本語）
- [ ] 4.7 その他modのAPIリファレンス作成
- [ ] 4.8 book-read パターンチュートリアル作成
- [ ] 4.9 async パターンチュートリアル作成
- [ ] 4.10 イベント処理チュートリアル作成
- [ ] 4.11 複数ペリフェラル並行操作チュートリアル作成

## Phase 5: Testing

- [ ] 5.1 CC:Tweaked Inventory ユニットテスト
- [ ] 5.2 CC:Tweaked Modem ユニットテスト
- [ ] 5.3 CC:Tweaked Monitor ユニットテスト
- [ ] 5.4 AdvancedPeripherals ユニットテスト
- [ ] 5.5 Create ユニットテスト
- [ ] 5.6 Request ordering プロパティテスト
- [ ] 5.7 Action accumulation プロパティテスト
- [ ] 5.8 Event polling termination プロパティテスト
- [ ] 5.9 Tick boundary consistency プロパティテスト
- [ ] 5.10 複数ペリフェラル並行操作統合テスト
- [ ] 5.11 イベント駆動統合テスト
- [ ] 5.12 Minecraft環境での実行テスト

## Phase 6: Optimization & Polish

- [ ] 6.1 FFI呼び出しのオーバーヘッド測定
- [ ] 6.2 ペリフェラルアクセスのレイテンシ測定
- [ ] 6.3 メモリ使用量の監視
- [ ] 6.4 バッチ処理の最適化
- [ ] 6.5 キャッシング戦略の実装
- [ ] 6.6 エラーメッセージの改善
- [ ] 6.7 ドキュメントの最終レビュー
- [ ] 6.8 リリース準備

## Notes

- 各タスクは依存関係を考慮して実行すること
- Phase 1の完了後、Phase 2と3は並行実行可能
- テストは各実装フェーズと並行して実施すること
- ドキュメントは実装と同時に作成すること
