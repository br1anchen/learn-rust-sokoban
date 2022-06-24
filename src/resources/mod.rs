pub mod audio;
pub mod gameplay;

pub use self::audio::*;
pub use self::gameplay::*;

use std::time::Duration;

use ggez::event::KeyCode;
use specs::World;

use crate::events::Event;

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}

// Registering resources
pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
    world.insert(Time::default());
    world.insert(EventQueue::default());
    world.insert(AudioStore::default());
}
