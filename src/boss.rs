use macroquad::prelude::*;

use crate::collision::{circle_circle_overlap, segment_circle_overlap};
use crate::constants::{
    BACKGROUND, BEAM_EDGE_OVERSHOOT, BEAM_WIDTH, BOSS_AIM_STEP, BOSS_AIM_STEPS, BOSS_BEAM_INTERVAL,
    BOSS_CLUSTER_COUNT, BOSS_CLUSTER_INTRA_GAP, BOSS_CLUSTER_SHOTS, BOSS_COLOR,
    BOSS_DEATH_BURST_COLOR, BOSS_DEATH_BURST_SPEED, BOSS_DEATH_BURST_THICKNESS, BOSS_DEATH_GRAVITY,
    BOSS_DEATH_INITIAL_DROP_SPEED, BOSS_DEATH_SPIN_HOLD, BOSS_FIRE_INTERVAL, BOSS_HEALTH,
    BOSS_HEIGHT, BOSS_IDLE_ROTATION_SPEED, BOSS_PROJECTILE_COLOR, BOSS_PROJECTILE_COUNT,
    BOSS_SPECIAL_DURATION, BOSS_SPECIAL_FIRE_INTERVAL, BOSS_SPECIAL_PROJECTILE_COLOR,
    BOSS_SPECIAL_PROJECTILE_SPEED, BOSS_SPECIAL_SPINUP_HOLD, BOSS_SPECIAL_SWEEP_STEP,
    BOSS_SPINUP_DURATION, BOSS_SPINUP_HOLD, BOSS_SPINUP_PEAK_SPEED, BOSS_SPINUP_RAMP_DOWN,
    BOSS_SPINUP_RAMP_UP, BOSS_WIDTH, HEALTH_BAR_DROP_SPEED, HEIGHT, PROJECTILE_RADIUS,
    PROJECTILE_SPEED,
};
use crate::projectile::{BeamProjectile, BulletProjectile, Projectile, ProjectileKind};
use crate::shape::Rectangle;
use crate::state::GameState;
use crate::utils::smoothstep;
use crate::world::GameEvent;

pub const BOSS_SPECIAL_HP_THRESHOLDS: [f32; 3] = [0.75, 0.5, 0.25];

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
    // how many volleys have been fired; drives the ping-pong aim offset
    volley: i32,
    // counts down to the next beam volley
    beam_timer: f32,
}

impl IdleState {
    fn new() -> Self {
        IdleState {
            fire_timer: BOSS_FIRE_INTERVAL,
            volley: 0,
            beam_timer: BOSS_BEAM_INTERVAL,
        }
    }
}

// this state does a spin up + spray some clusterd volleys
// to add some variation for difficulty
#[derive(Clone, Copy)]
struct SpecialMoveState {
    elapsed: f32,
    fire_timer: f32,
    volley: i32,
}

impl SpecialMoveState {
    fn new() -> Self {
        SpecialMoveState {
            elapsed: 0.0,
            fire_timer: BOSS_SPECIAL_FIRE_INTERVAL,
            volley: 0,
        }
    }
}

// when health hits zero, a ring expands from the boss center out over the whole
// arena, deleting every projectile it touches. runs before the fall
#[derive(Clone, Copy)]
struct DeathBurstState {
    radius: f32,
}

impl DeathBurstState {
    fn new() -> Self {
        DeathBurstState { radius: 0.0 }
    }
}

// after the burst clears the field the boss spins up and falls off the bottom
// of the screen; fall_speed accelerates under gravity as it drops
#[derive(Clone, Copy)]
struct DyingState {
    elapsed: f32,
    fall_speed: f32,
}

impl DyingState {
    fn new() -> Self {
        DyingState {
            elapsed: 0.0,
            fall_speed: BOSS_DEATH_INITIAL_DROP_SPEED,
        }
    }
}

