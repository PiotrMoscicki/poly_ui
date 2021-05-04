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
pub trait EventHandler {
    fn get_child_handlers(&mut self) -> Vec<&mut dyn EventHandler>;

    fn handle_event(&mut self, event: &Event) -> EventHandlerResult {
        let mut result = EventHandlerResult::NotHandled;
        for handler in self.get_child_handlers() {
            result = handler.handle_event(event);
        }

        if result == EventHandlerResult::Handled
            result

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
