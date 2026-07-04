use leptos::prelude::*;
use stylance::import_style;

use crate::ui::component::helpers::close_on_escape;

import_style!(style, "index.module.css");

#[component]
pub fn DialogShell(open: RwSignal<bool>, class: &'static str, children: ChildrenFn) -> impl IntoView {
    close_on_escape(open);

    view! {
        <Show when=move || open.get()>
            <div class=style::overlay>
                <div class=class>{children()}</div>
            </div>
        </Show>
    }
}
