use std::sync::Arc;

use leptos::prelude::*;
use serde::Serialize;
use shared_types::{DirListing, FsEntry, QuickLocation};
use stylance::import_style;

use crate::tauri::{invoke_parsed, invoke_parsed_with_args, invoke_with_args};
use crate::ui::component::helpers::{close_on_escape, toggle_class};
use crate::ui::context::RecentFilesVersion;

import_style!(style, "index.module.css");

#[derive(Serialize)]
struct ListDirectoryArgs {
    path: Option<String>,
}

#[derive(Serialize)]
struct RecordRecentFileArgs {
    path: String,
}

async fn fetch_quick_locations() -> Vec<QuickLocation> {
    invoke_parsed("list_quick_locations").await
}

async fn fetch_directory(path: Option<String>) -> DirListing {
    invoke_parsed_with_args("list_directory", &ListDirectoryArgs { path }).await
}

fn record_recent_file(path: String, recent_files_version: RwSignal<u32>) {
    wasm_bindgen_futures::spawn_local(async move {
        invoke_with_args("record_recent_file", &RecordRecentFileArgs { path }).await;
        recent_files_version.update(|v| *v += 1);
    });
}

fn has_sor_extension(name: &str) -> bool {
    name.to_lowercase().ends_with(".sor")
}

fn join_path(dir: &str, name: &str) -> String {
    if dir.ends_with('/') || dir.ends_with('\\') {
        format!("{dir}{name}")
    } else if dir.contains('\\') {
        format!("{dir}\\{name}")
    } else {
        format!("{dir}/{name}")
    }
}

fn resolve_open_path(current_path: &str, filename: &str, selected_path: Option<String>) -> String {
    match selected_path {
        Some(p) if p.rsplit(['/', '\\']).next() == Some(filename) => p,
        _ => join_path(current_path, filename),
    }
}

fn render_location_button(
    idx: usize,
    name: String,
    active_location: RwSignal<usize>,
    on_select: impl Fn(usize) + Copy + 'static,
) -> impl IntoView {
    view! {
        <button
            class=move || toggle_class(style::location_item, style::location_item_active, active_location.get() == idx)
            on:click=move |_| on_select(idx)
        >
            <img src="public/folder.svg" class=style::location_icon alt="" draggable="false" />
            <span>{name}</span>
        </button>
    }
}

fn render_entry_row(
    entry: FsEntry,
    selected_path: RwSignal<Option<String>>,
    on_click: impl Fn(&FsEntry) + Copy + Send + 'static,
    on_dblclick: impl Fn(&FsEntry) + Copy + Send + 'static,
) -> impl IntoView {
    let icon = if entry.is_dir {
        "public/folder.svg"
    } else {
        "public/file.svg"
    };
    let name = entry.name.clone();
    let size = entry.size_label.clone().unwrap_or_default();
    let date = entry.modified_label.clone().unwrap_or_default();
    let entry_path = entry.path.clone();

    let entry = Arc::new(entry);
    let click_entry = Arc::clone(&entry);
    let dblclick_entry = entry;

    view! {
        <div
            class=move || {
                toggle_class(
                    style::file_row,
                    style::file_row_active,
                    selected_path.get().as_deref() == Some(entry_path.as_str()),
                )
            }
            on:click=move |_| on_click(&click_entry)
            on:dblclick=move |_| on_dblclick(&dblclick_entry)
        >
            <img src=icon class=style::row_icon alt="" draggable="false" />
            <span class=style::row_name>{name}</span>
            <span class=style::row_size>{size}</span>
            <span class=style::row_date>{date}</span>
        </div>
    }
}

