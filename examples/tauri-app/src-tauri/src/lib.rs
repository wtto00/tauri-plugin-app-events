#[cfg(mobile)]
use tauri::ipc::Channel;
#[cfg(mobile)]
use tauri_plugin_app_events::AppEventsExt;

use tauri::Emitter;

// Learn more about Tauri commands at https://v2.tauri.app/develop/calling-rust/#commands
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            #[cfg(mobile)]
            {
                let app_handle = app.handle();
                app_handle.plugin(tauri_plugin_app_events::init())?;
                let app_cloned = app_handle.clone();
                app_handle
                    .app_events()
                    .set_resume_handler(Channel::new(move |_| {
                        app_cloned.emit("js-log", "set_resume_handler")?;
                        Ok(())
                    }))?;
                let app_cloned = app_handle.clone();
                app_handle
                    .app_events()
                    .set_pause_handler(Channel::new(move |_| {
                        app_cloned.emit("js-log", "set_pause_handler")?;
                        Ok(())
                    }))?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
