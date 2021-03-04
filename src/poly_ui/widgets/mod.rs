mod ownership;
mod widget_trait;
mod window;
mod window_provider_trait;
mod window_trait;

pub use ownership::NewWidget;
pub use ownership::OwnedWidget;
pub use widget_trait::MockWidget;
pub use widget_trait::WidgetTrait;
pub use window::Window;
pub use window_provider_trait::WindowProviderTrait;
pub use window_trait::WindowTrait;
