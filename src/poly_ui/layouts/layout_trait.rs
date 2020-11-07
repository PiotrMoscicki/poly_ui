use nalgebra::Vector2;
use std::{cell::RefCell, fmt::Debug, rc::Rc};
use uuid::Uuid;

use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::components::Transform;
use crate::poly_ui::widgets::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait LayoutTrait: Debug {
    fn set_owner_widget_hierarchy(&mut self, hierarchy: Rc<RefCell<Hierarchy>>);

    fn add(&mut self, child: Rc<RefCell<dyn WidgetTrait>>);

    fn transform(&self, parent_size: &Vector2<u32>, widget_id: &Uuid) -> Transform;
}
