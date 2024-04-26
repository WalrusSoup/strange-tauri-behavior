# Issue

We're experiencing an issue where our application will panic when a window goes missing, and in attempting to reproduce the issue in a systematic way, have discovered other oddities.

Our panics are caused by calls to `url()` - which is somehow missing. Our assumptino was that this window "disappeared", but the application continues to run. Maybe the WebView is crashing? In that case, how do we recover from this error gracefully or otherwise catch it and re-open our webview?


1. Calling `window.close().unwrap()` from a background `std::thread` causes `tauri::WindowEvent::Destroyed` to be emitted as expected.
2. Calling the same code from `#[tauri::command]` causes the application to close - no `tauri::WindowEvent::Destroyed` is emitted.
