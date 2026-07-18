use crate::constants::{STARTING_BOMBS, STARTING_LIVES};
use crate::gfx::ActiveVisualEffect;
use crate::modifiers::ModifiersGenerator;
use crate::projectile::Projectile;

pub struct GameState {
    pub lives: u32,
    pub bombs: u32,
    pub projectiles: Vec<Projectile>,
    pub visual_effects: Vec<ActiveVisualEffect>,
    pub modifiers_generator: ModifiersGenerator,
    pub protos_killed: u32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            lives: STARTING_LIVES,
            bombs: STARTING_BOMBS,
            projectiles: Vec::new(),
            visual_effects: Vec::new(),
            modifiers_generator: ModifiersGenerator::new(),
            protos_killed: 0,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::new()
    }
}
