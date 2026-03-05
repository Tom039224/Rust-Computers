package com.rustcomputers.gui;

import com.mojang.blaze3d.vertex.PoseStack;
import com.rustcomputers.ModRegistries;
import com.rustcomputers.network.ComputerActionPacket;
import com.rustcomputers.network.NetworkHandler;
import com.rustcomputers.network.StdinInputPacket;
import com.rustcomputers.network.UploadWasmPacket;
import com.rustcomputers.wasm.ComputerState;
import com.rustcomputers.wasm.LogBuffer;
import com.rustcomputers.wasm.WasmEngine;
import net.minecraft.client.gui.GuiGraphics;
import net.minecraft.client.gui.components.Button;
import net.minecraft.client.gui.components.EditBox;
import net.minecraft.client.gui.screens.inventory.AbstractContainerScreen;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.player.Inventory;
import net.minecraftforge.api.distmarker.Dist;
import net.minecraftforge.api.distmarker.OnlyIn;
import net.minecraftforge.client.event.ScreenEvent;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nullable;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

/**
 * コンピューター GUI 画面。
 * Computer GUI screen.
 *
 * <p>design-proposal W-4 のレイアウト:
 * - Computer #ID
 * - Status: STATE  program_name.wasm
 * - ログ欄（15行表示、200行バッファ、スクロール可）
 * - 入力欄（Enter で送信）</p>
 */
@OnlyIn(Dist.CLIENT)
public class ComputerScreen extends AbstractContainerScreen<ComputerMenu> {

    private static final Logger LOGGER = LoggerFactory.getLogger(ComputerScreen.class);


    // ------------------------------------------------------------------
    // レイアウト定数 / Layout constants
    // ------------------------------------------------------------------

    /** GUI 全体の幅 / Total GUI width */
    private static final int GUI_WIDTH = 340;

    /** GUI 全体の高さ / Total GUI height */
    private static final int GUI_HEIGHT = 260;

    /** ログ表示行数 / Number of visible log lines */
    private static final int LOG_VISIBLE_LINES = 15;

    /** ログ行の高さ（ピクセル） / Log line height in pixels */
    private static final int LOG_LINE_HEIGHT = 10;

    /** 左マージン / Left margin */
    private static final int MARGIN = 8;

    /** ヘッダー高さ / Header height */
    private static final int HEADER_HEIGHT = 42;

    // ------------------------------------------------------------------
    // フィールド / Fields
    // ------------------------------------------------------------------

    /** クライアント側ログバッファ / Client-side log buffer */
    private final LogBuffer clientLog = new LogBuffer(200);

    /** 入力フィールド / Input field */
    @Nullable
    private EditBox inputField;

    /** ログスクロールオフセット / Log scroll offset */
    private int scrollOffset = 0;

    /** 現在表示中のインスタンスの参照（S2C ログ受信用） */
    /** Reference for the currently displayed instance (for S2C log reception) */
    private static volatile ComputerScreen activeScreen;

    // ------------------------------------------------------------------
    // コンストラクタ / Constructor
    // ------------------------------------------------------------------

    public ComputerScreen(ComputerMenu menu, Inventory playerInv, Component title) {
        super(menu, playerInv, title);
        this.imageWidth = GUI_WIDTH;
        this.imageHeight = GUI_HEIGHT;
    }

    // ------------------------------------------------------------------
    // 初期化 / Initialization
    // ------------------------------------------------------------------

    @Override
    protected void init() {
        super.init();
        activeScreen = this;

        // 入力フィールド / Input field
        int inputY = topPos + GUI_HEIGHT - 24;
        inputField = new EditBox(font, leftPos + MARGIN, inputY, GUI_WIDTH - MARGIN * 2 - 50, 16,
                Component.translatable("gui.rustcomputers.computer.input_placeholder"));
        inputField.setMaxLength(4096);
        inputField.setResponder(text -> {}); // 入力時は何もしない / no-op on input
        addRenderableWidget(inputField);

        // 送信ボタン / Send button
        addRenderableWidget(Button.builder(Component.literal(">>"), btn -> submitInput())
                .bounds(leftPos + GUI_WIDTH - MARGIN - 44, inputY - 2, 40, 20)
                .build());

        // 開始/停止ボタン / Start/Stop button
        addRenderableWidget(Button.builder(Component.literal("▶/■"), btn -> toggleRunning())
                .bounds(leftPos + GUI_WIDTH - MARGIN - 44, topPos + 4, 40, 16)
                .build());

        // プログラム選択ボタン / Program selector buttons
        addRenderableWidget(Button.builder(Component.literal("<"), btn -> prevProgram())
                .bounds(leftPos + MARGIN, topPos + 28, 16, 14)
                .build());
        addRenderableWidget(Button.builder(Component.literal(">"), btn -> nextProgram())
                .bounds(leftPos + MARGIN + 18, topPos + 28, 16, 14)
                .build());
    }

