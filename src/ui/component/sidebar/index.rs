use leptos::prelude::*;
use stylance::import_style;

use crate::ui::component::helpers::toggle_class;

import_style!(style, "index.module.css");

#[component]
pub fn Sidebar(panel_open: RwSignal<bool>) -> impl IntoView {
    let (active, set_active) = signal(0usize);

    let btn_class = move |i: usize| toggle_class(style::icon_button, style::active, active.get() == i);

    view! {
        <nav class=style::sidebar>
            <button class=move || btn_class(0) on:click=move |_| set_active.set(0)>
                <img src="public/folder.svg" alt="folder" draggable="false" />
            </button>

            <button class=move || btn_class(1) on:click=move |_| set_active.set(1)>
                <img src="public/stack.svg" alt="stack" draggable="false" />
            </button>

            <button class=move || btn_class(2) on:click=move |_| set_active.set(2)>
                <img src="public/info.svg" alt="info" draggable="false" />
            </button>

            <div class=style::spacer></div>

            <button class=style::icon_button
                    title="Свернуть"
                    on:click=move |_| panel_open.update(|v| *v = !*v)>
                <img src="public/collapse.svg"
                     alt="collapse"
                     draggable="false"
                     class=move || if panel_open.get() { String::new() } else { style::flipped.to_string() } />
            </button>
        </nav>
    }
}
