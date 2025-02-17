use serde::de::DeserializeOwned;
use tauri::{
    ipc::Channel,
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::EventHandler;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_app_events);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<AppEvents<R>> {
    #[cfg(target_os = "android")]
    let handle =
        api.register_android_plugin("wang.tato.tauri_plugin_app_events", "AppEventsPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_app_events)?;
    Ok(AppEvents(handle))
}

/// Access to the app-events APIs.
pub struct AppEvents<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> AppEvents<R> {
    pub fn set_resume_handler(&self, handler: Channel) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("setResumeHandler", EventHandler { handler })
            .map_err(Into::into)
    }

    pub fn set_pause_handler(&self, handler: Channel) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("setPauseHandler", EventHandler { handler })
            .map_err(Into::into)
    }
}
