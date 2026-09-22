use ratatui::style::Color;
use serde::{
  de::{Deserializer, Visitor},
  Deserialize, Serialize,
};
use strum::Display;

use crate::game::celestial_bodies::solar_system::SolarSystem;
use crate::game::game_state::{FleetStatus, ShipyardInfo};
use crate::tabs::Tabs;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum Action {
  Tick,
  IngameTick,
  Render,
  Resize(u16, u16),
  Suspend,
  Resume,
  Quit,
  Refresh,
  Error(String),
  Help,
  UpdateObjectView,

  // Initialising
  InitResearch,
  InitColonies,

  // Loading data
  LoadTabs(Vec<Tabs>),
  LoadResearchFields(Vec<(String, String, Color)>),
  LoadResearches(Vec<String>),
  LoadResearchesForField(Vec<(String, String, Color)>),
  LoadResearchInfo(Vec<Vec<String>>),
  LoadDependencyInfo(Vec<Vec<(String, bool)>>),
  LoadResearchProgressText(String),
  LoadResearchProgress(u32),
  LoadSystemView(SolarSystem),
  LoadColonies(Vec<String>),
  LoadColonyInfo(Vec<(String, Color)>),
  LoadColonyBuildings(Vec<(String, u32, Color)>),
  LoadConstructionInfo(Vec<(String, u32)>),
  LoadShipModuleTypes(Vec<(String, Color)>),
  LoadShipModulesForType(Vec<(String, Color)>),
  LoadShipyardInfo(ShipyardInfo),
  LoadFleetStatus(FleetStatus),

  // Scheduling
  ScheduleLoadSystemView,
  ScheduleLoadResearchesForField(String),
  ScheduleLoadResearchInfo(String),
  ScheduleLoadColonyInfo(String),
  ScheduleLoadConstructionInfo(String),
  ScheduleLoadShipModuleTypes,
  ScheduleLoadShipModulesForType(String),
  ScheduleLoadShipyardInfo,

  // Navigation
  NavigateNextTab,
  NavigatePrevTab,
  NavigateToTab(usize),

  // Form actions
  StartSelecting,
  ContinueSelecting,
  /// Mouse-only: jump focus straight to a tab's second-level list pane (e.g. Researches,
  /// Buildings, Modules) regardless of the app's current mode — unlike `ContinueSelecting`,
  /// which only advances mode when it's already in the matching first-level state, this
  /// always resolves the correct mode for the current tab so a cold click (skipping the
  /// first-level pane entirely) still works.
  FocusSecondaryList,
  SelectNext,
  SelectPrevious,
  Select,
  Up,
  Down,
  Left,
  Right,
  ZoomIn,
  ZoomOut,

  // Tab actions
  MainAction,
  SecondaryAction,
  EnterSystemMapNavigation,
  StartResearch(String),
  StartSelectingBuilding,
  StartConstruction((String /* Colony name */, String /* Building type name */)),
  DesignShipModule(String /* module type */, String /* module name */),
  BuildShip,
  MoveFleet(usize /* body index */),
  EngageEnemy,
  ReturnToCapital,
  Victory,
  Defeat,
}
