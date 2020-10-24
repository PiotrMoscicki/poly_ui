use nalgebra::Vector3;
use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Debug,
    rc::Rc,
};
use uuid::Uuid;

use super::widget::Widget;
use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::layouts::{CanvasLayout, Layout};

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Base {
    id: Uuid,
    pos: Vector3<i32>,
    hierarchy: Rc<RefCell<Hierarchy>>,
    layout: Box<dyn Layout>,
}

//************************************************************************************************
impl Base {
    pub fn new() -> Self {
        return Self {
            id: Uuid::new_v4(),
            pos: Vector3::<i32>::new(0, 0, 0),
            hierarchy: Rc::new(RefCell::new(Hierarchy::new())),
            layout: Box::new(CanvasLayout::new()),
        };
    }
}

//************************************************************************************************
impl Widget for Base {
    fn id(&self) -> &Uuid {
        return &self.id;
    }

    fn pos(&self) -> &Vector3<i32> {
        return &self.pos;
    }

    fn hierarchy(&self) -> Ref<Hierarchy> {
        return self.hierarchy.borrow();
    }

    fn hierarchy_mut(&mut self) -> RefMut<Hierarchy> {
        return self.hierarchy.borrow_mut();
    }

    fn set_layout(&mut self, layout: Box<dyn Layout>) {
        self.layout = layout;
        self.layout
            .set_owner_widget_hierarchy(self.hierarchy.clone());
    }

    fn layout(&self) -> &dyn Layout {
        return self.layout.as_ref();
    }

    fn layout_mut(&mut self) -> &mut dyn Layout {
        return self.layout.as_mut();
    }
}
