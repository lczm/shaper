use macroquad::prelude::*;

use crate::constants::{
    BACKGROUND, BOSS_COLOR, BOSS_FIRE_INTERVAL, BOSS_HEIGHT, BOSS_IDLE_ROTATION_SPEED,
    BOSS_PROJECTILE_COLOR, BOSS_PROJECTILE_COUNT, BOSS_WIDTH, PROJECTILE_SPEED,
};
use crate::projectile::{Projectile, ProjectileKind};
use crate::shape::Rectangle;
use crate::state::GameState;

// spins in place and dribbles out projectiles; fire_timer counts down to the next shot
#[derive(Clone, Copy)]
struct IdleState {
    fire_timer: f32,
}

impl IdleState {
    fn new() -> Self {
        IdleState {
            fire_timer: BOSS_FIRE_INTERVAL,
        }
    }
}

#[derive(Clone, Copy)]
enum BossState {
    Idle(IdleState),
    // future: Moving { .. }, Attacking { .. }, etc.
}

pub struct Boss {
    pub position: Vec2,
    pub rect: Rectangle,
    // filled background-colored rect that hides projectiles under the boss
    mask: Rectangle,
    state: BossState,
    // current orientation in radians
    rotation: f32,
}

impl Boss {
    pub fn new(position: Vec2) -> Self {
        let mut rect = Rectangle::new(vec2(BOSS_WIDTH, BOSS_HEIGHT), BOSS_COLOR);
        rect.filled = false;
        // the mask is the same size as the rect but filled with the background color, so
        // the projectiles under the boss wont be visible
        let mask = Rectangle::new(vec2(BOSS_WIDTH, BOSS_HEIGHT), BACKGROUND);
        Boss {
            position,
            rect,
            mask,
            state: BossState::Idle(IdleState::new()),
            rotation: 0.0,
        }
    }

    // advance the boss, pushing any projectiles it fires into the game state
    pub fn update(&mut self, dt: f32, state: &mut GameState) {
        match self.state {
            BossState::Idle(idle) => self.update_idle(dt, idle, state),
        }
    }

    fn update_idle(&mut self, dt: f32, mut idle: IdleState, state: &mut GameState) {
        // spin clockwise in place while idle
        self.rotation += BOSS_IDLE_ROTATION_SPEED * dt;

        idle.fire_timer -= dt;
        if idle.fire_timer <= 0.0 {
            idle.fire_timer += BOSS_FIRE_INTERVAL;
            self.fire_ring(state);
        }

        self.state = BossState::Idle(idle);
    }

    // standard semi-ring pattern projectile downwards
    fn fire_ring(&self, state: &mut GameState) {
        for i in 0..BOSS_PROJECTILE_COUNT {
            // angles in (0, PI) all point downward (+y under the y-down camera)
            let angle = std::f32::consts::PI * (i as f32 + 0.5) / BOSS_PROJECTILE_COUNT as f32;
            let dir = vec2(angle.cos(), angle.sin());
            state.projectiles.push(Projectile::new(
                self.position,
                dir * PROJECTILE_SPEED,
                ProjectileKind::Boss,
                BOSS_PROJECTILE_COLOR,
            ));
        }
    }

    pub fn draw(&self) {
        // draw the mask first then the boss
        self.mask.draw_rotated(self.position, self.rotation, 1.0);
        self.rect.draw_rotated(self.position, self.rotation, 1.0);
    }
}
