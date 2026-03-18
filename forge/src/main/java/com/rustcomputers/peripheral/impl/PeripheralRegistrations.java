package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.PeripheralProvider;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraftforge.registries.ForgeRegistries;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;

/**
 * 全 Mod のペリフェラル登録を一元管理するクラス。
 * Central class managing peripheral registration for all supported mods.
 *
 * <p>各 Mod の登録メソッドは {@code RustComputers.commonSetup()} から
 * {@code ModList.get().isLoaded()} チェック後に呼ばれる。</p>
 * 
 * <p>Each mod's registration method is called from {@code RustComputers.commonSetup()}
 * after checking {@code ModList.get().isLoaded()}.</p>
 */
public final class PeripheralRegistrations {

    private static final Logger LOGGER = LoggerFactory.getLogger(PeripheralRegistrations.class);

    private PeripheralRegistrations() {}

    // ------------------------------------------------------------------
    // ヘルパー / Helpers
    // ------------------------------------------------------------------

    /** ブロック登録ヘルパー */
    @SuppressWarnings("deprecation")
    private static void reg(String namespace, String path, String typeName, String[] methods) {
        Block block = ForgeRegistries.BLOCKS.getValue(new ResourceLocation(namespace, path));
        if (block != null && block != Blocks.AIR) {
            PeripheralProvider.register(block, () -> new CcGenericPeripheral(typeName, methods));
            LOGGER.debug("Registered peripheral: {} -> {}", namespace + ":" + path, typeName);
        } else {
            LOGGER.debug("Block not found: {}:{} (mod may not include this block)", namespace, path);
        }
    }

    /** IMM メソッド指定付きブロック登録ヘルパー */
    @SuppressWarnings("deprecation")
    private static void regWithImm(String namespace, String path, String typeName,
                                    String[] allMethods, Set<String> immMethods) {
        Block block = ForgeRegistries.BLOCKS.getValue(new ResourceLocation(namespace, path));
        if (block != null && block != Blocks.AIR) {
            PeripheralProvider.register(block,
                    () -> new CcGenericPeripheral(typeName, allMethods, immMethods));
            LOGGER.debug("Registered peripheral (with IMM): {} -> {}", namespace + ":" + path, typeName);
        } else {
            LOGGER.debug("Block not found: {}:{} (mod may not include this block)", namespace, path);
        }
    }

    /** 専用 PeripheralType 実装を使ったブロック登録ヘルパー */
    @SuppressWarnings("deprecation")
    private static void regImpl(String namespace, String path,
                                java.util.function.Supplier<com.rustcomputers.peripheral.PeripheralType> factory) {
        Block block = ForgeRegistries.BLOCKS.getValue(new ResourceLocation(namespace, path));
        if (block != null && block != Blocks.AIR) {
            PeripheralProvider.register(block, factory);
            LOGGER.debug("Registered peripheral (custom impl): {}:{}", namespace, path);
        } else {
            LOGGER.debug("Block not found: {}:{} (mod may not include this block)", namespace, path);
        }
    }

    /** CcBlockEntityBridge を使ったブロック登録ヘルパー */
    @SuppressWarnings("deprecation")
    private static void regBE(String namespace, String path, String typeName,
                               String[] methods, Set<String> immMethods) {
        Block block = ForgeRegistries.BLOCKS.getValue(new ResourceLocation(namespace, path));
        if (block != null && block != Blocks.AIR) {
            PeripheralProvider.register(block,
                    () -> new CcBlockEntityBridge(typeName, methods, immMethods));
            LOGGER.debug("Registered peripheral (CcBEBridge): {}:{} -> {}", namespace, path, typeName);
        } else {
            LOGGER.debug("Block not found: {}:{} (mod may not include this block)", namespace, path);
        }
    }

    private static void regBE(String namespace, String path, String typeName, String[] methods) {
        regBE(namespace, path, typeName, methods, Set.of());
    }

    /** CcPeripheralBridge を使ったブロック登録ヘルパー */
    @SuppressWarnings("deprecation")
    private static void regBridge(String namespace, String path, String typeName,
                                   String[] methods, Set<String> immMethods, String providerClassName) {
        Block block = ForgeRegistries.BLOCKS.getValue(new ResourceLocation(namespace, path));
        if (block != null && block != Blocks.AIR) {
            PeripheralProvider.register(block, () -> {
                CcPeripheralBridge bridge = CcPeripheralBridge.create(typeName, methods, immMethods, providerClassName);
                return bridge != null ? bridge : new CcGenericPeripheral(typeName, methods, immMethods);
            });
            LOGGER.debug("Registered peripheral (CcBridge): {}:{} -> {}", namespace, path, typeName);
        } else {
            LOGGER.debug("Block not found: {}:{} (mod may not include this block)", namespace, path);
        }
    }

