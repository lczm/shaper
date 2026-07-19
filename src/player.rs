use macroquad::prelude::*;

use crate::constants::{
    ARENA_BORDER_THICKNESS, BOMB_RADIUS, HIT_INVULN_DURATION, PHASE_DISTANCE, PHASE_DURATION,
    PHASE_GHOST_OPACITY, PHASE_MIN_OPACITY, PHASE_TRAIL_LENGTH, PLAYER_CIRCLE_RADIUS, PLAYER_COLOR,
    PLAYER_DEV_BOMBS, PLAYER_DEV_CHEAT_FIRE_INTERVAL, PLAYER_FIRE_INTERVAL, PLAYER_PHASING_COLOR,
    PLAYER_PROJECTILE_COLOR, PLAYER_PROJECTILE_SPEED, PLAYER_SPEED, PLAYER_TRAIL_COLOR,
};
use crate::input::Input;
use crate::projectile::{BulletProjectile, Projectile, ProjectileKind};
use crate::recipe::ProjectileRecipe;
use crate::shape::Circle;
use crate::state::GameState;
use crate::world::GameEvent;

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
    // interval the fire timer resets to after each shot
    fire_interval: f32,
    // post-hit invulnerability window; counts down to 0
    hit_cooldown: f32,
    damage: i32,
    // apply modifiers on each bullet when projectile is spawned
    pub projectile_recipe: ProjectileRecipe,
    pub bomb_radius: f32,
}

impl Player {
    pub fn new(position: Vec2) -> Self {
        Player {
            position,
            circle: Circle::new(PLAYER_CIRCLE_RADIUS, PLAYER_COLOR),
            state: PlayerState::Normal,
            trail: Vec::new(),
            fire_timer: 0.0,
            fire_interval: PLAYER_FIRE_INTERVAL,
            hit_cooldown: 0.0,
            damage: 5,
            projectile_recipe: ProjectileRecipe::new(),
            bomb_radius: BOMB_RADIUS,
        }
    }

    pub fn damage(&self) -> i32 {
        self.damage
    }

    pub fn fire_interval(&self) -> f32 {
        self.fire_interval
    }

    pub fn add_damage(&mut self, damage_boost: i32) {
        self.damage += damage_boost;
    }

    pub fn increase_fire_rate(&mut self, fire_rate_boost: u32) {
        self.fire_interval /= 1.0 + (fire_rate_boost as f32) / 100.0;
    }

    pub fn potential_damage(&self) -> i32 {
        self.projectile_recipe.potential_damage(self.damage)
    }

    pub fn calculate_movement_vector(input: &Input) -> Vec2 {
        let mut direction = Vec2::ZERO;
        if input.arrow_up || input.w_down {
            direction.y -= 1.0;
        }
        if input.arrow_down || input.s_down {
            direction.y += 1.0;
        }
        if input.arrow_left || input.a_down {
            direction.x -= 1.0;
        }
        if input.arrow_right || input.d_down {
            direction.x += 1.0;
        }
        // normalize so diagonal movement isn't faster than axis-aligned
        if direction != Vec2::ZERO {
            direction = direction.normalize();
        }
        direction
    }

    pub fn update(
        &mut self,
        dt: f32,
        input: &Input,
        bounds: Rect,
        top_boundary_y: Option<f32>,
        state: &mut GameState,
        events: &mut Vec<GameEvent>,
    ) {
        // detonate a bomb on key press if any are left
        // push an event that the event handler will deal with later
        if (input.z_pressed || input.slash_pressed) && state.bombs > 0 {
            events.push(GameEvent::BombDetonated {
                position: self.position,
            });
        }

        // tick down invulnerability window
        if self.hit_cooldown > 0.0 {
            self.hit_cooldown -= dt;
        }

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

        self.clamp_position(bounds, top_boundary_y);

        // continuously fire
        // this is the start point of all the projectiles that the player fires
        self.fire(dt, state);
    }

