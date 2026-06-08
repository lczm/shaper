use macroquad::prelude::*;

pub struct Circle {
    pub radius: f32,
    // border thickness (only used when not filled)
    pub thickness: f32,
    pub color: Color,
    // solid disc when true, hollow ring when false
    pub filled: bool,
}

impl Circle {
    pub fn new(radius: f32, color: Color) -> Self {
        Circle {
            radius,
            thickness: 2.0,
            color,
            filled: false,
        }
    }

    pub fn draw(&self, pos: Vec2, opacity: f32) {
        let color = Color {
            a: self.color.a * opacity,
            ..self.color
        };
        if self.filled {
            draw_circle(pos.x, pos.y, self.radius, color);
        } else {
            draw_circle_lines(pos.x, pos.y, self.radius, self.thickness, color);
        }
    }
}
