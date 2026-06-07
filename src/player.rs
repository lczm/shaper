use macroquad::prelude::*;

use crate::constants::{HEIGHT, PLAYER_CIRCLE_RADIUS, PLAYER_SPEED, WIDTH};
use crate::input::Input;
use crate::shape::Circle;

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

    pub fn calculate_movement_vector(input: &Input) -> Vec2 {
        let mut direction = Vec2::ZERO;
        if input.arrow_up {
            direction.y -= 1.0;
        }
        if input.arrow_down {
            direction.y += 1.0;
        }
        if input.arrow_left {
            direction.x -= 1.0;
        }
        if input.arrow_right {
            direction.x += 1.0;
        }
        // normalize so diagonal movement isn't faster than axis-aligned
        if direction != Vec2::ZERO {
            direction = direction.normalize();
        }
        direction
    }

    pub fn update(&mut self, dt: f32, input: &Input) {
        // build direction and update player movement
        let direction = Self::calculate_movement_vector(input);
        self.position += direction * PLAYER_SPEED * dt;

        // keep to within the area
        let radius = self.circle.radius;
        self.position.x = self.position.x.clamp(radius, WIDTH - radius);
        self.position.y = self.position.y.clamp(radius, HEIGHT - radius);
    }

    pub fn draw(&self) {
        self.circle.draw(self.position);
    }
}
