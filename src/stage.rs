use macroquad::prelude::*;

use crate::constants::{HEIGHT, WIDTH};
use crate::input::Input;
use crate::player::Player;

pub struct Stage {
    pub player: Player,
}

impl Stage {
    pub fn new() -> Self {
        Stage {
            player: Player::new(vec2(WIDTH / 2.0, HEIGHT * 4.0 / 5.0)),
        }
    }

    pub fn update(&mut self, dt: f32, input: &Input) {
        self.player.update(dt, input);

        // todo for when clickable objects are on the screen
        // get coordiantes via input.mouse.x/y
        // if input.primary_pressed {}
    }

    pub fn draw(&self) {
        self.player.draw();
    }
}

impl Default for Stage {
    fn default() -> Self {
        Stage::new()
    }
}
