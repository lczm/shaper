use macroquad::prelude::*;

use crate::constants::{
    ARENA_BORDER_THICKNESS, PHASE_DISTANCE, PHASE_DURATION, PHASE_GHOST_OPACITY, PHASE_MIN_OPACITY,
    PHASE_TRAIL_LENGTH, PLAYER_CIRCLE_RADIUS, PLAYER_FIRE_INTERVAL, PLAYER_PROJECTILE_SPEED,
    PLAYER_SPEED,
};
use crate::input::Input;
use crate::projectile::{Projectile, ProjectileKind};
use crate::shape::Circle;
use crate::state::GameState;

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
    // recent positions (newest first) used to draw the phase ghost trail
    trail: Vec<Vec2>,
    // counts down to the next shot
    fire_timer: f32,
}

impl Player {
    pub fn new(position: Vec2) -> Self {
        Player {
            position,
            circle: Circle::new(PLAYER_CIRCLE_RADIUS, YELLOW),
            state: PlayerState::Normal,
            trail: Vec::new(),
            fire_timer: 0.0,
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

    pub fn update(&mut self, dt: f32, input: &Input, bounds: Rect, state: &mut GameState) {
        let dir = Self::calculate_movement_vector(input);

        // record where we are before moving so the ghost trail sits behind us
        if matches!(self.state, PlayerState::Phasing { .. }) {
            self.trail.insert(0, self.position);
            self.trail.truncate(PHASE_TRAIL_LENGTH);
        }

        match self.state {
            PlayerState::Normal => self.update_player_normal(dt, input, dir),
            PlayerState::Phasing { direction, elapsed } => {
                self.update_player_phasing(dt, dir, direction, elapsed)
            }
        }

        // drop the trail once the phase is over
        if !matches!(self.state, PlayerState::Phasing { .. }) {
            self.trail.clear();
        }

        // get the total inset from the bounds edge to the center of the player circle
        let inset =
            self.circle.radius + self.circle.thickness / 2.0 + ARENA_BORDER_THICKNESS / 2.0;

        // clamp player position within the arena
        self.position.x = self
            .position
            .x
            .clamp(bounds.x + inset, bounds.x + bounds.w - inset);
        self.position.y = self
            .position
            .y
            .clamp(bounds.y + inset, bounds.y + bounds.h - inset);

        self.fire(dt, state);
    }

    // continuously shoot straight up on a fixed cadence
    fn fire(&mut self, dt: f32, state: &mut GameState) {
        if self.fire_timer > 0.0 {
            self.fire_timer -= dt;
        }
        if self.fire_timer <= 0.0 {
            self.fire_timer = PLAYER_FIRE_INTERVAL;
            state.projectiles.push(Projectile::new(
                self.position,
                vec2(0.0, -PLAYER_PROJECTILE_SPEED),
                ProjectileKind::Player,
                SKYBLUE,
            ));
        }
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
        // ghost trail behind the phase movement: older ghosts fade out
        let trail_len = self.trail.len() as f32;
        for (i, &pos) in self.trail.iter().enumerate() {
            let fade = 1.0 - i as f32 / trail_len;
            self.circle
                .draw_colored(pos, GRAY, PHASE_GHOST_OPACITY * fade);
        }

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
