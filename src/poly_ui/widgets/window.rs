use nalgebra::Vector2;
use nalgebra::Point2;
use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Debug,
    rc::Rc,
    boxed::Box,
};
use uuid::Uuid;

use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::layouts::{CanvasLayout, Layout};

use super::WidgetTrait;
use super::WindowTrait;
use super::WindowProviderTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Window {
    id: Uuid,
    hierarchy: Rc<RefCell<Hierarchy>>,
    layout: Box<dyn Layout>,
    window_provider: Box<dyn WindowProviderTrait>,
}

//************************************************************************************************
impl Window {
    pub fn new(provider: Box<dyn WindowProviderTrait>) -> Self {
        return Self {
            id: Uuid::new_v4(),
            hierarchy: Rc::new(RefCell::new(Hierarchy::new())),
            layout: Box::new(CanvasLayout::new()),
            window_provider: provider,
        };
    }
}

//************************************************************************************************
impl WidgetTrait for Window {
    fn id(&self) -> &Uuid {
        return &self.id;
    }

    fn pos(&self) -> Point2<i32> {
        return self.window_provider.pos();
    }

    fn set_pos(&mut self, new: Point2<i32>) {
        self.window_provider.set_pos(new);
        return ();
    }

    fn size(&self) -> Vector2<u32> {
        return self.window_provider.size();
    }

    fn set_size(&mut self, new: Vector2<u32>) {
        self.window_provider.set_size(new);
        return ();
    }

    fn hierarchy(&self) -> Ref<Hierarchy> {
        return self.hierarchy.borrow();
    }

    fn hierarchy_mut(&mut self) -> RefMut<Hierarchy> {
        return self.hierarchy.borrow_mut();
    }

    fn set_layout(&mut self, layout: Box<dyn Layout>) {
        self.layout = layout;
        self.layout
            .set_owner_widget_hierarchy(self.hierarchy.clone());
    }

    fn layout(&self) -> &dyn Layout {
        return self.layout.as_ref();
    }

    fn layout_mut(&mut self) -> &mut dyn Layout {
        return self.layout.as_mut();
    }
}

//************************************************************************************************
impl WindowTrait for Window {

}