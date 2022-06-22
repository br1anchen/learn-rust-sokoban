use std::collections::HashMap;

use ggez::event::KeyCode;
use specs::{join::Join, world::Index, Entities, ReadStorage, System, Write, WriteStorage};

use crate::{
    components::{Immovable, Movable, Player, Position},
    map::{MAP_HEIGHT, MAP_WIDTH},
    resources::InputQueue,
};

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    // Data
    type SystemData = (
        Write<'a, InputQueue>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, entities, mut positions, players, movables, immovables) = data;

        let mut to_move = Vec::new();

        // Get the first key pressed
        if let Some(key) = input_queue.keys_pressed.pop() {
            // get all the movables and immovables
            let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                .join()
                .map(|t| ((t.2.x, t.2.y), t.0.id()))
                .collect::<HashMap<_, _>>();
            let immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                .join()
                .map(|t| ((t.2.x, t.2.y), t.0.id()))
                .collect::<HashMap<_, _>>();

            for (position, _player) in (&positions, &players).join() {
                let (start, end, is_x) = match key {
                    KeyCode::K => (position.y, 0, false),
                    KeyCode::Up => (position.y, 0, false),
                    KeyCode::J => (position.y, MAP_HEIGHT, false),
                    KeyCode::Down => (position.y, MAP_HEIGHT, false),
                    KeyCode::H => (position.x, 0, true),
                    KeyCode::Left => (position.x, 0, true),
                    KeyCode::L => (position.x, MAP_WIDTH, true),
                    KeyCode::Right => (position.x, MAP_WIDTH, true),
                    _ => continue,
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };

                    // find a movable
                    // if it exists, we try to move it and continue
                    // if it doesn't exist, we continue and try to find an immovable instead
                    match mov.get(&pos) {
                        Some(id) => to_move.push((key, *id)),
                        None => {
                            // find an immovable
                            // if it exists, we need to stop and not move anything
                            // if it doesn't exist, we stop because we found a gap
                            match immov.get(&pos) {
                                Some(_id) => to_move.clear(),
                                None => break,
                            }
                        }
                    }
                }
            }

            // Now actually move what needs to be moved
            for (key, id) in to_move {
                let position = positions.get_mut(entities.entity(id));
                if let Some(position) = position {
                    match key {
                        KeyCode::K => position.y -= 1,
                        KeyCode::Up => position.y -= 1,
                        KeyCode::J => position.y += 1,
                        KeyCode::Down => position.y += 1,
                        KeyCode::H => position.x -= 1,
                        KeyCode::Left => position.x -= 1,
                        KeyCode::L => position.x += 1,
                        KeyCode::Right => position.x += 1,
                        _ => (),
                    }
                }
            }
        }
    }
}
