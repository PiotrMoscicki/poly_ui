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
use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::app::Color;
use crate::poly_ui::app::Rect;
use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::layouts::{CanvasLayout, Layout};

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Widget {
    id: Uuid,
    pos: Point2<i32>,
    size: Vector2<u32>,
    hierarchy: Rc<RefCell<Hierarchy>>,
    layout: Box<dyn Layout>,
}

//************************************************************************************************
impl Widget {
    pub fn new() -> Self {
        return Self {
            id: Uuid::new_v4(),
            pos: Point2::<i32>::new(0, 0),
            size: Vector2::<u32>::new(0, 0),
            hierarchy: Rc::new(RefCell::new(Hierarchy::new())),
            layout: Box::new(CanvasLayout::new()),
        };
    }
}

//************************************************************************************************
impl WidgetTrait for Widget {
    fn id(&self) -> &Uuid {
        return &self.id;
    }

    fn pos(&self) -> Point2<i32> {
        return self.pos;
    }

    fn set_pos(&mut self, new: Point2<i32>) {
        self.pos = new;
        return ();
    }

    fn size(&self) -> Vector2<u32> {
        return self.size;
    }

    fn set_size(&mut self, new: Vector2<u32>) {
        self.size = new;
        return ();
    }

    fn hierarchy(&self) -> Ref<Hierarchy> {
        return self.hierarchy.borrow();
    }

    fn hierarchy_mut(&mut self) -> RefMut<Hierarchy> {
        return self.hierarchy.borrow_mut();
    }

    fn set_layout(&mut self, layout: Box<dyn Layout>) {
        self.layout = layout;
        self.layout
            .set_owner_widget_hierarchy(self.hierarchy.clone());
    }

    fn layout(&self) -> &dyn Layout {
        return self.layout.as_ref();
    }

    fn layout_mut(&mut self) -> &mut dyn Layout {
        return self.layout.as_mut();
    }

    fn update(&mut self, dt: f32) {
        update_children(&self.hierarchy(), dt);
    }

    fn paint(&self, painter: &mut dyn PainterTrait) {
        painter.set_draw_color(&Color{r: 255, g: 0, b: 255, a: 128});
        painter.draw_rect(Rect{pos: Point2::<i32>::new(50, 50), size: Vector2::<u32>::new(50, 50) });

        println!("paint");

        paint_children(&self.hierarchy(), painter);
    }
}
