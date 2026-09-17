use color_eyre::owo_colors::OwoColorize;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, BorderType, List, ListState, Paragraph};

use crate::action::Action;
use crate::components::Component;
use crate::components::utils::widget_utils::{select_next_in_list, select_prev_in_list};
use crate::game::game_state::ShipyardInfo;
use crate::tabs::Tabs;
use crate::tui::Frame;

#[derive(PartialEq)]
#[derive(Default)]
enum WidgetState {
    #[default]
    Normal,
    SelectingType,
    SelectingModule,
}


#[derive(Default)]
pub struct ShipModuleDesigner {
    module_types: Vec<(String, Color)>,
    modules: Vec<(String, Color)>,
    is_initialised: bool,
    types_list_state: ListState,
    modules_list_state: ListState,
    state: WidgetState,
    shipyard: ShipyardInfo,
}

impl Component for ShipModuleDesigner {
    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        if !self.is_initialised {
            self.is_initialised = true;
            return Ok(Some(Action::ScheduleLoadShipModuleTypes));
        }

        match action {
            Action::StartSelecting => {
                // Always (re)start type selection, even if a previous visit to this tab was
                // left mid-flow (e.g. no modules were unlocked yet) — otherwise the widget
                // gets stuck since there's no dedicated "cancel" key.
                self.state = WidgetState::SelectingType;
                if self.types_list_state.selected().is_none() {
                    self.types_list_state.select(Some(0))
                }
            }
            Action::ContinueSelecting => {
                if self.state == WidgetState::SelectingType && !self.module_types.is_empty() {
                    self.state = WidgetState::SelectingModule;
                    let type_name = self.module_types[
                        self.types_list_state.selected().unwrap_or(0)
                        ].0.clone();
                    return Ok(Some(Action::ScheduleLoadShipModulesForType(type_name)))
                }
            }
            Action::SelectNext => {
                match self.state {
                    WidgetState::Normal => {}
                    WidgetState::SelectingType => {
                        self.types_list_state.select(
                            Some(
                                select_next_in_list(
                                    self.types_list_state.selected().unwrap_or(0),
                                    self.module_types.len(),
                                )
                            )
                        )
                    }
                    WidgetState::SelectingModule => {
                        self.modules_list_state.select(
                            Some(
                                select_next_in_list(
                                    self.modules_list_state.selected().unwrap_or(0),
                                    self.modules.len(),
                                )
                            )
                        )
                    }
                }
            }
            Action::SelectPrevious => {
                match self.state {
                    WidgetState::Normal => {}
                    WidgetState::SelectingType => {
                        self.types_list_state.select(
                            Some(
                                select_prev_in_list(
                                    self.types_list_state.selected().unwrap_or(0),
                                    self.module_types.len(),
                                )
                            )
                        )
                    }
                    WidgetState::SelectingModule => {
                        self.modules_list_state.select(
                            Some(
                                select_prev_in_list(
                                    self.modules_list_state.selected().unwrap_or(0),
                                    self.modules.len(),
                                )
                            )
                        )
                    }
                }
            }
            Action::LoadShipModuleTypes(types) => { self.module_types = types }
            Action::LoadShipModulesForType(modules) => {
                self.modules = modules;
                self.modules_list_state.select(
                    if self.modules.is_empty() { None } else { Some(0) }
                );
            }
            Action::Select => {
                if self.state == WidgetState::SelectingModule && !self.modules.is_empty() {
                    let type_name = self.module_types[
                        self.types_list_state.selected().unwrap_or(0)
                        ].0.clone();
                    let module_name = self.modules[
                        self.modules_list_state.selected().unwrap_or(0)
                        ].0.clone();
                    self.state = WidgetState::Normal;
                    return Ok(Some(Action::DesignShipModule(type_name, module_name)))
                }
            }
            Action::MainAction => {
                if self.state == WidgetState::Normal
                    && self.shipyard.engine_design.is_some()
                    && self.shipyard.weapon_design.is_some()
                    && self.shipyard.build_progress_percent.is_none() {
                    return Ok(Some(Action::BuildShip))
                }
            }
            Action::IngameTick => {
                return Ok(Some(Action::ScheduleLoadShipyardInfo))
            }
            Action::LoadShipyardInfo(info) => {
                self.shipyard = info;
            }
            _ => {}
        }

