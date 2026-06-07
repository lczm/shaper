use macroquad::prelude::*;

use crate::constants::{HEIGHT, WIDTH};
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

    pub fn update(&mut self, _dt: f32) {}

    pub fn draw(&self) {
        self.player.draw();
    }
}

impl Default for Stage {
    fn default() -> Self {
        Stage::new()
    }
}
