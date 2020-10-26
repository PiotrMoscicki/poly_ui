use nalgebra::Point2;
use nalgebra::Vector2;

pub trait CanvasTrait {
    fn sub_canvas(&self, pos: Point2<u32>, size: Vector2<u32>) -> dyn CanvasTrait;

    fn size(&self);
    fn clear(&mut self);
    fn present(&mut self);

    fn draw_color(&self);
    fn set_draw_color(&mut self);

    fn draw_point(&mut self);
    fn draw_points(&mut self);
    fn draw_line(&mut self);
    fn draw_lines(&mut self);
    fn draw_rect(&mut self);
    fn draw_rects(&mut self);
    fn fill_rect(&mut self);
    fn fill_rects(&mut self);
}