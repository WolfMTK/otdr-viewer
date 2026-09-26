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
    let zoom_out = IconButton::new(Icon::ZoomOut).enabled(zoomed);
    if ui.add(zoom_out).on_hover_text("Уменьшить").clicked() {
        chart.zoom_out();
    }
}
