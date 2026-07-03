use leptos::prelude::*;
use serde::Deserialize;
use stylance::import_style;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};

use crate::tauri::invoke;
use crate::ui::component::recent_files::constants::{MAX_WIDTH, MIN_WIDTH, SIDEBAR_WIDTH};
use crate::ui::context::RecentFilesVersion;

import_style!(style, "index.module.css");

#[derive(Clone, Deserialize)]
struct RecentFileEntry {
    path: String,
    name: String,
    location: String,
    opened_at_label: String,
    length_label: Option<String>,
}

async fn fetch_recent_files() -> Vec<RecentFileEntry> {
    let result = invoke("list_recent_files", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result).unwrap_or_default()
}

fn clear_recent_files() {
    wasm_bindgen_futures::spawn_local(async move {
        invoke("clear_recent_files", JsValue::NULL).await;
    });
}

fn render_entry(entry: RecentFileEntry, selected_path: RwSignal<Option<String>>) -> impl IntoView {
    let entry_path = entry.path.clone();
    let full_path = entry.path.clone();
    let has_length = entry.length_label.is_some();
    let length_label = entry.length_label.clone().unwrap_or_default();
    let item_class = move || {
        if selected_path.get().as_deref() == Some(entry_path.as_str()) {
            stylance::classes!(style::file_item, style::file_item_selected)
        } else {
            style::file_item.to_string()
        }
    };

    view! {
        <div
            class=item_class
            title=full_path
            on:click=move |_| selected_path.set(Some(entry.path.clone()))
        >
            <img src="public/file.svg" class=style::file_icon alt="file" draggable="false" />
            <div class=style::file_info>
                <div class=style::file_name>{entry.name.clone()}</div>
                <div class=style::file_path>{entry.location.clone()}</div>
            </div>
            <div class=style::file_meta>
                <div class=style::file_date>{entry.opened_at_label.clone()}</div>
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

    let files = RwSignal::new(Vec::<RecentFileEntry>::new());
    let selected_path = RwSignal::new(None::<String>);
    let query = RwSignal::new(String::new());
    let width = RwSignal::new(MIN_WIDTH);
    let dragging = RwSignal::new(false);

    Effect::new(move |_| {
        version.get();
        wasm_bindgen_futures::spawn_local(async move {
            files.set(fetch_recent_files().await);
        });
    });

    Effect::new(move |_| {
        let Some(win) = web_sys::window() else { return };

        let on_move = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            if dragging.get_untracked() {
                let win_w = web_sys::window()
                    .and_then(|w| w.inner_width().ok())
                    .and_then(|v| v.as_f64())
                    .unwrap_or(MAX_WIDTH + SIDEBAR_WIDTH);
                let max = (win_w * 0.45).min(MAX_WIDTH);
                let w = (e.client_x() as f64 - SIDEBAR_WIDTH).clamp(MIN_WIDTH, max);
                width.set(w);
            }
        });
        let _ = win.add_event_listener_with_callback("mousemove", on_move.as_ref().unchecked_ref());
        on_move.forget();

        let on_up = Closure::<dyn FnMut()>::new(move || {
            if dragging.get_untracked() {
                dragging.set(false);
            }
        });
        let _ = win.add_event_listener_with_callback("mouseup", on_up.as_ref().unchecked_ref());
        on_up.forget();
    });

    let filtered = move || {
        let q = query.get().to_lowercase();
        files
            .get()
            .into_iter()
            .filter(|f| q.is_empty() || f.name.to_lowercase().contains(&q))
            .collect::<Vec<_>>()
    };

    let panel_class = move || {
        if dragging.get() {
            stylance::classes!(style::panel, style::panel_dragging)
        } else {
            style::panel.to_string()
        }
    };

    view! {
        <Show when=move || panel_open.get()>
            <aside class=panel_class style:width=move || format!("{}px", width.get())>
                <div class=style::header>"Последние файлы"</div>

                <div class=style::search>
                    <img src="public/search.svg" alt="search" draggable="false" />
                    <input
                        type="text"
                        placeholder="Поиск по недавним..."
                        prop:value=move || query.get()
                        on:input=move |e| query.set(event_target_value(&e))
                    />
                </div>

                <div class=style::list>
                    {move || filtered().into_iter().map(|f| render_entry(f, selected_path)).collect_view()}

                    <Show when=move || files.get().is_empty()>
                        <div class=style::empty>"Пока нет открытых файлов"</div>
                    </Show>
                </div>

                <div class=style::footer>
                    <button
                        class=style::clear_btn
                        on:click=move |_| {
                            clear_recent_files();
                            selected_path.set(None);
                            version.update(|v| *v += 1);
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
