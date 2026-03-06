package com.rustcomputers.command;

import com.mojang.brigadier.context.CommandContext;
import net.minecraft.commands.CommandSourceStack;
import net.minecraft.network.chat.Component;
import net.minecraft.server.MinecraftServer;
import net.minecraft.world.level.storage.LevelResource;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;

/**
 * {@code /rustcomputers get-dev <computerId>} コマンドの実装。
 * Implementation of the {@code /rustcomputers get-dev <computerId>} command.
 *
 * <p>指定したコンピューター ID 向けの Rust 開発環境を
 * {@code <world>/rust computers/dev/<id>/} に自動生成する。</p>
 *
 * <p>Generates a Rust development environment for the specified computer ID
 * at {@code <world>/rust computers/dev/<id>/}.</p>
 *
 * <h3>生成されるファイル / Generated files</h3>
 * <pre>
 *   dev/&lt;id&gt;/
 *   ├── dev.toml                 ← 開発レコード (コンピューター ID・使い方)
 *   ├── Cargo.toml               ← standalone bin crate
 *   ├── .cargo/
 *   │   └── config.toml          ← target = "wasm32-unknown-unknown"
 *   ├── src/
 *   │   └── main.rs              ← プログラムスキャフォールド
 *   ├── vendor/
 *   │   └── rust-computers-api/  ← JAR から抽出した API クレート
 *   └── .vscode/
 *       └── tasks.json           ← "Build &amp; Deploy" タスク
 * </pre>
 */
public final class DevEnvCommand {

    private static final Logger LOGGER = LoggerFactory.getLogger(DevEnvCommand.class);

    private DevEnvCommand() {}

    // ------------------------------------------------------------------
    // エントリーポイント / Entry point
    // ------------------------------------------------------------------

    /**
     * コマンド実行: 開発環境を生成する。
     * Execute the command: generate the dev environment.
     *
     * @param ctx        Brigadier コマンドコンテキスト / Brigadier command context
     * @param computerId 対象コンピューター ID / target computer ID
     * @return Brigadier 戻り値 (1 = 成功, 0 = 失敗) / 1 = success, 0 = failure
     */
    public static int execute(CommandContext<CommandSourceStack> ctx, int computerId) {
        CommandSourceStack source = ctx.getSource();
        MinecraftServer server = source.getServer();

        Path worldRoot = server.getWorldPath(LevelResource.ROOT);
        Path devDir = worldRoot
                .resolve("rust computers")
                .resolve("dev")
                .resolve(String.valueOf(computerId));
        Path computerDir = worldRoot
                .resolve("rust computers")
                .resolve("computer")
                .resolve(String.valueOf(computerId));

        // 既存チェック — src/ 以外を削除して再生成 / Already-exists: delete all except src/ then regenerate
        boolean isRegenerate = Files.exists(devDir);

        try {
            if (isRegenerate) {
                try (java.util.stream.Stream<Path> listing = Files.list(devDir)) {
                    listing.filter(p -> !p.getFileName().toString().equals("src"))
                           .forEach(p -> {
                               try { deleteRecursive(p); }
                               catch (IOException e) {
                                   LOGGER.warn("[RC] 削除失敗 / Failed to delete {}: {}", p, e.getMessage());
                               }
                           });
                }
                source.sendSuccess(
                        () -> Component.literal(
                                "[RC] Computer #" + computerId + " の開発環境を再生成します (src/ は保持)。\n" +
                                "  Regenerating dev env (src/ preserved)."),
                        false);
            }

            // ディレクトリ作成 / Create directory structure
            Files.createDirectories(devDir.resolve("src"));
            Files.createDirectories(devDir.resolve(".cargo"));
            Files.createDirectories(devDir.resolve(".vscode"));
            Files.createDirectories(devDir.resolve("vendor"));
            Files.createDirectories(computerDir); // WASM デプロイ先 / WASM deploy target

            // 1. rust-computers-api ソースを JAR から展開する。
            //    Extract rust-computers-api source from JAR resources.
            int extracted = extractApiSrc(devDir.resolve("vendor").resolve("rust-computers-api"));
            LOGGER.info("DevEnv computer #{}: extracted {} API source files", computerId, extracted);

            // 2. プロジェクトファイルを生成する / Generate project files
            writeDevToml(devDir, computerId);
            writeCargoToml(devDir, computerId);
            writeCargoConfig(devDir);
            // src/main.rs は初回生成のみ実行 (再生成時はユーザーのコードを保持)
            // Write src/main.rs only on first-time generation (preserve user code on regenerate)
            if (!isRegenerate) {
                writeMainRs(devDir.resolve("src"), computerId);
            }
            writeVsCodeTasks(devDir, computerId, computerDir);

            String devDirStr = devDir.toAbsolutePath().toString();
            source.sendSuccess(
                    () -> Component.literal(
                            "[RC] Computer #" + computerId + " の開発環境を生成しました:\n" +
                            "  " + devDirStr + "\n" +
                            "  VS Code で開く: code \"" + devDirStr + "\""),
                    false);
            LOGGER.info("DevEnv generated for computer #{} at: {}", computerId, devDir);

        } catch (IOException e) {
            LOGGER.error("DevEnv generation failed for computer #{}", computerId, e);
            source.sendFailure(Component.literal(
                    "[RC] 開発環境の生成に失敗しました: " + e.getMessage()));
            return 0;
        }

        return 1;
    }

