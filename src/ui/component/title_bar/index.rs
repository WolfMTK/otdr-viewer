use leptos::prelude::*;
use stylance::import_style;

import_style!(style, "index.module.css");

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
                <button id="titlebar-maximize" class=stylance::classes!(style::dot, style::dot_green)></button>
                <button id="titlebar-minimize" class=stylance::classes!(style::dot, style::dot_yellow)></button>
                <button id="titlebar-close" class=stylance::classes!(style::dot, style::dot_red)></button>
            </div>
        </div>
    }
}
