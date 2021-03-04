// std
use std::{boxed::Box, cell::RefCell, fmt::Debug, rc::Rc};
// deps
use nalgebra::Point2;
use nalgebra::Vector2;
use uuid::Uuid;
// crate
use crate::poly_ui::layouts::CanvasLayout;
// super
use super::OwnedWidget;
use super::WidgetTrait;
use super::WindowProviderTrait;
use super::WindowTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
/// Default implementation of WindowTrait. It contains Layout Widget and WindowProvider to be able
/// to paint child widgets and manage window position and size.
#[derive(Debug)]
pub struct Window {
    widget_ptr: Rc<RefCell<CanvasLayout>>,
    owned_widget: OwnedWidget,
    id: Uuid,
    window_provider: Box<dyn WindowProviderTrait>,
}

//************************************************************************************************
impl Window {
    /// # Returns
    /// Instance of the Window with default Layout Widget and WindowProvider as provided.println!
    /// # Arguments
    /// * `provider` - WindowProvider that should be used by this Window for managing window
    /// position and size and painting child widgets on it
    pub fn new(provider: Box<dyn WindowProviderTrait>) -> Self {
        let widget = CanvasLayout::new();
        Self {
            widget_ptr: widget.get().clone(),
            owned_widget: widget.make_owned(),
            id: Uuid::new_v4(),
            window_provider: provider,
        }
    }
}

//************************************************************************************************
impl WindowTrait for Window {
    fn widget(&self) -> &Rc<RefCell<CanvasLayout>> {
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
