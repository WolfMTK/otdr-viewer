use tauri::Window;
use tauri_runtime::ResizeDirection;

#[tauri::command]
fn start_resize(window: Window, direction: ResizeDirection) {
    let _ = window.start_resize_dragging(direction);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_resize])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
