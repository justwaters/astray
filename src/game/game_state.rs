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
/// In-game ticks it takes a fleet to travel one hop (one body index) in the system.
const TRAVEL_TICKS_PER_HOP: u32 = 5;
/// In-game ticks between the enemy advancing one hop toward the capital, while
/// the player's fleet isn't there to hold it off.
const ENEMY_ADVANCE_INTERVAL: u32 = 40;
/// Hit points of the capital colony. Reaching 0 loses the game.
const COLONY_MAX_HP: i32 = 30;
/// Damage the enemy deals to the colony each tick it besieges it unopposed.
const COLONY_DAMAGE_PER_TICK: i32 = 2;

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
    pub living_ships: u32,
    pub fleet_hp: i32,
    pub fleet_max_hp: i32,
    pub ship_max_hp: i32,
}

/// A single surviving ship's combat stats. Distinct from the public `Ship` build-log
/// record, which just remembers what was built, not whether it's still alive.
#[derive(Debug, Clone, PartialEq, Eq)]
struct FleetShip {
    hp: i32,
    weapon_damage: u32,
}

/// A snapshot of fleet position and movement state for the UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FleetStatus {
    pub has_ship: bool,
    pub fleet_location_name: String,
    pub enemy_location_name: String,
    pub at_enemy_location: bool,
    pub destination_name: Option<String>,
    pub travel_ticks_remaining: Option<u32>,
    pub colony_hp: i32,
    pub colony_max_hp: i32,
    pub colony_under_siege: bool,
    pub enemy_ticks_until_advance: Option<u32>,
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
    /// Ships currently alive and deployed with the fleet, front-to-back. The enemy's
    /// counter-fire always hits index 0; dead ships are pruned during combat.
    active_ships: Vec<FleetShip>,
    enemy_hp: i32,
    /// Index into the system's body list (0 = star, 1..=n = planets by orbit order).
    fleet_location: usize,
    enemy_location: usize,
    /// Fixed location of the capital colony (unlike fleet_location, this never moves).
    capital_location: usize,
    fleet_destination: Option<usize>,
    fleet_travel_ticks_remaining: Option<u32>,
    enemy_advance_counter: u32,
    colony_hp: i32,
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

        // Body indices match the System View's own scheme: 0 = star, 1..=n = planets
        // in orbit order. The fleet starts home at the capital's planet; the enemy sits
        // at the outermost planet, so reaching it takes deliberate travel. On the rare
        // chance the capital *is* the outermost planet, fall back to the star (always a
        // different index) — otherwise the enemy would besiege the colony from tick 0,
        // with no ship yet built and no warning.
        let fleet_location = system.get_satellites().iter()
            .position(|p| p.get_name() == capital_planet.get_name())
            .map(|i| i + 1)
            .unwrap_or(0);
        let enemy_location = if system.get_n_planets() != fleet_location {
            system.get_n_planets()
        } else {
            0
        };

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
            active_ships: Vec::new(),
            enemy_hp: ENEMY_MAX_HP,
            fleet_location,
            enemy_location,
            capital_location: fleet_location,
            fleet_destination: None,
            fleet_travel_ticks_remaining: None,
            enemy_advance_counter: 0,
            colony_hp: COLONY_MAX_HP,
        }
    }
}

