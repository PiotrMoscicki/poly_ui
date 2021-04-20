// std
use std::{cell::Ref, cell::RefCell, cell::RefMut, rc::Rc};
// super
use super::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct OwnedWidget {
    widget: Rc<RefCell<dyn WidgetTrait>>,
}

//************************************************************************************************
impl OwnedWidget {
    pub fn get(&self) -> &Rc<RefCell<dyn WidgetTrait>> {
        &self.widget
    }

    pub fn borrow(&self) -> Ref<'_, dyn WidgetTrait> {
        self.widget.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, dyn WidgetTrait> {
        self.widget.borrow_mut()
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct NewWidget<T: WidgetTrait + 'static> {
    widget: Rc<RefCell<T>>,
}

//************************************************************************************************
impl<T: WidgetTrait + 'static> NewWidget<T> {
    pub fn new(widget: T) -> Self {
        Self {
            widget: Rc::new(RefCell::new(widget)),
        }
    }

    pub fn make_owned(self) -> OwnedWidget {
        OwnedWidget {
            widget: self.widget,
        }
    }

    pub fn get(&self) -> &Rc<RefCell<T>> {
        &self.widget
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        self.widget.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.widget.borrow_mut()
    }
}
