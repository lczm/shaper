
use crate::constants::{STARTING_BOMBS, STARTING_LIVES};
use crate::gfx::ActiveVisualEffect;
use crate::projectile::Projectile;

pub struct GameState {
    pub lives: u32,
    pub bombs: u32,
    pub projectiles: Vec<Projectile>,
    pub visual_effects: Vec<ActiveVisualEffect>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            lives: STARTING_LIVES,
            bombs: STARTING_BOMBS,
            projectiles: Vec::new(),
            visual_effects: Vec::new(),
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::new()
    }
}
