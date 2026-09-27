use eframe::egui::{self, Grid, RichText, ScrollArea, Ui, vec2};
use view_model::params::Params;

use crate::{theme, widgets};

const DEFAULT_WIDTH: f32 = 280.0;
const MIN_WIDTH: f32 = 220.0;
const MAX_WIDTH: f32 = 420.0;
const MARGIN: i8 = 14;
const TITLE_SIZE: f32 = 14.0;
const SECTION_TITLE_SIZE: f32 = 11.0;
const SECTION_GAP: f32 = 14.0;
const GRID_SPACING: f32 = 6.0;
const COLUMN_GAP: f32 = 12.0;

pub fn show(ui: &mut Ui, params: &Params) {
    let panel = egui::Panel::right("params")
        .resizable(true)
        .default_size(DEFAULT_WIDTH)
        .size_range(MIN_WIDTH..=MAX_WIDTH)
        .show_separator_line(false)
        .frame(
            egui::Frame::new()
                .fill(theme::BG_SURFACE)
                .inner_margin(egui::Margin::same(MARGIN)),
        )
        .show(ui, |ui| {
            ui.label(
                RichText::new("Параметры измерения")
                    .strong()
                    .size(TITLE_SIZE),
            );
            let subtitle = format!("{} · {}", params.title, params.subtitle);
            ui.label(RichText::new(subtitle).color(theme::TEXT_MUTED));
            ui.add_space(SECTION_GAP);

            ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                for (index, section) in params.sections.iter().enumerate() {
                    let title = RichText::new(section.title)
                        .size(SECTION_TITLE_SIZE)
                        .strong()
                        .color(theme::TEXT_MUTED);
                    ui.label(title);
                    ui.add_space(4.0);
                    Grid::new(("params", index))
                        .num_columns(2)
                        .spacing(vec2(COLUMN_GAP, GRID_SPACING))
                        .show(ui, |ui| {
                            for (label, value) in &section.rows {
                                ui.label(RichText::new(*label).color(theme::TEXT_MUTED));
                                ui.label(value);
                                ui.end_row();
                            }
                        });
                    ui.add_space(SECTION_GAP);
                }
            });
        });
    widgets::left_border(ui, panel.response.rect, theme::BORDER);
}
