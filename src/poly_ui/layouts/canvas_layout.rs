// std
use std::fmt::Debug;
// deps
use nalgebra::Point2;
use nalgebra::Vector2;
use uuid::Uuid;
// crate
use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::components::Transform;
use crate::poly_ui::widgets::Ownerless;
use crate::poly_ui::widgets::WidgetTrait;

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
    pub fn add_child_with_transform(&mut self, child: Ownerless, transform: &Transform) {
        self.hierarchy.add_with_transform(child, transform);
    }

    pub fn set_child_transform(&mut self, child: &Uuid, new_transform: &Transform) {
        self.hierarchy.set_transform(child, new_transform);
    }

    pub fn set_child_pos(&mut self, child: &Uuid, new_pos: &Point2<i32>) {
        self.hierarchy.set_pos(child, new_pos);
    }

    pub fn set_child_size(&mut self, child: &Uuid, new_size: &Vector2<u32>) {
        self.hierarchy.set_size(child, new_size);
    }

    pub fn get_child_transform(&self, child: &Uuid) -> &Transform {
        self.hierarchy.get_transform(child)
    }
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

    fn get_hierarchy(&self) -> &Hierarchy {
        &self.hierarchy
    }

    fn get_child_transform(&self, child: &Uuid) -> &Transform {
        self.hierarchy.get_transform(child)
    }

    fn update(&mut self, dt: f32) {
        self.hierarchy.update_children(dt);
    }

    fn paint(&self, painter: &mut dyn PainterTrait) {
        self.hierarchy.paint_children(painter);
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[cfg(test)]
mod tests {
    //use super::*;

    //#[test]
    //fn get_total_stretch() {
    //}
}
