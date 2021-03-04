// std
use std::fmt::Debug;
use uuid::Uuid;
// crate
use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::components::Transform;
// super
use super::NewWidget;
use super::OwnedWidget;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
/// Base trait for all widgets. User can't add or remove child in widget of unknown type.
pub trait WidgetTrait: Debug {
    /// # Returns
    /// Id of this widget.All widgets are identified by their id.
    fn id(&self) -> &Uuid;

    /// Removes child with maching id. If there is no child with matching id the function wil
    /// panic.
    /// # Returns
    /// Removed child.
    fn remove_child(&mut self, child: &Uuid) -> OwnedWidget;

    /// # Returns
    /// Hierarchy of this Widget. From there youc an access all children of this widget and their
    /// transforms.
    fn get_hierarchy(&self) -> &Hierarchy;

    /// Convenience function for obtaining transform of specified child.
    /// # Arguments
    /// * `child` - id of child which transform we want to get.
    /// # Returns
    /// Transform of requested child.  
    fn get_child_transform(&self, child: &Uuid) -> &Transform {
        self.get_hierarchy().get_transform(child)
    }

    /// This function updateds state of this widget if necessary. This Widget will also call
    /// update on all its children Widgets.
    /// # Arguments
    /// * `dt` - delta time from the last update in milliseconds
    fn update(&mut self, dt: f32);

    /// This widget will paint itself oin this function call. It will also call paint on all its
    /// children Widgets
    /// # Arguments
    /// * `canvas` - canvas on which this widget should paint itself
    fn paint(&mut self, canvas: &mut dyn PainterTrait);
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

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
/// Empty implementation of WidgetTRait used for testing purposes.
#[derive(Debug)]
pub struct MockWidget {
    pub update_call_count: u32,
    pub paint_call_count: u32,

    id: Uuid,
    hierarchy: Hierarchy,
}

//************************************************************************************************
impl MockWidget {
    pub fn new_raw() -> Self {
        Self {
            update_call_count: 0,
            paint_call_count: 0,
            id: Uuid::new_v4(),
            hierarchy: Hierarchy::default(),
        }
    }

    pub fn new() -> NewWidget<Self> {
        NewWidget::new(Self::new_raw())
    }
}

//************************************************************************************************
impl WidgetTrait for MockWidget {
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
        self.update_call_count += 1;
    }

    fn paint(&mut self, painter: &mut dyn PainterTrait) {
        self.hierarchy.paint_children(painter);

        let const_self = self as *const Self;
        let mut mut_self = const_self as *mut Self;

        unsafe {
            (*mut_self).paint_call_count += 1;
        }
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[cfg(test)]
mod tests {
    // crate
    use crate::poly_ui::app::MockPainter;
    // super
    use super::*;

    //********************************************************************************************
    #[test]
    fn update() {
        let mock = MockWidget::new();

        mock.borrow_mut().update(0.0);
        assert_eq!(mock.borrow().update_call_count, 1);
    }

    //********************************************************************************************
    #[test]
    fn paint() {
        let mock = MockWidget::new();
        let mut painter = MockPainter::default();

        mock.borrow_mut().paint(&mut painter);
        mock.borrow_mut().paint(&mut painter);
        mock.borrow_mut().paint(&mut painter);
        assert_eq!(mock.borrow().paint_call_count, 3);
    }
}
