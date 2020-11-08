use nalgebra::Point2;
use nalgebra::Vector2;
use std::cell::{Ref, RefMut};
use uuid::Uuid;

use super::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait WindowTrait {
    fn widget(&self) -> Ref<dyn WidgetTrait>;
    fn widget_mut(&mut self) -> RefMut<dyn WidgetTrait>;

    fn id(&self) -> &Uuid;

    fn pos(&self) -> Point2<i32>;
    fn set_pos(&mut self, new: Point2<i32>);
    fn size(&self) -> Vector2<u32>;
    fn set_size(&mut self, new: Vector2<u32>);

    fn update(&mut self, dt: f32);
    fn paint(&mut self);
}
