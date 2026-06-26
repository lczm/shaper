mod beam;
mod post;
mod shake;

use macroquad::miniquad::{BlendFactor, BlendState, BlendValue, Equation};
use macroquad::prelude::*;

pub use post::Post;
pub use shake::Shake;

// default vertex shader
pub(crate) const VERTEX_SHADER: &str = include_str!("standard.vert");

pub(crate) fn alpha_blend() -> BlendState {
    BlendState::new(
        Equation::Add,
        BlendFactor::Value(BlendValue::SourceAlpha),
        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
    )
}

// todo: scaffolding for bloom pass in the future
pub(crate) fn additive_blend() -> BlendState {
    BlendState::new(
        Equation::Add,
        BlendFactor::Value(BlendValue::SourceAlpha),
        BlendFactor::One,
    )
}

// load a material given the fragment shader and list of uniforms
pub(crate) fn load(
    fragment: &str,
    uniforms: Vec<UniformDesc>,
    blend: Option<BlendState>,
) -> Option<Material> {
    let params = MaterialParams {
        pipeline_params: PipelineParams {
            color_blend: blend,
            ..Default::default()
        },
        uniforms,
        ..Default::default()
    };
    match load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment,
        },
        params,
    ) {
        Ok(material) => Some(material),
        Err(e) => {
            eprintln!("shader failed to load, using fallback: {e:?}");
            None
        }
    }
}

// all shaders live here
// when anything needs to be drawn with shaders, pull from this struct and call the appropriate draw function
pub struct Shaders {
    beam: Option<Material>,
}

impl Shaders {
    pub fn new() -> Self {
        Shaders {
            beam: beam::material(),
        }
    }

    // draw the beam from `start` to `end` as a flat solid band in the given color;
    // falls back to a plain line if the shader didn't load
    pub fn draw_beam(&self, start: Vec2, end: Vec2, width: f32, color: Color) {
        match &self.beam {
            Some(material) => beam::draw(material, start, end, width, color),
            None => draw_line(start.x, start.y, end.x, end.y, width, color),
        }
    }
}

impl Default for Shaders {
    fn default() -> Self {
        Shaders::new()
    }
}
