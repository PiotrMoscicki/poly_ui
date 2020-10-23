use nalgebra::Vector3;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::poly_ui::widgets::Widget;

// structs
//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Transform {
    pos: Vector3<i32>,
    size: Vector3<i32>,
}

#[derive(Debug)]
pub struct Hierarchy {
    children: Vec<Rc<RefCell<dyn Widget>>>,
}

impl Hierarchy {
    pub fn new() -> Self {
        return Self {
            children: Vec::new(),
        };
    }

    pub fn add(&mut self, child: Rc<RefCell<dyn Widget>>) {
        self.children.push(child);
    }

    pub fn remove(&mut self, child: &Rc<RefCell<dyn Widget>>) {
        self.children.remove(
            self.children
                .iter()
                .position(|elem| elem.borrow().id() == child.borrow().id())
                .unwrap(),
        );
    }

    pub fn children(&self) -> &Vec<Rc<RefCell<dyn Widget>>> {
        return &self.children;
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
    fn hierarchy_add_child() {
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
    fn hierarchy_remove_child() {
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