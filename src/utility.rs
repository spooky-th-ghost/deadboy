#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Setup,
    StartMenu,
    PauseMenu,
    DeathMenu,
    LevelUpMenu,
    Gameplay,
    LoadScreen,
}
