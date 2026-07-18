use macroquad::prelude::*;

use crate::bomb::Bomb;
use crate::boss::Boss;
use crate::collision::handle_collisions;
use crate::constants::{
    ARENA_BORDER_COLOR, ARENA_BORDER_THICKNESS, ARENA_MARGIN_HEIGHT, ARENA_MARGIN_WIDTH,
    BACKGROUND, BOMB_DURATION, FRAME_MASK_PAD, HEIGHT, PROTO_MAX_SLOTS, PROTO_SPAWN_MAX_INTERVAL,
    PROTO_SPAWN_MIN_INTERVAL, PROTO_SPAWN_OFFSET_X,
};
use crate::gfx::Shaders;
use crate::input::Input;
use crate::modifiers::ModifierContext;
use crate::player::Player;
use crate::proto::Proto;
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
        );
        let right_proto = Proto::new(boss_pos + vec2(PROTO_SPAWN_OFFSET_X, 0.0), 0);

        let proto_spawn_timer = rand::gen_range(PROTO_SPAWN_MIN_INTERVAL, PROTO_SPAWN_MAX_INTERVAL);

        Arena {
            bounds,
            // player near the bottom-center, boss near the top-center
            player: Player::new(vec2(center_x, bounds.y + bounds.h * 4.0 / 5.0)),
            boss: Boss::new(boss_pos),
            protos: vec![left_proto, right_proto],
            proto_spawn_timer,
            bomb: None,
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

    // set bomb at position (player center) and grant iframes
    pub fn detonate_bomb(&mut self, position: Vec2) {
        self.bomb = Some(Bomb::new(position));
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

        self.player.update(dt, input, self.bounds, state, events);

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

        // every nwo and then spawn soem protos when the boss is alive
        self.spawn_proto_periodically(dt);

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

    // when the boss isnt dead, spawn some protos at random intervals
    fn spawn_proto_periodically(&mut self, dt: f32) {
        if self.boss.is_dead() {
            return;
        }

        self.proto_spawn_timer -= dt;
        if self.proto_spawn_timer <= 0.0 {
            // reset the timer
            self.proto_spawn_timer =
                rand::gen_range(PROTO_SPAWN_MIN_INTERVAL, PROTO_SPAWN_MAX_INTERVAL);

            // get all the existing proto slots
            let existing: Vec<usize> = self.protos.iter().map(|p| p.slot_idx).collect();
            // then filter out all the existing slots to get empty slots
            let empty_slots: Vec<usize> = (0..PROTO_MAX_SLOTS)
                .filter(|idx| !existing.contains(idx))
                .collect();

            if !empty_slots.is_empty() {
                let rand_idx = rand::gen_range(0, empty_slots.len());
                let slot_idx = empty_slots[rand_idx];
                let angle = (slot_idx as f32) * std::f32::consts::PI / (PROTO_MAX_SLOTS - 1) as f32;
                let spawn_pos =
                    self.boss.position + Vec2::new(angle.cos(), angle.sin()) * PROTO_SPAWN_OFFSET_X;
                self.protos.push(Proto::new(spawn_pos, slot_idx));
            }
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