    @Override
    public void removed() {
        super.removed();
        if (activeScreen == this) {
            activeScreen = null;
        }
    }

    // ------------------------------------------------------------------
    // 描画 / Rendering
    // ------------------------------------------------------------------

    @Override
    protected void renderBg(GuiGraphics graphics, float partialTick, int mouseX, int mouseY) {
        // 背景（暗い半透明） / Background (dark semi-transparent)
        graphics.fill(leftPos, topPos, leftPos + GUI_WIDTH, topPos + GUI_HEIGHT, 0xCC1A1A2E);

        // ログ領域の背景 / Log area background
        int logTop = topPos + HEADER_HEIGHT;
        int logBottom = topPos + GUI_HEIGHT - 30;
        graphics.fill(leftPos + 4, logTop, leftPos + GUI_WIDTH - 4, logBottom, 0xCC0D0D1A);
    }

    @Override
    protected void renderLabels(GuiGraphics graphics, int mouseX, int mouseY) {
        // ヘッダー: Computer #ID / Header: Computer #ID
        int id = menu.getComputerId();
        graphics.drawString(font, "Computer #" + id, MARGIN, 6, 0xFFFFFF, false);

        // ステータス行 / Status line
        ComputerState state = menu.getComputerState();
        int stateColor = switch (state) {
            case RUNNING -> 0x55FF55;  // 緑
            case CRASHED -> 0xFF5555;  // 赤
            case STOPPED -> 0xAAAAAA;  // グレー
        };
        String statusText = state.name();
        graphics.drawString(font, statusText, MARGIN, 18, stateColor, false);

        // プログラム選択表示 / Program selector display
        String prog = menu.getSelectedProgram();
        String progLabel = prog != null ? prog : "(no program)";
        int progColor = (prog != null) ? 0xFFFFFF : 0x888888;
        // [<] [>] ボタンの右側に表示 (x = MARGIN + 18 + 18 + 4 = 40)
        graphics.drawString(font, progLabel, MARGIN + 40, 32, progColor, false);

        // ログ描画 / Draw log lines
        String[] allLines = clientLog.snapshot();
        int totalLines = allLines.length;
        int logStartY = HEADER_HEIGHT + 2;

        // スクロール位置を正規化 / Normalize scroll position
        int maxScroll = Math.max(0, totalLines - LOG_VISIBLE_LINES);
        scrollOffset = Math.max(0, Math.min(scrollOffset, maxScroll));

        int startIdx = Math.max(0, totalLines - LOG_VISIBLE_LINES - scrollOffset);
        int endIdx = Math.min(totalLines, startIdx + LOG_VISIBLE_LINES);

        for (int i = startIdx; i < endIdx; i++) {
            int y = logStartY + (i - startIdx) * LOG_LINE_HEIGHT;
            graphics.drawString(font, allLines[i], MARGIN, y, 0xCCCCCC, false);
        }
    }

    @Override
    public void render(GuiGraphics graphics, int mouseX, int mouseY, float partialTick) {
        renderBackground(graphics);
        super.render(graphics, mouseX, mouseY, partialTick);
    }

    // ------------------------------------------------------------------
    // 入力処理 / Input handling
    // ------------------------------------------------------------------

    @Override
    public boolean keyPressed(int keyCode, int scanCode, int modifiers) {
        // Enter キーで入力を送信 / Send input on Enter key
        if (keyCode == 257 && inputField != null && inputField.isFocused()) {
            submitInput();
            return true;
        }
        // inputField がフォーカス中なら他のキーバインドを無効化
        // Disable other key bindings when inputField is focused
        if (inputField != null && inputField.isFocused()) {
            return inputField.keyPressed(keyCode, scanCode, modifiers);
        }
        return super.keyPressed(keyCode, scanCode, modifiers);
    }

