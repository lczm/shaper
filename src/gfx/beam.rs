use macroquad::prelude::*;

// the fragment shader for the beam
pub const FRAGMENT: &str = include_str!("beam.frag");

pub fn material() -> Option<Material> {
    super::load(
        FRAGMENT,
        vec![UniformDesc::new("beam_color", UniformType::Float4)],
        Some(super::alpha_blend()),
    )
}

pub fn draw(material: &Material, start: Vec2, end: Vec2, width: f32, color: Color) {
    let dir = end - start;
    let length = dir.length();
    let angle = dir.y.atan2(dir.x);

    material.set_uniform("beam_color", [color.r, color.g, color.b, color.a]);

    // draw with this shader
    gl_use_material(material);

    // draw the actual beam with the shader
    draw_rectangle_ex(
        start.x,
        start.y,
        length,
        width,
        DrawRectangleParams {
            offset: vec2(0.0, 0.5),
            rotation: angle,
            color: WHITE,
        },
    );

    // revert back to the default shader
    gl_use_default_material();
}
