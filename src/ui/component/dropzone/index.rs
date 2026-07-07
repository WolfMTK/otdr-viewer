use leptos::prelude::*;
use serde::Deserialize;
use stylance::import_style;

use crate::tauri::{listen_app_lifetime, listen_app_lifetime_parsed};
use crate::ui::component::helpers::{has_sor_extension, open_sor_file, toggle_class};
use crate::ui::component::open_dialog::index::OpenFileDialog;
use crate::ui::component::sor_info::index::SorInfoDialog;
use crate::ui::context::{OpenedSor, RecentFilesVersion};

import_style!(style, "index.module.css");

#[derive(Deserialize)]
struct DragDropPayload {
    paths: Vec<String>,
}

#[component]
pub fn Dropzone() -> impl IntoView {
    let RecentFilesVersion(version) =
        use_context::<RecentFilesVersion>().expect("RecentFilesVersion is provided at app root");
    let OpenedSor(opened) = use_context::<OpenedSor>().expect("OpenedSor is provided at app root");

    let dragover = RwSignal::new(false);
    let dialog_open = RwSignal::new(false);
    let info_open = RwSignal::new(false);
    let drop_error = RwSignal::new(None::<String>);

    listen_app_lifetime("tauri://drag-enter", move |_| dragover.set(true));
    listen_app_lifetime("tauri://drag-leave", move |_| dragover.set(false));
    listen_app_lifetime_parsed::<DragDropPayload>("tauri://drag-drop", move |payload| {
        dragover.set(false);
        match payload.paths.into_iter().find(|p| has_sor_extension(p)) {
            Some(path) => {
                drop_error.set(None);
                wasm_bindgen_futures::spawn_local(async move {
                    if let Err(e) = open_sor_file(path, opened, version).await {
                        drop_error.set(Some(e));
                    }
                });
            }
            None => drop_error.set(Some("Можно открыть только файлы .sor".to_string())),
        }
    });

    let card_class = move || toggle_class(style::card, style::card_dragover, dragover.get());

    view! {
        <div class=card_class>
            <div class=style::upload_icon>
                <img src="public/upload.svg" alt="upload" draggable="false" />
            </div>

            <div class=style::title>"Перетащите файл .sor сюда"</div>
            <div class=style::subtitle>
                "Или выберите его через панель недавних файлов, либо нажмите кнопку ниже."
            </div>

            <Show when=move || drop_error.get().is_some()>
                <div class=style::drop_error>{move || drop_error.get().unwrap_or_default()}</div>
            </Show>

            <button class=style::open_btn
                    on:click=move |_| dialog_open.set(true)>
                <img src="public/folder-open.svg" alt="folder" draggable="false" />
                "Открыть файл..."
            </button>

            <div class=style::divider></div>

            <div class=style::footer>
                <span class=style::format>"Telcordia SR-4731 · .sor"</span>
                <a class=style::help_link
                   on:click=move |_| info_open.set(true)>
                    "Что такое .sor?"
                </a>
            </div>
        </div>

        <OpenFileDialog open=dialog_open/>

        <SorInfoDialog open=info_open/>
    }
}
