use ratatui::style::Color;

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
    ship_design: Option<String>,
    ship_build_progress: Option<u32>,
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
            ship_design: None,
            ship_build_progress: None,
        }
    }
}

impl GameState {
    pub fn tick(&mut self) {
        self.update_research();
        self.update_colonies();
        self.update_orbits();
        self.update_shipyard();
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
                let design = self.ship_design.clone().unwrap();
                self.ships.push(Ship::new(design));
                self.ship_build_progress = None;
            } else {
                self.ship_build_progress = Some(progress);
            }
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

    /// Returns the sublight engine designs available to the player for the given module
    /// type, i.e. those that are unlocked by default or whose required research is finished.
    pub fn get_ship_modules_for_type(&self, _type_name: String) -> Vec<(String, Color)> {
        self.ship_module_manager.get_sublight_engine_designs().into_iter()
            .filter(|(_, is_unlocked, required_research_id)| {
                *is_unlocked || required_research_id.as_ref()
                    .is_some_and(|id| self.research_manager.is_research_finished(id.clone()))
            })
            .map(|(name, _, _)| (name, Color::White))
            .collect()
    }

    pub fn set_ship_design(&mut self, name: String) {
        self.ship_design = Some(name);
    }

    /// Starts building a ship if a design is chosen, no build is already in progress, and the
    /// capital colony can afford the Engine Nozzles cost. Returns whether the build started.
    pub fn build_ship(&mut self) -> bool {
        if self.ship_design.is_none() || self.ship_build_progress.is_some() {
            return false
        }

        let Some(capital) = self.colonies.first_mut() else { return false };
        if capital.try_spend_resource(ResourceType::CEngineNozzles, SHIP_ENGINE_NOZZLE_COST) {
            self.ship_build_progress = Some(0);
            true
        } else {
            false
        }
    }

    /// Returns `(current design, ships built, build progress %, available/required Engine Nozzles)`.
    pub fn get_shipyard_info(&self) -> (Option<String>, u32, Option<u32>, u32, u32) {
        let progress_percent = self.ship_build_progress
            .map(|p| (p * 100 / SHIP_BUILD_TIME).min(100));
        let available_nozzles = self.colonies.first()
            .map(|c| c.get_resource_amount(&ResourceType::CEngineNozzles))
            .unwrap_or(0);
        (
            self.ship_design.clone(),
            self.ships.len() as u32,
            progress_percent,
            available_nozzles,
            SHIP_ENGINE_NOZZLE_COST,
        )
    }

    pub fn has_won(&self) -> bool {
        !self.ships.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn research_unlocks_ship_module() {
        let mut state = GameState::new();

        assert!(state.get_ship_modules_for_type("Sublight Thruster".to_string()).is_empty());

        state.start_research("ion-drive".to_string());
        for _ in 0..100 {
            state.tick();
        }

        let modules = state.get_ship_modules_for_type("Sublight Thruster".to_string());
        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].0, "Ion drive");
    }

    #[test]
    fn building_a_ship_requires_engine_nozzles() {
        let mut state = GameState::new();
        state.set_ship_design("Ion drive".to_string());

        // The capital colony starts with no stockpiled Engine Nozzles, so the build
        // shouldn't start even though a design has been chosen.
        assert!(!state.build_ship());
        assert!(state.ship_build_progress.is_none());
    }

    #[test]
    fn finishing_a_ship_build_wins_the_game() {
        let mut state = GameState::new();

        // Drive the tick-completion logic directly rather than through the full,
        // RNG-driven mining/production chain that would otherwise need to produce
        // enough Engine Nozzles first.
        state.ship_design = Some("Ion drive".to_string());
        state.ship_build_progress = Some(0);
        assert!(!state.has_won());

        for _ in 0..SHIP_BUILD_TIME {
            state.tick();
        }

        assert!(state.has_won());
        let (design, ships_built, progress, _, cost) = state.get_shipyard_info();
        assert_eq!(design, Some("Ion drive".to_string()));
        assert_eq!(ships_built, 1);
        assert_eq!(progress, None);
        assert_eq!(cost, SHIP_ENGINE_NOZZLE_COST);
    }
}
