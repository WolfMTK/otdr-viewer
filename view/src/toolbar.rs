use eframe::egui::{self, Ui};
use view_model::chart_view::{ChartSettings, ChartView};

use crate::icons::Icon;
use crate::theme;
use crate::widgets::{self, IconButton};

const HEIGHT: f32 = 44.0;
const MARGIN_X: i8 = 10;
const MARGIN_Y: i8 = 6;
const BUTTON_GAP: f32 = 4.0;

pub fn show(ui: &mut Ui, chart: &mut ChartView, settings: &mut ChartSettings) {
    let panel = egui::Panel::top("toolbar")
        .exact_size(HEIGHT)
        .show_separator_line(false)
        .frame(
            egui::Frame::new()
                .fill(theme::BG_TOOLBAR)
                .inner_margin(egui::Margin::symmetric(MARGIN_X, MARGIN_Y)),
        )
        .show(ui, |ui| {
            ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing.x = BUTTON_GAP;
                zoom_buttons(ui, chart);
                ui.separator();
                let grid = IconButton::new(Icon::Grid).active(settings.grid_visible);
                if ui.add(grid).on_hover_text("Сетка").clicked() {
                    settings.grid_visible = !settings.grid_visible;
                }
                let events = IconButton::new(Icon::Events).active(settings.events_visible);
                if ui.add(events).on_hover_text("Метки событий").clicked() {
                    settings.events_visible = !settings.events_visible;
                }
                let ideal = IconButton::new(Icon::Steps).active(settings.ideal_visible);
                if ui.add(ideal).on_hover_text("Линии аппроксимации").clicked() {
                    settings.ideal_visible = !settings.ideal_visible;
                }
                let auto = IconButton::new(Icon::AutoScale).active(settings.auto_levels);
                let hint = "Автомасштаб по вертикали при увеличении";
                if ui.add(auto).on_hover_text(hint).clicked() {
                    settings.auto_levels = !settings.auto_levels;
                    if !settings.auto_levels {
                        chart.reset_levels();
                    }
                }
                let markers = IconButton::new(Icon::Markers).active(settings.markers_visible);
                if ui.add(markers).on_hover_text("Маркеры A/B").clicked() {
                    settings.markers_visible = !settings.markers_visible;
                }
            });
        });
    widgets::bottom_border(ui, panel.response.rect, theme::BORDER);
}

fn zoom_buttons(ui: &mut Ui, chart: &mut ChartView) {
    let zoomed = !chart.is_fitted();

    let fit = IconButton::new(Icon::FitView).enabled(zoomed);
    if ui.add(fit).on_hover_text("Весь масштаб").clicked() {
        chart.fit();
    }
    if ui
        .add(IconButton::new(Icon::ZoomIn))
        .on_hover_text("Увеличить")
        .clicked()
    {
        chart.zoom_in();
    }
    let zoom_out = IconButton::new(Icon::ZoomOut);
    if ui.add(zoom_out).on_hover_text("Уменьшить").clicked() {
        chart.zoom_out();
    }
}
