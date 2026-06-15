use macroquad::prelude::*;

use crate::constants::UI_TEXT_COLOR;
use crate::state::GameState;

pub struct Ui;

impl Ui {
    pub fn new() -> Self {
        Ui
    }

    pub fn draw(&self, state: &GameState, bounds: Rect) {
        // use screen space camera
        set_default_camera();

        let x = bounds.right() + 40.0;
        let y = bounds.y + 40.0;
        draw_text(format!("Lives: {}", state.lives), x, y, 32.0, UI_TEXT_COLOR);
    }
}

impl Default for Ui {
    fn default() -> Self {
        Ui::new()
    }
}
