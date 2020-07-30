use nalgebra::Vector3;
use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Debug,
    rc::Rc,
};
use uuid::Uuid;

use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::layouts::{CanvasLayout, Layout};

// traits
//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait Widget: Debug {
    fn id(&self) -> &Uuid;
    fn pos(&self) -> &Vector3<i32>;
    fn hierarchy(&self) -> Ref<Hierarchy>;
    fn hierarchy_mut(&mut self) -> RefMut<Hierarchy>;
    fn set_layout(&mut self, layout: Box<dyn Layout>);
    fn layout(&self) -> &dyn Layout;
    fn layout_mut(&mut self) -> &mut dyn Layout;
}

impl std::hash::Hash for dyn Widget {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl std::cmp::PartialEq for dyn Widget {
    fn eq(&self, other: &Self) -> bool {
        return self.id() == other.id();
    }
}

impl std::cmp::Eq for dyn Widget {}

// impl
//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct BaseWidget {
    id: Uuid,
    pos: Vector3<i32>,
    hierarchy: Rc<RefCell<Hierarchy>>,
    layout: Box<dyn Layout>,
}

impl BaseWidget {
    pub fn new() -> Self {
        return Self {
            id: Uuid::new_v4(),
            pos: Vector3::<i32>::new(0, 0, 0),
            hierarchy: Rc::new(RefCell::new(Hierarchy::new())),
            layout: Box::new(CanvasLayout::new()),
        };
    }
}

impl Widget for BaseWidget {
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

// tests
//********************************************************************************************
//********************************************************************************************
//********************************************************************************************
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::poly_ui::widgets::{BaseWidget, Widget};

    //****************************************************************************************
    #[test]
    fn widget_add_child() {
        let mut parent_widget = BaseWidget::new();
        let child_widget = Rc::new(RefCell::new(BaseWidget::new()));
        parent_widget.hierarchy_mut().add(child_widget.clone());
        assert_eq!(
            parent_widget.hierarchy().children()[0].borrow().id(),
            child_widget.borrow().id()
        );
    }

    //****************************************************************************************
    #[test]
    fn widget_remove_child() {
        let mut parent_widget = BaseWidget::new();
        let child_widget_1 = Rc::new(RefCell::new(BaseWidget::new())) as Rc<RefCell<dyn Widget>>;
        let child_widget_2 = Rc::new(RefCell::new(BaseWidget::new())) as Rc<RefCell<dyn Widget>>;
        parent_widget.hierarchy_mut().add(child_widget_1.clone());
        parent_widget.hierarchy_mut().add(child_widget_2.clone());
        parent_widget.hierarchy_mut().remove(&child_widget_1);
        assert_eq!(
            parent_widget.hierarchy().children()[0].borrow().id(),
            child_widget_2.borrow().id()
        );
    }
}
