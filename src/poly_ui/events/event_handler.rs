// std
use std::{
    rc::Rc, 
    cell::RefCell,
    fmt::Debug,
};
// super
use super::Event;
use super::KeyPressEvent;
use super::KeyReleaseEvent;
use super::MousePressEvent;
use super::MouseReleaseEvent;
use super::MouseMoveEvent;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum EventHandlerResult {
    NotHandled,
    Handled,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
/// Trait that needs to be implemented by all widgets. The idea is that application calls handle
/// on top level widget (window) and it processes the event. If it doesn't handle event itself it
/// should call handle on all child widgets. If no child widgets handles the event it should
/// return NotHandled. In that case application will call handle with this event on another
/// window (if there is any).
pub trait EventHandler: Debug {
    fn try_handle_by_children(&mut self, event: &Event) -> EventHandlerResult;

    fn handle_event(&mut self, event: &Event) -> EventHandlerResult {
        if self.try_handle_by_children(event) == EventHandlerResult::Handled {
            return EventHandlerResult::Handled;
        }

        match event {
            Event::KeyPressEvent(e) => self.handle_key_press_event(e),
            Event::KeyReleaseEvent(e) => self.handle_key_release_event(e),
            Event::MousePressEvent(e) => self.handle_mouse_press_event(e),
            Event::MouseReleaseEvent(e) => self.handle_mouse_release_event(e),
            Event::MouseMoveEvent(e) => self.handle_mouse_move_event(e),
        }
    }

    fn handle_key_press_event(&mut self, _event: &KeyPressEvent) -> EventHandlerResult {
        EventHandlerResult::NotHandled
    }

    fn handle_key_release_event(&mut self, _event: &KeyReleaseEvent) -> EventHandlerResult {
        EventHandlerResult::NotHandled
    }

    fn handle_mouse_press_event(&mut self, _event: &MousePressEvent) -> EventHandlerResult {
        EventHandlerResult::NotHandled
    }

    fn handle_mouse_release_event(&mut self, _event: &MouseReleaseEvent) -> EventHandlerResult {
        EventHandlerResult::NotHandled
    }

    fn handle_mouse_move_event(&mut self, _event: &MouseMoveEvent) -> EventHandlerResult {
        EventHandlerResult::NotHandled
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
/// Empty implementation of EventHandler used for testing purposes.
#[derive(Debug)]
pub struct MockEventHandler {
    child_handlers: Vec<Rc<RefCell<dyn EventHandler>>>,
}

impl EventHandler for MockEventHandler {
    fn try_handle_by_children(&mut self, _event: &Event) -> EventHandlerResult {
        EventHandlerResult::NotHandled
    }
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[cfg(test)]
mod tests {
    // super
    //use super::*;

    //********************************************************************************************
    #[test]
    fn handle_event() {
    }
}
