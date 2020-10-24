use nalgebra::Vector2;

use crate::poly_ui::components::Transform;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait WindowProviderTrait {
    fn transform(&self) -> &Transform;
    fn set_transform(&mut self, new: &Transform);
}

//************************************************************************************************
pub fn set_window_pos(&mut dyn window_prvider: WindowProviderTrait, new: Vector2) {
    let transform = window_prvider.transform();
    transform.pos = new;
    window_provider.set_transform(transform);
}

//************************************************************************************************
pub fn set_window_size(&mut dyn window_provider: WindowProviderTrait, new: Vector2) {
    let transform = window_prvider.transform();
    transform.size = new;
    window_provider.set_transform(transform);
}