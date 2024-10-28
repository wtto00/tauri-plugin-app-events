#[cfg(mobile)]
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(mobile)]
mod mobile;

#[cfg(mobile)]
mod error;

#[cfg(mobile)]
pub use error::{Error, Result};

#[cfg(mobile)]
use mobile::AppEvents;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the app-events APIs.
///
#[cfg(mobile)]
pub trait AppEventsExt<R: Runtime> {
    fn app_events(&self) -> &AppEvents<R>;
}

#[cfg(mobile)]
impl<R: Runtime, T: Manager<R>> crate::AppEventsExt<R> for T {
    fn app_events(&self) -> &AppEvents<R> {
        self.state::<AppEvents<R>>().inner()
    }
}

/// Initializes the plugin.
#[cfg(mobile)]
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("app-events")
        .setup(|app, api| {
            let app_events = mobile::init(app, api)?;
            app.manage(app_events);
            Ok(())
        })
        .build()
}
