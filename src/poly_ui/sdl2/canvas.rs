use nalgebra::Point2;
use nalgebra::Vector2;
use std::{
    cell::{Ref, RefCell, RefMut},
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
    canvas : Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
    pos: Point2<i32>, 
    size: Vector2<u32>,
}

//************************************************************************************************
impl Canvas {
    pub fn new(canvas: sdl2::render::Canvas<sdl2::video::Window>) -> Self {
        let output_size = Vector2::<u32>::new(
            canvas.output_size().unwrap().0,
            canvas.output_size().unwrap().1
        );

        return Canvas {
            canvas: Rc::new(RefCell::new(canvas)),
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
        self.canvas.borrow_mut().clear();
    }

    fn present(&mut self) {
        self.canvas.borrow_mut().present();
    }

    fn draw_color(&self) -> Color{
        let color = self.canvas.borrow().draw_color();
        return Color{r: color.r, g: color.g, b: color.b, a: color.a};
    }

    fn set_draw_color(&mut self, new: &Color) {
        self.canvas.borrow_mut()
        .set_draw_color(sdl2::pixels::Color{r: new.r, g: new.g, b: new.b, a: new.a});
    }

    fn draw_point(&mut self, point: &Point2<i32>) {
        self.canvas.borrow_mut().draw_point(sdl2::rect::Point::new(point.x, point.y));
    }

    fn draw_points(&mut self, points: &Vec<Point2<i32>>) {
        let mut converted = Vec::<sdl2::rect::Point>::new();

        for point in points {
            converted.push(sdl2::rect::Point::new(point.x, point.y));
        }

        self.canvas.borrow_mut().draw_points(&*converted.into_boxed_slice());
    }

    fn draw_line(&mut self, line: &Line) {
        self.canvas.borrow_mut().draw_line(
            sdl2::rect::Point::new(line.start.x, line.start.y),
            sdl2::rect::Point::new(line.end.x, line.end.y)
        );
    }

    fn draw_lines(&mut self, lines: &Vec<Line>) {
        let mut converted = Vec::<sdl2::rect::Point>::new();

        for line in lines {
            converted.push(sdl2::rect::Point::new(line.start.x, line.start.y));
            converted.push(sdl2::rect::Point::new(line.end.x, line.end.y));
        }

        self.canvas.borrow_mut().draw_points(&*converted.into_boxed_slice());
    }

    fn draw_rect(&mut self, rect: Rect) {
        self.canvas.borrow_mut().draw_rect(
            sdl2::rect::Rect::new(rect.pos.x, rect.pos.x, rect.size.x, rect.size.x)
        );
    }

    fn draw_rects(&mut self, rects: &Vec<Rect>) {
        let mut converted = Vec::<sdl2::rect::Rect>::new();

        for rect in rects {
            converted.push(sdl2::rect::Rect::new(rect.pos.x, rect.pos.x, rect.size.x, rect.size.x));
        }

        self.canvas.borrow_mut().draw_rects(&*converted.into_boxed_slice());
    }

    fn fill_rect(&mut self, rect: Rect) {
        self.canvas.borrow_mut().fill_rect(
            sdl2::rect::Rect::new(rect.pos.x, rect.pos.x, rect.size.x, rect.size.x)
        );
    }

    fn fill_rects(&mut self, rects: &Vec<Rect>) {
        let mut converted = Vec::<sdl2::rect::Rect>::new();

        for rect in rects {
            converted.push(sdl2::rect::Rect::new(rect.pos.x, rect.pos.x, rect.size.x, rect.size.x));
        }

        self.canvas.borrow_mut().fill_rects(&*converted.into_boxed_slice());
    }
}