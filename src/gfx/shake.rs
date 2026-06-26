use macroquad::prelude::*;

use crate::constants::{SHAKE_DECAY, SHAKE_MAX_ANGLE, SHAKE_MAX_OFFSET};

// https://bevy.org/examples/camera/2d-screen-shake/
// trauma based screen shake
pub struct Shake {
    trauma: f32,
}

impl Shake {
    pub fn new() -> Self {
        Shake { trauma: 0.0 }
    }

    // bump trauma up (clamped to 1.0), e.g. when the player takes a hit
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).clamp(0.0, 1.0);
    }

    // decay towards 0 every frame
    pub fn update(&mut self, dt: f32) {
        self.trauma = (self.trauma - SHAKE_DECAY * dt).max(0.0);
    }

    pub fn is_active(&self) -> bool {
        self.trauma > 0.0
    }

    // rebuild the camera and apply shake
    pub fn apply(&self, camera: &Camera2D) -> Camera2D {
        // quadratic falloff feels snappier than scaling with raw trauma
        let shake = self.trauma * self.trauma;
        let offset = vec2(
            SHAKE_MAX_OFFSET * shake * rand::gen_range(-1.0, 1.0),
            SHAKE_MAX_OFFSET * shake * rand::gen_range(-1.0, 1.0),
        );
        let angle = SHAKE_MAX_ANGLE * shake * rand::gen_range(-1.0, 1.0);

        Camera2D {
            rotation: camera.rotation + angle,
            zoom: camera.zoom,
            target: camera.target + offset,
            offset: camera.offset,
            render_target: camera.render_target.clone(),
            viewport: camera.viewport,
        }
    }
}

impl Default for Shake {
    fn default() -> Self {
        Shake::new()
    }
}
