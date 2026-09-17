use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::game::celestial_bodies::Displayable;
use crate::game::colony::building::BuildingType;
use crate::game::colony::building_manager::BuildingManager;
use crate::game::resource::resource::{ResourceDeposit, ResourceTransaction, ResourceType};
use crate::game::resource::resource_manager::ResourceManager;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Colony {
    planet_name: String,
    population: i32,
    building_manager: BuildingManager,
    resource_manager: ResourceManager,
    resource_deposit: ResourceDeposit,
}

impl Colony {
    pub fn new(
        planet_name: String,
        population: i32,
    ) -> Self {
        Self {
            planet_name,
            population,
            building_manager: BuildingManager::new(),
            resource_manager: ResourceManager::new(),
            resource_deposit: ResourceDeposit::generate_for_planet(),
        }
    }

    pub fn tick(&mut self) {
        self.building_manager.update_construction();
    }

    pub fn resource_tick(&mut self) {
        self.building_manager.update_mines(
            &mut self.resource_manager,
            &self.resource_deposit,
            10,
        );

        self.building_manager.update_production(
            &mut self.resource_manager,
        );
    }

    pub fn get_population(&self) -> i32 {
        self.population
    }

    pub fn get_buildings(&self) -> Vec<(String, u32, Color)> {
        self.building_manager.get_buildings()
    }

    pub fn get_resources(&self) -> Vec<(ResourceType, u32)> {
        self.resource_manager.get_resources()
    }

    pub fn get_resource_amount(&self, resource_type: &ResourceType) -> u32 {
        self.resource_manager.get_amount(resource_type)
    }

    /// Attempts to deduct `amount` of `resource_type`, returning whether there was enough.
    pub fn try_spend_resource(&mut self, resource_type: ResourceType, amount: u32) -> bool {
        let transaction = ResourceTransaction::new(resource_type, -(amount as i32));
        if self.resource_manager.is_applicable(&transaction) {
            self.resource_manager.apply(transaction);
            true
        } else {
            false
        }
    }

    pub fn get_construction(&self) -> Vec<(String, u32)> {
        self.building_manager.get_construction()
    }

    pub fn start_construction(&mut self, building_type: BuildingType) {
        self.building_manager.start_construction(
            building_type
        )
    }

    pub fn get_info(&self) -> Vec<(String, Color)> {
        let mut lines = Vec::<(String, Color)>::new();
        lines.push((format!("Name: {}", self.get_name()), Color::Cyan));
        lines.push((format!("Population: {}", self.get_population()), Color::Gray));

        self.get_resources().iter()
            .for_each(|(resource, amount)| {
                let r: String = resource.clone().into();
                lines.push((
                    format!("{}: {}", r, amount),
                    match amount {
                        0 => Color::DarkGray,
                        _ => resource.get_menu_color()
                    }
                ));
            });

        lines
    }
}

impl Displayable for Colony {
    fn get_name(&self) -> String {
        self.planet_name.clone()
    }
}