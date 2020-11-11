// std
use std::{
    rc::Rc,
    cell::RefCell,
};
// deps
use nalgebra::Point2;
use nalgebra::Vector2;
use uuid::Uuid;
// crate
use crate::poly_ui::app::PainterTrait;
// super
use super::WidgetTrait;
use super::Widget;
use super::Ownerless;
use super::NewWidget;

#[derive(Debug)]
pub enum LinearLayoutWidgetDirection {
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
    base: Widget,
    dir: LinearLayoutWidgetDirection,
    order: Vec<Uuid>,
    stretch: Vec<u32>,
    min_size: Vec<Option<u32>>,
    max_size: Vec<Option<u32>>,
}

//************************************************************************************************
impl LinearLayoutWidget {
    pub fn new_raw() -> Self {
        return Self {
            base: Widget::new_raw(),
            dir: LinearLayoutWidgetDirection::LeftToRight,
            order: Vec::new(),
            stretch: Vec::new(),
            min_size: Vec::new(),
            max_size: Vec::new(),
        };
    }

    pub fn new() -> Ownerless {
        return NewWidget::new(Self::new_raw()).to_ownerless();
    }
}

//************************************************************************************************
impl WidgetTrait for LinearLayoutWidget {
    fn id(&self) -> &Uuid {
        return self.base.id();
    }

    fn pos(&self) -> &Point2<i32> {
        return self.base.pos();
    }

    fn set_pos(&mut self, value: &Point2<i32>) {
        self.base.set_pos(value);
    }

    fn size(&self) -> &Vector2<u32> {
        return self.base.size();
    }

    fn set_size(&mut self, value: &Vector2<u32>) {
        self.base.set_size(value);
    }

    fn add(&mut self, child: Ownerless) {
        self.base.add(child);
    }

    fn remove(&mut self, child: &Rc<RefCell<dyn WidgetTrait>>) -> Ownerless {
        return self.base.remove(child);
    }

    fn update(&mut self, dt: f32) {
        // ensure child widgets have correct transforms

        self.base.update(dt);
    }
    
    fn paint(&self, painter: &mut dyn PainterTrait) {
        self.base.paint(painter);
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[cfg(test)]
mod tests {
    // crate
    use crate::poly_ui::widgets::Widget;
    use crate::poly_ui::widgets::WidgetTrait;
    use crate::poly_ui::widgets::Ownerless;
    use crate::poly_ui::widgets::NewWidget;
    // super
    use super::LinearLayoutWidget;

    use std::{
        cell::RefCell,
        rc::Rc,
    };

    //********************************************************************************************
    #[test]
    fn add_child() {
        let a = Box::new(3);

        let mut parent_widget = LinearLayoutWidget::new_raw();
        let child_widget = NewWidget::new(Widget::new_raw());
        parent_widget.add(child_widget.to_ownerless());
        //     .add(child_widget.clone());
        // assert_eq!(
        //     parent_widget.borrow().hierarchy().children()[0]
        //         .borrow()
        //         .id(),
        //     child_widget.borrow().id()
        // );
    }
}