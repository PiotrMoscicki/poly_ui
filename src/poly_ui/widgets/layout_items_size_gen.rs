//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct ValidatedItem {
    pub stretch: u32, // any value
    pub min_size: u32, // any value
    pub max_size: u32, // ensured to be higher or equal min_size
    pub current_size: u32, // ensured to have at least one element
}

//************************************************************************************************
impl ValidatedItem {
    fn get_size_from_stretch(&mut self, layout_size: u32, layout_stretch: u32, items_count: u32) -> u32 {
        if layout_stretch > 0 {
           return layout_size / layout_stretch * self.stretch;
        }
        else {
            return layout_size / items_count;
        }
    }

    fn update_max_size(&mut self, layout_size: u32, layout_stretch: u32, items_count: u32) {
        self.max_size = std::cmp::min(
            self.max_size, 
            self.get_size_from_stretch(layout_size, layout_stretch, items_count)
        );
        self.max_size = std::cmp::max(self.max_size, self.min_size);
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct InputItem {
    pub stretch: u32, // any value
    pub min_size: u32, // any value
    pub max_size: u32, // if max is lower than min, min is used
}

//************************************************************************************************
impl InputItem {
    fn validate(mut self, layout_size: u32, layout_stretch: u32, items_count: u32) -> ValidatedItem {
        println!("layout_size: {}", layout_size);
        println!("layout_stretch: {}", layout_stretch);
        self.update_max_size(layout_size, layout_stretch, items_count);

        return ValidatedItem{
            stretch: self.stretch,
            min_size: self.min_size,
            max_size: self.max_size,
            current_size: 0,
        }
    }

    fn get_size_from_stretch(&mut self, layout_size: u32, layout_stretch: u32, items_count: u32) -> u32 {
        if layout_stretch > 0 {
           return layout_size / layout_stretch * self.stretch;
        }
        else {
            return layout_size / items_count;
        }
    }

    fn update_max_size(&mut self, layout_size: u32, layout_stretch: u32, items_count: u32) {
        self.max_size = std::cmp::min(
            self.max_size, 
            self.get_size_from_stretch(layout_size, layout_stretch, items_count)
        );
        self.max_size = std::cmp::max(self.max_size, self.min_size);
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct ValidatedLayout {
    pub size: u32, // size is large enough to fit all items in their lowest sizes
    pub stretch: u32, // sum of all items' stretch
    pub items: Vec<ValidatedItem>,
}

//************************************************************************************************
impl ValidatedLayout {
    pub fn calculate(&mut self) {
        println!("-------------------------------------------------");
        println!("calculate()");

        let mut idx = 0;
        for item in &self.items {
            println!("size[{}]: {}", idx, item.current_size);
            idx += 1;
        }

        // update items max sizes 
        let layout_stretch = self.get_stretch();
        let items_count = self.items.len() as u32;
        for item in &mut self.items {
            item.update_max_size(self.size, layout_stretch, items_count);
        }

        if let Some(index) = self.get_index_of_item_where_min_equals_max() {
            println!("get_index_of_item_where_min_equals_max() -> Some({})", index);
            // remove item from layout
            let mut item = self.items.remove(index);
            // set item final size
            item.current_size = item.min_size;
            // fix layout properties after removing item
            self.size -= item.current_size;
            self.stretch -= item.stretch;

            // calculate rest of items sizes
            self.calculate();

            // return item to layout
            self.size += item.current_size;
            self.stretch += item.stretch;
            self.items.insert(index, item);
        }
        else if let Some(index) = self.get_index_of_item_with_lowest_max_which_is_lower_than_stretch() {
            println!("get_index_of_item_with_lowest_max_which_is_lower_than_stretch() -> Some({})", index);
            // remove item from layout
            let mut item = self.items.remove(index);
            // set item final size
            item.current_size = item.max_size;
            // fix layout properties after removing item
            self.size -= item.current_size;
            self.stretch -= item.stretch;

            // calculate rest of items sizes
            self.calculate();

            // return item to layout
            self.size += item.current_size;
            self.stretch += item.stretch;
            self.items.insert(index, item);
        }
        else {
            println!("get_index_of_item_where_min_equals_max() -> None");
            // no item will have its final size lower or higher than what it gets from the stretch
            // so all available space can be spent on increasing  remaining items size
            for item in &mut self.items {
                item.current_size = item.max_size;
            }
        }
    }

    fn get_stretch(&self) -> u32 {
        let mut result = 0;
        for item in &self.items {
            result += item.stretch;
        }

        return result;
    }

    fn get_index_of_item_where_min_equals_max(&self) -> Option<usize> {
        let mut i = 0;
        for item in &self.items {
            if item.min_size == item.max_size {
                return Some(i);
            }
            i += 1;
        }

        return None;
    }

    fn get_index_of_item_with_lowest_max_which_is_lower_than_stretch(&self) -> Option<usize> {
        let mut i = 0;
        for item in &self.items {
            if item.min_size == item.max_size {
                return Some(i);
            }
            i += 1;
        }

        return None;
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Layout {
    pub size: u32, // size is large enough to fit all items in their lowest sizes
    pub stretch: u32, // sum of all items' stretch
    pub items: Vec<ValidatedItem>,
}

//************************************************************************************************
impl Layout {
    pub fn validate(&mut self) {
        // if there are some items in the layout
            // ensure layout has at least minimal width
                // layout size hast to be equal or higher to sum of all items min sizes
            // gather items stretch
                // save sum of all items stretch
            // at this point layout properties are valid
            // validate all items
                // if max item size is lower than min item size 
                    // set max item size to min item size
            // all items are valid

            // get lowest current -> max diff of all items
            // get remaining layout space
            // if there is some remaining space
                // if remaining space is big enough to increase each item by lowest current -> max diff
                    // increase each item size by lowest current -> max diff
                    // remove item with lowest diff from the layout
                    // validate
                    // return removed item to the collection at its old position
                    // validate layout width and stretch and all the items in the collection
                // else
                    // 
            // else
                // return
        // else
            // return

        if self.items.len() > 0 {
            return Some(ValidatedLayout {
                size: std::cmp::max(self.get_min_size(), self.size),
                stretch: self.get_stretch(),
                items: self.get_validated_items(),
            });
        }
        else {
            return None;
        }
    }

    fn get_min_size(&self) -> u32 {
        let mut result = 0;
        for item in &self.items {
            result += item.min_size;
        }
        return result;
    }

    fn get_stretch(&self) -> u32 {
        let mut result = 0;
        for item in &self.items {
            result += item.stretch;
        }

        return result;
    }

    fn get_validated_items(self) -> Vec<ValidatedItem> {
        let mut result = Vec::new();
        let layout_stretch = self.get_stretch();
        let items_count = self.items.len() as u32;
        for item in self.items {
            result.push(item.validate(self.size, layout_stretch, items_count));
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    // super
    use super::InputItem;
    use super::ValidatedItem;
    use super::Layout;

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