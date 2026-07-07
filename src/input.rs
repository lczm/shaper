use macroquad::prelude::*;

// macroquad input is polling
// per frame, we just call all the keys we want to track and pass it down
// to the game to interpret
pub struct Input {
    pub arrow_up: bool,
    pub arrow_down: bool,
    pub arrow_left: bool,
    pub arrow_right: bool,
    pub shift_pressed: bool,
    pub z_pressed: bool,
    pub tilde_pressed: bool,
    pub space_pressed: bool,
    pub escape_pressed: bool,

    // mouse and left click
    pub mouse: Vec2,
    pub primary_pressed: bool,
}

impl Input {
    // gather the inputs that the game uses
    pub fn gather(camera: &Camera2D) -> Self {
        Input {
            arrow_up: is_key_down(KeyCode::Up),
            arrow_down: is_key_down(KeyCode::Down),
            arrow_left: is_key_down(KeyCode::Left),
            arrow_right: is_key_down(KeyCode::Right),
            shift_pressed: is_key_pressed(KeyCode::LeftShift)
                || is_key_pressed(KeyCode::RightShift),
            z_pressed: is_key_pressed(KeyCode::Z),
            tilde_pressed: is_key_pressed(KeyCode::GraveAccent),
            space_pressed: is_key_pressed(KeyCode::Space),
            escape_pressed: is_key_pressed(KeyCode::Escape),
            mouse: camera.screen_to_world(mouse_position().into()),
            primary_pressed: is_mouse_button_pressed(MouseButton::Left),
        }
    }
}
