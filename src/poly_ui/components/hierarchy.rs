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
/// Helper structure containing Owned Widget and its Transform in parent. 
#[derive(Debug)]
pub struct HierarchyChild {
    pub widget: Owned,
    pub transform: Transform,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
/// Hierarchy is used to store child Widgets and their Transforms.
/// 
/// Transform is stored here not in the child Widget itself because it shouldn't be possible for 
/// any entity other than parent Widget to change Transform of any of its children. The main 
/// drawback is that child Widgets get their size in WidgetTrait::paint() so it might be a bit
/// counterintuitive to handle it this way fot the Widget developer.
#[derive(Debug, Default)]
pub struct Hierarchy {
    children: Vec<HierarchyChild>,
}

//************************************************************************************************
impl Hierarchy {
    // add / remove

    /// Adds provided Ownerless child to the Hierarchy. Children added using this method have 
    /// default Transform (pos:[0, 0]; size:[0, 0]). Can't add the same widget for the second 
    /// time (widgets are identified by WidgetTrait::id()).
    /// # Arguments
    /// * `child` - Widget to add
    pub fn add(&mut self, child: Ownerless) {
        assert_eq!(self.index(child.borrow().id()).is_none(), true);
        
        self.children.push(HierarchyChild {
            widget: child.make_owned(),
            transform: Transform::default(),
        });
    }

    /// Adds provided Ownerless Widget to the hierarchy and sets its transform to provided one.
    /// Can't add the same widget for the second time (widgets are identified by 
    /// WidgetTrait::id()).
    /// # Arguments
    /// * `child` - Widget to add
    /// * `transform` - initial transform for newly added widget
    pub fn add_with_transform(&mut self, child: Ownerless, transform: &Transform) {
        assert_eq!(self.index(child.borrow().id()).is_none(), true);

        self.children.push(HierarchyChild {
            widget: child.make_owned(),
            transform: *transform,
        });
    }

    /// Removes Widget with provided id from the hierarchy. Can't remove Widget that was not added
    /// to the Hierarchy.
    /// # Arguments
    /// * `id` - id of the widget that should be removed
    /// # Returns
    /// Ownerless Widget that was removed from the Hierarchy
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
