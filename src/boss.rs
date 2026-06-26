use macroquad::prelude::*;

use crate::constants::{
    BACKGROUND, BOSS_COLOR, BOSS_FIRE_INTERVAL, BOSS_HEIGHT, BOSS_IDLE_ROTATION_SPEED,
    BOSS_PROJECTILE_COLOR, BOSS_PROJECTILE_COUNT, BOSS_SPINUP_DURATION, BOSS_SPINUP_HOLD,
    BOSS_SPINUP_PEAK_SPEED, BOSS_SPINUP_RAMP_DOWN, BOSS_SPINUP_RAMP_UP, BOSS_WIDTH,
    PROJECTILE_SPEED,
};
use crate::projectile::{BeamProjectile, BulletProjectile, Projectile, ProjectileKind};
use crate::shape::Rectangle;
use crate::state::GameState;
use crate::utils::smoothstep;

// spin up animation, spin spin spin
#[derive(Clone, Copy)]
struct InitState {
    elapsed: f32,
}

impl InitState {
    fn new() -> Self {
        InitState { elapsed: 0.0 }
    }
}

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
    Init(InitState),
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
            state: BossState::Init(InitState::new()),
            rotation: 0.0,
        }
    }

    // match state and delegate to the appropriate update function
    pub fn update(&mut self, dt: f32, state: &mut GameState, bounds: Rect) {
        match self.state {
            BossState::Init(init) => self.update_init(dt, init, state, bounds),
            BossState::Idle(idle) => self.update_idle(dt, idle, state),
        }
    }

    // spin up animation
    fn update_init(&mut self, dt: f32, mut init: InitState, state: &mut GameState, bounds: Rect) {
        init.elapsed += dt;
        self.rotation += Self::spinup_speed(init.elapsed) * dt;

        if init.elapsed >= BOSS_SPINUP_DURATION {
            // the beam comes online once the spin settles
            self.spawn_beam(state, bounds);
            self.state = BossState::Idle(IdleState::new());
        } else {
            self.state = BossState::Init(init);
        }
    }

    fn update_idle(&mut self, dt: f32, mut idle: IdleState, state: &mut GameState) {
        // steady clockwise spin
        self.rotation += BOSS_IDLE_ROTATION_SPEED * dt;

        idle.fire_timer -= dt;
        if idle.fire_timer <= 0.0 {
            idle.fire_timer += BOSS_FIRE_INTERVAL;
            self.fire_ring(state);
        }

        self.state = BossState::Idle(idle);
    }

    // accelerate, then hold and decelerate back to idle speed and transition to idle
    fn spinup_speed(elapsed: f32) -> f32 {
        let hold_end = BOSS_SPINUP_RAMP_UP + BOSS_SPINUP_HOLD;
        let factor = if elapsed < BOSS_SPINUP_RAMP_UP {
            // 0 -> 1
            smoothstep(elapsed / BOSS_SPINUP_RAMP_UP) // 0 -> 1
        } else if elapsed < hold_end {
            // held at peak
            1.0
        } else {
            // 1 -> 0
            1.0 - smoothstep((elapsed - hold_end) / BOSS_SPINUP_RAMP_DOWN)
        };
        BOSS_IDLE_ROTATION_SPEED + (BOSS_SPINUP_PEAK_SPEED - BOSS_IDLE_ROTATION_SPEED) * factor
    }

    // standard semi-ring pattern projectile downwards
    fn fire_ring(&self, state: &mut GameState) {
        for i in 0..BOSS_PROJECTILE_COUNT {
            // angles in (0, PI) all point downward (+y under the y-down camera)
            let angle = std::f32::consts::PI * (i as f32 + 0.5) / BOSS_PROJECTILE_COUNT as f32;
            let dir = vec2(angle.cos(), angle.sin());
            state
                .projectiles
                .push(Projectile::Bullet(BulletProjectile::new(
                    self.position,
                    dir * PROJECTILE_SPEED,
                    ProjectileKind::Boss,
                    BOSS_PROJECTILE_COLOR,
                )));
        }
    }

    // spawn a beam from the boss straight down to the arena floor. BeamProjectile takes
    // arbitrary start/end points, so this could be aimed anywhere on the map.
    fn spawn_beam(&self, state: &mut GameState, bounds: Rect) {
        // todo : randomize start and end at some point
        let start = self.position;
        let end = vec2(self.position.x, bounds.y + bounds.h);
        state
            .projectiles
            .push(Projectile::Beam(BeamProjectile::new(start, end)));
    }

    // current orientation in radians (used by collision checks)
    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn draw(&self) {
        // draw the mask first then the boss
        self.mask.draw_rotated(self.position, self.rotation, 1.0);
        self.rect.draw_rotated(self.position, self.rotation, 1.0);
    }
}
