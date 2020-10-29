use nalgebra::Point2;
use nalgebra::Vector2;
use std::fmt::Debug;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait WindowProviderTrait: Debug {
    fn pos(&self) -> Point2<i32>;
    fn set_pos(&mut self, new: Point2<i32>);
    fn size(&self) -> Vector2<u32>;
    fn set_size(&mut self, new: Vector2<u32>);
}
