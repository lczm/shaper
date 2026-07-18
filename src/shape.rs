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
        self.draw_rotated(pos, 0.0, opacity);
    }

    pub fn draw_rotated(&self, pos: Vec2, rotation: f32, opacity: f32) {
        self.draw_colored(pos, self.color, rotation, opacity);
    }

    // draw with a one-off color override
    pub fn draw_colored(&self, pos: Vec2, color: Color, rotation: f32, opacity: f32) {
        let color = Color {
            a: color.a * opacity,
            ..color
        };
        // offset (0.5, 0.5) makes pos the center and rotation pivot
        let params = DrawRectangleParams {
            offset: vec2(0.5, 0.5),
            rotation,
            color,
        };
        if self.filled {
            draw_rectangle_ex(pos.x, pos.y, self.size.x, self.size.y, params);
        } else {
            draw_rectangle_lines_ex(
                pos.x,
                pos.y,
                self.size.x,
                self.size.y,
                self.thickness,
                params,
            );
        }
    }
}

pub struct Triangle {
    pub radius: f32,
    pub thickness: f32,
    pub color: Color,
    pub filled: bool,
}

impl Triangle {
    pub fn new(radius: f32, color: Color) -> Self {
        Triangle {
            radius,
            thickness: 2.0,
            color,
            filled: true,
        }
    }

    pub fn draw(&self, pos: Vec2, rotation: f32, opacity: f32) {
        self.draw_colored(pos, self.color, rotation, opacity);
    }

    pub fn draw_colored(&self, pos: Vec2, color: Color, rotation: f32, opacity: f32) {
        let color = Color {
            a: color.a * opacity,
            ..color
        };
        let angle1 = rotation;
        let angle2 = rotation + 2.0 * std::f32::consts::PI / 3.0;
        let angle3 = rotation + 4.0 * std::f32::consts::PI / 3.0;

        let v1 = pos + Vec2::from_angle(angle1) * self.radius;
        let v2 = pos + Vec2::from_angle(angle2) * self.radius;
        let v3 = pos + Vec2::from_angle(angle3) * self.radius;

        if self.filled {
            draw_triangle(v1, v2, v3, color);
        } else {
            draw_triangle_lines(v1, v2, v3, self.thickness, color);
        }
    }
}

