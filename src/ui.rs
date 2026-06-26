use macroquad::prelude::*;

use crate::constants::{
    ARENA_BORDER_COLOR, HEALTH_BAR_BG_COLOR, HEALTH_BAR_FILL_COLOR, HEALTH_BAR_HEIGHT,
    HEALTH_BAR_TOP_MARGIN, UI_TEXT_COLOR,
};
use crate::state::GameState;

pub struct Ui;

impl Ui {
    pub fn new() -> Self {
        Ui
    }

    pub fn draw(&self, state: &GameState, bounds: Rect, boss_health: (i32, i32)) {
        // use screen space camera
        set_default_camera();

        // boss health bar across the top, aligned over the arena
        self.draw_boss_health(bounds, boss_health.0, boss_health.1);

        let x = bounds.right() + 40.0;
        let mut y = bounds.y + 40.0;
        draw_text(format!("Lives: {}", state.lives), x, y, 32.0, UI_TEXT_COLOR);

        y += 40.0;
        draw_text(format!("Bombs: {}", state.bombs), x, y, 32.0, UI_TEXT_COLOR);
    }

    fn draw_boss_health(&self, bounds: Rect, current: i32, total: i32) {
        let x = bounds.x;
        let w = bounds.w;
        let y = HEALTH_BAR_TOP_MARGIN;
        let h = HEALTH_BAR_HEIGHT;

        // empty track underneath
        draw_rectangle(x, y, w, h, HEALTH_BAR_BG_COLOR);

        // red fill, width scaled by the remaining health fraction
        let frac = if total > 0 {
            (current as f32 / total as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        draw_rectangle(x, y, w * frac, h, HEALTH_BAR_FILL_COLOR);

        // outline on top of the fill
        draw_rectangle_lines(x, y, w, h, 2.0, ARENA_BORDER_COLOR);

        // current / total centered in the bar
        let label = format!("{current} / {total}");
        let font_size: u16 = 20;
        let dims = measure_text(&label, None, font_size, 1.0);
        draw_text(
            &label,
            x + (w - dims.width) / 2.0,
            y + (h + dims.height) / 2.0,
            font_size as f32,
            UI_TEXT_COLOR,
        );
    }
}

impl Default for Ui {
    fn default() -> Self {
        Ui::new()
    }
}
