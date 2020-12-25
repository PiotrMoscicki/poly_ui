//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Item {
    pub stretch: u32, // any value
    pub min_size: u32, // any value
    pub max_size: u32, // ensured to be higher or equal min_size
    pub current_size: u32, // ensured to have at least one element
}

impl Item {
    fn get_current_max_diff(&self) -> u32 {
        return self.max_size - self.current_size;
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Layout {
    pub size: u32, // size is large enough to fit all items in their lowest sizes
    pub items: Vec<Item>,
}

//************************************************************************************************
impl Layout {
    pub fn validate(&mut self) {
        if self.items.len() > 0 {
            self.ensure_layout_has_at_least_minimal_width();
            self.validate_all_items();
            
            let items_stretch = self.gather_items_stretch();
            let item_with_lowest_current_max_diff = self.get_item_with_lowest_current_max_diff();
            let lowest_current_max_diff = 
                self.items[item_with_lowest_current_max_diff].get_current_max_diff();
            let remaining_free_layout_space = self.remaining_free_layout_space();
            
            if remaining_free_layout_space == 0 {
                return;
            }
            else if remaining_free_layout_space < lowest_current_max_diff * items_stretch {
                self.set_every_item_size_to_at_least_min();
                self.increase_every_item_size(remaining_free_layout_space / items_stretch);
                let remainder = remaining_free_layout_space % items_stretch;
                for _ in 0..remainder {
                    let highest_diff = self.get_item_with_highest_current_vs_expected_stretch_diff();
                    self.items[highest_diff].current_size += 1;
                }
            }
            else {
                self.increase_every_item_size(lowest_current_max_diff);
                
                let mut item = self.items.remove(item_with_lowest_current_max_diff);
                self.size -= item.current_size;

                self.validate();
                
                self.size += item.current_size;
                self.items.insert(item_with_lowest_current_max_diff, item);
            }
        }
        else {
            return;
        }
    }

    fn ensure_layout_has_at_least_minimal_width(&mut self) {
        let mut sum_of_min_sizes = 0;
        
        for item in self.items {
            sum_of_min_sizes += item.min_size;
        }

        if self.size < sum_of_min_sizes {
            self.size = sum_of_min_sizes;
        }
    }

    fn gather_items_stretch(&mut self) -> u32 {
        let mut result = 0;

        for item in self.items {
            result += item.stretch;
        }

        return result;
    }

    fn validate_all_items(&mut self) {
        for item in self.items {
            if item.max_size < item.min_size {
                item.max_size = item.min_size;
            }
        }
    }

    fn get_item_with_lowest_current_max_diff(&mut self) -> usize {
        let mut idx = 0;
        let mut lowest_idx = 0;
        let mut lowest_diff = self.items[0].max_size - self.items[0].min_size;
        for item in self.items {
            let potential_lowest = item.max_size - item.min_size;
            if potential_lowest < lowest_diff {
                lowest_diff = potential_lowest;
                lowest_idx = idx;
            }

            idx += 1;
        }

        return lowest_idx;
    }

    fn remaining_free_layout_space(&mut self) -> u32 {
        let mut result = self.size;

        for item in self.items {
            result -= std::cmp::max(item.current_size, item.min_size);
        }

        return result;
    }

    fn increase_every_item_size(&mut self, diff: u32) {
        for item in self.items {
            item.current_size += diff * item.stretch;
        }
    }

    fn set_every_item_size_to_at_least_min(&mut self) {
        for item in self.items {
            if item.current_size < item.min_size {
                item.current_size = item.min_size;
            }
        }
    }

    fn get_item_with_highest_current_vs_expected_stretch_diff(&self) -> usize {
        let total_stretch = self.gather_items_stretch();
        let mut highest_idx = 0;
        let mut highest_diff = 0;
        let mut idx = 0;
        
        for item in &self.items {
            let current_stretch = item.current_size / self.size;
            let expected_stretch = item.stretch / total_stretch;
            idx += 1;
        }
        // stretch_value = stretch / total_stretch
        // current_stretch_value = size / total_size
        
        // stretch / total_stretch = size / total_size
        // stretch * total_size = size * total_stretch
        
        return highest_idx;
    }
}

#[cfg(test)]
mod tests {
    // super
    use super::*;

    //********************************************************************************************
    #[test]
    fn validated_item_update_max_size() {
        let mut item = ValidatedItem{
            stretch: 0,
            min_size: 0,
            max_size: u32::MAX,
            current_size: 0,
        };
        item.update_max_size(120, 0, 4);
        assert_eq!(item.max_size, 30);
    }

    //********************************************************************************************
    #[test]
    fn input_item_update_max_size() {
        let mut item = InputItem{
            stretch: 0,
            min_size: 0,
            max_size: u32::MAX,
        };
        item.update_max_size(120, 0, 4);
        assert_eq!(item.max_size, 30);
    }

    //********************************************************************************************
    #[test]
    fn no_items() {
        let input = InputLayout{
            size: 120,
            items: vec!{},
        };
        let validated = input.validate();
        assert_eq!(validated.is_none(), true);
    }

    //********************************************************************************************
    #[test]
    fn no_min_max_stretch() {
        let input = InputLayout{
            size: 120,
            items: vec!{
                InputItem{stretch: 0, min_size: 0, max_size: u32::MAX},
                InputItem{stretch: 0, min_size: 0, max_size: u32::MAX},
                InputItem{stretch: 0, min_size: 0, max_size: u32::MAX},
                InputItem{stretch: 0, min_size: 0, max_size: u32::MAX},
            },
        };
        let mut validated = input.validate().unwrap();
        validated.calculate();

        assert_eq!(validated.size, 120);
        assert_eq!(validated.stretch, 0);
        assert_eq!(validated.items[0].current_size, 30);
        assert_eq!(validated.items[1].current_size, 30);
        assert_eq!(validated.items[2].current_size, 30);
        assert_eq!(validated.items[3].current_size, 30);
    }

    //********************************************************************************************
    #[test]
    fn no_min_max() {
        let input = InputLayout{
            size: 120,
            items: vec!{
                InputItem{stretch: 0, min_size: 0, max_size: u32::MAX},
                InputItem{stretch: 1, min_size: 0, max_size: u32::MAX},
                InputItem{stretch: 2, min_size: 0, max_size: u32::MAX},
                InputItem{stretch: 3, min_size: 0, max_size: u32::MAX},
            },
        };
        let mut validated = input.validate().unwrap();
        validated.calculate();

        assert_eq!(validated.size, 120);
        assert_eq!(validated.stretch, 6);
        assert_eq!(validated.items[0].current_size, 0);
        assert_eq!(validated.items[1].current_size, 20);
        assert_eq!(validated.items[2].current_size, 40);
        assert_eq!(validated.items[3].current_size, 60);
    }

    //********************************************************************************************
    #[test]
    fn milka() {
        let input = InputLayout{
            size: 16,
            items: vec!{
                InputItem{stretch: 2, min_size: 1, max_size: u32::MAX},
                InputItem{stretch: 1, min_size: 6, max_size: u32::MAX},
                InputItem{stretch: 1, min_size: 2, max_size: u32::MAX},
            },
        };
        let mut validated = input.validate().unwrap();
        validated.calculate();

        assert_eq!(validated.size, 16);
        assert_eq!(validated.stretch, 4);
        assert_eq!(validated.items[0].current_size, 0);
        assert_eq!(validated.items[1].current_size, 20);
        assert_eq!(validated.items[2].current_size, 40);
    }
}