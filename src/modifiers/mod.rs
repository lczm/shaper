use macroquad::prelude::*;

pub mod generator;
pub use generator::ModifiersGenerator;

use crate::{
    constants::{
        ARENA_BORDER_THICKNESS, BOUNCING_PROJECTILE_COLOR, HOMING_PROJECTILE_COLOR,
        HOMING_TURN_SPEED, LIGHTNING_DAMAGE_MULTIPLIER, LIGHTNING_PROJECTILE_COLOR,
    },
    projectile::ProjectileKind,
};

pub struct ModifierContext {
    pub arena_bounds: Rect,
    pub enemy_positions: Vec<Vec2>,
    pub player_position: Vec2,
}

#[derive(Clone, Copy)]
pub enum SecondaryHitKind {
    Lightning,
}

#[derive(Clone, Copy)]
pub struct SecondaryHit {
    pub position: Vec2,
    pub damage: i32,
    pub kind: SecondaryHitKind,
}

pub struct HitResult {
    pub destroy: bool,
    pub extra_damage: i32,
    pub secondary_hits: Vec<SecondaryHit>,
}

impl Default for HitResult {
    fn default() -> Self {
        Self {
            destroy: true,
            extra_damage: 0,
            secondary_hits: Vec::new(),
        }
    }
}

pub struct ModifierState {
    pub original_direction: Vec2,
    pub elapsed_time: f32,
    pub pierce_count: i32,
    pub bounce_count: i32,
    pub dna_phase: f32,
}

