use nalgebra::Vector2;
use std::{
    cell::{Ref, RefMut},
    fmt::Debug,
};
use uuid::Uuid;

use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::layouts::Layout;

use super::WidgetTrait;
use super::WindowTrait;
use super::Widget;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Window {
    base: Widget,
}

//************************************************************************************************
impl Window {
    pub fn new() -> Self {
        return Self {
            base: Widget::new(),
        };
    }
}

//************************************************************************************************
impl WidgetTrait for Window {
    fn id(&self) -> &Uuid {
        return self.base.id();
    }

    fn pos(&self) -> &Vector2<i32> {
        return self.base.pos();
    }

    fn hierarchy(&self) -> Ref<Hierarchy> {
        return self.base.hierarchy();
    }

    fn hierarchy_mut(&mut self) -> RefMut<Hierarchy> {
        return self.base.hierarchy_mut();
    }

    fn set_layout(&mut self, layout: Box<dyn Layout>) {
        self.base.set_layout(layout);
    }

    fn layout(&self) -> &dyn Layout {
        return self.base.layout();
    }

    fn layout_mut(&mut self) -> &mut dyn Layout {
        return self.base.layout_mut();
    }
}

//************************************************************************************************
impl WindowTrait for Window {

}