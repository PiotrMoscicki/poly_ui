extern crate sdl2;

use std::{cell::RefCell, rc::Rc, vec::Vec};

use super::WindowProvider;
use crate::poly_ui::app::WindowsManagerTrait;
use crate::poly_ui::widgets::Window;
use crate::poly_ui::widgets::WindowTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub struct WindowsManager {
    sdl_video: Rc<RefCell<sdl2::VideoSubsystem>>,
    windows: Vec<Rc<RefCell<dyn WindowTrait>>>,
}

//************************************************************************************************
impl WindowsManager {
    pub fn new(video: Rc<RefCell<sdl2::VideoSubsystem>>) -> Self {
        return WindowsManager { sdl_video: video, windows: Vec::new() };
    }
}

//************************************************************************************************
impl WindowsManagerTrait for WindowsManager {
    fn create_window(
        &mut self,
        title: &str,
        width: u32,
        height: u32,
    ) -> Rc<RefCell<dyn WindowTrait>> {
        let window = self
            .sdl_video
            .borrow_mut()
            .window(title, width, height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let window_provider = Box::new(WindowProvider::new(window));
        let window = Rc::new(RefCell::new(Window::new(window_provider)));
        self.windows.push(window.clone());
        return window;
    }

    fn update_windows(&mut self, dt: f32) {
        for window in &mut self.windows {
            window.borrow_mut().update(dt);
        }
    }

    fn paint_windows(&mut self) {
        for window in &mut self.windows {
            window.borrow_mut().paint_window();
        }
    }
}
