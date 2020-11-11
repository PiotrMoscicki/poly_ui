use nalgebra::Point2;
use nalgebra::Vector2;
use std::{
    boxed::Box,
    cell::{Ref, RefMut},
    fmt::Debug,
};
use uuid::Uuid;

use super::Widget;
use super::WidgetTrait;
use super::WindowProviderTrait;
use super::WindowTrait;
use super::Owned;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Window {
    widget: Owned,
    id: Uuid,
    window_provider: Box<dyn WindowProviderTrait>,
}

//************************************************************************************************
impl Window {
    pub fn new(provider: Box<dyn WindowProviderTrait>) -> Self {
        return Self {
            widget: Widget::new().to_ownerless().to_owned(),
            id: Uuid::new_v4(),
            window_provider: provider,
        };
    }
}

//************************************************************************************************
impl WindowTrait for Window {
    fn widget(&self) -> Ref<dyn WidgetTrait> {
        return self.widget.get_widget_rc().borrow();
    }

    fn widget_mut(&mut self) -> RefMut<dyn WidgetTrait> {
        return self.widget.get_widget_rc().borrow_mut();
    }

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

    fn update(&mut self, dt: f32) {
        self.widget.get_widget_rc().borrow_mut().update(dt);
    }

    fn paint(&mut self) {
        self.window_provider.paint_widget(&*self.widget.get_widget_rc().borrow());
    }
}
