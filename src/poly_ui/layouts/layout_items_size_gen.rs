//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Item {
    stretch: u32,  // any value
    min_size: u32, // any value
    max_size: u32, // ensured to be higher or equal min_size
    current_size: u32,
}

impl Item {
    pub fn new(stretch: u32, min_size: Option<u32>, max_size: Option<u32>) -> Self {
        Self {
            stretch,
            min_size: min_size.unwrap_or(0),
            max_size: max_size.unwrap_or(u32::MAX),
            current_size: 0,
        }
    }

    pub fn get_stretch(&self) -> u32 {
        self.stretch
    }

    pub fn get_min_size(&self) -> u32 {
        self.min_size
    }

    pub fn get_max_size(&self) -> u32 {
        self.max_size
    }

    pub fn get_current_size(&self) -> u32 {
        self.current_size
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug)]
pub struct Layout {
    pub size: u32,        // size is large enough to fit all items in their lowest sizes
    pub items: Vec<Item>, // can be empty
}

//************************************************************************************************
impl Layout {
    pub fn new(size: u32, items: Vec<Item>) -> Self {
        let mut result = Self { size, items };

        result.ensure_layout_has_at_least_minimal_width();
        result.validate_all_items();

        result.validate();

        result
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn get_items(&self) -> &Vec<Item> {
        &self.items
    }

    fn validate(&mut self) {
        if !self.items.is_empty() {
            self.set_every_item_size_to_at_least_min();
            let remaining_free_layout_space = self.remaining_free_layout_space();
            for _ in 0..remaining_free_layout_space {
                let highest_diff = self.get_item_with_highest_expected_minus_current_stretch();
                self.items[highest_diff].current_size += 1;
            }
        }
    }

    fn ensure_layout_has_at_least_minimal_width(&mut self) {
        let mut sum_of_min_sizes = 0;

        for item in &self.items {
            sum_of_min_sizes += item.min_size;
        }

        if self.size < sum_of_min_sizes {
            self.size = sum_of_min_sizes;
        }
    }

    fn gather_items_stretch(&self) -> u32 {
        let mut result = 0;

        for item in &self.items {
            result += item.stretch;
        }

        result
    }

    fn validate_all_items(&mut self) {
        for item in &mut self.items {
            if item.max_size < item.min_size {
                item.max_size = item.min_size;
            }
            item.current_size = 0;
        }

        if self.gather_items_stretch() == 0 {
            for item in &mut self.items {
                item.stretch = 1;
            }
        }
    }

    fn remaining_free_layout_space(&self) -> u32 {
        let mut result = self.size;

        for item in &self.items {
            result -= std::cmp::max(item.current_size, item.min_size);
        }

        result
    }

    fn set_every_item_size_to_at_least_min(&mut self) {
        for item in &mut self.items {
            if item.current_size < item.min_size {
                item.current_size = item.min_size;
            }
        }
    }

    fn get_item_with_highest_expected_minus_current_stretch(&self) -> usize {
        let total_stretch = self.gather_items_stretch();

        let mut highest_idx = 0;
        let mut highest_diff = 0.0;
        for (idx, item) in self.items.iter().enumerate() {
            let potential_highest =
                Self::get_item_expected_minus_current_stretch(item, total_stretch, self.size);

            if potential_highest > highest_diff {
                highest_idx = idx;
                highest_diff = potential_highest;
            }
        }

        highest_idx
    }

    fn get_item_expected_minus_current_stretch(
        item: &Item,
        total_stretch: u32,
        total_size: u32,
    ) -> f32 {
        if total_stretch == 0 || total_size == 0 || item.current_size == item.max_size {
            0.0
        } else {
            item.stretch as f32 / total_stretch as f32
                - item.current_size as f32 / total_size as f32
        }
    }
}

#[cfg(test)]
mod tests {
    // super
    use super::*;

