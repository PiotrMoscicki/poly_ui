// deps
use enum_map::EnumMap;
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
    pub mouse_diff: Vector2<u32>,

    pub current_keyboard_key_state: EnumMap<KeyboardKey, KeyState>,
    pub previous_keyboard_key_state: EnumMap<KeyboardKey, KeyState>,

    pub current_mouse_button_state: EnumMap<MouseButton, KeyState>,
    pub previous_mouse_button_state: EnumMap<MouseButton, KeyState>,
}

//************************************************************************************************
impl InputState {
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
//************************************************************************************************
//************************************************************************************************
#[cfg(test)]
mod tests {
    // crate
    use crate::poly_ui::app::MockPainter;
    // super
    use super::*;

    //********************************************************************************************
    #[test]
    fn update() {
        let mock = MockWidget::new();

        mock.borrow_mut().update(0.0);
        assert_eq!(mock.borrow().update_call_count, 1);
    }

    //********************************************************************************************
    #[test]
    fn paint() {
        let mock = MockWidget::new();
        let mut painter = MockPainter::default();

        mock.borrow_mut().paint(&mut painter);
        mock.borrow_mut().paint(&mut painter);
        mock.borrow_mut().paint(&mut painter);
        assert_eq!(mock.borrow().paint_call_count, 3);
    }
}
