# Tauri Plugin app-events

A plugin for [tauri@v2](https://v2.tauri.app/zh-cn/) to listen some events on iOS and Android.

## Platform Supported

| Platform | Supported |
| -------- | :-------: |
| Linux    |    ❌     |
| Windows  |    ❌     |
| macOS    |    ❌     |
| Android  |    ✅     |
| iOS      |    ✅     |

For desktop platforms, please use [@tauri-apps/api/event](https://v2.tauri.app/reference/javascript/api/namespaceevent/) or the JavaScript method [keydown event](https://developer.mozilla.org/zh-CN/docs/Web/API/Element/keydown_event).

## Setup

Install the `app-events` plugin to get started.

1. Run the following command in the `src-tauri` folder to add the plugin to the project’s dependencies in `Cargo.toml`:

   ```shell
   cargo add tauri-plugin-app-events@0.1 --target 'cfg(any(target_os = "android", target_os = "ios"))'
   ```

1. Modify `lib.rs` to initialize the plugin:

   ```diff
    #[cfg_attr(mobile, tauri::mobile_entry_point)]
    pub fn run() {
        tauri::Builder::default()
            .setup(|app| {
   +            #[cfg(mobile)]
   +            let _ = app.handle().plugin(tauri_plugin_app_events::init());
                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
   ```

1. Modify `src-tauri/capabilities/mobile.json` to Allow the frontend to execute the `registerListener` command.

   ```diff
    {
      "$schema": "../gen/schemas/mobile-schema.json",
      "identifier": "mobile",
      "description": "Capability for the main window on mobile platform",
      "windows": ["main"],
      "permissions": [
   +    "app-events:default"
      ],
      "platforms": ["android", "iOS"]
    }
   ```

1. If you want to support key event listeners on the Android platform, you'll need to modify the `MainActivity.kt` file located at `src-tauri/gen/android/app/src/main/java/com/tauri/dev/MainActivity.kt`. The content should be updated as follows:

   ```kotlin
   package com.tauri.dev

   import android.view.KeyEvent
   import android.webkit.WebView

   class MainActivity : TauriActivity() {
     private lateinit var wv: WebView

     override fun onWebViewCreate(webView: WebView) {
       wv = webView
     }

     private val keyEventMap = mapOf(
       KeyEvent.KEYCODE_BACK to "back",
       KeyEvent.KEYCODE_MENU to "menu",
       KeyEvent.KEYCODE_SEARCH to "search",
       KeyEvent.KEYCODE_VOLUME_DOWN to "volume_down",
       KeyEvent.KEYCODE_VOLUME_UP to "volume_up"
     )

     override fun onKeyDown(keyCode: Int, event: KeyEvent?): Boolean {
       val jsCallbackName = keyEventMap[keyCode]
       wv.evaluateJavascript(
         """
           try {
             window.__tauri_android_on_${if (jsCallbackName != null) "${jsCallbackName}_" else ""}key_down__(${if (jsCallbackName != null) "" else keyCode})
           } catch (_) {
             true
           }
       """.trimIndent()
       ) { result ->
         run {
           if (result != "false") {
             super.onKeyDown(keyCode, event)
           }
         }
       }
       return true
     }
   }
   ```

1. Install the JavaScript Guest bindings using your preferred JavaScript package manager:

   ```shell
   pnpm add tauri-plugin-app-events-api@0.1
   ```

## Usage

### Api Support

| Api                 | Android | iOS |
| ------------------- | :-----: | :-: |
| onResume            |   ✅    | ✅  |
| onPause             |   ✅    | ✅  |
| onBackKeyDown       |   ✅    | ❌  |
| onMenuKeyDown       |   ✅    | ❌  |
| onSearchKeyDown     |   ✅    | ❌  |
| onVolumeDownKeyDown |   ✅    | ❌  |
| onVolumeUpKeyDown   |   ✅    | ❌  |
| onKeyDown           |   ✅    | ❌  |

### Import Apis

```js
import {
  onResume,
  onPause,
  onBackKeyDown,
  onMenuKeyDown,
  onSearchKeyDown,
  onVolumeDownKeyDown,
  onVolumeUpKeyDown,
  onKeyDown,
} from "tauri-plugin-app-events-api";
```

### Note

All listener APIs only support a single callback, so repeated calls will cause the previous listener to be overridden.

Therefore, if you want to cancel the listener, you can pass an empty parameter in functions like `onResume()`.

In `keydown event` listeners of Android, if the callback function returns `false`, the default handling logic will be prevented. If it returns any other value or doesn't return anything, the default handling logic will continue.

### onResume

The `resume` event fires when the system pulls the application out from the background.

```js
onResume(() => {
  console.log("App resume");
});
```

### onPause

The `pause` event fires when the system puts the application into the background, typically when the user switches to a different application.

```js
onResume(() => {
  console.log("App pause");
});
```

### onBackKeyDown

The event fires when the user presses the back button on Android.

```js
onBackKeyDown(() => {
  console.log("Back key triggered");
  return false;
});
```

### onMenuKeyDown

The event fires when the user presses the menu button on Android.

```js
onMenuKeyDown(() => {
  console.log("Menu key triggered");
  return false;
});
```

### onSearchKeyDown

The event fires when the user presses the search button on Android.

```js
onSearchKeyDown(() => {
  console.log("Search key triggered");
  return false;
});
```

### onVolumeDownKeyDown

The event fires when the user presses the volume down button on Android.

```js
onVolumeDownKeyDown(() => {
  console.log("Volume down key triggered");
  return false;
});
```

### onVolumeUpKeyDown

The event fires when the user presses the volume up button on Android.

```js
onVolumeUpKeyDown(() => {
  console.log("Volume up key triggered");
  return false;
});
```

### onKeyDown

If you want to listen for the events of pressing other buttons, you can use `onKeyDown`. Note that this does not include the aforementioned button events.

```js
onKeyDown((keyCode) => {
  console.log(`Key ${keyCode} triggered`);
  return false;
});
```

Regarding `keyCode`, you can refer to the [Android documentation](https://developer.android.com/reference/android/view/KeyEvent#constants_1).

## [Permission description](./permissions/autogenerated/reference.md)
