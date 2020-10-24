mod widget_trait;
mod window_trait;
mod window_provider_trait;
mod widget;
mod window;

pub use widget_trait::WidgetTrait;
pub use window_trait::WindowTrait;
pub use window_provider_trait::WindowProviderTrait;
pub use window_provider_trait::set_window_pos;
pub use window_provider_trait::set_window_size;
pub use widget::Widget;
pub use window::Window;