        Ok(None)
    }
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> color_eyre::Result<()> {
        let v_chunks = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(3),
            ],
        ).split(area);

        let a_chunks = Layout::new(
            Direction::Horizontal,
            vec![
                Constraint::Percentage(15),
                Constraint::Percentage(15),
                Constraint::Percentage(55),
                Constraint::Percentage(15),
            ],
        ).split(v_chunks[1]);

        let types_list = List::new(
            self.module_types.iter().map(
                |(i, c)| {
                    Line::styled(
                        i,
                        Style::default().fg(*c),
                    )
                }
            )
        )
            .highlight_symbol(">>")
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default()
                        .fg(if self.state == WidgetState::SelectingType {
                            Color::LightBlue
                        } else {
                            Color::White
                        }))
            );


        f.render_stateful_widget(types_list, a_chunks[0], &mut self.types_list_state);

        let modules_list = List::new(
            self.modules.iter().map(
                |(i, c)| {
                    Line::styled(
                        i,
                        Style::default().fg(*c),
                    )
                }
            )
        )
            .highlight_symbol(">>")
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default()
                        .fg(if self.state == WidgetState::SelectingModule {
                            Color::LightBlue
                        } else {
                            Color::White
                        }))
            );

        f.render_stateful_widget(modules_list, a_chunks[1], &mut self.modules_list_state);

        let shipyard = &self.shipyard;
        let mut info_lines = vec![
            Line::from(format!(
                "Engine design: {}",
                shipyard.engine_design.clone().unwrap_or_else(|| "None".to_string())
            )),
            Line::from(format!(
                "Weapon design: {}",
                shipyard.weapon_design.clone().unwrap_or_else(|| "None".to_string())
            )),
            Line::from(format!("Ships built: {}", shipyard.ships_built)),
            Line::styled(
                format!("Engine Nozzles: {}/{}", shipyard.available_nozzles, shipyard.nozzle_cost),
                Style::default().fg(
                    if shipyard.available_nozzles >= shipyard.nozzle_cost { Color::LightGreen } else { Color::Gray }
                ),
            ),
            Line::styled(
                format!("Microprocessors: {}/{}", shipyard.available_microprocessors, shipyard.microprocessor_cost),
                Style::default().fg(
                    if shipyard.available_microprocessors >= shipyard.microprocessor_cost { Color::LightGreen } else { Color::Gray }
                ),
            ),
            Line::from(""),
            Line::styled(
                format!("Enemy ship: {}/{} HP", shipyard.enemy_hp.max(0), shipyard.enemy_max_hp),
                Style::default().fg(if shipyard.enemy_hp <= 0 { Color::LightGreen } else { Color::LightRed }),
            ),
            match shipyard.active_ship_hp {
                Some(hp) => Line::styled(
                    format!("Your ship: {}/{} HP", hp.max(0), shipyard.ship_max_hp),
                    Style::default().fg(if hp <= 0 { Color::DarkGray } else { Color::LightCyan }),
                ),
                None => Line::styled("No ship currently deployed", Style::default().fg(Color::DarkGray)),
            },
        ];
        info_lines.push(match shipyard.build_progress_percent {
            Some(p) => Line::from(format!("Building... {p}%")),
            None if shipyard.engine_design.is_none() || shipyard.weapon_design.is_none() => {
                Line::from("Research and design an engine and a weapon to get started")
            }
            None if shipyard.available_nozzles < shipyard.nozzle_cost
                || shipyard.available_microprocessors < shipyard.microprocessor_cost => {
                Line::from("Stockpile enough Engine Nozzles and Microprocessors to afford a ship")
            }
            None => Line::from("Press <Alt-r> to build a ship"),
        });

        let shipyard_info = Paragraph::new(info_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Shipyard")
            );

        f.render_widget(shipyard_info, a_chunks[2]);

        Ok(())
    }

    fn is_drawn_in_tab(&self, tab: &Tabs) -> bool {
        *tab == Tabs::ShipModules
    }
}
