package wang.tato.tauri_plugin_app_events

import android.app.Activity
import android.util.Log
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@TauriPlugin
class AppEventsPlugin(private val activity: Activity) : Plugin(activity) {
    override fun onResume() {
        Log.d("TAURI_PLUGIN_APP_EVENTS", "onResume")
        trigger("resume", JSObject())
        super.onResume()
    }

    override fun onPause() {
        Log.d("TAURI_PLUGIN_APP_EVENTS", "onPause")
        trigger("pause", JSObject())
        super.onPause()
    }
}
