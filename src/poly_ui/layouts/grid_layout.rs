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
// super
use super::Item;
use super::Layout;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
/// Layout with rows and columns. They can have min/max sizes and stretch factors. Each cell can
/// get its own widget or can be empty
#[derive(Debug)]
pub struct GridLayout {
    id: Uuid,
    hierarchy: Hierarchy,

    column_layout: Layout,
    is_column_layout_dirty: bool,
    row_layout: Layout,
    is_row_layout_dirty: bool,

    children_columns_rows: Vec<Vec<Option<Uuid>>>,
}

//************************************************************************************************
impl GridLayout {
    pub fn new_raw() -> Self {
        Self {
            id: Uuid::new_v4(),
            hierarchy: Hierarchy::default(),
            column_layout: Layout::new(0, vec![]),
            is_column_layout_dirty: true,
            row_layout: Layout::new(0, vec![]),
            is_row_layout_dirty: true,
            children_columns_rows: vec![],
        }
    }

    pub fn new() -> NewWidget<Self> {
        NewWidget::new(Self::new_raw())
    }

    /// Inserts new child widget at specified cell identified by its column and row. If Widget was
    /// already added the function will panic.
    /// # Arguments
    /// * `child` - child Widget that will be added to this layout
    /// * `col` - column at which Widget should be inserted. If this parameter is None then widget
    ///         will be added to the column after the current last column (at column ==
    ///         column_count + 1).
    /// * `row` - row at which Widget should be inserted. If this parameter is None then widget
    ///         will be added to the row after the current last row (at row == row_count + 1).
    pub fn insert_child_at(
        &mut self,
        child: OwnedWidget,
        col: &Option<usize>,
        row: &Option<usize>,
    ) {
        let fixed_col = col.unwrap_or(self.column_layout.items.len());
        let fixed_row = row.unwrap_or(self.row_layout.items.len());
        self.ensure_column_exists(fixed_col);
        self.ensure_row_exists(fixed_row);
        self.children_columns_rows[fixed_col][fixed_row] = Some(*child.borrow().id());
        self.hierarchy
            .add_with_transform(child, &Transform::default());
    }

    /// Sets column count to the provided one. All widget which are present in columns with
    /// indices equal or greater than provided size will be removed.
    /// # Arguments
    /// * `size` - requested column count
    pub fn set_column_count(&mut self, size: usize) {
        self.column_layout
            .items
            .resize(size, Item::new(1, &None, &None));
        self.children_columns_rows
            .resize(size, vec![None; self.row_layout.items.len()]);
        self.is_column_layout_dirty = true;
    }

    /// Checks if there already is a column with provided index. If not then column count is
    /// increased to create column with provided index.
    /// # Arguments
    /// * `col` - index of the column that is ensured to exist after this function call.
    fn ensure_column_exists(&mut self, col: usize) {
        if self.column_layout.items.len() <= col + 1 {
            self.set_column_count(col + 1);
        }
    }

    /// Sets stretch for column with provided index.
    /// # Arguments
    /// * `col` - index of the column which stretch should be set. If column with given index does
    ///         not exist it will be automatically added.
    /// * `stretch` - new stretch value for the column.
    pub fn set_column_stretch(&mut self, col: usize, stretch: u32) {
        self.ensure_column_exists(col);
        self.column_layout.items[col].stretch = stretch;
        self.is_column_layout_dirty = true;
    }

    /// Sets max size for column with provided index.
    /// # Arguments
    /// * `col` - index of the column which size should be set. If column with given index does
    ///         not exist it will be automatically added.
    /// * `size` - new size value for the column.
    pub fn set_column_max_size(&mut self, col: usize, size: u32) {
        self.ensure_column_exists(col);
        self.column_layout.items[col].max_size = size;
        self.is_column_layout_dirty = true;
    }

