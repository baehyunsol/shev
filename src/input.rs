use macroquad::input::{
    KeyCode,
    MouseButton,
    get_keys_down,
    get_keys_pressed,
    get_keys_released,
    is_mouse_button_down,
    is_mouse_button_pressed,
    is_mouse_button_released,
    mouse_position,
    mouse_wheel,
};
use std::collections::HashSet;

pub struct Input {
    pub mouse_pos: (f32, f32),
    pub mouse_wheel: (f32, f32),
    pub mouse_down: [bool; 3],
    pub mouse_pressed: [bool; 3],
    pub mouse_released: [bool; 3],
    pub down_keys: HashSet<KeyCode>,
    pub pressed_keys: HashSet<KeyCode>,
    pub released_keys: HashSet<KeyCode>,
}

pub fn get_input() -> Input {
    Input {
        mouse_pos: mouse_position(),
        mouse_wheel: mouse_wheel(),
        mouse_down: [
            is_mouse_button_down(MouseButton::Left),
            is_mouse_button_down(MouseButton::Middle),
            is_mouse_button_down(MouseButton::Right),
        ],
        mouse_pressed: [
            is_mouse_button_pressed(MouseButton::Left),
            is_mouse_button_pressed(MouseButton::Middle),
            is_mouse_button_pressed(MouseButton::Right),
        ],
        mouse_released: [
            is_mouse_button_released(MouseButton::Left),
            is_mouse_button_released(MouseButton::Middle),
            is_mouse_button_released(MouseButton::Right),
        ],
        down_keys: get_keys_down(),
        pressed_keys: get_keys_pressed(),
        released_keys: get_keys_released(),
    }
}
