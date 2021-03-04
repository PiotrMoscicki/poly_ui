use std::{cell::RefCell, rc::Rc};

use crate::poly_ui::widgets::WindowTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
/// WindowsManager contains collection of all created windows. It's responsible for painting and 
/// updating all these windows. With WindowsManager you can also create new Windows.
pub trait WindowsManagerTrait {
    /// Function for creating new windows with specified title and size.assert_eq!
    /// # Arguments
    /// * `title` - title for the new wineow
    /// * `width` - width for the new window
    /// * `height` - height for the new window
    /// # Returns 
    /// Newly created window.
    fn create_window(
        &mut self,
        title: &str,
        width: u32,
        height: u32,
    ) -> Rc<RefCell<dyn WindowTrait>>;

    /// Updates all opened windows with provided delta time.
    /// # Arguments
    /// * `dt` - delta time in milliseconds from the last update
    fn update_windows(&mut self, dt: f32);

    /// Paints all opened windows.
    fn paint_windows(&mut self);
}
