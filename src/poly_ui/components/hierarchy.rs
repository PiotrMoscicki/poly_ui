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
    children: Vec<Owned<dyn WidgetTrait>>,
}

//************************************************************************************************
impl Hierarchy {
    pub fn new() -> Self {
        return Self {
            children: Vec::<Owned<dyn WidgetTrait>>::new(),
        };
    }

    pub fn add(&mut self, child: Ownerless<dyn WidgetTrait>) -> Rc<RefCell<dyn WidgetTrait>> {
        let owned = child.to_owned();
        let result = owned.get_widget_rc().clone();
        self.children.push(owned);
        return result;
    }

    pub fn remove(&mut self, child: &Rc<RefCell<dyn WidgetTrait>>) -> Ownerless<dyn WidgetTrait> {
        return self.children.remove(
            self.children
                .iter()
                .position(|elem| elem.get_widget_rc().borrow().id() == child.borrow().id())
                .unwrap(),
        ).to_ownerless();
    }

    pub fn children(&self) -> &Vec<Owned<dyn WidgetTrait>> {
        return &self.children;
    }
}