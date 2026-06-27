use leptos::prelude::*;
use stylance::import_style;

use crate::ui::component::recent_files::index::RecentFiles;
use crate::ui::component::sidebar::index::Sidebar;

import_style!(style, "home.module.css");

#[component]
pub fn Home() -> impl IntoView {
    let panel_open = RwSignal::new(true);

    view! {
        <div class=style::layout>
            <Sidebar panel_open=panel_open/>
            <RecentFiles panel_open=panel_open/>
            <main class=style::content></main>
        </div>
    }
}
