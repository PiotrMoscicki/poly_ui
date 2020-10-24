use nalgebra::Vector2;
use std::{
    cell::{Ref, RefMut},
    fmt::Debug,
};
use uuid::Uuid;

use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::layouts::Layout;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait Widget: Debug {
    fn id(&self) -> &Uuid;

    fn pos(&self) -> &Vector2<i32>;

    fn hierarchy(&self) -> Ref<Hierarchy>;
    fn hierarchy_mut(&mut self) -> RefMut<Hierarchy>;

    fn set_layout(&mut self, layout: Box<dyn Layout>);
    fn layout(&self) -> &dyn Layout;
    fn layout_mut(&mut self) -> &mut dyn Layout;
}

//************************************************************************************************
impl std::hash::Hash for dyn Widget {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

//************************************************************************************************
impl std::cmp::PartialEq for dyn Widget {
    fn eq(&self, other: &Self) -> bool {
        return self.id() == other.id();
    }
}

//************************************************************************************************
impl std::cmp::Eq for dyn Widget {}