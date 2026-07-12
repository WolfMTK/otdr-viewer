use crate::ui::trace_math::level_at;
use leptos::prelude::*;
use shared_types::SorEvent;
use stylance::import_style;

import_style!(style, "index.module.css");

fn event_kind_label(kind: &str, is_first: bool) -> &'static str {
    match kind {
        "reflection" if is_first => "Разъём (старт)",
        "reflection" => "Разъём",
        "loss/drop/gain" => "Сварной стык",
        "end of fiber" => "Конец волокна",
        "multiple events" => "Множественное событие",
        _ => "Неизвестно",
    }
}

fn atten_label(distances_km: &[f64], levels_db: &[f64], prev_distance: Option<f64>, e: &SorEvent) -> String {
    prev_distance
        .filter(|&d_prev| e.distance_km > d_prev)
        .and_then(|d_prev| {
            level_at(distances_km, levels_db, d_prev)
                .zip(level_at(distances_km, levels_db, e.distance_km))
                .map(|(l_prev, l_curr)| (l_prev - l_curr) / (e.distance_km - d_prev))
        })
        .map(|a| format!("{a:.3}"))
        .unwrap_or_else(|| "—".to_string())
}

#[component]
pub fn EventsTable(events: Vec<SorEvent>, distances_km: Vec<f64>, levels_db: Vec<f64>) -> impl IntoView {
    let count = events.len();
    let mut sorted = events;
    sorted.sort_by(|a, b| a.distance_km.partial_cmp(&b.distance_km).unwrap());

    let rows = sorted
        .into_iter()
        .enumerate()
        .scan(None::<f64>, |prev_distance, (i, e)| {
            let refl = if e.refl_db.abs() > f64::EPSILON {
                format!("{:.1}", e.refl_db)
            } else {
                "—".to_string()
            };
            let atten = atten_label(&distances_km, &levels_db, *prev_distance, &e);
            let kind_label = event_kind_label(&e.kind, i == 0);
            *prev_distance = Some(e.distance_km);

            Some(view! {
                <tr class=style::row>
                    <td class=style::cell_num>{e.number}</td>
                    <td class=style::cell_kind>{kind_label}</td>
                    <td class=style::cell_num>{format!("{:.3}", e.distance_km)}</td>
                    <td class=style::cell_num>{format!("{:.2}", e.loss_db)}</td>
                    <td class=style::cell_num>{refl}</td>
                    <td class=style::cell_num>{atten}</td>
                </tr>
            })
        })
        .collect_view();

    view! {
        <div class=style::events_panel>
            <div class=style::header>
                <span class=style::title>"События"</span>
                <span class=style::count>{count.to_string()}</span>
                <div class=style::spacer></div>
                <button class=style::export_link>"Экспорт CSV"</button>
            </div>

            <table class=style::head_table>
                <colgroup>
                    <col class=style::col_num />
                    <col class=style::col_kind />
                    <col class=style::col_num />
                    <col class=style::col_num />
                    <col class=style::col_num />
                    <col class=style::col_num />
                </colgroup>
                <thead>
                    <tr>
                        <th class=style::head_num>"№"</th>
                        <th>"Тип события"</th>
                        <th class=style::head_num>"Расст., км"</th>
                        <th class=style::head_num>"Потери, дБ"</th>
                        <th class=style::head_num>"Отраж., дБ"</th>
                        <th class=style::head_num>"Затух., дБ/км"</th>
                    </tr>
                </thead>
            </table>

            <div class=style::table_scroll>
                <table class=style::table>
                    <colgroup>
                        <col class=style::col_num />
                        <col class=style::col_kind />
                        <col class=style::col_num />
                        <col class=style::col_num />
                        <col class=style::col_num />
                        <col class=style::col_num />
                    </colgroup>
                    <tbody>{rows}</tbody>
                </table>
            </div>
        </div>
    }
}
