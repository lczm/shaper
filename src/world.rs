use macroquad::prelude::*;

use crate::arena::Arena;
use crate::state::GameState;
use crate::ui::Ui;

pub struct World {
    arena: Arena,
    state: GameState,
    ui: Ui,
}

impl World {
    pub fn new() -> Self {
        World {
            arena: Arena::new(),
            state: GameState::new(),
            ui: Ui::new(),
        }
    }

    // step per frame and gets next frame until window closes
    pub async fn run(mut self) {
        loop {
            // update game and update game state for ui to update later on
            self.arena.update(&mut self.state);
            self.arena.draw();

            self.ui.draw(&self.state, self.arena.bounds());

            next_frame().await;
        }
    }
}

impl Default for World {
    fn default() -> Self {
        World::new()
    }
}
