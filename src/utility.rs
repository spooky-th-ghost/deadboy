#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    StartMenu,
    PauseMenu,
    LevelUpMenu,
    Gameplay,
}
