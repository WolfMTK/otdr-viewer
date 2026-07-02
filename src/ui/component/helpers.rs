use leptos::prelude::{Effect, Get, RwSignal, Set};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

pub fn close_on_escape(open: RwSignal<bool>) {
    Effect::new(move |_| {
        if !open.get() {
            return;
        }
        let Some(win) = web_sys::window() else { return };
        let handler = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |effect: web_sys::KeyboardEvent| {
            if effect.key() == "Escape" {
                open.set(false);
            }
        });
        let _ = win.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref());
        handler.forget();
    });
}
