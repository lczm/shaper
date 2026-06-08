use macroquad::prelude::*;

use crate::constants::{
    HEIGHT, PHASE_DISTANCE, PHASE_DURATION, PHASE_MIN_OPACITY, PLAYER_CIRCLE_RADIUS, PLAYER_SPEED,
    WIDTH,
};
use crate::input::Input;
use crate::shape::Circle;

#[derive(Clone, Copy)]
enum PlayerState {
    Normal,
    // when phasing, it is i-framed
    Phasing { direction: Vec2, elapsed: f32 },
}

pub struct Player {
    pub position: Vec2,
    pub circle: Circle,
    state: PlayerState,
}

impl Player {
    pub fn new(position: Vec2) -> Self {
        Player {
            position,
            circle: Circle::new(PLAYER_CIRCLE_RADIUS, YELLOW),
            state: PlayerState::Normal,
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
        let dir = Self::calculate_movement_vector(input);

        match self.state {
            PlayerState::Normal => self.update_player_normal(dt, input, dir),
            PlayerState::Phasing { direction, elapsed } => {
                self.update_player_phasing(dt, dir, direction, elapsed)
            }
        }

        // keep to within the area
        let radius = self.circle.radius;
        self.position.x = self.position.x.clamp(radius, WIDTH - radius);
        self.position.y = self.position.y.clamp(radius, HEIGHT - radius);
    }

    fn update_player_normal(&mut self, dt: f32, input: &Input, dir: Vec2) {
        self.position += dir * PLAYER_SPEED * dt;
        // if tried to pahse with shift, it only works
        // when the player is moving, shift to phasing state
        if input.shift_pressed && dir != Vec2::ZERO {
            self.state = PlayerState::Phasing {
                direction: dir,
                elapsed: 0.0,
            };
            // change to blue when phasing
            self.circle.color = ORANGE;
        }
    }

    fn update_player_phasing(&mut self, dt: f32, dir: Vec2, mut direction: Vec2, mut elapsed: f32) {
        // keep the last direction used
        if dir != Vec2::ZERO {
            direction = dir;
        }
        let phase_speed = PHASE_DISTANCE / PHASE_DURATION;
        self.position += direction * phase_speed * dt;

        elapsed += dt;
        if elapsed >= PHASE_DURATION {
            self.state = PlayerState::Normal;
            self.circle.color = YELLOW;
        } else {
            self.state = PlayerState::Phasing { direction, elapsed };
        }
    }

    // true during the phase i-frame window (for future collision checks)
    // pub fn is_invulnerable(&self) -> bool {
    //     matches!(self.state, PlayerState::Phasing { .. })
    // }

    pub fn draw(&self) {
        // fade out then back in over the phase (dips at the midpoint)
        let opacity = match self.state {
            PlayerState::Normal => 1.0,
            PlayerState::Phasing { elapsed, .. } => {
                let t = (elapsed / PHASE_DURATION).clamp(0.0, 1.0);
                1.0 - (1.0 - PHASE_MIN_OPACITY) * (t * std::f32::consts::PI).sin()
            }
        };
        self.circle.draw(self.position, opacity);
    }
}
