use macroquad::prelude::*;

use crate::boss::Boss;
use crate::constants::{BEAM_WIDTH, PROJECTILE_RADIUS};
use crate::modifiers::ModifierContext;
use crate::player::Player;
use crate::projectile::{Projectile, ProjectileKind};
use crate::state::GameState;
use crate::world::GameEvent;

// circle vs circle collision check
// true when circle overlaps
pub fn circle_circle_overlap(a: Vec2, ar: f32, b: Vec2, br: f32) -> bool {
    a.distance_squared(b) <= (ar + br).powi(2)
}

// circle vs rotated box collision
// we check by undoing the rotation of the box
// then doing a circle vs AABB test
pub fn circle_box_overlap(c: Vec2, r: f32, center: Vec2, half: Vec2, rotation: f32) -> bool {
    let local = Vec2::from_angle(-rotation).rotate(c - center);
    let closest = local.clamp(-half, half);
    local.distance_squared(closest) <= r * r
}

// circle vs thick segment (capsule) collision
// closest point on segment [a, b] to c, compared against the combined radius
pub fn segment_circle_overlap(a: Vec2, b: Vec2, half_width: f32, c: Vec2, r: f32) -> bool {
    let ab = b - a;
    let len_sq = ab.length_squared();
    let t = if len_sq > 0.0 {
        ((c - a).dot(ab) / len_sq).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let closest = a + ab * t;
    c.distance_squared(closest) <= (half_width + r).powi(2)
}

// resolve all projectile hits for this frame.
// only detects hits and emits a GameEvent::PlayerHit; the handler applies the
// consequences (lose a life, start i-frames, screen shake) in one place, so
// this function stays read-only over the player.
pub fn handle_collisions(
    state: &mut GameState,
    player: &Player,
    boss: &Boss,
    bounds: Rect,
    events: &mut Vec<GameEvent>,
) {
    // pull out what the retain closure needs so it doesn't borrow `player`
    let player_invulnerable = player.is_invulnerable();
    let boss_invulnerable = boss.is_invulnerable();
    let player_pos = player.position;
    let player_r = player.circle.radius;
    let boss_pos = boss.position;
    let boss_half = boss.rect.size / 2.0;
    let boss_rot = boss.rotation();
    let mut player_hit = false;
    // there can be more than 1 bullet that hits the boss,
    // so accumulate all boss projectile damages
    let mut boss_damage = 0;
    state.projectiles.retain_mut(|p| match p {
        // deal with bullet projectiles
        Projectile::Bullet(b) => match b.kind {
            // boss bullet hits the player: lose a life unless i-framed, consume the bullet
            ProjectileKind::Boss => {
                if !player_invulnerable
                    && circle_circle_overlap(b.position, PROJECTILE_RADIUS, player_pos, player_r)
                {
                    player_hit = true;
                    false
                } else {
                    true
                }
            }
            // player bullet hits the boss: deal its damage, run modifier hooks, and
            // destroy unless a modifier says otherwise
            ProjectileKind::Player { damage } => {
                if !boss_invulnerable
                    && circle_box_overlap(
                        b.position,
                        PROJECTILE_RADIUS,
                        boss_pos,
                        boss_half,
                        boss_rot,
                    )
                {
                    let mut should_destroy = true;
                    let mut bonus = 0;

                    let modifiers = std::mem::take(&mut b.modifiers);
                    let mut modifier_state = std::mem::take(&mut b.modifier_state);
                    let ctx = ModifierContext {
                        arena_bounds: bounds,
                        enemy_positions: vec![boss_pos],
                        player_position: player_pos,
                    };

                    for modifier in &modifiers {
                        let result = modifier.on_hit(b, &mut modifier_state, &ctx);
                        if !result.destroy {
                            should_destroy = false;
                        }
                        bonus += result.extra_damage;
                    }
                    b.modifiers = modifiers;
                    b.modifier_state = modifier_state;

                    boss_damage += damage + bonus;

                    // keep the bullet if any modifier said don't destroy
                    !should_destroy
                } else {
                    true
                }
            }
        },
        // beam hits the player: only once fully activated (the telegraph is harmless), and
        // unless i-framed. the beam persists either way.
        Projectile::Beam(beam) => {
            if !player_invulnerable
                && beam.is_active()
                && segment_circle_overlap(
                    beam.start,
                    beam.end,
                    BEAM_WIDTH / 2.0,
                    player_pos,
                    player_r,
                )
            {
                player_hit = true;
            }
            true
        }
    });

    if player_hit {
        events.push(GameEvent::PlayerHit);
    }

    if boss_damage > 0 {
        events.push(GameEvent::BossHit {
            damage: boss_damage,
        });
    }
}

pub fn point_in_rect(p: Vec2, r: Rect) -> bool {
    p.x >= r.x && p.x <= r.x + r.w && p.y >= r.y && p.y <= r.y + r.h
}
