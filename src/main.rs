mod constants;
mod world;

use macroquad::prelude::*;
use world::World;

fn window_conf() -> Conf {
    Conf {
        window_title: "Shaper".to_owned(),
        window_width: constants::WIDTH as i32,
        window_height: constants::HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    World::new().run().await;
}
