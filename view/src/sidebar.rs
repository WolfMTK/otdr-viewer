use eframe::egui::{self, Ui};

use crate::icons::Icon;
use crate::theme;
use crate::widgets::{self, IconButton};

const WIDTH: f32 = 52.0;
const MARGIN_X: i8 = 10;
const MARGIN_Y: i8 = 10;
const BUTTON_GAP: f32 = 6.0;

pub fn show(ui: &mut Ui, recent_open: &mut bool, sor_info_open: &mut bool) {
    let panel = egui::Panel::left("sidebar")
        .exact_size(WIDTH)
        .resizable(false)
        .show_separator_line(false)
        .frame(
            egui::Frame::new()
                .fill(theme::BG)
                .inner_margin(egui::Margin::symmetric(MARGIN_X, MARGIN_Y)),
        )
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.spacing_mut().item_spacing.y = BUTTON_GAP;
                let recent = IconButton::new(Icon::Folder).active(*recent_open);
                if ui.add(recent).on_hover_text("Последние файлы").clicked() {
                    *recent_open = !*recent_open;
                }
                let info = IconButton::new(Icon::Info);
                if ui.add(info).on_hover_text("Что такое файл .sor?").clicked() {
                    *sor_info_open = true;
                }
            });
        });
    widgets::right_border(ui, panel.response.rect, theme::BORDER);
}
