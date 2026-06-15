use macroquad::prelude::*;

use crate::constants::{BOSS_HEIGHT, BOSS_WIDTH};
use crate::shape::Rectangle;

pub struct Boss {
    pub position: Vec2,
    pub rect: Rectangle,
}

impl Boss {
    pub fn new(position: Vec2) -> Self {
        let mut rect = Rectangle::new(vec2(BOSS_WIDTH, BOSS_HEIGHT), RED);
        rect.filled = false;
        Boss { position, rect }
    }

    pub fn update(&mut self, dt: f32) {
        // todo: boss movement and attack patterns
        let _ = dt;
    }

    pub fn draw(&self) {
        self.rect.draw(self.position, 1.0);
    }
}
