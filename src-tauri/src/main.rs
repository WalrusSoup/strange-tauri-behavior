// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::info;
use tauri::{
    AppHandle, CloseRequestApi, CustomMenuItem, Manager, SystemTray, SystemTrayMenu, SystemTrayMenuItem, Window, WindowBuilder, WindowUrl, Wry
};
use tauri_plugin_log::LogTarget;

#[tauri::command]
async fn close_window(app_handle: AppHandle<Wry>) -> Result<(), ()> {
    info!("Going to programatically close windows - which we don't do");
    let windows = app_handle.windows();
    for(_, window) in windows {
        // kill the window
        window.close().unwrap();
    }

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .system_tray(create_app_tray())
        .plugin(tauri_plugin_log::Builder::default().targets([LogTarget::Stdout]).build())
        .invoke_handler(tauri::generate_handler![close_window])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|handle, event| match event {
            tauri::RunEvent::Ready => {
                tauri_ready(&handle);
                start_background_thread(&handle);
            },
            tauri::RunEvent::ExitRequested { api, .. } => {
                info!("Exit requested - intercepting");
            },
            tauri::RunEvent::WindowEvent { event, .. } => match event {
                tauri::WindowEvent::Destroyed => {
                    info!("Window destroyed - but will this be called when we crash? Should we re-open the window?");
                },
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    info!("Intercepting close");
                    intercept_close_request(handle, &api);
                },
                _ => {}
            }
            _ => {}
        });
}

fn start_background_thread(app_handle: &AppHandle<Wry>) {
    let cloned_handle = app_handle.clone();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let main_window = cloned_handle.get_window("main").unwrap();
            let _ = main_window.url();
            info!("Main Window Is OK!");
        }
    });

    let handle_two = app_handle.clone();

    // std::thread::spawn(move || {
    //     loop {
    //         std::thread::sleep(std::time::Duration::from_secs(5));
    //         info!("CLOSING WINDOWS TO SEE WHAT HAPPENS");
    //         let window_thing = handle_two.get_window("main").unwrap().url();
    //         let windows = handle_two.windows();
    //         for(_, window) in windows {
    //             // kill the window
    //             window.close().unwrap();
    //         }

    //         info!("GOING TO CALL WINDOW URL NOW TO SEE WHAT HAPPENS");
    //         let window_ref = handle_two.get_window("main").unwrap();
    //         info!("GOT WINDOW REFERENCE - CHECKING URL");
    //         info!("URL: {:?}", window_ref.url());
    //         std::thread::sleep(std::time::Duration::from_secs(1));
    //     }
    // });
}

fn intercept_close_request(app_handle: &AppHandle<Wry>, api: &CloseRequestApi) {
    let main_window = app_handle.get_window("main").unwrap();
    let url = main_window.url();
    info!("Intercepted close request for window with URL: {:?}", url);
    api.prevent_close();
}


fn tauri_ready(handle: &AppHandle<Wry>) {
    let main_window = WindowBuilder::new(handle, "main", WindowUrl::App("index.html".into()))
    .title("Window CLoser Test")
    .resizable(true)
    .fullscreen(false)
    .min_inner_size(375.00, 500.00)
    .decorations(true)
    .inner_size(1280.00, 800.00)
    .build()
    .expect("Error building main window.");

    main_window.open_devtools();
}

fn create_app_tray() -> SystemTray {
    // Define menu items for the system tray
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    // Add a separator and the quit menu item to the tray menu
    let mut tray_menu = SystemTrayMenu::new().add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}