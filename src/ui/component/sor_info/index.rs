use leptos::prelude::*;
use stylance::import_style;

use crate::ui::component::helpers::close_on_escape;

import_style!(style, "index.module.css");

struct InfoSection {
    icon: &'static str,
    title: &'static str,
    text: &'static str,
}

const SECTIONS: [InfoSection; 3] = [
    InfoSection {
        icon: "public/file.svg",
        title: "Стандартный формат",
        text: "SOR — Standard OTDR Record, формат Telcordia SR-4731 (ранее Bellcore GR-196). \
               Это отраслевой стандарт обмена данными между рефлектометрами разных производителей: \
               один файл соответствует одному измерению одного волокна.",
    },
    InfoSection {
        icon: "public/stack.svg",
        title: "Структура и содержимое",
        text: "Файл состоит из блоков: общие параметры измерения, данные о приборе, длина волны \
               и длительность импульса, показатель преломления волокна (IOR) и коэффициент обратного \
               рассеяния, сама кривая мощности сигнала по расстоянию, а также таблица обнаруженных \
               событий — сварных стыков, разъёмов, отражений и обрывов.",
    },
    InfoSection {
        icon: "public/search.svg",
        title: "Назначение",
        text: "Метод основан на регистрации рэлеевского обратного рассеяния и френелевских отражений \
               зондирующего импульса во времени. По этим данным .sor-файл позволяет оценить затухание, \
               потери на стыках и разъёмах, а также локализовать обрывы и неоднородности вдоль всей \
               волоконно-оптической линии.",
    },
];

fn render_section(section: &InfoSection) -> impl IntoView {
    view! {
        <div class=style::section>
            <div class=style::section_icon>
                <img src=section.icon alt="" draggable="false" />
            </div>
            <div class=style::section_body>
                <h3>{section.title}</h3>
                <p>{section.text}</p>
            </div>
        </div>
    }
}

#[component]
pub fn SorInfoDialog(open: RwSignal<bool>) -> impl IntoView {
    let close = move || open.set(false);
    close_on_escape(open);

    view! {
        <Show when=move || open.get()>
            <div class=style::overlay>
                <div class=style::dialog>
                    <div class=style::header>
                        <div class=style::icon_wrap>
                            <img src="public/info.svg" alt="" draggable="false" />
                        </div>
                        <button class=style::close_btn on:click=move |_| close()>"×"</button>
                    </div>

                    <div class=style::intro>
                        <h2 class=style::title>"Что такое файл .sor?"</h2>
                        <p class=style::subtitle>
                            "Файл с результатом измерения оптического волокна рефлектометром (OTDR)."
                        </p>
                    </div>

                    <div class=style::sections>
                        {SECTIONS.iter().map(render_section).collect_view()}
                    </div>

                    <div class=style::footer>
                        <span class=style::footer_note>
                            "Расширение: .sor · Telcordia SR-4731 · совместимо с оборудованием EXFO, VIAVI, Yokogawa"
                        </span>
                        <button class=style::confirm_btn on:click=move |_| close()>"Понятно"</button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
