use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::poly_ui::widgets::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Hierarchy {
    children: Vec<Rc<RefCell<dyn WidgetTrait>>>,
}

//************************************************************************************************
impl Hierarchy {
    pub fn new() -> Self {
        return Self {
            children: Vec::new(),
        };
    }

    pub fn add(&mut self, child: Rc<RefCell<dyn WidgetTrait>>) {
        self.children.push(child);
    }

    pub fn remove(&mut self, child: &Rc<RefCell<dyn WidgetTrait>>) {
        self.children.remove(
            self.children
                .iter()
                .position(|elem| elem.borrow().id() == child.borrow().id())
                .unwrap(),
        );
    }

    pub fn children(&self) -> &Vec<Rc<RefCell<dyn WidgetTrait>>> {
        return &self.children;
    }
}
