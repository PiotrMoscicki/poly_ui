// super
use super::WindowsManagerTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
/// Trait for the application root. It encapsulates third party dependencies and initialization.
pub trait AppTrait {
    /// Application main loop is here. Wverything gets initialized and application enters loop. In
    /// this loop events will be processed, widgets updated and painted.
    /// # Returns
    /// Result; None if application exited with no error, String with error description otherwise.
    /// It should be probably changed to error code to keep to the industry standards.
    fn exec(&mut self) -> Result<(), String>;

    /// App not only manages events and windows painting but contains WindowsManager which can be
    /// obtained to create windows.
    /// # Returns
    /// Windows Manager owned by this App which allows you to create windows.
    fn get_windows_manager(&mut self) -> &mut dyn WindowsManagerTrait;
}
