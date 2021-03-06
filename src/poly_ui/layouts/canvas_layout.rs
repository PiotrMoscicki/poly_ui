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
use crate::poly_ui::widgets::OwnedWidget;
use crate::poly_ui::widgets::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
/// CanvasLayout allows user for explicitly setting position and size of children widgets. They
/// will remain unchanged when this layout size will change.
#[derive(Debug)]
pub struct CanvasLayout {
    id: Uuid,
    hierarchy: Hierarchy,
}

//************************************************************************************************
impl CanvasLayout {
    pub fn new_raw() -> Self {
        Self {
            id: Uuid::new_v4(),
            hierarchy: Hierarchy::default(),
        }
    }

    pub fn new() -> NewWidget<Self> {
        NewWidget::new(Self::new_raw())
    }

    /// Adds child to this layout with defined transform. Child widget is appended to the
    /// hierarchy so it gets the last index and so it will be painted and updated as last.
    /// # Arguments
    /// * `child` - child widget that will be added to this layout
    /// * `transform' - initial Transform for newly added child
    pub fn add_child_with_transform(&mut self, child: OwnedWidget, transform: &Transform) {
        self.hierarchy.add_with_transform(child, transform);
    }

    /// Sets new Transfrom for the child widget with the given id. If there is no child with such
    /// id the function will panic.
    /// * `child` - id of thechild that should get new Transform
    /// * `new_transform` - new Transform for the child
    pub fn set_child_transform(&mut self, child: &Uuid, new_transform: &Transform) {
        self.hierarchy.set_transform(child, new_transform);
    }

    /// Sets new position for the child widget with the given id. If there is no child with such
    /// id the function will panic.
    /// * `child` - id of thechild that should get new Transform
    /// * `new_pos` - new position for the child
    pub fn set_child_pos(&mut self, child: &Uuid, new_pos: &Point2<i32>) {
        self.hierarchy.set_pos(child, new_pos);
    }

    /// Sets new size for the child widget with the given id. If there is no child with such id
    /// the function will panic.
    /// * `child` - id of thechild that should get new Transform
    /// * `new_size` - new size for the child
    pub fn set_child_size(&mut self, child: &Uuid, new_size: &Vector2<u32>) {
        self.hierarchy.set_size(child, new_size);
    }
}

//************************************************************************************************
impl WidgetTrait for CanvasLayout {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn remove_child(&mut self, child: &Uuid) -> OwnedWidget {
        self.hierarchy.remove(child)
    }

    fn get_hierarchy(&self) -> &Hierarchy {
        &self.hierarchy
    }

    fn update(&mut self, dt: f32) {
        self.hierarchy.update_children(dt);
    }

    fn paint(&mut self, painter: &mut dyn PainterTrait) {
        self.hierarchy.paint_children(painter);
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[cfg(test)]
mod tests {
    // crate
    use crate::poly_ui::app::MockPainter;
    use crate::poly_ui::widgets::MockWidget;
    // super
    use super::*;

    //********************************************************************************************
    #[test]
    fn empty_layout() {
        let layout = CanvasLayout::new();

        assert_eq!(layout.borrow().get_hierarchy().children().len(), 0);
    }

    //********************************************************************************************
    #[test]
    fn add_child_with_transform() {
        let layout = CanvasLayout::new();
        let child = CanvasLayout::new();
        let child_id = *child.borrow().id();
        let transform = Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(0, 0));
        layout
            .get()
            .borrow_mut()
            .add_child_with_transform(child.make_owned(), &transform);

        assert_eq!(layout.borrow().get_child_transform(&child_id), &transform);
    }

    //********************************************************************************************
    #[test]
    fn remove_child() {
        let layout = CanvasLayout::new();
        let child1 = CanvasLayout::new();
        let child2 = CanvasLayout::new();
        let child1_id = *child1.borrow().id();
        let child2_id = *child2.borrow().id();
        let transform = Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(0, 0));
        layout
            .borrow_mut()
            .add_child_with_transform(child1.make_owned(), &transform);
        layout
            .borrow_mut()
            .add_child_with_transform(child2.make_owned(), &transform);
        layout.borrow_mut().remove_child(&child1_id);

        assert_eq!(layout.borrow().get_hierarchy().index(&child1_id), None);
        assert_eq!(layout.borrow().get_hierarchy().index(&child2_id), Some(0));
    }

    //********************************************************************************************
    #[test]
    fn set_child_transform() {
        let layout = CanvasLayout::new();
        let child = CanvasLayout::new();
        let child_id = *child.borrow().id();
        let transform = Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(0, 0));
        layout
            .borrow_mut()
            .add_child_with_transform(child.make_owned(), &transform);

        let transform = Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(0, 0));
        layout
            .borrow_mut()
            .set_child_transform(&child_id, &transform);

        assert_eq!(layout.borrow().get_child_transform(&child_id), &transform);
        assert_eq!(
            layout.borrow().get_hierarchy().get_transform(&child_id),
            &transform
        );
    }

    //********************************************************************************************
    #[test]
    fn set_child_pos() {
        let layout = CanvasLayout::new();
        let child = CanvasLayout::new();
        let child_id = *child.borrow().id();
        let transform = Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(0, 0));
        layout
            .borrow_mut()
            .add_child_with_transform(child.make_owned(), &transform);

        let transform = Transform::new(&Point2::<i32>::new(5, 10), &Vector2::<u32>::new(0, 0));
        layout.borrow_mut().set_child_pos(&child_id, &transform.pos);

        assert_eq!(layout.borrow().get_child_transform(&child_id), &transform);
        assert_eq!(
            layout.borrow().get_hierarchy().get_transform(&child_id),
            &transform
        );
    }

    //********************************************************************************************
    #[test]
    fn set_child_size() {
        let layout = CanvasLayout::new();
        let child = CanvasLayout::new();
        let child_id = *child.borrow().id();
        let transform = Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(0, 0));
        layout
            .borrow_mut()
            .add_child_with_transform(child.make_owned(), &transform);

        let transform = Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(10, 20));
        layout
            .borrow_mut()
            .set_child_size(&child_id, &transform.size);

        assert_eq!(layout.borrow().get_child_transform(&child_id), &transform);
        assert_eq!(
            layout.borrow().get_hierarchy().get_transform(&child_id),
            &transform
        );
    }

    //********************************************************************************************
    #[test]
    fn update() {
        let layout = CanvasLayout::new();
        let child = MockWidget::new();
        let child_ptr = child.get().clone();
        let transform = Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(0, 0));
        layout
            .borrow_mut()
            .add_child_with_transform(child.make_owned(), &transform);

        assert_eq!(child_ptr.borrow().update_call_count, 0);

        layout.borrow_mut().update(1.0);
        layout.borrow_mut().update(1.0);

        assert_eq!(child_ptr.borrow().update_call_count, 2);
    }

    //********************************************************************************************
    #[test]
    fn paint() {
        let layout = CanvasLayout::new();
        let child = MockWidget::new();
        let mut painter = MockPainter::default();
        let child_ptr = child.get().clone();
        let transform = Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(0, 0));
        layout
            .borrow_mut()
            .add_child_with_transform(child.make_owned(), &transform);

        assert_eq!(child_ptr.borrow().paint_call_count, 0);

        layout.borrow_mut().paint(&mut painter);
        layout.borrow_mut().paint(&mut painter);
        layout.borrow_mut().paint(&mut painter);
        layout.borrow_mut().paint(&mut painter);

        assert_eq!(child_ptr.borrow().paint_call_count, 4);
    }
}
