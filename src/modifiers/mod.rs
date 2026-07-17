use macroquad::prelude::*;

use crate::{
    constants::{HOMING_PROJECTILE_COLOR, HOMING_TURN_SPEED},
    projectile::BulletProjectile,
};

// some context for the projectile modifiers to query the state
// of the world, so they can do various modifiers to each of the projectiles
pub struct ModifierContext {
    pub arena_bounds: Rect,
    // all current enemy positions — homing picks the closest one
    pub enemy_positions: Vec<Vec2>,
    pub player_position: Vec2,
}

// what happens after a player projectile hits the boss
// if there are multiple modifiers on the same projectile and
// they are conflicting, then they get merged
pub struct HitResult {
    // if any of the modifiers says false to destroy, then the projectile survives
    pub destroy: bool,
    // sum up extra damage from all modifiers that apply extra damage
    pub extra_damage: i32,
}

impl Default for HitResult {
    fn default() -> Self {
        Self {
            destroy: true,
            extra_damage: 0,
        }
    }
}

pub struct ModifierState {
    pub original_direction: Vec2,
    pub elapsed_time: f32,

    // todo for piercing
    pub pierce_count: i32,
    // todo for bouncing around the arena
    pub bounce_count: i32,
}

impl Default for ModifierState {
    fn default() -> Self {
        Self {
            original_direction: Vec2::ZERO,
            elapsed_time: 0.0,
            pierce_count: 0,
            bounce_count: 0,
        }
    }
}

#[derive(Clone)]
pub enum Modifier {
    // no op for placeholder
    None,
    // steers towards the closest enemy
    Homing,
}

impl Modifier {
    // called on creation
    pub fn on_spawn(&self, projectile: &mut BulletProjectile, state: &mut ModifierState) {
        match self {
            Modifier::None => {}
            Modifier::Homing => {
                // record the original firing direction so we know the bullet's
                // intended heading even after we start bending it
                state.original_direction = projectile.velocity.normalize_or_zero();
                projectile.circle.color = HOMING_PROJECTILE_COLOR;
            }
        }
    }

    pub fn on_update(
        &self,
        projectile: &mut BulletProjectile,
        state: &mut ModifierState,
        dt: f32,
        ctx: &ModifierContext,
    ) {
        match self {
            Modifier::None => {}
            Modifier::Homing => {
                // find the closest enemy to the projectile
                let closest = ctx.enemy_positions.iter().min_by(|a, b| {
                    let da = projectile.position.distance_squared(**a);
                    let db = projectile.position.distance_squared(**b);
                    da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
                });

                if let Some(&target) = closest {
                    let to_target = (target - projectile.position).normalize_or_zero();
                    let speed = projectile.velocity.length();
                    let current_dir = projectile.velocity.normalize_or_zero();

                    // smoothly rotate toward the target by clamping the
                    // turn angle per frame
                    let max_angle = (HOMING_TURN_SPEED * dt).abs();
                    let desired_angle = current_dir.angle_between(to_target);
                    let clamped = desired_angle.clamp(-max_angle, max_angle);

                    let new_dir = Vec2::from_angle(current_dir.to_angle() + clamped);
                    projectile.velocity = new_dir * speed;
                } else {
                    // if no target (boss is dead), steer straight up
                    let to_target = vec2(0.0, -1.0);
                    let speed = projectile.velocity.length();
                    let current_dir = projectile.velocity.normalize_or_zero();

                    let max_angle = (HOMING_TURN_SPEED * dt).abs();
                    let desired_angle = current_dir.angle_between(to_target);
                    let clamped = desired_angle.clamp(-max_angle, max_angle);

                    let new_dir = Vec2::from_angle(current_dir.to_angle() + clamped);
                    projectile.velocity = new_dir * speed;
                }
            }
        }
    }

    pub fn on_hit(
        &self,
        projectile: &mut BulletProjectile,
        state: &mut ModifierState,
        ctx: &ModifierContext,
    ) -> HitResult {
        match self {
            Modifier::None => HitResult::default(),
            Modifier::Homing => HitResult::default(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Modifier::None => "Placeholder",
            Modifier::Homing => "Homing",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Modifier::None => "No effect.",
            Modifier::Homing => "Projectiles steer toward the nearest enemy.",
        }
    }
}
