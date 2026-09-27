use eframe::egui::{self, Align, Id, Layout, Modal, RichText};

use crate::theme;

const WIDTH: f32 = 520.0;
const TITLE_SIZE: f32 = 18.0;
const NOTE_SIZE: f32 = 11.0;
const GAP: f32 = 12.0;

struct Section {
    title: &'static str,
    text: &'static str,
}

const SECTIONS: [Section; 3] = [
    Section {
        title: "Стандартный формат",
        text: "SOR — Standard OTDR Record, формат Telcordia SR-4731 (ранее Bellcore GR-196). \
               Это отраслевой стандарт обмена данными между рефлектометрами разных \
               производителей: один файл соответствует одному измерению одного волокна.",
    },
    Section {
        title: "Структура и содержимое",
        text: "Файл состоит из блоков: общие параметры измерения, данные о приборе, длина \
               волны и длительность импульса, показатель преломления волокна (IOR) и \
               коэффициент обратного рассеяния, сама кривая мощности сигнала по расстоянию, \
               а также таблица обнаруженных событий — сварных стыков, разъёмов, отражений \
               и обрывов.",
    },
    Section {
        title: "Назначение",
        text: "Метод основан на регистрации рэлеевского обратного рассеяния и френелевских \
               отражений зондирующего импульса во времени. По этим данным файл .sor \
               позволяет оценить затухание, потери на стыках и разъёмах, а также \
               локализовать обрывы и неоднородности вдоль всей волоконно-оптической линии.",
    },
];

pub fn show(ctx: &egui::Context, open: &mut bool) {
    if !*open {
        return;
    }
    let modal = Modal::new(Id::new("sor-info")).show(ctx, |ui| {
        ui.set_width(WIDTH);
        ui.label(
            RichText::new("Что такое файл .sor?")
                .strong()
                .size(TITLE_SIZE),
        );
        ui.label(
            RichText::new(
                "Файл с результатом измерения оптического волокна рефлектометром (OTDR).",
            )
            .color(theme::TEXT_MUTED),
        );
        ui.add_space(GAP);

        for section in &SECTIONS {
            ui.label(RichText::new(section.title).strong());
            ui.label(section.text);
            ui.add_space(GAP);
        }

        ui.separator();
        ui.label(
            RichText::new("Расширение: .sor · Telcordia SR-4731 · EXFO, VIAVI, Yokogawa и др.")
                .size(NOTE_SIZE)
                .color(theme::TEXT_MUTED),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| ui.button("Понятно").clicked())
            .inner
    });
    if modal.inner || modal.should_close() {
        *open = false;
    }
}
