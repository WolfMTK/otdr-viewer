use eframe::egui::{self, Align, Layout, RichText, Ui};

use crate::{theme, widgets};

const HEIGHT: f32 = 26.0;
const MARGIN_X: i8 = 12;
const TEXT_SIZE: f32 = 12.0;
const APP_NAME: &str = concat!("OTDR Viewer ", env!("CARGO_PKG_VERSION"));

pub fn show(ui: &mut Ui, status: &str) {
    let panel = egui::Panel::bottom("status-bar")
        .exact_size(HEIGHT)
        .resizable(false)
        .show_separator_line(false)
        .frame(
            egui::Frame::new()
                .fill(theme::BG)
                .inner_margin(egui::Margin::symmetric(MARGIN_X, 0)),
        )
        .show(ui, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(text(status));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(text(APP_NAME));
                });
            });
        });
    widgets::top_border(ui, panel.response.rect, theme::BORDER);
}

fn text(value: &str) -> RichText {
    RichText::new(value)
        .size(TEXT_SIZE)
        .color(theme::TEXT_FAINT)
}