impl GameState {
    pub fn tick(&mut self) {
        self.update_research();
        self.update_colonies();
        self.update_orbits();
        self.update_shipyard();
        self.update_fleet_movement();
        self.update_combat();
        self.update_enemy_advance();
        self.update_colony_siege();
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
                self.active_ships.push(FleetShip { hp: SHIP_MAX_HP, weapon_damage: damage });
                self.ship_build_progress = None;
            } else {
                self.ship_build_progress = Some(progress);
            }
        }
    }

    fn update_fleet_movement(&mut self) {
        if let Some(remaining) = self.fleet_travel_ticks_remaining {
            if remaining <= 1 {
                self.fleet_location = self.fleet_destination.take().unwrap();
                self.fleet_travel_ticks_remaining = None;
            } else {
                self.fleet_travel_ticks_remaining = Some(remaining - 1);
            }
        }
    }

    /// Resolves one round of combat between the fleet and the scripted enemy, if both
    /// are alive and co-located. Every surviving ship fires at once, stacking damage;
    /// the enemy fires back at whichever ship is at the front of the fleet, and only if
    /// it survives the player's combined attack. Ships destroyed this round are pruned.
    fn update_combat(&mut self) {
        if self.enemy_hp <= 0 || self.fleet_location != self.enemy_location {
            return
        }

        self.active_ships.retain(|s| s.hp > 0);
        if self.active_ships.is_empty() {
            return
        }

        let total_damage: u32 = self.active_ships.iter().map(|s| s.weapon_damage).sum();
        self.enemy_hp = (self.enemy_hp - total_damage as i32).max(0);

        if self.enemy_hp > 0 {
            self.active_ships[0].hp = (self.active_ships[0].hp - ENEMY_DAMAGE_PER_TICK).max(0);
        }
    }

    /// Whether at least one surviving ship is at the enemy's location, holding it off.
    /// A fleet of wrecks sitting on the same tile doesn't count — it can't fight back.
    fn is_fleet_defending(&self) -> bool {
        self.fleet_location == self.enemy_location
            && self.active_ships.iter().any(|s| s.hp > 0)
    }

    /// While the enemy isn't being held off by the player's fleet, it slowly advances
    /// one hop closer to the capital every `ENEMY_ADVANCE_INTERVAL` ticks.
    fn update_enemy_advance(&mut self) {
        if self.enemy_hp <= 0 || self.is_fleet_defending() {
            return
        }

        self.enemy_advance_counter += 1;
        if self.enemy_advance_counter >= ENEMY_ADVANCE_INTERVAL {
            self.enemy_advance_counter = 0;
            match self.enemy_location.cmp(&self.capital_location) {
                std::cmp::Ordering::Greater => self.enemy_location -= 1,
                std::cmp::Ordering::Less => self.enemy_location += 1,
                std::cmp::Ordering::Equal => {}
            }
        }
    }

    /// Once the enemy reaches the capital unopposed, it besieges the colony directly.
    /// If the player's fleet is there instead, combat handles it and the colony is safe.
    fn update_colony_siege(&mut self) {
        if self.enemy_hp <= 0
            || self.enemy_location != self.capital_location
            || self.is_fleet_defending() {
            return
        }

        self.colony_hp = (self.colony_hp - COLONY_DAMAGE_PER_TICK).max(0);
    }

    pub fn has_lost(&self) -> bool {
        self.colony_hp <= 0
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

        let living_ships = self.active_ships.iter().filter(|s| s.hp > 0).count() as u32;
        let fleet_hp: i32 = self.active_ships.iter().filter(|s| s.hp > 0).map(|s| s.hp).sum();

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
            living_ships,
            fleet_hp,
            fleet_max_hp: living_ships as i32 * SHIP_MAX_HP,
            ship_max_hp: SHIP_MAX_HP,
        }
    }

    pub fn has_won(&self) -> bool {
        self.enemy_hp <= 0
    }

    fn get_body_name(&self, index: usize) -> String {
        if index == 0 {
            self.capital_system.get_star().get_name()
        } else {
            self.capital_system.get_satellites()[index - 1].get_name()
        }
    }

    /// Commands the fleet to travel to the given body index. Requires a surviving ship,
    /// no travel already in progress, and an actual destination change. Travel time is
    /// proportional to how many bodies away the destination is. Returns whether the
    /// fleet started moving.
    pub fn move_fleet_to(&mut self, destination: usize) -> bool {
        if !self.active_ships.iter().any(|s| s.hp > 0) {
            return false
        }
        if self.fleet_travel_ticks_remaining.is_some() {
            return false
        }

        let hops = (destination as i64 - self.fleet_location as i64).unsigned_abs() as u32;
        if hops == 0 {
            return false
        }

        self.fleet_destination = Some(destination);
        self.fleet_travel_ticks_remaining = Some(hops * TRAVEL_TICKS_PER_HOP);
        true
    }

    pub fn get_fleet_status(&self) -> FleetStatus {
        let engaged = self.is_fleet_defending();
        let colony_under_siege = self.enemy_hp > 0
            && self.enemy_location == self.capital_location
            && !engaged;
        let enemy_ticks_until_advance = if self.enemy_hp > 0 && !engaged {
            Some(ENEMY_ADVANCE_INTERVAL - self.enemy_advance_counter)
        } else {
            None
        };

        FleetStatus {
            has_ship: self.active_ships.iter().any(|s| s.hp > 0),
            fleet_location_name: self.get_body_name(self.fleet_location),
            enemy_location_name: self.get_body_name(self.enemy_location),
            at_enemy_location: engaged,
            destination_name: self.fleet_destination.map(|d| self.get_body_name(d)),
            travel_ticks_remaining: self.fleet_travel_ticks_remaining,
            colony_hp: self.colony_hp,
            colony_max_hp: COLONY_MAX_HP,
            colony_under_siege,
            enemy_ticks_until_advance,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fresh_game_never_starts_with_the_enemy_already_at_the_capital() {
        // Regression test: if a randomly-generated system happens to place the capital
        // on the outermost planet, enemy_location must not default to the same index —
        // otherwise the colony would be under siege from tick 0, with no ship built yet
        // and no warning the player could have acted on.
        for _ in 0..200 {
            let state = GameState::new();
            assert_ne!(state.enemy_location, state.capital_location);
            assert!(!state.has_lost());
            assert!(!state.get_fleet_status().colony_under_siege);
        }
    }

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
        // enough components first, and place the fleet at the enemy's location so
        // combat isn't gated on travel.
        state.engine_design = Some("Ion drive".to_string());
        state.weapon_design = Some("Ion Cannon".to_string());
        state.ship_build_progress = Some(0);
        state.fleet_location = state.enemy_location;
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

    #[test]
    fn multiple_ships_stack_damage_and_the_front_ship_absorbs_return_fire() {
        let mut state = GameState::new();
        state.fleet_location = state.enemy_location;
        state.enemy_hp = 100;
        state.active_ships = vec![
            FleetShip { hp: 5, weapon_damage: 3 },
            FleetShip { hp: 20, weapon_damage: 3 },
            FleetShip { hp: 20, weapon_damage: 3 },
        ];

        state.tick();
        assert_eq!(state.enemy_hp, 91, "all three ships should have fired");
        assert_eq!(state.active_ships[0].hp, 2, "the front ship absorbs the counter-fire");
        assert_eq!(state.active_ships.len(), 3);

        state.tick();
        assert_eq!(state.enemy_hp, 82);
        assert_eq!(state.active_ships[0].hp, 0, "the front ship should be destroyed this round");

        // The next round prunes the wreck before resolving damage: only the two
        // survivors fire, and the ship that was second in line now takes the hit.
        state.tick();
        assert_eq!(state.enemy_hp, 76);
        assert_eq!(state.active_ships.len(), 2);
        assert_eq!(state.active_ships[0].hp, 17);
        assert_eq!(state.active_ships[1].hp, 20);

        let info = state.get_shipyard_info();
        assert_eq!(info.living_ships, 2);
        assert_eq!(info.fleet_hp, 37);
        assert_eq!(info.fleet_max_hp, 40);
    }

    #[test]
    fn combat_requires_reaching_the_enemys_location() {
        let mut state = GameState::new();
        // Force a location gap so the fleet always has to travel.
        state.fleet_location = 0;
        state.enemy_location = 3;

        state.active_ships = vec![FleetShip { hp: SHIP_MAX_HP, weapon_damage: 100 }];

        // Not co-located yet: no damage should be dealt despite lethal weapon damage.
        for _ in 0..5 {
            state.tick();
        }
        assert_eq!(state.enemy_hp, ENEMY_MAX_HP);

        state.fleet_location = state.enemy_location;
        state.tick();
        assert_eq!(state.enemy_hp, 0);
        assert!(state.has_won());
    }

    #[test]
    fn fleet_cannot_move_without_a_surviving_ship() {
        let mut state = GameState::new();
        assert!(!state.move_fleet_to(state.fleet_location + 1));

        state.active_ships = vec![FleetShip { hp: 0, weapon_damage: 0 }];
        assert!(!state.move_fleet_to(state.fleet_location + 1));
    }

    #[test]
    fn fleet_travels_over_time_and_arrives() {
        let mut state = GameState::new();
        state.active_ships = vec![FleetShip { hp: SHIP_MAX_HP, weapon_damage: 0 }];
        let origin = state.fleet_location;
        // enemy_location is always a valid body index (the last planet), unlike an
        // arbitrary offset which could overshoot a randomly-generated small system.
        // Fall back to the star (index 0, always valid and never the capital's own
        // planet) on the rare chance the capital already sits at enemy_location.
        let destination = if state.enemy_location != origin { state.enemy_location } else { 0 };
        let expected_hops = (destination as i64 - origin as i64).unsigned_abs() as u32;

        assert!(state.move_fleet_to(destination));
        // Can't redirect mid-flight.
        assert!(!state.move_fleet_to(origin));

        let status = state.get_fleet_status();
        assert_eq!(status.travel_ticks_remaining, Some(expected_hops * TRAVEL_TICKS_PER_HOP));
        assert_eq!(status.fleet_location_name, state.get_body_name(origin));

        for _ in 0..(expected_hops * TRAVEL_TICKS_PER_HOP) {
            state.tick();
        }

        assert_eq!(state.fleet_location, destination);
        let status = state.get_fleet_status();
        assert_eq!(status.destination_name, None);
        assert_eq!(status.travel_ticks_remaining, None);
    }

    #[test]
    fn unopposed_enemy_advances_toward_the_capital() {
        let mut state = GameState::new();
        state.capital_location = 0;
        state.enemy_location = 3;
        state.fleet_location = 10; // nowhere near the enemy, so it isn't held off

        for _ in 0..(ENEMY_ADVANCE_INTERVAL - 1) {
            state.tick();
        }
        assert_eq!(state.enemy_location, 3, "shouldn't advance before the interval elapses");

        state.tick();
        assert_eq!(state.enemy_location, 2, "should have advanced one hop toward the capital");
    }

    #[test]
    fn enemy_does_not_advance_while_engaged() {
        let mut state = GameState::new();
        state.capital_location = 0;
        state.enemy_location = 3;
        state.fleet_location = 3; // fleet is holding the line at the enemy's location
        // A wreck wouldn't hold anything off, and a ship that dies to the enemy's
        // counter-fire mid-test would stop defending — give it enough HP to outlast the
        // whole test so this isolates "does engaging block the advance" from "does the
        // defender survive combat" (see `defending_the_capital_prevents_the_siege`).
        state.active_ships = vec![FleetShip { hp: 1_000_000, weapon_damage: 0 }];

        for _ in 0..(ENEMY_ADVANCE_INTERVAL * 2) {
            state.tick();
        }
        assert_eq!(state.enemy_location, 3);
    }

    #[test]
    fn unopposed_siege_destroys_the_colony_and_loses_the_game() {
        let mut state = GameState::new();
        state.capital_location = 0;
        state.enemy_location = 0; // enemy has already reached the capital
        // Index 1 (the first planet) is always valid, unlike an arbitrary offset that
        // could exceed a randomly-generated small system's planet count.
        state.fleet_location = 1; // fleet is elsewhere, not defending

        assert!(!state.has_lost());
        let ticks_to_destroy = (COLONY_MAX_HP as f32 / COLONY_DAMAGE_PER_TICK as f32).ceil() as u32;
        for _ in 0..ticks_to_destroy {
            state.tick();
        }

        assert!(state.has_lost());
        assert_eq!(state.get_fleet_status().colony_hp, 0);
    }

    #[test]
    fn defending_the_capital_prevents_the_siege() {
        let mut state = GameState::new();
        state.capital_location = 0;
        state.enemy_location = 0;
        state.fleet_location = 0; // fleet is home defending
        // A ship that dies mid-fight stops defending (see `enemy_does_not_advance_while_engaged`'s
        // sibling concern) — give it enough HP to outlast the whole test regardless of the
        // enemy's per-tick counter-damage, so this test isolates "does defending block the siege"
        // from "does the defender survive combat".
        state.active_ships = vec![FleetShip { hp: 1_000_000, weapon_damage: 0 }]; // don't accidentally win mid-test

        for _ in 0..(COLONY_MAX_HP * 2) {
            state.tick();
        }

        assert_eq!(state.get_fleet_status().colony_hp, COLONY_MAX_HP);
        assert!(!state.has_lost());
    }
}
