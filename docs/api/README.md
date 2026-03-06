# RustComputers API リファレンス / API Reference

**バージョン / Version**: v0.1.16


各ペリフェラルの Rust ラッパー API ドキュメント。\

Rust wrapper API documentation for each peripheral.


## ペリフェラル一覧 / Peripheral List

### CC:Tweaked (1 peripherals, 18 methods)

| Rust 型 | モジュールパス | メソッド数 |
|---------|--------------|---------|
| [`Monitor`](./computer_craft/Monitor.md) | `rc::computer_craft::monitor::Monitor` | 18 |

### Clockwork CC Compat (13 peripherals, 69 methods)

| Rust 型 | モジュールパス | メソッド数 |
|---------|--------------|---------|
| [`AirCompressor`](./clockwork/AirCompressor.md) | `rc::clockwork::air_compressor::AirCompressor` | 3 |
| [`Boiler`](./clockwork/Boiler.md) | `rc::clockwork::boiler::Boiler` | 14 |
| [`CoalBurner`](./clockwork/CoalBurner.md) | `rc::clockwork::coal_burner::CoalBurner` | 3 |
| [`DuctTank`](./clockwork/DuctTank.md) | `rc::clockwork::duct_tank::DuctTank` | 2 |
| [`Exhaust`](./clockwork/Exhaust.md) | `rc::clockwork::exhaust::Exhaust` | 1 |
| [`GasEngine`](./clockwork/GasEngine.md) | `rc::clockwork::gas_engine::GasEngine` | 2 |
| [`GasNetwork`](./clockwork/GasNetwork.md) | `rc::clockwork::gas_network::GasNetwork` | 3 |
| [`GasNozzle`](./clockwork/GasNozzle.md) | `rc::clockwork::gas_nozzle::GasNozzle` | 18 |
| [`GasPump`](./clockwork/GasPump.md) | `rc::clockwork::gas_pump::GasPump` | 3 |
| [`GasThruster`](./clockwork/GasThruster.md) | `rc::clockwork::gas_thruster::GasThruster` | 3 |
| [`GasValve`](./clockwork/GasValve.md) | `rc::clockwork::gas_valve::GasValve` | 2 |
| [`Radiator`](./clockwork/Radiator.md) | `rc::clockwork::radiator::Radiator` | 14 |
| [`RedstoneDuct`](./clockwork/RedstoneDuct.md) | `rc::clockwork::redstone_duct::RedstoneDuct` | 1 |

### Control-Craft (10 peripherals, 51 methods)

| Rust 型 | モジュールパス | メソッド数 |
|---------|--------------|---------|
| [`Camera`](./control_craft/Camera.md) | `rc::control_craft::camera::Camera` | 8 |
| [`DynamicMotor`](./control_craft/DynamicMotor.md) | `rc::control_craft::dynamic_motor::DynamicMotor` | 11 |
| [`FlapBearing`](./control_craft/FlapBearing.md) | `rc::control_craft::flap_bearing::FlapBearing` | 4 |
| [`Jet`](./control_craft/Jet.md) | `rc::control_craft::jet::Jet` | 3 |
| [`KinematicMotor`](./control_craft/KinematicMotor.md) | `rc::control_craft::kinematic_motor::KinematicMotor` | 6 |
| [`KineticResistor`](./control_craft/KineticResistor.md) | `rc::control_craft::kinetic_resistor::KineticResistor` | 2 |
| [`LinkBridge`](./control_craft/LinkBridge.md) | `rc::control_craft::link_bridge::LinkBridge` | 2 |
| [`PropellerController`](./control_craft/PropellerController.md) | `rc::control_craft::propeller_controller::PropellerController` | 2 |
| [`Slider`](./control_craft/Slider.md) | `rc::control_craft::slider::Slider` | 9 |
| [`Spinalyzer`](./control_craft/Spinalyzer.md) | `rc::control_craft::spinalyzer::Spinalyzer` | 4 |

### Create (5 peripherals, 17 methods)

| Rust 型 | モジュールパス | メソッド数 |
|---------|--------------|---------|
| [`NixieTube`](./create/NixieTube.md) | `rc::create::nixie_tube::NixieTube` | 2 |
| [`SpeedController`](./create/SpeedController.md) | `rc::create::speed_controller::SpeedController` | 2 |
| [`SpeedGauge`](./create/SpeedGauge.md) | `rc::create::speed_gauge::SpeedGauge` | 1 |
| [`Station`](./create/Station.md) | `rc::create::station::Station` | 10 |
| [`StressGauge`](./create/StressGauge.md) | `rc::create::stress_gauge::StressGauge` | 2 |

### Create Additions (2 peripherals, 15 methods)

