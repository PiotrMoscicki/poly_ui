// std
use std::{boxed::Box, cell::RefCell, fmt::Debug, rc::Rc};
// deps
use nalgebra::Point2;
use nalgebra::Vector2;
use uuid::Uuid;
// crate
use crate::poly_ui::layouts::GridLayout;
// super
use super::Owned;
use super::WidgetTrait;
use super::WindowProviderTrait;
use super::WindowTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Window {
    widget_ptr: Rc<RefCell<GridLayout>>,
    owned_widget: Owned,
    id: Uuid,
    window_provider: Box<dyn WindowProviderTrait>,
}

//************************************************************************************************
impl Window {
    pub fn new(provider: Box<dyn WindowProviderTrait>) -> Self {
        let widget = GridLayout::new();
        Self {
            widget_ptr: widget.get().clone(),
            owned_widget: widget.make_ownerless().make_owned(),
            id: Uuid::new_v4(),
            window_provider: provider,
        }
    }
}

//************************************************************************************************
impl WindowTrait for Window {
    fn widget(&self) -> &Rc<RefCell<GridLayout>> {
        &self.widget_ptr
    }

    fn id(&self) -> &Uuid {
        &self.id
    }

    fn pos(&self) -> Point2<i32> {
        self.window_provider.pos()
    }

    fn set_pos(&mut self, new: Point2<i32>) {
        self.window_provider.set_pos(new);
    }

    fn size(&self) -> Vector2<u32> {
        self.window_provider.size()
    }

    fn set_size(&mut self, new: Vector2<u32>) {
        self.window_provider.set_size(new);
    }

    fn update(&mut self, dt: f32) {
        self.widget_ptr.borrow_mut().update(dt);
    }

    fn paint(&mut self) {
        self.window_provider
            .paint_widget(&mut *self.widget_ptr.borrow_mut());
    }
}
