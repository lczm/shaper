use macroquad::prelude::*;

use crate::constants::{
    ARENA_BORDER_THICKNESS, ARENA_MARGIN_HEIGHT, ARENA_MARGIN_WIDTH, HEIGHT, WIDTH,
};
use crate::input::Input;
use crate::stage::Stage;
use crate::state::GameState;

// the bounded arena
pub struct Arena {
    last_time: f64,
    dt: f32,
    stage: Stage,
    // camera maps logical rect (which can be affected by screen dpi) onto physical screen
    // drawing uses all logical space rect coordinates and camera converts
    camera: Camera2D,
    // rectangular border that bounds gameplay; the player can't leave it
    bounds: Rect,
}

impl Arena {
    pub fn new() -> Self {
        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, WIDTH, HEIGHT));
        // flip it upside down so (0, 0) is top left
        // and (WIDTH, HEIGHT) is bottom right
        camera.zoom.y = -camera.zoom.y;

        // offset the arena by some margin
        let height = HEIGHT - 2.0 * ARENA_MARGIN_HEIGHT;
        let width = height * 3.0 / 4.0;
        let bounds = Rect::new(ARENA_MARGIN_WIDTH, ARENA_MARGIN_HEIGHT, width, height);

        Arena {
            last_time: get_time(),
            dt: 0.0,
            stage: Stage::new(bounds),
            camera,
            bounds,
        }
    }

    // refresh dt from the time elapsed since the previous frame
    fn compute_dt(&mut self) {
        let now = get_time();
        self.dt = (now - self.last_time) as f32;
        self.last_time = now;
    }

    // the rectangular gameplay border, in logical coordinates
    pub fn bounds(&self) -> Rect {
        self.bounds
    }

    // update game state prior to draw
    pub fn update(&mut self, state: &mut GameState) {
        self.compute_dt();
        // gather input here since the arena owns the camera (mouse -> world)
        let input = Input::gather(&self.camera);
        self.stage.update(self.dt, &input, self.bounds, state);
    }

    pub fn draw(&self) {
        set_camera(&self.camera);

        clear_background(LIGHTGRAY);
        self.stage.draw();
        self.draw_border();
    }

    fn draw_border(&self) {
        draw_rectangle_lines(
            self.bounds.x,
            self.bounds.y,
            self.bounds.w,
            self.bounds.h,
            ARENA_BORDER_THICKNESS,
            BLUE,
        );
    }
}

impl Default for Arena {
    fn default() -> Self {
        Arena::new()
    }
}
