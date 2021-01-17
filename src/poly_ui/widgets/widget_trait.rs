// std
use std::fmt::Debug;
use uuid::Uuid;
// crate
use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::components::Hierarchy;
// super
use super::Ownerless;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait WidgetTrait: Debug {
    fn id(&self) -> &Uuid;

    // child widgets
    fn add_child(&mut self, child: Ownerless);
    fn remove_child(&mut self, child: &Uuid) -> Ownerless;

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
        self.id() == other.id()
    }
}

//************************************************************************************************
impl std::cmp::Eq for dyn WidgetTrait {}


//************************************************************************************************
pub fn update_children(hierarchy: &Hierarchy, dt: f32) {
    for child in hierarchy.children() {
        child.get().borrow_mut().update(dt);
    }
}

//************************************************************************************************
pub fn paint_children(hierarchy: &Hierarchy, parent_canvas: &mut dyn PainterTrait) {
    for child in hierarchy.children() {
        //let borrowed_child = child.get().borrow();
        //let mut sub_canvas =
        //    parent_canvas.sub_painter(&Transform::new(borrowed_child.pos(), borrowed_child.size()));
        //borrowed_child.paint(&mut *sub_canvas);
    }
}