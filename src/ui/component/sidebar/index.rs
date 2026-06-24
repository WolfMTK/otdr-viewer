use leptos::prelude::*;
use stylance::import_style;

import_style!(style, "index.module.css");

#[component]
pub fn Sidebar() -> impl IntoView {
    let (active, set_active) = signal(0usize);

    let btn_class = move |i: usize| {
        if active.get() == i {
            stylance::classes!(style::icon_button, style::active)
        } else {
            style::icon_button.to_string()
        }
    };

    view! {
        <nav class=style::sidebar>
            <button class=move || btn_class(0) on:click=move |_| set_active.set(0)>
                <img src="public/folder.svg" alt="folder" />
            </button>

            <button class=move || btn_class(1) on:click=move |_| set_active.set(1)>
                <img src="public/stack.svg" alt="stack" />
            </button>

            <button class=move || btn_class(2) on:click=move |_| set_active.set(2)>
                <img src="public/info.svg" alt="info" />
            </button>

            <div class=style::spacer></div>

            <button class=style::icon_button title="Свернуть">
                <img src="public/collapse.svg" alt="collapse" />
            </button>
        </nav>
    }
}
