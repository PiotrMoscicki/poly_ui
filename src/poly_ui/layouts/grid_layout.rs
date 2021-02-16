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
use crate::poly_ui::widgets::NewWidget;
use crate::poly_ui::widgets::Ownerless;
use crate::poly_ui::widgets::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct GridLayout {
    id: Uuid,
    hierarchy: Hierarchy,
}

//************************************************************************************************
impl GridLayout {
    pub fn new_raw() -> Self {
        Self {
            id: Uuid::new_v4(),
            hierarchy: Hierarchy::default(),
        }
    }

    pub fn new() -> NewWidget<Self> {
        NewWidget::new(Self::new_raw())
    }

    pub fn insert_child_at(&mut self, child: Ownerless, &Option<u32> row, &Option<u32> col) {
        self.hierarchy.add_with_transform(child, transform);
    }
    
    pub fn set_row_stretch(&mut self, u32 row, u32 stretch) {

    }
    
    pub fn set_column_stretch(&mut self, u32 col, u32 stretch) {
        
    }
}

//************************************************************************************************
impl WidgetTrait for GridLayout {
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