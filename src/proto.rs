use macroquad::prelude::*;

use crate::constants::{
    BACKGROUND, HEIGHT, PROTO_BULLET_SPREAD, PROTO_CLUSTER_SPREAD, PROTO_COLOR,
    PROTO_DEATH_GRAVITY, PROTO_DEATH_INITIAL_DROP_SPEED, PROTO_DEATH_SPIN_HOLD, PROTO_HEALTH,
    PROTO_IDLE_ROTATION_SPEED, PROTO_PROJECTILE_COLOR, PROTO_PROJECTILE_SPEED, PROTO_RADIUS,
    PROTO_SPINUP_DURATION, PROTO_SPINUP_HOLD, PROTO_SPINUP_PEAK_SPEED, PROTO_SPINUP_RAMP_DOWN,
    PROTO_SPINUP_RAMP_UP,
};
use crate::projectile::{BulletProjectile, Projectile, ProjectileKind};
use crate::shape::Triangle;
use crate::state::GameState;
use crate::utils::smoothstep;

#[derive(Clone, Copy)]
pub struct ProtoInitState {
    pub elapsed: f32,
}

impl ProtoInitState {
    pub fn new() -> Self {
        ProtoInitState { elapsed: 0.0 }
    }
}

#[derive(Clone, Copy)]
pub struct ProtoIdleState {
    pub time_to_spin: f32,
}

impl ProtoIdleState {
    pub fn new() -> Self {
        // randomized time to spin
        let time_to_spin = rand::gen_range(3.0, 7.0);
        ProtoIdleState { time_to_spin }
    }
}

#[derive(Clone, Copy)]
pub struct ProtoSpinState {
    pub elapsed: f32,
    pub fired: bool,
}

