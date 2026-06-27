use leptos::prelude::*;

use crate::ui::component::dropzone::index::Dropzone;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Dropzone/>
    }
}
