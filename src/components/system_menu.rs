use color_eyre::owo_colors::OwoColorize;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Borders, BorderType, List, ListDirection, ListState, Paragraph, Row, Table};
use ratatui::widgets::canvas::Canvas;
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use crate::action::Action;
use crate::components::Component;
use crate::components::utils::widget_utils::list_item_at_position;
use crate::game::celestial_bodies::{Displayable, Orbitable};
use crate::game::celestial_bodies::solar_system::SolarSystem;
use crate::game::game_state::FleetStatus;
use crate::tabs::Tabs;
use crate::tui::Frame;

pub struct SystemMenu {
    state: ListState,
    system: Option<SolarSystem>,
    is_focused: bool,
    map_focused: bool,
    list_length: usize,
    properties: Vec<Vec<String>>,
    map_shift_x: f64,
    map_shift_y: f64,
    map_zoom: f64,
    fleet: FleetStatus,
    /// Rect from the last draw(), used to hit-test mouse clicks.
    list_area: Rect,
}

impl Default for SystemMenu {
    fn default() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Self {
            list_length: 0,
            state,
            system: None,
            is_focused: false,
            map_focused: false,
            properties: vec![],
            map_shift_x: 0.0,
            map_shift_y: 0.0,
            map_zoom: 1.0,
            fleet: FleetStatus::default(),
            list_area: Rect::default(),
        }
    }
}

impl SystemMenu {
    pub fn set_system(&mut self, system: SolarSystem) {
        self.list_length = 1 + system.get_n_planets();
        self.system = Some(system);
    }
}