    /// Sets min size for column with provided index.
    /// # Arguments
    /// * `col` - index of the column which max size should be set. If column with given index
    ///         does not exist it will be automatically added.
    /// * `size` - new min size value for the column.
    pub fn set_column_min_size(&mut self, col: usize, size: u32) {
        self.ensure_column_exists(col);
        self.column_layout.items[col].min_size = size;
        self.is_column_layout_dirty = true;
    }

    /// Sets row count to the provided one. All widgets which are present in rows with indices
    /// equal or greater than provided size will be removed.
    /// # Arguments
    /// * `size` - requested row count.
    pub fn set_row_count(&mut self, size: usize) {
        self.row_layout
            .items
            .resize(size, Item::new(1, &None, &None));
        for column in &mut self.children_columns_rows {
            column.resize(size, None);
        }
        self.is_row_layout_dirty = true;
    }

    /// Checks if there already is a row with provided index. If not then row count is increased
    /// to create row with provided index.
    /// # Arguments
    /// * `row` - index of the row that is ensured to exist after this function call.
    fn ensure_row_exists(&mut self, row: usize) {
        if self.row_layout.items.len() <= row + 1 {
            self.set_row_count(row + 1);
        }
    }

    /// Sets stretch for row with provided index.
    /// # Arguments
    /// * `row` - index of the row which stretch should be set. If row with given index does not
    ///         exist it will be automatically added.
    /// * `stretch` - new stretch value for the row.
    pub fn set_row_stretch(&mut self, row: usize, stretch: u32) {
        self.ensure_row_exists(row);
        self.row_layout.items[row].stretch = stretch;
        self.is_row_layout_dirty = true;
    }

    /// Sets max size for row with provided index.
    /// # Arguments
    /// * `row` - index of the row which size should be set. If row with given index does not
    ///         exist it will be automatically added.
    /// * `size` - new size value for the row.
    pub fn set_row_max_size(&mut self, row: usize, size: u32) {
        self.ensure_row_exists(row);
        self.row_layout.items[row].max_size = size;
        self.is_row_layout_dirty = true;
    }

    /// Sets min size for row with provided index.
    /// # Arguments
    /// * `row` - index of the row which max size should be set. If row with given index does not
    ///         exist it will be automatically added.
    /// * `size` - new min size value for the row.
    pub fn set_row_min_size(&mut self, row: usize, size: u32) {
        self.ensure_row_exists(row);
        self.row_layout.items[row].min_size = size;
        self.is_row_layout_dirty = true;
    }

    /// Items sizes might become invalid after new items are added or the layout is resized. This
    /// function recalculates all items sizes depending on what has changed.
    /// # Arguments
    /// * `size` - size of this layout.
    fn refresh_children_transforms(&mut self, size: &Vector2<u32>) {
        if self.column_layout.size != size.x || self.is_column_layout_dirty {
            self.column_layout.size = size.x;
            self.column_layout.refresh();
        }
        if self.row_layout.size != size.y || self.is_row_layout_dirty {
            self.row_layout.size = size.y;
            self.row_layout.refresh();
        }

        let mut col_offset = 0;

        for col in 0..self.column_layout.items.len() {
            let col_size = self.column_layout.items[col].current_size;
            let mut row_offset = 0;

            for row in 0..self.row_layout.items.len() {
                let row_size = self.row_layout.items[row].current_size;

                match &self.children_columns_rows[col][row] {
                    Some(id) => self.hierarchy.set_transform(
                        id,
                        &Transform::new(
                            &Point2::<i32>::new(col_offset, row_offset),
                            &Vector2::<u32>::new(col_size, row_size),
                        ),
                    ),
                    None => (),
                }
                row_offset += row_size as i32;
            }
            col_offset += col_size as i32;
        }
    }
}

//************************************************************************************************
impl WidgetTrait for GridLayout {
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
        let painter_size = painter.size();
        if self.is_row_layout_dirty
            || self.is_column_layout_dirty
            || self.row_layout.size != painter_size.y
            || self.column_layout.size != painter_size.x
        {
            self.refresh_children_transforms(&painter_size)
        }

        self.hierarchy.paint_children(painter);
    }
}