    fn clamp_position(&mut self, bounds: Rect, top_boundary_y: Option<f32>) {
        // keep the whole player circle inside the arena border.
        let shape_inset = self.circle.radius + self.circle.thickness / 2.0;
        let arena_inset = shape_inset + ARENA_BORDER_THICKNESS / 2.0;
        let min_x = bounds.x + arena_inset;
        let max_x = bounds.x + bounds.w - arena_inset;
        let arena_min_y = bounds.y + arena_inset;
        let max_y = bounds.y + bounds.h - arena_inset;

        // during the fight, the boss includes an additional horizontal boundary.
        // include the player's outline so no part of it can cross above that line.
        let min_y = top_boundary_y
            .map(|boundary_y| arena_min_y.max(boundary_y + shape_inset))
            .unwrap_or(arena_min_y)
            .min(max_y);

        self.position.x = self.position.x.clamp(min_x, max_x);
        self.position.y = self.position.y.clamp(min_y, max_y);
    }

    // continuously shoot straight up on a fixed cadence
    fn fire(&mut self, dt: f32, state: &mut GameState) {
        self.fire_timer -= dt;
        if self.fire_timer <= 0.0 {
            // keep the overshoot remainder so cadence doesnt drift
            self.fire_timer += self.fire_interval;

            let spawns = self.projectile_recipe.spawn_count();

            for i in 0..spawns {
                // continuously spawn new bullet projectile from the player position upwards
                let mut bullet = BulletProjectile::new(
                    self.position,
                    vec2(0.0, -PLAYER_PROJECTILE_SPEED),
                    ProjectileKind::Player {
                        damage: self.damage,
                    },
                    PLAYER_PROJECTILE_COLOR,
                );

                // apply all accumulated modifiers from the recipe
                let (modifiers, modifier_state) = self.projectile_recipe.apply(&mut bullet, i);
                bullet.modifiers = modifiers;
                bullet.modifier_state = modifier_state;
                state.projectiles.push(Projectile::Bullet(bullet));
            }
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
            // pale ghost tint while phasing
            self.circle.color = PLAYER_PHASING_COLOR;
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
            self.circle.color = PLAYER_COLOR;
        } else {
            self.state = PlayerState::Phasing { direction, elapsed };
        }
    }

    // true while phasing or during the post-hit invulnerability window
    pub fn is_invulnerable(&self) -> bool {
        matches!(self.state, PlayerState::Phasing { .. }) || self.hit_cooldown > 0.0
    }

    // start the invulnerability window after taking a hit
    pub fn register_hit(&mut self) {
        self.hit_cooldown = HIT_INVULN_DURATION;
    }

    // extend the invulnerability window (used by the bomb), never shortening an
    // already-longer cooldown
    pub fn grant_invulnerability(&mut self, duration: f32) {
        self.hit_cooldown = self.hit_cooldown.max(duration);
    }

    pub fn dev_damage_boost(&mut self, state: &mut GameState) {
        self.fire_interval = PLAYER_DEV_CHEAT_FIRE_INTERVAL;
        self.fire_timer = self.fire_interval;
        state.bombs = PLAYER_DEV_BOMBS;
    }

    pub fn dev_give_all_modifiers(&mut self, state: &mut GameState) {
        // give myself all modifiers available in the modifiers generator, and remove them from the pool
        state
            .modifiers_generator
            .available
            .iter()
            .for_each(|modifier| {
                self.projectile_recipe.add_modifier(modifier.clone());
            });
        state.modifiers_generator.available.clear();
    }

    pub fn draw(&self) {
        // ghost trail behind the phase movement: older ghosts fade out
        let trail_len = self.trail.len() as f32;
        for (i, &pos) in self.trail.iter().enumerate() {
            let fade = 1.0 - i as f32 / trail_len;
            self.circle
                .draw_colored(pos, PLAYER_TRAIL_COLOR, PHASE_GHOST_OPACITY * fade);
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
