# WASMランタイム比較分析

## 概要

RustComputersで利用可能な3つのWASMランタイムを比較します。

---

## 1. Wasmtime (Bytecode Alliance)

### 基本情報
- **組織**: Bytecode Alliance（非営利 / Mozilla系）
- **開発活動**: 非常に活発（17.7k stars, 672 contributors）
- **言語**: Rust実装
- **ライセンス**: Apache-2.0
- **JIT**: ✅ Cranelift（高速で最適化されたコンパイラ）

### 長所
1. **安定性・信頼性が高い**  
   - 24/7ファジング（Google OSS-Fuzz）  
   - 厳密なセキュリティレビュー & RFC プロセス  
   - 形式検証の実績あり

2. **性能が優秀**  
   - Cranelift JIT: ネイティブに近い速度  
   - インスタンス化が高速  
   - 並行実行に最適化

3. **標準準拠**  
   - Official WASM test suite 全通過  
   - WASI対応が充実  
   - Component Modelに対応予定  
   - Proposalsを積極採用

4. **ドキュメント充実**  
   - 公式ガイド: https://docs.wasmtime.dev/  
   - 多言語バインディング（C, Python, Go, Ruby, .NET等）

### 短所
1. **Java バインディングが公式にない**  
   - wasmtime-java リポジトリは404  
   - 自分たちで JNI ラッパーを実装する必要がある

2. **JNI実装コスト**  
   - ホスト関数（Java → Wasm）の仕掛けを自作  
   - C APIは充実しているが、Java向け薄いガイド

### 技術的難易度
**中程度** — Wasmtime C API は充実しており、JNI ラッパーは作成可能だが工数がかかる

### 採用実績（参考）
- Fastly（CDN）  
- Mozilla  
- Shopify  
- 多数のクラウド・エッジ企業

---

## 2. Wasmer (Wasmer Inc.)

### 基本情報
- **組織**: Wasmer Inc.（営利企業）
- **開発活動**: 見守り中（644 stars, 最終更新 2021年1月）
- **言語**: Rust実装  
- **ライセンス**: MIT
- **JIT**: ✅ 複数コンパイラ選択可（2 Wasm, LLVM, Cranelift等）

### 長所
1. **公式 Java バインディングが存在**  
   - wasmer-java: GitHub有  
   - JAR形式で配布（CPU/OS別）  
   - 即座にビルドして利用可能

2. **複数JITバックエンド対応**  
   - Singlepass（最速）  
   - Cranelift（安定）  
   - LLVM（最適化）  
   から選択可能

3. **Wasmer Registry統合**  
   - パッケージマネジャとしても機能  
   - エコシステムが充実

### 短所
1. **保守状況が不確実**  
   - 最終リリース: 2021年1月（4年以上前）  
   - Pull requestが大幅に遅延  
   - アクティブ開発の跡が弱い

2. **数年ジャンプの懸念**  
   - セキュリティパッチが不確か  
   - WASM標準化動き対応度不明  
   - Forgeされた互換ライブラリも出現

3. **Javaバインディングも古い**  
   - Wasmer 0.3.0 (2021) に相当  
   - 最新Wasmerランタイムとの互換性不確実

### 技術的難易度
**低** — 公式 Javaバインディングで即座に利用可能  
**ただし長期的な保守リスク**

### 採用実績
- Deno（初期段階で検討）  
- 中小企業複数

---

## 3. GraalVM (Oracle)

### 基本情報
- **組織**: Oracle  
- **開発活動**: 活発（21.5k stars, 313 contributors）  
- **言語**: Java + Rust等  
- **ライセンス**: UPL 1.0 / GPL 2.0 with Classpath Exception
- **JIT**: ✅ Oracle Graal Compiler（多言語統合JIT）

### 長所
1. **Java統合が自然**  
   - Truffle Frameowkで他言語も統合実装  
   - GraalVM内部で実装済み  
   - Java ↔ WASM の相互運用がスムーズ  
   - Java Interop API直接利用可

2. **包括的な言語・機能対応**  
   - JavaScript, Python, Ruby も同じVM内で動作  
   - ポリグロット開発に有利

3. **AOT (Native Image) 対応**  
   - WASM + GraalVM binary の組み合わせ検討可  
   - スタートアップ時間短縮

