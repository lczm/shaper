use macroquad::prelude::*;

use crate::constants::{HEIGHT, WIDTH};
use crate::input::Input;
use crate::stage::Stage;

pub struct World {
    last_time: f64,
    dt: f32,
    stage: Stage,
    // camera maps logical rect (which can be affected by screen dpi) onto physical screen
    // drawing uses all logical space rect coordinates and camera converts
    camera: Camera2D,
}

impl World {
    pub fn new() -> Self {
        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, WIDTH, HEIGHT));
        // flip it upside down so (0, 0) is top left
        // and (WIDTH, HEIGHT) is bottom right
        camera.zoom.y = -camera.zoom.y;

        World {
            last_time: get_time(),
            dt: 0.0,
            stage: Stage::new(),
            camera,
        }
    }

    // refresh dt from the time elapsed since the previous frame
    fn compute_dt(&mut self) {
        let now = get_time();
        self.dt = (now - self.last_time) as f32;
        self.last_time = now;
    }

    // step per frame and gets next frame until window closes
    pub async fn run(mut self) {
        loop {
            self.step();
            next_frame().await;
        }
    }

    fn step(&mut self) {
        self.compute_dt();
        self.update();
        self.draw();
    }

    // update game state prior to draw
    fn update(&mut self) {
        // gather input here since the World owns the camera (mouse -> world)
        let input = Input::gather(&self.camera);
        self.stage.update(self.dt, &input);
    }

    fn draw(&self) {
        set_camera(&self.camera);

        clear_background(LIGHTGRAY);
        self.stage.draw();
    }
}

impl Default for World {
    fn default() -> Self {
        World::new()
    }
}
