use macroquad::prelude::*;

// macroquad input is polling
// per frame, we just call all the keys we want to track and pass it down
// to the game to interpret
pub struct Input {
    pub arrow_up: bool,
    pub arrow_down: bool,
    pub arrow_left: bool,
    pub arrow_right: bool,
    pub w_down: bool,
    pub a_down: bool,
    pub s_down: bool,
    pub d_down: bool,
    pub shift_pressed: bool,
    pub z_pressed: bool,
    pub l_pressed: bool,
    pub tilde_pressed: bool,
    pub tab_pressed: bool,
    pub space_pressed: bool,
    pub escape_pressed: bool,
    pub f1_pressed: bool,
    pub f2_pressed: bool,
    pub number1_pressed: bool,
    pub number2_pressed: bool,
    pub number3_pressed: bool,

    // mouse and left click
    pub mouse: Vec2,
    pub screen_mouse: Vec2,
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
            w_down: is_key_down(KeyCode::W),
            a_down: is_key_down(KeyCode::A),
            s_down: is_key_down(KeyCode::S),
            d_down: is_key_down(KeyCode::D),
            shift_pressed: is_key_pressed(KeyCode::LeftShift)
                || is_key_pressed(KeyCode::RightShift),
            z_pressed: is_key_pressed(KeyCode::Z),
            l_pressed: is_key_pressed(KeyCode::L),
            tilde_pressed: is_key_pressed(KeyCode::GraveAccent),
            tab_pressed: is_key_pressed(KeyCode::Tab),
            space_pressed: is_key_pressed(KeyCode::Space),
            escape_pressed: is_key_pressed(KeyCode::Escape),
            f1_pressed: is_key_pressed(KeyCode::F1),
            f2_pressed: is_key_pressed(KeyCode::F2),
            number1_pressed: is_key_pressed(KeyCode::Key1),
            number2_pressed: is_key_pressed(KeyCode::Key2),
            number3_pressed: is_key_pressed(KeyCode::Key3),
            mouse: camera.screen_to_world(mouse_position().into()),
            screen_mouse: mouse_position().into(),
            primary_pressed: is_mouse_button_pressed(MouseButton::Left),
        }
    }
}
