use std::path::{Path, PathBuf};

use eframe::egui::text::{LayoutJob, TextWrapping};
use eframe::egui::{
    self, Align, Color32, CursorIcon, FontId, Layout, Margin, Pos2, Rect, Response, RichText,
    ScrollArea, Sense, Stroke, TextEdit, Ui, pos2, vec2,
};
use view_model::recent_files::{RecentFileRow, RecentFilesViewModel};

use crate::icons::{Icon, icon};
use crate::{theme, widgets};

const DEFAULT_WIDTH: f32 = 290.0;
const MIN_WIDTH: f32 = 250.0;
const MAX_WIDTH: f32 = 500.0;
const TITLE_SIZE: f32 = 14.0;
const SEARCH_ICON_SIZE: f32 = 15.0;
const SEARCH_RADIUS: u8 = 8;
const ROW_HEIGHT: f32 = 52.0;
const ROW_RADIUS: u8 = 8;
const ROW_PADDING: f32 = 10.0;
const TILE_SIZE: f32 = 34.0;
const TILE_RADIUS: u8 = 8;
const TILE_ICON_SIZE: f32 = 18.0;
const TILE_GAP: f32 = 10.0;
const META_WIDTH: f32 = 84.0;
const NAME_SIZE: f32 = 13.0;
const DETAIL_SIZE: f32 = 11.0;
const LINE_GAP: f32 = 3.0;
const FOOTER_HEIGHT: f32 = 40.0;
const CLEAR_ICON_SIZE: f32 = 15.0;

pub fn show(
    ui: &mut Ui,
    recent: &mut RecentFilesViewModel,
    current: Option<&Path>,
) -> Option<PathBuf> {
    let mut chosen = None;
    let panel = egui::Panel::left("recent-files")
        .resizable(true)
        .default_size(DEFAULT_WIDTH)
        .size_range(MIN_WIDTH..=MAX_WIDTH)
        .show_separator_line(false)
        .frame(egui::Frame::new().fill(theme::BG))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);

            egui::Frame::new()
                .inner_margin(Margin {
                    left: 16,
                    right: 16,
                    top: 14,
                    bottom: 8,
                })
                .show(ui, |ui| {
                    let title = RichText::new("Последние файлы")
                        .strong()
                        .size(TITLE_SIZE)
                        .color(theme::TEXT_HEADING);
                    ui.label(title);
                });
            egui::Frame::new()
                .inner_margin(Margin {
                    left: 12,
                    right: 12,
                    top: 0,
                    bottom: 8,
                })
                .show(ui, |ui| search_box(ui, &mut recent.query));

            footer(ui, recent);

            let list = ui.available_rect_before_wrap();
            widgets::top_border(ui, list, theme::BORDER);
            egui::Frame::new()
                .inner_margin(Margin::symmetric(8, 4))
                .show(ui, |ui| {
                    let rows = recent.rows();
                    if rows.is_empty() {
                        let hint = if recent.is_empty() {
                            "Пока нет открытых файлов"
                        } else {
                            "Ничего не найдено"
                        };
                        ui.add_space(16.0);
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new(hint).size(12.0).color(theme::TEXT_FAINT));
                        });
                        return;
                    }
                    ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                        for row in &rows {
                            let selected = current == Some(row.path.as_path());
                            if entry(ui, row, selected).clicked() {
                                chosen = Some(row.path.clone());
                            }
                        }
                    });
                });
        });
    widgets::right_border(ui, panel.response.rect, theme::BORDER);
    chosen
}

fn search_box(ui: &mut Ui, query: &mut String) {
    egui::Frame::new()
        .fill(theme::BG_SURFACE)
        .stroke(Stroke::new(1.0, theme::BORDER))
        .corner_radius(SEARCH_RADIUS)
        .inner_margin(Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                ui.add(icon(Icon::Search, theme::TEXT_FAINT, SEARCH_ICON_SIZE));
                ui.add(
                    TextEdit::singleline(query)
                        .hint_text("Поиск по недавним...")
                        .frame(egui::Frame::NONE)
                        .margin(Margin::ZERO)
                        .desired_width(f32::INFINITY),
                );
            });
        });
}

