// std
use std::{
    cell::RefCell,
    rc::Rc,
    fmt::Debug,
};
// deps
use nalgebra::Point2;
use nalgebra::Vector2;
use uuid::Uuid;
//crate
use crate::poly_ui::app::PainterTrait;
// super
use super::Ownerless;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait WidgetTrait: Debug {
    fn id(&self) -> &Uuid;

    // transform
    fn pos(&self) -> &Point2<i32>;
    fn set_pos(&mut self, value: &Point2<i32>);
    fn size(&self) -> &Vector2<u32>;
    fn set_size(&mut self, value: &Vector2<u32>);
    
    // child widgets
    fn add(&mut self, child: Ownerless);
    fn remove(&mut self, child: &Rc<RefCell<dyn WidgetTrait>>) -> Ownerless;

    fn update(&mut self, dt: f32);
    fn paint(&self, canvas: &mut dyn PainterTrait);
}

//************************************************************************************************
impl std::hash::Hash for dyn WidgetTrait {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

//************************************************************************************************
impl std::cmp::PartialEq for dyn WidgetTrait {
    fn eq(&self, other: &Self) -> bool {
        return self.id() == other.id();
    }
}

//************************************************************************************************
impl std::cmp::Eq for dyn WidgetTrait {}