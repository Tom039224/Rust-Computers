package com.rustcomputers;

import com.rustcomputers.computer.ComputerBlock;
import com.rustcomputers.computer.ComputerBlockEntity;
import com.rustcomputers.gui.ComputerMenu;
import net.minecraft.core.registries.Registries;
import net.minecraft.network.chat.Component;
import net.minecraft.world.inventory.MenuType;
import net.minecraft.world.item.BlockItem;
import net.minecraft.world.item.CreativeModeTab;
import net.minecraft.world.item.Item;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.entity.BlockEntityType;
import net.minecraftforge.common.extensions.IForgeMenuType;
import net.minecraftforge.eventbus.api.IEventBus;
import net.minecraftforge.registries.DeferredRegister;
import net.minecraftforge.registries.ForgeRegistries;
import net.minecraftforge.registries.RegistryObject;

/**
 * Mod 全体のレジストリ登録を一元管理するクラス。
 * Centralized registry for all blocks, items, block entities, menus, and tabs.
 */
public final class ModRegistries {

    // ------------------------------------------------------------------
    // DeferredRegister インスタンス / Deferred register instances
    // ------------------------------------------------------------------
    public static final DeferredRegister<Block> BLOCKS =
            DeferredRegister.create(ForgeRegistries.BLOCKS, RustComputers.MOD_ID);

    public static final DeferredRegister<Item> ITEMS =
            DeferredRegister.create(ForgeRegistries.ITEMS, RustComputers.MOD_ID);

    public static final DeferredRegister<BlockEntityType<?>> BLOCK_ENTITIES =
            DeferredRegister.create(ForgeRegistries.BLOCK_ENTITY_TYPES, RustComputers.MOD_ID);

    public static final DeferredRegister<MenuType<?>> MENUS =
            DeferredRegister.create(ForgeRegistries.MENU_TYPES, RustComputers.MOD_ID);

    public static final DeferredRegister<CreativeModeTab> CREATIVE_TABS =
            DeferredRegister.create(Registries.CREATIVE_MODE_TAB, RustComputers.MOD_ID);

    // ------------------------------------------------------------------
    // ブロック / Blocks
    // ------------------------------------------------------------------

    /** コンピューターブロック / Computer block */
    public static final RegistryObject<Block> COMPUTER_BLOCK =
            BLOCKS.register("computer", ComputerBlock::new);

    // ------------------------------------------------------------------
    // アイテム / Items
    // ------------------------------------------------------------------

    /** コンピューターブロックアイテム / Computer block item */
    public static final RegistryObject<Item> COMPUTER_ITEM =
            ITEMS.register("computer", () -> new BlockItem(
                    COMPUTER_BLOCK.get(), new Item.Properties()));

    // ------------------------------------------------------------------
    // ブロックエンティティ / Block entities
    // ------------------------------------------------------------------

    /** コンピューター BE タイプ / Computer block entity type */
    @SuppressWarnings("ConstantConditions")
    public static final RegistryObject<BlockEntityType<ComputerBlockEntity>> COMPUTER_BE =
            BLOCK_ENTITIES.register("computer", () ->
                    BlockEntityType.Builder
                            .of(ComputerBlockEntity::new, COMPUTER_BLOCK.get())
                            .build(null));

    // ------------------------------------------------------------------
    // メニュー / Menus
    // ------------------------------------------------------------------

    /** コンピューター GUI メニュータイプ / Computer GUI menu type */
    public static final RegistryObject<MenuType<ComputerMenu>> COMPUTER_MENU =
            MENUS.register("computer", () ->
                    IForgeMenuType.create(ComputerMenu::fromNetwork));

    // ------------------------------------------------------------------
    // クリエイティブタブ / Creative tabs
    // ------------------------------------------------------------------

    /** RustComputers クリエイティブタブ / RustComputers creative tab */
    public static final RegistryObject<CreativeModeTab> TAB =
            CREATIVE_TABS.register("tab", () -> CreativeModeTab.builder()
                    .title(Component.translatable("itemGroup.rustcomputers"))
                    .icon(() -> COMPUTER_ITEM.get().getDefaultInstance())
                    .displayItems((params, output) -> output.accept(COMPUTER_ITEM.get()))
                    .build());

    // ------------------------------------------------------------------
    // 一括登録 / Batch registration
    // ------------------------------------------------------------------

    /**
     * 全 DeferredRegister を Mod イベントバスに登録する。
     * Register all deferred registers on the mod event bus.
     */
    public static void register(IEventBus bus) {
        BLOCKS.register(bus);
        ITEMS.register(bus);
        BLOCK_ENTITIES.register(bus);
        MENUS.register(bus);
        CREATIVE_TABS.register(bus);
    }

    private ModRegistries() { /* ユーティリティクラス / Utility class */ }
}
