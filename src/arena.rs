use macroquad::prelude::*;

use crate::bomb::Bomb;
use crate::boss::Boss;
use crate::collision::handle_collisions;
use crate::constants::{
    ARENA_BORDER_COLOR, ARENA_BORDER_THICKNESS, ARENA_MARGIN_HEIGHT, ARENA_MARGIN_WIDTH,
    BACKGROUND, BOMB_DURATION, FRAME_MASK_PAD, HEIGHT, PROTO_MAX_SLOTS, PROTO_RADIUS,
    PROTO_SPAWN_MAX_INTERVAL, PROTO_SPAWN_MIN_INTERVAL, PROTO_SPAWN_OFFSET_X,
};
use crate::gfx::Shaders;
use crate::input::Input;
use crate::modifiers::ModifierContext;
use crate::player::Player;
use crate::projectile::{BeamProjectile, Projectile};
use crate::proto::Proto;
use crate::proto_beam::{ProtoBeam, ProtoBeamState};
use crate::state::GameState;
use crate::world::GameEvent;

// the gameplay arena and everything inside it
pub struct Arena {
    bounds: Rect,
    player: Player,
    boss: Boss,
    pub protos: Vec<Proto>,
    proto_spawn_timer: f32,
    // active clearing blast, if one is currently going off
    bomb: Option<Bomb>,
    pub proto_beams: Vec<ProtoBeam>,
}

impl Arena {
    pub fn new() -> Self {
        // offset the arena by some margin; portrait 3:4 rect anchored top-left
        let height = HEIGHT - 2.0 * ARENA_MARGIN_HEIGHT;
        let width = height * 3.0 / 4.0;
        let bounds = Rect::new(ARENA_MARGIN_WIDTH, ARENA_MARGIN_HEIGHT, width, height);

        let center_x = bounds.x + bounds.w / 2.0;
        let boss_pos = vec2(center_x, bounds.y + bounds.h / 5.0);

        // Spawn left proto at slot 4 (angle PI, left) and right proto at slot 0 (angle 0, right)
        let left_proto = Proto::new(
            boss_pos - vec2(PROTO_SPAWN_OFFSET_X, 0.0),
            PROTO_MAX_SLOTS - 1,
            0,
        );
        let right_proto = Proto::new(boss_pos + vec2(PROTO_SPAWN_OFFSET_X, 0.0), 0, 0);

        let proto_spawn_timer = rand::gen_range(PROTO_SPAWN_MIN_INTERVAL, PROTO_SPAWN_MAX_INTERVAL);

        Arena {
            bounds,
            // player near the bottom-center, boss near the top-center
            player: Player::new(vec2(center_x, bounds.y + bounds.h * 4.0 / 5.0)),
            boss: Boss::new(boss_pos),
            protos: vec![left_proto, right_proto],
            proto_spawn_timer,
            bomb: None,
            proto_beams: Vec::new(),
        }
    }

    // the rectangular gameplay border, in logical coordinates
    pub fn bounds(&self) -> Rect {
        self.bounds
    }

    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    // (current, total) boss health for the hud health bar
    pub fn boss_health(&self) -> (i32, i32) {
        self.boss.health()
    }

    // trailing chip value
    pub fn boss_displayed_health(&self) -> f32 {
        self.boss.displayed_health()
    }

    // used by hud to show the invulnerable fill colour on health bar
    pub fn boss_invulnerable(&self) -> bool {
        self.boss.is_invulnerable()
    }

    // todo : can used to check when there are more enemies
    pub fn alive_enemy_count(&self) -> (usize, Vec<Vec2>) {
        let mut count = 0;
        let mut positions = Vec::new();

        if !self.boss.is_dead() {
            count += 1;
            positions.push(self.boss.position);
        }
        for proto in &self.protos {
            if !proto.is_dead() {
                count += 1;
                positions.push(proto.position);
            }
        }
        (count, positions)
    }

    pub fn player_damage(&self) -> i32 {
        self.player.potential_damage()
    }

    pub fn damage_boss(&mut self, damage: i32) {
        self.boss.take_damage(damage);
    }

    pub fn dev_force_boss_transition_75(&mut self) {
        self.boss.force_transition_75();
    }

