mod score;
mod bats;

pub use score::*;
pub use bats::*;

#[derive(Debug)]
pub struct Player;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlayerIndex {
    Player1,
    Player2
}

impl PlayerIndex {
    pub fn opposing(&self) -> Self {
        match self {
            Self::Player1 => Self::Player2,
            Self::Player2 => Self::Player1,
        }
    }
}

impl From<PlayerIndex> for u8 {
    fn from(from: PlayerIndex) -> Self {
        match from {
            PlayerIndex::Player1 => 0,
            PlayerIndex::Player2 => 1
        }
    }
}

impl From<PlayerIndex> for usize {
    fn from(from: PlayerIndex) -> Self {
        u8::from(from) as usize
    }
}


