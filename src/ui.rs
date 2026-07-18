use macroquad::prelude::*;

use crate::boss::BOSS_SPECIAL_HP_THRESHOLDS;
use crate::constants::{
    ARENA_BORDER_COLOR, BANNER_FONT_SIZE, HEALTH_BAR_BG_COLOR, HEALTH_BAR_CHIP_COLOR,
    HEALTH_BAR_FILL_COLOR, HEALTH_BAR_HEIGHT, HEALTH_BAR_INVULN_FILL_COLOR,
    HEALTH_BAR_MARKER_COLOR, HEALTH_BAR_MARKER_THICKNESS, HEALTH_BAR_TOP_MARGIN, HEIGHT,
    UI_TEXT_COLOR, WIDTH,
};
use crate::modifiers::Modifier;
use crate::state::GameState;

pub struct Ui;

impl Ui {
    pub fn new() -> Self {
        Ui
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &self,
        state: &GameState,
        bounds: Rect,
        player_damage: i32,
        player_fire_interval: f32,
        boss_health: (i32, i32),
        boss_displayed: f32,
        boss_invulnerable: bool,
        reset_banner: f32,
        lost_banner: f32,
        game_over_banner: f32,
        paused: bool,
        modifiers: &[Modifier],
    ) {
        let sx = screen_width() / WIDTH;
        let sy = screen_height() / HEIGHT;

        // use screen space camera
        set_default_camera();

        // boss health bar across the top, aligned over the arena
        self.draw_boss_health(
            bounds,
            boss_health.0,
            boss_health.1,
            boss_displayed,
            boss_invulnerable,
            sx,
            sy,
        );

        let x = bounds.right() * sx + 40.0 * sx;
        let mut y = bounds.y * sy + 40.0 * sy;
        draw_text(
            format!("Lives: {}", state.lives),
            x,
            y,
            32.0 * sx,
            UI_TEXT_COLOR,
        );

        y += 40.0 * sy;
        draw_text(
            format!("Bombs: {}", state.bombs),
            x,
            y,
            32.0 * sx,
            UI_TEXT_COLOR,
        );

        // separator line
        y += 24.0 * sy;
        draw_line(x, y, x + 500.0 * sx, y, 2.0 * sx, ARENA_BORDER_COLOR);

        // calculate fire rate
        let fire_rate = 1.0 / player_fire_interval;
        // calculate player damage (from player potential damage), but scaled with fire rate
        let player_damage = player_damage * fire_rate as i32;

        // weapon stats
        y += 36.0 * sy;
        draw_text(
            format!("Damage : {player_damage}"),
            x,
            y,
            32.0 * sx,
            UI_TEXT_COLOR,
        );

        y += 40.0 * sy;
        draw_text(
            format!("Fire rate : {fire_rate:.1}/s"),
            x,
            y,
            32.0 * sx,
            UI_TEXT_COLOR,
        );

        // separator line below fire rate
        y += 24.0 * sy;
        draw_line(x, y, x + 500.0 * sx, y, 2.0 * sx, ARENA_BORDER_COLOR);

        //g et all modifiers
        let active_modifiers: Vec<&Modifier> = modifiers.iter().collect();
        if !active_modifiers.is_empty() {
            // draw heading
            y += 32.0 * sy;
            draw_text("Modifiers:", x, y, 24.0 * sx, ARENA_BORDER_COLOR);

            let bottom_limit = screen_height() - 30.0 * sy;
            let mut available_height = bottom_limit - (y + 10.0 * sy);

            let desc_font_size = 16.0 * sx;
            let title_font_size = 20.0 * sx;
            let max_desc_width = 480.0 * sx;
            let indent = 20.0 * sx;

            let mut modifiers_to_draw = Vec::new();
            for &modifier in active_modifiers.iter().rev() {
                let desc_lines = crate::utils::wrap_text(
                    &modifier.description(),
                    max_desc_width,
                    desc_font_size,
                );
                let desc_height = desc_lines.len() as f32 * 18.0 * sy;
                let needed_height = 34.0 * sy + desc_height;

                if available_height >= needed_height {
                    available_height -= needed_height;
                    modifiers_to_draw.push((modifier, desc_lines));
                } else {
                    break;
                }
            }
            modifiers_to_draw.reverse();

            for (modifier, desc_lines) in modifiers_to_draw {
                y += 28.0 * sy;
                // draw name
                draw_text(&modifier.name(), x, y, title_font_size, UI_TEXT_COLOR);
                // draw description
                for line in desc_lines {
                    y += 18.0 * sy;
                    draw_text(
                        &line,
                        x + indent,
                        y,
                        desc_font_size,
                        Color::new(0.70, 0.72, 0.76, 1.0),
                    );
                }
                y += 6.0 * sy;
            }
        }

        // the different banners
        if paused {
            self.draw_banner("Paused");
        } else if reset_banner > 0.0 {
            self.draw_banner("Reset");
        } else if lost_banner > 0.0 {
            self.draw_banner("Lost");
        } else if game_over_banner > 0.0 {
            self.draw_banner("Game Over");
        }
    }

