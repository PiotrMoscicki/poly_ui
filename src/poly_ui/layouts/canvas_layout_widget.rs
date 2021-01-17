// std
use std::fmt::Debug;
// deps
use nalgebra::Point2;
use nalgebra::Vector2;
use uuid::Uuid;
// crate
use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::components::Transform;
use crate::poly_ui::components::Hierarchy;
// super
use super::WidgetTrait;
use super::update_children;
use super::paint_children;
use super::Ownerless;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct CanvasLayoutWidget {
    id: Uuid,
    hierarchy: Hierarchy,
}

//************************************************************************************************
impl CanvasLayoutWidget {
    fn add_child_at(&mut self, child: Ownerless, transform: &Transform);

    fn set_child_transform(&mut self, child: &Uuid, new_transform: &Transform);
    fn set_child_pos(&mut self, child: &Uuid, new_pos: &Point2<i32>);
    fn set_child_size(&mut self, child: &Uuid, new_size: &Vector2<i32>);
    fn get_child_transform(&self, child: &Uuid) -> &Transform;
    fn get_child_pos(&self, child: &Uuid) -> &Point2<i32>;
    fn get_child_size(&self, child: &Uuid) -> &Vector2<i32>;
}

//************************************************************************************************
impl WidgetTrait for CanvasLayoutWidget {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn add_child(&mut self, child: Ownerless) {
        self.hierarchy.add(child);
    }

    fn remove_child(&mut self, child: &Uuid) -> Ownerless {
        self.hierarchy.remove(child)
    }

    fn update(&mut self, dt: f32) {
        update_children(&self.hierarchy, dt);
    }

    fn paint(&self, painter: &mut dyn PainterTrait) {
        paint_children(&self.hierarchy, painter);
    }
}


//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn get_total_stretch() {
    //}
}