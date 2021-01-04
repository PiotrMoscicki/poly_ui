// deps
use nalgebra::Point2;
use nalgebra::Vector2;
use uuid::Uuid;
// crate
use crate::poly_ui::app::Color;
use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::app::Rect;
use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::components::Transform;
// super
use super::NewWidget;
use super::Ownerless;
use super::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Widget {
    id: Uuid,
    pos: Point2<i32>,
    size: Vector2<u32>,
    hierarchy: Hierarchy,
}

//************************************************************************************************
impl Widget {
    pub fn new_raw() -> Self {
        Self {
            id: Uuid::new_v4(),
            pos: Point2::<i32>::new(0, 0),
            size: Vector2::<u32>::new(0, 0),
            hierarchy: Hierarchy::new(),
        }
    }

    pub fn new() -> NewWidget<Self> {
        NewWidget::new(Self::new_raw())
    }
}

//************************************************************************************************
impl WidgetTrait for Widget {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn pos(&self) -> &Point2<i32> {
        &self.pos
    }

    fn set_pos(&mut self, value: &Point2<i32>) {
        self.pos = *value;
    }

    fn size(&self) -> &Vector2<u32> {
        &self.size
    }

    fn set_size(&mut self, value: &Vector2<u32>) {
        self.size = *value;
    }

    fn add(&mut self, child: Ownerless) {
        self.hierarchy.add(child);
    }

    fn remove(&mut self, child: &Uuid) -> Ownerless {
        self.hierarchy.remove(child)
    }

    fn update(&mut self, dt: f32) {
        println!("update widget");
        update_children(&self.hierarchy, dt);
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
        paint_children(&self.hierarchy, painter);
    }
}

//************************************************************************************************
pub fn update_children(hierarchy: &Hierarchy, dt: f32) {
    for child in hierarchy.children() {
        child.get().borrow_mut().update(dt);
    }
}

//************************************************************************************************
pub fn paint_children(hierarchy: &Hierarchy, parent_canvas: &mut dyn PainterTrait) {
    for child in hierarchy.children() {
        let borrowed_child = child.get().borrow();
        let mut sub_canvas =
            parent_canvas.sub_painter(&Transform::new(borrowed_child.pos(), borrowed_child.size()));
        borrowed_child.paint(&mut *sub_canvas);
    }
}
