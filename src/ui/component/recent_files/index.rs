use leptos::prelude::*;
use stylance::import_style;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use crate::ui::component::recent_files::constants::{MAX_WIDTH, MIN_WIDTH, SIDEBAR_WIDTH};

import_style!(style, "index.module.css");

#[derive(Clone)]
struct RecentFile {
    name: &'static str,
    path: &'static str,
    date: &'static str,
    length: &'static str,
}

// TODO: remove the stub after connecting the database
fn files() -> Vec<RecentFile> {
    vec![
        RecentFile {
            name: "cable_route_A1.sor",
            path: "/home/user/measure...",
            date: "16.06.2026",
            length: "12.4 км",
        },
        RecentFile {
            name: "test_link_campus.sor",
            path: "/var/data/otdr/june",
            date: "3.06.2026",
            length: "2.7 км",
        },
        RecentFile {
            name: "fiber_repair_05.sor",
            path: "C:\\Measurements\\May",
            date: "28.04.2026",
            length: "6.9 км",
        },
        RecentFile {
            name: "ВОЛС_магистрал...",
            path: "D:\\Projects\\OTDR",
            date: "10.06.2026",
            length: "48.1 км",
        },
    ]
}

#[component]
pub fn RecentFiles(panel_open: RwSignal<bool>) -> impl IntoView {
    let (selected, set_selected) = signal(1usize);
    let width = RwSignal::new(MIN_WIDTH);
    let dragging = RwSignal::new(false);

    Effect::new(move |_| {
        let Some(win) = web_sys::window() else { return };

        let on_move = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            if dragging.get_untracked() {
                let w = (e.client_x() as f64 - SIDEBAR_WIDTH).clamp(MIN_WIDTH, MAX_WIDTH);
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

    let item_class = move |idx: usize| {
        if selected.get() == idx {
            stylance::classes!(style::file_item, style::file_item_selected)
        } else {
            style::file_item.to_string()
        }
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
                    <input type="text" placeholder="Поиск по недавним..." />
                </div>

                <div class=style::list>
                    {files()
                        .into_iter()
                        .enumerate()
                        .map(|(idx, f)| {
                            view! {
                                <div class=move || item_class(idx)
                                     on:click=move |_| set_selected.set(idx)>
                                    <img src="public/file.svg" class=style::file_icon alt="file" draggable="false" />
                                    <div class=style::file_info>
                                        <div class=style::file_name>{f.name}</div>
                                        <div class=style::file_path>{f.path}</div>
                                    </div>
                                    <div class=style::file_meta>
                                        <div class=style::file_date>{f.date}</div>
                                        <div class=style::file_length>{f.length}</div>
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>

                <div class=style::footer>
                    <button class=style::clear_btn>
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