    private static String[] gasNetworkPlus(String... extras) {
        String[] base = {"getTemperature", "getPressure", "getHeatEnergy", "getGasMass", "getPosition", "getNetworkInfo"};
        String[] result = new String[base.length + extras.length];
        System.arraycopy(base, 0, result, 0, base.length);
        System.arraycopy(extras, 0, result, base.length, extras.length);
        return result;
    }

    /** gas_network の IMM 対応メソッドセット (getNetworkInfo 以外) */
    private static Set<String> gasNetworkImm() {
        return new HashSet<>(Arrays.asList("getTemperature", "getPressure", "getHeatEnergy", "getGasMass", "getPosition"));
    }

    /** gas_network IMM + 追加 IMM メソッド */
    private static Set<String> gasNetworkImmPlus(String... extras) {
        Set<String> set = gasNetworkImm();
        set.addAll(Arrays.asList(extras));
        return set;
    }

    // ==================================================================
    // CC:Tweaked 自身のブロック / CC:Tweaked's own blocks
    // ==================================================================

    public static void registerCcTweaked() {
        LOGGER.info("Registering CC:Tweaked peripherals (speaker, modem, monitor, inventory)");

        // Speaker - 専用実装を使用
        Block speakerBlock = ForgeRegistries.BLOCKS.getValue(new ResourceLocation("computercraft", "speaker"));
        if (speakerBlock != null && speakerBlock != Blocks.AIR) {
            PeripheralProvider.register(speakerBlock, CcSpeakerPeripheral::new);
            LOGGER.debug("Registered peripheral: computercraft:speaker -> speaker");
        }

        // Modem (3 block variants) - 専用実装を使用
        String[] modemBlocks = {"wireless_modem_normal", "wireless_modem_advanced", "wired_modem_full"};
        for (String blockName : modemBlocks) {
            Block modemBlock = ForgeRegistries.BLOCKS.getValue(new ResourceLocation("computercraft", blockName));
            if (modemBlock != null && modemBlock != Blocks.AIR) {
                PeripheralProvider.register(modemBlock, CcModemPeripheral::new);
                LOGGER.debug("Registered peripheral: computercraft:{} -> modem", blockName);
            }
        }

        // Monitor (2 block variants) - 専用実装を使用
        String[] monitorBlocks = {"monitor_normal", "monitor_advanced"};
        for (String blockName : monitorBlocks) {
            Block monitorBlock = ForgeRegistries.BLOCKS.getValue(new ResourceLocation("computercraft", blockName));
            if (monitorBlock != null && monitorBlock != Blocks.AIR) {
                PeripheralProvider.register(monitorBlock, CcMonitorPeripheralExt::new);
                LOGGER.debug("Registered peripheral: computercraft:{} -> monitor", blockName);
            }
        }

        // Inventory - CC:Tweaked の GenericPeripheral として動的に付与されるため、
        // ここでは登録しない（任意のインベントリブロックに自動的に付与される）
        // Inventory is dynamically attached as CC:Tweaked's GenericPeripheral,
        // so we don't register it here (automatically attached to any inventory block)
    }

    // ==================================================================
    // Some Peripherals 追加 / Some Peripherals extras
    // ==================================================================

    private static final String SP_PROVIDER = "net.spaceeye.someperipherals.forge.integrations.cc.SomePeripheralsPeripheralProviderForge";

    public static void registerSomePeripheralsExtras() {
        LOGGER.info("Registering Some Peripherals extras (radar, ballistic_accelerator, digitizer, raycaster, world_scanner, goggle_link_port)");

        // Radar — scan/scanForEntities/scanForShips/scanForPlayers は mainThread (callMethod ルート)
        //         getConfigInfo は非 mainThread (IMM)
        regBridge("some_peripherals", "radar", "sp_radar",
            new String[]{"scan", "scanForEntities", "scanForShips", "scanForPlayers", "getConfigInfo"},
            new HashSet<>(Collections.singletonList("getConfigInfo")), SP_PROVIDER);

        // BallisticAccelerator — 全メソッド非 mainThread (全 IMM)
        regBridge("some_peripherals", "ballistic_accelerator", "ballistic_accelerator",
            new String[]{"timeInAir", "tryPitch", "calculatePitch", "batchCalculatePitches", "getDrag"},
            new HashSet<>(Arrays.asList("timeInAir", "tryPitch", "calculatePitch", "batchCalculatePitches", "getDrag")),
            SP_PROVIDER);

        // Digitizer — 全メソッド mainThread (全て callMethod ルート)
        regBridge("some_peripherals", "digitizer", "digitizer",
            new String[]{"digitizeAmount", "rematerializeAmount", "mergeDigitalItems", "separateDigitalItem",
                         "checkID", "getItemInSlot", "getItemLimitInSlot"},
            Set.of(), SP_PROVIDER);

        // Raycaster — 全メソッド非 mainThread (全 IMM)
        regBridge("some_peripherals", "raycaster", "raycaster",
            new String[]{"raycast", "addStickers", "getConfigInfo", "getFacingDirection"},
            new HashSet<>(Arrays.asList("raycast", "addStickers", "getConfigInfo", "getFacingDirection")),
            SP_PROVIDER);

        // WorldScanner — 非 mainThread (全 IMM)
        regBridge("some_peripherals", "world_scanner", "world_scanner",
            new String[]{"getBlockAt"},
            new HashSet<>(Collections.singletonList("getBlockAt")), SP_PROVIDER);

        // GoggleLinkPort
        regBridge("some_peripherals", "goggle_link_port", "goggle_link_port",
            new String[]{"getConnected"}, Set.of(), SP_PROVIDER);
    }

