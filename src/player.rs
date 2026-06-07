use macroquad::prelude::*;

use crate::{constants::PLAYER_CIRCLE_RADIUS, shape::Circle};

pub struct Player {
    pub position: Vec2,
    pub circle: Circle,
}

impl Player {
    pub fn new(position: Vec2) -> Self {
        Player {
            position,
            circle: Circle::new(PLAYER_CIRCLE_RADIUS, YELLOW),
        }
    }

    pub fn draw(&self) {
        self.circle.draw(self.position);
    }
}
