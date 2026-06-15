use crate::constants::STARTING_LIVES;
use crate::projectile::Projectile;

pub struct GameState {
    pub lives: u32,
    pub projectiles: Vec<Projectile>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            lives: STARTING_LIVES,
            projectiles: Vec::new(),
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::new()
    }
}
