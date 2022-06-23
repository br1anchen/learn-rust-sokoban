use std::{
    fmt::{Display, Formatter, Result},
    time::Duration,
};

use ggez::event::KeyCode;
use specs::World;

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

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

#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}

// Registering resources
pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
    world.insert(Time::default());
}
