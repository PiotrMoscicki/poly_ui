// deps
use nalgebra::Point2;
use nalgebra::Vector2;
use uuid::Uuid;
// crate
use crate::poly_ui::app::PainterTrait;
use crate::poly_ui::components::Hierarchy;
use crate::poly_ui::components::Transform;
use crate::poly_ui::widgets::NewWidget;
use crate::poly_ui::widgets::Ownerless;
use crate::poly_ui::widgets::WidgetTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
/// Enum controlling layout direction. This describes in which direction layout shoudl grow.
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
    max_item_size: u32,
}

//************************************************************************************************
impl Default for Item {
    fn default() -> Self {
        Self {
            widget: None,
            stretch: 1,
            min_item_size: 0,
            max_item_size: u32::MAX,
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
        Self {
            id: Uuid::new_v4(),
            pos: Point2::<i32>::new(0, 0),
            size: Vector2::<u32>::new(0, 0),
            hierarchy: Hierarchy::default(),

            dir: LinearLayoutDirection::LeftToRight,
            items: Vec::new(),
        }
    }

    pub fn new() -> NewWidget<Self> {
        NewWidget::new(Self::new_raw())
    }

    pub fn set_dir(&mut self, dir: LinearLayoutDirection) {
        self.dir = dir;
    }

    pub fn layout_length(&self) -> u32 {
        match &self.dir {
            LinearLayoutDirection::LeftToRight | LinearLayoutDirection::RightToLeft => self.size.x,
            LinearLayoutDirection::TopToBottom | LinearLayoutDirection::BotomToTop => self.size.y,
        }
    }
}

//************************************************************************************************
impl WidgetTrait for LinearLayoutWidget {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn remove_child(&mut self, child: &Uuid) -> Ownerless {
        self.hierarchy.remove(child)
    }

    fn get_hierarchy(&self) -> &Hierarchy {
        &self.hierarchy
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
        self.hierarchy.update_children(dt);
    }

