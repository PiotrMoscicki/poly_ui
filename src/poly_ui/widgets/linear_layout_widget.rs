// std
use std::{cell::RefCell, rc::Rc};
// deps
use nalgebra::Point2;
use nalgebra::Vector2;
use uuid::Uuid;
// crate
use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::components::Hierarchy;
// super
use super::NewWidget;
use super::Ownerless;
use super::update_children;
use super::paint_children;
use super::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub enum LinearLayoutDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BotomToTop,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct LinearLayoutWidget {
    id: Uuid,
    pos: Point2<i32>,
    size: Vector2<u32>,
    hierarchy: Hierarchy,

    dir: LinearLayoutDirection,
    order: Vec<Uuid>,
    stretch: Vec<u32>,
    min_size: Vec<Option<u32>>,
    max_size: Vec<Option<u32>>,
}

//************************************************************************************************
impl LinearLayoutWidget {
    pub fn new_raw() -> Self {
        return Self {
            id: Uuid::new_v4(),
            pos: Point2::<i32>::new(0, 0),
            size: Vector2::<u32>::new(0, 0),
            hierarchy: Hierarchy::new(),

            dir: LinearLayoutDirection::LeftToRight,
            order: Vec::new(),
            stretch: Vec::new(),
            min_size: Vec::new(),
            max_size: Vec::new(),
        };
    }

    pub fn new() -> NewWidget<Self> {
        return NewWidget::new(Self::new_raw());
    }

    pub fn set_dir(&mut self, dir: LinearLayoutDirection) {
        self.dir = dir;
    }

    fn update_child_transform(&self, child: &dyn WidgetTrait) {
        let index = self.hierarchy.index(child.id());

        
    }
}

//************************************************************************************************
impl WidgetTrait for LinearLayoutWidget {
    fn id(&self) -> &Uuid {
        return &self.id;
    }

    fn pos(&self) -> &Point2<i32> {
        return &self.pos;
    }

    fn set_pos(&mut self, value: &Point2<i32>) {
        self.pos = *value;
    }

    fn size(&self) -> &Vector2<u32> {
        return &self.size;
    }

    fn set_size(&mut self, value: &Vector2<u32>) {
        self.size = *value;
    }

    fn add(&mut self, child: Ownerless) {
        {
            let mut ref_mut = child.get().borrow_mut();
            ref_mut.set_pos(&Point2::<i32>::new(0, 0));
            ref_mut.set_size(&Vector2::<u32>::new(60, 20));
        }

        self.hierarchy.add(child);

        for child in self.hierarchy.children() {
            self.update_child_transform(&*child.get().borrow_mut());
        }
    }

    fn remove(&mut self, child: &Rc<RefCell<dyn WidgetTrait>>) -> Ownerless {
        return self.hierarchy.remove(&*child.borrow().id());
    }

    fn update(&mut self, dt: f32) {
        // ensure child widgets have correct transforms
        match &self.dir {
            LinearLayoutDirection::LeftToRight => {}
            LinearLayoutDirection::RightToLeft => {}
            LinearLayoutDirection::TopToBottom => {}
            LinearLayoutDirection::BotomToTop => {}
        }

        println!("update widget");
        update_children(&self.hierarchy, dt);
    }

    fn paint(&self, painter: &mut dyn PainterTrait) {
        println!("paint widget");
        paint_children(&self.hierarchy, painter);
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[cfg(test)]
mod tests {
    // deps
    use nalgebra::Point2;
    use nalgebra::Vector2;
    // crate
    use crate::poly_ui::widgets::Widget;
    use crate::poly_ui::widgets::WidgetTrait;
    // super
    use super::LinearLayoutWidget;
    use super::LinearLayoutDirection;

    //********************************************************************************************
    #[test]
    fn check_children_transforms_left_to_right_dir() {
        let mut parent_widget = LinearLayoutWidget::new_raw();
        parent_widget.set_size(&Vector2::<u32>::new(60, 20));
        parent_widget.set_dir(LinearLayoutDirection::LeftToRight);

        let new_child_widget = Widget::new();
        let child_widget = new_child_widget.get().clone();
        parent_widget.add(new_child_widget.to_ownerless());

        assert_eq!(
            child_widget.borrow().pos(),
            &Point2::<i32>::new(0, 0)
        );
        assert_eq!(
            child_widget.borrow().size(),
            &Vector2::<u32>::new(60, 20)
        );
    }
}