#[cfg(test)]
mod tests {
    // crate
    use crate::poly_ui::app::MockPainter;
    use crate::poly_ui::widgets::MockWidget;
    // super
    use super::*;

    //********************************************************************************************
    #[test]
    fn insert_child_at() {
        let layout = GridLayout::new();
        let child1 = MockWidget::new();
        let child_ptr1 = child1.get().clone();

        layout
            .borrow_mut()
            .insert_child_at(child1.make_owned(), &None, &None);

        assert_eq!(layout.borrow_mut().get_hierarchy().children().len(), 1);
        assert_eq!(
            layout.borrow_mut().get_hierarchy().children()[0]
                .widget
                .borrow()
                .id(),
            child_ptr1.borrow().id()
        );

        let mut painter = MockPainter::new();
        painter.size = Vector2::<u32>::new(50, 100);
        layout.borrow_mut().paint(&mut painter);

        assert_eq!(
            layout
                .borrow_mut()
                .get_child_transform(&child_ptr1.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(50, 100))
        );

        let child2 = MockWidget::new();
        let child_ptr2 = child2.get().clone();

        layout
            .borrow_mut()
            .insert_child_at(child2.make_owned(), &None, &None);

        assert_eq!(layout.borrow_mut().get_hierarchy().children().len(), 2);
        assert_eq!(
            layout.borrow_mut().get_hierarchy().children()[1]
                .widget
                .borrow()
                .id(),
            child_ptr2.borrow().id()
        );

        let mut painter = MockPainter::new();
        painter.size = Vector2::<u32>::new(800, 600);
        layout.borrow_mut().paint(&mut painter);

        assert_eq!(
            layout
                .borrow_mut()
                .get_child_transform(&child_ptr1.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(400, 300))
        );
        assert_eq!(
            layout
                .borrow_mut()
                .get_child_transform(&child_ptr2.borrow().id().clone()),
            &Transform::new(
                &Point2::<i32>::new(400, 300),
                &Vector2::<u32>::new(400, 300)
            )
        );

        let child3 = MockWidget::new();
        let child_ptr3 = child3.get().clone();

        layout
            .borrow_mut()
            .insert_child_at(child3.make_owned(), &Some(0), &Some(1));

        assert_eq!(layout.borrow_mut().get_hierarchy().children().len(), 3);
        assert_eq!(
            layout.borrow_mut().get_hierarchy().children()[2]
                .widget
                .borrow()
                .id(),
            child_ptr3.borrow().id()
        );

        let mut painter = MockPainter::new();
        painter.size = Vector2::<u32>::new(800, 600);
        layout.borrow_mut().paint(&mut painter);

        assert_eq!(
            layout
                .borrow_mut()
                .get_child_transform(&child_ptr1.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(400, 300))
        );
        assert_eq!(
            layout
                .borrow_mut()
                .get_child_transform(&child_ptr2.borrow().id().clone()),
            &Transform::new(
                &Point2::<i32>::new(400, 300),
                &Vector2::<u32>::new(400, 300)
            )
        );
        assert_eq!(
            layout
                .borrow_mut()
                .get_child_transform(&child_ptr3.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(0, 300), &Vector2::<u32>::new(400, 300))
        );
    }

    //********************************************************************************************
    #[test]
    fn set_column_count() {
        let layout = GridLayout::new();
        let child1 = MockWidget::new();
        let child_ptr1 = child1.get().clone();

        layout.borrow_mut().set_column_count(1);
        assert_eq!(layout.borrow_mut().get_hierarchy().children().len(), 0);

        layout
            .borrow_mut()
            .insert_child_at(child1.make_owned(), &None, &Some(0));

        let mut painter = MockPainter::new();
        painter.size = Vector2::<u32>::new(60, 100);
        layout.borrow_mut().paint(&mut painter);

        assert_eq!(
            layout
                .borrow_mut()
                .get_child_transform(&child_ptr1.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(30, 0), &Vector2::<u32>::new(30, 100))
        );

        layout.borrow_mut().set_column_count(3);

        layout.borrow_mut().paint(&mut painter);

        assert_eq!(
            layout
                .borrow_mut()
                .get_child_transform(&child_ptr1.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(20, 0), &Vector2::<u32>::new(20, 100))
        );
    }

