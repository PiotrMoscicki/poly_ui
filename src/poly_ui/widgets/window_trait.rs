// std
use std::{
    rc::Rc,
    cell::RefCell
};
// deps
use nalgebra::Point2;
use nalgebra::Vector2;
use uuid::Uuid;
// crate
use crate::poly_ui::layouts::CanvasLayout;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait WindowTrait {
    fn widget(&self) -> &Rc<RefCell<CanvasLayout>>;

    fn id(&self) -> &Uuid;

    fn pos(&self) -> Point2<i32>;
    fn set_pos(&mut self, new: Point2<i32>);
    fn size(&self) -> Vector2<u32>;
    fn set_size(&mut self, new: Vector2<u32>);

    fn update(&mut self, dt: f32);
    fn paint(&mut self);
}
