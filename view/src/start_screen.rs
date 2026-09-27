use eframe::egui::{
    self, Align, Layout, Margin, Rect, RichText, Sense, Shadow, Shape, Stroke, Ui, pos2, vec2,
};
use view_model::document::DocumentState;

use crate::icons::{Icon, icon};
use crate::{theme, widgets};

const CARD_WIDTH: f32 = 420.0;
const CARD_HEIGHT: f32 = 372.0;
const CARD_RADIUS: u8 = 16;
const CARD_PADDING_X: i8 = 48;
const CARD_PADDING_TOP: i8 = 44;
const CARD_PADDING_BOTTOM: i8 = 24;
const CARD_SHADOW: Shadow = Shadow {
    offset: [0, 8],
    blur: 30,
    spread: 0,
    color: theme::SHADOW,
};
const TILE_SIZE: f32 = 68.0;
const TILE_RADIUS: u8 = 16;
const TILE_ICON_SIZE: f32 = 30.0;
const TITLE_SIZE: f32 = 19.0;
const SUBTITLE_SIZE: f32 = 14.0;
const FOOTER_SIZE: f32 = 12.0;
const DASH: f32 = 3.0;
const DASH_GAP: f32 = 2.0;

pub fn show(ui: &mut Ui, state: &DocumentState, sor_info_open: &mut bool) -> bool {
    let outer_width = CARD_WIDTH + 2.0 * f32::from(CARD_PADDING_X);
    ui.add_space(((ui.available_height() - CARD_HEIGHT) / 2.0).max(0.0));

    let mut open_clicked = false;
    ui.horizontal(|ui| {
        ui.add_space(((ui.available_width() - outer_width) / 2.0).max(0.0));
        egui::Frame::new()
            .fill(theme::BG_SURFACE)
            .corner_radius(CARD_RADIUS)
            .shadow(CARD_SHADOW)
            .inner_margin(Margin {
                left: CARD_PADDING_X,
                right: CARD_PADDING_X,
                top: CARD_PADDING_TOP,
                bottom: CARD_PADDING_BOTTOM,
            })
            .show(ui, |ui| {
                ui.set_width(CARD_WIDTH);
                open_clicked = content(ui, state, sor_info_open);
            });
    });
    open_clicked
}

fn content(ui: &mut Ui, state: &DocumentState, sor_info_open: &mut bool) -> bool {
    let mut open_clicked = false;
    ui.vertical_centered(|ui| {
        upload_tile(ui);
        ui.add_space(20.0);
        ui.label(
            RichText::new("Перетащите файл .sor сюда")
                .size(TITLE_SIZE)
                .strong()
                .color(theme::TEXT_HEADING),
        );
        ui.add_space(8.0);
        ui.label(
            RichText::new(
                "Или выберите его через панель недавних\nфайлов, либо нажмите кнопку ниже.",
            )
            .size(SUBTITLE_SIZE)
            .color(theme::TEXT_FAINT),
        );
        ui.add_space(24.0);

        match state {
            DocumentState::Loading(_) => {
                ui.add(egui::Spinner::new().size(24.0));
            }
            DocumentState::Failed { message, .. } => {
                ui.colored_label(theme::ERROR, message);
                ui.add_space(12.0);
                open_clicked = open_button(ui);
            }
            _ => open_clicked = open_button(ui),
        }

        ui.add_space(24.0);
        ui.separator();
        ui.add_space(8.0);
        footer(ui, sor_info_open);
    });
    open_clicked
}

fn upload_tile(ui: &mut Ui) {
    let (rect, _) = ui.allocate_exact_size(vec2(TILE_SIZE, TILE_SIZE), Sense::hover());
    ui.painter()
        .rect_filled(rect, TILE_RADIUS, theme::ACCENT_SOFT);
    let glyph = Rect::from_center_size(rect.center(), vec2(TILE_ICON_SIZE, TILE_ICON_SIZE));
    icon(Icon::Upload, theme::ACCENT, TILE_ICON_SIZE).paint_at(ui, glyph);
}

fn open_button(ui: &mut Ui) -> bool {
    widgets::primary_button(ui, Some(Icon::FolderOpen), "Открыть файл...").clicked()
}

fn footer(ui: &mut Ui, sor_info_open: &mut bool) {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new("Telcordia SR-4731 · .sor")
                .size(FOOTER_SIZE)
                .color(theme::TEXT_FAINT),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if help_link(ui) {
                *sor_info_open = true;
            }
        });
    });
}

fn help_link(ui: &mut Ui) -> bool {
    let text = RichText::new("Что такое .sor?")
        .size(FOOTER_SIZE)
        .color(theme::ACCENT);
    let response = ui.add(egui::Label::new(text).sense(Sense::click()));
    let rect = response.rect;
    let y = rect.bottom();
    ui.painter().extend(Shape::dashed_line(
        &[pos2(rect.left(), y), pos2(rect.right(), y)],
        Stroke::new(1.0, theme::ACCENT),
        DASH,
        DASH_GAP,
    ));
    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    response.clicked()
}
