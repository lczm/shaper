use macroquad::prelude::*;

use super::clone_camera;
use crate::{
    constants::{
        SHAKE_DECAY, SHAKE_FREQUENCY, SHAKE_MAX_ANGLE, SHAKE_MAX_OFFSET, SHAKE_MIN_TRAUMA,
    },
    utils::noise,
};

// https://bevy.org/examples/camera/2d-screen-shake/
// trauma based screen shake
pub struct Shake {
    trauma: f32,
    // total elapsed time
    time: f32,
}

impl Shake {
    pub fn new() -> Self {
        Shake {
            trauma: 0.0,
            time: 0.0,
        }
    }

    // bump trauma up (clamped to 1.0), e.g. when the player takes a hit
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).clamp(0.0, 1.0);
    }

    // decay towards 0 and advance the noise clock every frame
    pub fn update(&mut self, dt: f32) {
        self.trauma = (self.trauma - SHAKE_DECAY * dt).max(0.0);
        self.time += dt;
    }

    pub fn is_active(&self) -> bool {
        self.trauma > SHAKE_MIN_TRAUMA
    }

    // rebuild the camera and apply shake
    pub fn apply(&self, camera: &Camera2D) -> Camera2D {
        // quadratic falloff feels snappier than scaling with raw trauma
        let shake = self.trauma * self.trauma;
        let t = self.time * SHAKE_FREQUENCY;
        // distinct seeds per channel so the two axes and the tilt don't correlate
        let offset = vec2(
            SHAKE_MAX_OFFSET * shake * noise(0, t),
            SHAKE_MAX_OFFSET * shake * noise(1, t),
        );
        let angle = SHAKE_MAX_ANGLE * shake * noise(2, t);

        let mut shaken = clone_camera(camera);
        shaken.target += offset;
        shaken.rotation += angle;
        shaken
    }
}

impl Default for Shake {
    fn default() -> Self {
        Shake::new()
    }
}
