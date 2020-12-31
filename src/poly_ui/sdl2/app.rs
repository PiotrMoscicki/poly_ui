extern crate sdl2;

use std::{cell::RefCell, rc::Rc};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use super::WindowsManager;
use crate::poly_ui::app::AppTrait;
use crate::poly_ui::app::WindowsManagerTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub struct App {
    sdl_context: sdl2::Sdl,
    _sdl_video: Rc<RefCell<sdl2::VideoSubsystem>>,
    windows_manager: WindowsManager,
}

//************************************************************************************************
impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

//************************************************************************************************
impl App {
    pub fn new() -> Self {
        let context = sdl2::init().unwrap();
        let video = Rc::new(RefCell::new(context.video().unwrap()));

        App {
            sdl_context: context,
            _sdl_video: video.clone(),
            windows_manager: WindowsManager::new(video),
        }
    }
}

//************************************************************************************************
impl AppTrait for App {
    fn exec(&mut self) -> Result<(), String> {
        'mainloop: loop {
            for event in self.sdl_context.event_pump()?.poll_iter() {
                match event {
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    }
                    | Event::Quit { .. } => break 'mainloop,
                    _ => {}
                }
            }

            self.windows_manager.update_windows(0.0);
            self.windows_manager.paint_windows();
        }

        Ok(())
    }

    fn get_windows_manager(&mut self) -> &mut dyn WindowsManagerTrait {
        &mut self.windows_manager
    }
}
