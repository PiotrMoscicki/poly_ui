pub mod poly_ui {
    use nalgebra::Vector3;
    use uuid::Uuid;
    use std::{collections::HashMap, fmt::Debug, rc::Rc, cell::{RefCell, Ref, RefMut}};

    // traits
    //********************************************************************************************
    //********************************************************************************************
    //********************************************************************************************
    pub trait Layout: Debug {
        fn set_owner_widget_hierarchy(&mut self, hierarchy: Rc<RefCell<Hierarchy>>);

        fn add(&mut self, child: Rc<RefCell<dyn Widget>>, pos: Vector3<i32>);
    }

    //********************************************************************************************
    pub trait Widget: Debug {
        fn id(&self) -> &Uuid;

        fn pos(&self) -> &Vector3<i32>;

        fn hierarchy(&self) -> Ref<Hierarchy>;
        fn hierarchy_mut(&mut self) -> RefMut<Hierarchy>;

        fn set_layout(&mut self, layout: Box<dyn Layout>);
        fn layout(&self) -> &dyn Layout;
        fn layout_mut(&mut self) -> &mut dyn Layout;
    }

    impl std::hash::Hash for dyn Widget {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.id().hash(state);
        }
    }

    impl std::cmp::PartialEq for dyn Widget {
        fn eq(&self, other: &Self) -> bool {
            return self.id() == other.id();
        }
    }
    impl std::cmp::Eq for dyn Widget {}

    // structs
    //********************************************************************************************
    //********************************************************************************************
    //********************************************************************************************
    
    //********************************************************************************************
    #[derive(Debug)]
    pub struct BaseWidget {
        id: Uuid,
        pos: Vector3<i32>,
        hierarchy: Rc<RefCell<Hierarchy>>,
        layout: Box<dyn Layout>,
    }
    
    impl BaseWidget {
        pub fn new() -> Self {
            return Self {
                id: Uuid::new_v4(),
                pos: Vector3::<i32>::new(0, 0, 0),
                hierarchy: Rc::new(RefCell::new(Hierarchy::new())),
                layout: Box::new(CanvasLayout::new()),
            };
        }
    }
    
    impl Widget for BaseWidget {
        fn id(&self) -> &Uuid {
            return &self.id;
        }
        fn pos(&self) -> &Vector3<i32> {
            return &self.pos;
        }
        fn hierarchy(&self) -> Ref<Hierarchy> {
            return self.hierarchy.borrow();
        }
        fn hierarchy_mut(&mut self) -> RefMut<Hierarchy> {
            return self.hierarchy.borrow_mut();
        }
        fn set_layout(&mut self, layout: Box<dyn Layout>) {
            self.layout = layout;
            self.layout.set_owner_widget_hierarchy(self.hierarchy.clone());
        }
        fn layout(&self) -> &dyn Layout {
            return self.layout.as_ref();
        }
        fn layout_mut(&mut self) -> &mut dyn Layout {
            return self.layout.as_mut();
        }
    }

    //********************************************************************************************
    #[derive(Debug)]
    pub struct Transform {
        pos: Vector3<i32>,
        size: Vector3<i32>,
    }

    //********************************************************************************************
    #[derive(Debug)]
    pub struct Hierarchy {
        children: Vec<Rc<RefCell<dyn Widget>>>,
    }

    impl Hierarchy {
        pub fn new() -> Self {
            return Self {
                children: Vec::new(),
            };
        }

        pub fn add(&mut self, child: Rc<RefCell<dyn Widget>>) {
            self.children.push(child);
        }

        pub fn remove(&mut self, child: &Rc<RefCell<dyn Widget>>) {
            self.children.remove(
                self.children
                    .iter()
                    .position(|elem| elem.borrow().id() == child.borrow().id())
                    .unwrap(),
            );
        }

        pub fn children(&self) -> &Vec<Rc<RefCell<dyn Widget>>> {
            return &self.children;
        }
    }

    //********************************************************************************************
    #[derive(Debug)]
    pub struct CanvasLayout {
        children: HashMap<Uuid, Vector3<i32>>,
        hierarchy: Rc<RefCell<Hierarchy>>,
    }

    impl CanvasLayout {
        pub fn new() -> Self {
            return Self {
                children: HashMap::new(),
                hierarchy: Rc::new(RefCell::new(Hierarchy::new())),
            };
        }
    }

    impl Layout for CanvasLayout {
        fn set_owner_widget_hierarchy(&mut self, hierarchy: Rc<RefCell<Hierarchy>>) {
            self.hierarchy = hierarchy;
        }

        fn add(&mut self, child: Rc<RefCell<dyn Widget>>, pos: Vector3<i32>) {
            self.children.insert(*child.borrow().id(), pos);
            self.hierarchy.borrow_mut().add(child);
        }
    }

    // tests
    //********************************************************************************************
    //********************************************************************************************
    //********************************************************************************************
    #[cfg(test)]
    mod tests {
        use super::*;

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
