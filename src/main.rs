pub mod tauri;
mod ui;

use leptos::prelude::*;
use serde::Serialize;
use stylance::import_style;
use wasm_bindgen::JsValue;

use crate::tauri::invoke;
use crate::ui::component::title_bar::index::TitleBar;
use crate::ui::page::home::Home;

import_style!(style, "main.module.css");

#[derive(Serialize)]
struct ResizeArgs {
    direction: &'static str,
}

fn start_resize(direction: &'static str) {
    wasm_bindgen_futures::spawn_local(async move {
        let args = serde_wasm_bindgen::to_value(&ResizeArgs { direction }).unwrap_or(JsValue::NULL);
        invoke("start_resize", args).await;
    });
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <div class=style::resize_handle data-resize-dir="n" on:mousedown=move |_| start_resize("North")></div>
            <div class=style::resize_handle data-resize-dir="s" on:mousedown=move |_| start_resize("South")></div>
            <div class=style::resize_handle data-resize-dir="w" on:mousedown=move |_| start_resize("West")></div>
            <div class=style::resize_handle data-resize-dir="e" on:mousedown=move |_| start_resize("East")></div>
            <div class=style::resize_handle data-resize-dir="nw" on:mousedown=move |_| start_resize("NorthWest")></div>
            <div class=style::resize_handle data-resize-dir="ne" on:mousedown=move |_| start_resize("NorthEast")></div>
            <div class=style::resize_handle data-resize-dir="sw" on:mousedown=move |_| start_resize("SouthWest")></div>
            <div class=style::resize_handle data-resize-dir="se" on:mousedown=move |_| start_resize("SouthEast")></div>

            <TitleBar/>
            <Home/>
        }
    })
}