4. **Oracle のリソース**  
   - エンプライズサポート有  
   - 継続的な開発体制

### 短所
1. **複雑性が高い**  
   - Truffle Framework の学習コスト  
   - GraalVM 全体の理解が必要  
   - ホスト関数実装が複雑かもしれない

2. **メモリフットプリント**  
   - 単純なWASM実行と比較すると overhead が大きい可能性  
   - Minecraft server 環境では負荷になるかもしれない

3. **ライセンス複雑**  
   - UPL + GPL dual license で商用利用時に注意必要  
   - エンタープライズ版とコミュニティ版の差がある

4. **メインスレッド依存**  
   - GraalVM の多くの機能は main thread でのみ動作  
   - マルチスレッド実行に制限がある可能性

### 技術的難易度
**高** — Truffle Framework学習必要、複雑性が高い

### 採用実績
- Oracle Cloud  
- NetflixのEdge Platform  
- Sueprmis等、大規模企業複数

---

## 比較表

| 項目 | Wasmtime | Wasmer | GraalVM |
|---|---|---|---|
| **JIT対応** | ✅ Cranelift | ✅ 複数選択 | ✅ Graal Compiler |
| **Java バインディング** | ❌ 公式なし（自作必要） | ✅ wasmer-java | ✅ GraalVM統合 |
| **保守状況** | ✅ 活発 | ⚠️ 停滞（4年） | ✅ 活発 |
| **性能** | ⭐⭐⭐⭐⭐ 最高 | ⭐⭐⭐⭐ 高い | ⭐⭐⭐⭐ 高い |
| **セキュリティ審査** | ✅ 形式検証・24/7ファジング | ⚠️ 不明 | ✅ Oracleセキュリティ |
| **ドキュメント** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ |
| **実装難度** | 中（JNI自作） | 低（公式あり） | 高（Truffle学習） |
| **エコシステム** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **メモリ効率** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **マルチスレッド** | ✅ 最適化 | ✅ 良好 | ⚠️ 制限あり |

---

## 推奨判断総合分析

### **推奨: Wasmtime via JNI**（ユーザーの想定通り）

**理由:**

1. **性能**  
   - Cranelift JIT との組み合わせで Native に近い性能  
   - Minecraft Server の tick loop での実行効率が最高

2. **セキュリティ・安定性**  
   - 形式検証 & 24/7 ファジング  
   - 長期メンテナンスの安心感  
   - バグ・セキュリティホール対応が迅速

3. **標準準拠**  
   - WASM標準化の最前線  
   - 将来的な新機能追加にも対応しやすい  
   - 他システムへの互換性最高

4. **実装は自前必要だが難しくない**  
   - Wasmtime C API が充実  
   - JNI ラッパーの実装は中程度の工数  
   - 他 Minecraft mod との互換性リスク少ない

---

## 代替案検討

### Wasmer を選ぶ場合
- **メリット**: 実装がすぐに始められる  
- **デメリット**: 4年前のバージョンしか公式サポートなし。セキュリティパッチ、新WASM機能対応未知数  
- **判定**: 短期 PoC には可。本番利用には不確実性がある

### GraalVM を選ぶ場合
- **メリット**: Java統合が自然。複数言語混在プロジェクト構想なら有利  
- **デメリット**: 複雑性高い。Minecraft server ( limited memory / CPU ) 環境では overhead の懸念。メインスレッド制約がtick loop 設計に合致しない可能性  
- **判定**: 大規模・ポリグロット案件向け。RustComputers は WASM 単一言語目指す設計なため過剰

---

## 推奨アクション

**→ Wasmtime (via JNI自作) を採用**

### 実装ロードマップ（概略）

1. **Phase 1: C Native ラッパーの実装**  
   - Wasmtime C API 使用高level wrapper (src/jni)

2. **Phase 2: JNI ブリッジ**  
   - Java → WASM 呼び出し  
   - WASM → Java ホスト関数

3. **Phase 3: 統合テスト**  
   - Minecraft tick loop 統合  
   - パフォーマンス検証

---

## 参考リンク

- Wasmtime: https://wasmtime.dev/  
- Wasmer: https://wasmer.io/  
- GraalVM: https://www.graalvm.org/
