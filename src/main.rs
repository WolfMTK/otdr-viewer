mod ui;

use crate::ui::component::title_bar::index::TitleBar;
use crate::ui::page::home::Home;
use leptos::prelude::*;
use stylance::import_style;

import_style!(style, "main.module.css");

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <div class=style::resize_handle data-tauri-drag-resize-region="n"></div>
            <div class=style::resize_handle data-tauri-drag-resize-region="s"></div>
            <div class=style::resize_handle data-tauri-drag-resize-region="w"></div>
            <div class=style::resize_handle data-tauri-drag-resize-region="e"></div>
            <div class=style::resize_handle data-tauri-drag-resize-region="nw"></div>
            <div class=style::resize_handle data-tauri-drag-resize-region="ne"></div>
            <div class=style::resize_handle data-tauri-drag-resize-region="sw"></div>
            <div class=style::resize_handle data-tauri-drag-resize-region="se"></div>

            <TitleBar/>
            <Home/>
        }
    })
}
