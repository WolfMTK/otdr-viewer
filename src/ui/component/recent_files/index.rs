use leptos::ev;
use leptos::prelude::*;
use leptos_use::{use_event_listener, use_window};
use shared_types::RecentFileEntry;
use stylance::import_style;

use crate::tauri::{try_invoke, try_invoke_parsed};
use crate::ui::component::helpers::{filter_by_name, toggle_class, SearchBox};
use crate::ui::component::recent_files::constants::{MAX_WIDTH, MIN_WIDTH, PANEL_MAX_WINDOW_FRACTION, SIDEBAR_WIDTH};
use crate::ui::context::RecentFilesVersion;

import_style!(style, "index.module.css");

fn render_entry(entry: RecentFileEntry, selected_path: RwSignal<Option<String>>) -> impl IntoView {
    let entry_path = entry.path.clone();
    let click_path = entry.path.clone();
    let has_length = entry.length_label.is_some();
    let length_label = entry.length_label.unwrap_or_default();
    let item_class = move || {
        toggle_class(
            style::file_item,
            style::file_item_selected,
            selected_path.get().as_deref() == Some(entry_path.as_str()),
        )
    };

    view! {
        <div
            class=item_class
            title=entry.path
            on:click=move |_| selected_path.set(Some(click_path.clone()))
        >
            <img src="public/file.svg" class=style::file_icon alt="file" draggable="false" />
            <div class=style::file_info>
                <div class=style::file_name>{entry.name}</div>
                <div class=style::file_path>{entry.location}</div>
            </div>
            <div class=style::file_meta>
                <div class=style::file_date>{entry.opened_at_label}</div>
                <Show when=move || has_length>
                    <div class=style::file_length>{length_label.clone()}</div>
                </Show>
            </div>
        </div>
    }
}

#[component]
pub fn RecentFiles(panel_open: RwSignal<bool>) -> impl IntoView {
    let RecentFilesVersion(version) =
        use_context::<RecentFilesVersion>().expect("RecentFilesVersion is provided at app root");

    let clear_error = RwSignal::new(None::<String>);
    let selected_path = RwSignal::new(None::<String>);
    let query = RwSignal::new(String::new());
    let width = RwSignal::new(MIN_WIDTH);
    let dragging = RwSignal::new(false);

    let files_res = LocalResource::new(move || {
        version.get();
        try_invoke_parsed::<Vec<RecentFileEntry>>("list_recent_files")
    });

    let load_error = Memo::new(move |_| files_res.get().and_then(|r| r.as_ref().err().cloned()));
    let shown_error = Memo::new(move |_| clear_error.get().or_else(|| load_error.get()));

    let is_empty = Memo::new(move |_| {
        files_res
            .get()
            .map(|r| r.as_ref().map(|v| v.is_empty()).unwrap_or(false))
            .unwrap_or(false)
    });

    let _ = use_event_listener(use_window(), ev::mousemove, move |e| {
        if dragging.get_untracked() {
            let win_w = window()
                .inner_width()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(MAX_WIDTH + SIDEBAR_WIDTH);
            let max = (win_w * PANEL_MAX_WINDOW_FRACTION).min(MAX_WIDTH);
            let w = (e.client_x() as f64 - SIDEBAR_WIDTH).clamp(MIN_WIDTH, max);
            width.set(w);
        }
    });

    let _ = use_event_listener(use_window(), ev::mouseup, move |_| {
        if dragging.get_untracked() {
            dragging.set(false);
        }
    });

    let filtered = filter_by_name(
        move || {
            files_res
                .get()
                .and_then(|r| r.as_ref().ok().cloned())
                .unwrap_or_default()
        },
        query,
        |f| &f.name,
    );

    let panel_class = move || toggle_class(style::panel, style::panel_dragging, dragging.get());

    view! {
        <Show when=move || panel_open.get()>
            <aside class=panel_class style:width=move || format!("{}px", width.get())>
                <div class=style::header>"Последние файлы"</div>

                <SearchBox query=query placeholder="Поиск по недавним..." class=style::search/>

                <div class=style::list>
                    <Show when=move || shown_error.get().is_some()>
                        <div class=style::error>{move || shown_error.get().unwrap_or_default()}</div>
                    </Show>

                    <Show when=move || load_error.get().is_none()>
                        <For
                            each=move || filtered.get()
                            key=|f| f.path.clone()
                            children=move |f| render_entry(f, selected_path)
                        />

                        <Show when=move || is_empty.get()>
                            <div class=style::empty>"Пока нет открытых файлов"</div>
                        </Show>
                    </Show>
                </div>

                <div class=style::footer>
                    <button
                        class=style::clear_btn
                        on:click=move |_| {
                            selected_path.set(None);
                            wasm_bindgen_futures::spawn_local(async move {
                                match try_invoke("clear_recent_files").await {
                                    Ok(()) => {
                                        clear_error.set(None);
                                        version.update(|v| *v += 1);
                                    }
                                    Err(e) => clear_error.set(Some(e)),
                                }
                            });
                        }
                    >
                        <img src="public/trash.svg" alt="trash" draggable="false" />
                        "Очистить список"
                    </button>
                </div>

        <div class=style::resize_handle
                     on:mousedown=move |e| {
                         e.prevent_default();
                         dragging.set(true);
                     }></div>
            </aside>
        </Show>
    }
}
