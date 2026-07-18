use crate::constants::{BACKGROUND, HEIGHT, PROTO_BEAM_COLOR, PROTO_BEAM_SPEED, PROTO_RADIUS};
use crate::shape::Triangle;
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ProtoBeamState {
    Active,
    Dying { elapsed: f32, fall_speed: f32 },
    Dead,
}

pub struct ProtoBeam {
    pub position: Vec2,
    pub state: ProtoBeamState,
    pub rotation: f32,
    // -1.0 for left side, 1.0 for right side
    pub side: f32,
    // -1.0 for moving up, 1.0 for moving down
    pub y_dir: f32,
    pub speed: f32,
    pub triangle: Triangle,
    pub mask: Triangle,
}

impl ProtoBeam {
    pub fn new(position: Vec2, side: f32) -> Self {
        // Left points right (rotation = 0.0), right points left (rotation = PI)
        let rotation = if side < 0.0 {
            0.0
        } else {
            std::f32::consts::PI
        };

        let mut triangle = Triangle::new(PROTO_RADIUS, PROTO_BEAM_COLOR);
        triangle.filled = false;
        let mask = Triangle::new(PROTO_RADIUS, BACKGROUND);

        ProtoBeam {
            position,
            state: ProtoBeamState::Active,
            rotation,
            side,
            y_dir: 1.0,
            speed: PROTO_BEAM_SPEED,
            triangle,
            mask,
        }
    }

    pub fn update(&mut self, dt: f32, bounds: Rect) {
        match self.state {
            ProtoBeamState::Active => {
                // move up/down
                self.position.y += self.y_dir * self.speed * dt;

                // bounce off the bottom probably
                let margin = 40.0;
                let min_y = bounds.y + margin;
                let max_y = bounds.y + bounds.h - margin;

                if self.position.y <= min_y {
                    self.position.y = min_y;
                    self.y_dir = 1.0;
                } else if self.position.y >= max_y {
                    self.position.y = max_y;
                    self.y_dir = -1.0;
                }
            }
            ProtoBeamState::Dying {
                ref mut elapsed,
                ref mut fall_speed,
            } => {
                *elapsed += dt;
                // Spin while falling
                self.rotation += 5.0 * dt;

                // gravity fall
                *fall_speed += 1200.0 * dt;
                self.position.y += *fall_speed * dt;

                // once off-screen (with margin), transition to Dead
                if self.position.y - PROTO_RADIUS > HEIGHT {
                    self.state = ProtoBeamState::Dead;
                }
            }
            ProtoBeamState::Dead => {}
        }
    }

    pub fn die(&mut self) {
        if self.state == ProtoBeamState::Active {
            self.state = ProtoBeamState::Dying {
                elapsed: 0.0,
                fall_speed: 30.0,
            };
        }
    }

    pub fn is_fully_dead(&self) -> bool {
        self.state == ProtoBeamState::Dead
    }

    pub fn draw(&self) {
        if self.state == ProtoBeamState::Dead {
            return;
        }
        self.mask.draw(self.position, self.rotation, 1.0);
        self.triangle.draw(self.position, self.rotation, 1.0);
    }
}