impl Component for SystemMenu {
    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::IngameTick => {
                return Ok(Some(Action::ScheduleLoadSystemView))
            }
            Action::StartSelecting => {
                self.is_focused = true;
            }
            Action::SecondaryAction => {
                self.map_focused = true;
                return Ok(Some(Action::EnterSystemMapNavigation))
            }
            Action::LoadSystemView(system) => {
                self.set_system(system);

            }
            Action::LoadFleetStatus(status) => {
                self.fleet = status;
            }
            Action::MainAction => {
                let selected = self.state.selected().unwrap_or(0);
                return Ok(Some(Action::MoveFleet(selected)))
            }
            Action::EngageEnemy => {
                return Ok(Some(Action::MoveFleet(self.fleet.enemy_body_index)))
            }
            Action::ReturnToCapital => {
                return Ok(Some(Action::MoveFleet(self.fleet.capital_body_index)))
            }
            Action::SelectNext => {
                let selected = self.state.selected().unwrap();
                if selected == (self.list_length - 1) {
                    self.state.select(Some(0))
                } else {
                    self.state.select(Some(selected + 1))
                }
            }
            Action::SelectPrevious => {
                let selected = self.state.selected().unwrap();
                if selected == 0 {
                    self.state.select(Some(self.list_length - 1))
                } else {
                    self.state.select(Some(selected - 1))
                }
            }
            Action::Select => {
                if self.is_focused {
                    self.is_focused = false;

                    let selected = self.state.selected().unwrap();
                    if selected == 0 {
                        let star = self.system.clone().unwrap().get_star();
                        self.properties = star.get_properties();
                    } else {
                        let planets = self.system.clone().unwrap().get_satellites();
                        self.properties = planets[selected - 1].get_properties();
                    }

                    return Ok(Some(
                        Action::UpdateObjectView
                    ))
                } else if self.map_focused {
                    self.map_focused = false;
                }
            }
            Action::Up => {
                self.map_shift_y += 2.0 * self.map_zoom
            }
            Action::Down => {
                self.map_shift_y -= 2.0 * self.map_zoom
            }
            Action::Left => {
                self.map_shift_x -= 2.0 * self.map_zoom
            }
            Action::Right => {
                self.map_shift_x += 2.0 * self.map_zoom
            }
            Action::ZoomIn => {
                self.map_zoom *= 0.9
            }
            Action::ZoomOut => {
                self.map_zoom /= 0.9
            }
            _ => {}
        }

        Ok(None)
    }

    /// Clicking a body jumps straight to it and views its properties in one motion, the
    /// same as arrow-keying to it then pressing Enter — but only once the list is
    /// already focused (i.e. <Alt-s>/<s> was already pressed), so mouse and keyboard
    /// stay interchangeable at every step. Use <Alt-r>/<r> afterward to send the fleet
    /// there, same as with keyboard-only navigation.
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> color_eyre::Result<Option<Action>> {
        if mouse.kind != MouseEventKind::Down(MouseButton::Left) || !self.is_focused {
            return Ok(None)
        }

        if let Some(index) = list_item_at_position(self.list_area, mouse.column, mouse.row, self.list_length) {
            self.state.select(Some(index));
            // Return the action itself (rather than calling self.update() directly) so
            // it also flows through app.rs's central mode-transition match, the same as
            // a keyboard Enter press does.
            return Ok(Some(Action::Select))
        }

        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> color_eyre::Result<()> {
        if self.system.is_none() {
            return Ok(())
        }

        let v_chunks = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3)
            ],
        ).split(area);

        let chunks = Layout::new(
            Direction::Horizontal,
            vec![
                Constraint::Percentage(20),
                Constraint::Fill(1),
            ]
        ).split(v_chunks[1]);

        let s_chunks = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Fill(1),
                Constraint::Max(10),
            ],
        ).split(chunks[1]);

        let l_chunks = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Fill(1),
                Constraint::Length(7),
            ],
        ).split(chunks[0]);

        let mut items = Vec::<Text>::with_capacity(1 + self.system.clone().unwrap().get_n_planets());
        items.push(
            Text::styled(
                self.system.clone().unwrap().get_star().get_name(),
                Style::default().fg(self.system.clone().unwrap().get_star().get_menu_color())
            )
        );

        self.system.clone().unwrap().get_satellites().iter().for_each(|p| {
           items.push(
               Text::styled(
                   p.get_name(),
                   Style::default().fg(p.get_menu_color())
               )
           )
        });

        let list = List::new(items)
            .block(
                Block::default()
                    .title(self.system.clone().unwrap().get_name())
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(
                        if self.is_focused {
                            Style::default().fg(Color::LightBlue)
                        } else { 
                            Style::default()
                        }
                    )
            )
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(false)
            .direction(ListDirection::TopToBottom);

        let mut rows: Vec<Row> = Vec::with_capacity(self.properties.len());

        for property in self.properties.iter() {
            rows.push(Row::new(property.clone()));
        }
        
        let widths = vec![
            Constraint::Fill(2),
            Constraint::Fill(3),
            Constraint::Fill(3),
        ];
        
        let object_view = Table::new(rows, widths)
            .header(Row::new(vec!["Property", "Value", "Value in relative units"])
                .style(Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD)))
            .block(
                Block::default()
                    .title("Selected object")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
            );

        let height = [
            (-15f64 + self.map_shift_x) * self.map_zoom,
            (15f64 + self.map_shift_x) * self.map_zoom
        ];
        let aspect_ratio = (s_chunks[0].width as f64) / (s_chunks[0].height as f64) / 2.0;
        let width = [
            ((-15f64 + self.map_shift_y) / aspect_ratio) * self.map_zoom ,
            ((15f64 + self.map_shift_y) / aspect_ratio) * self.map_zoom,
        ];


        let system_image = Canvas::default()
            .block(
                Block::default()
                    .title("System")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(
                        if self.map_focused {
                            Style::default().fg(Color::LightBlue)
                        } else {
                            Style::default()
                        }
                    )
            )
            .x_bounds(height)
            .y_bounds(width)
            .paint(|ctx| {
                if let Some(system) = self.system.clone() {
                    system.draw_image(
                        ctx,
                        self.map_zoom,
                        self.fleet.enemy_body_index,
                        self.fleet.fleet_body_index,
                    )
                }
            });

        let help = Paragraph::new(
            match (self.is_focused, self.map_focused) {
                (false, false) => "Press <Alt+S> to select body, <Alt+F> to enter map navigation",
                (true, false) => "Use arrows to highlight a body, then press <Enter> to select it",
                (false, true) => "Use arrows to move the view and <[> and <]> to control zoom. \
                Press <Enter> to exit map navigation",
                (true, true) => "This is a bug! Thanks for catching it!",
            }
        ).block(
            Block::default()
                .title("Controls help")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        );

        let mut fleet_lines = vec![
            Line::from(format!("Fleet at: {}", self.fleet.fleet_location_name)),
            Line::styled(
                format!("Enemy at: {}", self.fleet.enemy_location_name),
                Style::default().fg(if self.fleet.at_enemy_location { Color::LightRed } else { Color::Gray }),
            ),
            Line::styled(
                format!("Colony: {}/{} HP", self.fleet.colony_hp, self.fleet.colony_max_hp),
                Style::default().fg(if self.fleet.colony_under_siege { Color::LightRed } else { Color::LightGreen }),
            ),
        ];
        fleet_lines.push(match (&self.fleet.destination_name, self.fleet.travel_ticks_remaining) {
            (Some(dest), Some(ticks)) => Line::from(format!("En route to {dest}: {ticks} ticks left")),
            _ if !self.fleet.has_ship => Line::styled(
                "Build a ship to deploy a fleet",
                Style::default().fg(Color::DarkGray),
            ),
            _ if self.fleet.at_enemy_location => Line::styled(
                "Engaging the enemy!",
                Style::default().fg(Color::LightRed),
            ),
            _ => Line::from("Highlight a body and press <Alt-r> to move the fleet there"),
        });
        if self.fleet.colony_under_siege {
            fleet_lines.push(Line::styled(
                "\u{26A0} The colony is under attack!",
                Style::default().fg(Color::LightRed),
            ));
        } else if let Some(ticks) = self.fleet.enemy_ticks_until_advance {
            fleet_lines.push(Line::styled(
                format!("Enemy advances in {ticks} ticks if unopposed"),
                Style::default().fg(Color::Gray),
            ));
        }

        let fleet_panel = Paragraph::new(fleet_lines)
            .block(
                Block::default()
                    .title("Fleet")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
            );

        self.list_area = l_chunks[0];
        f.render_stateful_widget(list, l_chunks[0], &mut self.state);
        f.render_widget(fleet_panel, l_chunks[1]);
        f.render_widget(object_view, s_chunks[1]);
        f.render_widget(system_image, s_chunks[0]);
        f.render_widget(help, v_chunks[2]);

        Ok(())
    }

    fn is_drawn_in_tab(&self, tab: &Tabs) -> bool {
        *tab == Tabs::SystemView
    }
}