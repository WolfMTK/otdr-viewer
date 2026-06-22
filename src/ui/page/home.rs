use leptos::prelude::*;
use stylance::import_style;

use crate::ui::component::sidebar::index::Sidebar;

import_style!(style, "home.module.css");

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class=style::layout>
            <Sidebar/>
            <main class=style::content></main>
        </div>
    }
}
