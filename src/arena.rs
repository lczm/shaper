use macroquad::prelude::*;

use crate::boss::Boss;
use crate::constants::{
    ARENA_BORDER_COLOR, ARENA_BORDER_THICKNESS, ARENA_MARGIN_HEIGHT, ARENA_MARGIN_WIDTH, HEIGHT,
};
use crate::input::Input;
use crate::player::Player;
use crate::state::GameState;

// the gameplay arena and everything inside it
pub struct Arena {
    bounds: Rect,
    player: Player,
    boss: Boss,
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
        }
    }

    // the rectangular gameplay border, in logical coordinates
    pub fn bounds(&self) -> Rect {
        self.bounds
    }

    pub fn update(&mut self, dt: f32, input: &Input, state: &mut GameState) {
        self.player.update(dt, input, self.bounds, state);

        // boss may push some projectiles into the game state
        self.boss.update(dt, state);

        // update projectile movements
        for projectile in &mut state.projectiles {
            projectile.update(dt);
        }

        // drop any projectiles that have left the arena
        let bounds = self.bounds;
        state.projectiles.retain(|p| !p.is_off_screen(bounds));

        // TODO : gameplay state
        // collisions etc
    }

    pub fn draw(&self, state: &GameState) {
        // projectiles first so the boss draws on top and hides any under it
        for projectile in &state.projectiles {
            projectile.draw();
        }
        self.player.draw();
        self.boss.draw();
        self.draw_border();
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