| Rust 型 | モジュールパス | メソッド数 |
|---------|--------------|---------|
| [`ElectricMotor`](./createaddition/ElectricMotor.md) | `rc::createaddition::electric_motor::ElectricMotor` | 8 |
| [`ModularAccumulator`](./createaddition/ModularAccumulator.md) | `rc::createaddition::modular_accumulator::ModularAccumulator` | 7 |

### AdvancedPeripherals (21 peripherals, 96 methods)

| Rust 型 | モジュールパス | メソッド数 |
|---------|--------------|---------|
| [`AutomataWarping`](./advanced_peripherals/AutomataWarping.md) | `rc::advanced_peripherals::automata_warping::AutomataWarping` | 5 |
| [`Beacon`](./advanced_peripherals/Beacon.md) | `rc::advanced_peripherals::beacon::Beacon` | 3 |
| [`BlockReader`](./advanced_peripherals/BlockReader.md) | `rc::advanced_peripherals::block_reader::BlockReader` | 2 |
| [`ColonyIntegrator`](./advanced_peripherals/ColonyIntegrator.md) | `rc::advanced_peripherals::colony_integrator::ColonyIntegrator` | 12 |
| [`Compass`](./advanced_peripherals/Compass.md) | `rc::advanced_peripherals::compass::Compass` | 1 |
| [`EnergyDetector`](./advanced_peripherals/EnergyDetector.md) | `rc::advanced_peripherals::energy_detector::EnergyDetector` | 2 |
| [`EnvironmentDetector`](./advanced_peripherals/EnvironmentDetector.md) | `rc::advanced_peripherals::environment_detector::EnvironmentDetector` | 16 |
| [`GeoScanner`](./advanced_peripherals/GeoScanner.md) | `rc::advanced_peripherals::geo_scanner::GeoScanner` | 1 |
| [`InventoryManager`](./advanced_peripherals/InventoryManager.md) | `rc::advanced_peripherals::inventory_manager::InventoryManager` | 6 |
| [`MeBridge`](./advanced_peripherals/MeBridge.md) | `rc::advanced_peripherals::me_bridge::MeBridge` | 7 |
| [`NbtStorage`](./advanced_peripherals/NbtStorage.md) | `rc::advanced_peripherals::nbt_storage::NbtStorage` | 1 |
| [`NoteBlock`](./advanced_peripherals/NoteBlock.md) | `rc::advanced_peripherals::note_block::NoteBlock` | 4 |
| [`PlayerDetector`](./advanced_peripherals/PlayerDetector.md) | `rc::advanced_peripherals::player_detector::PlayerDetector` | 4 |
| [`PowahEnderCell`](./advanced_peripherals/PowahEnderCell.md) | `rc::advanced_peripherals::powah_ender_cell::PowahEnderCell` | 5 |
| [`PowahEnergyCell`](./advanced_peripherals/PowahEnergyCell.md) | `rc::advanced_peripherals::powah_energy_cell::PowahEnergyCell` | 2 |
| [`PowahFurnator`](./advanced_peripherals/PowahFurnator.md) | `rc::advanced_peripherals::powah_furnator::PowahFurnator` | 4 |
| [`PowahMagmator`](./advanced_peripherals/PowahMagmator.md) | `rc::advanced_peripherals::powah_magmator::PowahMagmator` | 3 |
| [`PowahReactor`](./advanced_peripherals/PowahReactor.md) | `rc::advanced_peripherals::powah_reactor::PowahReactor` | 7 |
| [`PowahSolarPanel`](./advanced_peripherals/PowahSolarPanel.md) | `rc::advanced_peripherals::powah_solar_panel::PowahSolarPanel` | 3 |
| [`PowahThermo`](./advanced_peripherals/PowahThermo.md) | `rc::advanced_peripherals::powah_thermo::PowahThermo` | 2 |
| [`RsBridge`](./advanced_peripherals/RsBridge.md) | `rc::advanced_peripherals::rs_bridge::RsBridge` | 6 |

### Some-Peripherals (5 peripherals, 14 methods)

| Rust 型 | モジュールパス | メソッド数 |
|---------|--------------|---------|
| [`BallisticAccelerator`](./some_peripherals/BallisticAccelerator.md) | `rc::some_peripherals::ballistic_accelerator::BallisticAccelerator` | 1 |
| [`Digitizer`](./some_peripherals/Digitizer.md) | `rc::some_peripherals::digitizer::Digitizer` | 5 |
| [`Radar`](./some_peripherals/Radar.md) | `rc::some_peripherals::radar::Radar` | 5 |
| [`Raycaster`](./some_peripherals/Raycaster.md) | `rc::some_peripherals::raycaster::Raycaster` | 2 |
| [`WorldScanner`](./some_peripherals/WorldScanner.md) | `rc::some_peripherals::world_scanner::WorldScanner` | 1 |

---

**合計 / Total**: 57 ペリフェラル、280 メソッド

---
*自動生成 / Auto-generated from `peripherals/` TOML manifests.*
