use nalgebra::Point2;
use nalgebra::Vector2;
use std::{
    cell::{Ref, RefMut},
    fmt::Debug,
};
use uuid::Uuid;

use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::layouts::Layout;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait WidgetTrait: Debug {
    fn id(&self) -> &Uuid;

    fn pos(&self) -> Point2<i32>;
    fn set_pos(&mut self, new: Point2<i32>);
    fn size(&self) -> Vector2<u32>;
    fn set_size(&mut self, new: Vector2<u32>);

    fn hierarchy(&self) -> Ref<Hierarchy>;
    fn hierarchy_mut(&mut self) -> RefMut<Hierarchy>;

    fn set_layout(&mut self, layout: Box<dyn Layout>);
    fn layout(&self) -> &dyn Layout;
    fn layout_mut(&mut self) -> &mut dyn Layout;
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