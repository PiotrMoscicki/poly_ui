use nalgebra::Vector2;
use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Debug,
    rc::Rc,
};
use uuid::Uuid;

use super::widget_trait::WidgetTrait;
use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::layouts::{CanvasLayout, Layout};

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Widget {
    id: Uuid,
    pos: Vector2<i32>,
    hierarchy: Rc<RefCell<Hierarchy>>,
    layout: Box<dyn Layout>,
}

//************************************************************************************************
impl Widget {
    pub fn new() -> Self {
        return Self {
            id: Uuid::new_v4(),
            pos: Vector2::<i32>::new(0, 0),
            hierarchy: Rc::new(RefCell::new(Hierarchy::new())),
            layout: Box::new(CanvasLayout::new()),
        };
    }
}

//************************************************************************************************
impl WidgetTrait for Widget {
    fn id(&self) -> &Uuid {
        return &self.id;
    }

    fn pos(&self) -> &Vector2<i32> {
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