    fn paint(&mut self, painter: &mut dyn PainterTrait) {
        println!("paint widget");
        self.hierarchy.paint_children(painter);
    }
}

fn get_total_stretch(items: &[(Item, u32)]) -> u32 {
    let mut result: u32 = 0;
    for item in items {
        result += item.0.stretch;
    }
    result
}

fn get_item_max_size(goal_total: u32, total_stretch: u32, item: &Item) -> u32 {
    let size_per_stretch_unit = goal_total / total_stretch;
    std::cmp::min(item.max_item_size, size_per_stretch_unit * item.stretch)
}

fn get_item_min_max_diff(goal_total: u32, total_stretch: u32, item: &Item) -> u32 {
    get_item_max_size(goal_total, total_stretch, item) - item.min_item_size
}

#[derive(Debug)]
struct LowestMinMaxDiff {
    index: usize,
    diff: u32,
}

impl LowestMinMaxDiff {
    fn get(goal_total: u32, items: &[(Item, u32)]) -> Self {
        let total_stretch = get_total_stretch(items);

        let mut lowest_idx = 0;
        let mut lowest_stretch = items[0].0.stretch;
        let mut lowest_diff = get_item_min_max_diff(goal_total, total_stretch, &items[0].0);

        let mut potential_lowest_idx = 1;
        for item in items.iter().skip(1) {
            let potential_lowest_diff = get_item_min_max_diff(goal_total, total_stretch, &item.0);
            let potential_lowest_stretch = item.0.stretch;

            if potential_lowest_diff < lowest_diff
                || (potential_lowest_diff == lowest_diff
                    && potential_lowest_stretch < lowest_stretch)
            {
                lowest_diff = potential_lowest_diff;
                lowest_stretch = potential_lowest_stretch;
                lowest_idx = potential_lowest_idx;
            }

            potential_lowest_idx += 1;
        }

        Self {
            index: lowest_idx,
            diff: lowest_diff,
        }
    }
}

fn get_remainder(mut goal_total: u32, items: &[(Item, u32)]) -> Option<u32> {
    for item in items {
        let item_required_size = std::cmp::max(item.1, item.0.min_item_size);

        if goal_total > item_required_size {
            goal_total -= item_required_size;
        } else {
            return None;
        }
    }

    Some(goal_total)
}

fn increase_every_item_size(diff: u32, items: &mut Vec<(Item, u32)>) {
    for item in items {
        item.1 += diff * item.0.stretch;
    }
}

fn get_items_sizes_impl(goal_total: u32, mut items: Vec<(Item, u32)>) -> Vec<u32> {
    // check if we have any items in the collection
    // get lowest min max diff
    // get remaining space
    // check if adding lowest min max diff to every item will not exceed total size
    // if not exceeds
    // add lowest min max diff to every item
    // remove item with lowest min max diff from the collection
    // get sizes of the rest of the items
    // prepend size of the removeditem (with lowest min max diff) to the returned collection
    // return the collection
    // if exceeds
    // set minimal size to each item where current size is lower than minimal size
    // remaining space should be the same
    //

    println!();
    println!("-------------------------------------------------");
    for (idx, item) in items.iter().enumerate() {
        println!("size[{}]: {}", idx, item.1);
    }

    // Get space that can be spent on resizing items. If item current width is lower than minimal
    // size the mimimal size will be used for calculating the remainder.
    let remainder = get_remainder(goal_total, &items);
    // Get lowest min maxdiff from all items.
    let lowest_min_max = LowestMinMaxDiff::get(goal_total, &items);
    // Get sum of all items stretches.
    let total_stretch = get_total_stretch(&items);

    println!("remainder: {}", remainder.unwrap_or(0));
    println!("lowest_min_max.diff: {}", lowest_min_max.diff);
    println!("lowest_min_max.index: {}", lowest_min_max.index);

    println!("total_stretch: {}", total_stretch);

    // Calculate how much remaining space do we need if we're going to increase all items
    // sizes by all items by the lowest min max diff.
    let required = lowest_min_max.diff as u64 * total_stretch as u64;
    println!("required: {}", required);
    // If this required space is lower than what we have ve proceed with resizing the
    // items.
    if required <= *remainder.as_ref().unwrap_or(&0) as u64 {
        println!("required space is lower or equal than remainder");
        increase_every_item_size(lowest_min_max.diff, &mut items);
        // Get total size of an item which min max diff was used to increase the sizes of
        // all the items.
        let lowest_min_max_diff_item_size = items[lowest_min_max.index].1;
        // Remove item which is already at its max size.
        items.remove(lowest_min_max.index);
        // Get sies for the rest of the items.
        let mut result = get_items_sizes_impl(goal_total - lowest_min_max_diff_item_size, items);
        // Insert removed item size at the front.
        result.insert(lowest_min_max.index, lowest_min_max_diff_item_size);
        result
    }
    // If we don't have enough space to increase all items sizes by the lowest min max
    // diff.
    else {
        if remainder.is_some() {
            // Find out how much size can we add to each item without extending the
            // available space.
            let diff = remainder.as_ref().unwrap() / total_stretch;
            // Find out how much remaining size do we have after applying diff calculated
            // line above.
            let mut remainder_after_adding_diff =
                (remainder.as_ref().unwrap() % total_stretch) as usize;
            println!("diff: {}", diff);
            println!(
                "remainder_after_adding_diff: {}",
                remainder_after_adding_diff
            );
            increase_every_item_size(diff, &mut items);

            // Ensure items have at least mimimal size
            for item in &mut items {
                item.1 = std::cmp::max(item.0.min_item_size, item.1);
            }

            // Add remaining free space to first items in collection
            let mut idx = 0;
            while remainder_after_adding_diff > 0 {
                items[idx].1 += 1;

                idx += 1;
                remainder_after_adding_diff -= 1;

                if idx == items.len() {
                    idx = 0;
                }
            }
        }

        let mut result: Vec<u32> = Vec::new();
        for item in items {
            result.push(item.1);
        }
        result
    }
}

fn get_items_sizes(goal_total: u32, items: &[Item]) -> Vec<u32> {
    let mut items_with_width: Vec<(Item, u32)> = Vec::new();

    for item in items {
        items_with_width.push((Item { ..*item }, 0));
    }

    get_items_sizes_impl(goal_total, items_with_width)
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[cfg(test)]
mod tests {
    // deps
    // use nalgebra::Point2;
    // use nalgebra::Vector2;
    // // crate
    // use crate::poly_ui::widgets::Widget;
    // use crate::poly_ui::widgets::WidgetTrait;
    // // super
    // use super::Item;
    // use super::LowestMinMaxDiff;
    // use super::LinearLayoutWidget;
    // use super::LinearLayoutDirection;

    //********************************************************************************************
    //#[test]
    // fn get_total_stretch() {
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{ stretch: 1, ..Default::default()}, 0),
    //             (Item{ stretch: 2, ..Default::default()}, 0),
    //             (Item{ stretch: 3, ..Default::default()}, 0),
    //         );

    //         assert_eq!(super::get_total_stretch(&items), 6);
    //     }
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{ stretch: 5, ..Default::default()}, 0),
    //             (Item{ stretch: 5, ..Default::default()}, 0),
    //             (Item{ stretch: 5, ..Default::default()}, 0),
    //         );

