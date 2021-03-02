// deps
use nalgebra::Point2;
use nalgebra::Vector2;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
/// Transform is a structure that stores relative position and size of the widget. Position of the
/// widget can be negative but its size must be always equal or greater than zero.
/// 
/// When creating Transform with Transform::default() it will have position (0, 0) and transform 
/// (0, 0),
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Transform {
    pub pos: Point2<i32>,
    pub size: Vector2<u32>,
}

//************************************************************************************************
impl Transform {
    pub fn new(pos: &Point2<i32>, size: &Vector2<u32>) -> Self {
        Self {
            pos: *pos,
            size: *size,
        }
    }
}

//************************************************************************************************
impl Default for Transform {
    fn default() -> Self {
        Self {
            pos: Point2::<i32>::new(0, 0),
            size: Vector2::<u32>::new(0, 0),
        }
    }
}
