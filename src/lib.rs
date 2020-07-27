pub mod poly_ui {
    use nalgebra::Vector3;
    use uuid::Uuid;
    use std::{collections::HashMap, fmt::Debug, rc::{Rc, Weak}, cell::RefCell};

    // traits
    //********************************************************************************************
    //********************************************************************************************
    //********************************************************************************************
    pub trait Layout: Debug {
        fn add(&mut self, child: Rc<RefCell<dyn Widget>>, pos: Vector3<i32>);
    }

    //********************************************************************************************
    pub trait Widget: Debug {
        fn id(&self) -> &Uuid;

        fn pos(&self) -> &Vector3<i32>;

        fn hierarchy(&self) -> &Hierarchy;
        fn hierarchy_mut(&mut self) -> &mut Hierarchy;

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
        hierarchy: Hierarchy,
        layout: Box<dyn Layout>,
    }
    
    impl BaseWidget {
        pub fn new() -> Self {
            return Self {
                id: Uuid::new_v4(),
                pos: Vector3::<i32>::new(0, 0, 0),
                hierarchy: Hierarchy::new(),
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
        fn hierarchy(&self) -> &Hierarchy {
            return &self.hierarchy;
        }
        fn hierarchy_mut(&mut self) -> &mut Hierarchy {
            return &mut self.hierarchy;
        }
        fn set_layout(&mut self, layout: Box<dyn Layout>) {
            layout.set_owner(owner: Owner)
            self.layout = layout;
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
    pub struct Owner {
        data: Weak<RefCell<dyn Widget>>,
    }

    //********************************************************************************************
    #[derive(Debug)]
    pub struct CanvasLayout {
        owner: Owner,
        children: HashMap<Uuid, Vector3<i32>>,
    }

    impl CanvasLayout {
        pub fn new() -> Self {
            return Self {
                owner: Owner{ data: Weak::<RefCell<BaseWidget>>::new() },
                children: HashMap::new(),
            };
        }
    }

    impl Layout for CanvasLayout {
        fn set_owner(&mut self, owner: Owner) {
            self.owner = owner;
        }

        fn owner(&self) -> &Owner {
            return &self.owner;
        }

        fn add(&mut self, child: Rc<RefCell<dyn Widget>>, pos: Vector3<i32>) {
            self.children.insert(*child.borrow().id(), pos);
            self.owner.data.upgrade().unwrap().borrow_mut().hierarchy_mut().add(child);
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