#[component]
pub fn OpenFileDialog(open: RwSignal<bool>) -> impl IntoView {
    let RecentFilesVersion(recent_files_version) =
        use_context::<RecentFilesVersion>().expect("RecentFilesVersion is provided at app root");
    let quick_locations = RwSignal::new(Vec::<QuickLocation>::new());
    let active_location = RwSignal::new(0usize);

    let current_path = RwSignal::new(String::new());
    let parent_path = RwSignal::new(None::<String>);
    let entries = RwSignal::new(Vec::<FsEntry>::new());
    let dir_error = RwSignal::new(None::<String>);
    let loading = RwSignal::new(false);
    let loaded_once = RwSignal::new(false);

    let selected_path = RwSignal::new(None::<String>);
    let query = RwSignal::new(String::new());
    let filename = RwSignal::new(String::new());

    let reset_selection = move || {
        query.set(String::new());
        selected_path.set(None);
        filename.set(String::new());
    };

    let load_dir = move |path: Option<String>| {
        loading.set(true);
        wasm_bindgen_futures::spawn_local(async move {
            let listing = fetch_directory(path).await;
            current_path.set(listing.current_path);
            parent_path.set(listing.parent_path);
            entries.set(listing.entries);
            dir_error.set(listing.error);
            loading.set(false);
        });
    };

    Effect::new(move |_| {
        if open.get() && !loaded_once.get_untracked() {
            loaded_once.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let locations = fetch_quick_locations().await;
                let first_path = locations.first().map(|l| l.path.clone());
                quick_locations.set(locations);
                load_dir(first_path);
            });
        }
    });

    let close = move || open.set(false);

    close_on_escape(open);

    let select_location = move |idx: usize| {
        active_location.set(idx);
        reset_selection();
        let path = quick_locations.get_untracked().get(idx).map(|l| l.path.clone());
        load_dir(path);
    };

    let navigate_into = move |path: String| {
        reset_selection();
        load_dir(Some(path));
    };

    let navigate_up = move || {
        if let Some(p) = parent_path.get_untracked() {
            reset_selection();
            load_dir(Some(p));
        }
    };

    let can_open = move || has_sor_extension(&filename.get());

    let confirm_open = move || {
        if !can_open() {
            return;
        }
        let path = resolve_open_path(
            &current_path.get_untracked(),
            &filename.get_untracked(),
            selected_path.get_untracked(),
        );
        record_recent_file(path.clone(), recent_files_version);
        leptos::logging::debug_warn!("Открываем файл: {path}");
        close();
    };

    let select_entry = move |entry: &FsEntry| {
        selected_path.set(Some(entry.path.clone()));
        if !entry.is_dir {
            filename.set(entry.name.clone());
        }
    };

    let activate_entry = move |entry: &FsEntry| {
        if entry.is_dir {
            navigate_into(entry.path.clone());
        } else {
            selected_path.set(Some(entry.path.clone()));
            filename.set(entry.name.clone());
            confirm_open();
        }
    };

    let filtered = Memo::new(move |_| {
        let q = query.get().to_lowercase();
        entries
            .get()
            .into_iter()
            .filter(|e| q.is_empty() || e.name.to_lowercase().contains(&q))
            .collect::<Vec<_>>()
    });

    view! {
        <Show when=move || open.get()>
            <div class=style::overlay>
                <div class=style::dialog>
                    <div class=style::header>
                        <img src="public/folder-open.svg" class=style::header_icon alt="folder" draggable="false" />
                        <span class=style::title>"Открыть рефлектограмму"</span>
                        <button class=style::close_btn on:click=move |_| close()>"×"</button>
                    </div>

                    <div class=style::body>
                        <div class=style::locations>
                            <div class=style::locations_label>"РАСПОЛОЖЕНИЯ"</div>
                            {move || {
                                quick_locations
                                    .get()
                                    .into_iter()
                                    .enumerate()
                                    .map(|(idx, loc)| {
                                        render_location_button(idx, loc.name.clone(), active_location, select_location)
                                    })
                                    .collect_view()
                            }}
                        </div>

                        <div class=style::main_panel>
                            <div class=style::toolbar>
                                <button
                                    class=style::up_btn
                                    title="Наверх"
                                    prop:disabled=move || parent_path.get().is_none()
                                    on:click=move |_| navigate_up()
                                >
                                    "↑"
                                </button>
                                <div class=style::search_box>
                                    <img src="public/search.svg" alt="search" draggable="false" />
                                    <input
                                        type="text"
                                        placeholder="Поиск в папке..."
                                        prop:value=move || query.get()
                                        on:input=move |e| query.set(event_target_value(&e))
                                    />
                                </div>
                                <div class=style::ext_select_wrap>
                                    <select class=style::ext_select>
                                        <option>".sor"</option>
                                    </select>
                                    <img
                                        src="public/chevron-down.svg"
                                        class=style::ext_select_arrow
                                        alt=""
                                        draggable="false"
                                    />
                                </div>
                            </div>

                            <div class=style::file_list>
                                <Show when=move || loading.get()>
                                    <div class=style::loading>"Загрузка..."</div>
                                </Show>

                                <Show when=move || !loading.get() && dir_error.get().is_some()>
                                    <div class=style::error_message>
                                        {move || dir_error.get().unwrap_or_default()}
                                    </div>
                                </Show>

                                <Show when=move || !loading.get() && dir_error.get().is_none()>
                                    {move || {
                                        filtered
                                            .get()
                                            .into_iter()
                                            .map(|entry| render_entry_row(entry, selected_path, select_entry, activate_entry))
                                            .collect_view()
                                    }}

                                    <Show when=move || filtered.with(|f| f.is_empty())>
                                        <div class=style::empty>"Файлы .sor не найдены"</div>
                                    </Show>
                                </Show>
                            </div>
                        </div>
                    </div>

                    <div class=style::footer>
                        <span class=style::filename_label>"Имя файла:"</span>
                        <input
                            type="text"
                            class=style::filename_input
                            prop:value=move || filename.get()
                            on:input=move |e| filename.set(event_target_value(&e))
                        />
                        <button class=style::cancel_btn on:click=move |_| close()>"Отмена"</button>
                        <button
                            class=style::open_btn
                            prop:disabled=move || !can_open()
                            on:click=move |_| confirm_open()
                        >
                            <img src="public/folder-open.svg" alt="" draggable="false" />
                            "Открыть"
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::ui::component::open_dialog::index::{has_sor_extension, join_path, resolve_open_path};

    #[rstest]
    #[case("trace.sor", true)]
    #[case("TRACE.SOR", true)]
    #[case("trace.txt", false)]
    #[case("sor", false)]
    #[case("trace.sorx", false)]
    fn has_sor_extension_cases(#[case] name: &str, #[case] expected: bool) {
        assert_eq!(has_sor_extension(name), expected);
    }

    #[rstest]
    #[case("/home/user", "a.sor", "/home/user/a.sor")]
    #[case("/home/user/", "a.sor", "/home/user/a.sor")]
    #[case(r"C:\Users\admin", "a.sor", r"C:\Users\admin\a.sor")]
    #[case(r"C:\Users\admin\", "a.sor", r"C:\Users\admin\a.sor")]
    fn join_path_cases(#[case] dir: &str, #[case] name: &str, #[case] expected: &str) {
        assert_eq!(join_path(dir, name), expected);
    }

    #[rstest]
    #[case("/current", "a.sor", Some("/data/traces/a.sor"), "/data/traces/a.sor")]
    #[case("/current", "b.sor", Some("/data/traces/a.sor"), "/current/b.sor")]
    #[case("/current", "b.sor", None, "/current/b.sor")]
    fn resolve_open_path_cases(
        #[case] current: &str,
        #[case] filename: &str,
        #[case] selected: Option<&str>,
        #[case] expected: &str,
    ) {
        let selected = selected.map(str::to_string);
        assert_eq!(resolve_open_path(current, filename, selected), expected);
    }
}