    // ------------------------------------------------------------------
    // API ソース抽出 / API source extraction from JAR
    // ------------------------------------------------------------------

    /**
     * JAR 内の {@code assets/rustcomputers/apisrc/MANIFEST} を読み込み、
     * 列挙されたファイルを {@code vendorDir} に展開する。
     *
     * Reads the MANIFEST bundled in the JAR and extracts all listed files
     * into {@code vendorDir}.
     *
     * @param vendorDir 展開先ディレクトリ / destination directory
     * @return 展開したファイル数 / number of extracted files
     * @throws IOException MANIFEST が見つからない場合 / if MANIFEST is not found
     */
    private static int extractApiSrc(Path vendorDir) throws IOException {
        // MANIFEST 読み込み / Read file list manifest
        try (InputStream manifestStream = DevEnvCommand.class
                .getResourceAsStream("/assets/rustcomputers/apisrc/MANIFEST")) {
            if (manifestStream == null) {
                throw new IOException(
                        "API ソース MANIFEST が JAR に存在しません。mod を再ビルドしてください。\n" +
                        "API source MANIFEST not found in JAR — rebuild the mod.");
            }

            String manifest = new String(manifestStream.readAllBytes(), StandardCharsets.UTF_8);
            int count = 0;

            for (String rawLine : manifest.split("\n")) {
                String relPath = rawLine.strip();
                if (relPath.isEmpty() || relPath.startsWith("#")) continue;

                // always use '/' internally, then resolve() handles OS separator
                String resourcePath = "/assets/rustcomputers/apisrc/" + relPath;
                try (InputStream fileStream = DevEnvCommand.class.getResourceAsStream(resourcePath)) {
                    if (fileStream == null) {
                        LOGGER.warn("API source file missing in JAR: {}", resourcePath);
                        continue;
                    }
                    // relPath uses '/' — Path.resolve() handles this correctly on all OS
                    Path dest = vendorDir;
                    for (String segment : relPath.split("/")) {
                        dest = dest.resolve(segment);
                    }
                    Files.createDirectories(dest.getParent());
                    Files.write(dest, fileStream.readAllBytes());
                    count++;
                }
            }
            return count;
        }
    }

    // ------------------------------------------------------------------
    // ファイル生成 / File generation
    // ------------------------------------------------------------------

