pub mod poly_ui {
    pub mod widgets;
    pub mod layouts;
    pub mod components;

    // tests
    //********************************************************************************************
    //********************************************************************************************
    //********************************************************************************************
    #[cfg(test)]
    mod tests {
        use nalgebra::Vector3;
        use std::{rc::Rc, cell::RefCell};
        
        use super::widgets::{Widget, BaseWidget};
        use super::layouts::CanvasLayout;

        // test cases
        //****************************************************************************************
        //****************************************************************************************
        //****************************************************************************************
        #[test]
        fn widget_add_child() {
            let mut parent_widget = BaseWidget::new();
            let child_widget = Rc::new(RefCell::new(BaseWidget::new()));

            parent_widget.hierarchy_mut().add(child_widget.clone());

            assert_eq!(parent_widget.hierarchy().children()[0].borrow().id(), child_widget.borrow().id());
        }

        //****************************************************************************************
        #[test]
        fn widget_remove_child() {
            let mut parent_widget = BaseWidget::new();
            let child_widget_1 = Rc::new(RefCell::new(BaseWidget::new())) as Rc<RefCell<dyn Widget>>;
            let child_widget_2 = Rc::new(RefCell::new(BaseWidget::new())) as Rc<RefCell<dyn Widget>>;

            parent_widget.hierarchy_mut().add(child_widget_1.clone());
            parent_widget.hierarchy_mut().add(child_widget_2.clone());
            parent_widget.hierarchy_mut().remove(&child_widget_1);

            assert_eq!(parent_widget.hierarchy().children()[0].borrow().id(), child_widget_2.borrow().id());
        }

        //****************************************************************************************
        #[test]
        fn canvas_layout_add_child() {
            let mut parent_widget = BaseWidget::new();
            parent_widget.set_layout(Box::new(CanvasLayout::new()));
            let child_widget = Rc::new(RefCell::new(BaseWidget::new()));

            parent_widget.layout_mut().add(child_widget.clone(), Vector3::<i32>::new(1, 2, 0));
            
            assert_eq!(parent_widget.hierarchy().children()[0].borrow().id(), child_widget.borrow().id());
        }
    }
}
