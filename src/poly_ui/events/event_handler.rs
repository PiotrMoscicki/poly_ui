// super
use super::Event;

pub enum EventHandlerResult {
    NotHandled,
    Handled
}

 /// Trait that needs to be implemented by all widgets. The idea is that application calls handle 
 /// on top level widget (window) and it processes the event. If it doesn't handle event itself it
 ///  should call handle on all child widgets. If no child widgets handles the event it should 
 /// return NotHandled. In that case application will call handle with this event on another 
 /// window (if there is any).
pub trait EventHandler {
    fn handle(&mut self, event: &Event) -> EventHandlerResult;
}
