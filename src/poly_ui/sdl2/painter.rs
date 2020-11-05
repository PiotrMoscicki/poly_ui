use nalgebra::Point2;
use nalgebra::Vector2;
use std::{cell::RefCell, rc::Rc, vec::Vec};

use crate::poly_ui::app::Color;
use crate::poly_ui::app::Line;
use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::app::Rect;
use crate::poly_ui::components::Transform;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub struct Painter {
    canvas: Rc<RefCell<Option<sdl2::render::Canvas<sdl2::video::Window>>>>,
    transform: Transform,
}

//************************************************************************************************
impl Painter {
    pub fn new(canvas: Rc<RefCell<Option<sdl2::render::Canvas<sdl2::video::Window>>>>) -> Self {
        let output_size = Vector2::<u32>::new(
            canvas.borrow().as_ref().unwrap().output_size().unwrap().0,
            canvas.borrow().as_ref().unwrap().output_size().unwrap().1,
        );

        return Painter {
            canvas: canvas,
            transform: Transform::new(&Point2::<i32>::new(0, 0), &output_size),
        };
    }

    fn ensure_correct_viewport(&mut self) {
        let current_rect = self.canvas.borrow().as_ref().unwrap().viewport();
        if current_rect.x() != self.transform.pos.x
            || current_rect.y() != self.transform.pos.y
            || current_rect.width() != self.transform.size.x
            || current_rect.height() != self.transform.size.y
        {
            let new_rect = sdl2::rect::Rect::new(
                self.transform.pos.x,
                self.transform.pos.y,
                self.transform.size.x,
                self.transform.size.y,
            );
            self.canvas
                .borrow_mut()
                .as_mut()
                .unwrap()
                .set_viewport(new_rect);
        }
    }
}

//************************************************************************************************
impl PainterTrait for Painter {
    fn sub_painter(&self, transform: &Transform) -> Box<dyn PainterTrait> {
        return Box::new(Painter {
            canvas: self.canvas.clone(),
            transform: Transform::new(
                &Point2::<i32>::new(
                    self.transform.pos.x + transform.pos.x,
                    self.transform.pos.y + transform.pos.y,
                ),
                &transform.size,
            ),
        });
    }

    fn size(&self) -> Vector2<u32> {
        return self.transform.size;
    }

    fn clear(&mut self) {
        self.ensure_correct_viewport();
        self.canvas.borrow_mut().as_mut().unwrap().clear();
    }

    fn present(&mut self) {
        self.ensure_correct_viewport();
        self.canvas.borrow_mut().as_mut().unwrap().present();
    }

    fn draw_color(&self) -> Color {
        let color = self.canvas.borrow().as_ref().unwrap().draw_color();
        return Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        };
    }

    fn set_draw_color(&mut self, new: &Color) {
        self.canvas
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_draw_color(sdl2::pixels::Color {
                r: new.r,
                g: new.g,
                b: new.b,
                a: new.a,
            });
    }

    fn draw_point(&mut self, point: &Point2<i32>) {
        self.ensure_correct_viewport();
        self.canvas
            .borrow_mut()
            .as_mut()
            .unwrap()
            .draw_point(sdl2::rect::Point::new(point.x, point.y))
            .unwrap();
    }

    fn draw_points(&mut self, points: &Vec<Point2<i32>>) {
        self.ensure_correct_viewport();
        let mut converted = Vec::<sdl2::rect::Point>::new();

        for point in points {
            converted.push(sdl2::rect::Point::new(point.x, point.y));
        }

        self.canvas
            .borrow_mut()
            .as_mut()
            .unwrap()
            .draw_points(&*converted.into_boxed_slice())
            .unwrap();
    }

    fn draw_line(&mut self, line: &Line) {
        self.ensure_correct_viewport();
        self.canvas
            .borrow_mut()
            .as_mut()
            .unwrap()
            .draw_line(
                sdl2::rect::Point::new(line.start.x, line.start.y),
                sdl2::rect::Point::new(line.end.x, line.end.y),
            )
            .unwrap();
    }

    fn draw_lines(&mut self, lines: &Vec<Line>) {
        self.ensure_correct_viewport();
        let mut converted = Vec::<sdl2::rect::Point>::new();

        for line in lines {
            converted.push(sdl2::rect::Point::new(line.start.x, line.start.y));
            converted.push(sdl2::rect::Point::new(line.end.x, line.end.y));
        }

        self.canvas
            .borrow_mut()
            .as_mut()
            .unwrap()
            .draw_points(&*converted.into_boxed_slice())
            .unwrap();
    }

    fn draw_rect(&mut self, rect: Rect) {
        self.ensure_correct_viewport();
        self.canvas
            .borrow_mut()
            .as_mut()
            .unwrap()
            .draw_rect(sdl2::rect::Rect::new(
                rect.pos.x,
                rect.pos.x,
                rect.size.x,
                rect.size.x,
            ))
            .unwrap();
    }

    fn draw_rects(&mut self, rects: &Vec<Rect>) {
        self.ensure_correct_viewport();
        let mut converted = Vec::<sdl2::rect::Rect>::new();

        for rect in rects {
            converted.push(sdl2::rect::Rect::new(
                rect.pos.x,
                rect.pos.x,
                rect.size.x,
                rect.size.x,
            ));
        }

        self.canvas
            .borrow_mut()
            .as_mut()
            .unwrap()
            .draw_rects(&*converted.into_boxed_slice())
            .unwrap();
    }

    fn fill_rect(&mut self, rect: Rect) {
        self.ensure_correct_viewport();
        self.canvas
            .borrow_mut()
            .as_mut()
            .unwrap()
            .fill_rect(sdl2::rect::Rect::new(
                rect.pos.x,
                rect.pos.x,
                rect.size.x,
                rect.size.x,
            ))
            .unwrap();
    }

    fn fill_rects(&mut self, rects: &Vec<Rect>) {
        self.ensure_correct_viewport();
        let mut converted = Vec::<sdl2::rect::Rect>::new();

        for rect in rects {
            converted.push(sdl2::rect::Rect::new(
                rect.pos.x,
                rect.pos.x,
                rect.size.x,
                rect.size.x,
            ));
        }

        self.canvas
            .borrow_mut()
            .as_mut()
            .unwrap()
            .fill_rects(&*converted.into_boxed_slice())
            .unwrap();
    }
}
