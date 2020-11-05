use nalgebra::Point2;
use nalgebra::Vector2;
use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Debug,
    rc::Rc,
};
use uuid::Uuid;

use super::paint_children;
use super::update_children;
use super::WidgetTrait;
use crate::poly_ui::app::Color;
use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::app::Rect;
use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::layouts::{CanvasLayout, LayoutTrait};

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Widget {
    id: Uuid,
    pos: Point2<i32>,
    size: Vector2<u32>,
    hierarchy: Rc<RefCell<Hierarchy>>,
    layout: Rc<RefCell<dyn LayoutTrait>>,
}

//************************************************************************************************
impl Widget {
    pub fn new() -> Self {
        return Self {
            id: Uuid::new_v4(),
            pos: Point2::<i32>::new(0, 0),
            size: Vector2::<u32>::new(0, 0),
            hierarchy: Rc::new(RefCell::new(Hierarchy::new())),
            layout: Rc::new(RefCell::new(CanvasLayout::new())),
        };
    }
}

//************************************************************************************************
impl WidgetTrait for Widget {
    fn id(&self) -> &Uuid {
        return &self.id;
    }

    fn hierarchy(&self) -> Ref<Hierarchy> {
        return self.hierarchy.borrow();
    }

    fn hierarchy_mut(&mut self) -> RefMut<Hierarchy> {
        return self.hierarchy.borrow_mut();
    }

    fn set_layout(&mut self, layout: Rc<RefCell<dyn LayoutTrait>>) {
        self.layout = layout;
        self.layout
            .borrow_mut()
            .set_owner_widget_hierarchy(self.hierarchy.clone());
    }

    fn layout(&self) -> Ref<dyn LayoutTrait> {
        return self.layout.borrow();
    }

    fn layout_mut(&mut self) -> RefMut<dyn LayoutTrait> {
        return self.layout.borrow_mut();
    }

    fn update(&mut self, dt: f32) {
        update_children(&self.hierarchy(), dt);
        println!("update widget");
    }

    fn paint(&self, painter: &mut dyn PainterTrait) {
        painter.set_draw_color(&Color {
            r: 255,
            g: 0,
            b: 255,
            a: 128,
        });
        painter.draw_rect(Rect {
            pos: Point2::<i32>::new(50, 50),
            size: Vector2::<u32>::new(50, 50),
        });

        println!("paint widget");

        paint_children(&self.hierarchy(), &*self.layout(), painter);
    }
}
