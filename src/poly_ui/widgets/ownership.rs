// std
use std::{cell::Ref, cell::RefCell, cell::RefMut, rc::Rc};
// super
use super::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
/// One of three wrapper structs for managing the Widgets lifetimes. We can obtain Ownerless 
/// Widgets from NewlyCreated and Owned wrappers. The purpose of this wrappers os to prevent from 
/// adding the same Widget to more than one parent.
pub struct Ownerless {
    widget: Rc<RefCell<dyn WidgetTrait>>,
}

//************************************************************************************************
impl Ownerless {
    // I just realised this is a unnecessary complication, just create non copyable WidgetPtr
    pub fn make_owned(self) -> Owned {
        Owned {
            widget: self.widget,
        }
    }

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