    pub fn dev_force_boss_transition_50(&mut self) {
        self.boss.force_transition_50();
    }

    pub fn dev_force_boss_transition_25(&mut self) {
        self.boss.force_transition_25();
    }

    // set bomb at position (player center) and grant iframes
    pub fn detonate_bomb(&mut self, position: Vec2) {
        self.bomb = Some(Bomb::new(position, self.player.bomb_radius));
        self.player.grant_invulnerability(BOMB_DURATION);
    }

    pub fn update(
        &mut self,
        dt: f32,
        input: &Input,
        state: &mut GameState,
        events: &mut Vec<GameEvent>,
    ) {
        if input.f1_pressed {
            self.player.dev_damage_boost(state);
        }

        if input.f2_pressed {
            self.player.dev_give_all_modifiers(state);
        }

        let player_top_boundary = self.boss.player_boundary_y();
        self.player
            .update(dt, input, self.bounds, player_top_boundary, state, events);

        // boss may push some projectiles into the game state; it aims beams at the player
        self.boss
            .update(dt, state, self.bounds, self.player.position, events);

        // when boss dies, kill of all the protos
        if self.boss.is_dead() {
            for proto in &mut self.protos {
                proto.kill();
            }
        }

        // update any protos that exist
        for proto in &mut self.protos {
            proto.update(dt, state, self.player.position);
        }
        // unless its dead
        self.protos.retain(|proto| !proto.is_fully_dead());

        // manage spawn and death of proto_beam pairs
        let transition_active = self.boss.is_in_transition();
        if transition_active {
            if self.proto_beams.is_empty() {
                // spawn the left and right proto beams, and a beam projectile between them
                let y_start = self.bounds.y + self.bounds.h / 2.0;
                let left_pos = vec2(self.bounds.x + PROTO_RADIUS, y_start);
                let right_pos = vec2(self.bounds.x + self.bounds.w - PROTO_RADIUS, y_start);

                self.proto_beams.push(ProtoBeam::new(left_pos, -1.0));
                self.proto_beams.push(ProtoBeam::new(right_pos, 1.0));

                let mut beam = BeamProjectile::new(left_pos, right_pos);
                beam.is_proto_beam = true;
                state.projectiles.push(Projectile::Beam(beam));
            }
        }
        // not in transition state and the proto beams are still active, so turn them off
        else if !self.proto_beams.is_empty() {
            let is_any_active = self
                .proto_beams
                .iter()
                .any(|pb| pb.state == ProtoBeamState::Active);
            if is_any_active {
                // turn off the beam
                for proj in &mut state.projectiles {
                    if let Projectile::Beam(beam) = proj {
                        if beam.is_proto_beam {
                            beam.is_proto_beam = false;
                            beam.elapsed = 9999.0;
                        }
                    }
                }
                // tell the ProtoBeams to die
                for pb in &mut self.proto_beams {
                    pb.die();
                }
            }
        }

        // update proto_beams
        for pb in &mut self.proto_beams {
            pb.update(dt, self.bounds);
        }

        // sync dynamic beam position
        if !self.proto_beams.is_empty() {
            let left_pos = self
                .proto_beams
                .iter()
                .find(|pb| pb.side < 0.0)
                .map(|pb| pb.position);
            let right_pos = self
                .proto_beams
                .iter()
                .find(|pb| pb.side > 0.0)
                .map(|pb| pb.position);
            if let (Some(lp), Some(rp)) = (left_pos, right_pos) {
                for proj in &mut state.projectiles {
                    if let Projectile::Beam(beam) = proj {
                        if beam.is_proto_beam {
                            beam.start = lp;
                            beam.end = rp;
                        }
                    }
                }
            }
        }

        self.proto_beams.retain(|pb| !pb.is_fully_dead());

        // every nwo and then spawn soem protos when the boss is alive
        self.spawn_proto(true, dt, state.protos_killed);

        let (_, enemy_positions) = self.alive_enemy_count();

        let modifier_context = ModifierContext {
            arena_bounds: self.bounds,
            enemy_positions,
            player_position: self.player.position,
        };

        // update projectiles, some projectiles are beams or bullets
        // that has to go through their lifecycle
        for projectile in &mut state.projectiles {
            projectile.update(dt, Some(&modifier_context));
        }

        // drop bullets that left the arena and beams that have expired
        let bounds = self.bounds;
        state.projectiles.retain(|p| !p.is_dead(bounds));

        // update visual effects and retain those that haven't expired
        state.visual_effects.retain_mut(|effect| effect.update(dt));

        // clears all hazards in the bomb radius
        if let Some(bomb) = &mut self.bomb {
            state.projectiles.retain(|p| !bomb.clears(p));
            if !bomb.update(dt) {
                self.bomb = None;
            }
        }

        // handle collisions after all movement is done
        handle_collisions(
            state,
            &self.player,
            &mut self.boss,
            &mut self.protos,
            self.bounds,
            events,
        );
    }

