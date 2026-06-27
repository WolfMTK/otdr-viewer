use leptos::prelude::*;
use stylance::import_style;

import_style!(style, "index.module.css");

#[component]
pub fn Dropzone() -> impl IntoView {
    let dragover = RwSignal::new(false);

    let card_class = move || {
        if dragover.get() {
            stylance::classes!(style::card, style::card_dragover)
        } else {
            style::card.to_string()
        }
    };

    view! {
        <div
            class=card_class
            on:dragover=move |e| {
                e.prevent_default();
                dragover.set(true);
            }
            on:dragleave=move |_| dragover.set(false)
            on:drop=move |e| {
                e.prevent_default();
                dragover.set(false);
            }
        >
            <div class=style::upload_icon>
                <img src="public/upload.svg" alt="upload" draggable="false" />
            </div>

            <div class=style::title>"Перетащите файл .sor сюда"</div>
            <div class=style::subtitle>
                "Или выберите его через панель недавних файлов, либо нажмите кнопку ниже."
            </div>

            <button class=style::open_btn
                    on:click=move |_| {
                    }>
                <img src="public/folder-open.svg" alt="folder" draggable="false" />
                "Открыть файл..."
            </button>

            <div class=style::divider></div>

            <div class=style::footer>
                <span class=style::format>"Telcordia SR-4731 · .sor"</span>
                <a class=style::help_link>"Что такое .sor?"</a>
            </div>
        </div>
    }
}
