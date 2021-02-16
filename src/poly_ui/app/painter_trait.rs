use nalgebra::Point2;
use nalgebra::Vector2;
use std::boxed::Box;

use crate::poly_ui::components::Transform;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Line {
    pub start: Point2<i32>,
    pub end: Point2<i32>,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Rect {
    pub pos: Point2<i32>,
    pub size: Vector2<u32>,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait PainterTrait {
    fn sub_painter(&self, transform: &Transform) -> Box<dyn PainterTrait>;

    fn size(&self) -> Vector2<u32>;
    fn clear(&mut self);
    fn present(&mut self);

    fn draw_color(&self) -> Color;
    fn set_draw_color(&mut self, new: &Color);

    fn draw_point(&mut self, point: &Point2<i32>);
    fn draw_points(&mut self, points: &[Point2<i32>]);
    fn draw_line(&mut self, line: &Line);
    fn draw_lines(&mut self, lines: &[Line]);
    fn draw_rect(&mut self, rect: Rect);
    fn draw_rects(&mut self, rect: &[Rect]);
    fn fill_rect(&mut self, rect: Rect);
    fn fill_rects(&mut self, rect: &[Rect]);
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Default)]
pub struct MockPainter {}

//************************************************************************************************
impl PainterTrait for MockPainter {
    fn sub_painter(&self, _transform: &Transform) -> Box<dyn PainterTrait> {
        Box::new(MockPainter {})
    }

    fn size(&self) -> Vector2<u32> {
        Vector2::<u32>::new(0, 0)
    }

    fn clear(&mut self) {}

    fn present(&mut self) {}

    fn draw_color(&self) -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }

    fn set_draw_color(&mut self, _new: &Color) {}

    fn draw_point(&mut self, _point: &Point2<i32>) {}

    fn draw_points(&mut self, _points: &[Point2<i32>]) {}

    fn draw_line(&mut self, _line: &Line) {}

    fn draw_lines(&mut self, _lines: &[Line]) {}

    fn draw_rect(&mut self, _rect: Rect) {}

    fn draw_rects(&mut self, _rect: &[Rect]) {}

    fn fill_rect(&mut self, _rect: Rect) {}

    fn fill_rects(&mut self, _rect: &[Rect]) {}
}
