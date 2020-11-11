// std
use std::{cell::RefCell, fmt::Debug, rc::Rc};
// deps
use uuid::Uuid;
// crate
use crate::poly_ui::widgets::Owned;
use crate::poly_ui::widgets::Ownerless;
use crate::poly_ui::widgets::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Hierarchy {
    children: Vec<Owned>,
}

//************************************************************************************************
impl Hierarchy {
    pub fn new() -> Self {
        return Self {
            children: Vec::new(),
        };
    }

    pub fn add(&mut self, child: Ownerless) {
        self.children.push(child.to_owned());
    }

    pub fn remove(&mut self, id: &Uuid) -> Ownerless {
        return self
            .children
            .remove(self.index(id).unwrap())
            .to_ownerless();
    }

    pub fn index(&self, id: &Uuid) -> Option<usize> {
        return self.children
            .iter()
            .position(|elem| elem.get().borrow().id() == id);
    }

    pub fn children(&self) -> &Vec<Owned> {
        return &self.children;
    }
}
