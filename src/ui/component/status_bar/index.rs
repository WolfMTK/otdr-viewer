use leptos::prelude::*;
use stylance::import_style;

import_style!(style, "index.module.css");

#[component]
pub fn StatusBar() -> impl IntoView {
    view! {
        <footer class=style::status_bar>
            <span>"Откройте файл .sor или перетащите его в окно"</span>
            <span>"OTDR Viewer 1.0"</span>
        </footer>
    }
}
