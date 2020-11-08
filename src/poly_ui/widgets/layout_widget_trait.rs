// std
use std::{
    boxed::Box,
    rc::Rc,
};
// crate
use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::app::PainterTrait;
// super
use super::WidgetTrait;
use super::Ownerless;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait LayoutWidgetTrait: WidgetTrait {
    fn add(&mut self, child: Ownerless<dyn WidgetTrait>) -> Rc<Box<dyn WidgetTrait>>;
    fn remove(&mut self, child: Rc<Box<dyn WidgetTrait>>) -> Ownerless<dyn WidgetTrait>;
}

//************************************************************************************************
pub fn update_children(hierarchy: &Hierarchy, dt: f32) {
    for child in hierarchy.children() {
        child.borrow_mut().update(dt);
    }
}

pub fn paint_children(
    hierarchy: &Hierarchy,
    parent_canvas: &mut dyn PainterTrait,
) {
    // for child in hierarchy.children() {
    //     let borrowed_child = child.borrow();

    //     let mut sub_canvas = parent_canvas
    //         .sub_painter(&layout.transform(&parent_canvas.size(), borrowed_child.id()));
    //     borrowed_child.paint(&mut *sub_canvas);
    // }
}