fn footer(ui: &mut Ui, recent: &mut RecentFilesViewModel) {
    let panel = egui::Panel::bottom("recent-files-footer")
        .exact_size(FOOTER_HEIGHT)
        .resizable(false)
        .show_separator_line(false)
        .frame(egui::Frame::new().inner_margin(Margin::symmetric(12, 0)))
        .show(ui, |ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if clear_button(ui, !recent.is_empty()).clicked() {
                    recent.clear();
                }
            });
        });
    widgets::top_border(ui, panel.response.rect, theme::BORDER_SOFT);
}

fn clear_button(ui: &mut Ui, enabled: bool) -> Response {
    let text = RichText::new("Очистить список").size(12.0);
    let trash = icon(Icon::Trash, theme::TEXT_FAINT, CLEAR_ICON_SIZE);
    let button = egui::Button::image_and_text(trash, text).frame(false);
    ui.add_enabled(enabled, button)
}

fn entry(ui: &mut Ui, row: &RecentFileRow, selected: bool) -> Response {
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), ROW_HEIGHT), Sense::click());
    let hovered = response.contains_pointer();

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        if selected {
            painter.rect_filled(rect, ROW_RADIUS, theme::ACCENT_SOFT);
        } else if hovered {
            painter.rect_filled(rect, ROW_RADIUS, theme::HOVER_FILL);
        }

        let inner = rect.shrink(ROW_PADDING);
        let tile = Rect::from_center_size(
            pos2(inner.left() + TILE_SIZE / 2.0, rect.center().y),
            vec2(TILE_SIZE, TILE_SIZE),
        );
        painter.rect_filled(tile, TILE_RADIUS, theme::ACCENT_SOFT);
        let glyph = Rect::from_center_size(tile.center(), vec2(TILE_ICON_SIZE, TILE_ICON_SIZE));
        icon(Icon::File, theme::ACCENT, TILE_ICON_SIZE).paint_at(ui, glyph);

        let text_left = tile.right() + TILE_GAP;
        let text_width = (inner.right() - META_WIDTH - text_left).max(0.0);
        let top = rect.center().y - (NAME_SIZE + DETAIL_SIZE + LINE_GAP) / 2.0;
        let bottom = top + NAME_SIZE + LINE_GAP;

        let name = Line::new(&row.name, NAME_SIZE, theme::TEXT_HEADING);
        let location = Line::new(&row.location, DETAIL_SIZE, theme::TEXT_FAINT);
        name.paint(ui, pos2(text_left, top), text_width);
        location.paint(ui, pos2(text_left, bottom), text_width);

        let right = inner.right();
        right_text(ui, pos2(right, top + 1.0), &row.opened_at);
        if let Some(length) = &row.length {
            right_text(ui, pos2(right, bottom), length);
        }
    }

    if hovered {
        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
    }
    response.on_hover_text(row.path.display().to_string())
}

struct Line<'a> {
    text: &'a str,
    size: f32,
    color: Color32,
}

impl<'a> Line<'a> {
    fn new(text: &'a str, size: f32, color: Color32) -> Self {
        Self { text, size, color }
    }

    fn paint(&self, ui: &Ui, pos: Pos2, max_width: f32) {
        let font = FontId::proportional(self.size);
        let mut job = LayoutJob::simple_singleline(self.text.to_string(), font, self.color);
        job.wrap = TextWrapping::truncate_at_width(max_width);
        let galley = ui.painter().layout_job(job);
        ui.painter().galley(pos, galley, self.color);
    }
}

fn right_text(ui: &Ui, top_right: Pos2, text: &str) {
    ui.painter().text(
        top_right,
        egui::Align2::RIGHT_TOP,
        text,
        FontId::proportional(DETAIL_SIZE),
        theme::TEXT_FAINT,
    );
}
