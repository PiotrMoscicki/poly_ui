// std
use std::fmt::Debug;
// deps
use nalgebra::Point2;
use nalgebra::Vector2;
// super
use super::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
/// This trait is used as a wrapper for the third party window (Window created from SDL for
/// example). It is responsible for the Window placement, size and providing Painter.
pub trait WindowProviderTrait: Debug {
    /// When this function is called implementation of the widget trait will call paint method on
    /// provided widget passing its own implementation of the PainterTrait. The widget on which
    /// paint is called will create subpainters and paint its children passing subpainters as
    /// arguments in paint methods called on them.
    fn paint_widget(&mut self, widget: &mut dyn WidgetTrait);

    /// # Returns
    /// Position of this Window
    fn pos(&self) -> Point2<i32>;

    /// Sets new position.
    /// # Arguments
    /// * `new` - position on the desktop
    fn set_pos(&mut self, new: Point2<i32>);

    /// # Returns
    /// Size of the window.
    fn size(&self) -> Vector2<u32>;

    /// Sets new size for the window.
    /// # Arguments
    /// * `new` - new size
    fn set_size(&mut self, new: Vector2<u32>);
}
