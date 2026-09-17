use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::game::celestial_bodies::{CelestialBody, Displayable, Orbitable};
use crate::game::celestial_bodies::planet::Planet;
use crate::game::celestial_bodies::solar_system::SolarSystem;
use crate::game::colony::building::BuildingType;
use crate::game::colony::colony::Colony;
use crate::game::research::research_manager::ResearchManager;
use crate::game::resource::resource::ResourceType;
use crate::game::shipbuilding::ship::Ship;
use crate::game::shipbuilding::ship_module::ShipModuleType;
use crate::game::shipbuilding::ship_module_manager::ShipModuleManager;

/// Number of in-game ticks it takes to build a ship once a design is chosen.
const SHIP_BUILD_TIME: u32 = 20;
/// Engine Nozzles spent from the capital colony's stockpile to start building a ship.
const SHIP_ENGINE_NOZZLE_COST: u32 = 5;
/// Microprocessors spent from the capital colony's stockpile to start building a ship.
const SHIP_MICROPROCESSOR_COST: u32 = 5;
/// Hit points of any newly built ship.
const SHIP_MAX_HP: i32 = 20;
/// Hit points of the scripted enemy ship guarding the system.
const ENEMY_MAX_HP: i32 = 40;
/// Damage the enemy ship deals to the player's active ship each tick.
const ENEMY_DAMAGE_PER_TICK: i32 = 3;

/// A snapshot of shipyard and combat state for the UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ShipyardInfo {
    pub engine_design: Option<String>,
    pub weapon_design: Option<String>,
    pub ships_built: u32,
    pub build_progress_percent: Option<u32>,
    pub available_nozzles: u32,
    pub nozzle_cost: u32,
    pub available_microprocessors: u32,
    pub microprocessor_cost: u32,
    pub enemy_hp: i32,
    pub enemy_max_hp: i32,
    pub active_ship_hp: Option<i32>,
    pub ship_max_hp: i32,
}

pub struct GameState {
    systems: Vec<SolarSystem>,
    capital: Planet,
    capital_system: SolarSystem,
    colonies: Vec<Colony>,
    resource_tick_ratio: u32,
    resource_tick_counter: u32,
    research_manager: ResearchManager,
    ship_module_manager: ShipModuleManager,
    ships: Vec<Ship>,
    engine_design: Option<String>,
    weapon_design: Option<String>,
    ship_build_progress: Option<u32>,
    active_ship_hp: Option<i32>,
    active_ship_weapon_damage: Option<u32>,
    enemy_hp: i32,
}

impl Default for GameState {
    fn default() -> Self {
        let mut system: SolarSystem;
        let capital_planet: Planet;
        loop {
            system = SolarSystem::generate(());
            if let Some(planet) = system.has_planets_in_habitable_zone() {
                capital_planet = planet;
                break
            }
        }

        Self {
            systems: vec![system.clone()],
            capital: capital_planet.clone(),
            capital_system: system,
            colonies: vec![
                Colony::new(
                    capital_planet.get_name(),
                    5_000,
                )
            ],
            resource_tick_counter: 0,
            resource_tick_ratio: 2,
            research_manager: ResearchManager::new(),

            ship_module_manager: ShipModuleManager::new(),
            ships: Vec::new(),
            engine_design: None,
            weapon_design: None,
            ship_build_progress: None,
            active_ship_hp: None,
            active_ship_weapon_damage: None,
            enemy_hp: ENEMY_MAX_HP,
        }
    }
}

