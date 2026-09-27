use eframe::egui::{self, Align, Layout, Ui};

use crate::icons::Icon;
use crate::theme;
use crate::widgets::{self, IconButton, IconStyle};

const WIDTH: f32 = 48.0;
const MARGIN_Y: i8 = 10;
const BUTTON_SIZE: f32 = 36.0;
const ICON_SIZE: f32 = 20.0;
const BUTTON_GAP: f32 = 4.0;
const STYLE: IconStyle = IconStyle {
    normal: theme::TEXT_FAINT,
    hover: theme::TEXT_MUTED,
    active: theme::ACCENT,
};

pub fn show(ui: &mut Ui, recent_open: &mut bool, sor_info_open: &mut bool, file_opened: bool) {
    let panel = egui::Panel::left("sidebar")
        .exact_size(WIDTH)
        .resizable(false)
        .show_separator_line(false)
        .frame(
            egui::Frame::new()
                .fill(theme::BG)
                .inner_margin(egui::Margin::symmetric(0, MARGIN_Y)),
        )
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = BUTTON_GAP;
            ui.vertical_centered(|ui| {
                if button(ui, Icon::Folder, *recent_open, "Последние файлы") {
                    *recent_open = true;
                }
                if file_opened && button(ui, Icon::Info, false, "Что такое файл .sor?")
                {
                    *sor_info_open = true;
                }
            });
            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                let hint = if *recent_open {
                    "Свернуть панель"
                } else {
                    "Развернуть панель"
                };
                if button(ui, Icon::Collapse, false, hint) {
                    *recent_open = !*recent_open;
                }
            });
        });
    widgets::right_border(ui, panel.response.rect, theme::BORDER);
}

fn button(ui: &mut Ui, glyph: Icon, active: bool, hint: &str) -> bool {
    let button = IconButton::new(glyph)
        .style(STYLE)
        .size(BUTTON_SIZE, ICON_SIZE)
        .active(active);
    ui.add(button).on_hover_text(hint).clicked()
}
