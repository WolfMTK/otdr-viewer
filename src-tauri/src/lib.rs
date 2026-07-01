mod commands;
use commands::files::{list_directory, list_quick_locations};
use commands::window::{close_window, is_window_maximized, minimize_window, start_resize, toggle_maximize};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            start_resize,
            close_window,
            toggle_maximize,
            minimize_window,
            is_window_maximized,
            list_directory,
            list_quick_locations,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