impl ProtoSpinState {
    pub fn new() -> Self {
        ProtoSpinState {
            elapsed: 0.0,
            fired: false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ProtoDyingState {
    pub elapsed: f32,
    pub fall_speed: f32,
}

impl ProtoDyingState {
    pub fn new() -> Self {
        ProtoDyingState {
            elapsed: 0.0,
            fall_speed: PROTO_DEATH_INITIAL_DROP_SPEED,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ProtoState {
    Init(ProtoInitState),
    Idle(ProtoIdleState),
    Spin(ProtoSpinState),
    Dying(ProtoDyingState),
    Dead,
}

pub struct Proto {
    pub position: Vec2,
    state: ProtoState,
    rotation: f32,
    current_health: i32,
    total_health: i32,
    triangle: Triangle,
    mask: Triangle,
    pub slot_idx: usize,
}

impl Proto {
    pub fn new(position: Vec2, slot_idx: usize, extra_health: i32) -> Self {
        let max_hp = PROTO_HEALTH + extra_health;
        let mut triangle = Triangle::new(PROTO_RADIUS, PROTO_COLOR);
        triangle.filled = false;
        let mask = Triangle::new(PROTO_RADIUS, BACKGROUND);
        Proto {
            position,
            state: ProtoState::Init(ProtoInitState::new()),
            rotation: 0.0,
            current_health: max_hp,
            total_health: max_hp,
            triangle,
            mask,
            slot_idx,
        }
    }

    pub fn update(&mut self, dt: f32, state: &mut GameState, player_pos: Vec2) {
        // if dead, do nothing
        if matches!(self.state, ProtoState::Dead) {
            return;
        }

        // if its 0 and not dying or dead, then transition to dead
        if self.current_health == 0
            && !matches!(self.state, ProtoState::Dying(_) | ProtoState::Dead)
        {
            self.state = ProtoState::Dying(ProtoDyingState::new());
        }

        match self.state {
            ProtoState::Init(init) => self.update_init(dt, init),
            ProtoState::Idle(idle) => self.update_idle(dt, idle),
            ProtoState::Spin(spin) => self.update_spin(dt, spin, state, player_pos),
            ProtoState::Dying(dying) => self.update_dying(dt, dying),
            ProtoState::Dead => {}
        }
    }

    fn update_init(&mut self, dt: f32, mut init: ProtoInitState) {
        init.elapsed += dt;
        self.rotation += Self::spinup_speed(init.elapsed, PROTO_SPINUP_HOLD) * dt;

        if init.elapsed >= PROTO_SPINUP_DURATION {
            self.state = ProtoState::Idle(ProtoIdleState::new());
        } else {
            self.state = ProtoState::Init(init);
        }
    }

    fn update_idle(&mut self, dt: f32, mut idle: ProtoIdleState) {
        // slight rotation
        self.rotation += PROTO_IDLE_ROTATION_SPEED * dt;

        idle.time_to_spin -= dt;
        if idle.time_to_spin <= 0.0 {
            self.state = ProtoState::Spin(ProtoSpinState::new());
        } else {
            self.state = ProtoState::Idle(idle);
        }
    }

    fn update_spin(
        &mut self,
        dt: f32,
        mut spin: ProtoSpinState,
        state: &mut GameState,
        player_pos: Vec2,
    ) {
        spin.elapsed += dt;
        self.rotation += Self::spinup_speed(spin.elapsed, PROTO_SPINUP_HOLD) * dt;

        // fire when reach peak spin speedf
        if !spin.fired && spin.elapsed >= PROTO_SPINUP_RAMP_UP {
            self.fire_clusters(state, player_pos);
            spin.fired = true;
        }

        if spin.elapsed >= PROTO_SPINUP_DURATION {
            self.state = ProtoState::Idle(ProtoIdleState::new());
        } else {
            self.state = ProtoState::Spin(spin);
        }
    }

    fn update_dying(&mut self, dt: f32, mut dying: ProtoDyingState) {
        dying.elapsed += dt;
        // keep spinning at peak speed during fall
        self.rotation += Self::spinup_speed(dying.elapsed, PROTO_DEATH_SPIN_HOLD) * dt;

        // gravity fall
        dying.fall_speed += PROTO_DEATH_GRAVITY * dt;
        self.position.y += dying.fall_speed * dt;

        // once off-screen (with margin), transition to Dead
        // and arena will kill it off screen
        if self.position.y - PROTO_RADIUS > HEIGHT {
            self.state = ProtoState::Dead;
        } else {
            self.state = ProtoState::Dying(dying);
        }
    }

    pub fn kill(&mut self) {
        if !matches!(self.state, ProtoState::Dying(_) | ProtoState::Dead) {
            self.current_health = 0;
            self.state = ProtoState::Dying(ProtoDyingState::new());
        }
    }

    pub fn take_damage(&mut self, damage: i32) -> bool {
        let was_alive = self.current_health > 0;
        self.current_health = (self.current_health - damage).max(0);
        was_alive && self.current_health == 0
    }

    pub fn is_invulnerable(&self) -> bool {
        matches!(
            self.state,
            ProtoState::Init(_) | ProtoState::Dying(_) | ProtoState::Dead
        )
    }

    pub fn is_dead(&self) -> bool {
        matches!(self.state, ProtoState::Dying(_) | ProtoState::Dead)
    }

    pub fn is_fully_dead(&self) -> bool {
        matches!(self.state, ProtoState::Dead)
    }

    pub fn draw(&self) {
        if matches!(self.state, ProtoState::Dead) {
            return;
        }

        self.mask.draw(self.position, self.rotation, 1.0);
        self.triangle.draw(self.position, self.rotation, 1.0);
    }

    fn spinup_speed(elapsed: f32, hold: f32) -> f32 {
        let hold_end = PROTO_SPINUP_RAMP_UP + hold;
        let factor = if elapsed < PROTO_SPINUP_RAMP_UP {
            smoothstep(elapsed / PROTO_SPINUP_RAMP_UP)
        } else if elapsed < hold_end {
            1.0
        } else {
            1.0 - smoothstep((elapsed - hold_end) / PROTO_SPINUP_RAMP_DOWN)
        };
        PROTO_IDLE_ROTATION_SPEED + (PROTO_SPINUP_PEAK_SPEED - PROTO_IDLE_ROTATION_SPEED) * factor
    }

    fn fire_clusters(&self, state: &mut GameState, player_pos: Vec2) {
        let to_player = (player_pos - self.position).normalize_or_zero();
        let center_angle = to_player.y.atan2(to_player.x);

        for c_idx in -1..=1 {
            let cluster_angle = center_angle + (c_idx as f32) * PROTO_CLUSTER_SPREAD;
            for b_idx in -1..=1 {
                let angle = cluster_angle + (b_idx as f32) * PROTO_BULLET_SPREAD;
                let dir = Vec2::new(angle.cos(), angle.sin());

                state
                    .projectiles
                    .push(Projectile::Bullet(BulletProjectile::new(
                        self.position,
                        dir * PROTO_PROJECTILE_SPEED,
                        ProjectileKind::Boss,
                        PROTO_PROJECTILE_COLOR,
                    )));
            }
        }
    }
}
