// deps
use enum_map::{enum_map, EnumMap};
use nalgebra::Point2;
use nalgebra::Vector2;
// super
use super::KeyState;
use super::KeyboardKey;
use super::MouseButton;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub struct InputState {
    pub mouse_pos: Point2<u32>,
    pub mouse_diff: Vector2<i32>,

    pub current_keyboard_key_state: EnumMap<KeyboardKey, KeyState>,
    pub previous_keyboard_key_state: EnumMap<KeyboardKey, KeyState>,

    pub current_mouse_button_state: EnumMap<MouseButton, KeyState>,
    pub previous_mouse_button_state: EnumMap<MouseButton, KeyState>,
}

//************************************************************************************************
impl InputState {
    // update

    pub fn update(
        &mut self,
        new_keyboard_key_state: EnumMap<KeyboardKey, KeyState>,
        new_mouse_button_state: EnumMap<MouseButton, KeyState>,
    ) {
        
        self.previous_keyboard_key_state = self.current_keyboard_key_state;
        self.current_keyboard_key_state = new_keyboard_key_state;
        self.previous_mouse_button_state = self.current_mouse_button_state;
        self.current_mouse_button_state = new_mouse_button_state;
    }

    // keyboard keys

    pub fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        self.current_keyboard_key_state[key] == KeyState::Pressed
    }

    pub fn is_key_released(&self, key: KeyboardKey) -> bool {
        self.current_keyboard_key_state[key] == KeyState::Released
    }

    pub fn was_key_just_pressed(&self, key: KeyboardKey) -> bool {
        self.is_key_pressed(key) && self.previous_keyboard_key_state[key] == KeyState::Released
    }

    pub fn was_key_just_released(&self, key: KeyboardKey) -> bool {
        self.is_key_released(key) && self.previous_keyboard_key_state[key] == KeyState::Pressed
    }

    // mouse buttons

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.current_mouse_button_state[button] == KeyState::Pressed
    }

    pub fn is_button_released(&self, button: MouseButton) -> bool {
        self.current_mouse_button_state[button] == KeyState::Released
    }

    pub fn was_button_just_pressed(&self, button: MouseButton) -> bool {
        self.is_button_pressed(button)
            && self.previous_mouse_button_state[button] == KeyState::Released
    }

    pub fn was_button_just_released(&self, button: MouseButton) -> bool {
        self.is_button_released(button)
            && self.previous_mouse_button_state[button] == KeyState::Pressed
    }
}

//************************************************************************************************
impl Default for InputState {
    fn default() -> Self {
        Self {
            mouse_pos: Point2::<u32>::new(0, 0),
            mouse_diff: Vector2::<i32>::new(0, 0),
            current_keyboard_key_state: enum_map!(_ => KeyState::Released),
            previous_keyboard_key_state: enum_map!(_ => KeyState::Released),
            current_mouse_button_state: enum_map!(_ => KeyState::Released),
            previous_mouse_button_state: enum_map!(_ => KeyState::Released),
        }
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[cfg(test)]
mod tests {
    // super
    use super::*;

    //********************************************************************************************
    #[test]
    fn mouse_diff() {
        let input_state = InputState::default();
    }
}
