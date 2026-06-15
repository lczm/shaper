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
        self.draw_colored(pos, self.color, opacity);
    }

    // draw with a one-off color override (e.g. gray ghost trail)
    pub fn draw_colored(&self, pos: Vec2, color: Color, opacity: f32) {
        let color = Color {
            a: color.a * opacity,
            ..color
        };
        if self.filled {
            draw_circle(pos.x, pos.y, self.radius, color);
        } else {
            draw_circle_lines(pos.x, pos.y, self.radius, self.thickness, color);
        }
    }
}

pub struct Rectangle {
    pub size: Vec2,
    // border thickness (only used when not filled)
    pub thickness: f32,
    pub color: Color,
    // solid rectangle when true, hollow outline when false
    pub filled: bool,
}

impl Rectangle {
    pub fn new(size: Vec2, color: Color) -> Self {
        Rectangle {
            size,
            thickness: 2.0,
            color,
            filled: true,
        }
    }

    pub fn draw(&self, pos: Vec2, opacity: f32) {
        self.draw_colored(pos, self.color, opacity);
    }

    // draw with a one-off color override. pos is the center of the rectangle
    pub fn draw_colored(&self, pos: Vec2, color: Color, opacity: f32) {
        let color = Color {
            a: color.a * opacity,
            ..color
        };
        // macroquad draws from the top-left corner, so offset from the center
        let top_left = pos - self.size / 2.0;
        if self.filled {
            draw_rectangle(top_left.x, top_left.y, self.size.x, self.size.y, color);
        } else {
            draw_rectangle_lines(
                top_left.x,
                top_left.y,
                self.size.x,
                self.size.y,
                self.thickness,
                color,
            );
        }
    }
}
