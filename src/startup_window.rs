use crate::{
    collision::point_in_rect,
    constants::{
        STARTUP_ACTION_TEXT_COLOR, STARTUP_BUTTON_BG_COLOR, STARTUP_BUTTON_BORDER_COLOR,
        STARTUP_BUTTON_HOVER_BG_COLOR, STARTUP_DIVIDER_COLOR, STARTUP_MUTED_TEXT_COLOR,
        STARTUP_OVERLAY_COLOR, STARTUP_PRIMARY_COLOR, STARTUP_WINDOW_BG_COLOR,
    },
};
use macroquad::prelude::*;

pub struct StartupWindow {
    button_hovered: bool,
}

impl StartupWindow {
    pub fn new() -> Self {
        StartupWindow {
            button_hovered: false,
        }
    }

    // returns true if the user clicks the "START" button or presses Enter to start
    pub fn update(&mut self, _dt: f32, mouse: Vec2, primary_pressed: bool) -> bool {
        let sw = screen_width();
        let sh = screen_height();
        let win_w = sw * 0.8;
        let win_h = sh * 0.8;
        let win_x = (sw - win_w) / 2.0;
        let win_y = (sh - win_h) / 2.0;

        let btn_w = 200.0;
        let btn_h = 50.0;
        let btn_x = win_x + (win_w - btn_w) / 2.0;
        let btn_y = win_y + win_h - 100.0;
        let btn_rect = Rect::new(btn_x, btn_y, btn_w, btn_h);

        self.button_hovered = point_in_rect(mouse, btn_rect);

        if (primary_pressed && self.button_hovered) || is_key_pressed(KeyCode::Enter) {
            return true;
        }

        false
    }

    pub fn draw(&self) {
        set_default_camera();

        let sw = screen_width();
        let sh = screen_height();

        // dim background
        draw_rectangle(0.0, 0.0, sw, sh, STARTUP_OVERLAY_COLOR);

        // window rect
        let win_w = sw * 0.8;
        let win_h = sh * 0.8;
        let win_x = (sw - win_w) / 2.0;
        let win_y = (sh - win_h) / 2.0;

        // body & border
        draw_rectangle(win_x, win_y, win_w, win_h, STARTUP_WINDOW_BG_COLOR);
        draw_rectangle_lines(win_x, win_y, win_w, win_h, 4.0, STARTUP_PRIMARY_COLOR);

        // title
        let title = "SHAPER";
        let title_size = 64.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            win_x + (win_w - title_dims.width) / 2.0,
            win_y + 80.0,
            title_size,
            STARTUP_PRIMARY_COLOR,
        );

        // subtitle
        let subtitle = "Controls Guide";
        let sub_size = 24.0;
        let sub_dims = measure_text(subtitle, None, sub_size as u16, 1.0);
        draw_text(
            subtitle,
            win_x + (win_w - sub_dims.width) / 2.0,
            win_y + 115.0,
            sub_size,
            STARTUP_MUTED_TEXT_COLOR,
        );

        // divider
        let line_y = win_y + 140.0;
        draw_line(
            win_x + 50.0,
            line_y,
            win_x + win_w - 50.0,
            line_y,
            1.0,
            STARTUP_DIVIDER_COLOR,
        );

        // controls listing
        let text_size = 22.0;
        let line_spacing = 40.0;
        let start_y = win_y + 200.0;

        let controls = [
            (
                "MOVEMENT",
                "Arrow Keys / WASD",
                "Move player around the arena",
            ),
            ("BOMB", "Z or L", "Clear all projectiles within bomb radius"),
            (
                "PHASE",
                "Shift Key",
                "Dash and gain temporary invulnerability (iframes)",
            ),
        ];

        for (idx, (action, key, desc)) in controls.iter().enumerate() {
            let y_pos = start_y + (idx as f32) * line_spacing * 2.0;

            let action_text = format!("{action}:");
            let action_dims = measure_text(&action_text, None, text_size as u16, 1.0);
            draw_text(
                &action_text,
                win_x + win_w * 0.25 - action_dims.width,
                y_pos,
                text_size,
                STARTUP_ACTION_TEXT_COLOR,
            );

            draw_text(
                key,
                win_x + win_w * 0.28,
                y_pos,
                text_size,
                STARTUP_PRIMARY_COLOR,
            );

            draw_text(
                desc,
                win_x + win_w * 0.48,
                y_pos,
                text_size - 2.0,
                STARTUP_MUTED_TEXT_COLOR,
            );
        }

        // play button
        let btn_w = 200.0;
        let btn_h = 50.0;
        let btn_x = win_x + (win_w - btn_w) / 2.0;
        let btn_y = win_y + win_h - 100.0;

        let btn_bg = if self.button_hovered {
            STARTUP_BUTTON_HOVER_BG_COLOR
        } else {
            STARTUP_BUTTON_BG_COLOR
        };
        let btn_border = if self.button_hovered {
            STARTUP_PRIMARY_COLOR
        } else {
            STARTUP_BUTTON_BORDER_COLOR
        };

        draw_rectangle(btn_x, btn_y, btn_w, btn_h, btn_bg);
        draw_rectangle_lines(btn_x, btn_y, btn_w, btn_h, 2.0, btn_border);

        let btn_text = "START GAME";
        let btn_text_size = 20.0;
        let btn_text_dims = measure_text(btn_text, None, btn_text_size as u16, 1.0);
        draw_text(
            btn_text,
            btn_x + (btn_w - btn_text_dims.width) / 2.0,
            btn_y + btn_h / 2.0 + 7.0,
            btn_text_size,
            STARTUP_PRIMARY_COLOR,
        );
    }
}
