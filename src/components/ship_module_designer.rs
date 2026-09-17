use color_eyre::owo_colors::OwoColorize;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, BorderType, List, ListState, Paragraph};

use crate::action::Action;
use crate::components::Component;
use crate::components::utils::widget_utils::{select_next_in_list, select_prev_in_list};
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
    current_design: Option<String>,
    ships_built: u32,
    build_progress: Option<u32>,
}

impl Component for ShipModuleDesigner {
    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        if !self.is_initialised {
            self.is_initialised = true;
            return Ok(Some(Action::ScheduleLoadShipModuleTypes));
        }

        match action {
            Action::StartSelecting => {
                if self.state == WidgetState::Normal {
                    self.state = WidgetState::SelectingType;
                    if self.types_list_state.selected().is_none() {
                        self.types_list_state.select(Some(0))
                    }
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
                    let name = self.modules[
                        self.modules_list_state.selected().unwrap_or(0)
                        ].0.clone();
                    self.state = WidgetState::Normal;
                    self.current_design = Some(name.clone());
                    return Ok(Some(Action::DesignShipModule(name)))
                }
            }
            Action::MainAction => {
                if self.state == WidgetState::Normal
                    && self.current_design.is_some()
                    && self.build_progress.is_none() {
                    return Ok(Some(Action::BuildShip))
                }
            }
            Action::IngameTick => {
                return Ok(Some(Action::ScheduleLoadShipyardInfo))
            }
            Action::LoadShipyardInfo(design, ships_built, progress) => {
                self.current_design = design;
                self.ships_built = ships_built;
                self.build_progress = progress;
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

        let mut info_lines = vec![
            Line::from(format!(
                "Current design: {}",
                self.current_design.clone().unwrap_or_else(|| "None".to_string())
            )),
            Line::from(format!("Ships built: {}", self.ships_built)),
        ];
        info_lines.push(match self.build_progress {
            Some(p) => Line::from(format!("Building... {p}%")),
            None if self.current_design.is_some() => {
                Line::from("Press <Alt-r> to build a ship")
            }
            None => Line::from("Research and design a sublight engine to get started"),
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