use macroquad::prelude::*;

use crate::arena::Arena;
use crate::constants::{BACKGROUND, HEIGHT, SHAKE_TRAUMA_PER_HIT, WIDTH};
use crate::dev_ui;
use crate::gfx::{Post, Shaders, Shake};
use crate::input::Input;
use crate::state::{GameEvent, GameState};
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
    // GPU materials, loaded once and shared across the scene
    shaders: Shaders,
    // full-screen post-process pipeline (dormant until a pass is enabled)
    post: Post,
    // camera screen shake, fed trauma when the player gets hit
    shake: Shake,
    // egui debug window, toggled with spacebar
    dev_ui: bool,
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
            shaders: Shaders::new(),
            post: Post::new(),
            shake: Shake::new(),
            dev_ui: false,
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

        // decay/advance the shake before applying this frame's events: a fresh
        // hit then renders at full trauma. decaying afterwards would clip the
        // opening kick by one frame's worth of decay.
        self.shake.update(self.dt);

        if is_key_pressed(KeyCode::Space) {
            self.dev_ui = !self.dev_ui;
        }

        // gather input here since World owns the camera (mouse -> world)
        let input = Input::gather(&self.camera);
        self.arena.update(self.dt, &input, &mut self.state);

        // the game itself might emit some game events,
        // like player hit, react to them here
        for event in self.state.events.drain(..) {
            match event {
                GameEvent::PlayerHit => {
                    self.state.lives = self.state.lives.saturating_sub(1);
                    self.arena.player_mut().register_hit();
                    self.shake.add_trauma(SHAKE_TRAUMA_PER_HIT);
                }
            }
        }
    }

    fn draw(&self) {
        // update the camera only if there is some shake effect
        let shaken = self
            .shake
            .is_active()
            .then(|| self.shake.apply(&self.camera));
        let camera = shaken.as_ref().unwrap_or(&self.camera);

        // if post processing pipeline is active, then pass it through the
        // offscreen rendering, otherwise go straight to screen
        if self.post.active() {
            self.post.begin(camera);
            self.arena.draw(&self.state, &self.shaders);
            self.post.present(&self.shaders);
        } else {
            set_camera(camera);
            clear_background(BACKGROUND);
            self.arena.draw(&self.state, &self.shaders);
        }

        // UI + egui always draw to the screen on top, unaffected by post
        self.ui.draw(&self.state, self.arena.bounds());

        // always render dev ui on top of everything else
        if self.dev_ui {
            dev_ui::draw(&self.state);
        }
    }
}

impl Default for World {
    fn default() -> Self {
        World::new()
    }
}
