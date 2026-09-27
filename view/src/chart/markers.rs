use eframe::egui::{
    Align2, Color32, CursorIcon, FontId, Grid, Painter, Rect, Response, Sense, Stroke, Ui, pos2,
};
use model::types::SorTrace;
use view_model::chart_view::DistanceRange;
use view_model::markers::{Marker, Markers};

use crate::chart::Mapping;
use crate::theme;

const BADGE_WIDTH: f32 = 20.0;
const BADGE_HEIGHT: f32 = 16.0;
const BADGE_GAP: f32 = 3.0;
const BADGE_RADIUS: u8 = 4;
const BADGE_FONT_SIZE: f32 = 11.0;
const BAND_OPACITY: f32 = 0.08;
const MARKERS: [(Marker, &str); 2] = [(Marker::A, "A"), (Marker::B, "B")];

pub(super) fn handle_drag(ui: &Ui, mapping: &Mapping, markers: &mut Markers, full: DistanceRange) {
    for (marker, label) in MARKERS {
        let x = mapping.x(markers.get(marker));
        if !mapping.plot.x_range().contains(x) {
            continue;
        }
        let response = ui.interact(badge_rect(x, mapping.plot), ui.id().with(label), Sense::drag());
        if response.hovered() || response.dragged() {
            ui.ctx().set_cursor_icon(CursorIcon::ResizeHorizontal);
        }
        if response.dragged()
            && let Some(pointer) = response.interact_pointer_pos()
        {
            let km = mapping.km(pointer.x).clamp(full.start(), full.end());
            markers.set(marker, km);
        }
    }
}

pub(super) fn draw(painter: &Painter, mapping: &Mapping, markers: &Markers) {
    let plot = mapping.plot;
    let (xa, xb) = (mapping.x(markers.a_km), mapping.x(markers.b_km));
    let clipped = painter.with_clip_rect(plot);
    let band = Rect::from_x_y_ranges(xa.min(xb)..=xa.max(xb), plot.y_range());
    clipped.rect_filled(band, 0.0, theme::ACCENT.gamma_multiply(BAND_OPACITY));

    for (x, label) in [(xa, "A"), (xb, "B")] {
        if !plot.x_range().contains(x) {
            continue;
        }
        clipped.vline(x, plot.y_range(), Stroke::new(1.0, theme::ACCENT));
        let badge = badge_rect(x, plot);
        painter.rect_filled(badge, BADGE_RADIUS, theme::ACCENT);
        painter.text(
            badge.center(),
            Align2::CENTER_CENTER,
            label,
            FontId::proportional(BADGE_FONT_SIZE),
            Color32::WHITE,
        );
    }
}

pub(super) fn tooltip(response: &Response, markers: &Markers, trace: &SorTrace) {
    let Some(measurement) = markers.measure(trace) else {
        return;
    };
    response.clone().on_hover_ui_at_pointer(|ui| {
        Grid::new("marker-measurement")
            .num_columns(2)
            .show(ui, |ui| {
                for (label, value) in measurement.labels() {
                    ui.label(label);
                    ui.strong(value);
                    ui.end_row();
                }
            });
    });
}

fn badge_rect(x: f32, plot: Rect) -> Rect {
    let bottom = plot.top() - BADGE_GAP;
    Rect::from_min_max(
        pos2(x - BADGE_WIDTH / 2.0, bottom - BADGE_HEIGHT),
        pos2(x + BADGE_WIDTH / 2.0, bottom),
    )
}
