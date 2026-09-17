use crate::game::shipbuilding::ship_module::{ShipModule, ShipModuleType};
use crate::game::shipbuilding::sublight_engine::SublightEngine;

pub struct ShipModuleManager {
    sublight_engines: Vec<SublightEngine>,
}

impl ShipModuleManager {
    pub fn new() -> Self {
        Self {
            sublight_engines: SublightEngine::load_from_str(
                include_str!("../../../assets/ship_modules/sublight_engines.json5")
            ),
        }
    }

    pub fn get_ship_module_types(&self) -> Vec<ShipModuleType> {
        vec![
            ShipModuleType::SublightThruster,
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
}