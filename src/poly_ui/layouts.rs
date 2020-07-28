use nalgebra::Vector3;
use uuid::Uuid;
use std::{collections::HashMap, fmt::Debug, rc::Rc, cell::RefCell};

use super::widgets::Widget;
use super::components::Hierarchy;

// traits
//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait Layout: Debug {
    fn set_owner_widget_hierarchy(&mut self, hierarchy: Rc<RefCell<Hierarchy>>);    
    
    fn add(&mut self, child: Rc<RefCell<dyn Widget>>, pos: Vector3<i32>);
}

// impl
//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct CanvasLayout {
    children: HashMap<Uuid, Vector3<i32>>,
    hierarchy: Rc<RefCell<Hierarchy>>,
}

impl CanvasLayout {
    pub fn new() -> Self {
        return Self {
            children: HashMap::new(),
            hierarchy: Rc::new(RefCell::new(Hierarchy::new())),
        };
    }
}

impl Layout for CanvasLayout {
    fn set_owner_widget_hierarchy(&mut self, hierarchy: Rc<RefCell<Hierarchy>>) {
        self.hierarchy = hierarchy;
    }

    fn add(&mut self, child: Rc<RefCell<dyn Widget>>, pos: Vector3<i32>) {
        self.children.insert(*child.borrow().id(), pos);
        self.hierarchy.borrow_mut().add(child);
    }
}