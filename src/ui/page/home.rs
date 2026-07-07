use leptos::prelude::*;

use crate::ui::component::dropzone::index::Dropzone;
use crate::ui::context::OpenedSor;

#[component]
pub fn Home() -> impl IntoView {
    let OpenedSor(opened) = use_context::<OpenedSor>().expect("OpenedSor is provided at app root");

    view! {
        <Show when=move || opened.get().is_none()>
            <Dropzone/>
        </Show>

        <Show when=move || opened.get().is_some()>
            <div>{move || {
                opened.get().map(|d| format!(
                    "Открыт файл: {} точек трассы, {} событий",
                    d.trace.distances_km.len(),
                    d.summary.events.len(),
                )).unwrap_or_default()
            }}</div>
        </Show>
    }
}
