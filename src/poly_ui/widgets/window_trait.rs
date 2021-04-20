// std
use std::{cell::RefCell, rc::Rc};
// deps
use nalgebra::Point2;
use nalgebra::Vector2;
use uuid::Uuid;
// crate
use crate::poly_ui::layouts::CanvasLayout;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
/// WindowTrait is a base trait for all top level widgets. Onlike other widgets it stores its
/// Transform by itself. It has one child widget which is a CanvasLayout.
pub trait WindowTrait {
    /// # Returns
    /// Theonly child Widget this window has; its Layout Widget.
    fn widget(&self) -> &Rc<RefCell<CanvasLayout>>;

    /// # Returns
    /// This Window id
    fn id(&self) -> &Uuid;

    /// # Returns
    /// This Window position on the desktop. Can be negative.
    fn pos(&self) -> Point2<i32>;

    /// Sets pos for this window.
    /// # Arguments
    /// * `new` - new position for this Window
    fn set_pos(&mut self, new: Point2<i32>);

    /// # Returns
    /// Size of this window
    fn size(&self) -> Vector2<u32>;

    /// Sets size for this Window
    /// # Arguments
    /// * `new` - new size for this Window
    fn set_size(&mut self, new: Vector2<u32>);

    /// Updates this Window and its Layout Widget with provided delta time
    /// # Arguments
    /// * `dt` - delta time from the last update in milliseconds
    fn update(&mut self, dt: f32);

    /// Paints this Window and its Layout Widget.
    fn paint(&mut self);
}
