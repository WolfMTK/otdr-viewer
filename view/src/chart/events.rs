use eframe::egui::{Align2, Color32, FontId, Painter, Rect, Shape, Stroke, StrokeKind, pos2, vec2};
use model::types::{SorEvent, SorTrace};

use crate::chart::Mapping;
use crate::theme;

const DOT_RADIUS: f32 = 4.5;
const DOT_OUTLINE: f32 = 1.5;
const LABEL_FONT_SIZE: f32 = 10.5;
const LABEL_PADDING: f32 = 4.0;
const LABEL_HEIGHT: f32 = 16.0;
const LABEL_LIFT: f32 = 10.0;
const LABEL_GAP: f32 = 3.0;
const LABEL_RADIUS: u8 = 4;
const LINE_OPACITY: f32 = 0.35;
const DASH: f32 = 4.0;
const DASH_GAP: f32 = 3.0;

pub(super) fn draw(painter: &Painter, mapping: &Mapping, events: &[SorEvent], trace: &SorTrace) {
    let plot = mapping.plot;
    let clipped = painter.with_clip_rect(plot);
    let font = FontId::proportional(LABEL_FONT_SIZE);
    let mut last_label_right = f32::NEG_INFINITY;

    for event in events {
        let x = mapping.x(event.distance_km);
        if !plot.x_range().contains(x) {
            continue;
        }
        let color = color_of(&event.kind);
        clipped.extend(Shape::dashed_line(
            &[pos2(x, plot.top()), pos2(x, plot.bottom())],
            Stroke::new(1.0, color.gamma_multiply(LINE_OPACITY)),
            DASH,
            DASH_GAP,
        ));

        let Some(level) = trace.level_at(event.distance_km) else {
            continue;
        };
        let dot = pos2(x, mapping.y(level).clamp(plot.top(), plot.bottom()));
        clipped.circle(dot, DOT_RADIUS, color, Stroke::new(DOT_OUTLINE, Color32::WHITE));

        let galley = painter.layout_no_wrap(label_of(event), font.clone(), color);
        let width = galley.size().x + 2.0 * LABEL_PADDING;
        let bottom = (dot.y - LABEL_LIFT).max(plot.top() + LABEL_HEIGHT);
        let label = Rect::from_min_size(
            pos2(x - width / 2.0, bottom - LABEL_HEIGHT),
            vec2(width, LABEL_HEIGHT),
        );
        if label.left() < last_label_right + LABEL_GAP {
            continue;
        }
        last_label_right = label.right();
        clipped.rect(
            label,
            LABEL_RADIUS,
            theme::BG_SURFACE,
            Stroke::new(1.0, color),
            StrokeKind::Inside,
        );
        let text_pos = Align2::CENTER_CENTER
            .anchor_size(label.center(), galley.size())
            .min;
        clipped.galley(text_pos, galley, color);
    }
}

fn label_of(event: &SorEvent) -> String {
    match event.kind.as_str() {
        "loss/drop/gain" => format!("{} · {:.2} дБ", event.number, event.loss_db),
        _ => event.number.to_string(),
    }
}

fn color_of(kind: &str) -> Color32 {
    match kind {
        "end of fiber" => theme::ERROR,
        "reflection" | "saturated reflection" => theme::ACCENT,
        _ => theme::EVENT_SPLICE,
    }
}
