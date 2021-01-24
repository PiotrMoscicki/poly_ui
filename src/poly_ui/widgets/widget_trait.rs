// std
use std::fmt::Debug;
use uuid::Uuid;
// crate
use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::components::Transform;
// super
use super::Ownerless;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait WidgetTrait: Debug {
    fn id(&self) -> &Uuid;

    // child widgets
    fn add_child(&mut self, child: Ownerless);
    fn remove_child(&mut self, child: &Uuid) -> Ownerless;

    // components
    fn get_hierarchy(&self) -> &Hierarchy;
    fn get_child_transform(&self, child: &Uuid) -> &Transform;

    // updates
    fn update(&mut self, dt: f32);
    fn paint(&self, canvas: &mut dyn PainterTrait);
}

//************************************************************************************************
impl std::hash::Hash for dyn WidgetTrait {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

//************************************************************************************************
impl std::cmp::PartialEq for dyn WidgetTrait {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

//************************************************************************************************
impl std::cmp::Eq for dyn WidgetTrait {}
