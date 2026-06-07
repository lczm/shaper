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

    pub fn draw(&self, pos: Vec2) {
        if self.filled {
            draw_circle(pos.x, pos.y, self.radius, self.color);
        } else {
            draw_circle_lines(pos.x, pos.y, self.radius, self.thickness, self.color);
        }
    }
}
