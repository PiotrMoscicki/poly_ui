// std
use std::{cell::RefCell, fmt::Debug, rc::Rc};
// crate
use crate::poly_ui::widgets::Ownerless;
use crate::poly_ui::widgets::Owned;

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

    pub fn remove(&mut self, child: &Rc<RefCell<dyn WidgetTrait>>) -> Ownerless {
        return self.children.remove(
            self.children
                .iter()
                .position(|elem| elem.get_widget_rc().borrow().id() == child.borrow().id())
                .unwrap(),
        ).to_ownerless();
    }

    pub fn children(&self) -> &Vec<Owned> {
        return &self.children;
    }
}