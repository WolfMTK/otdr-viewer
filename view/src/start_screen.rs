use eframe::egui::{self, RichText, Stroke, StrokeKind, Ui};
use view_model::document::DocumentState;

use crate::theme;

const DROP_OUTLINE_INSET: f32 = 16.0;
const DROP_OUTLINE_RADIUS: u8 = 12;
const HINT_FONT_SIZE: f32 = 15.0;
const GAP: f32 = 12.0;
const CONTENT_HEIGHT: f32 = 120.0;

pub fn show(
    ui: &mut Ui,
    state: &DocumentState,
    files_hovered: bool,
    sor_info_open: &mut bool,
) -> bool {
    if files_hovered {
        ui.painter().rect_stroke(
            ui.max_rect().shrink(DROP_OUTLINE_INSET),
            DROP_OUTLINE_RADIUS,
            Stroke::new(2.0, theme::ACCENT),
            StrokeKind::Inside,
        );
    }

    let mut open_clicked = false;
    ui.vertical_centered(|ui| {
        ui.add_space(((ui.available_height() - CONTENT_HEIGHT) / 2.0).max(0.0));

        if let DocumentState::Loading(path) = state {
            ui.add(egui::Spinner::new());
            ui.add_space(GAP);
            ui.label(format!("Открываю {}", file_name(path)));
            return;
        }

        ui.label(
            RichText::new("Откройте файл .sor или перетащите его в окно").size(HINT_FONT_SIZE),
        );
        ui.add_space(GAP);
        open_clicked = ui.button("Открыть файл").clicked();
        ui.add_space(GAP / 2.0);
        if ui.link("Что такое файл .sor?").clicked() {
            *sor_info_open = true;
        }

        if let DocumentState::Failed { path, message } = state {
            ui.add_space(GAP);
            ui.colored_label(theme::ERROR, format!("{}: {message}", file_name(path)));
        }
    });
    open_clicked
}

fn file_name(path: &std::path::Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}
