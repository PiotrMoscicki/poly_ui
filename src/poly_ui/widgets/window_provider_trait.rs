use std::fmt::Debug;
use nalgebra::Vector2;

use crate::poly_ui::components::Transform;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait WindowProviderTrait: Debug {
    fn transform(&self) -> Transform;
    fn set_transform(&mut self, new: Transform);
}

//************************************************************************************************
pub fn set_window_pos(window_provider: &mut dyn WindowProviderTrait, new: Vector2<i32>) {
    let old_transform = window_provider.transform();
    let new_transform = Transform {
        pos: new,
        size: old_transform.size,
    };
    window_provider.set_transform(new_transform);
}

//************************************************************************************************
pub fn set_window_size(window_provider: &mut dyn WindowProviderTrait, new: Vector2<i32>) {
    let old_transform = window_provider.transform();
    let new_transform = Transform {
        pos: old_transform.pos,
        size: new,
    };
    window_provider.set_transform(new_transform);
}