# CC:Tweaked互換性改善プロジェクト - 概要

## プロジェクト目標

RustComputersのCC:Tweaked互換性を大幅に改善し、以下を実現する：

1. **APIの完全性**: 全互換対象modの主要APIを実装
2. **イベント対応**: イベント駆動型メソッドの完全実装
3. **CC:Tweaked互換性**: Lua APIとの互換性を90%以上達成
4. **開発者体験**: 直感的で使いやすいRust APIの提供

## 現在の問題点

### 1. APIの過不足

- 実装されていないメソッドが多数存在
- 各modのペリフェラルが部分的にしか実装されていない
- 互換対象modが11個あるが、実装状況がばらばら

### 2. イベント系の未実装

- `modem.receive_raw()` などのイベント駆動メソッドが未実装
- `monitor.touch` イベントが未実装
- イベント監視フレームワークが存在しない

## 提案された解決策

### 3つの関数ペア

各ペリフェラルメソッドを3つの形式で提供：

```rust
// 1. book_next_* - リクエスト予約
sensor.book_next_get_data();

// 2. read_last_* - 結果読み取り
let data = sensor.read_last_get_data()?;

// 3. async_* - async/await対応
let data = sensor.async_get_data().await?;
```

### イベント実装

イベント系メソッドも同じ3つの形式で提供：

```rust
// 1. book_next_someevent - イベント監視開始
modem.book_next_receive_raw();

// 2. read_last_someevent - 複数リクエスト・複数回答
let events = modem.read_last_receive_raw()?;  // Vec<Option<Event>>

// 3. async_someevent - イベント受信まで待機
let event = modem.async_receive_raw().await?;
```

## 実装計画

### Phase 1: Research & Specification (1-2週間)

- 各互換対象modの仕様調査
- APIドキュメント作成
- 互換性マトリックス作成

**成果物:**
- `docs/lua_api/` - 各modの仕様ドキュメント
- `docs/lua_api/COMPATIBILITY_MATRIX.md` - 互換性マトリックス

### Phase 2: Java Side Implementation (3-4週間)

- ペリフェラルブリッジの拡張
- イベント監視フレームワーク実装
- 各modのペリフェラル実装

**成果物:**
- `forge/src/main/java/com/rustcomputers/peripheral/impl/` - Java実装

### Phase 3: Rust API Implementation (5-6週間)

- ペリフェラルラッパー実装
- 各modのペリフェラル実装
- テスト実装

**成果物:**
- `crates/rust-computers-api/src/{mod_name}/` - Rust実装

### Phase 4: Documentation (7-8週間)

- APIリファレンス作成
- チュートリアル作成
- 使用例作成

**成果物:**
- `docs/api_en/` - 英語ドキュメント
- `docs/api_ja/` - 日本語ドキュメント

### Phase 5: Testing (並行実施)

- ユニットテスト
- プロパティテスト
- 統合テスト

### Phase 6: Optimization & Polish (最終段階)

- パフォーマンス最適化
- エラーメッセージ改善
- リリース準備

## 互換対象mod一覧

| Mod | ペリフェラル数 | 実装状況 | 優先度 |
|-----|--------------|--------|--------|
| CC:Tweaked | 4 | 部分実装 | 高 |
| AdvancedPeripherals | 12 | 部分実装 | 高 |
| Create | 18 | 部分実装 | 高 |
| Some-Peripherals | 6 | 部分実装 | 中 |
| Toms-Peripherals | 7 | 部分実装 | 中 |
| Control-Craft | 14 | 未実装 | 中 |
| Create Additions | 5 | 部分実装 | 中 |
| Clockwork CC Compat | 13 | 未実装 | 低 |
| CC-VS | 3 | 最小限 | 低 |
| CBC CC Control | 1 | 最小限 | 低 |
| VS-Addition | ? | 未実装 | 低 |

## 主要な設計決定

### 1. Book-Read パターンの採用

- 1 tick = 1 ループの原則を維持
- FFI呼び出しをbatch処理で効率化
- Luaのループが進まない動作を再現

### 2. 3つの関数ペアの提供

- `book_next_*`: 低レベルAPI（パフォーマンス重視）
- `read_last_*`: 結果取得（複数操作対応）
- `async_*`: 高レベルAPI（使いやすさ重視）

### 3. イベント監視フレームワーク

- Java側でイベント監視を一元管理
- Rust側は統一されたインターフェースで利用
- 複数リクエスト・複数回答形式で対応

## 成功基準

1. **API完全性**: 全互換対象modの主要APIが実装される
2. **イベント対応**: イベント系メソッドが正常に動作する
3. **互換性**: CC:Tweaked Lua APIとの互換性が90%以上
4. **パフォーマンス**: FFI呼び出しのオーバーヘッドが許容範囲内
5. **ドキュメント**: 全APIが文書化され、使用例が提供される
6. **テスト**: 全主要機能がテストされ、カバレッジが80%以上

## リスク管理

### リスク1: 仕様の不明確性

**対策:**
- Phase 1で十分な調査を実施
- 各modの開発者に確認
- テストで動作確認

### リスク2: パフォーマンス問題

**対策:**
- FFI呼び出しの最適化
- バッチ処理の実装
- ベンチマーク測定

### リスク3: 互換性の問題

**対策:**
- 各modのバージョン確認
- 互換性マトリックスの作成
- 複数バージョンでのテスト

## 次のステップ

1. **デザインレビュー**: このドキュメントのレビュー
2. **Phase 1開始**: 各modの仕様調査開始
3. **チーム編成**: 実装チームの編成
4. **スケジュール確定**: 詳細なスケジュール作成

## 参考資料

- `design.md` - 詳細な技術設計ドキュメント
- `tasks.md` - 実装タスク一覧
- `docs/spec.md` - RustComputers全体の設計仕様
- `crates/rust-computers-api/src/` - 既存実装
- `forge/src/main/java/com/rustcomputers/peripheral/` - Java実装

---

**プロジェクト開始日**: 2025年3月
**予定完了日**: 2025年5月
**バージョン**: 0.3.0
