use macroquad::prelude::*;

pub struct World {
    last_time: f64,
    dt: f32,
}

impl World {
    pub fn new() -> Self {
        World {
            last_time: get_time(),
            dt: 0.0,
        }
    }

    // delta time since previous frame
    pub fn dt(&self) -> f32 {
        self.dt
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
    fn update(&mut self) {}

    /// Render the current frame.
    fn draw(&self) {
        clear_background(BLACK);
    }
}

impl Default for World {
    fn default() -> Self {
        World::new()
    }
}
