use macroquad::prelude::*;

use crate::collision::point_in_rect;
use crate::modifiers::Modifier;
use crate::utils::wrap_text;

// level up card, it holds a modifier that gets added
// to the players projectile modifier list when selected
#[derive(Clone)]
pub struct LevelUpOption {
    pub name: String,
    pub description: String,
    pub modifier: Modifier,
}

impl LevelUpOption {
    pub fn new(modifier: Modifier) -> Self {
        LevelUpOption {
            name: modifier.name().to_string(),
            description: modifier.description().to_string(),
            modifier,
        }
    }
}

// a window that pops up when the player levels up, showing 3 choices for upgrades
// todo : projectile modifications
pub struct LevelWindow {
    options: [LevelUpOption; 3],
    // which card the mouse is hovering over
    hovered: Option<usize>,
}

impl LevelWindow {
    pub fn new(options: [LevelUpOption; 3]) -> Self {
        LevelWindow {
            options,
            hovered: None,
        }
    }

    pub fn update(&mut self, _dt: f32, mouse: Vec2, primary_pressed: bool) -> Option<usize> {
        // check if mouse is hovering over any of the cards
        let rects = self.card_rects();
        self.hovered = None;
        for (i, r) in rects.iter().enumerate() {
            if point_in_rect(mouse, *r) {
                self.hovered = Some(i);
                break;
            }
        }

        // if clicked while hovering then return the card
        if primary_pressed {
            if let Some(i) = self.hovered {
                return Some(i);
            }
        }

        None
    }

    pub fn selected_modifier(&self, index: usize) -> Modifier {
        self.options[index].modifier.clone()
    }

    // draw the window over the rest of the scene. assumes the default (screen-space)
    // camera is active; the world sets this before calling us
    pub fn draw(&self) {
        set_default_camera();

        let sw = screen_width();
        let sh = screen_height();

        // dim the rest of the scene so the modal pops
        let dim = Color::new(0.0, 0.0, 0.0, 0.65);
        draw_rectangle(0.0, 0.0, sw, sh, dim);

        // window: 80% of the screen, centered
        let win_w = sw * 0.8;
        let win_h = sh * 0.8;
        let win_x = (sw - win_w) / 2.0;
        let win_y = (sh - win_h) / 2.0;

        // black background, white border
        draw_rectangle(win_x, win_y, win_w, win_h, BLACK);
        draw_rectangle_lines(win_x, win_y, win_w, win_h, 4.0, WHITE);

        // title
        let title = "LEVEL UP";
        let title_size = 64.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            win_x + (win_w - title_dims.width) / 2.0,
            win_y + 80.0,
            title_size,
            WHITE,
        );

        // subtitle
        let subtitle = "Choose an upgrade";
        let sub_size = 24.0;
        let sub_dims = measure_text(subtitle, None, sub_size as u16, 1.0);
        draw_text(
            subtitle,
            win_x + (win_w - sub_dims.width) / 2.0,
            win_y + 115.0,
            sub_size,
            Color::new(0.75, 0.75, 0.78, 1.0),
        );

        // 3 cards
        let rects = self.card_rects();
        for (i, r) in rects.iter().enumerate() {
            self.draw_card(i, *r);
        }
    }

    fn draw_card(&self, index: usize, rect: Rect) {
        let hovered = self.hovered == Some(index);
        let bg = if hovered {
            Color::new(0.18, 0.18, 0.22, 1.0)
        } else {
            Color::new(0.08, 0.08, 0.10, 1.0)
        };
        let border = if hovered {
            WHITE
        } else {
            Color::new(0.55, 0.55, 0.60, 1.0)
        };
        let border_thickness = if hovered { 3.0 } else { 2.0 };

        draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, border_thickness, border);

        let option = &self.options[index];
        let pad = 24.0;

        // number indicator in the top-left
        let num_label = format!("[{}]", index + 1);
        let num_size = 24.0;
        draw_text(
            &num_label,
            rect.x + pad,
            rect.y + pad + num_size * 0.8,
            num_size,
            if hovered {
                WHITE
            } else {
                Color::new(0.65, 0.65, 0.68, 1.0)
            },
        );

        // option name, centered horizontally near the top
        let name_size = 30.0;
        let name_dims = measure_text(&option.name, None, name_size as u16, 1.0);
        let name_y = rect.y + rect.h * 0.30;
        draw_text(
            &option.name,
            rect.x + (rect.w - name_dims.width) / 2.0,
            name_y,
            name_size,
            WHITE,
        );

        // divider line under the name
        let line_y = name_y + 20.0;
        draw_line(
            rect.x + pad * 2.0,
            line_y,
            rect.x + rect.w - pad * 2.0,
            line_y,
            1.0,
            Color::new(0.35, 0.35, 0.40, 1.0),
        );

        // description, wrapped to fit the card width
        let desc_size = 18.0;
        let max_w = rect.w - pad * 2.0;
        let lines = wrap_text(&option.description, max_w, desc_size);
        let mut y = rect.y + rect.h * 0.45;
        for line in lines {
            let line_dims = measure_text(&line, None, desc_size as u16, 1.0);
            draw_text(
                &line,
                rect.x + (rect.w - line_dims.width) / 2.0,
                y,
                desc_size,
                Color::new(0.82, 0.82, 0.85, 1.0),
            );
            y += desc_size * 1.4;
        }
    }

    // compute the 3 card rects in screen space. matches the layout in draw()
    // so hover hit-testing lines up with what's actually drawn
    fn card_rects(&self) -> [Rect; 3] {
        let sw = screen_width();
        let sh = screen_height();
        let win_w = sw * 0.8;
        let win_h = sh * 0.8;
        let win_x = (sw - win_w) / 2.0;
        let win_y = (sh - win_h) / 2.0;

        let pad_x = win_w * 0.05;
        let pad_top = win_h * 0.22;
        let pad_bottom = win_h * 0.10;
        let gap = win_w * 0.025;

        let inner_w = win_w - 2.0 * pad_x;
        let card_w = (inner_w - 2.0 * gap) / 3.0;
        let card_h = win_h - pad_top - pad_bottom;

        let card_y = win_y + pad_top;
        let card1_x = win_x + pad_x;
        let card2_x = card1_x + card_w + gap;
        let card3_x = card2_x + card_w + gap;

        [
            Rect::new(card1_x, card_y, card_w, card_h),
            Rect::new(card2_x, card_y, card_w, card_h),
            Rect::new(card3_x, card_y, card_w, card_h),
        ]
    }
}
