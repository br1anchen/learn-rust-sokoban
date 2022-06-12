use crate::components::{Box, BoxSpot, Player, Position, Renderable, Wall};
use specs::{Builder, World, WorldExt};

pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "/images/wall.png".to_string(),
        })
        .with(Wall {})
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable {
            path: "/images/floor.png".to_string(),
        })
        .build();
}

pub fn create_box(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "/images/box.png".to_string(),
        })
        .with(Box {})
        .build();
}

pub fn create_box_spot(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 9, ..position })
        .with(Renderable {
            path: "/images/box_spot.png".to_string(),
        })
        .with(BoxSpot {})
        .build();
}

pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "/images/player.png".to_string(),
        })
        .with(Player {})
        .build();
}

pub fn initialize_level(world: &mut World) {
    create_player(
        world,
        Position {
            x: 0,
            y: 0,
            z: 0, // we will get the z from the factory functions
        },
    );
    create_wall(
        world,
        Position {
            x: 1,
            y: 0,
            z: 0, // we will get the z from the factory functions
        },
    );
    create_box(
        world,
        Position {
            x: 2,
            y: 0,
            z: 0, // we will get the z from the factory functions
        },
    );
}
