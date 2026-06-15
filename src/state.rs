use crate::constants::STARTING_LIVES;

pub struct GameState {
    pub lives: u32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            lives: STARTING_LIVES,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::new()
    }
}
