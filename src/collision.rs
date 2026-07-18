use macroquad::prelude::*;

use crate::boss::Boss;
use crate::constants::{BEAM_WIDTH, PROJECTILE_RADIUS, PROTO_RADIUS};
use crate::modifiers::{ModifierContext, SecondaryHitKind};
use crate::player::Player;
use crate::projectile::{Projectile, ProjectileKind};
use crate::proto::Proto;
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
    boss: &mut Boss,
    protos: &mut [Proto],
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
    // unless iframedd, player colliding with the boss is also a hit
    if !player_invulnerable
        && circle_box_overlap(player_pos, player_r, boss_pos, boss_half, boss_rot)
    {
        player_hit = true;
    }

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
            // player bullet hits the (boss or protos) / enemies
            // deal its damage, run modifier hooks, and
            // destroy unless a modifier says otherwise
            ProjectileKind::Player { damage } => {
                let mut hit_boss = false;
                if !boss_invulnerable
                    && circle_box_overlap(
                        b.position,
                        PROJECTILE_RADIUS,
                        boss_pos,
                        boss_half,
                        boss_rot,
                    )
                {
                    hit_boss = true;
                }

                let mut hit_proto_idx = None;
                for (idx, proto) in protos.iter().enumerate() {
                    if !proto.is_invulnerable()
                        && !proto.is_dead()
                        && circle_circle_overlap(
                            b.position,
                            PROJECTILE_RADIUS,
                            proto.position,
                            PROTO_RADIUS,
                        )
                    {
                        hit_proto_idx = Some(idx);
                        break;
                    }
                }

                if hit_boss || hit_proto_idx.is_some() {
                    let mut should_destroy = true;
                    let mut bonus = 0;

                    let mut enemy_positions = vec![boss_pos];
                    for proto in protos.iter() {
                        if !proto.is_dead() {
                            enemy_positions.push(proto.position);
                        }
                    }

                    let ctx = ModifierContext {
                        arena_bounds: bounds,
                        enemy_positions,
                        player_position: player_pos,
                    };

                    let mut secondary_hits = Vec::new();
                    for modifier in &b.modifiers {
                        let mut result = modifier.on_hit(
                            &mut b.modifier_state,
                            &b.position,
                            &b.velocity,
                            &b.kind,
                            &ctx,
                        );
                        if !result.destroy {
                            should_destroy = false;
                        }
                        bonus += result.extra_damage;
                        secondary_hits.append(&mut result.secondary_hits);
                    }

                    let total_damage = damage + bonus;
                    if hit_boss {
                        boss_damage += total_damage;
                    } else if let Some(idx) = hit_proto_idx {
                        protos[idx].take_damage(total_damage);
                    }

                    for hit in secondary_hits {
                        let mut closest_idx = None;
                        let mut closest_dist = f32::INFINITY;

                        // check boss
                        let boss_dist = hit.position.distance_squared(boss_pos);
                        if !boss_invulnerable && boss_dist < closest_dist {
                            closest_dist = boss_dist;
                            closest_idx = Some(0);
                        }

                        // check protos
                        for (p_idx, proto) in protos.iter().enumerate() {
                            if !proto.is_invulnerable() && !proto.is_dead() {
                                let d = hit.position.distance_squared(proto.position);
                                if d < closest_dist {
                                    closest_dist = d;
                                    closest_idx = Some(p_idx + 1);
                                }
                            }
                        }

                        if let Some(target) = closest_idx {
                            if target == 0 {
                                boss_damage += hit.damage;
                            } else {
                                protos[target - 1].take_damage(hit.damage);
                            }
                        }

                        let visual_effect = match hit.kind {
                            SecondaryHitKind::Lightning => crate::world::VisualEffect::Lightning {
                                start: b.position,
                                end: hit.position,
                            },
                        };
                        events.push(GameEvent::TriggerVisualEffect(visual_effect));
                    }

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
