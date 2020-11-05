use nalgebra::Point2;
use nalgebra::Vector2;
use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};
use uuid::Uuid;

use super::LayoutTrait;
use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::components::Transform;
use crate::poly_ui::widgets::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct CanvasLayout {
    children: HashMap<Uuid, Transform>,
    hierarchy: Rc<RefCell<Hierarchy>>,
}

//************************************************************************************************
impl CanvasLayout {
    pub fn new() -> Self {
        return Self {
            children: HashMap::new(),
            hierarchy: Rc::new(RefCell::new(Hierarchy::new())),
        };
    }

    pub fn set_pos(&mut self, widget_id: &Uuid, new: &Point2<i32>) {
        self.children.get_mut(&widget_id).as_mut().unwrap().pos = *new;
    }

    pub fn set_size(&mut self, widget_id: &Uuid, new: &Vector2<u32>) {
        self.children.get_mut(&widget_id).as_mut().unwrap().size = *new;
    }
}

//************************************************************************************************
impl LayoutTrait for CanvasLayout {
    fn set_owner_widget_hierarchy(&mut self, hierarchy: Rc<RefCell<Hierarchy>>) {
        self.hierarchy = hierarchy;
    }

    fn add(&mut self, child: Rc<RefCell<dyn WidgetTrait>>) {
        self.children.insert(
            *child.borrow().id(),
            Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(0, 0)),
        );
        self.hierarchy.borrow_mut().add(child);
    }

    fn transform(&self, widget_id: &Uuid) -> Transform {
        return *self.children.get(&widget_id).unwrap();
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::poly_ui::layouts::CanvasLayout;
    use crate::poly_ui::widgets::{Widget, WidgetTrait};

    //********************************************************************************************
    #[test]
    fn canvas_layout_add_child() {
        let mut parent_widget = Widget::new();
        parent_widget.set_layout(Box::new(CanvasLayout::new()));
        let child_widget = Rc::new(RefCell::new(Widget::new()));
        parent_widget.layout_mut().add(child_widget.clone());

        assert_eq!(
            parent_widget.hierarchy().children()[0].borrow().id(),
            child_widget.borrow().id()
        );
    }
}
