mod arena;
mod bomb;
mod boss;
mod collision;
mod constants;
mod dev_ui;
mod gfx;
mod input;
mod level_window;
mod player;
mod projectile;
mod shape;
mod state;
mod ui;
mod utils;
mod world;

use macroquad::prelude::*;
use world::World;

fn window_conf() -> Conf {
    Conf {
        window_title: "Shaper".to_owned(),
        window_width: constants::WIDTH as i32,
        window_height: constants::HEIGHT as i32,
        // rendering scaled for high dpi
        high_dpi: true,
        // msaa antialiasing
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // seed the rng from the current time so each run is different
    macroquad::rand::srand(macroquad::miniquad::date::now() as u64);

    World::new().run().await;
}
