use nalgebra::Point2;
use nalgebra::Vector2;
use std::boxed::Box;

use crate::poly_ui::components::Transform;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
/// Color struct used by Painters
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
/// Line struct used by Painters
#[derive(Debug)]
pub struct Line {
    pub start: Point2<i32>,
    pub end: Point2<i32>,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
/// Rect struct used by Painters
#[derive(Debug)]
pub struct Rect {
    pub pos: Point2<i32>,
    pub size: Vector2<u32>,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
/// Painter is a wrapper for third party painting logic. It is used by all widgets to paint
/// themselves on the screen.
pub trait PainterTrait {
    /// SubPainters are used when widget has a child that should get its own part of the screen
    /// to paint. We don't want to give it the whole window to paint so we provide transform for
    /// the new subpainter and the returned painter has new position and new size.
    /// # Arguments
    /// * `transform` - transform for the SubPainter relative to this Painter
    /// # returns
    /// Child SubPainter.
    fn sub_painter(&self, transform: &Transform) -> Box<dyn PainterTrait>;

    /// # Returns
    /// Size of this subpainter.
    fn size(&self) -> Vector2<u32>;

    /// Clears the whole Painter rect with currently set draw Color.
    fn clear(&mut self);

    /// # Returns
    /// Currently set draw color.
    fn draw_color(&self) -> Color;

    /// Sets new draw color for this Painter. All points lines and shapes will be drawed with this
    /// color from now on.
    /// # Arguments
    /// * `new` - new draw color for this painter
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
/// MockPainter is an empty implementation for the PainterTrait for testing purposes.
#[derive(Default)]
pub struct MockPainter {
    pub size: Vector2<u32>,
}

impl MockPainter {
    pub fn new() -> Self {
        Self {
            size: Vector2::<u32>::new(0, 0),
        }
    }
}

//************************************************************************************************
impl PainterTrait for MockPainter {
    fn sub_painter(&self, _transform: &Transform) -> Box<dyn PainterTrait> {
        Box::new(MockPainter::new())
    }

    fn size(&self) -> Vector2<u32> {
        self.size
    }

    fn clear(&mut self) {}

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
