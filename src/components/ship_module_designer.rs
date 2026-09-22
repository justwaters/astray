use color_eyre::owo_colors::OwoColorize;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, BorderType, List, ListState, Paragraph};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use crate::action::Action;
use crate::components::Component;
use crate::components::utils::widget_utils::{list_item_at_position, select_next_in_list, select_prev_in_list};
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
    /// Rects from the last draw(), used to hit-test mouse clicks.
    types_list_area: Rect,
    modules_list_area: Rect,
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

            // Mouse-only: a click landed directly on the Modules pane while it wasn't
            // already focused (e.g. it still holds modules from a type visited earlier
            // in this session) — just (re)focus it, keeping the click's already-selected
            // index intact rather than reloading via ContinueSelecting.
            Action::FocusSecondaryList => {
                self.state = WidgetState::SelectingModule;
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

    /// Clicking a type or module jumps straight to it and confirms in one motion, the
    /// same as arrow-keying to it then pressing Enter. A pane doesn't need to already be
    /// active for this to work: clicking the types pane focuses it (the mouse equivalent
    /// of pressing <Alt-s>/<s>) regardless of the current state, and clicking the modules
    /// pane focuses it too as long as it already holds modules from a type picked earlier.
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> color_eyre::Result<Option<Action>> {
        if mouse.kind != MouseEventKind::Down(MouseButton::Left) {
            return Ok(None)
        }

        if let Some(index) = list_item_at_position(
            self.types_list_area, mouse.column, mouse.row, self.module_types.len()
        ) {
            self.types_list_state.select(Some(index));
            return Ok(Some(
                if self.state == WidgetState::SelectingType {
                    Action::ContinueSelecting
                } else {
                    Action::StartSelecting
                }
            ))
        }

        if !self.modules.is_empty() {
            if let Some(index) = list_item_at_position(
                self.modules_list_area, mouse.column, mouse.row, self.modules.len()
            ) {
                self.modules_list_state.select(Some(index));
                return Ok(Some(
                    if self.state == WidgetState::SelectingModule {
                        Action::Select
                    } else {
                        Action::FocusSecondaryList
                    }
                ))
            }
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


        self.types_list_area = a_chunks[0];
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

        self.modules_list_area = a_chunks[1];
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
            if shipyard.living_enemies > 0 {
                Line::styled(
                    format!(
                        "Enemy fleet: {}/{} HP ({} ship{})",
                        shipyard.enemy_hp, shipyard.enemy_max_hp, shipyard.living_enemies,
                        if shipyard.living_enemies == 1 { "" } else { "s" },
                    ),
                    Style::default().fg(Color::LightRed),
                )
            } else {
                Line::styled("Enemy fleet: destroyed", Style::default().fg(Color::LightGreen))
            },
            if shipyard.living_ships > 0 {
                Line::styled(
                    format!(
                        "Fleet: {}/{} HP ({} ship{})",
                        shipyard.fleet_hp, shipyard.fleet_max_hp, shipyard.living_ships,
                        if shipyard.living_ships == 1 { "" } else { "s" },
                    ),
                    Style::default().fg(Color::LightCyan),
                )
            } else {
                Line::styled("No ships currently deployed", Style::default().fg(Color::DarkGray))
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