    @Override
    public boolean mouseScrolled(double mouseX, double mouseY, double delta) {
        // ログ欄のスクロール / Log area scrolling
        scrollOffset += (delta > 0) ? 1 : -1;
        scrollOffset = Math.max(0, scrollOffset);
        return true;
    }

    /**
     * 入力欄のテキストを送信する。
     * Submit the text in the input field.
     */
    private void submitInput() {
        if (inputField == null) return;
        String text = inputField.getValue().trim();
        if (text.isEmpty()) return;

        // stdin パケットを送信 / Send stdin packet
        NetworkHandler.CHANNEL.sendToServer(new StdinInputPacket(menu.getBlockPos(), text));

        // ローカルログにもエコー / Echo to local log
        clientLog.append("> " + text);

        inputField.setValue("");
    }

    /**
     * 開始/停止を切り替える。
     * Toggle start/stop.
     */
    private void toggleRunning() {
        ComputerState state = menu.getComputerState();
        if (state == ComputerState.RUNNING) {
            NetworkHandler.CHANNEL.sendToServer(
                    new ComputerActionPacket(menu.getBlockPos(), ComputerActionPacket.Action.STOP, ""));
        } else {
            String selected = menu.getSelectedProgram();
            if (selected == null) {
                clientLog.append("[ERROR] No program selected. Upload a .wasm file first.");
                return;
            }
            NetworkHandler.CHANNEL.sendToServer(
                    new ComputerActionPacket(menu.getBlockPos(), ComputerActionPacket.Action.START, selected));
        }
    }

    /**
     * 前のプログラムに切り替える。
     * Switch to the previous program.
     */
    private void prevProgram() {
        List<String> programs = menu.getPrograms();
        if (programs.isEmpty()) return;
        int idx = menu.getSelectedProgramIndex();
        menu.setSelectedProgramIndex((idx - 1 + programs.size()) % programs.size());
    }

    /**
     * 次のプログラムに切り替える。
     * Switch to the next program.
     */
    private void nextProgram() {
        List<String> programs = menu.getPrograms();
        if (programs.isEmpty()) return;
        int idx = menu.getSelectedProgramIndex();
        menu.setSelectedProgramIndex((idx + 1) % programs.size());
    }

    // ------------------------------------------------------------------
    // Drag & Drop 対応 / Drag & Drop support
    // ------------------------------------------------------------------

    @Override
    public boolean mouseClicked(double mouseX, double mouseY, int button) {
        return super.mouseClicked(mouseX, mouseY, button);
    }

    /**
     * ファイルドロップ処理。
     * Handle file drops.
     */
    @Override
    public void onFilesDrop(List<Path> paths) {
        for (Path path : paths) {
            String name = path.getFileName().toString();
            if (!name.endsWith(".wasm")) continue;

            try {
                byte[] data = Files.readAllBytes(path);
                if (!WasmEngine.isValidWasm(data)) {
                    LOGGER.warn("Dropped file is not a valid WASM binary: {}", name);
                    continue;
                }

                // アップロードパケットを送信 / Send upload packet
                NetworkHandler.CHANNEL.sendToServer(
                        new UploadWasmPacket(menu.getBlockPos(), name, data));
                clientLog.append("[Upload] " + name + " (" + data.length + " bytes)");

            } catch (Exception e) {
                LOGGER.error("Failed to read dropped file: {}", path, e);
                clientLog.append("[ERROR] Failed to read: " + name);
            }
        }
    }

    // ------------------------------------------------------------------
    // 静的メソッド / Static methods
    // ------------------------------------------------------------------

    /**
     * S2C ログ更新パケットを処理する。
     * Handle an S2C log update packet.
     */
    public static void handleLogUpdate(int computerId, List<String> lines) {
        ComputerScreen screen = activeScreen;
        if (screen != null && screen.menu.getComputerId() == computerId) {
            for (String line : lines) {
                screen.clientLog.append(line);
            }
        }
    }

    /**
     * S2C プログラム一覧更新を処理する。
     * Handle an S2C program list update.
     */
    public static void handleProgramListUpdate(List<String> programs) {
        ComputerScreen screen = activeScreen;
        if (screen != null) {
            screen.menu.updatePrograms(programs);
        }
    }

    /**
     * クライアントセットアップ時にメニュースクリーンを登録する。
     * Register the menu screen during client setup.
     */
    public static void register() {
        net.minecraft.client.gui.screens.MenuScreens.register(
                ModRegistries.COMPUTER_MENU.get(), ComputerScreen::new);
    }
}
