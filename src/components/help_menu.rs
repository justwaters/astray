use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, BorderType, Clear, Paragraph, Wrap};

use crate::action::Action;
use crate::components::Component;
use crate::tabs::Tabs;
use crate::tui::Frame;

#[derive(Default)]
pub struct HelpMenu {
    visible: bool,
}

/// Carves a centered rectangle of the given percentage size out of `area`.
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::new(
        Direction::Vertical,
        vec![
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ],
    ).split(area);

    Layout::new(
        Direction::Horizontal,
        vec![
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ],
    ).split(vertical[1])[1]
}

impl Component for HelpMenu {
    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::Help => {
                self.visible = true;
            }
            Action::Select => {
                self.visible = false;
            }
            _ => {}
        }

        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> color_eyre::Result<()> {
        if !self.visible {
            return Ok(())
        }

        let popup = centered_rect(70, 80, area);

        let lines = vec![
            Line::styled("Goal", Style::default().add_modifier(Modifier::BOLD)),
            Line::from("Your colony's system holds an enemy fleet guarding the outer planets. Build a"),
            Line::from("warship, destroy every enemy ship, and don't let the enemy reach your colony first."),
            Line::from(""),
            Line::styled("1. Research", Style::default().add_modifier(Modifier::BOLD)),
            Line::from("Research tab: pick a field, then a technology, <Alt-r> to start it. You need an"),
            Line::from("engine (e.g. Ion Drive) and a weapon (e.g. Ion Cannon)."),
            Line::from(""),
            Line::styled("2. Economy", Style::default().add_modifier(Modifier::BOLD)),
            Line::from("Colonies tab: <Alt-s> to select your colony, <Alt-r> to queue mines and factories."),
            Line::from("A ship costs Engine Nozzles (Heat Resistant Alloy chain) and Microprocessors"),
            Line::from("(Superconductors + Electronics chain)."),
            Line::from(""),
            Line::styled("3. Design & build", Style::default().add_modifier(Modifier::BOLD)),
            Line::from("Ship modules tab: <Alt-s> to pick a type then a module, Enter to confirm a design."),
            Line::from("You need both an engine and a weapon design, then <Alt-r> to build a ship. Build"),
            Line::from("more ships any time to grow your fleet — more ships means more combined firepower"),
            Line::from("and more HP to absorb the enemy's counter-fire."),
            Line::from(""),
            Line::styled("4. Move & fight", Style::default().add_modifier(Modifier::BOLD)),
            Line::from("System View tab: <Alt-s> plus arrow keys to highlight a body, <Alt-r> to send your"),
            Line::from("fleet there. Travel takes time. Reaching the enemy's location starts combat"),
            Line::from("automatically each tick. Watch for a rare, much tougher escort alongside the"),
            Line::from("standard enemy — it keeps firing even while you're still working through the other one."),
            Line::from(""),
            Line::styled("5. Don't dawdle", Style::default().add_modifier(Modifier::BOLD)),
            Line::from("If you never engage it, the enemy periodically advances toward your colony (watch"),
            Line::from("the Fleet panel for a countdown) and sieges it on arrival. Losing the colony loses"),
            Line::from("the game; destroying every enemy ship wins it."),
            Line::from(""),
            Line::styled("Keys", Style::default().add_modifier(Modifier::BOLD)),
            Line::from("<Tab>/<Shift+Tab> switch tabs   <Alt-s> select/start selecting   <Alt-r> main action"),
            Line::from("<Alt-f> secondary action   <Esc> cancel a selection   <q>/<Ctrl-c> quit"),
            Line::from(""),
            Line::styled(
                "Press <Enter> or <Esc> to close this screen",
                Style::default().fg(Color::DarkGray),
            ),
        ];

        let help = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .title(" How to Play ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::LightBlue))
            );

        f.render_widget(Clear, popup);
        f.render_widget(help, popup);

        Ok(())
    }

    fn is_drawn_in_tab(&self, _tab: &Tabs) -> bool {
        true
    }
}
