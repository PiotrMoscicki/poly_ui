use nalgebra::Point2;
use nalgebra::Vector2;
use std::{
    cell::RefCell,
    rc::Rc,
    vec::Vec,
};

use crate::poly_ui::app::CanvasTrait;
use crate::poly_ui::app::Color;
use crate::poly_ui::app::Line;
use crate::poly_ui::app::Rect;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub struct Canvas {
    canvas : Rc<RefCell<Option<sdl2::render::Canvas<sdl2::video::Window>>>>,
    pos: Point2<i32>, 
    size: Vector2<u32>,
}

//************************************************************************************************
impl Canvas {
    pub fn new(canvas: Rc<RefCell<Option<sdl2::render::Canvas<sdl2::video::Window>>>>) -> Self {
        let output_size = Vector2::<u32>::new(
            canvas.borrow().as_ref().unwrap().output_size().unwrap().0,
            canvas.borrow().as_ref().unwrap().output_size().unwrap().1
        );

        return Canvas {
            canvas: canvas,
            pos: Point2::<i32>::new(0, 0),
            size: output_size,
        }
    }
}

//************************************************************************************************
impl CanvasTrait for Canvas {
    fn sub_canvas(&self, pos: Point2<i32>, size: Vector2<u32>) -> Box<dyn CanvasTrait> {
        return Box::new(Canvas {
            canvas: self.canvas.clone(),
            pos: pos,
            size: size,
        });
    }

    fn size(&self) -> Vector2<u32>{
        return self.size;
    }

    fn clear(&mut self) {
        self.canvas.borrow_mut().as_mut().unwrap().clear();
    }

    fn present(&mut self) {
        self.canvas.borrow_mut().as_mut().unwrap().present();
    }

    fn draw_color(&self) -> Color{
        let color = self.canvas.borrow().as_ref().unwrap().draw_color();
        return Color{r: color.r, g: color.g, b: color.b, a: color.a};
    }

    fn set_draw_color(&mut self, new: &Color) {
        self.canvas.borrow_mut().as_mut().unwrap()
        .set_draw_color(sdl2::pixels::Color{r: new.r, g: new.g, b: new.b, a: new.a});
    }

    fn draw_point(&mut self, point: &Point2<i32>) {
        self.canvas.borrow_mut().as_mut().unwrap().draw_point(sdl2::rect::Point::new(point.x, point.y)).unwrap();
    }

    fn draw_points(&mut self, points: &Vec<Point2<i32>>) {
        let mut converted = Vec::<sdl2::rect::Point>::new();

        for point in points {
            converted.push(sdl2::rect::Point::new(point.x, point.y));
        }

        self.canvas.borrow_mut().as_mut().unwrap().draw_points(&*converted.into_boxed_slice()).unwrap();
    }

    fn draw_line(&mut self, line: &Line) {
        self.canvas.borrow_mut().as_mut().unwrap().draw_line(
            sdl2::rect::Point::new(line.start.x, line.start.y),
            sdl2::rect::Point::new(line.end.x, line.end.y)
        ).unwrap();
    }

    fn draw_lines(&mut self, lines: &Vec<Line>) {
        let mut converted = Vec::<sdl2::rect::Point>::new();

        for line in lines {
            converted.push(sdl2::rect::Point::new(line.start.x, line.start.y));
            converted.push(sdl2::rect::Point::new(line.end.x, line.end.y));
        }

        self.canvas.borrow_mut().as_mut().unwrap().draw_points(&*converted.into_boxed_slice()).unwrap();
    }

    fn draw_rect(&mut self, rect: Rect) {
        self.canvas.borrow_mut().as_mut().unwrap().draw_rect(
            sdl2::rect::Rect::new(rect.pos.x, rect.pos.x, rect.size.x, rect.size.x)
        ).unwrap();
    }

    fn draw_rects(&mut self, rects: &Vec<Rect>) {
        let mut converted = Vec::<sdl2::rect::Rect>::new();

        for rect in rects {
            converted.push(sdl2::rect::Rect::new(rect.pos.x, rect.pos.x, rect.size.x, rect.size.x));
        }

        self.canvas.borrow_mut().as_mut().unwrap().draw_rects(&*converted.into_boxed_slice()).unwrap();
    }

    fn fill_rect(&mut self, rect: Rect) {
        self.canvas.borrow_mut().as_mut().unwrap().fill_rect(
            sdl2::rect::Rect::new(rect.pos.x, rect.pos.x, rect.size.x, rect.size.x)
        ).unwrap();
    }

    fn fill_rects(&mut self, rects: &Vec<Rect>) {
        let mut converted = Vec::<sdl2::rect::Rect>::new();

        for rect in rects {
            converted.push(sdl2::rect::Rect::new(rect.pos.x, rect.pos.x, rect.size.x, rect.size.x));
        }

        self.canvas.borrow_mut().as_mut().unwrap().fill_rects(&*converted.into_boxed_slice()).unwrap();
    }
}