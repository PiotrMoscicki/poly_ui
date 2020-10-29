use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::poly_ui::widgets::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Hierarchy {
    children: Vec<Rc<RefCell<dyn WidgetTrait>>>,
}

//************************************************************************************************
impl Hierarchy {
    pub fn new() -> Self {
        return Self {
            children: Vec::new(),
        };
    }

    pub fn add(&mut self, child: Rc<RefCell<dyn WidgetTrait>>) {
        self.children.push(child);
    }

    pub fn remove(&mut self, child: &Rc<RefCell<dyn WidgetTrait>>) {
        self.children.remove(
            self.children
                .iter()
                .position(|elem| elem.borrow().id() == child.borrow().id())
                .unwrap(),
        );
    }

    pub fn children(&self) -> &Vec<Rc<RefCell<dyn WidgetTrait>>> {
        return &self.children;
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::poly_ui::widgets::{Widget, WidgetTrait};

    //********************************************************************************************
    #[test]
    fn hierarchy_add_child() {
        let mut parent_widget = Widget::new();
        let child_widget = Rc::new(RefCell::new(Widget::new()));
        parent_widget.hierarchy_mut().add(child_widget.clone());
        assert_eq!(
            parent_widget.hierarchy().children()[0].borrow().id(),
            child_widget.borrow().id()
        );
    }

    //********************************************************************************************
    #[test]
    fn hierarchy_remove_child() {
        let mut parent_widget = Widget::new();
        let child_widget_1 = Rc::new(RefCell::new(Widget::new())) as Rc<RefCell<dyn WidgetTrait>>;
        let child_widget_2 = Rc::new(RefCell::new(Widget::new())) as Rc<RefCell<dyn WidgetTrait>>;
        parent_widget.hierarchy_mut().add(child_widget_1.clone());
        parent_widget.hierarchy_mut().add(child_widget_2.clone());
        parent_widget.hierarchy_mut().remove(&child_widget_1);
        assert_eq!(
            parent_widget.hierarchy().children()[0].borrow().id(),
            child_widget_2.borrow().id()
        );
    }
}
