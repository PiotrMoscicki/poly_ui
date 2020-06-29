pub mod poly_ui {
    use std::{fmt::Debug, rc::Rc};

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

    pub trait Widget: Debug {
        fn hierarchy(&self) -> &Hierarchy;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[derive(Debug)]
        struct TestWidget {
            hierarchy: Hierarchy,
        }

        impl TestWidget {
            fn new() -> Self {
                return Self {
                    hierarchy: Hierarchy::new(),
                };
            }
        }

        impl Widget for TestWidget {
            fn hierarchy(&self) -> &Hierarchy {
                return &self.hierarchy;
            }
        }

        #[test]
        fn add_child() {
            let mut parent_widget = TestWidget::new();
            let child_widget = Rc::new(TestWidget::new());

            parent_widget.hierarchy.add(child_widget.clone());
            assert_eq!(
                std::ptr::eq(
                    parent_widget.hierarchy().children()[0].as_ref(),
                    child_widget.as_ref()
                ),
                true
            );
        }

        #[test]
        fn remove_child() {
            let mut parent_widget = TestWidget::new();
            let child_widget_1 = Rc::new(TestWidget::new());
            let child_widget_2 = Rc::new(TestWidget::new());

            parent_widget.hierarchy.add(child_widget_1.clone());
            parent_widget.hierarchy.add(child_widget_2.clone());
            parent_widget.hierarchy.remove(child_widget_1.as_ref());
            assert_eq!(
                std::ptr::eq(
                    parent_widget.hierarchy().children()[0].as_ref(),
                    child_widget_2.as_ref()
                ),
                true
            );
        }
    }
}