    //********************************************************************************************
    #[test]
    fn ensure_layout_has_at_least_minimal_width() {
        {
            let mut layout = Layout {
                size: 0,
                items: vec![
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                ],
            };
            layout.ensure_layout_has_at_least_minimal_width();
            assert_eq!(layout.size, 0);
        }
        {
            let mut layout = Layout {
                size: 0,
                items: vec![
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                ],
            };
            layout.ensure_layout_has_at_least_minimal_width();
            assert_eq!(layout.size, 0);
        }
        {
            let mut layout = Layout {
                size: 0,
                items: vec![
                    Item {
                        stretch: 5,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 5,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 1,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                ],
            };
            layout.ensure_layout_has_at_least_minimal_width();
            assert_eq!(layout.size, 0);
        }
        {
            let mut layout = Layout {
                size: 0,
                items: vec![
                    Item {
                        stretch: 5,
                        min_size: 15,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 5,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 1,
                        min_size: 5,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                ],
            };
            layout.ensure_layout_has_at_least_minimal_width();
            assert_eq!(layout.size, 20);
        }
        {
            let mut layout = Layout {
                size: 0,
                items: vec![
                    Item {
                        stretch: 5,
                        min_size: 35,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 5,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 1,
                        min_size: 5,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                ],
            };
            layout.ensure_layout_has_at_least_minimal_width();
            assert_eq!(layout.size, 40);
        }
        {
            let mut layout = Layout {
                size: 73,
                items: vec![
                    Item {
                        stretch: 1,
                        min_size: 40,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 2,
                        min_size: 0,
                        max_size: 40,
                        current_size: 0,
                    },
                    Item {
                        stretch: 3,
                        min_size: 20,
                        max_size: 60,
                        current_size: 0,
                    },
                ],
            };
            layout.ensure_layout_has_at_least_minimal_width();
            assert_eq!(layout.size, 73);
        }
    }

    //********************************************************************************************
    #[test]
    fn validate_all_items() {
        {
            let mut layout = Layout {
                size: 0,
                items: vec![
                    Item {
                        stretch: 1,
                        min_size: 0,
                        max_size: 0,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: 0,
                        current_size: 1,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: 0,
                        current_size: 2,
                    },
                ],
            };
            layout.validate_all_items();
            assert_eq!(layout.items[0].max_size, 0);
            assert_eq!(layout.items[1].max_size, 0);
            assert_eq!(layout.items[2].max_size, 0);
            assert_eq!(layout.items[0].stretch, 1);
            assert_eq!(layout.items[1].stretch, 0);
            assert_eq!(layout.items[2].stretch, 0);
            assert_eq!(layout.items[0].current_size, 0);
            assert_eq!(layout.items[1].current_size, 0);
            assert_eq!(layout.items[2].current_size, 0);
        }
        {
            let mut layout = Layout {
                size: 0,
                items: vec![
                    Item {
                        stretch: 0,
                        min_size: 5,
                        max_size: 10,
                        current_size: 10,
                    },
                    Item {
                        stretch: 0,
                        min_size: 10,
                        max_size: 5,
                        current_size: 20,
                    },
                    Item {
                        stretch: 0,
                        min_size: 5,
                        max_size: 5,
                        current_size: 30,
                    },
                ],
            };
            layout.validate_all_items();
            assert_eq!(layout.items[0].max_size, 10);
            assert_eq!(layout.items[1].max_size, 10);
            assert_eq!(layout.items[2].max_size, 5);
            assert_eq!(layout.items[0].stretch, 1);
            assert_eq!(layout.items[1].stretch, 1);
            assert_eq!(layout.items[2].stretch, 1);
            assert_eq!(layout.items[0].current_size, 0);
            assert_eq!(layout.items[1].current_size, 0);
            assert_eq!(layout.items[2].current_size, 0);
        }
    }

    //********************************************************************************************
    #[test]
    fn gather_items_stretch() {
        {
            let layout = Layout {
                size: 0,
                items: vec![
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                ],
            };
            assert_eq!(layout.gather_items_stretch(), 0);
        }
        {
            let layout = Layout {
                size: 0,
                items: vec![
                    Item {
                        stretch: 1,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 2,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 3,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                ],
            };
            assert_eq!(layout.gather_items_stretch(), 6);
        }
    }

    //********************************************************************************************
    #[test]
    fn remaining_free_layout_space() {
        {
            let layout = Layout {
                size: 0,
                items: vec![
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                ],
            };
            assert_eq!(layout.remaining_free_layout_space(), 0);
        }
        {
            let layout = Layout {
                size: 10,
                items: vec![
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                ],
            };
            assert_eq!(layout.remaining_free_layout_space(), 10);
        }
        {
            let layout = Layout {
                size: 10,
                items: vec![
                    Item {
                        stretch: 0,
                        min_size: 1,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 2,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 3,
                    },
                ],
            };
            assert_eq!(layout.remaining_free_layout_space(), 4);
        }
        {
            let layout = Layout {
                size: 73,
                items: vec![
                    Item {
                        stretch: 1,
                        min_size: 40,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 2,
                        min_size: 0,
                        max_size: 40,
                        current_size: 0,
                    },
                    Item {
                        stretch: 3,
                        min_size: 20,
                        max_size: 60,
                        current_size: 0,
                    },
                ],
            };
            assert_eq!(layout.remaining_free_layout_space(), 13);
        }
    }

    //********************************************************************************************
    #[test]
    fn set_every_item_size_to_at_least_min() {
        {
            let mut layout = Layout {
                size: 0,
                items: vec![
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: 0,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: 0,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: 0,
                        current_size: 0,
                    },
                ],
            };
            layout.set_every_item_size_to_at_least_min();
            assert_eq!(layout.items[0].max_size, 0);
            assert_eq!(layout.items[1].max_size, 0);
            assert_eq!(layout.items[2].max_size, 0);
        }
        {
            let mut layout = Layout {
                size: 0,
                items: vec![
                    Item {
                        stretch: 0,
                        min_size: 5,
                        max_size: 0,
                        current_size: 3,
                    },
                    Item {
                        stretch: 0,
                        min_size: 10,
                        max_size: 0,
                        current_size: 4,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: 0,
                        current_size: 5,
                    },
                ],
            };
            layout.set_every_item_size_to_at_least_min();
            assert_eq!(layout.items[0].current_size, 5);
            assert_eq!(layout.items[1].current_size, 10);
            assert_eq!(layout.items[2].current_size, 5);
        }
    }

    //********************************************************************************************
    #[test]
    fn get_item_expected_minus_current_stretch() {
        {
            let layout = Layout {
                size: 0,
                items: vec![Item {
                    stretch: 0,
                    min_size: 0,
                    max_size: u32::MAX,
                    current_size: 0,
                }],
            };
            assert_eq!(
                Layout::get_item_expected_minus_current_stretch(&layout.items[0], 0, 0),
                0.0
            );
        }
        {
            let layout = Layout {
                size: 0,
                items: vec![Item {
                    stretch: 1,
                    min_size: 0,
                    max_size: u32::MAX,
                    current_size: 0,
                }],
            };
            assert_eq!(
                Layout::get_item_expected_minus_current_stretch(&layout.items[0], 2, 10),
                0.5
            );
        }
        {
            let layout = Layout {
                size: 0,
                items: vec![Item {
                    stretch: 2,
                    min_size: 0,
                    max_size: u32::MAX,
                    current_size: 0,
                }],
            };
            assert_eq!(
                Layout::get_item_expected_minus_current_stretch(&layout.items[0], 2, 10),
                1.0
            );
        }
        {
            let layout = Layout {
                size: 0,
                items: vec![Item {
                    stretch: 3,
                    min_size: 0,
                    max_size: 5,
                    current_size: 4,
                }],
            };
            assert_eq!(
                Layout::get_item_expected_minus_current_stretch(&layout.items[0], 4, 8),
                0.25
            );
        }
        {
            let layout = Layout {
                size: 0,
                items: vec![Item {
                    stretch: 3,
                    min_size: 0,
                    max_size: 4,
                    current_size: 4,
                }],
            };
            assert_eq!(
                Layout::get_item_expected_minus_current_stretch(&layout.items[0], 4, 8),
                0.0
            );
        }
    }

    //********************************************************************************************
    #[test]
    fn get_item_with_highest_expected_minus_current_stretch() {
        {
            let layout = Layout {
                size: 0,
                items: vec![
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                ],
            };
            assert_eq!(
                layout.get_item_with_highest_expected_minus_current_stretch(),
                0
            );
        }
        {
            let layout = Layout {
                size: 12,
                items: vec![
                    Item {
                        stretch: 1,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 2,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 3,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                ],
            };
            assert_eq!(
                layout.get_item_with_highest_expected_minus_current_stretch(),
                2
            );
        }
        {
            let layout = Layout {
                size: 12,
                items: vec![
                    Item {
                        stretch: 1,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 2,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 3,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 2,
                    },
                ],
            };
            assert_eq!(
                layout.get_item_with_highest_expected_minus_current_stretch(),
                1
            );
        }
        {
            let layout = Layout {
                size: 12,
                items: vec![
                    Item {
                        stretch: 1,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 2,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 3,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 4,
                    },
                ],
            };
            assert_eq!(
                layout.get_item_with_highest_expected_minus_current_stretch(),
                1
            );
        }
        {
            let layout = Layout {
                size: 12,
                items: vec![
                    Item {
                        stretch: 1,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 2,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 4,
                    },
                    Item {
                        stretch: 3,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 4,
                    },
                ],
            };
            assert_eq!(
                layout.get_item_with_highest_expected_minus_current_stretch(),
                0
            );
        }
    }

    //********************************************************************************************
    #[test]
    fn validate_no_items() {
        let layout = Layout::new(120, vec![]);

        assert_eq!(layout.size, 120);
        assert_eq!(layout.items.len(), 0);
    }

    //********************************************************************************************
    #[test]
    fn validate_no_stretch_min_max() {
        let layout = Layout::new(
            120,
            vec![
                Item {
                    stretch: 0,
                    min_size: 0,
                    max_size: u32::MAX,
                    current_size: 0,
                },
                Item {
                    stretch: 0,
                    min_size: 0,
                    max_size: u32::MAX,
                    current_size: 0,
                },
                Item {
                    stretch: 0,
                    min_size: 0,
                    max_size: u32::MAX,
                    current_size: 0,
                },
            ],
        );

        assert_eq!(layout.size, 120);
        assert_eq!(layout.items.len(), 3);
        assert_eq!(layout.items[0].current_size, 40);
        assert_eq!(layout.items[1].current_size, 40);
        assert_eq!(layout.items[2].current_size, 40);
    }

    //********************************************************************************************
    #[test]
    fn validate_stretch() {
        {
            let layout = Layout::new(
                120,
                vec![
                    Item {
                        stretch: 1,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 2,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 3,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                ],
            );

            assert_eq!(layout.size, 120);
            assert_eq!(layout.items.len(), 3);
            assert_eq!(layout.items[0].current_size, 20);
            assert_eq!(layout.items[1].current_size, 40);
            assert_eq!(layout.items[2].current_size, 60);
        }
        {
            let layout = Layout::new(
                120,
                vec![
                    Item {
                        stretch: 1,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 0,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 3,
                        min_size: 0,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                ],
            );

            assert_eq!(layout.size, 120);
            assert_eq!(layout.items.len(), 3);
            assert_eq!(layout.items[0].current_size, 30);
            assert_eq!(layout.items[1].current_size, 0);
            assert_eq!(layout.items[2].current_size, 90);
        }
    }

    //********************************************************************************************
    #[test]
    fn validate() {
        {
            let layout = Layout::new(
                10,
                vec![
                    Item {
                        stretch: 1,
                        min_size: 40,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 2,
                        min_size: 0,
                        max_size: 40,
                        current_size: 0,
                    },
                    Item {
                        stretch: 3,
                        min_size: 20,
                        max_size: 60,
                        current_size: 0,
                    },
                ],
            );

            assert_eq!(layout.size, 60);
            assert_eq!(layout.items.len(), 3);
            assert_eq!(layout.items[0].current_size, 40);
            assert_eq!(layout.items[1].current_size, 0);
            assert_eq!(layout.items[2].current_size, 20);
        }
        {
            let layout = Layout::new(
                61,
                vec![
                    Item {
                        stretch: 1,
                        min_size: 40,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 2,
                        min_size: 0,
                        max_size: 40,
                        current_size: 0,
                    },
                    Item {
                        stretch: 3,
                        min_size: 20,
                        max_size: 60,
                        current_size: 0,
                    },
                ],
            );

            assert_eq!(layout.size, 61);
            assert_eq!(layout.items.len(), 3);
            assert_eq!(layout.items[0].current_size, 40);
            assert_eq!(layout.items[1].current_size, 1);
            assert_eq!(layout.items[2].current_size, 20);
        }
        {
            let layout = Layout::new(
                73,
                vec![
                    Item {
                        stretch: 1,
                        min_size: 40,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 2,
                        min_size: 0,
                        max_size: 40,
                        current_size: 0,
                    },
                    Item {
                        stretch: 3,
                        min_size: 20,
                        max_size: 60,
                        current_size: 0,
                    },
                ],
            );

            assert_eq!(layout.size, 73);
            assert_eq!(layout.items.len(), 3);
            assert_eq!(layout.items[0].current_size, 40);
            assert_eq!(layout.items[1].current_size, 10);
            assert_eq!(layout.items[2].current_size, 23);
        }
        {
            let layout = Layout::new(
                300,
                vec![
                    Item {
                        stretch: 1,
                        min_size: 40,
                        max_size: u32::MAX,
                        current_size: 0,
                    },
                    Item {
                        stretch: 2,
                        min_size: 0,
                        max_size: 40,
                        current_size: 0,
                    },
                    Item {
                        stretch: 3,
                        min_size: 20,
                        max_size: 60,
                        current_size: 0,
                    },
                ],
            );

            assert_eq!(layout.size, 300);
            assert_eq!(layout.items.len(), 3);
            assert_eq!(layout.items[0].current_size, 200);
            assert_eq!(layout.items[1].current_size, 40);
            assert_eq!(layout.items[2].current_size, 60);
        }
    }
}