impl Default for ModifierState {
    fn default() -> Self {
        Self {
            original_direction: Vec2::ZERO,
            elapsed_time: 0.0,
            pierce_count: 0,
            bounce_count: 0,
            dna_phase: 0.0,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Modifier {
    None,
    Homing,
    Bouncing,
    Lightning,
    Dna,
    TripleShot,

    // stat upgrades
    DamageBoost(i32),
    FireRateBoost(u32),
    BombBoost,
}

pub mod homing {
    use super::*;
    pub fn on_spawn(
        state: &mut ModifierState,
        velocity: &mut Vec2,
        circle: &mut crate::shape::Circle,
    ) {
        state.original_direction = velocity.normalize_or_zero();
        circle.color = HOMING_PROJECTILE_COLOR;
    }

    pub fn on_update(position: &mut Vec2, velocity: &mut Vec2, dt: f32, ctx: &ModifierContext) {
        let closest = ctx.enemy_positions.iter().min_by(|a, b| {
            let da = position.distance_squared(**a);
            let db = position.distance_squared(**b);
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        });

        let to_target = if let Some(&target) = closest {
            (target - *position).normalize_or_zero()
        } else {
            vec2(0.0, -1.0)
        };

        let speed = velocity.length();
        let current_dir = velocity.normalize_or_zero();

        let max_angle = (HOMING_TURN_SPEED * dt).abs();
        let desired_angle = current_dir.angle_between(to_target);
        let clamped = desired_angle.clamp(-max_angle, max_angle);

        let new_dir = Vec2::from_angle(current_dir.to_angle() + clamped);
        *velocity = new_dir * speed;
    }
}

pub mod bouncing {
    use super::*;
    pub fn on_spawn(state: &mut ModifierState, circle: &mut crate::shape::Circle) {
        state.bounce_count = 1;
        circle.color = BOUNCING_PROJECTILE_COLOR;
    }

    pub fn on_update(
        state: &mut ModifierState,
        position: &mut Vec2,
        velocity: &mut Vec2,
        circle: &crate::shape::Circle,
        ctx: &ModifierContext,
    ) {
        if state.bounce_count > 0 {
            let r = circle.radius;
            let inset = ARENA_BORDER_THICKNESS / 2.0;
            let bounds = ctx.arena_bounds;
            let min_x = bounds.x + inset + r;
            let max_x = bounds.x + bounds.w - inset - r;
            let min_y = bounds.y + inset + r;
            let max_y = bounds.y + bounds.h - inset - r;

            let mut bounced = false;
            if position.x < min_x {
                position.x = min_x;
                velocity.x = -velocity.x;
                bounced = true;
            } else if position.x > max_x {
                position.x = max_x;
                velocity.x = -velocity.x;
                bounced = true;
            }

            if position.y < min_y {
                position.y = min_y;
                velocity.y = -velocity.y;
                bounced = true;
            } else if position.y > max_y {
                position.y = max_y;
                velocity.y = -velocity.y;
                bounced = true;
            }

            if bounced {
                let speed = velocity.length();
                let angle = velocity.to_angle();
                let random_offset = macroquad::rand::gen_range(-0.175, 0.175);
                *velocity = Vec2::from_angle(angle + random_offset) * speed;

                state.bounce_count -= 1;
            }
        }
    }
}

pub mod lightning {
    use super::*;
    pub fn on_spawn(circle: &mut crate::shape::Circle) {
        circle.color = LIGHTNING_PROJECTILE_COLOR;
    }

    pub fn on_hit(kind: &ProjectileKind, ctx: &ModifierContext) -> HitResult {
        let damage = match kind {
            ProjectileKind::Player { damage } => *damage,
            _ => 0,
        };
        let lightning_damage = ((damage as f32) * LIGHTNING_DAMAGE_MULTIPLIER).max(1.0) as i32;

        let mut secondary_hits = Vec::new();
        if let Some(&primary_target) = ctx.enemy_positions.first() {
            if ctx.enemy_positions.len() > 1 {
                for &pos in ctx.enemy_positions.iter().take(3) {
                    if secondary_hits.len() < 2 {
                        secondary_hits.push(SecondaryHit {
                            position: pos,
                            damage: lightning_damage,
                            kind: SecondaryHitKind::Lightning,
                        });
                    }
                }
            } else {
                let offset1 = vec2(
                    macroquad::rand::gen_range(-40.0, -15.0),
                    macroquad::rand::gen_range(-20.0, 20.0),
                );
                let offset2 = vec2(
                    macroquad::rand::gen_range(15.0, 40.0),
                    macroquad::rand::gen_range(-20.0, 20.0),
                );
                secondary_hits.push(SecondaryHit {
                    position: primary_target + offset1,
                    damage: lightning_damage,
                    kind: SecondaryHitKind::Lightning,
                });
                secondary_hits.push(SecondaryHit {
                    position: primary_target + offset2,
                    damage: lightning_damage,
                    kind: SecondaryHitKind::Lightning,
                });
            }
        }

        HitResult {
            destroy: true,
            extra_damage: 0,
            secondary_hits,
        }
    }
}

pub mod dna {
    use super::*;
    pub fn on_spawn(state: &ModifierState, circle: &mut crate::shape::Circle) {
        if (state.dna_phase - 0.0).abs() < 0.01 {
            circle.color = crate::constants::DNA_PROJECTILE_COLOR_1;
        } else {
            circle.color = crate::constants::DNA_PROJECTILE_COLOR_2;
        }
    }

    pub fn on_update(state: &mut ModifierState, position: &mut Vec2, velocity: &Vec2, dt: f32) {
        let speed = velocity.length();
        if speed > 0.0 {
            let dir = velocity.normalize_or_zero();
            let perp = vec2(-dir.y, dir.x);
            state.elapsed_time += dt;
            let frequency = 12.0;
            let amplitude = 250.0;
            let phase = state.dna_phase;
            let wave_vel = perp * (amplitude * (frequency * state.elapsed_time + phase).cos());
            *position += wave_vel * dt;
        }
    }
}

impl Modifier {
    pub fn on_spawn(
        &self,
        state: &mut ModifierState,
        position: &mut Vec2,
        velocity: &mut Vec2,
        circle: &mut crate::shape::Circle,
    ) {
        match self {
            Modifier::None => {}
            Modifier::Homing => homing::on_spawn(state, velocity, circle),
            Modifier::Bouncing => bouncing::on_spawn(state, circle),
            Modifier::Lightning => lightning::on_spawn(circle),
            Modifier::Dna => dna::on_spawn(state, circle),
            Modifier::TripleShot => {}
            Modifier::DamageBoost(_) | Modifier::FireRateBoost(_) | Modifier::BombBoost => {}
        }
    }

    pub fn on_update(
        &self,
        state: &mut ModifierState,
        position: &mut Vec2,
        velocity: &mut Vec2,
        circle: &mut crate::shape::Circle,
        dt: f32,
        ctx: &ModifierContext,
    ) {
        match self {
            Modifier::None => {}
            Modifier::Homing => homing::on_update(position, velocity, dt, ctx),
            Modifier::Bouncing => bouncing::on_update(state, position, velocity, circle, ctx),
            Modifier::Lightning => {}
            Modifier::Dna => dna::on_update(state, position, velocity, dt),
            Modifier::TripleShot => {}
            Modifier::DamageBoost(_) | Modifier::FireRateBoost(_) | Modifier::BombBoost => {}
        }
    }

    pub fn on_hit(
        &self,
        state: &mut ModifierState,
        position: &Vec2,
        velocity: &Vec2,
        kind: &ProjectileKind,
        context: &ModifierContext,
    ) -> HitResult {
        match self {
            Modifier::None => HitResult::default(),
            Modifier::Homing => HitResult::default(),
            Modifier::Bouncing => HitResult::default(),
            Modifier::Lightning => lightning::on_hit(kind, context),
            Modifier::Dna => HitResult::default(),
            Modifier::TripleShot => HitResult::default(),
            Modifier::DamageBoost(_) | Modifier::FireRateBoost(_) | Modifier::BombBoost => {
                HitResult::default()
            }
        }
    }

    pub fn name(&self) -> String {
        match self {
            Modifier::None => "Placeholder".to_string(),
            Modifier::Homing => "Homing".to_string(),
            Modifier::Bouncing => "Bouncing".to_string(),
            Modifier::Lightning => "Chain Lightning".to_string(),
            Modifier::Dna => "DNA".to_string(),
            Modifier::TripleShot => "Triple Shot".to_string(),
            Modifier::DamageBoost(_) => "Damage Boost".to_string(),
            Modifier::FireRateBoost(_) => "Fire Rate Boost".to_string(),
            Modifier::BombBoost => "Bomb Boost".to_string(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            Modifier::None => "No effect.".to_string(),
            Modifier::Homing => "Projectiles steer toward the nearest enemy.".to_string(),
            Modifier::Bouncing => {
                "Projectiles can bounce once off the arena bounds. When it bounces, it is a little random!".to_string()
            }
            Modifier::Lightning => {
                "Projectiles release chain lightning on hit, dealing 30% damage to up to 2 nearby targets.".to_string()
            }
            Modifier::Dna => {
                "Fires 2 projectiles in opposite sine waves, forming a double helix pattern.".to_string()
            }
            Modifier::TripleShot => {
                "Triples the number of projectiles fired per shot.".to_string()
            }
            Modifier::DamageBoost(damage_boost) => {
                format!("Increases flat damage by +{damage_boost}.")
            }
            Modifier::FireRateBoost(fire_rate_boost) => {
                format!("Increases fire rate by +{fire_rate_boost}%.")
            }
            Modifier::BombBoost => {
                "Adds +1 Bomb and increases bomb radius by 20%.".to_string()
            }
        }
    }

    pub fn damage_contribution(&self, base_damage: i32) -> (i32, i32) {
        match self {
            Modifier::Lightning => {
                let bonus =
                    ((base_damage as f32) * LIGHTNING_DAMAGE_MULTIPLIER).max(1.0) as i32 * 2;
                (bonus, 1)
            }
            Modifier::Dna => (0, 2),
            Modifier::TripleShot => (0, 3),
            _ => (0, 1),
        }
    }
}
