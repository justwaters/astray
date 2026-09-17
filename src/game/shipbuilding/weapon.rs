use derive_getters::Getters;
use serde::Deserialize;

use crate::game::shipbuilding::module_trait::ModuleTrait;
use crate::game::shipbuilding::ship_module::ShipModule;

#[derive(Clone, Deserialize, Getters)]
pub struct Weapon {
    name: String,
    traits: Vec<ModuleTrait>,
    is_unlocked: bool,
    #[serde(default)]
    required_research_id: Option<String>,
    damage: u32,
}

impl ShipModule for Weapon {
    fn get_traits(&self) -> Vec<ModuleTrait> {
        self.traits.clone()
    }
}
