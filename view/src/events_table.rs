use eframe::egui::{self, Align, Layout, RichText, Ui};
use egui_extras::{Column, TableBuilder};
use view_model::events::EventRow;

use crate::{theme, widgets};

const DEFAULT_HEIGHT: f32 = 220.0;
const MIN_HEIGHT: f32 = 120.0;
const MAX_HEIGHT: f32 = 520.0;
const MARGIN_X: i8 = 12;
const MARGIN_Y: i8 = 8;
const TITLE_SIZE: f32 = 14.0;
const HEADER_HEIGHT: f32 = 24.0;
const ROW_HEIGHT: f32 = 22.0;
const TITLES: [&str; 6] = [
    "№",
    "Тип события",
    "Расст., км",
    "Потери, дБ",
    "Отраж., дБ",
    "Затух., дБ/км",
];

pub fn show(ui: &mut Ui, rows: &[EventRow]) {
    let panel = egui::Panel::bottom("events")
        .resizable(true)
        .default_size(DEFAULT_HEIGHT)
        .size_range(MIN_HEIGHT..=MAX_HEIGHT)
        .show_separator_line(false)
        .frame(
            egui::Frame::new()
                .fill(theme::BG_SURFACE)
                .inner_margin(egui::Margin::symmetric(MARGIN_X, MARGIN_Y)),
        )
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("События").strong().size(TITLE_SIZE));
                ui.label(RichText::new(rows.len().to_string()).color(theme::TEXT_MUTED));
            });
            ui.add_space(6.0);
            if rows.is_empty() {
                ui.label(RichText::new("В файле нет событий").color(theme::TEXT_MUTED));
                return;
            }
            table(ui, rows);
        });
    widgets::top_border(ui, panel.response.rect, theme::BORDER);
}

fn table(ui: &mut Ui, rows: &[EventRow]) {
    TableBuilder::new(ui)
        .striped(true)
        .auto_shrink(false)
        .cell_layout(Layout::left_to_right(Align::Center))
        .column(Column::exact(40.0))
        .column(Column::initial(190.0).at_least(120.0).clip(true))
        .columns(Column::initial(110.0), 3)
        .column(Column::remainder())
        .header(HEADER_HEIGHT, |mut header| {
            for title in TITLES {
                header.col(|ui| {
                    ui.strong(title);
                });
            }
        })
        .body(|body| {
            body.rows(ROW_HEIGHT, rows.len(), |mut row| {
                let event = &rows[row.index()];
                let cells = [
                    event.number.to_string(),
                    event.kind.to_string(),
                    event.distance.clone(),
                    event.loss.clone(),
                    event.reflection.clone(),
                    event.attenuation.clone(),
                ];
                for cell in cells {
                    row.col(|ui| {
                        ui.label(cell);
                    });
                }
            });
        });
}
