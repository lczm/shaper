use macroquad::prelude::*;

use crate::arena::Arena;
use crate::constants::{HEIGHT, WIDTH};
use crate::input::Input;
use crate::state::GameState;
use crate::ui::Ui;

// owns the per-frame plumbing (camera, timing, input) and the top-level pieces
// (arena, stage, ui, state), and orchestrates the frame.
pub struct World {
    last_time: f64,
    dt: f32,
    // camera maps logical rect (which can be affected by screen dpi) onto physical screen
    // drawing uses all logical space rect coordinates and camera converts
    camera: Camera2D,
    arena: Arena,
    state: GameState,
    ui: Ui,
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
            camera,
            arena: Arena::new(),
            state: GameState::new(),
            ui: Ui::new(),
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
            self.update();
            self.draw();
            next_frame().await;
        }
    }

    fn update(&mut self) {
        self.compute_dt();
        // gather input here since World owns the camera (mouse -> world)
        let input = Input::gather(&self.camera);
        self.arena.update(self.dt, &input, &mut self.state);
    }

    fn draw(&self) {
        set_camera(&self.camera);
        clear_background(LIGHTGRAY);

        self.arena.draw();

        self.ui.draw(&self.state, self.arena.bounds());
    }
}

impl Default for World {
    fn default() -> Self {
        World::new()
    }
}
