use nalgebra::Point2;
use nalgebra::Vector2;
use std::{
    cell::{Ref, RefMut},
    fmt::Debug,
    boxed::Box,
};
use uuid::Uuid;

use crate::poly_ui::app::CanvasTrait;
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

    fn update(&mut self, dt: f32);
    fn paint(&self, canvas: &mut dyn CanvasTrait);
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

//************************************************************************************************
pub fn update_children(hierarchy: &Hierarchy, dt: f32) {
    for child in hierarchy.children() {
        child.borrow_mut().update(dt);
    }
}

pub fn paint_children(hierarchy: &Hierarchy, parent_canvas: &mut dyn CanvasTrait) {
    for child in hierarchy.children() {
        let mut mut_child = child.borrow_mut();
        let mut sub_canvas = parent_canvas.sub_canvas(mut_child.pos(), mut_child.size());
        mut_child.paint(&mut *sub_canvas);
    }
}