// deps
// use nalgebra::Point2;
// use nalgebra::Vector2;
use uuid::Uuid;
// // crate
// use crate::poly_ui::app::Color;
// use crate::poly_ui::app::PainterTrait;
// use crate::poly_ui::app::Rect;
use crate::poly_ui::components::Hierarchy;
// use crate::poly_ui::components::Transform;
// // super
// use super::NewWidget;
// use super::Ownerless;
// use super::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct MockWidget {
    pub update_call_count: u32,
    pub paint_call_count: u32,

    id: Uuid,
    hierarchy: Hierarchy,
}

//************************************************************************************************
impl MockWidget {
    // pub fn new_raw() -> Self {
    //     Self {
    //         id: Uuid::new_v4(),
    //         hierarchy: Hierarchy::default(),
    //     }
    // }

    // pub fn new() -> NewWidget<Self> {
    //     NewWidget::new(Self::new_raw())
    // }
}

// //************************************************************************************************
// impl WidgetTrait for MockWidget {
//     fn id(&self) -> &Uuid {
//         &self.id
//     }

//     fn add_child(&mut self, child: Ownerless) {
//         self.hierarchy.add(child);
//     }

//     fn remove_child(&mut self, child: &Uuid) -> Ownerless {
//         self.hierarchy.remove(child)
//     }

//     fn get_hierarchy(&self) -> &Hierarchy {
//         &self.hierarchy
//     }

//     fn get_child_transform(&self, child: &Uuid) -> &Transform {
//         self.hierarchy.get_transform(child)
//     }

//     fn update(&mut self, dt: f32) {
//         self.hierarchy.update_children(dt);
//         update_call_count += 1;
//     }

//     fn paint(&self, painter: &mut dyn PainterTrait) {
//         self.hierarchy.paint_children(painter);
//         paint_call_count += 1;
//     }
// }
