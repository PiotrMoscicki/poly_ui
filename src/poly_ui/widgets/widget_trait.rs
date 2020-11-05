use std::{
    rc::Rc,
    cell::{Ref, RefCell, RefMut},
    fmt::Debug,
};
use uuid::Uuid;

use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::layouts::LayoutTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait WidgetTrait: Debug {
    fn id(&self) -> &Uuid;

    fn hierarchy(&self) -> Ref<Hierarchy>;
    fn hierarchy_mut(&mut self) -> RefMut<Hierarchy>;

    fn set_layout(&mut self, layout: Rc<RefCell<dyn LayoutTrait>>);
    fn layout(&self) -> Ref<dyn LayoutTrait>;
    fn layout_mut(&mut self) -> RefMut<dyn LayoutTrait>;

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

//************************************************************************************************
pub fn update_children(hierarchy: &Hierarchy, dt: f32) {
    for child in hierarchy.children() {
        child.borrow_mut().update(dt);
    }
}

pub fn paint_children(
    hierarchy: &Hierarchy,
    layout: &dyn LayoutTrait,
    parent_canvas: &mut dyn PainterTrait,
) {
    for child in hierarchy.children() {
        let borrowed_child = child.borrow();

        let mut sub_canvas = parent_canvas.sub_painter(&layout.transform(borrowed_child.id()));
        borrowed_child.paint(&mut *sub_canvas);
    }
}
