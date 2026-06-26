use crate::constants::{STARTING_BOMBS, STARTING_LIVES};
use crate::projectile::Projectile;

// things that the game events can emit
// like the player getting hit
// maybe a BombDetonated event in the future
pub enum GameEvent {
    PlayerHit,
}

pub struct GameState {
    pub lives: u32,
    pub bombs: u32,
    pub projectiles: Vec<Projectile>,
    // drained every frame
    pub events: Vec<GameEvent>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            lives: STARTING_LIVES,
            bombs: STARTING_BOMBS,
            projectiles: Vec::new(),
            events: Vec::new(),
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::new()
    }
}
