use macroquad::prelude::*;

use crate::input::Input;
use crate::player::Player;
use crate::state::GameState;

pub struct Stage {
    pub player: Player,
}

impl Stage {
    pub fn new(bounds: Rect) -> Self {
        // start the player near the bottom-center of the arena
        Stage {
            player: Player::new(vec2(
                bounds.x + bounds.w / 2.0,
                bounds.y + bounds.h * 4.0 / 5.0,
            )),
        }
    }

    pub fn update(&mut self, dt: f32, input: &Input, bounds: Rect, state: &mut GameState) {
        self.player.update(dt, input, bounds);

        // todo: gameplay that mutates shared state lives here, e.g. when an
        // enemy bullet hits the player:
        //   state.lives = state.lives.saturating_sub(1);
        let _ = state;

        // todo for when clickable objects are on the screen
        // get coordiantes via input.mouse.x/y
        // if input.primary_pressed {}
    }

    pub fn draw(&self) {
        self.player.draw();
    }
}