    /**
     * 開発レコードファイル {@code dev.toml} を書き出す。
     * Write the development record file {@code dev.toml}.
     */
    private static void writeDevToml(Path devDir, int id) throws IOException {
        String programName = "computer_" + id + ".wasm";
        String content = """
                # RustComputers 開発環境レコード / Development Environment Record
                # /rustcomputers get-dev %d により自動生成されました。
                # Auto-generated by /rustcomputers get-dev %d

                computer_id  = %d
                program_name = "%s"

                # -------------------------------------------------------------------
                # ビルド & デプロイ手順 / Build & Deploy steps
                # -------------------------------------------------------------------
                # 1. VS Code でこのフォルダを開く
                #    Open this folder in VS Code:
                #      $ code .
                #
                # 2. "Build & Deploy" タスクを実行する ( Ctrl+Shift+B )
                #    Run "Build & Deploy" task in VS Code ( Ctrl+Shift+B )
                #
                # 3. ゲーム内でコンピューターの GUI を開く
                #    Open the Computer GUI in-game:
                #      - "%s" を選択 / Select "%s"
                #      - Start ボタンを押す / Press Start
                # -------------------------------------------------------------------
                """.formatted(id, id, id, programName, programName, programName);
        Files.writeString(devDir.resolve("dev.toml"), content, StandardCharsets.UTF_8);
    }

    /**
     * {@code Cargo.toml} を書き出す。
     * Write the {@code Cargo.toml} for the dev crate.
     */
    private static void writeCargoToml(Path devDir, int id) throws IOException {
        String content = """
                # RustComputers Computer #%d — /rustcomputers get-dev %d により自動生成
                [package]
                name = "computer_%d"
                version = "0.1.0"
                edition = "2021"
                publish = false

                [[bin]]
                name    = "computer_%d"
                path    = "src/main.rs"

                [dependencies]
                rust-computers-api = { path = "vendor/rust-computers-api" }

                # WASM サイズ最適化 / WASM size optimization
                [profile.release]
                opt-level     = "s"
                lto           = true
                strip         = "debuginfo"
                codegen-units = 1
                """.formatted(id, id, id, id);
        Files.writeString(devDir.resolve("Cargo.toml"), content, StandardCharsets.UTF_8);
    }

    /**
     * {@code .cargo/config.toml} を書き出す。
     * Write the {@code .cargo/config.toml} to set WASM as the default target.
     */
    private static void writeCargoConfig(Path devDir) throws IOException {
        String content = """
                # WASM ターゲットをデフォルトに設定 / Set WASM as the default build target
                [build]
                target = "wasm32-unknown-unknown"
                """;
        Files.writeString(
                devDir.resolve(".cargo").resolve("config.toml"),
                content, StandardCharsets.UTF_8);
    }

    /**
     * {@code src/main.rs} スキャフォールドを書き出す。
     * Write the {@code src/main.rs} scaffold.
     */
    private static void writeMainRs(Path srcDir, int id) throws IOException {
        String content = """
                //! Computer #%d メインプログラム。
                //! Main program for Computer #%d.
                //!
                //! ## ビルド & デプロイ / Build & Deploy
                //! VS Code で Ctrl+Shift+B → "Build & Deploy" を選択。
                //! In VS Code, press Ctrl+Shift+B and select "Build & Deploy".
                //!
                //! ## ゲーム内での起動 / Starting in-game
                //! コンピューターの GUI を開き `computer_%d.wasm` を選択して Start。
                //! Open the Computer GUI, select `computer_%d.wasm`, and press Start.

                #![no_std]
                #![no_main]

                extern crate alloc;

                use alloc::format;
                use rust_computers_api as rc;

                /// エントリーポイント / Entry point
                rc::entry!(main);

                async fn main() {
                    rc::println!("Computer #%d 起動しました / started!");

                    // -------------------------------------------------------------------
                    // ここにプログラムを書いてください。
                    // Write your program here.
                    //
                    // 例: 南側に CC:Tweaked Monitor を置いた場合のモニター書き込み:
                    // Example: writing to a CC:Tweaked Monitor placed to the south:
                    // -------------------------------------------------------------------

                    // use rc::computer_craft::monitor::Monitor;
                    // use rc::peripheral::Direction;
                    //
                    // let mon = Monitor::new(Direction::South);
                    //
                    // if let Ok((w, h)) = mon.get_size() {
                    //     mon.clear().await.ok();
                    //     mon.set_cursor_pos(1, 1).await.ok();
                    //     mon.write(&format!("Hello from Computer #%d!")).await.ok();
                    //     mon.write(&format!("Size: {}x{}", w, h)).await.ok();
                    // }
                }
                """.formatted(id, id, id, id, id, id);
        Files.writeString(srcDir.resolve("main.rs"), content, StandardCharsets.UTF_8);
    }

