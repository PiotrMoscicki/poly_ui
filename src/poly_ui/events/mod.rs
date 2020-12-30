mod event;
mod event_handler;

pub use event::Key;
pub use event::MouseButton;
pub use event::KeyPressEvent;
pub use event::KeyReleaseEvent;
pub use event::MousePressEvent;
pub use event::MouseReleaseEvent;
pub use event::MouseMoveEvent;
pub use event::Event;

pub use event_handler::EventHandler;