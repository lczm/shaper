use macroquad::prelude::*;

use crate::boss::Boss;
use crate::constants::PROJECTILE_RADIUS;
use crate::player::Player;
use crate::projectile::ProjectileKind;
use crate::state::GameState;

// circle vs circle collision check
// true when circle overlaps
fn circle_circle_overlap(a: Vec2, ar: f32, b: Vec2, br: f32) -> bool {
    a.distance_squared(b) <= (ar + br).powi(2)
}

// circle vs rotated box collision
// we check by undoing the rotation of the box
// then doing a circle vs AABB test
fn circle_box_overlap(c: Vec2, r: f32, center: Vec2, half: Vec2, rotation: f32) -> bool {
    let local = Vec2::from_angle(-rotation).rotate(c - center);
    let closest = local.clamp(-half, half);
    local.distance_squared(closest) <= r * r
}

// resolve all projectile hits for this frame
pub fn handle_collisions(state: &mut GameState, player: &Player, boss: &Boss) {
    let invulnerable = player.is_invulnerable();
    let player_pos = player.position;
    let player_r = player.circle.radius;
    let boss_pos = boss.position;
    let boss_half = boss.rect.size / 2.0;
    let boss_rot = boss.rotation();

    let mut lives_lost: u32 = 0;
    state.projectiles.retain(|p| match p.kind {
        // boss bullet hits the player
        // for now, lose a life except when iframing on phasing, consume the bullet
        ProjectileKind::Boss => {
            if !invulnerable
                && circle_circle_overlap(p.position, PROJECTILE_RADIUS, player_pos, player_r)
            {
                lives_lost += 1;
                false
            } else {
                true
            }
        }
        // when player projectile hit the boss
        // just consume it for now,
        // TODO : add boss health and damage later
        ProjectileKind::Player => {
            !circle_box_overlap(p.position, PROJECTILE_RADIUS, boss_pos, boss_half, boss_rot)
        }
    });
    state.lives = state.lives.saturating_sub(lives_lost);
}