    /**
     * {@code .vscode/tasks.json} を書き出す。
     * Write the {@code .vscode/tasks.json} with Build and Deploy tasks.
     *
     * <p>生成タスク一覧:
     * <ul>
     *   <li>Build WASM — {@code cargo build --release} を実行してWASMを生成</li>
     *   <li>Deploy to Computer #N — .wasm ファイルをコンピューターディレクトリにコピー</li>
     *   <li>Build & Deploy — 上記2つを順番に実行 (デフォルトビルドタスク)</li>
     * </ul>
     * </p>
     */
    private static void writeVsCodeTasks(Path devDir, int id, Path computerDir) throws IOException {
        String programName = "computer_" + id + ".wasm";
        // WASM ビルド出力パス (devDir からの相対パス)
        String wasmSrc = "target/wasm32-unknown-unknown/release/" + programName;
        // デプロイ先の絶対パス (スペース対応のためシングルクォートでラップ)
        String wasmDestAbs = computerDir.toAbsolutePath() + "/" + programName;

        // shell コマンド: コンピューターディレクトリを作成してWASMをコピー
        // mkdir -p ensures the dir exists, then cp copies the WASM
        String deployCmd = "mkdir -p '" + escapeShellSingleQuote(computerDir.toAbsolutePath().toString())
                + "' && cp '" + escapeShellSingleQuote(wasmSrc)
                + "' '" + escapeShellSingleQuote(wasmDestAbs) + "'";

        // JSON テンプレート (Java text block — double quotes inside are fine)
        // JSON template using Java text block
        String content = """
                {
                    "version": "2.0.0",
                    "tasks": [
                        {
                            "label": "Build WASM",
                            "type": "shell",
                            "command": "cargo build --release",
                            "group": {
                                "kind": "build",
                                "isDefault": false
                            },
                            "presentation": {
                                "echo": true,
                                "reveal": "always",
                                "focus": false,
                                "panel": "shared"
                            },
                            "problemMatcher": ["$rustc"]
                        },
                        {
                            "label": "Deploy to Computer #%d",
                            "type": "shell",
                            "command": "%s",
                            "dependsOn": ["Build WASM"],
                            "presentation": {
                                "echo": true,
                                "reveal": "always",
                                "focus": false,
                                "panel": "shared"
                            },
                            "problemMatcher": []
                        },
                        {
                            "label": "Build & Deploy",
                            "dependsOn": ["Deploy to Computer #%d"],
                            "group": {
                                "kind": "build",
                                "isDefault": true
                            },
                            "problemMatcher": []
                        }
                    ]
                }
                """.formatted(id, deployCmd, id);
        Files.writeString(
                devDir.resolve(".vscode").resolve("tasks.json"),
                content, StandardCharsets.UTF_8);
    }

    // ------------------------------------------------------------------
    // ユーティリティ / Utilities
    // ------------------------------------------------------------------

    /**
     * パスを再帰的に削除する (ファイルでもディレクトリでも可)。
     * Recursively delete a path (works for both files and directories).
     */
    private static void deleteRecursive(Path path) throws IOException {
        if (Files.isDirectory(path)) {
            try (java.util.stream.Stream<Path> children = Files.list(path)) {
                for (Path child : (Iterable<Path>) children::iterator) {
                    deleteRecursive(child);
                }
            }
        }
        Files.deleteIfExists(path);
    }

    /**
     * シェルのシングルクォートをエスケープする ('→'"'"')。
     * Escape ASCII single quotes for use inside shell single-quoted strings.
     */
    private static String escapeShellSingleQuote(String s) {
        // シングルクォート内でシングルクォートを表現: 閉じて '"'"' で再開
        return s.replace("'", "'\"'\"'");
    }
}
