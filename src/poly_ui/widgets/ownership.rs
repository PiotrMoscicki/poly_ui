// std
use std::{cell::Ref, cell::RefCell, cell::RefMut, rc::Rc};
// super
use super::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub struct Ownerless {
    widget: Rc<RefCell<dyn WidgetTrait>>,
}

//************************************************************************************************
impl Ownerless {
    pub fn make_owned(self) -> Owned {
        Owned {
            widget: self.widget,
        }
    }

    pub fn get(&self) -> &Rc<RefCell<dyn WidgetTrait>> {
        &self.widget
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Owned {
    widget: Rc<RefCell<dyn WidgetTrait>>,
}

//************************************************************************************************
impl Owned {
    pub fn make_ownerless(self) -> Ownerless {
        Ownerless {
            widget: self.widget,
        }
    }

    pub fn get(&self) -> &Rc<RefCell<dyn WidgetTrait>> {
        &self.widget
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

    pub fn make_ownerless(self) -> Ownerless {
        Ownerless {
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
