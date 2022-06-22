use crate::components::Position;
use crate::entities::{create_box, create_box_spot, create_floor, create_player, create_wall};
use specs::World;

pub const MAP_HEIGHT: u8 = 9;
pub const MAP_WIDTH: u8 = 8;

pub fn load_map_func(world: &mut World, map_string: String) {
    // read all lines
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate() {
            // Create the position at which to create something on the map
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0, // we will get the z from the factory functions
            };

            // Figure out what object we should create
            match *column {
                "." => create_floor(world, position),
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position);
                }
                "P" => {
                    create_floor(world, position);
                    create_player(world, position);
                }
                "B" => {
                    create_floor(world, position);
                    create_box(world, position);
                }
                "S" => {
                    create_floor(world, position);
                    create_box_spot(world, position);
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}

#[macro_export]
macro_rules! load_map {
    ($w: expr, $ws: expr) => {
        load_map_func($w, $ws)
    };
    ($w: expr) => {
        const MAP: &str = "
            N N W W W W W W
            W W W . . . . W
            W . . . B . . W
            W . . . . . . W
            W . P . . . . W
            W . . . . . . W
            W . . S . . . W
            W . . . . . . W
            W W W W W W W W
            ";

        load_map_func($w, MAP.to_string())
    };
}
