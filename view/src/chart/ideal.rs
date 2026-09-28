use eframe::egui::{Painter, Stroke};
use view_model::ideal::Segment;

use crate::chart::Mapping;
use crate::theme;

const WIDTH: f32 = 2.0;

pub(super) fn draw(painter: &Painter, mapping: &Mapping, segments: &[Segment]) {
    let clipped = painter.with_clip_rect(mapping.plot);
    let stroke = Stroke::new(WIDTH, theme::IDEAL_LINE);
    let mut previous: Option<&Segment> = None;
    for segment in segments {
        let start = mapping.to_screen(segment.from_km, segment.from_db);
        let end = mapping.to_screen(segment.to_km, segment.to_db);
        if let Some(prev) = previous
            && prev.to_km == segment.from_km
        {
            let prev_end = mapping.to_screen(prev.to_km, prev.to_db);
            clipped.line_segment([prev_end, start], stroke);
        }
        clipped.line_segment([start, end], stroke);
        previous = Some(segment);
    }
}
