use std::path::PathBuf;

use eframe::egui::{
    self, Align, Align2, FontId, Layout, Response, RichText, ScrollArea, Sense, TextEdit, Ui, pos2,
    vec2,
};
use view_model::recent_files::{RecentFileRow, RecentFilesViewModel};

use crate::{theme, widgets};

const DEFAULT_WIDTH: f32 = 280.0;
const MIN_WIDTH: f32 = 220.0;
const MAX_WIDTH: f32 = 480.0;
const MARGIN: i8 = 12;
const TITLE_SIZE: f32 = 14.0;
const GAP: f32 = 8.0;
const ROW_HEIGHT: f32 = 44.0;
const ROW_PADDING_X: f32 = 8.0;
const ROW_PADDING_Y: f32 = 6.0;
const ROW_RADIUS: u8 = 6;
const META_WIDTH: f32 = 80.0;
const NAME_SIZE: f32 = 13.0;
const DETAIL_SIZE: f32 = 11.0;

pub fn show(ui: &mut Ui, recent: &mut RecentFilesViewModel) -> Option<PathBuf> {
    let mut chosen = None;
    let panel = egui::Panel::left("recent-files")
        .resizable(true)
        .default_size(DEFAULT_WIDTH)
        .size_range(MIN_WIDTH..=MAX_WIDTH)
        .show_separator_line(false)
        .frame(
            egui::Frame::new()
                .fill(theme::BG_TOOLBAR)
                .inner_margin(egui::Margin::same(MARGIN)),
        )
        .show(ui, |ui| {
            header(ui, recent);
            ui.add_space(GAP);
            ui.add(
                TextEdit::singleline(&mut recent.query)
                    .hint_text("Поиск по недавним...")
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(GAP);

            let rows = recent.rows();
            if rows.is_empty() {
                let hint = if recent.is_empty() {
                    "Здесь появятся открытые файлы"
                } else {
                    "Ничего не найдено"
                };
                ui.label(RichText::new(hint).color(theme::TEXT_MUTED));
                return;
            }
            ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                for row in &rows {
                    if entry(ui, row).clicked() {
                        chosen = Some(row.path.clone());
                    }
                }
            });
        });
    widgets::right_border(ui, panel.response.rect, theme::BORDER);
    chosen
}

fn header(ui: &mut Ui, recent: &mut RecentFilesViewModel) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Последние файлы").strong().size(TITLE_SIZE));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let clear = egui::Button::new("Очистить").frame(false);
            if ui.add_enabled(!recent.is_empty(), clear).clicked() {
                recent.clear();
            }
        });
    });
}

fn entry(ui: &mut Ui, row: &RecentFileRow) -> Response {
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), ROW_HEIGHT), Sense::click());

    if ui.is_rect_visible(rect) {
        if response.contains_pointer() {
            ui.painter()
                .rect_filled(rect, ROW_RADIUS, theme::HOVER_FILL);
        }
        let inner = rect.shrink2(vec2(ROW_PADDING_X, ROW_PADDING_Y));
        let text_area = inner.with_max_x(inner.right() - META_WIDTH);
        let text_painter = ui
            .painter()
            .with_clip_rect(text_area.intersect(ui.clip_rect()));
        let painter = ui.painter();

        text_painter.text(
            inner.left_top(),
            Align2::LEFT_TOP,
            &row.name,
            FontId::proportional(NAME_SIZE),
            theme::TEXT,
        );
        text_painter.text(
            inner.left_bottom(),
            Align2::LEFT_BOTTOM,
            &row.location,
            FontId::proportional(DETAIL_SIZE),
            theme::TEXT_MUTED,
        );
        painter.text(
            inner.right_top(),
            Align2::RIGHT_TOP,
            &row.opened_at,
            FontId::proportional(DETAIL_SIZE),
            theme::TEXT_MUTED,
        );
        if let Some(length) = &row.length {
            painter.text(
                pos2(inner.right(), inner.bottom()),
                Align2::RIGHT_BOTTOM,
                length,
                FontId::proportional(DETAIL_SIZE),
                theme::TEXT_MUTED,
            );
        }
    }

    if response.contains_pointer() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    response.on_hover_text(row.path.display().to_string())
}