    pub fn spawn_proto(&mut self, check_timer: bool, dt: f32, protos_killed: u32) {
        if self.boss.is_dead() {
            return;
        }

        if check_timer {
            self.proto_spawn_timer -= dt;
            if self.proto_spawn_timer > 0.0 {
                return;
            }
            // reset the timer
            self.proto_spawn_timer =
                rand::gen_range(PROTO_SPAWN_MIN_INTERVAL, PROTO_SPAWN_MAX_INTERVAL);
        }

        let existing: Vec<usize> = self.protos.iter().map(|p| p.slot_idx).collect();
        let empty_slots: Vec<usize> = (0..PROTO_MAX_SLOTS)
            .filter(|idx| !existing.contains(idx))
            .collect();

        if !empty_slots.is_empty() {
            let rand_idx = rand::gen_range(0, empty_slots.len());
            let slot_idx = empty_slots[rand_idx];
            let angle = (slot_idx as f32) * std::f32::consts::PI / (PROTO_MAX_SLOTS - 1) as f32;
            let spawn_pos =
                self.boss.position + Vec2::new(angle.cos(), angle.sin()) * PROTO_SPAWN_OFFSET_X;
            let extra_health = (protos_killed as i32) * 100;
            self.protos
                .push(Proto::new(spawn_pos, slot_idx, extra_health));
        }
    }

    pub fn draw(&self, state: &GameState, shaders: &Shaders) {
        // projectiles first so the boss draws on top and hides any under it
        for projectile in &state.projectiles {
            projectile.draw(shaders);
        }
        // beams are drawn overshooting the arena edges (see Boss::beam_span); paint over
        // everything outside the arena now to hide that overflow, before the rest of the
        // scene draws on top
        self.draw_frame_mask();

        self.player.draw();

        for proto in &self.protos {
            proto.draw();
        }

        for pb in &self.proto_beams {
            pb.draw();
        }

        self.boss.draw();

        // draw active visual effects on top of the boss
        for effect in &state.visual_effects {
            effect.draw();
        }

        // bomb ring on top of the scene while it's active
        if let Some(bomb) = &self.bomb {
            bomb.draw();
        }
        self.draw_border();
    }

    // fill everything just outside the arena with the background colour, masking the bit
    // of each beam drawn past the edges. pad is generous so screen shake can't reveal a seam
    fn draw_frame_mask(&self) {
        let b = self.bounds;
        let pad = FRAME_MASK_PAD;
        draw_rectangle(b.x - pad, b.y - pad, pad, b.h + 2.0 * pad, BACKGROUND); // left
        draw_rectangle(b.x + b.w, b.y - pad, pad, b.h + 2.0 * pad, BACKGROUND); // right
        draw_rectangle(b.x - pad, b.y - pad, b.w + 2.0 * pad, pad, BACKGROUND); // top
        draw_rectangle(b.x - pad, b.y + b.h, b.w + 2.0 * pad, pad, BACKGROUND); // bottom
    }

    fn draw_border(&self) {
        draw_rectangle_lines(
            self.bounds.x,
            self.bounds.y,
            self.bounds.w,
            self.bounds.h,
            ARENA_BORDER_THICKNESS,
            ARENA_BORDER_COLOR,
        );
    }
}

impl Default for Arena {
    fn default() -> Self {
        Arena::new()
    }
}