    //********************************************************************************************
    #[test]
    fn set_column_stretch() {
        let layout = GridLayout::new();
        let child1 = MockWidget::new();
        let child2 = MockWidget::new();
        let child_ptr1 = child1.get().clone();
        let child_ptr2 = child2.get().clone();

        layout
            .borrow_mut()
            .insert_child_at(child1.make_owned(), &None, &Some(0));
        layout.borrow_mut().set_column_stretch(0, 7);

        layout
            .borrow_mut()
            .insert_child_at(child2.make_owned(), &None, &Some(0));
        layout.borrow_mut().set_column_stretch(1, 3);
        layout.borrow_mut().set_column_stretch(2, 10);

        let mut painter = MockPainter::new();
        painter.size = Vector2::<u32>::new(100, 100);
        layout.borrow_mut().paint(&mut painter);

        assert_eq!(
            layout
                .borrow()
                .get_child_transform(&child_ptr1.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(35, 100))
        );
        assert_eq!(
            layout
                .borrow()
                .get_child_transform(&child_ptr2.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(35, 0), &Vector2::<u32>::new(15, 100))
        );
    }

    //********************************************************************************************
    #[test]
    fn set_column_max_size() {
        let layout = GridLayout::new();
        let child1 = MockWidget::new();
        let child2 = MockWidget::new();
        let child_ptr1 = child1.get().clone();
        let child_ptr2 = child2.get().clone();

        layout
            .borrow_mut()
            .insert_child_at(child1.make_owned(), &None, &Some(0));
        layout.borrow_mut().set_column_max_size(0, 10);

        layout
            .borrow_mut()
            .insert_child_at(child2.make_owned(), &None, &Some(0));
        layout.borrow_mut().set_column_max_size(2, 20);

        let mut painter = MockPainter::new();
        painter.size = Vector2::<u32>::new(100, 100);
        layout.borrow_mut().paint(&mut painter);

        assert_eq!(
            layout
                .borrow()
                .get_child_transform(&child_ptr1.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(10, 100))
        );
        assert_eq!(
            layout
                .borrow()
                .get_child_transform(&child_ptr2.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(10, 0), &Vector2::<u32>::new(70, 100))
        );
    }

    //********************************************************************************************
    #[test]
    fn set_column_min_size() {
        let layout = GridLayout::new();
        let child1 = MockWidget::new();
        let child2 = MockWidget::new();
        let child_ptr1 = child1.get().clone();
        let child_ptr2 = child2.get().clone();

        layout
            .borrow_mut()
            .insert_child_at(child1.make_owned(), &None, &Some(0));
        layout.borrow_mut().set_column_min_size(0, 50);

        layout
            .borrow_mut()
            .insert_child_at(child2.make_owned(), &None, &Some(0));
        layout.borrow_mut().set_column_min_size(2, 30);

        let mut painter = MockPainter::new();
        painter.size = Vector2::<u32>::new(100, 100);
        layout.borrow_mut().paint(&mut painter);

        assert_eq!(
            layout
                .borrow()
                .get_child_transform(&child_ptr1.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(50, 100))
        );
        assert_eq!(
            layout
                .borrow()
                .get_child_transform(&child_ptr2.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(50, 0), &Vector2::<u32>::new(20, 100))
        );
    }

