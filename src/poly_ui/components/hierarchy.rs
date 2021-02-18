// std
use std::fmt::Debug;
// deps
use nalgebra::Point2;
use nalgebra::Vector2;
use uuid::Uuid;
// crate
use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::widgets::Owned;
use crate::poly_ui::widgets::Ownerless;
// super
use super::Transform;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct HierarchyChild {
    pub widget: Owned,
    pub transform: Transform,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug, Default)]
pub struct Hierarchy {
    children: Vec<HierarchyChild>,
}

//************************************************************************************************
impl Hierarchy {
    // add / remove
    pub fn add(&mut self, child: Ownerless) {
        self.children.push(HierarchyChild {
            widget: child.make_owned(),
            transform: Transform::default(),
        });
    }

    pub fn add_with_transform(&mut self, child: Ownerless, transform: &Transform) {
        self.children.push(HierarchyChild {
            widget: child.make_owned(),
            transform: *transform,
        });
    }

    pub fn remove(&mut self, id: &Uuid) -> Ownerless {
        self.children
            .remove(self.index(id).unwrap())
            .widget
            .make_ownerless()
    }

    // move resize
    pub fn set_pos(&mut self, id: &Uuid, pos: &Point2<i32>) {
        let idx = self.index(id).unwrap();
        self.children[idx].transform.pos = *pos;
    }

    pub fn set_size(&mut self, id: &Uuid, size: &Vector2<u32>) {
        let idx = self.index(id).unwrap();
        self.children[idx].transform.size = *size;
    }

    pub fn set_transform(&mut self, id: &Uuid, transform: &Transform) {
        let idx = self.index(id).unwrap();
        self.children[idx].transform = *transform;
    }

    // getters
    pub fn index(&self, id: &Uuid) -> Option<usize> {
        self.children
            .iter()
            .position(|elem| elem.widget.get().borrow().id() == id)
    }

    pub fn children(&self) -> &Vec<HierarchyChild> {
        &self.children
    }

    pub fn get_transform(&self, id: &Uuid) -> &Transform {
        &self.children[self.index(id).unwrap()].transform
    }

    // updates
    pub fn update_children(&self, dt: f32) {
        for child in self.children() {
            child.widget.get().borrow_mut().update(dt);
        }
    }

    pub fn paint_children(&self, parent_canvas: &mut dyn PainterTrait) {
        for child in self.children() {
            let mut borrowed_child = child.widget.get().borrow_mut();
            let mut sub_canvas = parent_canvas.sub_painter(&child.transform);
            borrowed_child.paint(&mut *sub_canvas);
        }
    }
}
