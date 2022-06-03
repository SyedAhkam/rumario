#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    MainMenu,
    Paused,
    InGame
}

// This is a hack to get enum fields as `String`
impl std::fmt::Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}