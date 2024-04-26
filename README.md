# Issue

We're experiencing an issue where our application will panic when a window goes missing, and in attempting to reproduce the issue in a systematic way, have discovered other oddities.

Our panics are caused by calls to `url()` (when we send a message to the window). Our assumption was that this window "disappeared", but the application continues to run. Maybe the WebView is crashing?

Our end goal is to somehow have a mechanism for catching when windows go missing and gracefully recover the application, if at all possible.


1. Calling `window.close().unwrap()` from a background `std::thread` causes `tauri::WindowEvent::Destroyed` to be emitted as expected.
2. Calling the same code from `#[tauri::command]` causes the application to close - no `tauri::WindowEvent::Destroyed` or `tauri::RunEvent::ExitRequested` is emitted


To simulate #1, comment back in lines `64-81`.