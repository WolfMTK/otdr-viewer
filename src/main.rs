use leptos::prelude::{
    mount_to_body, signal, ClassAttribute, CollectView, CustomAttribute, ElementChild, Get, OnAttribute, RwSignal, Set,
    StyleAttribute,
};
pub mod tauri;
mod ui;

use leptos::{view, web_sys};
use serde::Serialize;
use stylance::import_style;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};

use crate::tauri::{invoke, invoke_with_args};
use crate::ui::component::recent_files::index::RecentFiles;
use crate::ui::component::sidebar::index::Sidebar;
use crate::ui::component::status_bar::index::StatusBar;
use crate::ui::component::title_bar::index::TitleBar;
use crate::ui::context::provide_recent_files_version;
use crate::ui::page::home::Home;

import_style!(style, "main.module.css");

const RESIZE_HANDLES: [(&str, &str); 8] = [
    ("n", "North"),
    ("s", "South"),
    ("w", "West"),
    ("e", "East"),
    ("nw", "NorthWest"),
    ("ne", "NorthEast"),
    ("sw", "SouthWest"),
    ("se", "SouthEast"),
];

#[derive(Serialize)]
struct ResizeArgs {
    direction: &'static str,
}

fn start_resize(direction: &'static str) {
    wasm_bindgen_futures::spawn_local(async move {
        invoke_with_args("start_resize", &ResizeArgs { direction }).await;
    });
}

fn main() {
    console_error_panic_hook::set_once();

    let (is_maximized, set_is_maximized) = signal(false);
    let refresh_maximized = move || {
        wasm_bindgen_futures::spawn_local(async move {
            if let Some(value) = invoke("is_window_maximized", JsValue::NULL).await.as_bool() {
                set_is_maximized.set(value);
            }
        });
    };
    refresh_maximized();

    let on_resize = Closure::<dyn FnMut()>::new(refresh_maximized);
    if let Some(win) = web_sys::window() {
        let _ = win.add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref());
    }
    on_resize.forget();

    let panel_open = RwSignal::new(true);

    mount_to_body(move || {
        provide_recent_files_version();
        view! {
            <div style:display=move || if is_maximized.get() { "none" } else { "contents" }>
                {RESIZE_HANDLES
                    .into_iter()
                    .map(|(dir, direction)| {
                        view! {
                            <div
                                class=style::resize_handle
                                data-resize-dir=dir
                                on:mousedown=move |_| start_resize(direction)
                            ></div>
                        }
                    })
                    .collect_view()}
            </div>

            <TitleBar/>

            <div class=style::screen>
                <div class=style::layout>
                    <Sidebar panel_open=panel_open/>
                    <RecentFiles panel_open=panel_open/>
                    <main class=style::content>
                        <Home/>
                    </main>
                </div>
                <StatusBar/>
            </div>
        }
    })
}
