mod components;
mod entities;
mod game;
mod map;
mod resources;
mod systems;

use components::register_components;
use game::Game;
use ggez::{conf, event, ContextBuilder, GameResult};
use map::load_map_func;
use resources::register_resources;
use specs::{World, WorldExt};
use std::path;

pub fn initialize_level(world: &mut World) {
    load_map!(world);
}

pub fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    // Create a game context and event loop
    let context_builder = ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = context_builder.build()?;

    // Create the game state
    let game = Game { world };
    // Run the main event loop
    event::run(context, event_loop, game)
}
