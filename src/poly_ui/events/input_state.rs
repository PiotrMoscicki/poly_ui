// deps
use enum_map::EnumMap;
use nalgebra::Point2;
use nalgebra::Vector2;
// super
use super::KeyState;
use super::KeyboardKey;
use super::MouseButton;

pub struct InputState {
    pub mouse_pos: Point2<u32>,
    pub mouse_diff: Vector2<u32>,

    pub current_keyboard_key_state: EnumMap<KeyboardKey, KeyState>,
    pub previous_keyboard_key_state: EnumMap<KeyboardKey, KeyState>,

    pub current_mouse_button_state: EnumMap<MouseButton, KeyState>,
    pub previous_mouse_button_state: EnumMap<MouseButton, KeyState>,
}

impl InputState {
    // keyboard keys

    pub fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        self.current_keyboard_key_state[key] == KeyState::Pressed
    }

    pub fn was_key_just_pressed(&self, key: KeyboardKey) -> bool {
        self.is_key_pressed(key) && self.previous_keyboard_key_state[key] == KeyState::Released
    }

    // mouse buttons

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.current_mouse_button_state[button] == KeyState::Pressed
    }

    pub fn was_button_just_pressed(&self, button: MouseButton) -> bool {
        self.is_button_pressed(button)
            && self.previous_mouse_button_state[button] == KeyState::Released
    }
}
