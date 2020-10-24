extern crate sdl2;

use sdl2::video::WindowPos;

use crate::poly_ui::widgets::WindowProviderTrait;
use crate::poly_ui::components::Transform;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub struct WindowProvider {
    window: sdl2::video::Window,
}

//************************************************************************************************
impl WindowProvider {
    pub fn new(wnd: sdl2::video::Window) -> Self {
        return WindowProvider {
            window: wnd, 
        };
    }
}

impl WindowProviderTrait for WindowProvider {
    fn transform(&self) -> Transform {
        let mut transform = Transform::new();
        transform.pos.x = self.window.position().0;
        transform.pos.y = self.window.position().1;
        transform.size.x = self.window.size().0 as i32;
        transform.size.y = self.window.size().1 as i32;
        return transform;
    }

    fn set_transform(&mut self, new: Transform) {
        self.window.set_position(
            WindowPos::Positioned(new.pos.x), 
            WindowPos::Positioned(new.pos.y)
        );
    }
}

impl std::fmt::Debug for WindowProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowProvider")
         .finish()
    }
}