use leptos::__reexports::send_wrapper::SendWrapper;
use leptos::prelude::{on_cleanup, GetUntracked, RwSignal, Set};
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

    let cleanup = SendWrapper::new((win, handler));
    on_cleanup(move || {
        let (win, handler) = cleanup.take();
        let _ = win.remove_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref());
    });
}

pub fn toggle_class(base: &str, active: &str, condition: bool) -> String {
    if condition {
        format!("{base} {active}")
    } else {
        base.to_string()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::ui::component::helpers::toggle_class;

    #[rstest]
    #[case(true, "base active")]
    #[case(false, "base")]
    fn toggle_class_cases(#[case] condition: bool, #[case] expected: &str) {
        assert_eq!(toggle_class("base", "active", condition), expected);
    }
}
