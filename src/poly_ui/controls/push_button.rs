// std
use std::fmt::Debug;
// deps
use nalgebra::Point2;
use uuid::Uuid;
// crate
use crate::poly_ui::app::Color;
use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::app::Rect;
use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::components::Transform;
use crate::poly_ui::widgets::NewWidget;
use crate::poly_ui::widgets::OwnedWidget;
use crate::poly_ui::widgets::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct PushButton {
    id: Uuid,
    hierarchy: Hierarchy,

    border_color: Color,
}

//************************************************************************************************
impl PushButton {
    pub fn new_raw() -> Self {
        Self {
            id: Uuid::new_v4(),
            hierarchy: Hierarchy::default(),
            border_color: Color {
                r: 255,
                g: 0,
                b: 255,
                a: 128,
            },
        }
    }

    pub fn new() -> NewWidget<Self> {
        NewWidget::new(Self::new_raw())
    }
}

//************************************************************************************************
impl WidgetTrait for PushButton {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn remove_child(&mut self, child: &Uuid) -> OwnedWidget {
        self.hierarchy.remove(child)
    }

    fn get_hierarchy(&self) -> &Hierarchy {
        &self.hierarchy
    }

    fn get_child_transform(&self, child: &Uuid) -> &Transform {
        self.hierarchy.get_transform(child)
    }

    fn update(&mut self, dt: f32) {
        self.hierarchy.update_children(dt);
    }

    fn paint(&mut self, painter: &mut dyn PainterTrait) {
        painter.set_draw_color(&self.border_color);
        let painter_size = painter.size();
        painter.draw_rect(Rect {
            pos: Point2::<i32>::new(0, 0),
            size: painter_size,
        });

        self.hierarchy.paint_children(painter);
    }
}
