extern crate sdl2;

use std::{
    cell::RefCell,
    rc::Rc,
};

use super::WindowProvider;
use crate::poly_ui::widgets::WindowTrait;
use crate::poly_ui::widgets::Window;
use crate::poly_ui::app::WindowsManagerTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub struct WindowsManager {
    sdl_video: Rc<RefCell<sdl2::VideoSubsystem>>,
}

//************************************************************************************************
impl WindowsManager {
    pub fn new(video: Rc<RefCell<sdl2::VideoSubsystem>>) -> Self {
        return WindowsManager {
            sdl_video: video,
        };
    }
}

//************************************************************************************************
impl WindowsManagerTrait for WindowsManager {
    fn create_window(&mut self, title: &str, width: u32, height: u32) 
        -> Rc<RefCell<dyn WindowTrait>> {
        let window = self.sdl_video.borrow_mut().window(title, width, height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        let window_provider =  Box::new(WindowProvider::new(window));
        return Rc::new(RefCell::new(Window::new(window_provider)));
    }
}