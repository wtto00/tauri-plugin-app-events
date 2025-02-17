use serde::Serialize;
use tauri::ipc::Channel;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventHandler {
    pub handler: Channel,
}
