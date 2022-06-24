use std::fmt::{Display, Formatter, Result};

pub enum GameplayState {
    Playing,
    Won,
}
impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}
impl Display for GameplayState {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
        })?;
        Ok(())
    }
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32,
}
