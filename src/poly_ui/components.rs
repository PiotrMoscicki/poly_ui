use nalgebra::Vector3;
use std::{fmt::Debug, rc::Rc, cell::RefCell};

use super::widgets::Widget;

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