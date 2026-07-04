mod commands;
mod db;
use commands::files::{list_directory, list_quick_locations};
use commands::recent_files::{clear_recent_files, list_recent_files, record_recent_file};
use commands::sor::parse_sor_file;
use commands::window::{close_window, is_window_maximized, minimize_window, start_resize, toggle_maximize};
use db::Db;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let pool = tauri::async_runtime::block_on(db::init(app)).map_err(|e| {
                log::error!("Не удалось инициализировать базу данных: {e}");
                e
            })?;
            app.manage(Db(pool));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_resize,
            close_window,
            toggle_maximize,
            minimize_window,
            is_window_maximized,
            list_directory,
            list_quick_locations,
            record_recent_file,
            list_recent_files,
            clear_recent_files,
            parse_sor_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