    //         assert_eq!(super::get_total_stretch(&items), 15);
    //     }
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{ stretch: 0, ..Default::default()}, 0),
    //             (Item{ stretch: 0, ..Default::default()}, 0),
    //             (Item{ stretch: 0, ..Default::default()}, 0),
    //         );

    //         assert_eq!(super::get_total_stretch(&items), 0);
    //     }
    // }

    // //********************************************************************************************
    // #[test]
    // fn get_item_max_size() {
    //     assert_eq!(super::get_item_max_size(100, 5, &Item{..Default::default()}), 20);
    //     assert_eq!(super::get_item_max_size(100, 3, &Item{..Default::default()}), 33);
    //     assert_eq!(
    //         super::get_item_max_size(100, 3, &Item{max_item_size: 40, ..Default::default()}),
    //         33
    //     );
    //     assert_eq!(
    //         super::get_item_max_size(100, 3, &Item{max_item_size: 10, ..Default::default()}),
    //         10
    //     );
    //     assert_eq!(
    //         super::get_item_max_size(100, 20, &Item{max_item_size: 10, ..Default::default()}),
    //         5
    //     );
    // }

    // //********************************************************************************************
    // #[test]
    // fn get_item_min_max_diff() {
    //     assert_eq!(super::get_item_min_max_diff(100, 5, &Item{..Default::default()}), 20);
    //     assert_eq!(super::get_item_min_max_diff(100, 3, &Item{..Default::default()}), 33);
    //     assert_eq!(
    //         super::get_item_min_max_diff(100, 3, &Item{max_item_size: 40, ..Default::default()}),
    //         33
    //     );
    //     assert_eq!(
    //         super::get_item_min_max_diff(100, 3, &Item{max_item_size: 10, ..Default::default()}),
    //         10
    //     );
    //     assert_eq!(
    //         super::get_item_min_max_diff(100, 20, &Item{max_item_size: 10, ..Default::default()}),
    //         5
    //     );
    //     assert_eq!(
    //         super::get_item_min_max_diff(100, 5, &Item{min_item_size: 10, ..Default::default()}),
    //         10
    //     );
    //     assert_eq!(
    //         super::get_item_min_max_diff(
    //             100,
    //             5,
    //             &Item{min_item_size: 10, max_item_size: 15, ..Default::default()}),
    //         5
    //     );
    // }

    // //********************************************************************************************
    // #[test]
    // fn get_lowest_min_max_diff() {
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{..Default::default()}, 0),
    //             (Item{..Default::default()}, 0),
    //             (Item{..Default::default()}, 0),
    //         );

    //         let diff = LowestMinMaxDiff::get(100, &items);
    //         assert_eq!(diff.diff, 33);
    //         assert_eq!(diff.index, 0);
    //     }
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{ stretch: 2, ..Default::default()}, 0),
    //             (Item{..Default::default()}, 0),
    //             (Item{..Default::default()}, 0),
    //         );

    //         let diff = LowestMinMaxDiff::get(100, &items);
    //         assert_eq!(diff.diff, 25);
    //         assert_eq!(diff.index, 1);
    //     }
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{ stretch: 2, ..Default::default()}, 0),
    //             (Item{ stretch: 2, ..Default::default()}, 0),
    //             (Item{..Default::default()}, 0),
    //         );

    //         let diff = LowestMinMaxDiff::get(100, &items);
    //         assert_eq!(diff.diff, 20);
    //         assert_eq!(diff.index, 2);
    //     }
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{ stretch: 2, ..Default::default()}, 0),
    //             (Item{ stretch: 2, ..Default::default()}, 0),
    //             (Item{ stretch: 2, ..Default::default()}, 0),
    //         );

    //         let diff = LowestMinMaxDiff::get(100, &items);
    //         assert_eq!(diff.diff, 32);
    //         assert_eq!(diff.index, 0);
    //     }
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{..Default::default()}, 0),
    //             (Item{ min_item_size: 10, ..Default::default()}, 0),
    //             (Item{..Default::default()}, 0),
    //         );

    //         let diff = LowestMinMaxDiff::get(100, &items);
    //         assert_eq!(diff.diff, 23);
    //         assert_eq!(diff.index, 1);
    //     }
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{ max_item_size: 11, ..Default::default()}, 0),
    //             (Item{ min_item_size: 10, ..Default::default()}, 0),
    //             (Item{..Default::default()}, 0),
    //         );

    //         let diff = LowestMinMaxDiff::get(100, &items);
    //         assert_eq!(diff.diff, 11);
    //         assert_eq!(diff.index, 0);
    //     }
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{ max_item_size: 11, ..Default::default()}, 0),
    //             (Item{ min_item_size: 10, ..Default::default()}, 0),
    //             (Item{ max_item_size: 11, stretch: 2, ..Default::default()}, 0),
    //         );

    //         let diff = LowestMinMaxDiff::get(100, &items);
    //         assert_eq!(diff.diff, 11);
    //         assert_eq!(diff.index, 0);
    //     }
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{ max_item_size: 11, stretch: 2, ..Default::default()}, 0),
    //             (Item{ min_item_size: 10, ..Default::default()}, 0),
    //             (Item{ max_item_size: 11, ..Default::default()}, 0),
    //         );

    //         let diff = LowestMinMaxDiff::get(100, &items);
    //         assert_eq!(diff.diff, 11);
    //         assert_eq!(diff.index, 2);
    //     }
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{min_item_size: 30, stretch: 2, ..Default::default()}, 0),
    //             (Item{stretch: 5, ..Default::default()}, 0),
    //             (Item{..Default::default()}, 0),
    //         );

    //         let diff = LowestMinMaxDiff::get(90, &items);
    //         assert_eq!(diff.diff, 11);
    //         assert_eq!(diff.index, 2);
    //     }
    // }

    // //********************************************************************************************
    // #[test]
    // fn get_remainder() {
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{..Default::default()}, 25),
    //             (Item{..Default::default()}, 15),
    //             (Item{..Default::default()}, 2),
    //         );

    //         assert_eq!(super::get_remainder(50, &items).unwrap(), 8);
    //         assert_eq!(super::get_remainder(40, &items), None);
    //         assert_eq!(super::get_remainder(42, &items), None);
    //         assert_eq!(super::get_remainder(0, &items), None);
    //     }
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{min_item_size: 3, ..Default::default()}, 12),
    //             (Item{..Default::default()}, 15),
    //             (Item{min_item_size: 12, ..Default::default()}, 3),
    //         );

    //         assert_eq!(super::get_remainder(45, &items).unwrap(), 6);
    //         assert_eq!(super::get_remainder(39, &items), None);
    //         assert_eq!(super::get_remainder(30, &items), None);
    //         assert_eq!(super::get_remainder(0, &items), None);
    //     }
    // }

    // //********************************************************************************************
    // #[test]
    // fn increase_every_item_size() {
    //     {
    //         let mut items: Vec<(Item, u32)> = vec!(
    //             (Item{ stretch: 1, ..Default::default()}, 25),
    //             (Item{ stretch: 2, ..Default::default()}, 15),
    //             (Item{ stretch: 3, ..Default::default()}, 2),
    //         );

    //         super::increase_every_item_size(5, &mut items);

    //         assert_eq!(items[0].1, 30);
    //         assert_eq!(items[1].1, 25);
    //         assert_eq!(items[2].1, 17);
    //     }
    //     {
    //         let mut items: Vec<(Item, u32)> = vec!(
    //             (Item{ stretch: 0, ..Default::default()}, 25),
    //             (Item{ stretch: 0, ..Default::default()}, 15),
    //             (Item{ stretch: 0, ..Default::default()}, 2),
    //         );

    //         super::increase_every_item_size(5, &mut items);

    //         assert_eq!(items[0].1, 25);
    //         assert_eq!(items[1].1, 15);
    //         assert_eq!(items[2].1, 2);
    //     }
    // }

    // //********************************************************************************************
    // #[test]
    // fn get_items_sizes_impl() {
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{..Default::default()}, 0),
    //             (Item{..Default::default()}, 0),
    //             (Item{..Default::default()}, 0),
    //         );

    //         let sizes = super::get_items_sizes_impl(6, items);

    //         assert_eq!(sizes[0], 2);
    //         assert_eq!(sizes[1], 2);
    //         assert_eq!(sizes[2], 2);
    //     }
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{stretch: 2, ..Default::default()}, 0),
    //             (Item{stretch: 0, ..Default::default()}, 0),
    //             (Item{stretch: 0, ..Default::default()}, 0),
    //         );

    //         let sizes = super::get_items_sizes_impl(6, items);

    //         let mut idx = 0;
    //         for size in &sizes {
    //             println!("size[{}]: {}", idx, size);
    //             idx += 1;
    //         }

    //         assert_eq!(sizes[0], 6);
    //         assert_eq!(sizes[1], 0);
    //         assert_eq!(sizes[2], 0);
    //     }
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{stretch: 2, ..Default::default()}, 0),
    //             (Item{..Default::default()}, 0),
    //             (Item{..Default::default()}, 0),
    //         );

    //         let sizes = super::get_items_sizes_impl(8, items);

    //         assert_eq!(sizes[0], 4);
    //         assert_eq!(sizes[1], 2);
    //         assert_eq!(sizes[2], 2);
    //     }
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{stretch: 2, ..Default::default()}, 0),
    //             (Item{stretch: 5, ..Default::default()}, 0),
    //             (Item{..Default::default()}, 0),
    //         );

    //         let sizes = super::get_items_sizes_impl(8, items);

    //         assert_eq!(sizes[0], 2);
    //         assert_eq!(sizes[1], 5);
    //         assert_eq!(sizes[2], 1);
    //     }
    //     {
    //         let items: Vec<(Item, u32)> = vec!(
    //             (Item{min_item_size: 30, stretch: 2, ..Default::default()}, 0),
    //             (Item{stretch: 5, ..Default::default()}, 0),
    //             (Item{..Default::default()}, 0),
    //         );

    //         let sizes = super::get_items_sizes_impl(90, items);

    //         let mut idx = 0;
    //         for size in &sizes {
    //             println!("size[{}]: {}", idx, size);
    //             idx += 1;
    //         }

    //         assert_eq!(sizes[0], 30);
    //         assert_eq!(sizes[1], 50);
    //         assert_eq!(sizes[2], 10);
    //     }
    // }

    // //********************************************************************************************
    // #[test]
    // fn check_children_transforms_left_to_right_dir() {
    //     let mut parent_widget = LinearLayoutWidget::new_raw();
    //     parent_widget.set_size(&Vector2::<u32>::new(60, 20));
    //     parent_widget.set_dir(LinearLayoutDirection::LeftToRight);

    //     let new_child_widget = Widget::new();
    //     let child_widget = new_child_widget.get().clone();
    //     parent_widget.add(new_child_widget.to_ownerless());

    //     assert_eq!(
    //         child_widget.borrow().pos(),
    //         &Point2::<i32>::new(0, 0)
    //     );
    //     assert_eq!(
    //         child_widget.borrow().size(),
    //         &Vector2::<u32>::new(60, 20)
    //     );
    // }
}