    //********************************************************************************************
    #[test]
    fn set_row_count() {
        let layout = GridLayout::new();
        let child1 = MockWidget::new();
        let child_ptr1 = child1.get().clone();

        layout.borrow_mut().set_row_count(1);
        assert_eq!(layout.borrow_mut().get_hierarchy().children().len(), 0);

        layout
            .borrow_mut()
            .insert_child_at(child1.make_owned(), &Some(0), &None);

        let mut painter = MockPainter::new();
        painter.size = Vector2::<u32>::new(100, 60);
        layout.borrow_mut().paint(&mut painter);

        assert_eq!(
            layout
                .borrow_mut()
                .get_child_transform(&child_ptr1.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(0, 30), &Vector2::<u32>::new(100, 30))
        );

        layout.borrow_mut().set_row_count(3);

        layout.borrow_mut().paint(&mut painter);

        assert_eq!(
            layout
                .borrow_mut()
                .get_child_transform(&child_ptr1.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(0, 20), &Vector2::<u32>::new(100, 20))
        );
    }

    //********************************************************************************************
    #[test]
    fn set_row_stretch() {
        let layout = GridLayout::new();
        let child1 = MockWidget::new();
        let child2 = MockWidget::new();
        let child_ptr1 = child1.get().clone();
        let child_ptr2 = child2.get().clone();

        layout
            .borrow_mut()
            .insert_child_at(child1.make_owned(), &Some(0), &None);
        layout.borrow_mut().set_row_stretch(0, 7);

        layout
            .borrow_mut()
            .insert_child_at(child2.make_owned(), &Some(0), &None);
        layout.borrow_mut().set_row_stretch(1, 3);
        layout.borrow_mut().set_row_stretch(2, 10);

        let mut painter = MockPainter::new();
        painter.size = Vector2::<u32>::new(100, 100);
        layout.borrow_mut().paint(&mut painter);

        assert_eq!(
            layout
                .borrow()
                .get_child_transform(&child_ptr1.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(100, 35))
        );
        assert_eq!(
            layout
                .borrow()
                .get_child_transform(&child_ptr2.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(0, 35), &Vector2::<u32>::new(100, 15))
        );
    }

    //********************************************************************************************
    #[test]
    fn set_row_max_size() {
        let layout = GridLayout::new();
        let child1 = MockWidget::new();
        let child2 = MockWidget::new();
        let child_ptr1 = child1.get().clone();
        let child_ptr2 = child2.get().clone();

        layout
            .borrow_mut()
            .insert_child_at(child1.make_owned(), &Some(0), &None);
        layout.borrow_mut().set_row_max_size(0, 10);

        layout
            .borrow_mut()
            .insert_child_at(child2.make_owned(), &Some(0), &None);
        layout.borrow_mut().set_row_max_size(2, 20);

        let mut painter = MockPainter::new();
        painter.size = Vector2::<u32>::new(100, 100);
        layout.borrow_mut().paint(&mut painter);

        assert_eq!(
            layout
                .borrow()
                .get_child_transform(&child_ptr1.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(100, 10))
        );
        assert_eq!(
            layout
                .borrow()
                .get_child_transform(&child_ptr2.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(0, 10), &Vector2::<u32>::new(100, 70))
        );
    }

    //********************************************************************************************
    #[test]
    fn set_row_min_size() {
        let layout = GridLayout::new();
        let child1 = MockWidget::new();
        let child2 = MockWidget::new();
        let child_ptr1 = child1.get().clone();
        let child_ptr2 = child2.get().clone();

        layout
            .borrow_mut()
            .insert_child_at(child1.make_owned(), &Some(0), &None);
        layout.borrow_mut().set_row_min_size(0, 50);

        layout
            .borrow_mut()
            .insert_child_at(child2.make_owned(), &Some(0), &None);
        layout.borrow_mut().set_row_min_size(2, 30);

        let mut painter = MockPainter::new();
        painter.size = Vector2::<u32>::new(100, 100);
        layout.borrow_mut().paint(&mut painter);

        assert_eq!(
            layout
                .borrow()
                .get_child_transform(&child_ptr1.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(0, 0), &Vector2::<u32>::new(100, 50))
        );
        assert_eq!(
            layout
                .borrow()
                .get_child_transform(&child_ptr2.borrow().id().clone()),
            &Transform::new(&Point2::<i32>::new(0, 50), &Vector2::<u32>::new(100, 20))
        );
    }
}
