use ggez::{
    event::{self, KeyCode, KeyMods},
    timer, Context, GameError, GameResult,
};
use specs::{RunNow, World, WorldExt};

use crate::{
    resources::{InputQueue, Time},
    systems::*,
};

// This struct will hold all our game state
// For now there is nothing to be held, but we'll add
// things shortly.
pub struct Game {
    pub world: World,
}

// This is the main event loop. ggez tells us to implement
// two things:
// - updating
// - rendering
impl event::EventHandler<GameError> for Game {
    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        println!("Key pressed: {:?}", keycode);

        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(keycode);
    }

    fn update(&mut self, context: &mut Context) -> GameResult {
        // Run input system
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }

        // Run gameplay state system
        {
            let mut gss = GameplayStateSystem {};
            gss.run_now(&self.world);
        }

        // Get and update time resource
        {
            let mut time = self.world.write_resource::<Time>();
            time.delta += timer::delta(context);
        }

        // Run event system
        {
            let mut es = EventSystem { context };
            es.run_now(&self.world);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        {
            let mut rs = RenderingSystem { context };
            rs.run_now(&self.world);
        }
        Ok(())
    }
}
