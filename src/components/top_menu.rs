use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, BorderType, Tabs};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use crate::action::Action;
use crate::components::Component;
use crate::components::utils::widget_utils::tab_at_position;
use crate::tui::Frame;

/// Matches the padding/divider passed to `.padding(...)`/`.divider(...)` below — needed
/// again in `handle_mouse_events` to replicate the widget's layout for hit-testing.
const TAB_PADDING: &str = " == ";
const TAB_DIVIDER: &str = "|";

pub struct TopMenu {
    tabs: Vec<String>,
    selected: usize,
    victory: bool,
    defeat: bool,
    /// Rect from the last draw(), used to hit-test mouse clicks on the tab bar.
    tabs_area: Rect,
}

impl Default for TopMenu {
    fn default() -> Self {
        Self {
            tabs: vec![],
            selected: 1,
            victory: false,
            defeat: false,
            tabs_area: Rect::default(),
        }
    }
}

impl Component for TopMenu {
    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::LoadTabs(tabs) => {
                self.tabs = tabs.iter().map(|t| String::from(t.clone()))
                    .collect();
                self.tabs.insert(0, String::from("<Shift+Tab>"));
                self.tabs.push(String::from("<Tab>"));
            }
            Action::NavigateNextTab => {
                self.selected += 1;
                self.selected %= self.tabs.len() - 1;
                if self.selected == 0 { self.selected += 1 }
            }
            Action::NavigatePrevTab => {
                if self.selected != 1 {
                    self.selected -= 1;
                } else {
                    self.selected = self.tabs.len() - 2;
                }
            }
            Action::NavigateToTab(index) => {
                self.selected = index + 1;
            }
            Action::Victory => {
                self.victory = true;
            }
            Action::Defeat => {
                self.defeat = true;
            }
            _ => {}
        }

        Ok(None)
    }

    /// Clicking a tab's name switches to it directly, without needing `<Tab>`/`<Shift+Tab>`.
    /// Clicking the leading "<Shift+Tab>" or trailing "<Tab>" labels does what they say.
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> color_eyre::Result<Option<Action>> {
        if mouse.kind != MouseEventKind::Down(MouseButton::Left) || self.tabs.is_empty() {
            return Ok(None)
        }

        if let Some(index) = tab_at_position(
            self.tabs_area, mouse.column, mouse.row, &self.tabs,
            TAB_PADDING.chars().count() as u16, TAB_PADDING.chars().count() as u16,
            TAB_DIVIDER.chars().count() as u16,
        ) {
            return Ok(Some(if index == 0 {
                Action::NavigatePrevTab
            } else if index == self.tabs.len() - 1 {
                Action::NavigateNextTab
            } else {
                Action::NavigateToTab(index - 1)
            }))
        }

        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> color_eyre::Result<()> {
        let chunks = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Length(3),
                Constraint::Min(0),
            ],
        ).split(area);

        let mut banners = Vec::new();
        if self.victory {
            banners.push("\u{1F3C6} VICTORY \u{2014} you built a warship and destroyed the enemy!".to_string());
        }
        if self.defeat {
            banners.push("\u{1F480} DEFEAT \u{2014} the enemy destroyed your colony!".to_string());
        }
        banners.push("Press <Alt-h> for help".to_string());
        let title = format!(" {} ", banners.join(" | "));

        let tabs = Tabs::new(self.tabs.clone())
            .block(
                Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .title(title)
            )
            .select(self.selected)
            .divider("|")
            .padding(" == ", " == ");

        self.tabs_area = chunks[0];
        f.render_widget(tabs, chunks[0]);

        Ok(())
    }

    fn is_drawn_in_tab(&self, tab: &crate::tabs::Tabs) -> bool {
        true
    }
}
