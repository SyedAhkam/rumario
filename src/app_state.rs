#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    MainMenu,
    Paused,
    InGame
}