#[derive(Clone, Copy)]
enum BossState {
    Init(InitState),
    Idle(IdleState),
    SpecialMove(SpecialMoveState),
    // expanding ring clearing the field right after health reaches zero
    DeathBurst(DeathBurstState),
    // spinning and dropping off-screen after the burst clears
    Dying(DyingState),
    // animation finished; the boss is gone and no longer drawn or updated
    Dead,
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
    // boss health
    current_health: i32,
    total_health: i32,
    // displayed is the trailing chip value
    displayed_health: f32,
    // used to track the special moves fired
    special_moves_fired: usize,
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
            current_health: BOSS_HEALTH,
            total_health: BOSS_HEALTH,
            displayed_health: BOSS_HEALTH as f32,
            special_moves_fired: 0,
        }
    }

    // match state and delegate to the appropriate update function
    pub fn update(
        &mut self,
        dt: f32,
        state: &mut GameState,
        bounds: Rect,
        player_pos: Vec2,
        events: &mut Vec<GameEvent>,
    ) {
        // ease damage to current health
        self.update_displayed_health(dt);

        // death takes priority over everything: once health is gone, kick off
        // the death sequence (burst -> fall) unless it's already under way
        if self.current_health == 0
            && !matches!(
                self.state,
                BossState::DeathBurst(_) | BossState::Dying(_) | BossState::Dead
            )
        {
            self.state = BossState::DeathBurst(DeathBurstState::new());
        }

        // transition when its time to switch to a more difficult state
        if matches!(self.state, BossState::Idle(_)) {
            let health_frac = self.current_health as f32 / self.total_health as f32;
            if let Some(&threshold) = BOSS_SPECIAL_HP_THRESHOLDS.get(self.special_moves_fired) {
                if health_frac <= threshold {
                    self.special_moves_fired += 1;
                    self.state = BossState::SpecialMove(SpecialMoveState::new());
                }
            }
        }

        match self.state {
            BossState::Init(init) => self.update_init(dt, init, state),
            BossState::Idle(idle) => self.update_idle(dt, idle, state, bounds, player_pos),
            BossState::SpecialMove(sm) => self.update_special_move(dt, sm, state),
            BossState::DeathBurst(burst) => self.update_death_burst(dt, burst, state, bounds),
            BossState::Dying(dying) => self.update_dying(dt, dying, events),
            // nothing left to do once the boss is gone
            BossState::Dead => {}
        }
    }

    // spin up animation
    fn update_init(&mut self, dt: f32, mut init: InitState, _state: &mut GameState) {
        init.elapsed += dt;
        self.rotation += Self::spinup_speed(init.elapsed, BOSS_SPINUP_HOLD) * dt;

        if init.elapsed >= BOSS_SPINUP_DURATION {
            // the beam comes online once the spin settles
            self.state = BossState::Idle(IdleState::new());
        } else {
            self.state = BossState::Init(init);
        }
    }

    fn update_idle(
        &mut self,
        dt: f32,
        mut idle: IdleState,
        state: &mut GameState,
        bounds: Rect,
        player_pos: Vec2,
    ) {
        // steady clockwise spin
        self.rotation += BOSS_IDLE_ROTATION_SPEED * dt;

        idle.fire_timer -= dt;
        if idle.fire_timer <= 0.0 {
            idle.fire_timer += BOSS_FIRE_INTERVAL;
            // aim with this volley's ping-pong offset, then advance the counter
            self.fire_ring(state, Self::aim_offset(idle.volley));
            idle.volley += 1;
        }

        // beam valley timer, the number of beams scales with the hp loss
        idle.beam_timer -= dt;
        if idle.beam_timer <= 0.0 {
            idle.beam_timer += BOSS_BEAM_INTERVAL;
            self.fire_beams(state, bounds, player_pos);
        }

        self.state = BossState::Idle(idle);
    }

    // spin up while spraying
    fn update_special_move(
        &mut self,
        dt: f32,
        mut special_move: SpecialMoveState,
        state: &mut GameState,
    ) {
        special_move.elapsed += dt;
        self.rotation += Self::spinup_speed(special_move.elapsed, BOSS_SPECIAL_SPINUP_HOLD) * dt;

        special_move.fire_timer -= dt;
        if special_move.fire_timer <= 0.0 {
            special_move.fire_timer += BOSS_SPECIAL_FIRE_INTERVAL;
            // the sweep is a multiple of BOSS_SPECIAL_SWEEP_STEP, so the pattern drifts
            self.fire_clusters(state, special_move.volley as f32 * BOSS_SPECIAL_SWEEP_STEP);
            special_move.volley += 1;
        }

        if special_move.elapsed >= BOSS_SPECIAL_DURATION {
            self.state = BossState::Idle(IdleState::new());
        } else {
            self.state = BossState::SpecialMove(special_move);
        }
    }

    // death burst: grow the ring outward from the boss center, deleting every
    // projectile it has swept over. once it covers the whole arena, start the fall
    fn update_death_burst(
        &mut self,
        dt: f32,
        mut burst: DeathBurstState,
        state: &mut GameState,
        bounds: Rect,
    ) {
        burst.radius += BOSS_DEATH_BURST_SPEED * dt;

        // wipe any projectile the ring has reached (inside the current radius)
        let center = self.position;
        let r = burst.radius;
        state.projectiles.retain(|p| match p {
            Projectile::Bullet(b) => {
                !circle_circle_overlap(b.position, PROJECTILE_RADIUS, center, r)
            }
            Projectile::Beam(beam) => {
                !segment_circle_overlap(beam.start, beam.end, BEAM_WIDTH / 2.0, center, r)
            }
        });

        // done once the ring has passed the farthest arena corner
        if burst.radius >= Self::max_burst_radius(center, bounds) {
            self.state = BossState::Dying(DyingState::new());
        } else {
            self.state = BossState::DeathBurst(burst);
        }
    }

    // distance from the burst center to the farthest arena corner, so the ring
    // is guaranteed to cover the entire arena before the fall begins
    fn max_burst_radius(center: Vec2, bounds: Rect) -> f32 {
        [
            vec2(bounds.x, bounds.y),
            vec2(bounds.x + bounds.w, bounds.y),
            vec2(bounds.x, bounds.y + bounds.h),
            vec2(bounds.x + bounds.w, bounds.y + bounds.h),
        ]
        .iter()
        .map(|corner| center.distance(*corner))
        .fold(0.0, f32::max)
    }

    // death: keep spinning at peak speed while accelerating off the bottom of
    // the screen. once fully below the screen, fire the game over event and go dead
    fn update_dying(&mut self, dt: f32, mut dying: DyingState, events: &mut Vec<GameEvent>) {
        dying.elapsed += dt;
        // long hold keeps the spin pinned near peak speed the whole way down
        self.rotation += Self::spinup_speed(dying.elapsed, BOSS_DEATH_SPIN_HOLD) * dt;

        // gravity-driven fall
        dying.fall_speed += BOSS_DEATH_GRAVITY * dt;
        self.position.y += dying.fall_speed * dt;

        // fully past the bottom edge (BOSS_HEIGHT of margin so the whole body clears)
        if self.position.y - BOSS_HEIGHT > HEIGHT {
            events.push(GameEvent::GameOver);
            self.state = BossState::Dead;
        } else {
            self.state = BossState::Dying(dying);
        }
    }

    // simple way to scale beams with health
    fn beam_count(&self) -> usize {
        let frac = self.current_health as f32 / self.total_health as f32;
        if frac < 0.15 {
            4
        } else if frac < 0.30 {
            3
        } else if frac < 0.50 {
            2
        } else if frac < 0.80 {
            1
        } else {
            0
        }
    }

    // fire a volley of beams that all pass through the player's current position, so
    // every beam converges where the player is standing and they're forced to move.
    // each beam is a random point on a circle around the player, aimed back through them
    fn fire_beams(&self, state: &mut GameState, bounds: Rect, player_pos: Vec2) {
        let count = self.beam_count();
        for _ in 0..count {
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            self.spawn_beam(state, bounds, player_pos, angle);
        }
    }

    // accelerate, then hold and decelerate back to idle speed and transition to idle
    fn spinup_speed(elapsed: f32, hold: f32) -> f32 {
        let hold_end = BOSS_SPINUP_RAMP_UP + hold;
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

    // offset the semi-ring volley so the player can't just stay still
    fn aim_offset(volley: i32) -> f32 {
        let steps = BOSS_AIM_STEPS;
        // full cycle calculated with number of volleys
        let period = 4 * BOSS_AIM_STEPS;
        let q = volley % period;
        let tri = if q <= steps {
            q
        } else if q <= 3 * steps {
            2 * steps - q
        } else {
            q - 4 * steps
        };
        tri as f32 * BOSS_AIM_STEP
    }

    // semi-ring pattern fired downwards
    fn fire_ring(&self, state: &mut GameState, aim_offset: f32) {
        for i in 0..BOSS_PROJECTILE_COUNT {
            // angles in (0, PI) all point downward (+y under the y-down camera);
            // adding the offset sweeps each volley's volley left/right in a triangle wave
            let base = std::f32::consts::PI * (i as f32 + 0.5) / BOSS_PROJECTILE_COUNT as f32;
            let angle = base + aim_offset;
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

    // clustered semi rings for some variety
    // sweeps through the downward semicircle
    fn fire_clusters(&self, state: &mut GameState, sweep: f32) {
        // each cluster owns an equal slice of the semicircle
        let slice = std::f32::consts::PI / BOSS_CLUSTER_COUNT as f32;
        for c in 0..BOSS_CLUSTER_COUNT {
            for j in 0..BOSS_CLUSTER_SHOTS {
                // angles in (0, PI) all point downward (+y under the y-down camera)
                let angle = (c as f32 * slice + j as f32 * BOSS_CLUSTER_INTRA_GAP + sweep)
                    .rem_euclid(std::f32::consts::PI);
                let dir = vec2(angle.cos(), angle.sin());
                state
                    .projectiles
                    .push(Projectile::Bullet(BulletProjectile::new(
                        self.position,
                        dir * BOSS_SPECIAL_PROJECTILE_SPEED,
                        ProjectileKind::Boss,
                        BOSS_SPECIAL_PROJECTILE_COLOR,
                    )));
            }
        }
    }

    // spawn one full-length beam passing through the player along `angle`. the beam spans
    // the whole arena (overshooting the edges so the frame mask can hide the ends), and
    // because it goes through the player, standing still means getting hit
    fn spawn_beam(&self, state: &mut GameState, bounds: Rect, player_pos: Vec2, angle: f32) {
        let dir = Vec2::from_angle(angle);
        let (start, end) = Self::beam_span(player_pos, dir, bounds);
        state
            .projectiles
            .push(Projectile::Beam(BeamProjectile::new(start, end)));
    }

    // the segment where the line through `point` along `dir` crosses the arena, extended
    // past the edges by BEAM_EDGE_OVERSHOOT. the overshoot means the beam's flat end caps
    // land outside the arena, so they never leave a triangular gap at a shallow angle; the
    // overflow is painted over by the arena frame mask (see Arena::draw_frame_mask)
    fn beam_span(point: Vec2, dir: Vec2, bounds: Rect) -> (Vec2, Vec2) {
        let o = BEAM_EDGE_OVERSHOOT;
        let lo = vec2(bounds.x - o, bounds.y - o);
        let hi = vec2(bounds.x + bounds.w + o, bounds.y + bounds.h + o);

        // slab method: intersect the line with each axis-aligned pair of (outset) edges.
        // `point` is the player, always inside the arena, so a valid chord always exists
        let mut t_min = f32::NEG_INFINITY;
        let mut t_max = f32::INFINITY;
        for (p, d, edge_lo, edge_hi) in [(point.x, dir.x, lo.x, hi.x), (point.y, dir.y, lo.y, hi.y)]
        {
            if d.abs() > f32::EPSILON {
                let mut t0 = (edge_lo - p) / d;
                let mut t1 = (edge_hi - p) / d;
                if t0 > t1 {
                    std::mem::swap(&mut t0, &mut t1);
                }
                t_min = t_min.max(t0);
                t_max = t_max.min(t1);
            }
        }
        (point + dir * t_min, point + dir * t_max)
    }

    // current orientation in radians (used by collision checks)
    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    // (current, total) health, used to draw the boss health bar
    pub fn health(&self) -> (i32, i32) {
        (self.current_health, self.total_health)
    }

    // the trailing chip value while the drop is animating
    pub fn displayed_health(&self) -> f32 {
        self.displayed_health
    }

    pub fn is_invulnerable(&self) -> bool {
        matches!(
            self.state,
            BossState::Init(_)
                | BossState::SpecialMove(_)
                | BossState::DeathBurst(_)
                | BossState::Dying(_)
                | BossState::Dead
        )
    }

    pub fn is_dead(&self) -> bool {
        matches!(
            self.state,
            BossState::DeathBurst(_) | BossState::Dying(_) | BossState::Dead
        )
    }

    // chip drains smoothly towards current health
    fn update_displayed_health(&mut self, dt: f32) {
        let target = self.current_health as f32;
        let t = 1.0 - (-HEALTH_BAR_DROP_SPEED * dt).exp();
        self.displayed_health += (target - self.displayed_health) * t;
        // snap the last sliver so the chip doesn't linger forever
        if (self.displayed_health - target).abs() < 0.5 {
            self.displayed_health = target;
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.current_health = (self.current_health - damage).max(0);
    }

    pub fn draw(&self) {
        // once dead the boss is gone entirely
        if matches!(self.state, BossState::Dead) {
            return;
        }
        // draw the mask first then the boss
        self.mask.draw_rotated(self.position, self.rotation, 1.0);
        self.rect.draw_rotated(self.position, self.rotation, 1.0);

        // the expanding clear-out ring during the death burst
        if let BossState::DeathBurst(burst) = self.state {
            draw_circle_lines(
                self.position.x,
                self.position.y,
                burst.radius,
                BOSS_DEATH_BURST_THICKNESS,
                BOSS_DEATH_BURST_COLOR,
            );
        }
    }
}