impl GameState {
    pub fn tick(&mut self) {
        self.update_research();
        self.update_colonies();
        self.update_orbits();
        self.update_shipyard();
        self.update_combat();
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_starting_system(&self) -> SolarSystem {
        self.systems[0].clone()
    }

    pub fn get_research_fields(&self) -> Vec<(String, String, Color)> {
        self.research_manager.get_research_fields()
    }

    pub fn get_research_info(&self, id: String) -> Vec<Vec<String>> {
        self.research_manager.get_research_info(id)
    }

    pub fn get_research_dependency_info(&self, id: String) -> Vec<Vec<(String, bool)>> {
        self.research_manager.get_dependency_info(id)
    }

    pub fn get_researches_by_field(&self, id: String) -> Vec<(String, String, Color)> {
        self.research_manager.get_researches_with_colors_by_field(id)
    }

    pub fn get_research_progress_text(&self, id: String) -> String {
        self.research_manager.get_research_text(id)
    }

    pub fn get_research_progress(&self, id: String) -> u32 {
        self.research_manager.get_research_progress(id)
    }

    pub fn start_research(&mut self, id: String) {
        self.research_manager.start_research(id)
    }

    fn update_research(&mut self) {
        self.research_manager.tick();
    }

    fn update_colonies(&mut self) {
        self.colonies.iter_mut().for_each(|c| c.tick());
        self.resource_tick_counter += 1;
        if self.resource_tick_ratio == self.resource_tick_counter {
            self.resource_tick_counter = 0;
            self.colonies.iter_mut().for_each(|c| c.resource_tick());
        }
    }

    fn update_orbits(&mut self) {
        for system in self.systems.as_mut_slice() {
            system.update_orbits();
        }
    }

    fn update_shipyard(&mut self) {
        if let Some(progress) = self.ship_build_progress {
            let progress = progress + 1;
            if progress >= SHIP_BUILD_TIME {
                let engine = self.engine_design.clone().unwrap();
                let weapon = self.weapon_design.clone().unwrap();
                let damage = self.ship_module_manager.get_weapon_damage(&weapon).unwrap_or(0);
                self.ships.push(Ship::new(engine, weapon));
                self.active_ship_hp = Some(SHIP_MAX_HP);
                self.active_ship_weapon_damage = Some(damage);
                self.ship_build_progress = None;
            } else {
                self.ship_build_progress = Some(progress);
            }
        }
    }

    /// Resolves one round of combat between the active ship and the scripted enemy, if
    /// both are alive. The enemy fires back only if it survives the player's attack.
    fn update_combat(&mut self) {
        if self.enemy_hp <= 0 {
            return
        }

        let (Some(ship_hp), Some(weapon_damage)) = (self.active_ship_hp, self.active_ship_weapon_damage) else {
            return
        };
        if ship_hp <= 0 {
            return
        }

        self.enemy_hp = (self.enemy_hp - weapon_damage as i32).max(0);
        if self.enemy_hp > 0 {
            self.active_ship_hp = Some((ship_hp - ENEMY_DAMAGE_PER_TICK).max(0));
        }
    }

    pub fn get_colonies(&self) -> Vec<Colony> {
        self.colonies.clone()
    }

    pub fn start_construction(&mut self, colony: Colony, building: BuildingType) {
        self.colonies.iter_mut().find(|c| c == &&colony).unwrap()
            .start_construction(building)
    }

    pub fn get_colony_by_name(&self, name: String) -> Option<Colony> {
        self.colonies.iter().find(|c| c.get_name() == name).cloned()
    }

    pub fn get_ship_module_types(&self) -> Vec<ShipModuleType> {
        self.ship_module_manager.get_ship_module_types()
    }

    /// Returns the designs available to the player for the given module type, i.e. those
    /// that are unlocked by default or whose required research is finished.
    pub fn get_ship_modules_for_type(&self, type_name: String) -> Vec<(String, Color)> {
        let designs = if type_name == ShipModuleType::Weapon.get_name() {
            self.ship_module_manager.get_weapon_designs()
        } else {
            self.ship_module_manager.get_sublight_engine_designs()
        };

        designs.into_iter()
            .filter(|(_, is_unlocked, required_research_id)| {
                *is_unlocked || required_research_id.as_ref()
                    .is_some_and(|id| self.research_manager.is_research_finished(id.clone()))
            })
            .map(|(name, _, _)| (name, Color::White))
            .collect()
    }

    pub fn set_ship_design(&mut self, type_name: String, module_name: String) {
        if type_name == ShipModuleType::Weapon.get_name() {
            self.weapon_design = Some(module_name);
        } else {
            self.engine_design = Some(module_name);
        }
    }

    /// Starts building a ship if both an engine and a weapon are designed, no build is
    /// already in progress, and the capital colony can afford the component cost.
    /// Returns whether the build started.
    pub fn build_ship(&mut self) -> bool {
        if self.engine_design.is_none() || self.weapon_design.is_none() || self.ship_build_progress.is_some() {
            return false
        }

        let Some(capital) = self.colonies.first_mut() else { return false };
        let affordable = capital.get_resource_amount(&ResourceType::CEngineNozzles) >= SHIP_ENGINE_NOZZLE_COST
            && capital.get_resource_amount(&ResourceType::CMicroprocessors) >= SHIP_MICROPROCESSOR_COST;

        if !affordable {
            return false
        }

        capital.try_spend_resource(ResourceType::CEngineNozzles, SHIP_ENGINE_NOZZLE_COST);
        capital.try_spend_resource(ResourceType::CMicroprocessors, SHIP_MICROPROCESSOR_COST);
        self.ship_build_progress = Some(0);
        true
    }

    pub fn get_shipyard_info(&self) -> ShipyardInfo {
        let build_progress_percent = self.ship_build_progress
            .map(|p| (p * 100 / SHIP_BUILD_TIME).min(100));
        let (available_nozzles, available_microprocessors) = self.colonies.first()
            .map(|c| (
                c.get_resource_amount(&ResourceType::CEngineNozzles),
                c.get_resource_amount(&ResourceType::CMicroprocessors),
            ))
            .unwrap_or((0, 0));

        ShipyardInfo {
            engine_design: self.engine_design.clone(),
            weapon_design: self.weapon_design.clone(),
            ships_built: self.ships.len() as u32,
            build_progress_percent,
            available_nozzles,
            nozzle_cost: SHIP_ENGINE_NOZZLE_COST,
            available_microprocessors,
            microprocessor_cost: SHIP_MICROPROCESSOR_COST,
            enemy_hp: self.enemy_hp,
            enemy_max_hp: ENEMY_MAX_HP,
            active_ship_hp: self.active_ship_hp,
            ship_max_hp: SHIP_MAX_HP,
        }
    }

    pub fn has_won(&self) -> bool {
        self.enemy_hp <= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn research_unlocks_ship_modules() {
        let mut state = GameState::new();

        assert!(state.get_ship_modules_for_type("Sublight Thruster".to_string()).is_empty());
        assert!(state.get_ship_modules_for_type("Weapon".to_string()).is_empty());

        state.start_research("ion-drive".to_string());
        state.start_research("ion-cannon".to_string());
        for _ in 0..100 {
            state.tick();
        }

        let engines = state.get_ship_modules_for_type("Sublight Thruster".to_string());
        assert_eq!(engines.len(), 1);
        assert_eq!(engines[0].0, "Ion drive");

        let weapons = state.get_ship_modules_for_type("Weapon".to_string());
        assert_eq!(weapons.len(), 1);
        assert_eq!(weapons[0].0, "Ion Cannon");
    }

    #[test]
    fn building_a_ship_requires_both_designs_and_resources() {
        let mut state = GameState::new();

        // Neither design chosen yet.
        assert!(!state.build_ship());

        state.set_ship_design("Sublight Thruster".to_string(), "Ion drive".to_string());
        // Only the engine is designed; still shouldn't build.
        assert!(!state.build_ship());

        state.set_ship_design("Weapon".to_string(), "Ion Cannon".to_string());
        // Both designed, but the capital colony has no stockpiled components yet.
        assert!(!state.build_ship());
        assert!(state.ship_build_progress.is_none());
    }

    #[test]
    fn defeating_the_enemy_wins_the_game() {
        let mut state = GameState::new();

        // Drive the tick-completion logic directly rather than through the full,
        // RNG-driven mining/production chain that would otherwise need to produce
        // enough components first.
        state.engine_design = Some("Ion drive".to_string());
        state.weapon_design = Some("Ion Cannon".to_string());
        state.ship_build_progress = Some(0);
        assert!(!state.has_won());

        for _ in 0..SHIP_BUILD_TIME {
            state.tick();
        }

        // The ship is built and now fighting the enemy; the fight itself takes several
        // more ticks (40 HP enemy vs. 6 damage/tick from the Ion Cannon).
        assert_eq!(state.ships.len(), 1);
        assert!(!state.has_won());

        for _ in 0..20 {
            state.tick();
        }

        assert!(state.has_won());
        let info = state.get_shipyard_info();
        assert_eq!(info.engine_design, Some("Ion drive".to_string()));
        assert_eq!(info.weapon_design, Some("Ion Cannon".to_string()));
        assert_eq!(info.ships_built, 1);
        assert_eq!(info.enemy_hp, 0);
    }
}
