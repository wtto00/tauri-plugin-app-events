import { addPluginListener } from "@tauri-apps/api/core";

let _resumeCallback: () => void;
function resumeCallback() {
  _resumeCallback?.();
}
export function onResume(callback: () => void = () => {}) {
  if (!_resumeCallback) {
    addPluginListener("app-events", "resume", resumeCallback);
  }
  _resumeCallback = callback;
}

let _pauseCallback: () => void;
function pauseCallback() {
  _pauseCallback?.();
}
export function onPause(callback: () => void = () => {}) {
  if (!_pauseCallback) {
    addPluginListener("app-events", "pause", pauseCallback);
  }
  _pauseCallback = callback;
}

window.__tauri_android_on_back_key_down__ = () => {};
/** Android Only */
export function onBackKeyDown(callback: KeyDownCallback = () => {}) {
  window.__tauri_android_on_back_key_down__ = callback;
}

window.__tauri_android_on_menu_key_down__ = () => {};
/** Android Only */
export function onMenuKeyDown(callback: KeyDownCallback = () => {}) {
  window.__tauri_android_on_menu_key_down__ = callback;
}

window.__tauri_android_on_search_key_down__ = () => {};
/** Android Only */
export function onSearchKeyDown(callback: KeyDownCallback = () => {}) {
  window.__tauri_android_on_search_key_down__ = callback;
}

window.__tauri_android_on_volume_down_key_down__ = () => {};
/** Android Only */
export function onVolumeDownKeyDown(callback: KeyDownCallback = () => {}) {
  window.__tauri_android_on_volume_down_key_down__ = callback;
}

window.__tauri_android_on_volume_up_key_down__ = () => {};
/** Android Only */
export function onVolumeUpKeyDown(callback: KeyDownCallback = () => {}) {
  window.__tauri_android_on_volume_up_key_down__ = callback;
}

window.__tauri_android_on_key_down__ = (keyCode: number) => {};
/** Android only */
export function onKeyDown(callback: KeyDownCodeCallabck = (keyCode: number) => {}) {
  window.__tauri_android_on_key_down__ = callback;
}
