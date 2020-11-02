extern crate sdl2;

use nalgebra::Point2;
use nalgebra::Vector2;
use sdl2::video::WindowPos;
use std::{cell::RefCell, rc::Rc};

use super::Painter;
use crate::poly_ui::widgets::WidgetTrait;
use crate::poly_ui::widgets::WindowProviderTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub struct WindowProvider {
    window: Option<sdl2::video::Window>,
}

//************************************************************************************************
impl WindowProvider {
    pub fn new(wnd: sdl2::video::Window) -> Self {
        return WindowProvider { window: Some(wnd) };
    }
}

impl WindowProviderTrait for WindowProvider {
    fn paint_widget(&mut self, widget: &dyn WidgetTrait) {
        let sdl_canvas = Rc::new(RefCell::new(Some(
            self.window
                .take()
                .unwrap()
                .into_canvas()
                .present_vsync()
                .build()
                .unwrap(),
        )));

        {
            let mut painter = Painter::new(sdl_canvas.clone());
            widget.paint(&mut painter);
        }

        if Rc::strong_count(&sdl_canvas) != 1 {
            panic!();
        }

        sdl_canvas.borrow_mut().as_mut().unwrap().present();

        self.window = Some(sdl_canvas.borrow_mut().take().unwrap().into_window());
    }

    fn pos(&self) -> Point2<i32> {
        match &self.window {
            Some(wnd) => {
                return Point2::<i32>::new(wnd.position().0, wnd.position().1);
            }
            None => panic!(),
        }
    }

    fn set_pos(&mut self, new: Point2<i32>) {
        match &mut self.window {
            Some(wnd) => {
                return wnd
                    .set_position(WindowPos::Positioned(new.x), WindowPos::Positioned(new.y));
            }
            None => panic!(),
        }
    }

    fn size(&self) -> Vector2<u32> {
        match &self.window {
            Some(wnd) => {
                return Vector2::<u32>::new(wnd.size().0, wnd.size().1);
            }
            None => panic!(),
        }
    }

    fn set_size(&mut self, new: Vector2<u32>) {
        match &mut self.window {
            Some(wnd) => {
                return wnd.set_size(new.x, new.y).unwrap();
            }
            None => panic!(),
        }
    }
}

impl std::fmt::Debug for WindowProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowProvider").finish()
    }
}
