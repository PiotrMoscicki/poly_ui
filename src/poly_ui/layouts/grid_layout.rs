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
// super
use super::Item;
use super::Layout;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct GridLayout {
    id: Uuid,
    hierarchy: Hierarchy,

    row_layout: Layout,
    is_row_layout_dirty: bool,
    column_layout: Layout,
    is_column_layout_dirty: bool,

    children: Vec<Vec<Uuid>>,
}

//************************************************************************************************
impl GridLayout {
    pub fn new_raw() -> Self {
        Self {
            id: Uuid::new_v4(),
            hierarchy: Hierarchy::default(),
            row_layout: Layout::new(0, vec!()),
            is_row_layout_dirty: true,
            column_layout: Layout::new(0, vec!()),
            is_column_layout_dirty: true,
            children: vec!(),
        }
    }

    pub fn new() -> NewWidget<Self> {
        NewWidget::new(Self::new_raw())
    }

    pub fn insert_child_at(&mut self, child: Ownerless, row: &Option<u32>, col: &Option<u32>) {
        self.hierarchy.add_with_transform(child, &Transform::default());

    }


    pub fn set_row_count(&mut self, size: usize) {
        self.row_layout.items.resize(size, Item::new(1, &None, &None));
        self.is_row_layout_dirty = true;
    }
    
    pub fn set_row_stretch(&mut self, row: u32, stretch: u32) {

    }
    
    pub fn set_row_max_size(&mut self, row: u32, size: u32) {

    }
    
    pub fn set_row_min_size(&mut self, row: u32, size: u32) {
    }
    

    pub fn set_column_count(&mut self, size: usize) {
        self.column_layout.items.resize(size, Item::new(1, &None, &None));
        self.is_column_layout_dirty = true;
    }

    pub fn set_column_stretch(&mut self, col: u32, stretch: u32) {
        
    }
    
    pub fn set_column_max_size(&mut self, col: u32, size: u32) {
        
    }
    
    pub fn set_column_min_size(&mut self, col: u32, size: u32) {

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

    fn paint(&mut self, painter: &mut dyn PainterTrait) {
        self.hierarchy.paint_children(painter);
    }
}

#[cfg(test)]
mod tests {
    // crate
    use crate::poly_ui::widgets::MockWidget;
    use crate::poly_ui::app::MockPainter;
    // super
    use super::*;

    //********************************************************************************************
    #[test]
    fn insert_child_at() {
        let layout = GridLayout::new();
        let child = MockWidget::new();
        let child_ptr = child.get().clone();

        layout.borrow_mut().insert_child_at(child.make_ownerless(), &None, &None);

        assert_eq!(layout.borrow_mut().get_hierarchy().children().len(), 1);
        assert_eq!(
            layout.borrow_mut().get_hierarchy().children()[0].widget.borrow().id(), 
            child_ptr.borrow().id()
        );

        let mut painter = MockPainter{};
        layout.borrow_mut().paint(&mut painter);
        Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(0, 0));
        assert_eq!(
            layout.borrow_mut().get_child_transform(&child_ptr.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(0, 0))
        );
    }
}
