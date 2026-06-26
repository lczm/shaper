use macroquad::prelude::*;

use crate::collision::{circle_circle_overlap, segment_circle_overlap};
use crate::constants::{
    BEAM_WIDTH, BOMB_BORDER_COLOR, BOMB_BORDER_THICKNESS, BOMB_DURATION, BOMB_RADIUS,
    PROJECTILE_RADIUS,
};
use crate::projectile::{Projectile, ProjectileKind};

pub struct Bomb {
    center: Vec2,
    elapsed: f32,
}

impl Bomb {
    pub fn new(center: Vec2) -> Self {
        Bomb {
            center,
            elapsed: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        self.elapsed += dt;
        self.elapsed < BOMB_DURATION
    }

    // true if the boss projectile is within the bomb radius to be cleared
    pub fn clears(&self, projectile: &Projectile) -> bool {
        match projectile {
            Projectile::Bullet(b) => match b.kind {
                ProjectileKind::Boss => {
                    circle_circle_overlap(b.position, PROJECTILE_RADIUS, self.center, BOMB_RADIUS)
                }
                ProjectileKind::Player { .. } => false,
            },
            Projectile::Beam(beam) => segment_circle_overlap(
                beam.start,
                beam.end,
                BEAM_WIDTH / 2.0,
                self.center,
                BOMB_RADIUS,
            ),
        }
    }

    pub fn draw(&self) {
        // fade the ring out over the bomb's lifetime
        let fade = 1.0 - (self.elapsed / BOMB_DURATION).clamp(0.0, 1.0);
        let color = Color {
            a: BOMB_BORDER_COLOR.a * fade,
            ..BOMB_BORDER_COLOR
        };
        draw_circle_lines(
            self.center.x,
            self.center.y,
            BOMB_RADIUS,
            BOMB_BORDER_THICKNESS,
            color,
        );
    }
}
