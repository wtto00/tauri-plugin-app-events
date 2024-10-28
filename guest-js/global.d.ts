/**
 * @returns Will prevent default behavior?
 * - true: Not prevent default behavior
 * - false: Prevent default behavior
 */
declare type KeyDownCallback = (() => boolean) | (() => void);

declare type KeyDownCodeCallabck = ((keyCode: number) => boolean) | ((keyCode: number) => void);

declare interface Window {
  __tauri_android_on_back_key_down__: KeyDownCallback;
  __tauri_android_on_menu_key_down__: KeyDownCallback;
  __tauri_android_on_search_key_down__: KeyDownCallback;
  __tauri_android_on_volume_down_key_down__: KeyDownCallback;
  __tauri_android_on_volume_up_key_down__: KeyDownCallback;
  __tauri_android_on_key_down__: KeyDownCodeCallabck;
}