    // full-screen centered text banner
    fn draw_banner(&self, label: &str) {
        let sx = screen_width() / WIDTH;
        let font_size = BANNER_FONT_SIZE * sx;
        let dims = measure_text(label, None, font_size as u16, 1.0);
        draw_text(
            label,
            (screen_width() - dims.width) / 2.0,
            (screen_height() + dims.height) / 2.0,
            font_size,
            UI_TEXT_COLOR,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_boss_health(
        &self,
        bounds: Rect,
        current: i32,
        total: i32,
        displayed: f32,
        invulnerable: bool,
        sx: f32,
        sy: f32,
    ) {
        let x = bounds.x * sx;
        let w = bounds.w * sx;
        let y = HEALTH_BAR_TOP_MARGIN * sy;
        let h = HEALTH_BAR_HEIGHT * sy;

        // red snaps to the current health, chip trails behind at the displayed health
        let frac = |v: f32| {
            if total > 0 {
                (v / total as f32).clamp(0.0, 1.0)
            } else {
                0.0
            }
        };
        let cur_frac = frac(current as f32);
        let chip_frac = frac(displayed).max(cur_frac);
        // swap to the invulnerable tint while the boss can't be damaged
        let boss_fill_color = if invulnerable {
            HEALTH_BAR_INVULN_FILL_COLOR
        } else {
            HEALTH_BAR_FILL_COLOR
        };

        // empty track underneath
        draw_rectangle(x, y, w, h, HEALTH_BAR_BG_COLOR);

        // pale chip over the just-removed health, draining down to the red fill
        if chip_frac > cur_frac {
            let chip_x = x + w * cur_frac;
            draw_rectangle(
                chip_x,
                y,
                w * (chip_frac - cur_frac),
                h,
                HEALTH_BAR_CHIP_COLOR,
            );
        }

        // health fill, width scaled by the remaining health fraction
        draw_rectangle(x, y, w * cur_frac, h, boss_fill_color);

        // thin vertical strips at each special-move threshold, so the player can see the
        // upcoming invulnerability windows on the bar
        let marker_w = HEALTH_BAR_MARKER_THICKNESS * sx;
        for &threshold in BOSS_SPECIAL_HP_THRESHOLDS.iter() {
            draw_rectangle(
                x + w * threshold - marker_w / 2.0,
                y,
                marker_w,
                h,
                HEALTH_BAR_MARKER_COLOR,
            );
        }

        // outline on top of the fill
        draw_rectangle_lines(x, y, w, h, 2.0 * sx, ARENA_BORDER_COLOR);

        // current / total centered in the bar
        let label = format!("{current} / {total}");
        let font_size: f32 = 20.0 * sx;
        let dims = measure_text(&label, None, font_size as u16, 1.0);
        draw_text(
            &label,
            x + (w - dims.width) / 2.0,
            y + (h + dims.height) / 2.0,
            font_size,
            UI_TEXT_COLOR,
        );
    }
}

impl Default for Ui {
    fn default() -> Self {
        Ui::new()
    }
}
