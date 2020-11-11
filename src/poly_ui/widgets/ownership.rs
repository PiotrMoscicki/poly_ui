// std
use std::{
    cell::RefCell,
    rc::Rc,
};
// super
use super::WidgetTrait;
use super::Widget;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Ownerless<T> {
    widget: Rc<RefCell<T>>,
}

//************************************************************************************************
impl<T> Ownerless<T> {
    pub fn new(widget: Rc<RefCell<T>>) -> Self {
        return Self {
            widget: widget,
        };
    }

    pub fn to_trait_object(self) -> Ownerless<dyn WidgetTrait> {
        let widget: T = self.widget.borrow_mut().replace(Widget::new_raw());


        let result: Ownerless<dyn WidgetTrait> = Ownerless::new(Rc::new(RefCell::new(widget)));
        return result;
    }

    pub fn to_owned(self) -> Owned<T> {
        return Owned{
            widget: self.widget, 
        };
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Owned<T: WidgetTrait + ?Sized> {
    widget: Rc<RefCell<T>>,
}

//************************************************************************************************
impl<T: WidgetTrait + ?Sized> Owned<T> {
    pub fn to_ownerless(self) -> Ownerless<T> {
        return Ownerless{
            widget: self.widget, 
        };
    }

    pub fn get_widget_rc(&self) -> &Rc<RefCell<T>> {
        return &self.widget;
    }
}