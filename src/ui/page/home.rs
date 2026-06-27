use leptos::prelude::*;
use stylance::import_style;

use crate::ui::component::recent_files::index::RecentFiles;
use crate::ui::component::sidebar::index::Sidebar;
use crate::ui::component::status_bar::index::StatusBar;
use crate::ui::component::dropzone::index::Dropzone;

import_style!(style, "home.module.css");

#[component]
pub fn Home() -> impl IntoView {
    let panel_open = RwSignal::new(true);

    view! {
        <div class=style::screen>
            <div class=style::layout>
                <Sidebar panel_open=panel_open/>
                <RecentFiles panel_open=panel_open/>
                <main class=style::content>
                    <Dropzone/>
                </main>
            </div>
            <StatusBar/>
        </div>
    }
}
