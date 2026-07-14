use macroquad::prelude::*;

use crate::bomb::Bomb;
use crate::boss::Boss;
use crate::collision::handle_collisions;
use crate::constants::{
    ARENA_BORDER_COLOR, ARENA_BORDER_THICKNESS, ARENA_MARGIN_HEIGHT, ARENA_MARGIN_WIDTH,
    BACKGROUND, BOMB_DURATION, FRAME_MASK_PAD, HEIGHT,
};
use crate::gfx::Shaders;
use crate::input::Input;
use crate::player::Player;
use crate::state::GameState;
use crate::world::GameEvent;

// the gameplay arena and everything inside it
pub struct Arena {
    bounds: Rect,
    player: Player,
    boss: Boss,
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
        Arena {
            bounds,
            // player near the bottom-center, boss near the top-center
            player: Player::new(vec2(center_x, bounds.y + bounds.h * 4.0 / 5.0)),
            boss: Boss::new(vec2(center_x, bounds.y + bounds.h / 5.0)),
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

    // (current, total) boss health for the hud health bar
    pub fn boss_health(&self) -> (i32, i32) {
        self.boss.health()
    }

    // trailing chip value
    pub fn boss_displayed_health(&self) -> f32 {
        self.boss.displayed_health()
    }

    pub fn player_damage(&self) -> i32 {
        self.player.damage()
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
            self.player.dev_damage_boost();
        }

        self.player.update(dt, input, self.bounds, state, events);

        // boss may push some projectiles into the game state; it aims beams at the player
        self.boss
            .update(dt, state, self.bounds, self.player.position, events);

        // update projectiles, some projectiles are beams or bullets
        // that has to go through their lifecycle
        for projectile in &mut state.projectiles {
            projectile.update(dt);
        }

        // drop bullets that left the arena and beams that have expired
        let bounds = self.bounds;
        state.projectiles.retain(|p| !p.is_dead(bounds));

        // clears all hazards in the bomb radius
        if let Some(bomb) = &mut self.bomb {
            state.projectiles.retain(|p| !bomb.clears(p));
            if !bomb.update(dt) {
                self.bomb = None;
            }
        }

        // handle collisions after all movement is done
        handle_collisions(state, &self.player, &self.boss, events);
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
        self.boss.draw();

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
