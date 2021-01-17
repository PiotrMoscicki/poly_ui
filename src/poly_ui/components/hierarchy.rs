// std
use std::fmt::Debug;
// deps
use uuid::Uuid;
// crate
use crate::poly_ui::widgets::Owned;
use crate::poly_ui::widgets::Ownerless;
// super
use super::Transform

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug, Default)]
pub struct Hierarchy {
    children: Vec<Owned>,
    transforms: Vec<Transform>,
}

//************************************************************************************************
impl Hierarchy {
    pub fn add(&mut self, child: Ownerless) {
        self.children.push(child.make_owned());
    }

    pub fn remove(&mut self, id: &Uuid) -> Ownerless {
        self.children
            .remove(self.index(id).unwrap())
            .make_ownerless()
    }

    pub fn index(&self, id: &Uuid) -> Option<usize> {
        self.children
            .iter()
            .position(|elem| elem.get().borrow().id() == id)
    }

    pub fn children(&self) -> &Vec<Owned> {
        &self.children
    }
}