    // ==================================================================
    // CC-VS / CC-VS peripherals
    // ==================================================================

    public static void registerCcVs() {
        LOGGER.info("Registering CC-VS peripherals (ship, aerodynamics, drag)");

        // Ship
        String[] shipMethods = {
            "getId", "getMass", "getMomentOfInertiaTensor", "getSlug",
            "getAngularVelocity", "getQuaternion", "getScale", "getShipyardPosition",
            "getSize", "getVelocity", "getWorldspacePosition", "isStatic",
            "getTransformationMatrix", "getJoints", "transformPositionToWorld",
            "setSlug", "setStatic", "setScale", "teleport",
            "applyWorldForce", "applyWorldTorque", "applyModelForce", "applyModelTorque",
            "applyWorldForceToModelPos", "applyBodyForce", "applyBodyTorque", "applyWorldForceToBodyPos",
            "try_pull_physics_ticks"
        };
        Set<String> shipImm = new HashSet<>(Arrays.asList(
            "getId", "getMass", "getMomentOfInertiaTensor", "getSlug",
            "getAngularVelocity", "getQuaternion", "getScale", "getShipyardPosition",
            "getSize", "getVelocity", "getWorldspacePosition", "isStatic",
            "getTransformationMatrix", "getJoints", "transformPositionToWorld"
        ));
        regWithImm("cc_vs", "ship_reader", "ship", shipMethods, shipImm);

        // Aerodynamics
        String[] aeroMethods = {
            "defaultMax", "defaultSeaLevel", "dragCoefficient",
            "gravitationalAcceleration", "universalGasConstant", "airMolarMass",
            "getAtmosphericParameters", "getAirDensity", "getAirPressure", "getAirTemperature"
        };
        regWithImm("cc_vs", "aerodynamics", "vs_aerodynamics", aeroMethods,
                new HashSet<>(Arrays.asList(aeroMethods)));

        // Drag — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("cc_vs", "drag", "vs_drag",
            new String[]{"enableDrag", "disableDrag", "enableLift", "disableLift",
                "enableRotDrag", "disableRotDrag", "setWindDirection", "setWindSpeed", "applyWindImpulse"});
    }

    // ==================================================================
    // Toms Peripherals / Tom's Peripherals
    // ==================================================================

    public static void registerTomsPeripherals() {
        LOGGER.info("Registering Tom's Peripherals (gpu, keyboard, redstone_port, watchdog_timer)");

        // GPU — 専用実装で描画系メソッドを実際の GPUPeripheral に委譲する
        regImpl("toms_peripherals", "gpu", TmGpuPeripheral::new);

        // Keyboard — CcBlockEntityBridge で setFireNativeEvents を実際の KeyboardPeripheral に委譲する
        regBE("toms_peripherals", "keyboard", "tm_keyboard",
            new String[]{"setFireNativeEvents"});

        // Redstone Port — 専用実装で setter 系を実際の RedstonePortPeripheral に委譲する
        regImpl("toms_peripherals", "redstone_port", TmRedstonePortPeripheral::new);

        // Watchdog Timer — 専用実装で setter 系を実際の WatchdogTimerPeripheral に委譲する
        regImpl("toms_peripherals", "wdt", TmWatchdogTimerPeripheral::new);
    }

    // ==================================================================
    // Clockwork CC Compat / Clockwork CC Compat
    // ==================================================================

    private static final String DUCT_PROVIDER = "com.tom039224.clockworkcccompat.peripheral.DuctPeripheralProvider";
    private static final String BOILER_PROVIDER = "com.tom039224.clockworkcccompat.peripheral.BoilerPeripheralProvider";

    public static void registerClockworkCcCompat() {
        LOGGER.info("Registering Clockwork CC Compat peripherals (12 types) via CcPeripheralBridge");

        // Air Compressor (gas_network + unique)
        String[] airCompMethods = gasNetworkPlus("getFacing", "getStatus", "getSpeed");
        regBridge("clockwork_cc_compat", "air_compressor", "cw_air_compressor",
                airCompMethods, gasNetworkImmPlus("getFacing", "getStatus", "getSpeed"), DUCT_PROVIDER);

        // Boiler (NO gas_network! — uses BoilerPeripheralProvider)
        String[] boilerMethods = {
            "isActive", "getHeatLevel", "getActiveHeat", "isPassiveHeat",
            "getWaterSupply", "getAttachedEngines", "getAttachedWhistles",
            "getEngineEfficiency", "getBoilerSize", "getWidth", "getHeight",
            "getMaxHeatForSize", "getMaxHeatForWater", "getFillState",
            "getFluidContents", "getControllerPos"
        };
        regBridge("create", "fluid_tank", "Create_Boiler",
                boilerMethods, new HashSet<>(Arrays.asList(boilerMethods)), BOILER_PROVIDER);

        // Coal Burner (gas_network + unique)
        String[] coalBurnerMethods = gasNetworkPlus("getFuelTicks", "getMaxBurnTime", "isBurning");
        regBridge("clockwork_cc_compat", "coal_burner", "cw_coal_burner",
                coalBurnerMethods, gasNetworkImmPlus("getFuelTicks", "getMaxBurnTime", "isBurning"), DUCT_PROVIDER);

        // Duct Tank (gas_network + unique)
        String[] ductTankMethods = gasNetworkPlus("getHeight", "getWidth");
        regBridge("clockwork_cc_compat", "duct_tank", "cw_duct_tank",
                ductTankMethods, gasNetworkImmPlus("getHeight", "getWidth"), DUCT_PROVIDER);

        // Exhaust (gas_network + unique)
        String[] exhaustMethods = gasNetworkPlus("getFacing");
        regBridge("clockwork_cc_compat", "exhaust", "cw_exhaust",
                exhaustMethods, gasNetworkImmPlus("getFacing"), DUCT_PROVIDER);

        // Gas Engine (NO gas_network!)
        String[] gasEngineMethods = {"getAttachedEngines", "getTotalEfficiency"};
        regBridge("clockwork_cc_compat", "gas_engine", "cw_gas_engine",
                gasEngineMethods, new HashSet<>(Arrays.asList(gasEngineMethods)), DUCT_PROVIDER);

        // Gas Nozzle (gas_network + many unique)
        String[] gasNozzleMethods = gasNetworkPlus(
            "setPointer", "getPointer", "getPointerSpeed",
            "getPocketTemperature", "getDuctTemperature", "getTargetTemperature",
            "getBalloonVolume", "getLeaks", "getTemperatureDelta", "hasBalloon",
            "getBuoyancyForce", "getBalloonPressure", "getBalloonGasContents", "getLossRate",
            "getInflowRate", "getMissingPositions", "getTotalGasMass", "getLeakIntegrity",
            "getMaxLeaks", "getInternalDensity"
        );
        regBridge("clockwork_cc_compat", "gas_nozzle", "cw_gas_nozzle",
                gasNozzleMethods, gasNetworkImmPlus(
                    "getPointer", "getPointerSpeed", "getPocketTemperature", "getDuctTemperature",
                    "getTargetTemperature", "getBalloonVolume", "getLeaks", "getTemperatureDelta", "hasBalloon"),
                DUCT_PROVIDER);

        // Gas Pump (gas_network + unique)
        String[] gasPumpMethods = gasNetworkPlus("getPumpPressure", "getSpeed", "getFacing");
        regBridge("clockwork_cc_compat", "gas_pump", "cw_gas_pump",
                gasPumpMethods, gasNetworkImmPlus("getPumpPressure", "getSpeed", "getFacing"), DUCT_PROVIDER);

        // Gas Thruster (gas_network + unique)
        String[] gasThrusterMethods = gasNetworkPlus("getThrust", "getFlowRate", "getGasMassFlow", "getFacing");
        regBridge("clockwork_cc_compat", "gas_thruster", "cw_gas_thruster",
                gasThrusterMethods, gasNetworkImmPlus("getThrust", "getFlowRate", "getGasMassFlow", "getFacing"),
                DUCT_PROVIDER);

        // Gas Valve (gas_network + unique)
        String[] gasValveMethods = gasNetworkPlus("getAperture", "getFacing");
        regBridge("clockwork_cc_compat", "gas_valve", "cw_gas_valve",
                gasValveMethods, gasNetworkImmPlus("getAperture", "getFacing"), DUCT_PROVIDER);

        // Radiator (gas_network + many unique)
        String[] radiatorMethods = gasNetworkPlus(
            "getFanType", "getFanRPM", "getFanCount", "getFans",
            "isActive", "isCooling", "isHeating", "getTargetTemp",
            "getInputTemperature", "getOutputTemperature", "getThermalFactor",
            "getAtmosphericPressure", "getPressureScale", "getThermalPower",
            "getStatus", "getConversionRate", "getConversions"
        );
        regBridge("clockwork_cc_compat", "radiator", "cw_radiator",
                radiatorMethods, gasNetworkImmPlus(
                    "getFanType", "getFanRPM", "getFanCount", "getFans",
                    "isActive", "isCooling", "isHeating", "getTargetTemp",
                    "getInputTemperature", "getOutputTemperature", "getThermalFactor",
                    "getAtmosphericPressure", "getPressureScale", "getThermalPower",
                    "getStatus", "getConversionRate", "getConversions"),
                DUCT_PROVIDER);

        // Redstone Duct (gas_network + unique)
        String[] redstoneDuctMethods = gasNetworkPlus("getPower", "getConditional");
        regBridge("clockwork_cc_compat", "redstone_duct", "cw_redstone_duct",
                redstoneDuctMethods, gasNetworkImmPlus("getPower", "getConditional"), DUCT_PROVIDER);
    }

    // ==================================================================
    // Create / Create mod
    // ==================================================================

    public static void registerCreate() {
        LOGGER.info("Registering Create peripherals (18 types)");

        // Creative Motor — CcBlockEntityBridge で setGeneratedSpeed を実際の peripheral に委譲
        regBE("create", "creative_motor", "Create_CreativeMotor",
            new String[]{"setGeneratedSpeed"});

        // DisplayLink - 専用実装を使用
        Block displayLinkBlock = ForgeRegistries.BLOCKS.getValue(new ResourceLocation("create", "display_link"));
        if (displayLinkBlock != null && displayLinkBlock != Blocks.AIR) {
            PeripheralProvider.register(displayLinkBlock, CreateDisplayLinkPeripheral::new);
            LOGGER.debug("Registered peripheral: create:display_link -> Create_DisplayLink");
        }

        // Frogport — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("create", "frogport", "Create_Frogport",
            new String[]{"setAddress", "setConfiguration", "getItemDetail",
                "try_pull_package_received", "try_pull_package_sent"});

        // Nixie Tube — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("create", "nixie_tube", "Create_NixieTube",
            new String[]{"setText", "setTextColour", "setSignal"});

        // Packager — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("create", "packager", "Create_Packager",
            new String[]{"makePackage", "getItemDetail", "setAddress",
                "try_pull_package_received", "try_pull_package_sent"});

        // Postbox — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("create", "postbox", "Create_Postbox",
            new String[]{"setAddress", "getItemDetail", "setConfiguration",
                "try_pull_package_received", "try_pull_package_sent"});

        // Redstone Requester — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("create", "redstone_requester", "Create_RedstoneRequester",
            new String[]{"request", "setRequest", "setCraftingRequest", "getRequest",
                "setConfiguration", "setAddress"});

        // Repackager — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("create", "repackager", "Create_Repackager",
            new String[]{"makePackage", "getItemDetail", "setAddress",
                "try_pull_package_repackaged", "try_pull_package_received", "try_pull_package_sent"});

        // Rotation Speed Controller — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("create", "rotation_speed_controller", "Create_RotationSpeedController",
            new String[]{"setTargetSpeed"});

        // Sequenced Gearshift — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("create", "sequenced_gearshift", "Create_SequencedGearshift",
            new String[]{"rotate", "moveBy"});

        // Signal — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("create", "signal", "Create_Signal",
            new String[]{"setForcedRed", "cycleSignalType", "try_pull_train_signal_state_change"});

        // Speedometer — getSpeed() は BlockEntity から直接読む (CcBlockEntityBridge)
        regBE("create", "speedometer", "Create_Speedometer",
            new String[]{"getSpeed", "try_pull_speed_change"});

        // Station - 専用実装を使用
        Block stationBlock = ForgeRegistries.BLOCKS.getValue(new ResourceLocation("create", "station"));
        if (stationBlock != null && stationBlock != Blocks.AIR) {
            PeripheralProvider.register(stationBlock, CreateStationPeripheral::new);
            LOGGER.debug("Registered peripheral: create:station -> create:station");
        }

        // Sticker — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("create", "sticker", "Create_Sticker",
            new String[]{"extend", "retract", "toggle"});

        // Stock Ticker — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("create", "stock_ticker", "Create_StockTicker",
            new String[]{"getStockItemDetail", "requestFiltered", "getItemDetail"});

        // Stressometer — getStress()/getStressCapacity() は BlockEntity から直接読む (CcBlockEntityBridge)
        regBE("create", "stressometer", "Create_Stressometer",
            new String[]{"getStress", "getStressCapacity", "try_pull_overstressed", "try_pull_stress_change"});

        // Tablecloth Shop — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("create", "tablecloth_shop", "Create_TableClothShop",
            new String[]{"setAddress", "setPriceTagItem", "setPriceTagCount", "setWares"});

        // Track Observer — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("create", "track_observer", "Create_TrainObserver",
            new String[]{"try_pull_train_passing", "try_pull_train_passed"});
    }

    // ==================================================================
    // Create Addition / Create Addition mod
    // ==================================================================

    public static void registerCreateAddition() {
        LOGGER.info("Registering Create Addition peripherals (5 types)");

        // Digital Adapter — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("createaddition", "digital_adapter", "digital_adapter",
            new String[]{
                "clearLine", "clear", "print", "getLine", "setLine", "getMaxLines",
                "setTargetSpeed", "getTargetSpeed", "getKineticStress", "getKineticCapacity",
                "getKineticSpeed", "getKineticTopSpeed", "getPulleyDistance", "getPistonDistance",
                "getBearingAngle", "getElevatorFloor", "hasElevatorArrived", "getElevatorFloors",
                "getElevatorFloorName", "gotoElevatorFloor", "getDurationAngle", "getDurationDistance"
            });

        // Electric Motor — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("createaddition", "electric_motor", "electric_motor",
            new String[]{
                "getType", "setSpeed", "stop", "getSpeed", "getStressCapacity",
                "getEnergyConsumption", "rotate", "translate", "getMaxInsert", "getMaxExtract"
            });

        // Modular Accumulator — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("createaddition", "modular_accumulator", "modular_accumulator",
            new String[]{
                "getEnergy", "getCapacity", "getPercent", "getMaxInsert", "getMaxExtract",
                "getHeight", "getWidth"
            });

        // Portable Energy Interface — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("createaddition", "portable_energy_interface", "portable_energy_interface",
            new String[]{"getEnergy", "getCapacity", "isConnected", "getMaxInsert", "getMaxExtract"});

        // Redstone Relay — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("createaddition", "redstone_relay", "redstone_relay",
            new String[]{"getMaxInsert", "getMaxExtract", "getThroughput", "isPowered"});
    }

    // ==================================================================
    // Control-Craft / Control-Craft mod
    // ==================================================================

    public static void registerControlCraft() {
        LOGGER.info("Registering Control-Craft peripherals (14 types) via CcBlockEntityBridge");

        // Camera
        String[] cameraMethods = {
            "getAbsViewTransform", "getPitch", "getYaw", "getTransformedPitch", "getTransformedYaw",
            "getClipDistance", "latestShip", "latestPlayer", "latestEntity", "latestBlock",
            "getCameraPosition", "getAbsViewForward", "isBeingUsed", "getDirection",
            "clip", "clipEntity", "clipBlock", "clipAllEntity", "clipShip", "clipPlayer",
            "setPitch", "setYaw", "outlineToUser", "forcePitchYaw", "setClipRange", "setConeAngle",
            "raycast", "getEntities", "getMobs", "reset"
        };
        Set<String> cameraImm = new HashSet<>(Arrays.asList(
            "getAbsViewTransform", "getPitch", "getYaw", "getTransformedPitch", "getTransformedYaw",
            "getClipDistance", "latestShip", "latestPlayer", "latestEntity", "latestBlock",
            "getCameraPosition", "getAbsViewForward", "isBeingUsed", "getDirection"
        ));
        regBE("controlcraft", "camera", "camera", cameraMethods, cameraImm);

        // Cannon Mount
        String[] cannonMountMethods = {"getPitch", "getYaw", "setPitch", "setYaw", "assemble", "disassemble"};
        regBE("controlcraft", "cannon_mount", "controlcraft\\$cannon_mount",
                cannonMountMethods, new HashSet<>(Arrays.asList("getPitch", "getYaw")));

        // Compact Flap
        String[] compactFlapMethods = {"getAngle", "getTilt", "setAngle", "setTilt"};
        regBE("controlcraft", "compact_flap", "compact_flap",
                compactFlapMethods, new HashSet<>(Arrays.asList("getAngle", "getTilt")));

        // Dynamic Motor
        String[] dynMotorMethods = {
            "getTargetValue", "getPhysics", "getAngle", "getAngularVelocity",
            "getCurrentValue", "getRelative", "isLocked",
            "setPID", "setTargetValue", "setOutputTorque", "setIsAdjustingAngle", "lock", "unlock"
        };
        regBE("controlcraft", "dynamic_motor", "servo",
                dynMotorMethods, new HashSet<>(Arrays.asList(
                    "getTargetValue", "getPhysics", "getAngle", "getAngularVelocity",
                    "getCurrentValue", "getRelative", "isLocked")));

        // Flap Bearing
        String[] flapBearingMethods = {"getAngle", "setAngle", "assembleNextTick", "disassembleNextTick"};
        regBE("controlcraft", "flap_bearing", "WingController",
                flapBearingMethods, new HashSet<>(Arrays.asList("getAngle")));

        // Jet
        regBE("controlcraft", "jet", "attacker",
            new String[]{"setOutputThrust", "setHorizontalTilt", "setVerticalTilt"});

        // Kinematic Motor
        String[] kinMotorMethods = {
            "getTargetAngle", "getControlTarget", "getPhysics", "getAngle", "getRelative",
            "setTargetAngle", "setControlTarget", "setIsForcingAngle"
        };
        regBE("controlcraft", "kinematic_motor", "servo",
                kinMotorMethods, new HashSet<>(Arrays.asList(
                    "getTargetAngle", "getControlTarget", "getPhysics", "getAngle", "getRelative")));

        // Kinetic Resistor
        String[] kinResistorMethods = {"getRatio", "setRatio"};
        regBE("controlcraft", "kinetic_resistor", "resistor",
                kinResistorMethods, new HashSet<>(Arrays.asList("getRatio")));

        // Link Bridge
        String[] linkBridgeMethods = {"setInput", "getOutput"};
        regBE("controlcraft", "link_bridge", "cc_link_bridge",
                linkBridgeMethods, new HashSet<>(Arrays.asList("getOutput")));

        // Propeller Controller
        String[] propCtrlMethods = {"getTargetSpeed", "setTargetSpeed"};
        regBE("controlcraft", "propeller_controller", "PropellerController",
                propCtrlMethods, new HashSet<>(Arrays.asList("getTargetSpeed")));

        // Slider
        String[] sliderMethods = {
            "getDistance", "getCurrentValue", "getTargetValue", "getPhysics", "isLocked",
            "setOutputForce", "setPID", "setTargetValue", "lock", "unlock"
        };
        regBE("controlcraft", "slider", "slider",
                sliderMethods, new HashSet<>(Arrays.asList(
                    "getDistance", "getCurrentValue", "getTargetValue", "getPhysics", "isLocked")));

        // Spatial Anchor
        regBE("controlcraft", "spatial_anchor", "spatial",
            new String[]{"setStatic", "setRunning", "setOffset", "setPPID", "setQPID", "setChannel"});

        // Spinalyzer
        String[] spinalyzerMethods = {
            "getQuaternion", "getQuaternionJ", "getRotationMatrix", "getRotationMatrixT",
            "getVelocity", "getAngularVelocity", "getPosition",
            "getSpinalyzerPosition", "getSpinalyzerVelocity", "getPhysics",
            "applyInvariantForce", "applyInvariantTorque", "applyRotDependentForce", "applyRotDependentTorque"
        };
        regBE("controlcraft", "spinalyzer", "spinalyzer",
                spinalyzerMethods, new HashSet<>(Arrays.asList(
                    "getQuaternion", "getQuaternionJ", "getRotationMatrix", "getRotationMatrixT",
                    "getVelocity", "getAngularVelocity", "getPosition",
                    "getSpinalyzerPosition", "getSpinalyzerVelocity", "getPhysics")));

        // Transmitter
        regBE("controlcraft", "transmitter", "transmitter",
            new String[]{"callRemote", "callRemoteAsync", "setProtocol"});
    }

    // ==================================================================
    // Advanced Peripherals / Advanced Peripherals mod
    // ==================================================================

    public static void registerAdvancedPeripherals() {
        LOGGER.info("Registering Advanced Peripherals (12 types)");

        // BlockReader - 専用実装を使用
        Block blockReaderBlock = ForgeRegistries.BLOCKS.getValue(new ResourceLocation("advancedperipherals", "block_reader"));
        if (blockReaderBlock != null && blockReaderBlock != Blocks.AIR) {
            PeripheralProvider.register(blockReaderBlock, ApBlockReaderPeripheral::new);
            LOGGER.debug("Registered peripheral: advancedperipherals:block_reader -> block_reader");
        }

        // ChatBox - 専用実装を使用
        Block chatBoxBlock = ForgeRegistries.BLOCKS.getValue(new ResourceLocation("advancedperipherals", "chat_box"));
        if (chatBoxBlock != null && chatBoxBlock != Blocks.AIR) {
            PeripheralProvider.register(chatBoxBlock, ApChatBoxPeripheral::new);
            LOGGER.debug("Registered peripheral: advancedperipherals:chat_box -> chat_box");
        }

        // Colony Integrator — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("advancedperipherals", "colony_integrator", "colony_integrator",
            new String[]{
                "isInColony", "getColonyID", "getColonyName", "getColonyStyle", "isActive",
                "getAmountOfCitizens", "getMaxCitizens", "getHappiness", "getPosition",
                "getCitizens", "getCitizenInfo", "getBuildings", "getBuildingInfo",
                "getWorkOrders", "getRequests", "getBuilderResources", "isUnderAttack"
            });

        // Compass — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("advancedperipherals", "compass", "compass",
            new String[]{"getFacing"});

        // Energy Detector — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("advancedperipherals", "energy_detector", "energy_detector",
            new String[]{"getTransferRate", "getTransferRateLimit", "setTransferRateLimit"});

        // Environment Detector
        String[] envDetMethods = {
            "getBiome", "getDimension", "isDimension", "listDimensions",
            "isRaining", "isThunder", "isSunny", "getSkyLightLevel", "getBlockLightLevel",
            "getDayLightLevel", "getTime", "getMoonId", "getMoonName", "isMoon",
            "isSlimeChunk", "canSleepHere", "canSleepPlayer", "scanEntities", "scanCost"
        };
        regWithImm("advancedperipherals", "environment_detector", "environment_detector",
                envDetMethods, new HashSet<>(Arrays.asList("scanCost")));

        // Geo Scanner - 専用実装を使用
        Block geoScannerBlock = ForgeRegistries.BLOCKS.getValue(new ResourceLocation("advancedperipherals", "geo_scanner"));
        if (geoScannerBlock != null && geoScannerBlock != Blocks.AIR) {
            PeripheralProvider.register(geoScannerBlock, ApGeoScannerPeripheral::new);
            LOGGER.debug("Registered peripheral: advancedperipherals:geo_scanner -> geo_scanner");
        }

        // Inventory Manager — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("advancedperipherals", "inventory_manager", "inventory_manager",
            new String[]{
                "getOwner", "addItemToPlayer", "removeItemFromPlayer", "list", "getArmor",
                "isPlayerEquipped", "isWearing", "getItemInHand", "getItemInOffHand",
                "getEmptySpace", "isSpaceAvailable", "getFreeSlot", "listChest"
            });

        // ME Bridge - 専用実装を使用
        Block meBridgeBlock = ForgeRegistries.BLOCKS.getValue(new ResourceLocation("advancedperipherals", "me_bridge"));
        if (meBridgeBlock != null && meBridgeBlock != Blocks.AIR) {
            PeripheralProvider.register(meBridgeBlock, ApMEBridgePeripheral::new);
            LOGGER.debug("Registered peripheral: advancedperipherals:me_bridge -> me_bridge");
        }

        // NBT Storage — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("advancedperipherals", "nbt_storage", "nbt_storage",
            new String[]{"read", "writeJson", "writeTable"});

        // PlayerDetector - 専用実装を使用
        Block playerDetectorBlock = ForgeRegistries.BLOCKS.getValue(new ResourceLocation("advancedperipherals", "player_detector"));
        if (playerDetectorBlock != null && playerDetectorBlock != Blocks.AIR) {
            PeripheralProvider.register(playerDetectorBlock, ApPlayerDetectorPeripheral::new);
            LOGGER.debug("Registered peripheral: advancedperipherals:player_detector -> player_detector");
        }

        // RS Bridge (same methods as ME Bridge + chemical storage getters) — CcBlockEntityBridge で実際の peripheral に委譲
        regBE("advancedperipherals", "rs_bridge", "rs_bridge",
            new String[]{
                "listItems", "getItem", "exportItem", "importItem", "exportItemToPeripheral",
                "importItemFromPeripheral", "listFluids", "getFluid", "exportFluid", "importFluid",
                "exportFluidToPeripheral", "importFluidFromPeripheral", "listChemicals", "getChemical",
                "exportChemical", "importChemical", "exportChemicalToPeripheral", "importChemicalFromPeripheral",
                "craftItem", "craftFluid", "craftChemical", "isItemCrafting", "isFluidCrafting",
                "getEnergyStorage", "getMaxEnergyStorage", "getAvgPowerUsage", "getAvgPowerInjection",
                "getTotalItemStorage", "getUsedItemStorage", "getAvailableItemStorage",
                "getTotalFluidStorage", "getUsedFluidStorage", "getAvailableFluidStorage",
                "getTotalChemicalStorage", "getUsedChemicalStorage", "getAvailableChemicalStorage"
            });
    }

    // ==================================================================
    // CBC CC Control / CBC CC Control mod
    // ==================================================================

    public static void registerCbcCcControl() {
        LOGGER.info("Registering CBC CC Control peripherals (cannon mount)");

        String[] cannonMethods = {"isRunning", "getYaw", "getPitch", "assemble", "disassemble", "setYaw", "setPitch", "fire"};
        regWithImm("createbigcannons", "cannon_mount", "cbc_cannon_mount",
                cannonMethods, new HashSet<>(Arrays.asList("isRunning", "getYaw", "getPitch")));
    }
}
