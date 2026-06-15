use macroquad::prelude::*;

use crate::constants::{ARENA_BORDER_THICKNESS, PROJECTILE_RADIUS};
use crate::shape::Circle;

pub struct Projectile {
    pub position: Vec2,
    velocity: Vec2,
    circle: Circle,
}

impl Projectile {
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        let mut circle = Circle::new(PROJECTILE_RADIUS, RED);
        circle.filled = true;
        Projectile {
            position,
            velocity,
            circle,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }

    // true once the bullet's edge reaches the inner edge of the border, so it's
    // culled right at the border instead of visibly crossing it
    pub fn is_off_screen(&self, bounds: Rect) -> bool {
        let r = self.circle.radius;
        let inset = ARENA_BORDER_THICKNESS / 2.0;
        self.position.x - r < bounds.x + inset
            || self.position.x + r > bounds.x + bounds.w - inset
            || self.position.y - r < bounds.y + inset
            || self.position.y + r > bounds.y + bounds.h - inset
    }

    pub fn draw(&self) {
        self.circle.draw(self.position, 1.0);
    }
}
