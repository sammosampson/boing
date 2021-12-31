pub fn create_game_style() -> GameStyle {
    GameStyle::OnePlayer
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameStyle {
    OnePlayer,
    TwoPlayer
}
