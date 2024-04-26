# Issue

We're experiencing an issue where our application will panic when a window goes missing, and in attempting to reproduce the issue in a systematic way, have discovered other oddities.


1. Calling `window.close().unwrap()` from a background `std::thread` causes `tauri::WindowEvent::Destroyed` to be emitted as expected.
2. Calling the same code from `#[tauri::command]` causes the application to close fully gracefully - no `tauri::WindowEvent::Destroyed` is emitted.

