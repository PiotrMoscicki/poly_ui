// deps
use nalgebra::Point2;
use enum_map::EnumMap;
// super
use super::Key;
use super::MouseButton;

pub struct InputState {
    mouse_pos: Point2<u32>,
    current_key_press_state: Vec<EnumMap<Key, bool>>
    previous_key_press_state: Vec<EnumMap<Key, bool>>
    current_mouse_buttons: Vec<EnumMap<>>
}

impl InputState {
    
}