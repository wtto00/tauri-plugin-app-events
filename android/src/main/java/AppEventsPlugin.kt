package wang.tato.tauri_plugin_app_events

import android.app.Activity
import app.tauri.Logger
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Channel
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@InvokeArg
class SetEventHandlerArgs {
    lateinit var handler: Channel
}

@TauriPlugin
class AppEventsPlugin(private val activity: Activity) : Plugin(activity) {
    private var resumeChannel: Channel? = null
    private var pauseChannel: Channel? = null

    override fun onResume() {
        Logger.debug("TAURI_PLUGIN_APP_EVENTS", "onResume")
        val event = JSObject()
        trigger("resume", event)
        resumeChannel?.send(event)
        super.onResume()
    }

    @Command
    fun setResumeHandler(invoke: Invoke) {
        Logger.debug("TAURI_PLUGIN_APP_EVENTS","setResumeHandler")
        val args = invoke.parseArgs(SetEventHandlerArgs::class.java)
        resumeChannel = args.handler
        invoke.resolve()
    }

    override fun onPause() {
        Logger.debug("TAURI_PLUGIN_APP_EVENTS", "onPause")
        val event = JSObject()
        trigger("pause", event)
        pauseChannel?.send(event)
        super.onPause()
    }

    @Command
    fun setPauseHandler(invoke: Invoke) {
        Logger.debug("TAURI_PLUGIN_APP_EVENTS","setPauseHandler")
        val args = invoke.parseArgs(SetEventHandlerArgs::class.java)
        pauseChannel = args.handler
        invoke.resolve()
    }

}
