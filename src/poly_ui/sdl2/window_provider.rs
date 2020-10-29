extern crate sdl2;

use nalgebra::Point2;
use nalgebra::Vector2;
use sdl2::video::WindowPos;

use super::Canvas;
use crate::poly_ui::app::CanvasTrait;
use crate::poly_ui::widgets::WindowProviderTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub struct WindowProvider {
    window: sdl2::video::Window,
}

//************************************************************************************************
impl WindowProvider {
    pub fn new(wnd: sdl2::video::Window) -> Self {
        return WindowProvider { window: wnd };
    }
}

impl WindowProviderTrait for WindowProvider {
    fn canvas(&mut self) -> Box<dyn CanvasTrait> {
        return Box::new(Canvas::new(self.window.into_canvas().present_vsync().build().unwrap()));
    }
    
    fn pos(&self) -> Point2<i32> {
        return Point2::<i32>::new(self.window.position().0, self.window.position().1);
    }

    fn set_pos(&mut self, new: Point2<i32>) {
        self.window
            .set_position(WindowPos::Positioned(new.x), WindowPos::Positioned(new.y));
    }

    fn size(&self) -> Vector2<u32> {
        return Vector2::<u32>::new(self.window.size().0, self.window.size().1);
    }

    fn set_size(&mut self, new: Vector2<u32>) {
        self.window.set_size(new.x, new.y).unwrap();
    }
}

impl std::fmt::Debug for WindowProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowProvider").finish()
    }
}
