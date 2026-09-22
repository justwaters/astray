use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, BorderType, Clear, Paragraph, Wrap};

use crate::action::Action;
use crate::components::Component;
use crate::tabs::Tabs;
use crate::tui::Frame;

pub struct HelpMenu {
    visible: bool,
}

impl Default for HelpMenu {
    fn default() -> Self {
        // Shown automatically as the startup splash screen; App::new() also starts in
        // Mode::Help so <Enter>/<Esc> can dismiss it immediately.
        Self { visible: true }
    }
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

        let popup = centered_rect(80, 90, area);

        let lines = vec![
            Line::styled("Welcome to Astray", Style::default().add_modifier(Modifier::BOLD)),
            Line::from(""),
            Line::styled("Goal", Style::default().add_modifier(Modifier::BOLD)),
            Line::from("Your colony's system holds an enemy fleet guarding the outer planets. Build a"),
            Line::from("warship, destroy every enemy ship, and don't let the enemy reach your colony first."),
            Line::from(""),
            Line::styled("1. Research", Style::default().add_modifier(Modifier::BOLD)),
            Line::from("Research tab: pick a field, then a technology, <Alt-r>/<r> to start it. You need an"),
            Line::from("engine (e.g. Ion Drive) and a weapon (e.g. Ion Cannon)."),
            Line::from(""),
            Line::styled("2. Economy", Style::default().add_modifier(Modifier::BOLD)),
            Line::from("Colonies tab: <Alt-s>/<s> to select your colony, <Alt-r>/<r> to queue mines and"),
            Line::from("factories. A ship costs Engine Nozzles (Heat Resistant Alloy chain) and"),
            Line::from("Microprocessors (Superconductors + Electronics chain)."),
            Line::from(""),
            Line::styled("3. Design & build", Style::default().add_modifier(Modifier::BOLD)),
            Line::from("Ship modules tab: <Alt-s>/<s> to pick a type then a module, Enter to confirm a"),
            Line::from("design. You need both an engine and a weapon design, then <Alt-r>/<r> to build a"),
            Line::from("ship. Build more ships any time to grow your fleet — more ships means more"),
            Line::from("combined firepower and more HP to absorb the enemy's counter-fire."),
            Line::from(""),
            Line::styled("4. Move & fight", Style::default().add_modifier(Modifier::BOLD)),
            Line::from("System View tab: <Alt-e>/<e> sends your fleet straight to the enemy, <Alt-c>/<c>"),
            Line::from("sends it straight home — no need to manually page through the body list. (You can"),
            Line::from("still do that manually with <Alt-s>/<s> plus arrow keys, then <Alt-r>/<r>, to visit"),
            Line::from("anywhere else.) Travel takes time. Reaching the enemy's location starts combat"),
            Line::from("automatically each tick. Watch for a rare, much tougher escort alongside the"),
            Line::from("standard enemy — it keeps firing even while you're still working through the other"),
            Line::from("one."),
            Line::from(""),
            Line::styled("5. Don't dawdle", Style::default().add_modifier(Modifier::BOLD)),
            Line::from("If you never engage it, the enemy periodically advances toward your colony (watch"),
            Line::from("the Fleet panel for a countdown) and sieges it on arrival. Losing the colony loses"),
            Line::from("the game; destroying every enemy ship wins it."),
            Line::from(""),
            Line::styled("Symbols & colors", Style::default().add_modifier(Modifier::BOLD)),
            Line::from(">> marks the highlighted item in a list."),
            Line::styled("\u{2620} marks the enemy and \u{25C6} marks your fleet on the System View map.", Style::default().fg(Color::LightRed)),
            Line::from("Planet colors: green = habitable, yellow = too close, red = too far."),
            Line::from("Research colors: gray = locked, red/yellow/green/cyan = rising completion."),
            Line::from("Resource colors: gray = primary, yellow = secondary, cyan = component."),
            Line::from("A red \u{26A0} warning and a red colony HP reading mean you're under siege right now."),
            Line::from(""),
            Line::styled("Keys", Style::default().add_modifier(Modifier::BOLD)),
            Line::from("<Tab>/<Shift+Tab> switch tabs   <Alt-s>/<s> select/start selecting"),
            Line::from("<Alt-r>/<r> main action   <Alt-f>/<f> secondary action"),
            Line::from("<Alt-e>/<e> engage the enemy   <Alt-c>/<c> return fleet home"),
            Line::from("<Esc> cancel a selection   <q>/<Ctrl-c> quit   <Alt-h>/<h> reopen this screen"),
            Line::from(""),
            Line::from("Click a tab name to switch to it. Clicking a list pane focuses it and highlights"),
            Line::from("the item, then clicking it again confirms it; on System View, clicking the map"),
            Line::from("enters map navigation. Mouse and keyboard can be freely mixed at every step."),
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
