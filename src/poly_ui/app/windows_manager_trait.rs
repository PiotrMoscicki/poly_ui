
use std::{
    cell::RefCell,
    rc::Rc,
};

use crate::poly_ui::widgets::WindowTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait WindowsManagerTrait {
    fn create_window(&mut self, title: &str, width: u32, height: u32) 
        -> Rc<RefCell<dyn WindowTrait>>;
}