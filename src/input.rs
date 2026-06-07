use macroquad::prelude::*;

// macroquad input is polling
// per frame, we just call all the keys we want to track and pass it down
// to the game to interpret
pub struct Input {
    pub arrow_up: bool,
    pub arrow_down: bool,
    pub arrow_left: bool,
    pub arrow_right: bool,
    pub shift: bool,

    // mouse and left click
    pub mouse: Vec2,
    pub primary_pressed: bool,
}

impl Input {
    /// Read the current input. `camera` is needed to convert the physical mouse
    /// pixels back into logical world space (we render with high_dpi).
    pub fn gather(camera: &Camera2D) -> Self {
        Input {
            arrow_up: is_key_down(KeyCode::Up),
            arrow_down: is_key_down(KeyCode::Down),
            arrow_left: is_key_down(KeyCode::Left),
            arrow_right: is_key_down(KeyCode::Right),
            shift: is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift),
            mouse: camera.screen_to_world(mouse_position().into()),
            primary_pressed: is_mouse_button_pressed(MouseButton::Left),
        }
    }
}
