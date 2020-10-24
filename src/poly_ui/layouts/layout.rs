use nalgebra::Vector3;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::widgets::Widget;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait Layout: Debug {
    fn set_owner_widget_hierarchy(&mut self, hierarchy: Rc<RefCell<Hierarchy>>);

    fn add(&mut self, child: Rc<RefCell<dyn Widget>>, pos: Vector3<i32>);
}