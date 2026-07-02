use leptos::prelude::*;
use stylance::import_style;

use crate::tauri::invoke_fire_and_forget;

import_style!(style, "index.module.css");

#[component]
pub fn TitleBar() -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);

    view! {
        <div class=style::titlebar>
            <div class=style::titlebar_left>
                <img src="public/logo.svg" class=style::app_icon alt="logo" draggable="false" />

                <Show when=move || !menu_open.get()>
                    <button class=style::menu_button
                            on:click=move |_| set_menu_open.set(true)>
                        <img src="public/menu.svg" class=style::menu_icon alt="menu" draggable="false" />
                    </button>
                </Show>

                <Show when=move || menu_open.get()>
                    <nav class=style::menu_bar>
                        <button class=style::menu_item>"Файл"</button>
                        <button class=style::menu_item>"Редактирование"</button>
                        <button class=style::menu_item>"Вид"</button>
                        <button class=style::menu_item>"Справка"</button>
                    </nav>
                </Show>
            </div>

            <div data-tauri-drag-region
                 class=style::titlebar_drag></div>

            <div class=style::titlebar_controls>
                <button class=stylance::classes!(style::dot, style::dot_green)
                        on:click=move |_| invoke_fire_and_forget("minimize_window")/>
                <button class=stylance::classes!(style::dot, style::dot_yellow)
                        on:click=move |_| invoke_fire_and_forget("toggle_maximize")/>
                <button class=stylance::classes!(style::dot, style::dot_red)
                        on:click=move |_| invoke_fire_and_forget("close_window")/>
            </div>
        </div>

        <Show when=move || menu_open.get()>
            <div class=style::menu_overlay
                 on:click=move |_| set_menu_open.set(false)></div>
        </Show>
    }
}
