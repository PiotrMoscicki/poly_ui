use nalgebra::Point2;
use nalgebra::Vector2;
use std::fmt::Debug;

use crate::poly_ui::app::CanvasTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait WindowProviderTrait: Debug {
    fn canvas(&mut self) -> Box<dyn CanvasTrait>;

    fn pos(&self) -> Point2<i32>;
    fn set_pos(&mut self, new: Point2<i32>);
    fn size(&self) -> Vector2<u32>;
    fn set_size(&mut self, new: Vector2<u32>);
}
