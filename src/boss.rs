use macroquad::prelude::*;

use crate::constants::{BOSS_HEIGHT, BOSS_IDLE_ROTATION_SPEED, BOSS_WIDTH};
use crate::shape::Rectangle;

#[derive(Clone, Copy)]
enum BossState {
    // not doing anything yet; slowly spins in place
    Idle,
    // future: Moving { .. }, Attacking { .. }, etc.
}

pub struct Boss {
    pub position: Vec2,
    pub rect: Rectangle,
    state: BossState,
    // current orientation in radians
    rotation: f32,
}

impl Boss {
    pub fn new(position: Vec2) -> Self {
        let mut rect = Rectangle::new(vec2(BOSS_WIDTH, BOSS_HEIGHT), RED);
        rect.filled = false;
        Boss {
            position,
            rect,
            state: BossState::Idle,
            rotation: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        match self.state {
            BossState::Idle => self.update_idle(dt),
        }
    }

    fn update_idle(&mut self, dt: f32) {
        // spin clockwise in place while idle
        self.rotation += BOSS_IDLE_ROTATION_SPEED * dt;
    }

    pub fn draw(&self) {
        self.rect.draw_rotated(self.position, self.rotation, 1.0);
    }
}
