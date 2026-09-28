mod events;
mod ideal;
mod markers;
mod scale;

use eframe::egui::{
    Align2, CursorIcon, FontId, Painter, Pos2, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2,
    pos2, vec2,
};
use model::sor::downsample_min_max;
use view_model::chart_view::{ChartSettings, ChartView, DistanceRange, LevelRange};
use view_model::document::ChartParts;

use crate::chart::scale::{nice_step, tick_decimals, ticks, visible_range};
use crate::theme;

const MARGIN_LEFT: f32 = 56.0;
const MARGIN_RIGHT: f32 = 20.0;
const MARGIN_TOP: f32 = 24.0;
const MARGIN_BOTTOM: f32 = 32.0;
const LABEL_GAP: f32 = 6.0;
const LABEL_FONT_SIZE: f32 = 11.0;
const TRACE_WIDTH: f32 = 1.5;
const WHEEL_ZOOM_SPEED: f64 = 0.002;

pub fn show(ui: &mut Ui, parts: ChartParts<'_>, settings: &ChartSettings) {
    let ChartParts {
        trace,
        events,
        ideal,
        view,
        markers,
    } = parts;
    let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());
    let plot = Rect::from_min_max(
        response.rect.min + vec2(MARGIN_LEFT, MARGIN_TOP),
        response.rect.max - vec2(MARGIN_RIGHT, MARGIN_BOTTOM),
    );
    if plot.width() < 1.0 || plot.height() < 1.0 {
        return;
    }

    if settings.markers_visible {
        markers::handle_drag(ui, &Mapping::new(plot, view), markers, view.full());
    }
    handle_input(ui, &response, plot, view);

    let mapping = Mapping::new(plot, view);
    let Some((first, last)) = visible_range(&trace.distances_km, mapping.distance) else {
        return;
    };

    painter.rect_filled(plot, 0.0, theme::BG_SURFACE);
    draw_axes(&painter, &mapping, settings.grid_visible);

    let columns = plot.width().ceil() as usize;
    let on_screen = downsample_min_max(
        &trace.distances_km[first..last],
        &trace.levels_db[first..last],
        columns,
    );
    let points = on_screen
        .distances_km
        .iter()
        .zip(&on_screen.levels_db)
        .map(|(&km, &db)| mapping.to_screen(km, db))
        .collect();
    painter
        .with_clip_rect(plot)
        .line(points, Stroke::new(TRACE_WIDTH, theme::ACCENT));
    if settings.ideal_visible {
        ideal::draw(&painter, &mapping, ideal);
    }
    if settings.events_visible {
        events::draw(&painter, &mapping, events, trace);
    }
    painter.rect_stroke(plot, 0.0, Stroke::new(1.0, theme::BORDER_STRONG), StrokeKind::Inside);

    if settings.markers_visible {
        markers::draw(&painter, &mapping, markers);
        markers::tooltip(&response, markers, trace);
    }
}

struct Mapping {
    plot: Rect,
    distance: DistanceRange,
    levels: LevelRange,
}

impl Mapping {
    fn new(plot: Rect, view: &ChartView) -> Self {
        Self {
            plot,
            distance: view.visible(),
            levels: view.levels(),
        }
    }

    fn km(&self, x: f32) -> f64 {
        let share = f64::from((x - self.plot.left()) / self.plot.width());
        self.distance.start() + share * self.distance.span()
    }

    fn x(&self, km: f64) -> f32 {
        let share = (km - self.distance.start()) / self.distance.span();
        self.plot.left() + share as f32 * self.plot.width()
    }

    fn db(&self, y: f32) -> f64 {
        let share = f64::from((y - self.plot.top()) / self.plot.height());
        self.levels.min() + share * self.levels.span()
    }

    fn y(&self, db: f64) -> f32 {
        let share = (db - self.levels.min()) / self.levels.span();
        self.plot.top() + share as f32 * self.plot.height()
    }

    fn to_screen(&self, km: f64, db: f64) -> Pos2 {
        pos2(self.x(km), self.y(db))
    }
}

fn handle_input(ui: &Ui, response: &Response, plot: Rect, chart: &mut ChartView) {
    let mapping = Mapping::new(plot, chart);
    let (distance, levels) = (mapping.distance, mapping.levels);

    if let Some(pointer) = response.hover_pos().filter(|p| plot.contains(*p)) {
        let (scroll, pinch) = ui.input(|i| (i.smooth_scroll_delta.y, i.zoom_delta()));
        let factor = (f64::from(scroll) * WHEEL_ZOOM_SPEED).exp();
        if factor != 1.0 {
            chart.zoom_at(factor, mapping.km(pointer.x));
        }
        if pinch != 1.0 {
            chart.zoom_levels_at(f64::from(pinch), mapping.db(pointer.y));
        }
    }

    if response.dragged() {
        ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
        let drag = response.drag_delta();
        if drag != Vec2::ZERO {
            let delta_km = -f64::from(drag.x) * distance.span() / f64::from(plot.width());
            let delta_db = -f64::from(drag.y) * levels.span() / f64::from(plot.height());
            chart.pan(delta_km, delta_db);
        }
    }

    if response.double_clicked() {
        chart.fit();
    }
}

fn draw_axes(painter: &Painter, mapping: &Mapping, grid_visible: bool) {
    let font = FontId::proportional(LABEL_FONT_SIZE);
    let grid = Stroke::new(1.0, theme::BORDER_SOFT);
    let plot = mapping.plot;

    let distance = mapping.distance;
    let step = nice_step(distance.span());
    let decimals = tick_decimals(step);
    for km in ticks(distance.start(), distance.end(), step) {
        let x = mapping.x(km);
        if grid_visible {
            painter.vline(x, plot.y_range(), grid);
        }
        painter.text(
            pos2(x, plot.bottom() + LABEL_GAP),
            Align2::CENTER_TOP,
            format!("{km:.decimals$}"),
            font.clone(),
            theme::TEXT,
        );
    }

    let levels = mapping.levels;
    let step = nice_step(levels.span());
    let decimals = tick_decimals(step);
    for db in ticks(levels.min(), levels.max(), step) {
        let y = mapping.y(db);
        if grid_visible {
            painter.hline(plot.x_range(), y, grid);
        }
        painter.text(
            pos2(plot.left() - LABEL_GAP, y),
            Align2::RIGHT_CENTER,
            format!("{db:.decimals$}"),
            font.clone(),
            theme::TEXT,
        );
    }

    painter.text(
        pos2(plot.right(), plot.bottom() + LABEL_GAP + LABEL_FONT_SIZE + 2.0),
        Align2::RIGHT_TOP,
        "км",
        font.clone(),
        theme::TEXT,
    );
    painter.text(
        pos2(plot.left() - LABEL_GAP, plot.top() - LABEL_GAP),
        Align2::RIGHT_BOTTOM,
        "дБ",
        font,
        theme::TEXT,
    );
}
