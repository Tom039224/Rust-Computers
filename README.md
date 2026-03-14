# RustComputers

Rustで書いたコードをMinecraft内のコンピュータで実行できるようにするForge 1.20.1（Forge 47.4.10）向けModです。
CC:TweakedのLua VMをRust/WASMに置き換えつつ、CCのJavaペリフェラルエコシステムと互換性を維持します。

## 特徴

- Rust/WASMによる高速な実行
- CC:Tweakedとの互換性を維持
- **Chicory 1.7.2**を使用した純Java実装（JNI不要）

## ドキュメント

- [docs/spec.md](docs/spec.md) — 設計仕様（日本語）
- [docs/security-report.md](docs/security-report.md) — セキュリティレポート

### APIリファレンス

- 日本語: `docs/api_ja/` 以下
- English: `docs/api_en/` 以下

### 参照元Lua API

- `docs/lua_api/` 以下

## ビルド方法

### Mod JAR のビルド

```bash
cd forge
./gradlew build
# 出力: build/libs/rustcomputers-*.jar
```

### WASM バイナリの開発フロー

1. Minecraft内でワールドを開く
2. コマンドを実行します:
   ```
   /rustcomputers get-dev <id>
   ```
3. `/world/dev/<id>/` にVS Code向けの開発環境が自動生成されます

生成された環境では `src/main.rs` を編集してビルドできます。
`Ctrl + Shift + B` などでビルドすると、ビルド後のバイナリが自動で配置されます。
画面上へドラッグ＆ドロップしてバイナリをアップロードすることも可能です。

## ライセンス

このプロジェクトは **MITライセンス** の元で公開されています。
