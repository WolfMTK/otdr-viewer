use leptos::prelude::*;
use stylance::import_style;
use wasm_bindgen::JsValue;

use crate::tauri::invoke;

import_style!(style, "index.module.css");

fn close_window() {
    wasm_bindgen_futures::spawn_local(async move {
        invoke("close_window", JsValue::NULL).await;
    })
}

fn toggle_maximize() {
    wasm_bindgen_futures::spawn_local(async move {
        invoke("toggle_maximize", JsValue::NULL).await;
    })
}

fn minimize_window() {
    wasm_bindgen_futures::spawn_local(async move {
        invoke("minimize_window", JsValue::NULL).await;
    })
}

#[component]
pub fn TitleBar() -> impl IntoView {
    view! {
        <div class=style::titlebar>
            <div class=style::titlebar_left>
                <img src="public/logo.svg" class=style::app_icon alt="logo" draggable="false" />
                <button class=style::menu_button on:click=|_| {  }>
                    <img src="public/menu.svg" class=style::menu_icon alt="menu" draggable="false" />
                </button>
            </div>
            <div data-tauri-drag-region class=style::titlebar_drag></div>
            <div class=style::titlebar_controls>
                <button class=stylance::classes!(style::dot, style::dot_green)
                        on:click=move |_| minimize_window()/>
                <button class=stylance::classes!(style::dot, style::dot_yellow)
                        on:click=move |_| toggle_maximize()/>
                <button class=stylance::classes!(style::dot, style::dot_red)
                        on:click=move |_| close_window()/>
            </div>
        </div>
    }
}
