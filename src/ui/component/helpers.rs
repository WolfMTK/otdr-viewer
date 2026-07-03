use leptos::prelude::{GetUntracked, RwSignal, Set};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

pub fn close_on_escape(open: RwSignal<bool>) {
    let Some(win) = web_sys::window() else { return };
    let handler = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event: web_sys::KeyboardEvent| {
        if event.key() == "Escape" && open.get_untracked() {
            open.set(false);
        }
    });
    let _ = win.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref());
    handler.forget();
}

pub fn toggle_class(base: &str, active: &str, condition: bool) -> String {
    if condition {
        format!("{base} {active}")
    } else {
        base.to_string()
    }
}
