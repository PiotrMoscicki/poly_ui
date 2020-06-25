pub mod poly_ui {
    use std::{
        rc::Rc,
        fmt::Debug,
    };

    #[derive(Debug)]
    pub struct Hierarchy {
        children: Vec<Rc<dyn Widget>>,
    }
    
    impl Hierarchy {
        pub fn new() -> Self {
            return Self{
                children: Vec::new(),
            };
        }

        pub fn add(&mut self, child: Rc<dyn Widget>) {
            self.children.push(child);
        }

        pub fn children(&self) -> &Vec<Rc<dyn Widget>> {
            return &self.children;
        }
    }

    pub trait Widget : Debug {
        fn hierarchy(&self) -> &Hierarchy;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[derive(Debug)]
        struct TestWidget {
            hierarchy: Hierarchy,
        }

        impl Widget for TestWidget {
            fn hierarchy(&self) -> &Hierarchy {
                return &self.hierarchy;
            }
        }

        impl TestWidget {
            fn new() -> Self {
                return Self{
                    hierarchy: Hierarchy::new(),
                };
            }
        }

        #[test]
        fn add_child() {
            let mut parent_widget = TestWidget::new();
            let child_widget = Rc::new(TestWidget::new());

            parent_widget.hierarchy.add(child_widget.clone());
            assert_eq!(std::ptr::eq(parent_widget.hierarchy().children()[0].as_ref(), child_widget.as_ref()), true);
        }
    }
}
