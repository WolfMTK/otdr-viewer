use leptos::prelude::*;
use stylance::import_style;

use crate::ui::component::chart::index::Chart;
use crate::ui::component::dropzone::index::Dropzone;
use crate::ui::component::events_table::index::EventsTable;
use crate::ui::component::params_panel::index::ParamsPanel;
use crate::ui::context::{CompareMode, OpenedSor};

import_style!(style, "home.module.css");

#[component]
pub fn Home() -> impl IntoView {
    let OpenedSor(opened) = use_context::<OpenedSor>().expect("OpenedSor is provided at app root");
    let CompareMode(compare_mode) = use_context::<CompareMode>().expect("CompareMode is provided at app root");

    view! {
        <Show when=move || opened.get().is_none()>
            <Dropzone/>
        </Show>

        <Show when=move || opened.get().is_some()>
            {move || {
                opened.get().map(|data| {
                    let events = data.summary.events.clone();
                    let distances_km = data.trace.distances_km.clone();
                    let levels_db = data.trace.levels_db.clone();

                    if compare_mode.get() {
                        view! {
                            <div class=style::viewer>
                                <div class=style::viewer_main>
                                    <EventsTable events=events distances_km=distances_km levels_db=levels_db/>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class=style::viewer>
                                <div class=style::viewer_main>
                                    <Chart
                                        distances_km=distances_km.clone()
                                        levels_db=levels_db.clone()
                                        events=events.clone()
                                    />
                                    <EventsTable events=events distances_km=distances_km levels_db=levels_db/>
                                </div>

                                <ParamsPanel summary=data.summary/>
                            </div>
                        }.into_any()
                    }
                })
            }}
        </Show>
    }
}