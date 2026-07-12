use leptos::prelude::*;
use stylance::import_style;

use crate::ui::component::helpers::toggle_class;

import_style!(style, "index.module.css");

#[component]
pub fn Toolbar() -> impl IntoView {
    let grid_on = RwSignal::new(true);
    let markers_on = RwSignal::new(false);
    let legend_on = RwSignal::new(false);
    let route_scheme_on = RwSignal::new(false);
    let loupe_on = RwSignal::new(false);
    let bidirectional_on = RwSignal::new(false);
    let wavelength_nm = RwSignal::new(1550u16);

    let toggle_btn_class = move |on: bool| toggle_class(style::icon_button, style::icon_button_active, on);
    let wave_btn_class =
        move |nm: u16| toggle_class(style::wave_button, style::wave_button_active, wavelength_nm.get() == nm);

    view! {
        <div class=style::toolbar>
            <div class=style::group>
                <button class=style::icon_button title="Весь масштаб">
                    <img src="public/toolbar-fit-view.svg" alt="fit view" draggable="false" />
        </button>
                <button class=style::icon_button title="Увеличение">
                    <img src="public/toolbar-zoom-in.svg" alt="zoom in" draggable="false" />
                </button>
                <button class=style::icon_button title="Уменьшение">
                    <img src="public/toolbar-zoom-out.svg" alt="zoom out" draggable="false" />
                </button>
            </div>

            <div class=style::divider></div>

            <div class=style::group>
                <button
                    class=move || toggle_btn_class(grid_on.get())
                    title="Сетка"
                    on:click=move |_| grid_on.update(|v| *v = !*v)
                >
                    <img src="public/toolbar-grid.svg" alt="grid" draggable="false" />
                </button>
                <button
                    class=move || toggle_btn_class(markers_on.get())
                    title="Маркеры A/B"
                    on:click=move |_| markers_on.update(|v| *v = !*v)
                >
                    <img src="public/toolbar-markers.svg" alt="markers a/b" draggable="false" />
                </button>
            </div>

            <div class=style::divider></div>

            <div class=style::group>
                <button
                    class=move || toggle_btn_class(legend_on.get())
                    title="Легенда"
                    on:click=move |_| legend_on.update(|v| *v = !*v)
                >
                    <img src="public/toolbar-legend.svg" alt="legend" draggable="false" />
                </button>
                <button
                    class=move || toggle_btn_class(route_scheme_on.get())
                    title="Схема трассы"
                    on:click=move |_| route_scheme_on.update(|v| *v = !*v)
                >
                    <img src="public/toolbar-route.svg" alt="route scheme" draggable="false" />
                </button>
            </div>

            <div class=style::divider></div>

            <div class=style::group>
                <button
                    class=move || toggle_btn_class(loupe_on.get())
                    title="Лупа"
                    on:click=move |_| loupe_on.update(|v| *v = !*v)
                >
                    <img src="public/search.svg" alt="loupe" draggable="false" />
                </button>
                <button
                    class=move || toggle_btn_class(bidirectional_on.get())
                    title="Двунаправленный анализ"
                    on:click=move |_| bidirectional_on.update(|v| *v = !*v)
                >
                    <img src="public/toolbar-bidirectional.svg" alt="bidirectional analysis" draggable="false" />
                </button>
                <button class=style::icon_button title="Пакетная обработка">
                    <img src="public/stack.svg" alt="batch processing" draggable="false" />
                </button>
            </div>

            <div class=style::spacer></div>

            <div class=style::right_group>
                <div class=style::wave_group>
                    <button class=move || wave_btn_class(1550) on:click=move |_| wavelength_nm.set(1550)>
                        "1550 нм"
                    </button>
                    <button class=move || wave_btn_class(1310) on:click=move |_| wavelength_nm.set(1310)>
                        "1310 нм"
                    </button>
                </div>

                <button class=style::report_button>
                    <img src="public/toolbar-report.svg" alt="report" draggable="false" />
                    "Отчёт"
                </button>
            </div>
        </div>
    }
}
