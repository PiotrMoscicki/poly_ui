// std
use std::fmt::Debug;
// deps
use nalgebra::Point2;
use nalgebra::Vector2;
// super
use super::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************

pub trait WindowProviderTrait: Debug {
    fn paint_widget(&mut self, widget: &mut dyn WidgetTrait);

    fn pos(&self) -> Point2<i32>;
    fn set_pos(&mut self, new: Point2<i32>);
    fn size(&self) -> Vector2<u32>;
    fn set_size(&mut self, new: Vector2<u32>);
}
