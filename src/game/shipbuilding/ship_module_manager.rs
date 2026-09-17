use crate::game::shipbuilding::ship_module::{ShipModule, ShipModuleType};
use crate::game::shipbuilding::sublight_engine::SublightEngine;
use crate::game::shipbuilding::weapon::Weapon;

pub struct ShipModuleManager {
    sublight_engines: Vec<SublightEngine>,
    weapons: Vec<Weapon>,
}

impl ShipModuleManager {
    pub fn new() -> Self {
        Self {
            sublight_engines: SublightEngine::load_from_str(
                include_str!("../../../assets/ship_modules/sublight_engines.json5")
            ),
            weapons: Weapon::load_from_str(
                include_str!("../../../assets/ship_modules/weapons.json5")
            ),
        }
    }

    pub fn get_ship_module_types(&self) -> Vec<ShipModuleType> {
        vec![
            ShipModuleType::SublightThruster,
            ShipModuleType::Weapon,
        ]
    }

    pub fn get_ship_module_type_by_name(&self, name: String) -> ShipModuleType {
        ShipModuleType::from(name)
    }

    /// Returns `(name, is_unlocked, required_research_id)` for every sublight engine design.
    pub fn get_sublight_engine_designs(&self) -> Vec<(String, bool, Option<String>)> {
        self.sublight_engines.iter()
            .map(|e| (e.name().clone(), *e.is_unlocked(), e.required_research_id().clone()))
            .collect()
    }

    /// Returns `(name, is_unlocked, required_research_id)` for every weapon design.
    pub fn get_weapon_designs(&self) -> Vec<(String, bool, Option<String>)> {
        self.weapons.iter()
            .map(|w| (w.name().clone(), *w.is_unlocked(), w.required_research_id().clone()))
            .collect()
    }

    pub fn get_weapon_damage(&self, name: &str) -> Option<u32> {
        self.weapons.iter().find(|w| w.name() == name).map(|w| *w.damage())
    }
}
