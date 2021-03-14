// std
use std::{boxed::Box, cell::RefCell, ops::FnMut, rc::Rc};

//************************************************************************************************
#[allow(dead_code)]
pub struct Slot<Args> {
    fn_slot: Box<dyn FnMut(Args)>,
}

#[allow(dead_code)]
impl<Args> Slot<Args> {
    pub fn new(fn_slot: Box<dyn FnMut(Args)>) -> Self {
        Self { fn_slot }
    }
}

//************************************************************************************************
#[allow(dead_code)]
pub struct Signal<Args> {
    slots: Vec<Rc<RefCell<Slot<Args>>>>,
}

#[allow(dead_code)]
impl<Args> Signal<Args> {
    pub fn connect(&mut self, slot: Rc<RefCell<Slot<Args>>>) {
        self.slots.push(slot);
    }
}

//************************************************************************************************
#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    
    use super::*;

    struct WidgetInternal {}

    impl WidgetInternal {
        fn new() -> Self {
            Self {}
        }

        fn on_damage_dealt(&mut self, damage: i32) {
            println!("Damage dealt: {}", damage);
        }
    }

    struct Widget {
        internal: Rc<RefCell<WidgetInternal>>,
        sl_damage_dealt: Slot<i32>,
    }

    impl Widget {
        // pub fn new() -> Self {
        //     let tmp_internal = Rc::new(RefCell::new(WidgetInternal::new()));
        //     let fn_slot = {
        //         |damage: i32| {
        //             tmp_internal.borrow_mut().on_damage_dealt(damage);
        //         }
        //     };
        //     let sl_damage_dealt = Slot::<i32>::new(Box::new(fn_slot));

        //     Self {
        //         internal: tmp_internal.clone(),
        //         sl_damage_dealt,
        //     }
        // }
    }

    #[test]
    fn handle_intent() {}
}
