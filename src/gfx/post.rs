use macroquad::prelude::*;

use crate::constants::{BACKGROUND, HEIGHT, WIDTH};
use crate::gfx::Shaders;

const RENDER_SCALE: f32 = 2.0;
const POST_FLIP_Y: bool = true;

pub struct Post {
    target: RenderTarget,
}

// full screen post processing scaffold
// used for when all the draws are completed and we want to apply a post-processing effect to the entire scene
impl Post {
    pub fn new() -> Self {
        let target = render_target(
            (WIDTH * RENDER_SCALE) as u32,
            (HEIGHT * RENDER_SCALE) as u32,
        );
        target.texture.set_filter(FilterMode::Linear);
        Post { target }
    }

    // todo : false for now because not used,
    // draw the scene directly to the screen instead of the offscreen target
    pub fn active(&self) -> bool {
        false
    }

    // render into offscreen target
    pub fn begin(&self, base: &Camera2D) {
        let mut camera = crate::gfx::clone_camera(base);
        camera.render_target = Some(self.target.clone());
        set_camera(&camera);
        clear_background(BACKGROUND);
    }

    // todo :
    // postprocessing shader effects applied here
    pub fn present(&self, _shaders: &Shaders) {
        set_default_camera();
        // todo: wrap this draw in gl_use_material(bloom/shockwave), possibly multi-pass
        draw_texture_ex(
            &self.target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: POST_FLIP_Y,
                ..Default::default()
            },
        );
    }
}

impl Default for Post {
    fn default() -> Self {
        Post::new()
    }
}
