pub mod poly_ui {
    use nalgebra::Vector3;
    use std::{collections::HashMap, fmt::Debug, rc::Rc};

    // traits
    //********************************************************************************************
    //********************************************************************************************
    //********************************************************************************************
    pub trait Layout: Debug {
        fn add(&mut self, child: Rc<dyn Widget>, pos: Vector3<i32>);
    }

    //********************************************************************************************
    pub trait Widget: Debug {
        fn pos(&self) -> &Vector3<i32>;

        fn hierarchy(&self) -> &Hierarchy;
        fn hierarchy_mut(&mut self) -> &mut Hierarchy;

        fn set_layout(&mut self, layout: Box<dyn Layout>);
        fn layout(&self) -> &dyn Layout;
        fn layout_mut(&mut self) -> &mut dyn Layout;
    }

    // structs
    //********************************************************************************************
    //********************************************************************************************
    //********************************************************************************************
    #[derive(Debug)]
    pub struct Transform {
        pos: Vector3<i32>,
        size: Vector3<i32>,
    }

    #[derive(Debug)]
    pub struct Hierarchy {
        children: Vec<Rc<dyn Widget>>,
    }

    impl Hierarchy {
        pub fn new() -> Self {
            return Self {
                children: Vec::new(),
            };
        }

        pub fn add(&mut self, child: Rc<dyn Widget>) {
            self.children.push(child);
        }

        pub fn remove(&mut self, child: &dyn Widget) {
            self.children.remove(
                self.children
                    .iter()
                    .position(|elem| std::ptr::eq(elem.as_ref(), child))
                    .unwrap(),
            );
        }

        pub fn children(&self) -> &Vec<Rc<dyn Widget>> {
            return &self.children;
        }
    }

    //********************************************************************************************
    #[derive(Debug)]
    pub struct CanvasLayout {
        children: HashMap<Box<Rc<dyn Widget>>, Vector3<i32>>,
    }

    impl CanvasLayout {
        pub fn new() -> Self {
            return Self {
                children: HashMap::new(),
            };
        }
    }

    impl Layout for CanvasLayout {
        fn add(&mut self, child: Rc<dyn Widget>, pos: Vector3<i32>) {
            //match self.children.get(&child) {
            //    Some(&mut childPos) => childPos = pos,
            //    None => self.children.insert(child, pos),
            //}
        }
    }

    // tests
    //********************************************************************************************
    //********************************************************************************************
    //********************************************************************************************
    #[cfg(test)]
    mod tests {
        use super::*;

        // test structs
        //****************************************************************************************
        //****************************************************************************************
        //****************************************************************************************
        #[derive(Debug)]
        struct TestWidget {
            pos: Vector3<i32>,
            hierarchy: Hierarchy,
            layout: Box<dyn Layout>,
        }

        //****************************************************************************************
        impl TestWidget {
            fn new() -> Self {
                return Self {
                    pos: Vector3::<i32>::new(0, 0, 0),
                    hierarchy: Hierarchy::new(),
                    layout: Box::new(CanvasLayout::new()),
                };
            }
        }

        //****************************************************************************************
        impl Widget for TestWidget {
            fn pos(&self) -> &Vector3<i32> {
                return &self.pos;
            }
            fn hierarchy(&self) -> &Hierarchy {
                return &self.hierarchy;
            }

            fn hierarchy_mut(&mut self) -> &mut Hierarchy {
                return &mut self.hierarchy;
            }

            fn set_layout(&mut self, layout: Box<dyn Layout>) {
                self.layout = layout;
            }

            fn layout(&self) -> &dyn Layout {
                return self.layout.as_ref();
            }

            fn layout_mut(&mut self) -> &mut dyn Layout {
                return self.layout.as_mut();
            }
        }

        // test cases
        //****************************************************************************************
        //****************************************************************************************
        //****************************************************************************************
        #[test]
        fn widget_add_child() {
            let mut parent_widget = TestWidget::new();
            let child_widget = Rc::new(TestWidget::new());

            parent_widget.hierarchy_mut().add(child_widget.clone());

            assert_eq!(
                std::ptr::eq(
                    parent_widget.hierarchy().children()[0].as_ref(),
                    child_widget.as_ref()
                ),
                true
            );
        }

        //****************************************************************************************
        #[test]
        fn widget_remove_child() {
            let mut parent_widget = TestWidget::new();
            let child_widget_1 = Rc::new(TestWidget::new());
            let child_widget_2 = Rc::new(TestWidget::new());

            parent_widget.hierarchy_mut().add(child_widget_1.clone());
            parent_widget.hierarchy_mut().add(child_widget_2.clone());
            parent_widget
                .hierarchy_mut()
                .remove(child_widget_1.as_ref());

            assert_eq!(
                std::ptr::eq(
                    parent_widget.hierarchy().children()[0].as_ref(),
                    child_widget_2.as_ref()
                ),
                true
            );
        }

        //****************************************************************************************
        #[test]
        fn canvas_layout_add_child() {
            let mut parent_widget = TestWidget::new();
            parent_widget.set_layout(Box::new(CanvasLayout::new()));
            let child_widget = Rc::new(TestWidget::new());

            parent_widget.layout_mut().add(child_widget, Vector3::<i32>::new(1, 2, 0));
            
            //assert_eq!(parent_widget.layout()
        }
    }
}
