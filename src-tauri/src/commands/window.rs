use tauri::Window;
use tauri_runtime::ResizeDirection;

#[tauri::command]
pub(crate) fn start_resize(window: Window, direction: ResizeDirection) {
    let _ = window.start_resize_dragging(direction);
}

#[tauri::command]
pub(crate) fn close_window(window: Window) {
    let _ = window.close();
}

#[tauri::command]
pub(crate) fn toggle_maximize(window: Window) {
    if let Ok(is_maximized) = window.is_maximized() {
        if is_maximized {
            let _ = window.unmaximize();
        } else {
            let _ = window.maximize();
        }
    }
}

#[tauri::command]
pub(crate) fn minimize_window(window: Window) {
    let _ = window.minimize();
}
