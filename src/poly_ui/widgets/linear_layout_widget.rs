// deps
use nalgebra::Point2;
use nalgebra::Vector2;
use uuid::Uuid;
// crate
use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::components::Hierarchy;
// super
use super::NewWidget;
use super::Ownerless;
use super::update_children;
use super::paint_children;
use super::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub enum LinearLayoutDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BotomToTop,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
struct Item {
    widget: Option<Uuid>,
    stretch: u32,
    min_item_size: u32,
    max_item_size: Option<u32>,
}

//************************************************************************************************
impl Item {
    pub fn min_max_diff(&self) -> Option<u32> {
        match self.max_item_size.as_ref() {
            Some(val) => return Some(val - self.min_item_size),
            None => return None,
        }
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct LinearLayoutWidget {
    id: Uuid,
    pos: Point2<i32>,
    size: Vector2<u32>,
    hierarchy: Hierarchy,

    dir: LinearLayoutDirection,
    items: Vec<Item>,
}

//************************************************************************************************
impl LinearLayoutWidget {
    pub fn new_raw() -> Self {
        return Self {
            id: Uuid::new_v4(),
            pos: Point2::<i32>::new(0, 0),
            size: Vector2::<u32>::new(0, 0),
            hierarchy: Hierarchy::new(),

            dir: LinearLayoutDirection::LeftToRight,
            items: Vec::new(),
        };
    }

    pub fn new() -> NewWidget<Self> {
        return NewWidget::new(Self::new_raw());
    }

    pub fn set_dir(&mut self, dir: LinearLayoutDirection) {
        self.dir = dir;
    }

    pub fn layout_length(&self) -> u32 {
        match &self.dir {
            LinearLayoutDirection::LeftToRight | LinearLayoutDirection::RightToLeft => {
                return self.size.x;
            }
            LinearLayoutDirection::TopToBottom | LinearLayoutDirection::BotomToTop => {
                return self.size.y;
            }
        }
    }

    fn get_lowest_min_max_diff_idx(items: &Vec<Item>) -> Option<usize> {
        if items.len() > 0 {
            let mut lowest_idx = 0;
            let mut lowest = items[0].min_max_diff();
            
            let mut idx = 1;
            for item in items.iter().skip(1) {
                let potential_lowest = item.min_max_diff();

                match potential_lowest.as_ref() {
                    Some(potential_new_val) => {
                        match lowest.as_ref() {
                            Some(old_val) => {
                                if potential_new_val < old_val {
                                    lowest = Some(*potential_new_val);
                                    lowest_idx = idx;
                                }
                            },
                            None => {
                                lowest = Some(*potential_new_val);
                                lowest_idx = idx;
                            }
                        }
                    },
                    None => {}
                }

                idx += 1;
            }

            return Some(lowest_idx);
        }
        else {
            return None;
        }
    }

    fn update_children_transform(&self) {
        Self::get_lowest_min_max_diff_idx(&self.items);
    }
}

//************************************************************************************************
impl WidgetTrait for LinearLayoutWidget {
    fn id(&self) -> &Uuid {
        return &self.id;
    }

    fn pos(&self) -> &Point2<i32> {
        return &self.pos;
    }

    fn set_pos(&mut self, value: &Point2<i32>) {
        self.pos = *value;
    }

    fn size(&self) -> &Vector2<u32> {
        return &self.size;
    }

    fn set_size(&mut self, value: &Vector2<u32>) {
        self.size = *value;
    }

    fn add(&mut self, child: Ownerless) {
        self.items.push(Item{
            widget: Some(*child.get().borrow().id()),
            stretch: 1,
            min_item_size: 0,
            max_item_size: None,
        });
        self.hierarchy.add(child);

        self.update_children_transform();
    }

    fn remove(&mut self, child: &Uuid) -> Ownerless {
        return self.hierarchy.remove(child);
    }

    fn update(&mut self, dt: f32) {
        // ensure child widgets have correct transforms
        match &self.dir {
            LinearLayoutDirection::LeftToRight => {}
            LinearLayoutDirection::RightToLeft => {}
            LinearLayoutDirection::TopToBottom => {}
            LinearLayoutDirection::BotomToTop => {}
        }

        println!("update widget");
        update_children(&self.hierarchy, dt);
    }

    fn paint(&self, painter: &mut dyn PainterTrait) {
        println!("paint widget");
        paint_children(&self.hierarchy, painter);
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[cfg(test)]
mod tests {
    // deps
    use nalgebra::Point2;
    use nalgebra::Vector2;
    // crate
    use crate::poly_ui::widgets::Widget;
    use crate::poly_ui::widgets::WidgetTrait;
    // super
    use super::LinearLayoutWidget;
    use super::LinearLayoutDirection;

    //********************************************************************************************
    #[test]
    fn check_children_transforms_left_to_right_dir() {
        let mut parent_widget = LinearLayoutWidget::new_raw();
        parent_widget.set_size(&Vector2::<u32>::new(60, 20));
        parent_widget.set_dir(LinearLayoutDirection::LeftToRight);

        let new_child_widget = Widget::new();
        let child_widget = new_child_widget.get().clone();
        parent_widget.add(new_child_widget.to_ownerless());

        assert_eq!(
            child_widget.borrow().pos(),
            &Point2::<i32>::new(0, 0)
        );
        assert_eq!(
            child_widget.borrow().size(),
            &Vector2::<u32>::new(60, 20)
        );
    }
}
